<div align="center">
  <img src="./athena_logo_holo_green.png" alt="Athena Logo" width="300" />
  <h1>Athena: Meta-Agent Orchestrator</h1>
  
  <p>
    <strong>A high-performance, asynchronous, CLI-native orchestrator for AI coding agents.</strong>
  </p>

  <!-- Badges -->
  <p>
    <img src="https://img.shields.io/badge/Language-Rust-orange?style=for-the-badge&logo=rust" alt="Rust" />
    <img src="https://img.shields.io/badge/Runtime-Tokio-yellow?style=for-the-badge" alt="Tokio" />
    <img src="https://img.shields.io/badge/Environment-Tmux-12B548?style=for-the-badge&logo=tmux" alt="Tmux" />
    <img src="https://img.shields.io/badge/AI-Multi--Agent-blue?style=for-the-badge" alt="AI Agents" />
  </p>
</div>

<hr />

## What Makes Athena Unique?

Unlike traditional singular coding assistants, Athena acts as an **Engineering Manager**. It digests massive project descriptions, compiles them into a Directed Acyclic Graph (DAG) of parallel tasks, and dispatches them across a fleet of specialized AI agents (`agy`, `claude`, `codex`) concurrently.

1. **Parallel Execution via DAG**: Tasks that don't depend on each other are executed simultaneously by different agents, slashing development time.
2. **Heterogeneous AI Teams**: Athena delegates tasks based on agent strengths (e.g., Gemini for rapid scaffolding, Claude for complex logic, Codex for security).
3. **Workspace Isolation & Integration**: Agents operate in deeply isolated virtual workspaces (`.orchestrator/sessions/`). Once all agents finish their parallel tracks, a final integration agent seamlessly merges everything into a clean production directory.
4. **Bulletproof Project Generation**: Instead of relying on flaky LLM tool-calling (which often fails path permissions or writes to scratch files), Athena forces agents to output raw Markdown code blocks. A specialized Rust parser intercepts this text stream, extracts the code, and physically writes the finalized project files to disk natively.
5. **CLI-Native TUI Multiplexing**: Athena exposes the multi-agent chaos by attaching the user to an interactive `tmux` grid, allowing real-time monitoring of all AI agents simultaneously, paired with a gorgeous `rustyline`-based command loop.

---

## Technology Stack

* **Rust**: The core language, chosen for memory safety, unmatched speed, and phenomenal CLI ecosystem.
* **Tokio**: The asynchronous runtime powering parallel agent processes, file state polling, and concurrent terminal streams.
* **Rustyline**: Drives the interactive prompt loop, persistent history, and custom keybinding/completion logic (e.g., the `/` commands menu).
* **Inquire & Crossterm**: Powers the beautiful terminal UI rendering, dropdown menus, and colorized outputs.
* **Tmux**: The backbone of the orchestration execution environment. Athena automatically spawns, splits, and manages tmux sessions to give each AI agent its own isolated terminal UI.

---

## Architecture & Codebase

```text
/athena/
├── athena-cli/                # The Frontend
│   ├── src/main.rs            # Rustyline event loop, ASCII art, TUI menus
│   └── src/orchestrate.rs     # Bridges UI and core logic, handles Tmux panes & visual loaders
│
├── athena-core/               # The Brains (Library Crate)
│   ├── src/orchestrator.rs    # Builds agent scripts and manages lifecycle
│   ├── src/scheduler.rs       # DAG logic, topological sorting, parallel grouping
│   ├── src/session.rs         # Manages .orchestrator/ workspace isolation
│   └── src/discovery.rs       # Probes host OS for installed AI CLIs
│
└── templates/                 # AI Instructions
    └── *.md                   # Injected system prompts enforcing strict agent rules
```

---

## How the DAG Orchestration Works

When you run `/orch`:

1. **Planning**: Athena asks the primary AI to read your prompt and break it down into an array of discrete `Tasks`.
2. **DAG Construction**: Each task declares its `dependencies`. The `scheduler` algorithm maps these into a Directed Acyclic Graph.
3. **Parallel Dispatch**: 
   * Tasks with zero dependencies are launched instantly.
   * Tasks *with* dependencies generate bash `while` loops that poll for `.done` files emitted by their prerequisite tasks.
   * *Impact:* A 10-minute linear generation process becomes a 2-minute process by building the UI, Backend, and DB concurrently.
4. **Execution**: A `tmux` session dynamically splits into panes based on active agents.
5. **Integration**: Once all DAG leaves complete, Athena constructs a massive integration prompt and spawns a final agent. The output is parsed natively by Rust and written to `./<project_name>`.

---

## Challenges Conquered

Building a multi-agent orchestrator natively in the terminal introduced severe technical hurdles:

* **TUI Loop vs. Raw Mode**: Building the input loop using `crossterm` raw mode broke `rustyline`'s native history navigation. We migrated entirely to `rustyline`'s advanced `ConditionalEventHandler` and `Completer` traits.
* **Double UI Rendering**: The bottom status bar flickered as the manual loop and a `Hinter` trait fought to draw it. We streamlined it using robust relative cursor movements (`\x1b[2A`).
* **Sub-process State Management**: AI CLIs often use non-zero exit codes for successful output streams. Standard `status.success()` checks falsely triggered errors. We overhauled the child process pipeline to pipe `stdout`, check chunk streams, and gracefully wait.
* **LLM Tool-Calling Stubbornness**: AI agents possess deep internal rules about pathing and scratch directories, leading to empty project output folders. We overcame this by completely bypassing their file-writing tools. We aggressively prompt them to output raw Markdown code blocks, and our native Rust parser in `orchestrate.rs` manually extracts and saves the final files.

---

## Installation & Usage

### Prerequisites
1. [Rust](https://rustup.rs/) (`curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh`)
2. `tmux` (`sudo apt install tmux` or `brew install tmux`)
3. At least one AI CLI agent in your path (e.g., `agy`).

### Installation
Clone the repository and install globally:
```bash
cargo install --path athena-cli --force
```

### Usage
1. Run the orchestrator:
   ```bash
   athena
   ```
2. You will be dropped into the Athena TUI. Type `/help` to see commands.
3. To start a project, type:
   ```bash
   /orch
   ```
   Paste your project prompt, type `EOF` on a new line, and hit enter.
4. Watch Athena build the plan, launch the tmux grid, and generate your code in parallel!

---
<div align="center">
  <i>Built with Rust. Designed for the Future of Coding.</i>
</div>
