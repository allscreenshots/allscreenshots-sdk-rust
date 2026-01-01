# Allscreenshots Demo

A demo web application showcasing the Allscreenshots Rust SDK.

## Prerequisites

- Rust 1.70 or later
- An Allscreenshots API key

## Setup

1. Set your API key as an environment variable:

```bash
export ALLSCREENSHOTS_API_KEY=your-api-key
```

2. Build and run the application:

```bash
cargo run
```

3. Open your browser and navigate to:

```
http://localhost:3000
```

## Usage

1. Enter a URL in the text field (e.g., `https://github.com`)
2. Select a device preset from the dropdown
3. Optionally check "Full page" to capture the entire scrollable page
4. Click "Take Screenshot" to capture

The screenshot will be displayed in the result area below.

## Features

- Synchronous screenshot capture
- Multiple device presets (Desktop, Mobile, Tablet)
- Full page capture option
- Real-time loading state
- Error handling with user-friendly messages

## Project structure

```
sample-app/
├── Cargo.toml          # Project dependencies
├── src/
│   └── main.rs         # Application code with embedded HTML
├── README.md
└── LICENSE
```

## API endpoint

The application exposes a single API endpoint:

### POST /api/screenshot

Request body:
```json
{
  "url": "https://example.com",
  "device": "Desktop HD",
  "full_page": false
}
```

Response:
```json
{
  "success": true,
  "image": "data:image/png;base64,...",
  "error": null
}
```

## Configuration

| Environment Variable | Description | Required |
|---------------------|-------------|----------|
| `ALLSCREENSHOTS_API_KEY` | Your Allscreenshots API key | Yes |
| `RUST_LOG` | Log level (e.g., `debug`, `info`) | No |

## License

Apache License 2.0
