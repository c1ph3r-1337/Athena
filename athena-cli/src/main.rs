use anyhow::Result;
use clap::{Parser, Subcommand};

mod auth;
mod orchestrate;
mod setup;
mod tui;

#[derive(Parser)]
#[command(name = "athena", version = "0.1", about = "Athena: Multi-Agent Orchestrator")]
struct Cli {
    #[command(subcommand)]
    command: Option<Commands>,
}

#[derive(Subcommand)]
enum Commands {
    /// Authenticate via OAuth Device Flow
    Login,

    /// Run the first-time setup wizard
    Setup,

    /// Run a project with orchestration
    Run {
        /// The objective/prompt to execute (or path to a .md file)
        objective: String,

        /// Token budget for the entire run
        #[arg(long)]
        budget: Option<usize>,
    },
}
#[derive(serde::Serialize, serde::Deserialize, Default, Clone)]
struct Config {
    default_agent: Option<String>,
    default_model: Option<String>,
}

impl Config {
    fn load() -> Self {
        if let Some(home) = dirs::home_dir() {
            let path = home.join(".athena/config.json");
            if let Ok(content) = std::fs::read_to_string(path) {
                if let Ok(config) = serde_json::from_str(&content) {
                    return config;
                }
            }
        }
        Config::default()
    }

    fn save(&self) -> Result<()> {
        if let Some(home) = dirs::home_dir() {
            let path = home.join(".athena/config.json");
            if let Some(parent) = path.parent() {
                std::fs::create_dir_all(parent)?;
            }
            std::fs::write(path, serde_json::to_string_pretty(self)?)?;
        }
        Ok(())
    }
}

#[tokio::main]
async fn main() -> Result<()> {
    // Ignore SIGINT in the main orchestrator loop using Tokio.
    // This allows Ctrl+C to kill child processes (like `agy`) without killing Athena itself.
    tokio::spawn(async move {
        loop {
            let _ = tokio::signal::ctrl_c().await;
        }
    });

    // Parse command line arguments first
    let cli = Cli::parse();

    match cli.command {
        Some(Commands::Login) => {
            auth::login().await?;
        }
        Some(Commands::Setup) => {
            setup::run_setup(true).await?;
        }
        Some(Commands::Run { objective, .. }) => {
            // Check if objective is a file path
            let actual_objective = if std::path::Path::new(&objective).exists() {
                std::fs::read_to_string(&objective)?
            } else {
                objective
            };

            orchestrate::run_orchestration(&actual_objective).await?;
        }
        None => {
            // Run the first-time setup wizard
            setup::run_setup(false).await?;

            fn draw_logo(session_id: &str) {
                print!("{esc}[2J{esc}[1;1H", esc = 27 as char);
                let img_data = include_bytes!("../../athena_logo_holo_green.png");
                if let Ok(img) = image::load_from_memory(img_data) {
                    let conf = viuer::Config {
                        width: Some(30),
                        height: Some(15),
                        absolute_offset: false,
                        ..Default::default()
                    };
                    let _ = viuer::print(&img, &conf);
                    
                    use crossterm::{cursor, execute};
                    use std::io::stdout;
                    let mut stdout = stdout();
                    
                    let _ = execute!(stdout, cursor::MoveUp(10));
                    println!("\x1b[32C   \x1b[38;2;101;147;47mAthena Meta-Orchestrator 0.1.0\x1b[0m");
                    println!("\x1b[32C   c1ph3r");
                    println!("\x1b[32C   Multi-Agent Swarm");
                    println!("\x1b[32C   /vault/Projects/Athena");
                    println!("\x1b[32C   \x1b[38;2;101;147;47mSession:\x1b[0m {}", session_id);
                    let _ = execute!(stdout, cursor::MoveDown(5));
                } else {
                    let logo = format!(r#"
                ██████████              
             ███████████                
           █████      █                 
          ███ ███ █████████             
         ███ ███ █████████              
         ███ ███ ████                      \x1b[38;2;101;147;47mAthena Meta-Orchestrator 0.1.0\x1b[0m
          ██  ██ ████                      c1ph3r
          ███    ███  █                    Multi-Agent Swarm
            █  █████                       /vault/Projects/Athena
             ███████                       \x1b[38;2;101;147;47mSession:\x1b[0m {}
           ███   ██ ██   ██             
                 ██  ██████             
"#, session_id);
                    println!("{}", logo);
                }
            }

            use std::env;
            use std::io::{self, Write, Read};
            use std::process::{Command, Stdio};
            use std::time::Duration;
            use inquire::{Select, ui::{RenderConfig, StyleSheet, Styled, Color}};
            
            struct MyHelper {
                term_width: usize,
                default_agent: String,
            }

            impl rustyline::completion::Completer for MyHelper {
                type Candidate = rustyline::completion::Pair;
                fn complete(&self, line: &str, pos: usize, _ctx: &rustyline::Context<'_>) -> rustyline::Result<(usize, Vec<Self::Candidate>)> {
                    let mut matches = Vec::new();
                    let input = &line[..pos];
                    
                    if input.starts_with('/') {
                        let input_lower = input.to_lowercase();
                        
                        if input_lower.starts_with("/default ") {
                            let prefix = input_lower.trim_start_matches("/default ");
                            let agents = ["agy", "codex", "claude"];
                            for a in agents {
                                if a.starts_with(prefix) {
                                    matches.push(rustyline::completion::Pair {
                                        display: format!("/default {}", a),
                                        replacement: format!("/default {}", a),
                                    });
                                }
                            }
                            return Ok((0, matches));
                        }
                        
                        let cmds = [
                            ("/add-dir", "Add a directory to the workspace"),
                            ("/agents", "List available custom agents"),
                            ("/artifact", "View and review artifacts"),
                            ("/btw", "Ask a side question without interrupting"),
                            ("/changelog", "Show release notes and changes"),
                            ("/default", "Set the default agent (e.g. /default codex)"),
                            ("/help", "Show help for commands"),
                            ("/orch", "Run an orchestration task inline or multiline"),
                            ("/orch -md", "Run an orchestration task from a markdown file"),
                            ("/resume", "Resume a past orchestration session"),
                            ("/login", "Login to agents"),
                            ("/logout", "Logout of all agents"),
                            ("/clear", "Clear the terminal"),
                            ("/exit", "Exit the orchestrator"),
                            ("/quit", "Exit the orchestrator"),
                        ];
                        
                        for (c, d) in cmds {
                            if c.starts_with(&input_lower) {
                                matches.push(rustyline::completion::Pair {
                                    display: format!("{:<20} {}", c, d),
                                    replacement: c.to_string(),
                                });
                            }
                        }
                        return Ok((0, matches));
                    }
                    Ok((0, matches))
                }
            }

            impl rustyline::hint::Hinter for MyHelper {
                type Hint = String;
                fn hint(&self, _line: &str, _pos: usize, _ctx: &rustyline::Context<'_>) -> Option<String> {
                    None
                }
            }
            
            impl rustyline::highlight::Highlighter for MyHelper {}
            impl rustyline::validate::Validator for MyHelper {
                fn validate(&self, _ctx: &mut rustyline::validate::ValidationContext) -> rustyline::Result<rustyline::validate::ValidationResult> {
                    Ok(rustyline::validate::ValidationResult::Valid(None))
                }
                fn validate_while_typing(&self) -> bool { false }
            }
            impl rustyline::Helper for MyHelper {}

            let session_id = format!("{}-{}", 
                chrono::Local::now().format("%Y-%m-%d"),
                uuid::Uuid::new_v4().to_string().chars().take(4).collect::<String>()
            );

            draw_logo(&session_id);
            
            let mut config = RenderConfig::default();
            config.prompt_prefix = Styled::new(">").with_fg(Color::Rgb { r: 101, g: 147, b: 47 });
            config.highlighted_option_prefix = Styled::new(">").with_fg(Color::Rgb { r: 101, g: 147, b: 47 });
            config.scroll_up_prefix = Styled::new("↑");
            config.scroll_down_prefix = Styled::new("↓");
            config.answered_prompt_prefix = Styled::new(">").with_fg(Color::Rgb { r: 101, g: 147, b: 47 });
            config.help_message = StyleSheet::new().with_fg(Color::DarkGrey);

            use std::sync::atomic::{AtomicBool, Ordering};
            use std::sync::Arc;

            let slash_flag = Arc::new(AtomicBool::new(false));

            struct SlashHandler {
                flag: Arc<AtomicBool>,
            }
            impl rustyline::ConditionalEventHandler for SlashHandler {
                fn handle(&self, _evt: &rustyline::Event, _n: rustyline::RepeatCount, _positive: bool, ctx: &rustyline::EventContext) -> Option<rustyline::Cmd> {
                    if ctx.line().is_empty() {
                        self.flag.store(true, Ordering::SeqCst);
                        Some(rustyline::Cmd::AcceptLine)
                    } else {
                        None // default: insert '/'
                    }
                }
            }

            let rl_config = rustyline::Config::builder()
                .completion_type(rustyline::CompletionType::List)
                .build();
            let mut rl = rustyline::Editor::<MyHelper, rustyline::history::DefaultHistory>::with_config(rl_config)
                .expect("Failed to create rustyline editor");

            rl.bind_sequence(
                rustyline::KeyEvent(rustyline::KeyCode::Char('/'), rustyline::Modifiers::NONE),
                rustyline::EventHandler::Conditional(Box::new(SlashHandler { flag: Arc::clone(&slash_flag) })),
            );

            let mut app_config = Config::load();

            loop {
                let default_agent = app_config.default_agent.as_deref().unwrap_or("agy");
                let term_width = crossterm::terminal::size().map(|(w, _)| w as usize).unwrap_or(105);

                let helper = MyHelper {
                    term_width,
                    default_agent: default_agent.to_string(),
                };
                rl.set_helper(Some(helper));

                let prompt_str = "\x1b[38;2;101;147;47m>\x1b[0m ";
                let spaces = term_width.saturating_sub(14 + default_agent.len());
                let separator = "\x1b[90m".to_string() + &"\u{2500}".repeat(term_width) + "\x1b[0m";
                let help_text = format!("\x1b[90m/help for help{}{}\x1b[0m", " ".repeat(spaces), default_agent);

                // Draw bottom UI below, then move cursor back up with relative movement
                // (relative \x1b[2A is reliable even if terminal scrolls; \x1b[s/\x1b[u is not)
                print!("\n{}\n{}\x1b[2A\r", separator, help_text);
                std::io::Write::flush(&mut std::io::stdout()).unwrap();

                slash_flag.store(false, Ordering::SeqCst);

                let input_res = rl.readline(prompt_str);

                // After readline, cursor is on the line below the input.
                // Clear everything below (separator + help text remnants)
                print!("\x1b[J");
                std::io::Write::flush(&mut std::io::stdout()).unwrap();

                // If '/' was typed on empty line, show the Select menu
                if slash_flag.load(Ordering::SeqCst) {
                    let cmd_options: Vec<String> = vec![
                        "/help", "/orch", "/orch -md", "/resume",
                        "/default", "/model", "/agents",
                        "/login", "/logout", "/clear", "/exit",
                    ].into_iter().map(|s| s.to_string()).collect();

                    let mut select_config = config.clone();
                    select_config.prompt_prefix = Styled::new(">").with_fg(Color::Rgb { r: 101, g: 147, b: 47 });
                    select_config.answered_prompt_prefix = Styled::new(">").with_fg(Color::Rgb { r: 101, g: 147, b: 47 });
                    select_config.help_message = StyleSheet::new().with_fg(Color::DarkGrey);

                    match Select::new("", cmd_options)
                        .with_render_config(select_config)
                        .with_help_message("up/down Navigate  enter Select  esc Go Back")
                        .prompt()
                    {
                        Ok(choice) => {
                            // We need to re-assign input so the command dispatch below works
                            let input = choice;
                            let cmd = input.trim();
                            // We can't easily fall-through to the match below, so
                            // we duplicate a minimal dispatch here for the select items.
                            // This is intentional to keep the flow simple.
                            match cmd {
                                "/help" => {
                                    println!("\n  \x1b[1mAvailable Commands:\x1b[0m");
                                    println!("  /orch <prompt>      Run inline orchestration");
                                    println!("  /orch -md <file>    Run orchestration from file");
                                    println!("  /orch               Enter multi-line orchestration mode");
                                    println!("  /resume             Resume a past orchestration session");
                                    println!("  /default <agent>    Set default agent for normal inputs");
                                    println!("  /login              Login to agents");
                                    println!("  /logout             Logout of agents");
                                    println!("  /clear              Clear the terminal");
                                    println!("  /exit, /quit        Exit orchestrator");
                                    println!("\n  Any other input is sent to the default agent.\n");
                                }
                                "/clear" => { draw_logo(&session_id); }
                                "/exit" => break,
                                "/login" => { setup::run_setup(true).await?; }
                                "/logout" => {
                                    if let Some(home) = dirs::home_dir() {
                                        let _ = std::fs::remove_file(home.join(".athena/agents.json"));
                                        println!("Logged out of all agents. Run '/login' to re-authenticate.");
                                    }
                                }
                                "/resume" => {
                                    if let Err(e) = orchestrate::run_resume().await {
                                        println!("Error: {}", e);
                                    }
                                }
                                _ => {
                                    // For /orch, /default, /model, /agents — re-inject into the
                                    // normal dispatch by setting a flag. But simpler: just
                                    // add to history and let the next iteration handle it.
                                    // Actually, handle inline:
                                    if cmd == "/agents" {
                                        let _ = std::process::Command::new("agy").arg("agents").status();
                                    } else if cmd == "/default" {
                                        // Show agent selector
                                        let agents = vec!["agy", "codex", "claude"];
                                        let current = app_config.default_agent.as_deref().unwrap_or("agy");
                                        let options: Vec<String> = agents.iter().map(|&a| {
                                            if a == current { format!("{:<28} (current)", a) } else { a.to_string() }
                                        }).collect();
                                        let starting_cursor = agents.iter().position(|&a| a == current).unwrap_or(0);
                                        let mut sc = config.clone();
                                        sc.prompt_prefix = Styled::new("").with_fg(Color::Rgb { r: 101, g: 147, b: 47 });
                                        sc.answered_prompt_prefix = Styled::new("").with_fg(Color::Rgb { r: 101, g: 147, b: 47 });
                                        sc.help_message = StyleSheet::new().with_fg(Color::DarkGrey);
                                        println!();
                                        if let Ok(choice) = Select::new("Switch Agent", options)
                                            .with_starting_cursor(starting_cursor).with_render_config(sc)
                                            .with_help_message("up/down Navigate  enter Select  esc Go Back").prompt() {
                                            let agent = choice.split_whitespace().next().unwrap().to_string();
                                            app_config.default_agent = Some(agent.clone());
                                            let _ = app_config.save();
                                            println!("  \x1b[90m⎿\x1b[0m  Default agent set to {}", agent);
                                        }
                                        println!();
                                    } else if cmd == "/model" {
                                        let models = vec![
                                            "Gemini 3.6 Flash", "Gemini 3.5 Flash", "Gemini 3.1 Pro",
                                            "Claude Sonnet 4.6 (Thinking)", "Claude Opus 4.6 (Thinking)", "GPT-OSS 120B (Medium)"
                                        ];
                                        let current = app_config.default_model.as_deref().unwrap_or("Gemini 3.1 Pro");
                                        let options: Vec<String> = models.iter().map(|&m| {
                                            if m == current { format!("{:<28} (current)", m) } else { m.to_string() }
                                        }).collect();
                                        let starting_cursor = models.iter().position(|&m| m == current).unwrap_or(2);
                                        let mut sc = config.clone();
                                        sc.prompt_prefix = Styled::new("").with_fg(Color::Rgb { r: 101, g: 147, b: 47 });
                                        sc.answered_prompt_prefix = Styled::new("").with_fg(Color::Rgb { r: 101, g: 147, b: 47 });
                                        sc.help_message = StyleSheet::new().with_fg(Color::DarkGrey);
                                        println!();
                                        if let Ok(choice) = Select::new("Switch Model", options)
                                            .with_starting_cursor(starting_cursor).with_render_config(sc)
                                            .with_help_message("up/down Navigate  enter Select  esc Go Back").prompt() {
                                            let model = choice.split("   ").next().unwrap_or(&choice).trim().to_string();
                                            app_config.default_model = Some(model.clone());
                                            let _ = app_config.save();
                                            println!("  \x1b[90m⎿\x1b[0m  Model set to {}", model);
                                        }
                                        println!();
                                    } else if cmd == "/orch" {
                                        println!("\x1b[90m[Multi-line mode: paste your prompt, then type 'EOF' on a new line and press Enter]\x1b[0m");
                                        print!("\x1b[38;2;101;147;47m");
                                        io::stdout().flush().ok();
                                        use tokio::io::AsyncBufReadExt;
                                        let mut reader = tokio::io::BufReader::new(tokio::io::stdin());
                                        let mut multi_line = String::new();
                                        let mut aborted = false;
                                        loop {
                                            let mut line = String::new();
                                            tokio::select! {
                                                res = reader.read_line(&mut line) => {
                                                    match res {
                                                        Ok(0) => break,
                                                        Ok(_) => {
                                                            if line.contains('\x1b') { aborted = true; break; }
                                                            if line.trim() == "EOF" { break; }
                                                            multi_line.push_str(&line);
                                                        }
                                                        Err(_) => { aborted = true; break; }
                                                    }
                                                }
                                                _ = tokio::signal::ctrl_c() => { aborted = true; break; }
                                            }
                                        }
                                        print!("\x1b[0m");
                                        if aborted {
                                            println!("\x1b[90m<cancelled>\x1b[0m\n");
                                        } else if let Err(e) = orchestrate::run_orchestration(&multi_line).await {
                                            println!("Error: {}", e);
                                        }
                                    } else if cmd == "/orch -md" {
                                        println!("Usage: /orch -md <path-to-file>");
                                    }
                                }
                            }
                            continue;
                        }
                        Err(_) => continue, // Escaped
                    }
                }

                let input = match input_res {
                    Ok(i) => {
                        rl.add_history_entry(i.as_str()).ok();
                        i
                    },
                    Err(rustyline::error::ReadlineError::Interrupted) => {
                        println!("\x1b[90m<cancelled>\x1b[0m");
                        continue;
                    },
                    Err(_) => break,
                };
                let mut cmd = input.trim();
                
                let all_cmds = vec![
                    ("/add-dir", "Add a directory to the workspace"),
                    ("/agents", "List available custom agents"),
                    ("/artifact", "View and review artifacts"),
                    ("/btw", "Ask a side question without interrupting"),
                    ("/changelog", "Show release notes and changes"),
                    ("/default", "Set the default agent (e.g. /default codex)"),
                    ("/help", "Show help for commands"),
                    ("/model", "Switch the active LLM model"),
                    ("/orch", "Run an orchestration task inline or multiline"),
                    ("/orch -md", "Run an orchestration task from a markdown file"),
                    ("/resume", "Resume a past orchestration session"),
                    ("/login", "Login to agents"),
                    ("/logout", "Logout of all agents"),
                    ("/clear", "Clear the terminal"),
                    ("/exit", "Exit the orchestrator"),
                    ("/quit", "Exit the orchestrator"),
                ];
                for (c, d) in all_cmds.iter() {
                    let formatted = format!("{:<20} {}", c, d);
                    if cmd == formatted.trim() {
                        cmd = c;
                        break;
                    }
                }


                match cmd {
                    "/help" => {
                        println!("\n  \x1b[1mAvailable Commands:\x1b[0m");
                        println!("  /orch <prompt>      Run inline orchestration");
                        println!("  /orch -md <file>    Run orchestration from file");
                        println!("  /orch               Enter multi-line orchestration mode");
                        println!("  /resume             Resume a past orchestration session");
                        println!("  /default <agent>    Set default agent for normal inputs");
                        println!("  /login              Login to agents");
                        println!("  /logout             Logout of agents");
                        println!("  /clear              Clear the terminal");
                        println!("  /exit, /quit        Exit orchestrator");
                        println!("\n  Any other input is sent to the default agent.\n");
                        continue;
                    }
                    cmd if cmd.starts_with("/default ") => {
                        let agent = cmd.trim_start_matches("/default ").trim().to_string();
                        if agent.is_empty() {
                            println!("Error: must specify an agent name.");
                        } else {
                            app_config.default_agent = Some(agent.clone());
                            if let Err(e) = app_config.save() {
                                println!("Error saving config: {}", e);
                            } else {
                                println!("Default agent set to '{}'", agent);
                            }
                        }
                        println!();
                        continue;
                    }
                    "/agents" => {
                        let _ = std::process::Command::new("agy").arg("agents").status();
                        continue;
                    }
                    "/model" => {
                        let models = vec![
                            "Gemini 3.6 Flash",
                            "Gemini 3.5 Flash",
                            "Gemini 3.1 Pro",
                            "Claude Sonnet 4.6 (Thinking)",
                            "Claude Opus 4.6 (Thinking)",
                            "GPT-OSS 120B (Medium)"
                        ];
                        let current = app_config.default_model.as_deref().unwrap_or("Gemini 3.1 Pro");
                        
                        let options: Vec<String> = models.iter().map(|&m| {
                            if m == current {
                                format!("{:<28} (current)", m)
                            } else {
                                m.to_string()
                            }
                        }).collect();
                        
                        let starting_cursor = models.iter().position(|&m| m == current).unwrap_or(2);
                        
                        let mut select_config = config.clone();
                        select_config.prompt_prefix = Styled::new("").with_fg(Color::Rgb { r: 101, g: 147, b: 47 });
                        select_config.answered_prompt_prefix = Styled::new("").with_fg(Color::Rgb { r: 101, g: 147, b: 47 });
                        select_config.help_message = StyleSheet::new().with_fg(Color::DarkGrey);

                        println!();
                        let ans = Select::new("Switch Model", options)
                            .with_starting_cursor(starting_cursor)
                            .with_render_config(select_config)
                            .with_help_message("Keyboard: ↑/↓ Navigate  enter Select  esc Go Back")
                            .prompt();

                        match ans {
                            Ok(choice) => {
                                // Extract the model name (remove the "(current)" part if present)
                                let model = choice.split("   ").next().unwrap_or(&choice).trim().to_string();
                                app_config.default_model = Some(model.clone());
                                if let Err(e) = app_config.save() {
                                    println!("Error saving config: {}", e);
                                } else {
                                    println!("  \x1b[90m⎿\x1b[0m  Model set to {}", model);
                                }
                            }
                            Err(_) => {} // Escaped or interrupted
                        }
                        println!();
                        continue;
                    }
                    "/default" => {
                        let agents = vec!["agy", "codex", "claude"];
                        let current = app_config.default_agent.as_deref().unwrap_or("agy");
                        
                        let options: Vec<String> = agents.iter().map(|&a| {
                            if a == current {
                                format!("{:<28} (current)", a)
                            } else {
                                a.to_string()
                            }
                        }).collect();
                        
                        let starting_cursor = agents.iter().position(|&a| a == current).unwrap_or(0);
                        
                        let mut select_config = config.clone();
                        select_config.prompt_prefix = Styled::new("").with_fg(Color::Rgb { r: 101, g: 147, b: 47 });
                        select_config.answered_prompt_prefix = Styled::new("").with_fg(Color::Rgb { r: 101, g: 147, b: 47 });
                        select_config.help_message = StyleSheet::new().with_fg(Color::DarkGrey);

                        println!();
                        let ans = Select::new("Switch Agent", options)
                            .with_starting_cursor(starting_cursor)
                            .with_render_config(select_config)
                            .with_help_message("Keyboard: ↑/↓ Navigate  enter Select  esc Go Back")
                            .prompt();

                        match ans {
                            Ok(choice) => {
                                let agent = choice.split_whitespace().next().unwrap().to_string();
                                app_config.default_agent = Some(agent.clone());
                                if let Err(e) = app_config.save() {
                                    println!("Error saving config: {}", e);
                                } else {
                                    println!("  \x1b[90m⎿\x1b[0m  Default agent set to {}", agent);
                                }
                            }
                            Err(_) => {} // Escaped or interrupted
                        }
                        println!();
                        continue;
                    }
                    "/exit" | "/quit" | "exit" | "quit" => break,
                    "/clear" | "clear" => {
                        draw_logo(&session_id);
                        continue;
                    }
                    "/login" | "/setup" | "login" | "setup" => {
                        setup::run_setup(true).await?;
                        continue;
                    }
                    "/resume" | "resume" => {
                        if let Err(e) = orchestrate::run_resume().await {
                            println!("Error: {}", e);
                        }
                        continue;
                    }
                    "/logout" | "logout" => {
                        if let Some(home) = dirs::home_dir() {
                            let _ = std::fs::remove_file(home.join(".athena/agents.json"));
                            println!("Logged out of all agents. Run '/login' to re-authenticate.");
                        }
                        continue;
                    }
                    "" => continue,
                    cmd => {
                        println!(); // Spacing before output
                        
                        let (is_orchestration, prompt_content) = if cmd == "/orch" {
                            println!("\x1b[90m[Multi-line mode: paste your prompt, then type 'EOF' on a new line and press Enter]\x1b[0m");
                            print!("\x1b[38;2;101;147;47m");
                            io::stdout().flush()?;
                            
                            use tokio::io::AsyncBufReadExt;
                            let mut reader = tokio::io::BufReader::new(tokio::io::stdin());
                            let mut multi_line = String::new();
                            let mut aborted = false;
                            
                            loop {
                                let mut line = String::new();
                                tokio::select! {
                                    res = reader.read_line(&mut line) => {
                                        match res {
                                            Ok(0) => break,
                                            Ok(_) => {
                                                if line.contains('\x1b') {
                                                    aborted = true;
                                                    break;
                                                }
                                                if line.trim() == "EOF" {
                                                    break;
                                                }
                                                multi_line.push_str(&line);
                                            }
                                            Err(_) => {
                                                aborted = true;
                                                break;
                                            }
                                        }
                                    }
                                    _ = tokio::signal::ctrl_c() => {
                                        aborted = true;
                                        break;
                                    }
                                }
                            }
                            print!("\x1b[0m");
                            if aborted {
                                println!("\x1b[90m<cancelled>\x1b[0m\n");
                                continue;
                            }
                            (true, multi_line)
                        } else if cmd.starts_with("/orch -md ") {
                            let path = cmd.trim_start_matches("/orch -md ").trim();
                            if std::path::Path::new(path).exists() {
                                (true, std::fs::read_to_string(path)?)
                            } else {
                                println!("Error: File {} not found.", path);
                                continue;
                            }
                        } else if cmd.starts_with("/orch ") {
                            (true, cmd.trim_start_matches("/orch ").trim().to_string())
                        } else {
                            (false, cmd.to_string())
                        };

                        if is_orchestration {
                            if let Err(e) = orchestrate::run_orchestration(&prompt_content).await {
                                println!("Error: {}", e);
                            }
                        } else {
                            let default_agent = app_config.default_agent.as_deref().unwrap_or("agy");
                            let default_model = app_config.default_model.as_deref().unwrap_or("Gemini 3.1 Pro");
                            
                            let model_id = match default_model {
                                "Gemini 3.6 Flash" => "gemini-3.6-flash",
                                "Gemini 3.5 Flash" => "gemini-3.5-flash",
                                "Gemini 3.1 Pro" => "gemini-3.1-pro",
                                "Claude Sonnet 4.6 (Thinking)" => "claude-3-5-sonnet",
                                "Claude Opus 4.6 (Thinking)" => "claude-3-opus",
                                "GPT-OSS 120B (Medium)" => "gpt-4o-mini",
                                _ => "gemini-3.1-pro",
                            };

                            let mut cmd_obj = std::process::Command::new("agy");
                            if default_agent != "agy" {
                                cmd_obj.arg("--agent").arg(default_agent);
                            }
                            cmd_obj.arg("--model").arg(model_id);
                            
                            // All Gemini models require the effort flag
                            if model_id.starts_with("gemini") {
                                cmd_obj.arg("--effort").arg("low");
                            }
                            
                            let mut child = match cmd_obj
                                .arg("--print")
                                .arg(&prompt_content)
                                .stdout(Stdio::piped())
                                .stderr(Stdio::piped()) // pipe stderr so it doesn't leak UI
                                .spawn() 
                            {
                                Ok(c) => c,
                                Err(e) => {
                                    println!("Error spawning default agent: {}", e);
                                    continue;
                                }
                            };
                            
                            let mut stdout_handle = child.stdout.take().expect("Failed to get stdout");
                            
                            let (tx, rx) = std::sync::mpsc::channel::<Vec<u8>>();
                            
                            std::thread::spawn(move || {
                                let mut buf = [0u8; 1024];
                                loop {
                                    match stdout_handle.read(&mut buf) {
                                        Ok(0) => break,
                                        Ok(n) => {
                                            if tx.send(buf[..n].to_vec()).is_err() { break; }
                                        }
                                        Err(_) => break,
                                    }
                                }
                            });

                            let frames = ['⠋', '⠙', '⠹', '⠸', '⠼', '⠴', '⠦', '⠧', '⠇', '⠏'];
                            let mut i = 0;
                            let mut first = true;
                            let mut start_of_line = true;
                            
                            loop {
                                if first {
                                    match rx.recv_timeout(Duration::from_millis(80)) {
                                        Ok(data) => {
                                            print!("\r\x1b[K"); // Clear the line
                                            let chunk = String::from_utf8_lossy(&data);
                                            let mut out = String::with_capacity(chunk.len() + 10);
                                            for ch in chunk.chars() {
                                                if start_of_line {
                                                    out.push_str("  ");
                                                    start_of_line = false;
                                                }
                                                out.push(ch);
                                                if ch == '\n' {
                                                    start_of_line = true;
                                                }
                                            }
                                            print!("{}", out);
                                            let _ = std::io::stdout().flush();
                                            first = false;
                                        }
                                        Err(std::sync::mpsc::RecvTimeoutError::Timeout) => {
                                            print!("\r\x1b[38;2;101;147;47m{}\x1b[0m Thinking...", frames[i % frames.len()]);
                                            let _ = std::io::stdout().flush();
                                            i += 1;
                                        }
                                        Err(_) => break, // Process finished with no output
                                    }
                                } else {
                                    match rx.recv() {
                                        Ok(data) => {
                                            let chunk = String::from_utf8_lossy(&data);
                                            let mut out = String::with_capacity(chunk.len() + 10);
                                            for ch in chunk.chars() {
                                                if start_of_line {
                                                    out.push_str("  ");
                                                    start_of_line = false;
                                                }
                                                out.push(ch);
                                                if ch == '\n' {
                                                    start_of_line = true;
                                                }
                                            }
                                            print!("{}", out);
                                            let _ = std::io::stdout().flush();
                                        }
                                        Err(_) => break,
                                    }
                                }
                            }
                            let _ = child.wait();
                        }
                        println!(); // Spacing before next prompt
                    }
                }
            }
        }
    }

    Ok(())
}
