use anyhow::Result;
use std::sync::Arc;
use crate::providers::ModelProvider;
use crate::sandbox::Sandbox;
use crate::task::Task;
use crate::agent::AgentProfile;

/// AgentRuntime executes tasks using either an internal LLM or external CLI.
/// For the orchestration pipeline, external CLIs are launched via tmux scripts
/// managed by the Orchestrator. This runtime is used for direct single-agent execution.
pub struct AgentRuntime {
    provider: Arc<dyn ModelProvider>,
    sandbox: Sandbox,
}

impl AgentRuntime {
    pub fn new(provider: Arc<dyn ModelProvider>, sandbox: Sandbox) -> Self {
        Self { provider, sandbox }
    }

    /// Execute a task using the internal LLM provider
    pub async fn execute_task(&self, task: &Task, _agent: &AgentProfile) -> Result<String> {
        tracing::info!("Agent starting work on task: {}", task.title);

        let prompt = format!(
            "You are an autonomous agent executing a task.\nTask: {}\nDescription: {}\n\nProvide a complete implementation.",
            task.title, task.description
        );

        let response = self.provider.generate(&prompt).await?;
        Ok(response)
    }
}
