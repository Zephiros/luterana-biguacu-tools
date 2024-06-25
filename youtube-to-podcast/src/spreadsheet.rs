use std::error::Error;
use reqwest::blocking::Client;
use serde::{Deserialize};
use std::fs::File;
use std::io::BufReader;
use serde_json::{json};
use yup_oauth2::{read_service_account_key, ServiceAccountAuthenticator};
use std::env;
use dotenv::dotenv;
use once_cell::sync::Lazy;

struct Config {
    api_key: String,
    spreadsheet_id: String,
    credentials: String,
    sample_file: String,
}

static CONFIG: Lazy<Config> = Lazy::new(|| {
    dotenv().ok();
    Config {
        api_key: env::var("SPREADSHEET_API_KEY").expect("SPREADSHEET_API_KEY must be set"),
        spreadsheet_id: env::var("SPREADSHEET_ID").expect("SPREADSHEET_ID must be set"),
        credentials: env::var("SPREADSHEET_CREDENTIALS_FILE").expect("SPREADSHEET_CREDENTIALS_FILE must be set"),
        sample_file: env::var("SPREADSHEET_CACHE_FILE").expect("SPREADSHEET_CACHE_FILE must be set"),
    }
});

#[derive(Debug, Deserialize)]
pub struct SheetsResponse {
    pub values: Vec<Vec<String>>,
}

async fn get_token() -> Result<String, Box<dyn Error>> {
    rustls::crypto::ring::default_provider().install_default().expect("Failed to install rustls crypto provider");

    let sa_key = read_service_account_key(CONFIG.credentials.to_string()).await?;

    let auth = ServiceAccountAuthenticator::builder(sa_key)
        .build()
        .await?;

    let token = auth
        .token(&["https://www.googleapis.com/auth/spreadsheets"])
        .await?;

    Ok(String::from(token.token().unwrap()))
}

fn list_from_sample() -> SheetsResponse {
    let file = File::open(&CONFIG.sample_file).expect("File not found");
    let reader = BufReader::new(file);

    return serde_json::from_reader(reader).expect("Error while reading or parsing the file")
}

fn list_from_sheet(sheet_name: &str, range: &str) -> SheetsResponse {
    let url = format!(
        "https://sheets.googleapis.com/v4/spreadsheets/{}/values/{}?key={}",
        CONFIG.spreadsheet_id, format!("{}!{}", sheet_name, range), CONFIG.api_key
    );

    let client = Client::new();
    let response = client.get(&url).send().expect("Failed to send request");

    let response_text = response.text().expect("Failed to read response text");
    let list: SheetsResponse = serde_json::from_str(&response_text).expect("Failed to parse JSON");

    return list;
}

fn add_to_sheet(sheet_name: &str, value: Vec<Vec<String>>) -> bool {
    let token = tokio::runtime::Runtime::new()
        .unwrap()
        .block_on(get_token())
        .expect("Failed to obtain token");

    let url = format!(
        "https://sheets.googleapis.com/v4/spreadsheets/{}/values/{}:append?valueInputOption=RAW&",
        CONFIG.spreadsheet_id, sheet_name
    );

    let client = Client::new();

    let json = json!({
        "values": value
    });

    let response = client
        .post(&url)
        .json(&json)
        .bearer_auth(token)
        .send()
        .expect("Failed to send request");

    if !response.status().is_success() {
        println!("Failed to append data: {:?}", response.status());
    }

    return response.status().is_success();
}

pub fn list(debug: bool, sheet_name: &str, range: &str) -> SheetsResponse {
    return if debug {
        list_from_sample()
    } else {
        list_from_sheet(sheet_name, range)
    }
}

pub fn add(debug: bool, sheet_name: &str, value: Vec<Vec<String>>) -> bool {
    return if debug {
        add_to_sheet(sheet_name, value)
    } else {
        add_to_sheet(sheet_name, value)
    }
}