use anyhow::{Context, Result};
use std::path::{Path, PathBuf};
use crate::task::Task;

pub struct TemplateManager {
    template_dir: PathBuf,
}

impl TemplateManager {
    pub fn new(template_dir: &Path) -> Self {
        Self {
            template_dir: template_dir.to_path_buf(),
        }
    }

    pub fn load_agent_template(&self, agent_name: &str) -> Result<String> {
        let name_lower = agent_name.to_lowercase();
        let filename = if name_lower.contains("agy") {
            "agy.md"
        } else if name_lower.contains("codex") {
            "codex.md"
        } else if name_lower.contains("claude") {
            "claude.md"
        } else {
            return Ok("Generic Agent Operating Instructions\n\nFollow your assignment carefully and output results cleanly.".to_string());
        };

        let path = self.template_dir.join(filename);
        if path.exists() {
            std::fs::read_to_string(&path).context(format!("Failed to read template file: {:?}", path))
        } else {
            Ok("Generic Agent Operating Instructions\n\nFollow your assignment carefully and output results cleanly.".to_string())
        }
    }

    pub fn generate_assignment(
        agent_name: &str,
        tasks: &[&Task],
        session_id: &str,
        project_name: &str,
        workspace: &str,
    ) -> String {
        let mut out = String::new();
        out.push_str(&format!("# Task Assignment: {}\n\n", agent_name));
        out.push_str(&format!("**Project:** {}\n", project_name));
        out.push_str(&format!("**Session ID:** {}\n", session_id));
        out.push_str(&format!("**Workspace:** {}\n\n", workspace));
        out.push_str("## Role Description\n");
        out.push_str("You are responsible for completing the tasks listed below. Work strictly within your assigned workspace.\n\n");
        
        out.push_str("## Assigned Tasks\n\n");
        for task in tasks {
            out.push_str(&format!("### Task {}: {}\n", task.id, task.title));
            out.push_str(&format!("**Description:** {}\n", task.description));
            if !task.dependencies.is_empty() {
                out.push_str(&format!("**Dependencies:** {}\n", task.dependencies.join(", ")));
            }
            if !task.expected_outputs.is_empty() {
                out.push_str("**Expected Outputs:**\n");
                for output in &task.expected_outputs {
                    out.push_str(&format!("- {}\n", output));
                }
            }
            out.push_str("\n");
        }
        
        out.push_str("## Allowed Workspace Path\n");
        out.push_str(&format!("All files must be created within: {}\n\n", workspace));
        out.push_str("## Completion Criteria\n");
        out.push_str("1. All expected outputs are present and correct.\n");
        out.push_str("2. Code compiles and runs without errors.\n");
        out.push_str("3. Exit cleanly upon finishing.\n");
        
        out
    }

    pub fn generate_integration_instructions(
        tasks: &[Task],
        project_name: &str,
        agent_workspaces: &[(String, String)],
    ) -> String {
        let mut out = String::new();
        out.push_str(&format!("# Integration Instructions for Project: {}\n\n", project_name));
        
        out.push_str("## Original Project Requirements Summary\n");
        out.push_str("Integrate the outputs of all agents to produce the final deliverable.\n\n");
        
        out.push_str("## Agent Workspaces\n\n");
        for (agent, workspace) in agent_workspaces {
            out.push_str(&format!("- **{}**: {}\n", agent, workspace));
        }
        out.push_str("\n");
        
        out.push_str("## Assigned Work\n\n");
        for task in tasks {
            let assignee = task.assigned_agent.as_deref().unwrap_or("Unassigned");
            out.push_str(&format!("- **Task {}** ({}) assigned to: {}\n", task.id, task.title, assignee));
        }
        out.push_str("\n");

        out.push_str("## Dependencies Between Work\n\n");
        for task in tasks {
            if !task.dependencies.is_empty() {
                out.push_str(&format!("- Task {} depends on: {}\n", task.id, task.dependencies.join(", ")));
            }
        }
        out.push_str("\n");

        out.push_str("## Artifacts Produced\n\n");
        for task in tasks {
            if !task.expected_outputs.is_empty() {
                out.push_str(&format!("- From Task {}:\n", task.id));
                for output in &task.expected_outputs {
                    out.push_str(&format!("  - {}\n", output));
                }
            }
        }
        out.push_str("\n");

        out.push_str("## CRITICAL INTEGRATION INSTRUCTIONS\n");
        out.push_str("You are the final integration agent. You must output the fully merged code for the final project.\n\n");
        out.push_str("1. **READ**: Use your tools to read the generated code files from the Agent Workspaces listed above.\n");
        out.push_str("2. **MERGE**: Combine their work, resolving any import/dependency issues into a single clean project.\n");
        out.push_str("3. **WRITE**: DO NOT USE FILE-WRITING TOOLS. Instead, you MUST output the final code directly in this chat as Markdown code blocks.\n");
        out.push_str("   - The VERY FIRST LINE inside each code block MUST be a comment with the exact filename (e.g. `# main.py` or `// src/app.rs`).\n");
        out.push_str("   - Do not skip any files. Provide the complete code for every file needed.\n");
        out.push_str("4. **VALIDATE**: Ensure the final code is complete and cross-agent functionality works.\n\n");
        out.push_str("If you do not output Markdown code blocks with filenames, the final project will be completely empty!\n");
        
        out
    }
}
