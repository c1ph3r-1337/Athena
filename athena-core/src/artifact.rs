use std::path::{Path, PathBuf};
use std::fs;
use anyhow::{Context, Result};
use uuid::Uuid;

#[derive(Debug, Clone)]
pub struct Artifact {
    pub id: String,
    pub name: String,
    pub file_path: PathBuf,
    pub producer_task_id: Option<String>,
}

pub struct ArtifactManager {
    workspace_dir: PathBuf,
    artifacts: Vec<Artifact>,
}

impl ArtifactManager {
    pub fn new(workspace_dir: impl AsRef<Path>) -> Result<Self> {
        let dir = workspace_dir.as_ref().to_path_buf();
        if !dir.exists() {
            fs::create_dir_all(&dir)
                .with_context(|| format!("Failed to create workspace dir: {:?}", dir))?;
        }
        Ok(Self {
            workspace_dir: dir,
            artifacts: Vec::new(),
        })
    }

    pub fn register_artifact(&mut self, name: &str, content: &str, task_id: Option<String>) -> Result<Artifact> {
        let id = Uuid::new_v4().to_string();
        // use a simplified path: workspace_dir / {id}_{name}
        let file_name = format!("{}_{}", &id[0..8], name);
        let file_path = self.workspace_dir.join(&file_name);
        
        fs::write(&file_path, content)
            .with_context(|| format!("Failed to write artifact to {:?}", file_path))?;

        let artifact = Artifact {
            id,
            name: name.to_string(),
            file_path,
            producer_task_id: task_id,
        };

        self.artifacts.push(artifact.clone());
        tracing::info!("Registered new artifact: {} at {:?}", name, artifact.file_path);
        
        Ok(artifact)
    }

    pub fn get_artifact(&self, id: &str) -> Option<&Artifact> {
        self.artifacts.iter().find(|a| a.id == id)
    }
}
