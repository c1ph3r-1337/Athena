use anyhow::Result;
use chrono::Local;
use serde::{Deserialize, Serialize};
use std::fs;
use std::path::{Path, PathBuf};
use uuid::Uuid;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Session {
    pub id: String,
    pub project_name: String,
    pub base_dir: PathBuf,
    pub created_at: String,
}

impl Session {
    pub fn new(project_name: &str, base_dir: &Path) -> Result<Self> {
        let date = Local::now().format("%Y-%m-%d").to_string();
        let suffix = &Uuid::new_v4().to_string()[..4];
        let id = format!("{}-{}", date, suffix);
        let created_at = Local::now().to_rfc3339();

        let session_dir = base_dir.join(".orchestrator").join("sessions").join(&id);
        
        fs::create_dir_all(session_dir.join("tasks"))?;
        fs::create_dir_all(session_dir.join("assignments"))?;
        fs::create_dir_all(session_dir.join("logs"))?;
        fs::create_dir_all(session_dir.join("artifacts"))?;
        fs::create_dir_all(session_dir.join("results"))?;

        Ok(Self {
            id,
            project_name: project_name.to_string(),
            base_dir: base_dir.to_path_buf(),
            created_at,
        })
    }

    pub fn dir(&self) -> PathBuf {
        self.base_dir.join(".orchestrator").join("sessions").join(&self.id)
    }

    pub fn save_original_prompt(&self, content: &str) -> Result<()> {
        fs::write(self.dir().join("original_prompt.md"), content)?;
        Ok(())
    }

    pub fn save_plan(&self, content: &str) -> Result<()> {
        fs::write(self.dir().join("plan.md"), content)?;
        Ok(())
    }

    pub fn save_integration_instructions(&self, content: &str) -> Result<()> {
        fs::write(self.dir().join("integration.md"), content)?;
        Ok(())
    }

    pub fn save_task_file(&self, task_id: &str, content: &str) -> Result<()> {
        fs::write(self.task_dir().join(format!("{}.md", task_id)), content)?;
        Ok(())
    }

    pub fn save_assignment(&self, agent_name: &str, content: &str) -> Result<()> {
        fs::write(self.assignment_dir().join(format!("{}.md", agent_name)), content)?;
        Ok(())
    }

    pub fn task_dir(&self) -> PathBuf {
        self.dir().join("tasks")
    }

    pub fn assignment_dir(&self) -> PathBuf {
        self.dir().join("assignments")
    }

    pub fn logs_dir(&self) -> PathBuf {
        self.dir().join("logs")
    }

    pub fn project_workspace(&self, agent_name: &str) -> PathBuf {
        self.dir().join("workspace").join(agent_name)
    }
}
