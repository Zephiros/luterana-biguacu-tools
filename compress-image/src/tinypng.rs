use reqwest::Client;
use serde::Deserialize;
use std::fs;

#[derive(Deserialize)]
struct TinyPngResponse {
    output: Output,
}

#[derive(Deserialize)]
struct Output {
    url: String,
}

#[derive(Clone)]
pub struct TinyPngClient {
    client: Client,
    api_url: String,
    api_key: String,
}

impl TinyPngClient {
    pub fn new(api_url: String, api_key: String) -> Self {
        TinyPngClient {
            client: Client::new(),
            api_url,
            api_key,
        }
    }

    pub async fn compress_image(&self, input_path: &str, output_path: &str) -> Result<(), Box<dyn std::error::Error>> {
        let image_data = fs::read(input_path)?;
        let response = self.client
            .post(&self.api_url)
            .basic_auth("api", Some(&self.api_key))
            .body(image_data)
            .send()
            .await?;

        if response.status().is_success() {
            let tinypng_response: TinyPngResponse = response.json().await?;
            let output_data = self.client
                .get(&tinypng_response.output.url)
                .send()
                .await?
                .bytes()
                .await?;

            fs::write(output_path, output_data)?;
            Ok(())
        } else {
            let error_text = response.text().await?;
            Err(Box::new(std::io::Error::new(
                std::io::ErrorKind::Other,
                format!("TinyPNG API error: {}", error_text),
            )))
        }
    }
}