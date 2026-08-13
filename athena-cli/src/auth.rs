use anyhow::{Context, Result};
use directories::ProjectDirs;
use reqwest::Client;
use serde::{Deserialize, Serialize};
use std::fs;
use std::path::PathBuf;
use tokio::time::{sleep, Duration};

const GITHUB_CLIENT_ID: &str = "178c6fc778ccc68e1d6a"; // Public GitHub CLI client ID for testing

#[derive(Deserialize)]
struct DeviceCodeResponse {
    device_code: String,
    user_code: String,
    verification_uri: String,
    interval: u64,
}

#[derive(Deserialize)]
struct TokenResponse {
    access_token: Option<String>,
    error: Option<String>,
}

#[derive(Serialize, Deserialize)]
pub struct AuthState {
    pub access_token: String,
    pub username: String,
}

pub fn get_config_path() -> Result<PathBuf> {
    let proj_dirs = ProjectDirs::from("com", "athena", "athena")
        .context("Could not determine config directory")?;
    let config_dir = proj_dirs.config_dir();
    if !config_dir.exists() {
        fs::create_dir_all(config_dir)?;
    }
    Ok(config_dir.join("credentials.json"))
}

pub fn load_auth() -> Result<AuthState> {
    let path = get_config_path()?;
    if !path.exists() {
        anyhow::bail!("Not authenticated. Please run `athena login` first.");
    }
    let content = fs::read_to_string(path)?;
    let state = serde_json::from_str(&content)?;
    Ok(state)
}

pub async fn login() -> Result<()> {
    let client = Client::new();
    
    println!("Initiating GitHub OAuth Device Flow...");
    
    // 1. Request device code
    let res = client.post("https://github.com/login/device/code")
        .header("Accept", "application/json")
        .query(&[("client_id", GITHUB_CLIENT_ID), ("scope", "read:user")])
        .send()
        .await?
        .json::<DeviceCodeResponse>()
        .await?;

    println!("\n=======================================================");
    println!("Please visit: {}", res.verification_uri);
    println!("And enter code: {}", res.user_code);
    println!("=======================================================\n");
    println!("Waiting for authorization...");

    // 2. Poll for token
    let mut interval = res.interval;
    let access_token = loop {
        sleep(Duration::from_secs(interval)).await;
        
        let token_res = client.post("https://github.com/login/oauth/access_token")
            .header("Accept", "application/json")
            .query(&[
                ("client_id", GITHUB_CLIENT_ID),
                ("device_code", &res.device_code),
                ("grant_type", "urn:ietf:params:oauth:grant-type:device_code")
            ])
            .send()
            .await?
            .json::<TokenResponse>()
            .await?;

        if let Some(token) = token_res.access_token {
            break token;
        }

        if let Some(error) = token_res.error {
            if error == "authorization_pending" {
                continue;
            } else if error == "slow_down" {
                interval += 5;
                continue;
            } else {
                anyhow::bail!("OAuth error: {}", error);
            }
        }
    };

    // 3. Fetch user profile
    let user_res: serde_json::Value = client.get("https://api.github.com/user")
        .header("Authorization", format!("Bearer {}", access_token))
        .header("User-Agent", "Athena-Swarm-CLI")
        .send()
        .await?
        .json()
        .await?;
        
    let username = user_res["login"].as_str().unwrap_or("unknown").to_string();
    
    // 4. Save state
    let state = AuthState {
        access_token,
        username: username.clone(),
    };
    
    let path = get_config_path()?;
    fs::write(&path, serde_json::to_string_pretty(&state)?)?;

    println!("\x1b[32m✓\x1b[0m Successfully authenticated as GitHub user: {}", username);
    Ok(())
}
