use serde::Deserialize;
use chrono::{Datelike, DateTime};
use chrono::Utc;
use dotenv::dotenv;
use std::env;
use once_cell::sync::Lazy;
use regex::Regex;

mod youtube;
mod spreadsheet;

struct Config {
    debug: bool,
    sheet_name: String,
    range: String,
    download_folder: String,
}

static CONFIG: Lazy<Config> = Lazy::new(|| {
    dotenv().ok();
    Config {
        debug: env::var("DEBUG").expect("DEBUG must be set").parse().unwrap(),
        sheet_name: env::var("SPREADSHEET_SHEET_NAME").expect("SPREADSHEET_SHEET_NAME must be set"),
        range: env::var("SPREADSHEET_SHEET_RANGE").expect("SPREADSHEET_SHEET_RANGE must be set"),
        download_folder: env::var("DOWNLOAD_FOLDER").expect("DOWNLOAD_FOLDER must be set"),
    }
});

#[allow(dead_code)]
#[derive(Debug, Deserialize)]
struct SpreadsheetTuple {
    year: String,
    title: String,
    link: String,
    start_time: String,
    end_time: String,
    is_downloaded: bool,
    is_online: bool
}

#[derive(Debug, Deserialize)]
struct VideoToDownload {
    id: String,
    title: String,
    start_time: String,
    end_time: String
}

struct MainResult {
    items_to_add: Vec<Vec<String>>,
    items_to_download: Vec<VideoToDownload>
}

fn extract_year(date_str: &str) -> i32 {
    let date_time = DateTime::parse_from_rfc3339(date_str)
        .expect("Failed to parse date")
        .with_timezone(&Utc);

    return date_time.year();
}

fn extract_title(input: &str) -> String {
    let re = Regex::new(r"^Mensagem \d{2}/\d{2} - ").unwrap();
    re.replace(input, "").to_string()
}

fn list_spreadsheet_tuples(sheet_name: &str, range: &str, debug: bool) -> Vec<SpreadsheetTuple> {
    let mut tuples: Vec<SpreadsheetTuple> = Vec::new();
    let spreadsheet_list = spreadsheet::list(debug, sheet_name, range);

    for (index, row) in spreadsheet_list.values.iter().enumerate() {
        if index == 0 {
            continue;
        }

        if row.len() >= 7 {
            let tuple = SpreadsheetTuple {
                year: row[0].clone(),
                title: row[1].clone(),
                link: row[2].clone(),
                start_time: row[3].clone(),
                end_time: row[4].clone(),
                is_downloaded: if row[5].clone() == "Sim" { true } else { false },
                is_online: if row[6].clone() == "Sim" { true } else { false }
            };
            tuples.push(tuple);
        }
    }

    return tuples;
}

fn get_result() -> MainResult {
    let youtube_list = youtube::list(CONFIG.debug);
    let spreadsheet_list = list_spreadsheet_tuples(&CONFIG.sheet_name, &CONFIG.range, CONFIG.debug);

    let mut items_to_add: Vec<Vec<String>> = Vec::new();
    let mut items_to_download: Vec<VideoToDownload> = Vec::new();

    for item in youtube_list.items.iter().rev() {
        if let Some(video_id) = &item.id.videoId {
            let mut item_found = false;

            for spreadsheet in spreadsheet_list.iter() {
                let title_to_find = item.snippet.title.replace(" - Luterana Biguaçu", "");
                if title_to_find == spreadsheet.title {
                    item_found = true;
                    if !spreadsheet.is_online && !spreadsheet.is_downloaded && spreadsheet.start_time != "00:00:00" && spreadsheet.end_time != "00:00:00" {
                        items_to_download.push(VideoToDownload {
                            id: video_id.to_string(),
                            title: spreadsheet.title.clone(),
                            start_time: spreadsheet.start_time.clone(),
                            end_time: spreadsheet.end_time.clone(),
                        });
                    }
                    break;
                }
            }

            if !item_found {
                let year = extract_year(&item.snippet.publishTime);
                let url = format!("https://www.youtube.com/watch?v={}", video_id);
                items_to_add.push(vec![
                    year.to_string(),
                    item.snippet.title.clone().replace(" - Luterana Biguaçu", ""),
                    url,
                    String::from("00:00:00"),
                    String::from("00:00:00"),
                    String::from("00:00:00"),
                    String::from("Não"),
                    String::from("Não")
                ]);
            }
        }
    }

    return MainResult {
        items_to_add,
        items_to_download
    };
}

fn main() {
    println!("Ready to start!");

    let response = get_result();

    spreadsheet::add(CONFIG.debug, &CONFIG.sheet_name, response.items_to_add);

    for item in response.items_to_download {
        let video_id = &item.id.to_string();

        println!(
            "  [+] Downloading video {:?} (ID: {:?}). Start on {:?} and finish on {:?}",
            item.title, video_id, &item.start_time.to_string(), &item.end_time.to_string()
        );

        let filename = extract_title(&item.title);

        youtube::download_video(
            video_id,
            &filename,
            &item.start_time.to_string(),
            &item.end_time.to_string(),
            &CONFIG.download_folder
        );

        youtube::download_video_cover(
            video_id,
            &filename,
            &CONFIG.download_folder
        );
    }

    println!("Ended!");
}
