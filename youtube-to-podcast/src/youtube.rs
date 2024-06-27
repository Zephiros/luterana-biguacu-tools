use std::env;
use youtube_dl::{YoutubeDl};
use serde::{Deserialize};
use reqwest::blocking::Client;
use std::fs::File;
use std::io::copy;
use std::io::BufReader;
use dotenv::dotenv;
use once_cell::sync::Lazy;

struct Config {
    api_key: String,
    channel_id: String,
    sample_file: String,
}

static CONFIG: Lazy<Config> = Lazy::new(|| {
    dotenv().ok();
    Config {
        api_key: env::var("YOUTUBE_API_KEY").expect("YOUTUBE_API_KEY must be set"),
        channel_id: env::var("YOUTUBE_CHANNEL_ID").expect("YOUTUBE_CHANNEL_ID must be set"),
        sample_file: env::var("YOUTUBE_CACHE_FILE").expect("YOUTUBE_CACHE_FILE must be set"),
    }
});

#[allow(dead_code)]
#[allow(non_snake_case)]
#[derive(Debug, Deserialize)]
pub struct PageInfo {
    pub total_results: u32,
}

#[allow(non_snake_case)]
#[derive(Debug, Deserialize)]
pub struct VideoId {
    pub videoId: Option<String>,
}

#[allow(dead_code)]
#[allow(non_snake_case)]
#[derive(Debug, Deserialize)]
pub struct Thumbnails {
    pub default: Thumbnail,
    pub medium: Thumbnail,
    pub high: Thumbnail,
}

#[allow(dead_code)]
#[allow(non_snake_case)]
#[derive(Debug, Deserialize)]
pub struct Thumbnail {
    pub url: String,
    pub width: Option<u32>,
    pub height: Option<u32>,
}

#[allow(dead_code)]
#[allow(non_snake_case)]
#[derive(Debug, Deserialize)]
pub struct VideoSnippet {
    pub title: String,
    pub description: String,
    pub thumbnails: Thumbnails,
    pub publishTime: String
}

#[allow(non_snake_case)]
#[derive(Debug, Deserialize)]
pub struct VideoItem {
    pub id: VideoId,
    pub snippet: VideoSnippet,
}

#[allow(non_snake_case)]
#[derive(Debug, Deserialize)]
pub struct VideoListResponse {
    pub items: Vec<VideoItem>,
}

fn list_from_sample() -> VideoListResponse {
    let file = File::open(&CONFIG.sample_file).expect("File not found");
    let reader = BufReader::new(file);

    return serde_json::from_reader(reader).expect("Error while reading or parsing the file")
}

fn list_from_channel() -> VideoListResponse {
    let url = format!(
        "https://www.googleapis.com/youtube/v3/search?key={}&channelId={}&part=snippet,id&order=date&maxResults=50",
        CONFIG.api_key, CONFIG.channel_id);

    let client = Client::new();
    let response = client.get(&url).send().expect("Failed to send request");

    let response_text = response.text().expect("Failed to read response text");
    let video_list: VideoListResponse = serde_json::from_str(&response_text).expect("Failed to parse JSON");

    return video_list;
}

pub fn list(debug: bool) -> VideoListResponse {
    let video_list = if debug {
        list_from_sample()
    } else {
        list_from_channel()
    };

    return video_list;
}

pub fn download_video(video_code: &str, filename: &str, start_time: &str, end_time: &str, video_folder: &str) {
    let url = format!("https://www.youtube.com/watch?v={}", video_code);
    let output_template = format!("{}.%(ext)s", filename);

    let result = YoutubeDl::new(url)
        .extra_arg("--extract-audio")
        .extra_arg("--audio-format")
        .extra_arg("mp3")
        .extra_arg("--postprocessor-args")
        .extra_arg(format!("-ss {} -to {}", start_time, end_time))
        .extra_arg("-o")
        .extra_arg(&output_template)
        .download_to(video_folder);

    match result {
        Err(e) => {
            eprintln!("Error: {:?}", e);
        }
        _ => {}
    }
}

pub fn download_video_cover(video_code: &str, video_title: &str, video_folder: &str) {
    let url = format!("https://i3.ytimg.com/vi/{}/maxresdefault.jpg", video_code);
    let file_path = format!("{}{}.jpg", video_folder, video_title);
    let response = Client::new().get(url).send().expect("Failed to send request");
    let mut file = File::create(file_path).expect("Failed to create image file");
    let content = response.bytes().expect("Failed to convert to bytes");

    copy(&mut content.as_ref(), &mut file).expect("Failed to copy image");
}