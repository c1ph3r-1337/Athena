use serde::{Deserialize, Serialize};
use std::collections::HashMap;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum AgentType {
    InternalLLM,
    ExternalCLI {
        command: String,
        args: Vec<String>,
    },
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AgentProfile {
    pub name: String,
    pub model: String,
    pub capabilities: HashMap<String, f32>,
    pub token_cost: f32,
    pub latency_score: f32, // Lower is better
    pub agent_type: AgentType,
    pub context_window: usize,
}

impl AgentProfile {
    pub fn new(name: String, model: String) -> Self {
        Self {
            name,
            model,
            capabilities: HashMap::new(),
            token_cost: 0.0,
            latency_score: 1.0,
            agent_type: AgentType::InternalLLM,
            context_window: 128_000,
        }
    }

    pub fn new_cli(name: String, command: String, args: Vec<String>) -> Self {
        Self {
            name,
            model: command.clone(),
            capabilities: HashMap::new(),
            token_cost: 0.0,
            latency_score: 1.0,
            agent_type: AgentType::ExternalCLI { command, args },
            context_window: 128_000,
        }
    }
}
