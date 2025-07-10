use std::{
    env,
    io::{self, BufRead},
};

use reqwest::Client;
use thiserror::Error;

#[tokio::main]
async fn main() {
    if let Err(e) = dial_from_input().await {
        eprintln!("Error: {e}");
    }
}

async fn dial_from_input() -> Result<(), AppError> {
    let base_url = env::var("CTI_BASE_URL")?;
    let passcode = env::var("CTI_PASSCODE")?;
    let base = format!("{base_url}/cgi-bin/api-send_key?passcode={passcode}&keys=");

    let stdin = io::stdin();
    let number = stdin
        .lock()
        .lines()
        .next()
        .ok_or(AppError::NoInput)??
        .trim_matches('"')
        .replace(' ', "");

    if number.is_empty() {
        return Err(AppError::NoInput);
    }

    let client = Client::builder().build()?;

    println!("Dialing: {number}...");

    for ch in number.chars() {
        send_key(&client, &base, ch).await?;
    }

    client
        .get(format!("{base}SEND"))
        .send()
        .await?
        .error_for_status()?;

    Ok(())
}

async fn send_key(client: &Client, url: &str, ch: char) -> Result<(), AppError> {
    let key = match ch {
        '0'..='9' => ch.to_string(),
        '*' => "STAR".to_string(),
        '#' => "POUND".to_string(),
        _ => return Err(AppError::InvalidChar(ch)),
    };
    let url = format!("{url}{key}");
    client.get(&url).send().await?.error_for_status()?;

    Ok(())
}

#[derive(Debug, Error)]
pub enum AppError {
    #[error("Reqwest error: {0}")]
    Reqwest(#[from] reqwest::Error),
    #[error("Env error: {0}")]
    Env(#[from] std::env::VarError),
    #[error("IO error: {0}")]
    Io(#[from] io::Error),
    #[error("No input provided")]
    NoInput,
    #[error("Invalid character in phone number: {0}")]
    InvalidChar(char),
}
