// run without open terminal
#![windows_subsystem = "windows"]

use std::process;
use std::fs;
use clap::Parser;
use humantime;
use reqwest;
use tokio::time;
use serde::Deserialize;
use serde_json;
use home;
use std::path::Path;

#[derive(Parser, Debug)]
#[command(author, version, about, long_about = None)]
struct Args {
    #[arg(short = 'c', long, value_name = "config_path", required = false)]
    config_path: Option<String>
}

#[derive(Debug, Deserialize)]
struct Config {
    token: String,
    duration: String,
    domains: Vec<String>,
}

async fn update_domains(token: &str, domains: &Vec<String>) -> Result<(), Box<dyn std::error::Error>> {
    // Create a comma-separated domains string
    let domains_str = domains.join(",");
    let url = format!("https://www.duckdns.org/update?domains={}&token={}", domains_str, token);

    let text: String = reqwest::get(&url).await?.text().await?;

    if text != "OK" {
        return Err("domains update failed".into());
    }
    Ok(())
}

#[tokio::main]
async fn main() {

    let args = Args::parse();
    let config_path = if let Some(config) = &args.config_path {
        Path::new(config).to_path_buf()
    } else {
        let config_path = Path::new("duckdns.config.json").to_path_buf();
        config_path
    };
    
    if !config_path.exists() {
        eprintln!("Error: The config file does not exist.");
        process::exit(1);
    }
    let config_data = fs::read_to_string(config_path).unwrap();
    let config: Config = serde_json::from_str(config_data.as_str()).unwrap();

    let dur = humantime::parse_duration(config.duration.as_str()).unwrap();
    let token = config.token.as_str();
    let domains = &config.domains;
    loop {
        update_domains(token, domains).await.unwrap_or_else(|error| {
            println!("{}", error);
        });
        time::sleep(dur).await;
    }
}
