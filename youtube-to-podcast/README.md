# YouTube to Podcast Project

This project utilizes Rust to prepare YouTube video to be uploaded on Podcast.

## Installation

### Prerequisites

- Rust Programming Language ([Install Rust](https://www.rust-lang.org/tools/install))
- YouTube API Key ([Getting Started on YouTube API Key](https://developers.google.com/youtube/v3/getting-started))

### Set up Environment Variables

Create a `.env` file in the root of the project based on the `.env.example` file provided.

```bash
cp .env.example .env
```

### Build and Run

```bash
cargo build --release
```

### Run the Application

```bash
cargo run --release
```

### Debugging

To enable debug logs, set DEBUG=true in your `.env` file before running the application.