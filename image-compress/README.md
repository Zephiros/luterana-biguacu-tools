# Image Compress Project

This project utilizes Rust to compress images using the TinyPNG API.

## Installation

### Prerequisites

- Rust Programming Language ([Install Rust](https://www.rust-lang.org/tools/install))
- TinyPNG API Key ([Sign up for TinyPNG](https://tinypng.com/developers))

### Set up Environment Variables

Create a `.env` file in the root of the project based on the `.env.example` file provided.

```bash
cp .env.example .env
```

* **DEBUG:** Set this to `true` or `false` to enable/disable debug logging.
* **INPUT_COMPRESS_FOLDER:** Path to the folder containing images to be compressed. 
  * Example: `INPUT_COMPRESS_FOLDER=input/`
* **OUTPUT_COMPRESS_FOLDER:** Path where compressed images will be saved.
  * Example: `OUTPUT_COMPRESS_FOLDER=output/`
* **TINYPNG_API_URL:** URL endpoint for the TinyPNG API.
  * Example: `TINYPNG_API_URL=https://api.tinify.com/shrink`
* **TINYPNG_API_KEY:** Your API key for accessing the TinyPNG API.
  * Example: `TINYPNG_API_KEY=your_tinypng_api_key_here`

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