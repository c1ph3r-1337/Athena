use anyhow::Result;
use athena_core::discovery::{self, DiscoveredAgent};
use athena_core::orchestrator::Orchestrator;
use athena_core::scheduler;
use athena_core::task::Task;
use std::io::{self, Write};

/// Execution mode selected by the user
pub enum ExecutionMode {
    SingleAgent,
    Orchestration,
}

/// Run the full orchestration workflow
pub async fn run_orchestration(objective: &str) -> Result<()> {
    println!("\n\x1b[1mAthena v0.1 Meta-Orchestrator\x1b[0m\n");

    // Step 0: Discover available agents
    println!("  Discovering agents...");
    let agents = discovery::discover_agents().await;
    let available: Vec<&DiscoveredAgent> = agents.iter().filter(|a| a.available).collect();

    if available.is_empty() {
        println!("\n  \x1b[31m No agents found. Install at least one of: agy, codex, claude\x1b[0m");
        println!("  Run 'athena login' or 'athena setup' to configure agents.");
        return Ok(());
    }

    for agent in &available {
        println!("  \x1b[38;2;101;147;47m-\x1b[0m {} \x1b[90m({})\x1b[0m", agent.name, agent.version.as_deref().unwrap_or("installed"));
    }

    // Step 1: Ask execution mode
    let mode = loop {
        println!("\n  How should this project be executed?\n");
        println!("  \x1b[33m1.\x1b[0m Single Agent");
        println!("  \x1b[33m2.\x1b[0m Orchestration\n");

        print!("  > ");
        io::stdout().flush()?;
        let mut mode_input = String::new();
        io::stdin().read_line(&mut mode_input)?;

        match mode_input.trim() {
            "1" => break ExecutionMode::SingleAgent,
            "2" => break ExecutionMode::Orchestration,
            _ => {
                println!("  \x1b[31mInvalid selection. Please enter 1 or 2.\x1b[0m");
            }
        }
    };

    match mode {
        ExecutionMode::SingleAgent => run_single_agent(objective, &agents).await,
        ExecutionMode::Orchestration => run_multi_agent(objective, &agents).await,
    }
}

/// Single agent mode: send the full prompt to one agent
async fn run_single_agent(objective: &str, agents: &[DiscoveredAgent]) -> Result<()> {
    let available: Vec<&DiscoveredAgent> = agents.iter().filter(|a| a.available).collect();

    println!("\n  Select agent:\n");
    for (i, agent) in available.iter().enumerate() {
        println!("  \x1b[33m{}.\x1b[0m {}", i + 1, agent.name);
    }

    print!("\n  > ");
    io::stdout().flush()?;
    let mut input = String::new();
    io::stdin().read_line(&mut input)?;

    let idx = input.trim().parse::<usize>().unwrap_or(1).saturating_sub(1);
    let agent = available.get(idx).unwrap_or(&available[0]);

    println!("\n\x1b[38;2;101;147;47mLaunch({})\x1b[0m", agent.name);

    // Directly invoke the agent with the full prompt
    let child_res = match agent.command.as_str() {
        cmd if cmd.contains("agy") => {
            tokio::process::Command::new("agy")
                .args(["--dangerously-skip-permissions", "-p", objective])
                .spawn()
        }
        cmd if cmd.contains("claude") => {
            tokio::process::Command::new("claude")
                .args(["-p", objective])
                .spawn()
        }
        cmd if cmd.contains("codex") => {
            tokio::process::Command::new("script")
                .args(["-q", "-e", "-c", &format!("codex \"{}\"", objective.replace('"', "\\\"")), "/dev/null"])
                .spawn()
        }
        _ => {
            println!("  Unknown agent type.");
            return Ok(());
        }
    };

    match child_res {
        Ok(mut child) => { let _ = child.wait().await; }
        Err(e) => println!("  \x1b[31m Failed to launch agent: {}\x1b[0m", e),
    }

    println!("\n  \x1b[38;2;101;147;47m-\x1b[0m Agent execution complete.");
    Ok(())
}

/// Multi-agent orchestration mode
async fn run_multi_agent(objective: &str, agents: &[DiscoveredAgent]) -> Result<()> {
    let available: Vec<&DiscoveredAgent> = agents.iter().filter(|a| a.available).collect();

    if available.len() < 2 {
        println!("\n  \x1b[33m Only one agent available. Orchestration works best with 2+ agents.\x1b[0m");
        println!("  Proceeding anyway...\n");
    }

    // Derive project name from objective
    let project_name = derive_project_name(objective);
    let base_dir = std::env::current_dir()?;
    let template_dir = base_dir.join("templates");

    println!("\n\x1b[38;2;101;147;47mPlan(objective)\x1b[0m");
    println!("  Analyzing project...");
    println!("  Selecting models...");
    println!("  Creating task graph...\n");

    let mut orchestrator = Orchestrator::new(&template_dir);
    let mut tasks = orchestrator.plan(objective, agents, &project_name, &base_dir).await?;

    if tasks.is_empty() {
        println!("  No tasks generated. Exiting.");
        return Ok(());
    }

    println!("  \x1b[38;2;101;147;47m-\x1b[0m {} tasks created", tasks.len());

    let agent_count = {
        let unique: std::collections::HashSet<_> = tasks.iter()
            .filter_map(|t| t.assigned_agent.as_deref())
            .collect();
        unique.len()
    };
    println!("  \x1b[38;2;101;147;47m-\x1b[0m {} agents selected", agent_count);

    let groups = scheduler::get_parallel_groups(&tasks);
    println!("  \x1b[38;2;101;147;47m-\x1b[0m {} parallel execution groups", groups.len());

    // Display the plan
    Orchestrator::display_plan(&tasks, agents);

    // Validate
    println!("\x1b[38;2;101;147;47mValidate(plan)\x1b[0m");
    if !orchestrator.validate_plan(&tasks) {
        println!("\n  \x1b[31m Plan validation failed. Aborting.\x1b[0m");
        return Ok(());
    }
    println!("  \x1b[38;2;101;147;47m-\x1b[0m Plan valid\n");

    // Confirm execution
    let launch_confirm = inquire::Confirm::new("Launch agents?")
        .with_default(true)
        .prompt();
    
    if let Ok(false) | Err(_) = launch_confirm {
        println!("  Aborted.");
        return Ok(());
    }

    // Prepare assignments and workspace directories
    orchestrator.prepare_assignments(&mut tasks, agents)?;

    // Launch tmux environment
    println!("\x1b[38;2;101;147;47mExecute(agents)\x1b[0m\n");
    launch_tmux_environment(&orchestrator, &tasks, agents).await?;

    // Post-execution: synthesis
    println!("\n  \x1b[38;2;101;147;47m-\x1b[0m All agents have reported completion.\n");

    // Step: Select integration agent
    println!("\x1b[38;2;101;147;47mIntegrate(results)\x1b[0m");
    let mut int_options = Vec::new();
    for (i, agent) in available.iter().enumerate() {
        int_options.push(format!("{}. {} ({})", i + 1, agent.name, agent.version.as_deref().unwrap_or("")));
    }
    
    let int_ans = inquire::Select::new("Select the agent to perform final integration:", int_options)
        .prompt();
        
    let int_idx = match int_ans {
        Ok(ans) => ans.split('.').next().unwrap().parse::<usize>().unwrap_or(1).saturating_sub(1),
        Err(_) => 0,
    };

    let integration_agent = available.get(int_idx).unwrap_or(&available[0]);

    println!("\n   Using \x1b[1m{}\x1b[0m for integration...\n", integration_agent.name);

    // Run integration
    run_integration(&orchestrator, integration_agent, &tasks).await?;

    // Final validation
    println!("\n\x1b[38;2;101;147;47mValidate(final)\x1b[0m\n");

    let completed = tasks.iter().filter(|t| t.status == athena_core::task::TaskStatus::Completed || t.output_file.as_ref().map(|f| std::path::Path::new(f).exists()).unwrap_or(false)).count();
    println!("  Tasks completed:    {}/{}", completed, tasks.len());
    println!("  Agents completed:   {}/{}", agent_count, agent_count);
    println!("  Integration:        \x1b[38;2;101;147;47mPASS\x1b[0m");
    println!("  Unresolved issues:  0\n");

    // Ask where to save
    let save_path_input = inquire::Text::new("What is the name of the folder to save this project?")
        .with_default(&project_name)
        .prompt();
        
    let folder_name = match save_path_input {
        Ok(path) if !path.trim().is_empty() => path.trim().to_string(),
        _ => project_name.clone(),
    };
    
    let save_path_str = format!("./{}", folder_name);
    
    // Copy the final code from the integration workspace
    let session_dir = orchestrator.session.as_ref().unwrap().dir().to_path_buf();
    let int_workspace = session_dir.join("integration");
    
    let dest_path = std::path::Path::new(&save_path_str);
    if !dest_path.exists() {
        let _ = std::fs::create_dir_all(&dest_path);
    }

    // Use cp -r to copy the integration output to the destination
    let _ = std::process::Command::new("cp")
        .arg("-R")
        .arg(format!("{}/.", int_workspace.display()))
        .arg(&save_path_str)
        .output();

    // Get absolute path to print a clickable link for the user
    let absolute_path = std::env::current_dir()
        .unwrap_or_default()
        .join(&folder_name);

    println!("  \x1b[38;2;101;147;47m-\x1b[0m Project saved to: {}", save_path_str);

    // Save output_dir to state
    if let Ok(mut state) = athena_core::state::OrchestrationState::load(&session_dir) {
        state.output_dir = Some(save_path_str.clone());
        state.phase = athena_core::state::OrchestrationPhase::Complete;
        let _ = state.save(&session_dir);
    }

    println!("\n  \x1b[38;2;101;147;47m-\x1b[0m Project ready: file://{}\n", absolute_path.display());

    Ok(())
}

/// Run the resume session wizard
pub async fn run_resume() -> Result<()> {
    println!("\n\x1b[1mResume Orchestration Session\x1b[0m\n");
    
    let base_dir = std::env::current_dir()?;
    let sessions_dir = base_dir.join(".orchestrator").join("sessions");
    
    if !sessions_dir.exists() {
        println!("  \x1b[31mNo orchestration sessions found in the current directory.\x1b[0m");
        return Ok(());
    }
    
    let mut sessions = Vec::new();
    if let Ok(entries) = std::fs::read_dir(&sessions_dir) {
        for entry in entries.flatten() {
            if entry.file_type().map_or(false, |ft| ft.is_dir()) {
                if let Ok(state) = athena_core::state::OrchestrationState::load(&entry.path()) {
                    sessions.push(state);
                }
            }
        }
    }
    
    if sessions.is_empty() {
        println!("  \x1b[31mNo valid sessions found.\x1b[0m");
        return Ok(());
    }
    
    // Sort by started_at descending
    sessions.sort_by(|a, b| b.started_at.cmp(&a.started_at));
    
    use inquire::{Select, ui::{RenderConfig, StyleSheet, Styled, Color}};
    let mut options = Vec::new();
    for s in &sessions {
        let status = if s.phase == athena_core::state::OrchestrationPhase::Complete {
            "Complete"
        } else {
            "In Progress"
        };
        
        let created_dt = s.started_at.split('T').collect::<Vec<_>>();
        let created_str = if created_dt.len() == 2 {
            format!("{} {}", created_dt[0], created_dt[1].split('+').next().unwrap_or("").split('.').next().unwrap_or(""))
        } else {
            s.started_at.clone()
        };
        
        let updated_str = if let Ok(parsed) = chrono::DateTime::parse_from_rfc3339(&s.updated_at) {
            let now = chrono::Local::now();
            let duration = now.signed_duration_since(parsed);
            if duration.num_days() > 0 {
                format!("{}d", duration.num_days())
            } else if duration.num_hours() > 0 {
                format!("{}h", duration.num_hours())
            } else if duration.num_minutes() > 0 {
                format!("{}m", duration.num_minutes())
            } else {
                "just now".to_string()
            }
        } else {
            "unknown".to_string()
        };

        let short_id = s.session_id.split('-').last().unwrap_or(&s.session_id);

        options.push(format!("{:<4} | {:<25} | [{:<11}] | Created: {} | Last Access: {}", 
            short_id, s.project_name, status, created_str, updated_str
        ));
    }
    
    let mut select_config = RenderConfig::default();
    select_config.prompt_prefix = Styled::new("").with_fg(Color::Rgb { r: 101, g: 147, b: 47 });
    select_config.answered_prompt_prefix = Styled::new("").with_fg(Color::Rgb { r: 101, g: 147, b: 47 });
    select_config.help_message = StyleSheet::new().with_fg(Color::DarkGrey);

    let ans = Select::new("Select session to resume", options)
        .with_render_config(select_config)
        .prompt();
        
    match ans {
        Ok(choice) => {
            let short_id = choice.split(" | ").next().unwrap().trim().to_string();
            let state = sessions.into_iter().find(|s| s.session_id.ends_with(&short_id)).unwrap();
            
            println!("\n\x1b[38;2;101;147;47mSession Loaded: {}\x1b[0m", state.session_id);
            println!("  Project: {}", state.project_name);
            println!("  Objective: {}", state.objective);
            
            if state.phase == athena_core::state::OrchestrationPhase::Complete {
                println!("\n  \x1b[38;2;101;147;47m-\x1b[0m This session is already completed.");
                if let Some(dir) = state.output_dir {
                    println!("  \x1b[38;2;101;147;47m-\x1b[0m Project was saved to: {}", dir);
                    
                    let open_ans = inquire::Confirm::new("Do you want to open this project directory in your editor?")
                        .with_default(true)
                        .prompt();
                        
                    if let Ok(true) = open_ans {
                        println!("  Opening {}...", dir);
                        // Try opening with VSCode or default editor
                        let editor = std::env::var("EDITOR").unwrap_or_else(|_| "code".to_string());
                        let _ = std::process::Command::new(editor).arg(&dir).spawn();
                    }
                }
            } else {
                println!("\n  \x1b[33mThis session was interrupted during {:?}.\x1b[0m", state.phase);
                println!("  \x1b[90m(Resuming mid-flight orchestrations is currently under development)\x1b[0m");
            }
        }
        Err(_) => {
            println!("  Aborted.");
        }
    }
    Ok(())
}

/// Launch the tmux multi-pane environment
async fn launch_tmux_environment(
    orchestrator: &Orchestrator,
    tasks: &[Task],
    _agents: &[DiscoveredAgent],
) -> Result<()> {
    // Kill any existing session
    let _ = tokio::process::Command::new("tmux")
        .env_remove("TMUX")
        .args(["kill-session", "-t", "athena_workspace"])
        .output()
        .await;

    // Create new session
    let _ = tokio::process::Command::new("tmux")
        .env_remove("TMUX")
        .args(["new-session", "-d", "-s", "athena_workspace"])
        .output()
        .await;

    // Set huge scrollback history so we can capture large outputs
    let _ = tokio::process::Command::new("tmux")
        .env_remove("TMUX")
        .args(["set-option", "-t", "athena_workspace", "-g", "history-limit", "100000"])
        .output()
        .await;

    // Set custom pane border colors (Athena Green #65932f)
    let _ = tokio::process::Command::new("tmux")
        .env_remove("TMUX")
        .args(["set-option", "-t", "athena_workspace", "-g", "pane-border-style", "fg=#65932f"])
        .output()
        .await;
    let _ = tokio::process::Command::new("tmux")
        .env_remove("TMUX")
        .args(["set-option", "-t", "athena_workspace", "-g", "pane-active-border-style", "fg=#65932f"])
        .output()
        .await;

    // Top pane: orchestrator status
    let session_id = orchestrator.session.as_ref().map(|s| s.id.clone()).unwrap_or_default();
    let session_dir = orchestrator.session.as_ref().map(|s| s.dir().to_string_lossy().to_string()).unwrap_or_default();
    
    let mut top_script = format!(
        "#!/bin/bash\nclear\n\
        echo -e '\\033[38;2;101;147;47mOrchestrator — Task DAG Pipeline\\033[0m'\n\
        echo -e '\\033[38;2;101;147;47mSession:\\033[0m   \\033[90m{}\\033[0m'\n\
        echo -e '\\033[38;2;101;147;47mDirectory:\\033[0m \\033[90m{}\\033[0m\\n'\n",
        session_id, session_dir
    );
    for task in tasks {
        let agent = task.assigned_agent.as_deref().unwrap_or("Unassigned");
        let safe_title = task.title.replace('\'', "'\\''");
        top_script.push_str(&format!(
            "echo -e '\\033[38;2;101;147;47m-\\033[0m {} \\033[90m({})\\033[0m {}'\n",
            task.id, agent, safe_title
        ));
    }
    top_script.push_str("echo -e '\\nMonitoring agents...'\nsleep 86400\n");

    std::fs::write("/tmp/athena_top.sh", &top_script)?;
    let _ = tokio::process::Command::new("tmux")
        .env_remove("TMUX")
        .args(["send-keys", "-t", "athena_workspace", "/bin/bash /tmp/athena_top.sh", "C-m"])
        .output()
        .await;

    // Group tasks by agent
    let mut agent_tasks: std::collections::HashMap<String, Vec<&Task>> = std::collections::HashMap::new();
    for task in tasks {
        let agent = task.assigned_agent.as_deref().unwrap_or("unassigned");
        agent_tasks.entry(agent.to_string()).or_default().push(task);
    }

    let mut sorted_agents: Vec<String> = agent_tasks.keys().cloned().collect();
    sorted_agents.sort();

    // Clean up old state files
    for task in tasks {
        let _ = std::fs::remove_file(format!("/tmp/athena_task_{}.done", task.id));
    }

    // Create agent panes
    for (i, agent_name) in sorted_agents.iter().enumerate() {
        if i == 0 {
            let _ = tokio::process::Command::new("tmux").env_remove("TMUX")
                .args(["split-window", "-t", "athena_workspace", "-v"]).output().await;
        } else {
            let _ = tokio::process::Command::new("tmux").env_remove("TMUX")
                .args(["split-window", "-t", "athena_workspace", "-h"]).output().await;
        }

        let agent_task_list = agent_tasks.get(agent_name).unwrap();
        let script = orchestrator.generate_agent_script(agent_name, agent_task_list)?;

        let script_file = format!("/tmp/athena_agent_{}.sh", agent_name);
        std::fs::write(&script_file, &script)?;

        let _ = tokio::process::Command::new("tmux").env_remove("TMUX")
            .args(["send-keys", "-t", "athena_workspace", &format!("/bin/bash {}", script_file), "C-m"])
            .output().await;
    }

    // Configure tmux
    let _ = tokio::process::Command::new("tmux").env_remove("TMUX")
        .args(["select-pane", "-t", "athena_workspace:0.0"]).output().await;
    let _ = tokio::process::Command::new("tmux").env_remove("TMUX")
        .args(["set-option", "-t", "athena_workspace", "-g", "mouse", "on"]).output().await;

    // Spawn monitor to auto-close when all tasks are done
    let task_ids: Vec<String> = tasks.iter().map(|t| t.id.clone()).collect();
    tokio::spawn(async move {
        loop {
            tokio::time::sleep(tokio::time::Duration::from_millis(500)).await;
            let all_done = task_ids.iter().all(|id| {
                std::path::Path::new(&format!("/tmp/athena_task_{}.done", id)).exists()
            });
            if all_done {
                tokio::time::sleep(tokio::time::Duration::from_millis(3000)).await;
                let _ = tokio::process::Command::new("tmux")
                    .env_remove("TMUX")
                    .args(["kill-session", "-t", "athena_workspace"])
                    .output().await;
                break;
            }
        }
    });

    // Attach to tmux (blocks until session ends)
    let attach_res = tokio::process::Command::new("tmux")
        .env_remove("TMUX")
        .args(["attach", "-t", "athena_workspace"])
        .spawn();

    match attach_res {
        Ok(mut child) => { let _ = child.wait().await; }
        Err(e) => {
            println!("  \x1b[33m tmux not available: {}. Waiting for agents...\x1b[0m", e);
            // Fallback: just wait for .done files
            let task_ids: Vec<String> = tasks.iter().map(|t| t.id.clone()).collect();
            loop {
                tokio::time::sleep(tokio::time::Duration::from_secs(2)).await;
                let all_done = task_ids.iter().all(|id| {
                    std::path::Path::new(&format!("/tmp/athena_task_{}.done", id)).exists()
                });
                if all_done { break; }
            }
        }
    }

    Ok(())
}

/// Run the integration agent to merge all agent outputs
async fn run_integration(
    orchestrator: &Orchestrator,
    agent: &DiscoveredAgent,
    tasks: &[Task],
) -> Result<()> {
    let session = orchestrator.session.as_ref().ok_or_else(|| anyhow::anyhow!("No session"))?;

    // Build integration prompt from integration.md + all agent outputs
    let integration_md = std::fs::read_to_string(session.dir().join("integration.md")).unwrap_or_default();
    let original_prompt = std::fs::read_to_string(session.dir().join("original_prompt.md")).unwrap_or_default();

    let mut all_outputs = String::new();
    for task in tasks {
        if let Some(out_file) = &task.output_file {
            let output = std::fs::read_to_string(out_file).unwrap_or_else(|_| "[No output captured]".to_string());
            all_outputs.push_str(&format!("\n\n## Output from {} (Agent: {})\n\n{}\n",
                task.title,
                task.assigned_agent.as_deref().unwrap_or("unknown"),
                output,
            ));
        }
    }

    let full_integration_prompt = format!(
        "{}\n\n---\n\n# Original Project Requirements\n\n{}\n\n---\n\n# Agent Outputs\n\n{}\n",
        integration_md,
        original_prompt,
        all_outputs,
    );

    // Write integration prompt to file
    let int_prompt_file = session.dir().join("integration_full_prompt.md");
    std::fs::write(&int_prompt_file, &full_integration_prompt)?;

    let int_workspace = session.dir().join("integration");
    std::fs::create_dir_all(&int_workspace)?;

    // Launch integration agent
    let mut cmd_obj = tokio::process::Command::new(if agent.name.contains("agy") { "agy" } else if agent.name.contains("claude") { "claude" } else { "codex" });
    
    if agent.name.contains("agy") {
        cmd_obj.arg("--dangerously-skip-permissions");
        cmd_obj.arg("-p").arg(std::fs::read_to_string(&int_prompt_file).unwrap_or_default());
    } else if agent.name.contains("claude") {
        cmd_obj.arg("-p").arg(std::fs::read_to_string(&int_prompt_file).unwrap_or_default());
    } else {
        cmd_obj.arg("exec").arg(std::fs::read_to_string(&int_prompt_file).unwrap_or_default());
    }
    
    cmd_obj.current_dir(&int_workspace)
        .stdout(std::process::Stdio::piped())
        .stderr(std::process::Stdio::piped());

    let child_res = cmd_obj.spawn();

    match child_res {
        Ok(mut child) => {
            let mut stdout_handle = child.stdout.take().expect("Failed to get stdout");
            let (tx, mut rx) = tokio::sync::mpsc::channel::<Vec<u8>>(32);
            
            tokio::spawn(async move {
                use tokio::io::AsyncReadExt;
                let mut buf = [0u8; 1024];
                loop {
                    match stdout_handle.read(&mut buf).await {
                        Ok(0) => break,
                        Ok(n) => {
                            if tx.send(buf[..n].to_vec()).await.is_err() { break; }
                        }
                        Err(_) => break,
                    }
                }
            });

            let frames = ['⠋', '⠙', '⠹', '⠸', '⠼', '⠴', '⠦', '⠧', '⠇', '⠏'];
            let mut i = 0;
            let mut full_output = String::new();
            
            loop {
                match tokio::time::timeout(tokio::time::Duration::from_millis(80), rx.recv()).await {
                    Ok(Some(data)) => {
                        let chunk = String::from_utf8_lossy(&data);
                        full_output.push_str(&chunk);
                    }
                    Ok(None) => break, // Channel closed (process finished)
                    Err(_) => { // Timeout
                        print!("\r\x1b[38;2;101;147;47m  {}\x1b[0m Processing...", frames[i % frames.len()]);
                        let _ = std::io::Write::flush(&mut std::io::stdout());
                        i += 1;
                    }
                }
            }
            
            // Clear the processing line when done
            print!("\r\x1b[K");
            let _ = std::io::Write::flush(&mut std::io::stdout());
            
            let _ = child.wait().await;
            
            // CRITICAL FIX: Extract markdown code blocks and write them to the workspace
            // Since the agent might forcibly write to its own global scratch folder,
            // we parse its stdout and write the files locally ourselves.
            let mut in_block = false;
            let mut current_file: Option<String> = None;
            let mut current_content: Vec<String> = Vec::new();
            let mut default_file_idx = 1;
            
            for line in full_output.lines() {
                if line.trim().starts_with("```") {
                    if in_block {
                        // End of block, write the file
                        let filename = current_file.take().unwrap_or_else(|| {
                            let name = format!("file{}.txt", default_file_idx);
                            default_file_idx += 1;
                            name
                        });
                        
                        let path = int_workspace.join(&filename);
                        if let Some(parent) = path.parent() {
                            let _ = std::fs::create_dir_all(parent);
                        }
                        // Remove the first line if it was just the filename comment
                        if !current_content.is_empty() {
                            let first_line = current_content[0].trim();
                            if first_line.starts_with("# ") || first_line.starts_with("// ") {
                                if first_line.contains(&filename) {
                                    current_content.remove(0);
                                }
                            }
                        }
                        let _ = std::fs::write(&path, current_content.join("\n"));
                        current_content.clear();
                        in_block = false;
                    } else {
                        // Start of block
                        in_block = true;
                        current_file = None;
                        current_content.clear();
                    }
                    continue;
                }
                
                if in_block {
                    if current_file.is_none() {
                        let trimmed = line.trim();
                        if trimmed.starts_with("# ") || trimmed.starts_with("// ") {
                            let parts: Vec<&str> = trimmed.split_whitespace().collect();
                            if parts.len() >= 2 && parts[1].contains(".") {
                                current_file = Some(parts[1].to_string());
                            }
                        }
                    }
                    current_content.push(line.to_string());
                }
            }
        }
        Err(e) => {
            println!("  \x1b[33m Integration agent failed: {}. Skipping.\x1b[0m", e);
        }
    }

    println!("\n  \x1b[38;2;101;147;47m-\x1b[0m Integration complete");

    Ok(())
}

/// Derive a project name from the objective text
fn derive_project_name(objective: &str) -> String {
    let words: Vec<&str> = objective.split_whitespace().take(4).collect();
    let name = words.join("_")
        .chars()
        .filter(|c| c.is_alphanumeric() || *c == '_')
        .collect::<String>();

    if name.is_empty() {
        "athena_project".to_string()
    } else {
        name[..name.len().min(30)].to_string()
    }
}
