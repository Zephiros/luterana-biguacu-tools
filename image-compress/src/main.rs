mod tinypng;

use std::{env, fs};
use std::path::Path;
use dotenv::dotenv;
use crate::tinypng::TinyPngClient;

fn create_output_directory(output_folder: &str) {
    if !Path::new(output_folder).exists() {
        fs::create_dir_all(output_folder).expect("Failed to create output directory");
    }
}

async fn process_directory(client: &TinyPngClient, debug: bool, input_folder: &str, output_folder: &str) -> Result<(), Box<dyn std::error::Error>> {
    let mut handles = vec![];

    for entry in fs::read_dir(input_folder)? {
        let entry = entry?;
        let input_path = entry.path();

        if input_path.is_file() {
            let file_name = input_path.file_name().expect("Failed to get file name");
            let output_path = Path::new(output_folder).join(file_name);

            let client = client.clone();
            let input_path = input_path.clone();

            if debug {
                println!("  [+] Image {:?} compressed", input_path);
                continue;
            }

            handles.push(tokio::spawn(async move {
                match client.compress_image(input_path.to_str().unwrap(), output_path.to_str().unwrap()).await {
                    Ok(_) => println!("  [+] Image {:?} compressed", input_path),
                    Err(e) => eprintln!("Failed to compress image {:?}: {}", input_path, e),
                }
            }));
        }
    }

    for handle in handles {
        if let Err(e) = handle.await {
            eprintln!("Task failed: {}", e);
        }
    }

    Ok(())
}

#[tokio::main]
async fn main() {
    println!("Ready to start!");

    dotenv().ok();

    let debug: bool = env::var("DEBUG").expect("DEBUG must be set").parse().unwrap();
    let input_folder = env::var("INPUT_COMPRESS_FOLDER").expect("INPUT_COMPRESS_FOLDER must be set");
    let output_folder =  env::var("OUTPUT_COMPRESS_FOLDER").expect("OUTPUT_COMPRESS_FOLDER must be set");
    let tinypng_api_url = env::var("TINYPNG_API_URL").expect("TINYPNG_API_URL must be set");
    let tinypng_api_key = env::var("TINYPNG_API_KEY").expect("TINYPNG_API_KEY must be set");

    create_output_directory(&output_folder);

    let client = TinyPngClient::new(tinypng_api_url, tinypng_api_key);

    if let Err(e) = process_directory(&client, debug, &input_folder, &output_folder).await {
        eprintln!("Error processing directory: {}", e);
    }

    println!("Ended!");
}