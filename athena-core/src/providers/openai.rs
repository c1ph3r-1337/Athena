use super::ModelProvider;
use anyhow::{Context, Result};
use async_trait::async_trait;
use reqwest::Client;
use serde_json::{json, Value};
use std::env;

pub struct OpenAiProvider {
    client: Client,
    api_key: String,
    model: String,
}

impl OpenAiProvider {
    pub fn new(model: String) -> Result<Self> {
        let api_key = env::var("OPENAI_API_KEY")
            .context("OPENAI_API_KEY environment variable is not set")?;
        
        Ok(Self {
            client: Client::new(),
            api_key,
            model,
        })
    }
}

#[async_trait]
impl ModelProvider for OpenAiProvider {
    async fn generate(&self, prompt: &str) -> Result<String> {
        let payload = json!({
            "model": self.model,
            "messages": [
                {
                    "role": "user",
                    "content": prompt
                }
            ],
            "response_format": { "type": "json_object" }
        });

        let response = self.client
            .post("https://api.openai.com/v1/chat/completions")
            .header("Authorization", format!("Bearer {}", self.api_key))
            .json(&payload)
            .send()
            .await?
            .error_for_status()?
            .json::<Value>()
            .await?;

        let content = response["choices"][0]["message"]["content"]
            .as_str()
            .context("Invalid response format from OpenAI")?;

        Ok(content.to_string())
    }

    async fn call_with_tools(&self, prompt: &str, tools: Vec<Value>) -> Result<Value> {
        let payload = json!({
            "model": self.model,
            "messages": [
                {
                    "role": "user",
                    "content": prompt
                }
            ],
            "tools": tools,
            "tool_choice": "auto"
        });

        let response = self.client
            .post("https://api.openai.com/v1/chat/completions")
            .header("Authorization", format!("Bearer {}", self.api_key))
            .json(&payload)
            .send()
            .await?
            .error_for_status()?
            .json::<Value>()
            .await?;

        // Return the whole message object so the caller can check for tool_calls
        let message = response["choices"][0]["message"].clone();
        Ok(message)
    }
}
