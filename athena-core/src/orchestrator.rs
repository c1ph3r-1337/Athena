use crate::discovery::DiscoveredAgent;
use crate::planner::Planner;
use crate::session::Session;
use crate::state::{OrchestrationPhase, OrchestrationState};
use crate::task::Task;
use crate::templates::TemplateManager;
use crate::validator;
use crate::scheduler;
use anyhow::Result;
use std::collections::HashMap;
use std::path::Path;

/// Events emitted by the orchestrator for UI consumption
#[derive(Debug, Clone)]
pub enum OrchestratorEvent {
    PhaseChanged(OrchestrationPhase),
    TaskStarted(String),
    TaskCompleted(String),
    TaskFailed(String, String),
    AgentLaunched(String),
    AllTasksComplete,
    IntegrationStarted(String),
    IntegrationComplete,
    ValidationResult(bool),
}

/// The Orchestrator manages the full lifecycle of a multi-agent project execution.
pub struct Orchestrator {
    pub planner: Planner,
    pub template_manager: TemplateManager,
    pub session: Option<Session>,
    pub state: Option<OrchestrationState>,
}

impl Orchestrator {
    pub fn new(template_dir: &Path) -> Self {
        Self {
            planner: Planner::new(),
            template_manager: TemplateManager::new(template_dir),
            session: None,
            state: None,
        }
    }

    /// Phase 1: Plan the project
    pub async fn plan(
        &mut self,
        objective: &str,
        agents: &[DiscoveredAgent],
        project_name: &str,
        base_dir: &Path,
    ) -> Result<Vec<Task>> {
        // Create session
        let session = Session::new(project_name, base_dir)?;
        session.save_original_prompt(objective)?;

        println!("  Session:   {}", session.id);
        println!("  Directory: {}", session.dir().display());

        // Plan using real agent
        let tasks = self.planner.plan(objective, agents, project_name).await?;

        // Save plan
        let plan_content = Self::format_plan(&tasks, agents);
        session.save_plan(&plan_content)?;

        // Save individual task files
        for task in &tasks {
            let task_content = format!(
                "# Task: {}\n\n## ID: {}\n\n## Status: {}\n\n## Assigned Agent: {}\n\n## Dependencies: {}\n\n## Description\n\n{}\n\n## Expected Outputs\n\n{}\n",
                task.title,
                task.id,
                task.status,
                task.assigned_agent.as_deref().unwrap_or("Unassigned"),
                if task.dependencies.is_empty() { "None".to_string() } else { task.dependencies.join(", ") },
                task.description,
                if task.expected_outputs.is_empty() { "Not specified".to_string() } else { task.expected_outputs.join("\n- ") },
            );
            session.save_task_file(&task.id, &task_content)?;
        }

        // Initialize orchestration state
        let mut state = OrchestrationState::new(
            session.id.clone(),
            project_name.to_string(),
            objective.to_string(),
        );
        state.tasks = tasks.clone();
        state.agents_used = agents.iter().filter(|a| a.available).map(|a| a.name.clone()).collect();
        state.save(&session.dir())?;

        self.session = Some(session);
        self.state = Some(state);

        Ok(tasks)
    }

    /// Phase 2: Validate the plan before execution
    pub fn validate_plan(&self, tasks: &[Task]) -> bool {
        let result = validator::validate_plan(tasks);
        validator::display_validation(&result);
        result.valid
    }

    /// Phase 3: Generate assignment files and agent workspace directories
    pub fn prepare_assignments(
        &self,
        tasks: &mut [Task],
        _agents: &[DiscoveredAgent],
    ) -> Result<()> {
        let session = self.session.as_ref().ok_or_else(|| anyhow::anyhow!("No active session"))?;

        // First pass: collect agent names and generate assignment files
        let mut agent_names: Vec<String> = Vec::new();
        for task in tasks.iter() {
            let agent = task.assigned_agent.as_deref().unwrap_or("unassigned").to_string();
            if !agent_names.contains(&agent) {
                agent_names.push(agent);
            }
        }

        for agent_name in &agent_names {
            let workspace = session.project_workspace(agent_name);
            std::fs::create_dir_all(&workspace)?;

            // Collect tasks for this agent
            let task_refs: Vec<&Task> = tasks.iter().filter(|t| t.assigned_agent.as_deref() == Some(agent_name)).collect();
            let assignment = TemplateManager::generate_assignment(
                agent_name,
                &task_refs,
                &session.id,
                &session.project_name,
                &workspace.to_string_lossy(),
            );

            session.save_assignment(agent_name, &assignment)?;
        }

        // Second pass: set workspace/prompt/output paths on tasks
        for task in tasks.iter_mut() {
            if let Some(agent) = &task.assigned_agent {
                let workspace = session.project_workspace(agent);
                task.workspace = Some(workspace.to_string_lossy().to_string());

                let prompt_file = session.dir().join("tasks").join(format!("{}.md", task.id));
                task.prompt_file = Some(prompt_file.to_string_lossy().to_string());

                let output_file = session.dir().join("results").join(format!("{}.out", task.id));
                task.output_file = Some(output_file.to_string_lossy().to_string());
            }
        }

        // Generate integration instructions
        let agent_workspaces: Vec<(String, String)> = agent_names.iter().map(|name| {
            (name.clone(), session.project_workspace(name).to_string_lossy().to_string())
        }).collect();

        let integration_md = TemplateManager::generate_integration_instructions(
            tasks,
            &session.project_name,
            &agent_workspaces,
        );
        session.save_integration_instructions(&integration_md)?;

        Ok(())
    }

    /// Generate the bash script for an agent's tmux pane
    pub fn generate_agent_script(
        &self,
        agent_name: &str,
        tasks: &[&Task],
    ) -> Result<String> {
        let session = self.session.as_ref().ok_or_else(|| anyhow::anyhow!("No active session"))?;

        let mut script = format!("#!/bin/bash\nclear\necho -e '\\033[38;2;101;147;47mAgent Terminal: {}\\033[0m\\n'\n", agent_name);

        // Load static agent template
        let template = self.template_manager.load_agent_template(agent_name).unwrap_or_default();

        for task in tasks {
            let assignment_file = session.dir().join("assignments").join(format!("{}.md", agent_name));
            let out_file = task.output_file.as_deref().unwrap_or("/dev/null");
            let workspace = task.workspace.as_deref().unwrap_or(".");

            // Create a comprehensive prompt file combining template + assignment + task
            let full_prompt = format!(
                "{}\n\n---\n\n# Your Assignment\n\n{}\n\n---\n\n# Current Task\n\n## {}\n\n{}\n\n## Workspace\n\nWork inside: {}\n",
                template,
                std::fs::read_to_string(&assignment_file).unwrap_or_default(),
                task.title,
                task.description,
                workspace,
            );

            let prompt_file = format!("/tmp/athena_full_prompt_{}.md", task.id);
            std::fs::write(&prompt_file, &full_prompt)?;

            // Generate the CLI execution command
            let cli_exec = if agent_name.contains("claude") {
                format!(
                    "cd {} || exit 1\n\
                    PROMPT=$(cat {}) || exit 1\n\
                    (while true; do for c in / - \\\\ \\|; do echo -ne \"\\r\\033[38;2;101;147;47mGenerating... $c\\033[0m\"; sleep 0.2; done; done) & SPID=$!\n\
                    claude -p \"$PROMPT\"\n\
                    kill $SPID 2>/dev/null\n\
                    echo -ne \"\\r\\033[K\"",
                    workspace, prompt_file
                )
            } else if agent_name.contains("agy") {
                format!(
                    "cd {} || exit 1\n\
                    PROMPT=$(cat {}) || exit 1\n\
                    (while true; do for c in / - \\\\ \\|; do echo -ne \"\\r\\033[38;2;101;147;47mThinking... $c\\033[0m\"; sleep 0.2; done; done) & SPID=$!\n\
                    agy --dangerously-skip-permissions -p \"$PROMPT\"\n\
                    kill $SPID 2>/dev/null\n\
                    echo -ne \"\\r\\033[K\"",
                    workspace, prompt_file
                )
            } else if agent_name.contains("codex") {
                format!("cd {} || exit 1\nPROMPT=$(cat {})\ncodex exec \"$PROMPT\"", workspace, prompt_file)
            } else {
                format!("echo 'Unknown agent: {}'", agent_name)
            };

            script.push_str(&format!("echo -e '\\033[38;2;101;147;47m> Task:\\033[0m {}'\n", task.title));

            if !task.dependencies.is_empty() {
                script.push_str("(\n");
                script.push_str("  while true; do\n");
                script.push_str("    for c in / - \\\\ \\|; do\n");
                script.push_str("      echo -ne \"\\r\\033[38;2;101;147;47m  Waiting for dependencies... $c\\033[0m\"\n");
                script.push_str("      sleep 0.2\n");
                script.push_str("    done\n");
                script.push_str("  done\n");
                script.push_str(") & SPID_DEP=$!\n");
                
                script.push_str("while true; do\n");
                script.push_str("  all_done=true\n");
                for dep in &task.dependencies {
                    script.push_str(&format!("  if [ ! -f /tmp/athena_task_{}.done ]; then all_done=false; fi\n", dep));
                }
                script.push_str("  if [ \"$all_done\" = true ]; then break; fi\n");
                script.push_str("  sleep 1\n");
                script.push_str("done\n");
                script.push_str("kill $SPID_DEP 2>/dev/null\n");
                script.push_str("echo -ne \"\\r\\033[K\"\n");
                script.push_str("echo -e '\\033[38;2;101;147;47m  - Dependencies met!\\033[0m'\n");
            }

            script.push_str(&format!("{}\n", cli_exec));
            script.push_str(&format!("tmux capture-pane -p -J -S - -E - -t $TMUX_PANE > {}\n", out_file));
            script.push_str(&format!("touch /tmp/athena_task_{}.done\n", task.id));
        }

        script.push_str("echo -e '\\n\\033[32mAgent finished.\\033[0m Signalling Orchestrator...'\nsleep 3\nexit\n");

        Ok(script)
    }

    /// Display the plan summary
    pub fn display_plan(tasks: &[Task], _agents: &[DiscoveredAgent]) {
        // Group tasks by agent
        let mut agent_tasks: HashMap<String, Vec<&Task>> = HashMap::new();
        for task in tasks {
            let agent = task.assigned_agent.as_deref().unwrap_or("Unassigned");
            agent_tasks.entry(agent.to_string()).or_default().push(task);
        }

        let parallel_groups = scheduler::get_parallel_groups(tasks);

        println!("\n\x1b[1m  Orchestration Plan\x1b[0m\n");

        println!("  Tasks:            {}", tasks.len());
        println!("  Agents:           {}", agent_tasks.len());
        println!("  Parallel groups:  {}\n", parallel_groups.len());

        let mut sorted_agents: Vec<_> = agent_tasks.keys().collect();
        sorted_agents.sort();

        for agent_name in sorted_agents {
            let tasks_for_agent = &agent_tasks[agent_name];
            println!("  \x1b[33m{}\x1b[0m", agent_name.to_uppercase());
            for (i, task) in tasks_for_agent.iter().enumerate() {
                let connector = if i == tasks_for_agent.len() - 1 { "└──" } else { "├──" };
                let deps = if task.dependencies.is_empty() {
                    String::new()
                } else {
                    format!(" \x1b[90m(after: {})\x1b[0m", task.dependencies.join(", "))
                };
                println!("   {} {} {}{}",
                    connector,
                    task.id,
                    task.title,
                    deps,
                );
            }
            println!();
        }
    }

    /// Format the plan as markdown for saving
    fn format_plan(tasks: &[Task], agents: &[DiscoveredAgent]) -> String {
        let mut md = String::from("# Orchestration Plan\n\n");
        md.push_str(&format!("## Tasks: {}\n\n", tasks.len()));

        for task in tasks {
            md.push_str(&format!("### {} - {}\n", task.id, task.title));
            md.push_str(&format!("- **Agent**: {}\n", task.assigned_agent.as_deref().unwrap_or("Unassigned")));
            md.push_str(&format!("- **Dependencies**: {}\n", if task.dependencies.is_empty() { "None".to_string() } else { task.dependencies.join(", ") }));
            md.push_str(&format!("- **Description**: {}\n\n", task.description));
        }

        md.push_str("## Available Agents\n\n");
        for agent in agents.iter().filter(|a| a.available) {
            md.push_str(&format!("- **{}** ({}): {}\n", agent.name, agent.command, agent.capabilities.join(", ")));
        }

        md
    }
}
