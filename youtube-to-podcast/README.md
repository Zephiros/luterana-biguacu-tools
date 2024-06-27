# YouTube to Podcast Project

This project utilizes Rust to prepare YouTube video to be uploaded on Podcast.

## Installation

### Prerequisites

- Rust Programming Language ([Install Rust](https://www.rust-lang.org/tools/install))
- TinyPNG API Key ([Sign up for TinyPNG](https://tinypng.com/developers))

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