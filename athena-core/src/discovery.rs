use serde::{Deserialize, Serialize};
use tokio::process::Command;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct DiscoveredAgent {
    pub name: String,
    pub command: String,
    pub version: Option<String>,
    pub available: bool,
    pub capabilities: Vec<String>,
    pub description: String,
}

pub async fn discover_agents() -> Vec<DiscoveredAgent> {
    let agent_definitions = vec![
        (
            "agy",
            vec!["coding", "architecture", "debugging", "testing", "documentation"],
            "Agy agent",
        ),
        (
            "codex",
            vec!["coding", "refactoring", "review", "security"],
            "Codex agent",
        ),
        (
            "claude",
            vec!["coding", "architecture", "analysis", "documentation", "review"],
            "Claude agent",
        ),
    ];

    let mut agents = Vec::new();

    for (name, caps, desc) in agent_definitions {
        let which_output = Command::new("which").arg(name).output().await;
        
        let (available, command, version) = match which_output {
            Ok(output) if output.status.success() => {
                let cmd_path = String::from_utf8_lossy(&output.stdout).trim().to_string();
                
                let version = if let Ok(v_output) = Command::new(name).arg("--version").output().await {
                    if v_output.status.success() {
                        Some(String::from_utf8_lossy(&v_output.stdout).trim().to_string())
                    } else {
                        None
                    }
                } else {
                    None
                };
                let mut is_available = true;

                // Check auth status for Claude
                if name == "claude" {
                    if let Ok(auth_output) = Command::new(name).args(["auth", "status"]).output().await {
                        let stdout = String::from_utf8_lossy(&auth_output.stdout);
                        if !auth_output.status.success() || stdout.contains("\"loggedIn\":false") {
                            is_available = false;
                        }
                    } else {
                        is_available = false;
                    }
                }

                (is_available, cmd_path, version)
            },
            _ => (false, name.to_string(), None),
        };

        agents.push(DiscoveredAgent {
            name: name.to_string(),
            command,
            version,
            available,
            capabilities: caps.iter().map(|&s| s.to_string()).collect(),
            description: desc.to_string(),
        });
    }

    agents
}

pub fn format_agent_list(agents: &[DiscoveredAgent]) -> String {
    let mut out = String::new();
    out.push_str("Available Agents:\n");
    for agent in agents {
        if agent.available {
            out.push_str(&format!("- {}: {} (Version: {})\n", agent.name, agent.description, agent.version.as_deref().unwrap_or("Unknown")));
            out.push_str(&format!("  Capabilities: {}\n", agent.capabilities.join(", ")));
        }
    }
    out
}

pub fn agent_available(agents: &[DiscoveredAgent], name: &str) -> bool {
    agents.iter().any(|a| a.name == name && a.available)
}
