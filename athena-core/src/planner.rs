use crate::discovery::DiscoveredAgent;
use crate::task::Task;
use anyhow::Result;

/// The Planner generates a task decomposition from a user objective.
/// It uses a real agent (agy/claude/codex) to analyze the objective and produce tasks.
pub struct Planner;

impl Planner {
    pub fn new() -> Self {
        Self
    }

    /// Generate a task plan from the objective using a real planning agent.
    /// Falls back to mock planning if no agent is available.
    pub async fn plan(
        &self,
        objective: &str,
        available_agents: &[DiscoveredAgent],
        project_name: &str,
    ) -> Result<Vec<Task>> {
        // Build the agent context for the planner
        let agent_context = Self::build_agent_context(available_agents);

        let planning_prompt = format!(
r#"You are the Meta-Planner for Athena, a multi-agent orchestration runtime.

Your job is to analyze the user's project objective and decompose it into logically independent tasks that can be assigned to different agents.

## Available Agents
{agent_context}

## User's Project Objective
{objective}

## Instructions
1. Analyze the project requirements.
2. Identify logically independent components.
3. Determine which agent is best suited for each component based on their capabilities.
4. Create a task DAG (Directed Acyclic Graph) with proper dependencies.
5. Each task should be a meaningful unit of work, not an arbitrary split.

## Output Format
Respond with ONLY a JSON object in this exact format:
{{
  "tasks": [
    {{
      "id": "task_1",
      "title": "Short descriptive title",
      "description": "Detailed description of what needs to be done. Include specific requirements, expected files to create, and any technical details.",
      "assigned_agent": "agent_name",
      "depends_on": [],
      "expected_outputs": ["file1.rs", "file2.rs"]
    }},
    {{
      "id": "task_2",
      "title": "Another task",
      "description": "This task depends on task_1",
      "assigned_agent": "another_agent",
      "depends_on": ["task_1"],
      "expected_outputs": ["file3.rs"]
    }}
  ]
}}

Important:
- Use the actual agent names from the available agents list above.
- Tasks with no dependencies can run in parallel.
- Be specific in descriptions - agents need clear instructions.
- Keep the number of tasks reasonable (2-8 for most projects).
"#
        );

        // Try to use a real agent for planning
        let planning_result = self.invoke_planning_agent(&planning_prompt, available_agents).await;

        match planning_result {
            Ok(response) => self.parse_plan_response(&response, project_name),
            Err(e) => {
                tracing::warn!("Planning agent failed ({}), using mock plan", e);
                Ok(self.mock_plan(available_agents, project_name))
            }
        }
    }

    /// Try to use a real agent (preferring agy, then claude, then codex) for planning
    async fn invoke_planning_agent(
        &self,
        prompt: &str,
        agents: &[DiscoveredAgent],
    ) -> Result<String> {
        use tokio::io::{AsyncBufReadExt, BufReader};

        // Priority: agy > claude > codex
        let preferred_order = ["agy", "claude", "codex"];

        for preferred in &preferred_order {
            if let Some(agent) = agents.iter().find(|a| a.available && a.command.contains(preferred)) {
                println!("\x1b[38;2;101;147;47mPlan({})\x1b[0m", agent.name);
                println!("  Waiting for response (this may take 30-90 seconds)...\n");

                // Spawn a progress ticker while waiting for agent to initialize
                let ticker = tokio::spawn(async {
                    let frames = ['⠋', '⠙', '⠹', '⠸', '⠼', '⠴', '⠦', '⠧', '⠇', '⠏'];
                    let mut i = 0;
                    loop {
                        eprint!("\r  {} Planning...", frames[i % frames.len()]);
                        i += 1;
                        tokio::time::sleep(tokio::time::Duration::from_millis(200)).await;
                    }
                });

                // Write prompt to temp file to avoid shell argument length limits
                let prompt_file = "/tmp/athena_planning_prompt.md";
                std::fs::write(prompt_file, prompt)?;

                let mut child = match agent.command.as_str() {
                    cmd if cmd.contains("agy") => {
                        tokio::process::Command::new("bash")
                            .args(["-c", &format!("agy --dangerously-skip-permissions -p \"$(< {prompt_file})\"")])
                            .stdout(std::process::Stdio::piped())
                            .stderr(std::process::Stdio::piped())
                            .spawn()?
                    }
                    cmd if cmd.contains("claude") => {
                        tokio::process::Command::new("bash")
                            .args(["-c", &format!("claude -p \"$(< {prompt_file})\"")])
                            .stdout(std::process::Stdio::piped())
                            .stderr(std::process::Stdio::piped())
                            .spawn()?
                    }
                    cmd if cmd.contains("codex") => {
                        tokio::process::Command::new("bash")
                            .args(["-c", &format!("script -q -e -c \"codex \\\"$(< {prompt_file})\\\"\" /dev/null")])
                            .stdout(std::process::Stdio::piped())
                            .stderr(std::process::Stdio::piped())
                            .spawn()?
                    }
                    _ => continue,
                };

                // Stream stdout live while capturing it
                let mut captured_output = String::new();
                let mut first_line = true;

                if let Some(stdout) = child.stdout.take() {
                    let mut reader = BufReader::new(stdout).lines();
                    
                    while let Some(line) = reader.next_line().await? {
                        if first_line {
                            // Stop the spinner and clear its line when output starts
                            ticker.abort();
                            eprint!("\r                              \r");
                            println!("  \x1b[90m┌─── {} output ───\x1b[0m", agent.name);
                            first_line = false;
                        }
                        println!("  \x1b[90m│\x1b[0m {}", line);
                        captured_output.push_str(&line);
                        captured_output.push('\n');
                    }
                    if !first_line {
                        println!("  \x1b[90m└────────────────────\x1b[0m\n");
                    }
                }
                
                // If it never printed anything, make sure spinner is aborted
                if first_line {
                    ticker.abort();
                    eprint!("\r                              \r");
                }

                let status = child.wait().await?;

                if status.success() && !captured_output.trim().is_empty() {
                    println!("  \x1b[32m✓\x1b[0m Planning complete!\n");
                    return Ok(captured_output);
                }

                println!("  \x1b[33m▸ {} returned empty/failed, trying next agent...\x1b[0m", agent.name);
            }
        }

        anyhow::bail!("No planning agent available")
    }

    /// Parse the JSON response from the planning agent into Task objects
    fn parse_plan_response(&self, response: &str, _project_name: &str) -> Result<Vec<Task>> {
        // Try to extract JSON from the response (agent might wrap it in markdown)
        let json_str = Self::extract_json(response);

        #[derive(serde::Deserialize)]
        struct PlanResponse {
            tasks: Vec<TaskDef>,
        }

        #[derive(serde::Deserialize)]
        struct TaskDef {
            id: String,
            title: String,
            description: String,
            assigned_agent: Option<String>,
            depends_on: Vec<String>,
            expected_outputs: Option<Vec<String>>,
        }

        let plan_data: PlanResponse = serde_json::from_str(&json_str)
            .map_err(|e| anyhow::anyhow!("Failed to parse planner output: {}. Raw: {}", e, &json_str[..json_str.len().min(200)]))?;

        // Map abstract string IDs to stable IDs
        let mut id_map = std::collections::HashMap::new();
        let mut final_tasks = Vec::new();

        for tdef in &plan_data.tasks {
            let stable_id = format!("T{:03}", final_tasks.len() + 1);
            id_map.insert(tdef.id.clone(), stable_id.clone());

            let mut task = Task::new(stable_id, tdef.title.clone(), tdef.description.clone());
            task.assigned_agent = tdef.assigned_agent.clone();
            task.expected_outputs = tdef.expected_outputs.clone().unwrap_or_default();
            final_tasks.push((task, tdef.depends_on.clone()));
        }

        let mut tasks_to_return = Vec::new();
        for (mut task, deps) in final_tasks {
            for d in deps {
                if let Some(real_id) = id_map.get(&d) {
                    task.dependencies.push(real_id.clone());
                }
            }
            tasks_to_return.push(task);
        }

        Ok(tasks_to_return)
    }

    /// Extract JSON from a potentially markdown-wrapped response
    fn extract_json(text: &str) -> String {
        // Try to find JSON in code blocks first
        if let Some(start) = text.find("```json") {
            let json_start = start + 7;
            if let Some(end) = text[json_start..].find("```") {
                return text[json_start..json_start + end].trim().to_string();
            }
        }
        if let Some(start) = text.find("```") {
            let json_start = start + 3;
            // Skip the language identifier line
            let json_start = text[json_start..].find('\n').map(|n| json_start + n + 1).unwrap_or(json_start);
            if let Some(end) = text[json_start..].find("```") {
                return text[json_start..json_start + end].trim().to_string();
            }
        }
        // Try to find raw JSON object
        if let Some(start) = text.find('{') {
            if let Some(end) = text.rfind('}') {
                return text[start..=end].to_string();
            }
        }
        text.to_string()
    }

    /// Build a context string describing available agents
    fn build_agent_context(agents: &[DiscoveredAgent]) -> String {
        let mut ctx = String::new();
        for (i, agent) in agents.iter().filter(|a| a.available).enumerate() {
            ctx.push_str(&format!(
                "{}. {}\n   Command: {}\n   Version: {}\n   Capabilities: {}\n   Description: {}\n\n",
                i + 1,
                agent.name,
                agent.command,
                agent.version.as_deref().unwrap_or("unknown"),
                agent.capabilities.join(", "),
                agent.description,
            ));
        }
        if ctx.is_empty() {
            ctx = "No agents currently available.\n".to_string();
        }
        ctx
    }

    /// Generate a reasonable mock plan when no planning agent is available
    fn mock_plan(&self, agents: &[DiscoveredAgent], _project_name: &str) -> Vec<Task> {
        let available: Vec<&DiscoveredAgent> = agents.iter().filter(|a| a.available).collect();

        let agent1 = available.first().map(|a| a.command.clone()).unwrap_or_else(|| "agy".to_string());
        let agent2 = available.get(1).map(|a| a.command.clone()).unwrap_or_else(|| agent1.clone());

        let mut t1 = Task::new(
            "T001".to_string(),
            "Architecture & Core Implementation".to_string(),
            "Design and implement the core architecture, data models, and primary business logic.".to_string(),
        );
        t1.assigned_agent = Some(agent1);

        let mut t2 = Task::new(
            "T002".to_string(),
            "Testing, Security & Documentation".to_string(),
            "Write comprehensive tests, perform security review, and create documentation.".to_string(),
        );
        t2.assigned_agent = Some(agent2);
        t2.dependencies = vec!["T001".to_string()];

        vec![t1, t2]
    }
}
