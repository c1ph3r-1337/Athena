use async_trait::async_trait;
use serde_json::Value;

#[async_trait]
pub trait ModelProvider: Send + Sync {
    async fn generate(&self, prompt: &str) -> anyhow::Result<String>;
    async fn call_with_tools(&self, prompt: &str, tools: Vec<Value>) -> anyhow::Result<Value>;
}

// Submodules to implement later (e.g. openai, local)
pub mod openai;
// pub mod local;
