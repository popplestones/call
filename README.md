# call

A minimal CLI utility to initiate outbound calls from a Grandstream desk phone via its HTTP API.

Designed for use with `rolodex` or other CLI tools that provide phone numbers via `stdin`.

## Features

- Reads a phone number from `stdin`
- Sends individual key presses to the phone
- Supports digits `0–9`, `*`, and `#`
- Triggers the `SEND` key to initiate the call
- Displays the number being dialed
- Validates and trims input

## Usage

```bash
echo "0400 123 456" | call
# or
rolodex | jq .phone | call
```

You can use this tool with [rolodex](https://github.com/popplestones/rolodex), a simple CLI contact browser that outputs structured JSON.

## Requirements

 - A Grandstream phone with HTTP API access enabled
 - CTI_BASE_URL and CTI_PASSCODE must be set in the environment

### Example:

```bash
export CTI_BASE_URL=http://192.168.1.100
export CTI_PASSCODE=1234
```

## Installation

Clone and build:

```bash
git clone https://github.com/popplestones/call.git
cd call
cargo build --release
```

Then copy the binary to somewhere in your `PATH`:

```bash
cp target/release/call /usr/local/bin/
```

## Error Handling

The app will return user-friendly errors for:

 - Missing or invalid environment variables
 - Empty or malformed input
 - Unsupported characters in the number
 - HTTP request failures

## License

MIT License
