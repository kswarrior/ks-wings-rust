use anyhow::Result;
use clap::Parser;
use reqwest::Client;
use serde::{Deserialize, Serialize};
use std::fs;

/// Minimal KS Wings Config
#[derive(Serialize, Deserialize, Debug)]
struct Config {
    panel_url: String,
    api_key: String,
}

/// CLI arguments
#[derive(Parser, Debug)]
#[command(author, version, about)]
struct Args {
    /// Panel URL (e.g., https://panel.ksweb.qzz.io)
    #[arg(long)]
    panel: String,

    /// API Key for authentication
    #[arg(long)]
    key: String,
}

#[tokio::main]
async fn main() -> Result<()> {
    let args = Args::parse();

    println!("Testing connection to panel: {}", args.panel);

    // Test connectivity
    match test_panel_connection(&args.panel, &args.key).await {
        Ok(true) => {
            println!("✅ Successfully connected to panel");

            // Save config
            let config = Config {
                panel_url: args.panel.clone(),
                api_key: args.key.clone(),
            };
            save_config(&config)?;
            println!("Configuration saved to config.json");
        }
        Ok(false) | Err(_) => {
            println!("❌ Failed to connect to panel");
        }
    }

    Ok(())
}

/// Test if panel is reachable
async fn test_panel_connection(panel_url: &str, api_key: &str) -> Result<bool> {
    let client = Client::new();

    // Send a GET request with header "Authorization: Bearer <API_KEY>"
    let res = client
        .get(panel_url)
        .header("Authorization", format!("Bearer {}", api_key))
        .send()
        .await;

    match res {
        Ok(response) => Ok(response.status().is_success()),
        Err(_) => Ok(false),
    }
}

/// Save config to local file (JSON)
fn save_config(config: &Config) -> Result<()> {
    let content = serde_json::to_string_pretty(config)?;
    fs::write("config.json", content)?;
    Ok(())
}
