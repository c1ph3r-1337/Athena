use anyhow::Result;
use std::process::Command;
use std::io::{self, Write};

pub async fn run_setup(force: bool) -> Result<()> {
    if !force {
        if let Some(home) = dirs::home_dir() {
            let agents_file = home.join(".athena/agents.json");
            if agents_file.exists() {
                // Setup has already been completed previously.
                return Ok(());
            }
        }
    }

    println!("\n[Athena Setup Wizard]");
    println!("Detecting required agent CLIs...");

    let tools = vec![
        ("claude", "Claude Code", "npm install -g @anthropic-ai/claude-code", "claude login"),
        ("agy", "Antigravity (agy)", "npm install -g agy", "agy login"),
        ("codex", "OpenAI Codex", "npm install -g codex-cli", "codex login")
    ];

    let os = std::env::consts::OS;
    println!("Detected OS: {}", os);

    let mut active_agents = Vec::new();

    for (cmd, name, install_cmd, login_cmd) in tools {
        if !is_installed(cmd) {
            print!("{} ({}) is not installed. Install now? [y/N]: ", name, cmd);
            io::stdout().flush()?;
            
            let mut input = String::new();
            io::stdin().read_line(&mut input)?;
            
            if input.trim().eq_ignore_ascii_case("y") {
                println!("Running: {}", install_cmd);
                let status = Command::new("sh").arg("-c").arg(install_cmd).status();
                if let Ok(s) = status {
                    if s.success() {
                        println!("  \x1b[32m✓\x1b[0m {} installed successfully.", name);
                        println!("Please login to {}:", name);
                        let _ = Command::new("sh").arg("-c").arg(login_cmd).status();
                        active_agents.push(cmd.to_string());
                    } else {
                        println!("  \x1b[31m✗\x1b[0m Failed to install {}.", name);
                    }
                }
            } else {
                println!("Skipping {} installation.", name);
            }
        } else {
            println!("  \x1b[32m✓\x1b[0m {} is already installed.", name);
            print!("Would you like to authenticate/login to {} now? [y/N]: ", name);
            io::stdout().flush()?;
            
            let mut input = String::new();
            io::stdin().read_line(&mut input)?;
            
            if input.trim().eq_ignore_ascii_case("y") {
                println!("Running: {}", login_cmd);
                let _ = Command::new("sh").arg("-c").arg(login_cmd).status();
                active_agents.push(cmd.to_string());
            }
        }
    }
    
    // Save active agents
    if let Some(home) = dirs::home_dir() {
        let athena_dir = home.join(".athena");
        std::fs::create_dir_all(&athena_dir)?;
        let _ = std::fs::write(athena_dir.join("agents.json"), serde_json::to_string(&active_agents).unwrap_or_default());
    }

    println!("[Setup Complete]\n");
    Ok(())
}

fn is_installed(cmd: &str) -> bool {
    Command::new("which")
        .arg(cmd)
        .output()
        .map(|out| out.status.success())
        .unwrap_or(false)
}
