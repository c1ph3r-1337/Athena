use anyhow::Result;
use crossterm::{
    event::{self, Event as CEvent, KeyCode},
    execute,
    terminal::{disable_raw_mode, enable_raw_mode, EnterAlternateScreen, LeaveAlternateScreen},
};
use ratatui::{
    backend::CrosstermBackend,
    layout::{Constraint, Direction, Layout},
    style::{Color, Style},
    widgets::{Block, Borders, List, ListItem},
    Terminal,
};
use std::{io, sync::{Arc, Mutex}};
use tokio::sync::mpsc;
use athena_core::orchestrator::OrchestratorEvent;
use athena_core::task::TaskStatus;

pub struct AppState {
    pub tasks: Vec<athena_core::task::Task>,
}

pub async fn run_tui(mut rx: mpsc::Receiver<OrchestratorEvent>, state: Arc<Mutex<AppState>>) -> Result<()> {
    enable_raw_mode()?;
    let mut stdout = io::stdout();
    execute!(stdout, EnterAlternateScreen)?;
    let backend = CrosstermBackend::new(stdout);
    let mut terminal = Terminal::new(backend)?;

    loop {
        terminal.draw(|f| {
            let chunks = Layout::default()
                .direction(Direction::Vertical)
                .margin(1)
                .constraints([Constraint::Length(8), Constraint::Min(0)].as_ref())
                .split(f.area());

            let state_lock = state.lock().unwrap();
            
            // Top Pane: Orchestrator (Task DAG)
            let mut top_items = Vec::new();
            for task in &state_lock.tasks {
                let status_str = match task.status {
                    TaskStatus::Completed  => "████ DONE   ",
                    TaskStatus::Running    => "██░░ RUNNING",
                    TaskStatus::Failed     => "████ FAILED ",
                    TaskStatus::Pending    => "░░░░ PENDING",
                    TaskStatus::Blocked    => "░░░░ BLOCKED",
                    _                      => "░░░░ WAITING",
                };
                let color = match task.status {
                    TaskStatus::Completed => Color::Green,
                    TaskStatus::Running => Color::Yellow,
                    TaskStatus::Failed => Color::Red,
                    _ => Color::DarkGray,
                };
                let agent = task.assigned_agent.as_deref().unwrap_or("Unassigned");
                top_items.push(ListItem::new(format!("{} - {} (Assigned: {})", status_str, task.title, agent)).style(Style::default().fg(color)));
            }
            
            let orchestrator_list = List::new(top_items)
                .block(Block::default().borders(Borders::ALL).title("Orchestrator - Task DAG Pipeline"));
            f.render_widget(orchestrator_list, chunks[0]);
            
            // Group tasks by assigned agent for bottom pane
            let mut agent_tasks: std::collections::HashMap<String, Vec<athena_core::task::Task>> = std::collections::HashMap::new();
            for task in &state_lock.tasks {
                let agent = task.assigned_agent.clone().unwrap_or_else(|| "Unassigned".to_string());
                agent_tasks.entry(agent).or_default().push(task.clone());
            }

            let mut agents: Vec<String> = agent_tasks.keys().cloned().collect();
            agents.sort();
            
            if agents.is_empty() {
                agents.push("Unassigned".to_string());
            }

            // Create columns layout for agents
            let num_cols = agents.len().max(1);
            let mut constraints = Vec::new();
            for _ in 0..num_cols {
                constraints.push(Constraint::Percentage((100 / num_cols) as u16));
            }
            
            let columns = Layout::default()
                .direction(Direction::Horizontal)
                .constraints(constraints.as_slice())
                .split(chunks[1]);

            for (i, agent_name) in agents.iter().enumerate() {
                let mut log_items = Vec::new();
                if let Some(tasks) = agent_tasks.get(agent_name) {
                    for task in tasks {
                        for log in &task.logs {
                            log_items.push(ListItem::new(log.clone()).style(Style::default().fg(Color::Gray)));
                        }
                    }
                }

                // Keep only last 20 logs
                let display_logs: Vec<ListItem> = log_items.into_iter().rev().take(20).rev().collect();
                let list = List::new(display_logs)
                    .block(Block::default().borders(Borders::ALL).title(format!("Agent Terminal: {}", agent_name)));
                f.render_widget(list, columns[i]);
            }
        })?;

        // Non-blocking event check
        if event::poll(std::time::Duration::from_millis(50))?
            && let CEvent::Key(key) = event::read()? {
                if key.code == KeyCode::Char('q') {
                    break;
                }
            }

        match rx.try_recv() {
            Ok(event) => {
                let mut state_lock = state.lock().unwrap();
                match event {
                    OrchestratorEvent::TaskStarted(id) | OrchestratorEvent::TaskCompleted(id) => {
                        if let Some(existing) = state_lock.tasks.iter_mut().find(|x| x.id == id) {
                            existing.logs.push(format!("Status changed"));
                        }
                    }
                    OrchestratorEvent::TaskFailed(id, err) => {
                        if let Some(existing) = state_lock.tasks.iter_mut().find(|x| x.id == id) {
                            existing.logs.push(format!("FAILED: {}", err));
                        }
                    }
                    OrchestratorEvent::AgentLaunched(name) => {
                        // Just log it
                        let _ = name;
                    }
                    _ => {}
                }
            }
            Err(mpsc::error::TryRecvError::Empty) => {}
            Err(mpsc::error::TryRecvError::Disconnected) => break, // Orchestrator finished
        }
    }

    disable_raw_mode()?;
    execute!(terminal.backend_mut(), LeaveAlternateScreen)?;
    terminal.show_cursor()?;

    Ok(())
}
