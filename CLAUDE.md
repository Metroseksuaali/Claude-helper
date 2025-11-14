# CLAUDE.md - AI Assistant Guide for Claude Helper

**Last Updated**: 2025-11-14
**Project Version**: 0.1.0
**Status**: Alpha - Core features implemented, testing in progress

## Table of Contents

1. [Project Overview](#project-overview)
2. [Architecture & Design](#architecture--design)
3. [Codebase Structure](#codebase-structure)
4. [Development Setup](#development-setup)
5. [Code Conventions](#code-conventions)
6. [Key Components](#key-components)
7. [Testing Strategy](#testing-strategy)
8. [Common Development Tasks](#common-development-tasks)
9. [Important Implementation Details](#important-implementation-details)
10. [File Reference](#file-reference)
11. [Workflow Guidelines for AI Assistants](#workflow-guidelines-for-ai-assistants)

---

## Project Overview

### What is Claude Helper?

Claude Helper is a **high-performance Rust-based orchestration and optimization toolkit** for Claude Code. It provides:

- **Master Coder System**: Intelligent multi-agent orchestration for complex tasks
- **Real-time Token Tracking**: Live usage monitoring with cost analysis
- **Session Analysis**: AI-powered optimization detection to reduce token waste
- **Interactive TUI**: Beautiful terminal UI for monitoring and control
- **Learning System**: Improves from past sessions stored in SQLite

### Tech Stack

- **Language**: Rust (2021 Edition)
- **Runtime**: Tokio (async/await)
- **Database**: SQLite via sqlx (async, type-safe)
- **CLI Framework**: clap 4.5 with derive macros
- **TUI Framework**: ratatui + crossterm
- **HTTP Client**: reqwest with rustls-tls
- **Error Handling**: anyhow + thiserror
- **Logging**: tracing + tracing-subscriber

### Performance Characteristics

- **Execution Speed**: ~8ms for status line (15x faster than Node.js alternatives)
- **Binary Size**: ~12MB (stripped release build)
- **Startup Time**: <100ms cold start
- **Memory Usage**: 5-10MB typical usage
- **Build Time**: ~2 minutes clean, 50-60s incremental

### Project Goals

1. **Performance**: Native Rust for maximum speed and minimal resource usage
2. **Reliability**: Memory safety, comprehensive error handling
3. **Usability**: Rich CLI with multiple autonomy modes
4. **Extensibility**: Trait-based agent system for easy customization
5. **Learning**: Build knowledge from past sessions to optimize future work

---

## Architecture & Design

### Design Principles

1. **Modular Architecture**: Clear separation of concerns across 7 main modules
2. **Trait-Based Polymorphism**: Extensible agent system via the `Agent` trait
3. **Async-First**: Tokio-based async runtime throughout for I/O operations
4. **Type Safety**: Leverage Rust's type system to prevent bugs at compile-time
5. **Database-Backed Learning**: SQLite for persistence and historical analysis
6. **Configuration-Driven**: TOML-based config with sensible defaults

### Core Architectural Patterns

| Pattern | Usage | Location |
|---------|-------|----------|
| **Trait Objects** | Polymorphic agent execution | `src/agents/base.rs:14-36` |
| **Factory Pattern** | Dynamic agent creation | `src/master/agent_factory.rs:15-33` |
| **Builder Pattern** | Configuration, API clients | Throughout |
| **Repository Pattern** | Database abstraction | `src/db/mod.rs` |
| **Strategy Pattern** | Autonomy modes, capabilities | `src/master/mod.rs:18-29` |
| **Observer Pattern** | Progress tracking, status updates | `src/master/orchestrator.rs` |
| **Command Pattern** | CLI subcommand routing | `src/main.rs:131-245` |

### System Flow

```
User Request
    ↓
CLI Parser (clap)
    ↓
Command Router → [run, status, analyze, tui, etc.]
    ↓
Master Coder System
    ├─ Task Planner → Analyzes task, estimates complexity
    ├─ Agent Factory → Creates specialized agents
    ├─ Orchestrator → Executes plan (parallel/sequential)
    └─ Database → Stores results for learning
    ↓
Results & Learning
```

---

## Codebase Structure

### Directory Layout

```
Claude-helper/
├── src/
│   ├── agents/          # Agent system (trait, implementations, manager)
│   │   ├── base.rs         # Agent trait definition
│   │   ├── capabilities.rs # AgentCapability enum
│   │   ├── claude_agent.rs # Claude API integration
│   │   ├── manager.rs      # Agent stats and history
│   │   └── mod.rs          # Public exports
│   │
│   ├── analyzer/        # Session analysis and optimization
│   │   ├── session_parser.rs  # Parse Claude Code JSONL sessions
│   │   ├── optimizer.rs       # Optimization detection strategies
│   │   └── mod.rs             # Public exports
│   │
│   ├── config/          # Configuration management
│   │   ├── auth.rs         # Authentication (Claude Code, API key)
│   │   ├── mod.rs          # Config loading, saving, editing
│   │   └── defaults.rs     # Default configuration values
│   │
│   ├── db/              # Database layer
│   │   ├── schema.rs       # Table definitions and indexes
│   │   ├── mod.rs          # CRUD operations, queries
│   │   └── migrations.rs   # Schema migrations (future)
│   │
│   ├── master/          # Master Coder orchestration
│   │   ├── planner.rs        # Task analysis and planning
│   │   ├── agent_factory.rs  # Agent creation with custom prompts
│   │   ├── orchestrator.rs   # Execution engine (parallel/sequential)
│   │   └── mod.rs            # MasterCoder struct, autonomy modes
│   │
│   ├── statusline/      # Token usage tracking
│   │   ├── usage_tracker.rs  # Fetch and calculate usage stats
│   │   └── mod.rs            # Display formatting, status line
│   │
│   ├── tui/             # Terminal UI
│   │   ├── app.rs          # TUI application state and rendering
│   │   └── mod.rs          # Terminal setup/teardown
│   │
│   ├── lib.rs           # Library entry point (public API)
│   └── main.rs          # Binary entry point (CLI)
│
├── Cargo.toml           # Project manifest and dependencies
├── install.sh           # Installation script
├── README.md            # User-facing documentation
├── QUICKSTART.md        # Quick start guide
├── TESTING.md           # Testing status report
└── .gitignore           # Git ignore rules
```

### Module Dependencies

```
main.rs
  ├─ config
  ├─ master → planner, orchestrator, agent_factory
  ├─ statusline → usage_tracker
  ├─ analyzer → session_parser, optimizer
  ├─ tui → app
  └─ agents → manager

lib.rs (public API)
  ├─ config::Config
  ├─ master::MasterCoder
  ├─ statusline::StatusLine
  ├─ analyzer::SessionAnalyzer
  └─ tui::App
```

---

## Development Setup

### Prerequisites

- **Rust**: 1.70+ (install via [rustup](https://rustup.rs/))
- **SQLite**: System SQLite library (usually pre-installed)
- **Git**: For version control

### Initial Setup

```bash
# Clone the repository
git clone https://github.com/Metroseksuaali/Claude-helper.git
cd Claude-helper

# Build in debug mode
cargo build

# Run tests
cargo test

# Run with debug logging
RUST_LOG=debug cargo run -- status

# Build release version
cargo build --release
```

### Development Tools

```bash
# Format code (run before committing)
cargo fmt

# Run linter (run before committing)
cargo clippy

# Check for common issues
cargo clippy -- -D warnings

# Generate documentation
cargo doc --open

# Run specific tests
cargo test test_name

# Run with verbose output
cargo test -- --nocapture
```

### IDE Setup Recommendations

- **VSCode**: Install rust-analyzer extension
- **IntelliJ**: Rust plugin
- **Vim/Neovim**: rust.vim + coc-rust-analyzer

### Environment Variables

- `RUST_LOG`: Set logging level (trace, debug, info, warn, error)
- `ANTHROPIC_API_KEY`: API key for direct API access
- `EDITOR`: Editor for `config edit` command (defaults to nano)

---

## Code Conventions

### Naming Conventions

| Type | Convention | Example |
|------|------------|---------|
| Structs | PascalCase | `MasterCoder`, `AgentFactory` |
| Traits | PascalCase | `Agent` |
| Enums | PascalCase | `AgentCapability::CodeWriting` |
| Functions | snake_case | `analyze_task`, `execute_plan` |
| Constants | SCREAMING_SNAKE_CASE | `CREATE_TABLES`, `DEFAULT_BUDGET` |
| Modules | snake_case | `src/agent_factory.rs` |
| Type Aliases | PascalCase | `type Result<T> = anyhow::Result<T>` |

### Error Handling

**Use `anyhow::Result` for application code:**

```rust
use anyhow::{Result, Context};

pub async fn load_config() -> Result<Config> {
    let path = config_file()?;
    let content = fs::read_to_string(&path)
        .context("Failed to read config file")?;
    // ...
}
```

**Use `thiserror` for library errors with custom types:**

```rust
use thiserror::Error;

#[derive(Error, Debug)]
pub enum AgentError {
    #[error("Agent execution failed: {0}")]
    ExecutionFailed(String),

    #[error("Invalid capability: {0}")]
    InvalidCapability(String),
}
```

### Async/Await Style

**Always use async for I/O operations:**

```rust
// Good
pub async fn fetch_usage() -> Result<Usage> {
    let client = reqwest::Client::new();
    let response = client.get(url).send().await?;
    Ok(response.json().await?)
}

// Bad - blocking in async context
pub async fn bad_example() -> Result<Data> {
    std::thread::sleep(Duration::from_secs(1)); // Don't do this!
    Ok(data)
}
```

### Module Organization Pattern

**Each module follows this structure:**

```rust
// mod.rs - Public interface
mod submodule1;
mod submodule2;

pub use submodule1::PublicType;
pub use submodule2::PublicFunction;

// Private implementation stays in submodules
```

**Example: `src/agents/mod.rs:1-12`**

### Documentation Style

**Public APIs require doc comments:**

```rust
/// Represents an AI agent that can execute tasks.
///
/// # Examples
///
/// ```
/// let agent = ClaudeAgent::new("agent-1", AgentCapability::CodeWriting)?;
/// let result = agent.execute("Write a function").await?;
/// ```
pub trait Agent: Send + Sync {
    /// Returns the unique identifier for this agent.
    fn id(&self) -> &str;

    // ... more methods
}
```

### Testing Conventions

**Test organization:**

```rust
#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_complexity_calculation() {
        let analysis = TaskAnalysis::new("simple task");
        assert!(analysis.complexity <= 5);
    }

    #[tokio::test]
    async fn test_async_function() {
        let result = async_function().await;
        assert!(result.is_ok());
    }
}
```

---

## Key Components

### 1. Agent System

**Core Trait: `src/agents/base.rs:14-36`**

```rust
#[async_trait]
pub trait Agent: Send + Sync {
    fn id(&self) -> &str;
    fn agent_type(&self) -> &str;
    fn capability(&self) -> &AgentCapability;
    async fn execute(&mut self, task: &str) -> Result<AgentResult>;
    fn conversation_history(&self) -> Vec<String>;
    fn reset(&mut self);
}
```

**Agent Capabilities: `src/agents/capabilities.rs:3-14`**

Nine specialized agent types:
1. Architecture - High-level design
2. CodeWriting - Implementation
3. Testing - Test creation and execution
4. Security - Security audits
5. Documentation - API docs, README
6. Debugging - Bug investigation
7. Performance - Optimization
8. Migration - Database/code migrations
9. Review - Code review and quality

**Implementation: `src/agents/claude_agent.rs`**

- Uses Anthropic Messages API
- Model: `claude-sonnet-4-5-20250929`
- Max tokens: 8192 per request
- Maintains conversation history
- Custom system prompts per capability

### 2. Master Coder System

**Autonomy Modes: `src/master/mod.rs:18-29`**

```rust
pub enum AutonomyMode {
    Conservative,  // Frequent approval
    Balanced,      // Important points only (default)
    Trust,         // Fully automatic
    Interactive    // Every step control
}
```

**Task Planning: `src/master/planner.rs`**

Key responsibilities:
- **Task Analysis** (lines 7-17): Complexity scoring (0-10), token estimation
- **Capability Detection** (lines 111-142): Keyword-based requirement detection
- **Team Composition** (lines 188-290): Dynamic agent selection
- **Dependency Resolution** (lines 292-341): Build execution phases with topological sort

**Orchestration: `src/master/orchestrator.rs`**

Execution strategies:
- **Parallel Execution** (lines 129-217): Concurrent agents with semaphore control
- **Sequential Execution** (lines 219-288): Dependency-ordered execution
- **Progress Tracking**: Multi-progress bars with `indicatif`

**Agent Factory: `src/master/agent_factory.rs`**

- Creates agents from `AgentSpec` (lines 15-33)
- Generates specialized system prompts (lines 57-165)
- Customizes instructions per capability

### 3. Database Layer

**Location**: `~/.config/claude-helper/db/claude-helper.db`

**Schema: `src/db/schema.rs`**

Three main tables:

1. **task_executions** (lines 2-13)
   - Stores completed tasks for learning
   - Fields: complexity, estimated/actual tokens, success, JSON data
   - Index on created_at

2. **agent_executions** (lines 15-25)
   - Individual agent performance tracking
   - Fields: agent_id, type, capability, tokens_used, execution_time
   - Indexes on created_at and agent_type

3. **optimizations** (lines 27-35)
   - Suggested optimizations catalog
   - Fields: type, title, description, estimated_savings, applied

**Operations: `src/db/mod.rs`**

- `save_task_execution()` (lines 41-70)
- `save_agent_execution()` (lines 144-173)
- `get_agent_stats()` (lines 73-108): Aggregated statistics
- `get_agent_history()` (lines 110-142): Recent executions
- `get_hourly_breakdown()` (lines 176-207): Time-series data

### 4. Session Analysis

**Session Parser: `src/analyzer/session_parser.rs`**

- Parses Claude Code JSONL sessions from `~/.claude/sessions/`
- Extracts messages, tool calls, file accesses
- Methods:
  - `find_recent_sessions()` (lines 42-72)
  - `parse()` (lines 86-152)

**Optimizer: `src/analyzer/optimizer.rs`**

Six optimization strategies:

1. **QuickCommand**: Repetitive bash commands → scripts
2. **ParameterizedScript**: Similar commands → parameterized scripts
3. **FileMerge**: Frequently co-accessed files → merge suggestion
4. **FileSplit**: Large files with multiple concerns → split suggestion
5. **ContextPruning**: Redundant context → pruning suggestion
6. **ToolCallBatching**: Repeated tool calls → batch suggestion

Detection algorithms:
- Bash chain detection (lines 55-101)
- File access pattern analysis (lines 103-146)
- Tool call repetition (lines 160-200)

### 5. Configuration Management

**Config Structure: `src/config/mod.rs:10-53`**

```toml
[auth]
method = "claude_code"  # or "api_key"
api_key = "sk-..."      # optional

[master_coder]
default_mode = "balanced"
max_parallel_agents = 5
token_budget = 50000
enable_learning = true

[statusline]
update_interval = 30
show_costs = true
api_endpoint = "https://claude.ai/api"

[analyzer]
history_depth = 50
min_savings_threshold = 500
```

**File Locations:**
- Config: `~/.config/claude-helper/config.toml`
- Database: `~/.config/claude-helper/db/claude-helper.db`
- Cache: `~/.cache/claude-helper/`
- Logs: `~/.cache/claude-helper/logs/`

**Authentication: `src/config/auth.rs`**

Two methods:
1. **ClaudeCode**: Reads `~/.claude/sessions/sessionKey`
2. **ApiKey**: Direct Anthropic API key

### 6. Status Line & Token Tracking

**Usage Tracker: `src/statusline/usage_tracker.rs`**

Tracks:
- 5-hour block usage (20,000 token limit)
- 7-day rolling usage (200,000 token limit)
- Cost calculations ($3/M input, $15/M output)
- Burn rate estimation

**Display Modes:**
- Compact (lines 21-41): For status bars
- Detailed (lines 44-90): Full breakdown with progress bars
- Watch (lines 92-109): Real-time monitoring

### 7. Terminal UI

**TUI Framework: `src/tui/mod.rs`**

Setup:
- Raw mode terminal
- Alternate screen
- Mouse capture
- Crossterm for cross-platform support

**App: `src/tui/app.rs`**

Three tabs:
1. **Usage**: Token tracking graphs
2. **Optimization**: Suggestions list
3. **AgentHistory**: Recent agent executions

Navigation:
- Arrow keys: Switch tabs, scroll
- `q` / `Esc`: Quit
- `r`: Refresh data

---

## Testing Strategy

### Current Testing Status

**From `TESTING.md`:**

✅ **Manually Tested (No API Required):**
- Build system (debug and release)
- CLI interface (--version, --help, all commands)
- Configuration system (auto-creation, defaults)
- Database initialization (schema, tables, indexes)
- Agent system structure (list, stats commands)
- Status line rendering (with mock data)
- Error handling and user feedback

❌ **Not Yet Tested (Requires API Access):**
- Master Coder orchestration end-to-end
- Real agent execution with Claude API
- Session file parsing from actual Claude Code sessions
- Live token tracking with real usage data
- TUI interactive features
- Optimization detection accuracy

### Test Organization

**Current approach:**
- Manual testing documented in `TESTING.md`
- Mock data for development
- No automated tests yet

**Planned testing structure:**

```rust
// Unit tests in each module
#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_logic() { /* ... */ }

    #[tokio::test]
    async fn test_async() { /* ... */ }
}

// Integration tests in tests/ directory
// tests/integration_test.rs
use claude_helper::MasterCoder;

#[tokio::test]
async fn test_full_workflow() {
    // End-to-end test
}
```

### Testing Recommendations

**High Priority:**

1. **Unit Tests**
   - Task complexity calculation (planner.rs)
   - Capability detection logic (planner.rs)
   - Dependency resolution (planner.rs)
   - Cost calculations (usage_tracker.rs)
   - Optimization detection algorithms (optimizer.rs)

2. **Integration Tests**
   - Database operations (save, retrieve)
   - Configuration loading/saving
   - Agent creation and prompt generation
   - Session parsing with sample JSONL files

3. **Mock-Based Tests**
   - HTTP client with mockito
   - File system operations with tempfile
   - Claude API responses

**Example Test Structure:**

```rust
// In src/master/planner.rs
#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_simple_task_complexity() {
        let analysis = analyze_task_sync("Fix typo in README");
        assert!(analysis.complexity <= 3);
        assert!(analysis.required_capabilities.contains(&AgentCapability::Documentation));
    }

    #[test]
    fn test_complex_task_analysis() {
        let analysis = analyze_task_sync(
            "Implement OAuth2 authentication with tests and security audit"
        );
        assert!(analysis.complexity >= 7);
        assert!(analysis.required_capabilities.len() >= 3);
    }

    #[test]
    fn test_dependency_resolution() {
        let specs = vec![
            AgentSpec {
                id: "arch".into(),
                capability: AgentCapability::Architecture,
                depends_on: vec![],
                /* ... */
            },
            AgentSpec {
                id: "code".into(),
                capability: AgentCapability::CodeWriting,
                depends_on: vec!["arch".into()],
                /* ... */
            },
        ];
        let phases = build_execution_phases(&specs);
        assert_eq!(phases.len(), 2); // Architecture first, then CodeWriting
    }
}
```

---

## Common Development Tasks

### Adding a New Agent Capability

1. **Add to capabilities enum** (`src/agents/capabilities.rs:3-14`)

```rust
pub enum AgentCapability {
    // ... existing ...
    YourNewCapability,
}
```

2. **Add description and emoji** (capabilities.rs:16-43)

```rust
pub fn description(&self) -> &str {
    match self {
        // ... existing ...
        Self::YourNewCapability => "Description of capability",
    }
}

pub fn emoji(&self) -> &str {
    match self {
        // ... existing ...
        Self::YourNewCapability => "🎯",
    }
}
```

3. **Add to factory system prompts** (`src/master/agent_factory.rs:57-165`)

```rust
fn generate_system_prompt(capability: &AgentCapability) -> String {
    match capability {
        // ... existing ...
        AgentCapability::YourNewCapability => {
            "You are a specialized agent for [purpose].\n\
             Your responsibilities:\n\
             - Task 1\n\
             - Task 2\n\
             \n\
             Guidelines:\n\
             - Guideline 1\n\
             - Guideline 2".to_string()
        }
    }
}
```

4. **Update task planner keyword detection** (`src/master/planner.rs:111-142`)

```rust
fn detect_capabilities(task: &str) -> Vec<AgentCapability> {
    let task_lower = task.to_lowercase();
    let mut capabilities = Vec::new();

    // ... existing checks ...

    if task_lower.contains("your_keyword") {
        capabilities.push(AgentCapability::YourNewCapability);
    }

    capabilities
}
```

### Adding a New CLI Command

1. **Add to CLI enum** (`src/main.rs`)

```rust
#[derive(Parser)]
#[command(name = "claude-helper")]
enum Cli {
    // ... existing commands ...

    /// Your new command description
    YourCommand {
        /// Command-specific arguments
        #[arg(short, long)]
        option: Option<String>,
    },
}
```

2. **Add handler in main** (`src/main.rs:148-205`)

```rust
async fn main() -> Result<()> {
    // ... setup ...

    match cli {
        // ... existing handlers ...

        Cli::YourCommand { option } => {
            handle_your_command(option).await?;
        }
    }
}

async fn handle_your_command(option: Option<String>) -> Result<()> {
    // Implementation
    Ok(())
}
```

### Adding a New Optimization Strategy

1. **Add to optimization types** (`src/analyzer/optimizer.rs:5-23`)

```rust
pub enum OptimizationType {
    // ... existing ...
    YourOptimization,
}
```

2. **Implement detection** (`src/analyzer/optimizer.rs`)

```rust
fn detect_your_optimization(session: &SessionData) -> Vec<Optimization> {
    let mut optimizations = Vec::new();

    // Your detection logic
    if some_condition {
        optimizations.push(Optimization {
            optimization_type: OptimizationType::YourOptimization,
            title: "Title".to_string(),
            description: "Description".to_string(),
            estimated_savings: calculate_savings(),
        });
    }

    optimizations
}
```

3. **Add to analysis pipeline** (`src/analyzer/optimizer.rs:34-53`)

```rust
pub fn analyze_session(session: &SessionData) -> Vec<Optimization> {
    let mut all_optimizations = Vec::new();

    // ... existing detectors ...
    all_optimizations.extend(detect_your_optimization(session));

    // ... filtering ...
    all_optimizations
}
```

### Adding Configuration Options

1. **Add to config struct** (`src/config/mod.rs:10-53`)

```rust
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Config {
    // ... existing fields ...

    pub your_section: YourSectionConfig,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct YourSectionConfig {
    pub option1: String,
    pub option2: u32,
}
```

2. **Add defaults** (`src/config/mod.rs` or `defaults.rs`)

```rust
impl Default for YourSectionConfig {
    fn default() -> Self {
        Self {
            option1: "default_value".to_string(),
            option2: 42,
        }
    }
}
```

3. **Use in code**

```rust
let config = Config::load()?;
let value = config.your_section.option1;
```

### Database Schema Changes

1. **Update schema** (`src/db/schema.rs`)

```rust
pub const CREATE_NEW_TABLE: &str = "
CREATE TABLE IF NOT EXISTS new_table (
    id INTEGER PRIMARY KEY AUTOINCREMENT,
    field1 TEXT NOT NULL,
    field2 INTEGER,
    created_at DATETIME DEFAULT CURRENT_TIMESTAMP
);

CREATE INDEX idx_new_table_field1 ON new_table(field1);
";
```

2. **Add to initialization** (`src/db/mod.rs`)

```rust
pub async fn initialize_db() -> Result<SqlitePool> {
    // ... existing setup ...

    sqlx::query(schema::CREATE_TABLES).execute(&pool).await?;
    sqlx::query(schema::CREATE_NEW_TABLE).execute(&pool).await?; // Add this

    Ok(pool)
}
```

3. **Add operations**

```rust
pub async fn save_to_new_table(
    pool: &SqlitePool,
    field1: &str,
    field2: i64,
) -> Result<i64> {
    let result = sqlx::query(
        "INSERT INTO new_table (field1, field2) VALUES (?, ?)"
    )
    .bind(field1)
    .bind(field2)
    .execute(pool)
    .await?;

    Ok(result.last_insert_rowid())
}
```

---

## Important Implementation Details

### Complexity Calculation Algorithm

**Location**: `src/master/planner.rs:76-109`

**Logic**:
1. Start with base complexity: 3
2. Add +2 for high complexity keywords (refactor, migrate, security, etc.)
3. Add +1 for medium keywords (implement, create, api, etc.)
4. Add +1 if multiple requirements detected
5. Clamp to 0-10 range

**Keyword Lists**:
- **High complexity**: refactor, migrate, architecture, security, performance, database, integration, deployment
- **Medium complexity**: implement, create, add, api, authentication, testing, documentation
- **Low complexity**: fix, update, change, modify, improve

### Token Estimation

**Location**: `src/master/planner.rs` (estimate_tokens method)

**Formula**:
- Base: 500 tokens
- Per complexity point: 2000 tokens
- Per file: 1500 tokens
- Per capability: 1000 tokens

**Example**: Complexity 7, 5 files, 3 capabilities = 500 + (7 × 2000) + (5 × 1500) + (3 × 1000) = 25,000 tokens

### Parallel Execution Control

**Location**: `src/master/orchestrator.rs:129-217`

**Mechanism**:
- Uses `tokio::sync::Semaphore` for concurrency control
- Max concurrent agents configurable (default: 5)
- Agents acquire permit before execution
- Progress bars track each agent independently

**Key Code**:
```rust
let semaphore = Arc::new(Semaphore::new(max_agents));
for spec in agents {
    let permit = semaphore.clone().acquire_owned().await?;
    let handle = tokio::spawn(async move {
        let result = agent.execute(&spec.task).await;
        drop(permit); // Release when done
        result
    });
    handles.push(handle);
}
```

### Cost Calculation

**Location**: `src/statusline/usage_tracker.rs:89-117`

**Pricing** (as of 2025):
- Input tokens: $3.00 per million
- Output tokens: $15.00 per million
- Cache reads: Free
- Cache writes: $3.75 per million (1.25× input rate)

**Formula**:
```rust
cost = (input_tokens as f64 / 1_000_000.0 * 3.0)
     + (output_tokens as f64 / 1_000_000.0 * 15.0)
     + (cache_writes as f64 / 1_000_000.0 * 3.75)
```

### Session File Format

**Location**: `~/.claude/sessions/*.jsonl`

**Format**: JSON Lines (one JSON object per line)

**Example Entry**:
```json
{
  "type": "message",
  "role": "user",
  "content": "Fix the bug in main.rs",
  "timestamp": "2025-11-14T12:00:00Z"
}
{
  "type": "tool_call",
  "tool": "Read",
  "file_path": "/path/to/main.rs",
  "timestamp": "2025-11-14T12:00:01Z"
}
```

**Parser**: `src/analyzer/session_parser.rs:86-152`

### Build Optimizations

**Location**: `Cargo.toml:64-68`

**Release Profile**:
```toml
[profile.release]
opt-level = 3           # Maximum optimization
lto = true              # Link-time optimization
codegen-units = 1       # Single codegen unit for better optimization
strip = true            # Remove debug symbols
```

**Impact**:
- Binary size: ~150MB (debug) → ~12MB (release)
- Execution speed: ~40ms → ~8ms for status line
- Compile time: Slower, but worth it for production

---

## File Reference

### Entry Points

| File | Purpose | Key Functions |
|------|---------|---------------|
| `src/main.rs` | Binary entry point | `main()` (131-245), command routing |
| `src/lib.rs` | Library entry point | Public API exports |

### Core Modules

| File | Purpose | Key Types/Functions |
|------|---------|---------------------|
| **Agents** | | |
| `src/agents/base.rs` | Agent trait | `Agent` trait (14-36), `AgentResult` |
| `src/agents/capabilities.rs` | Capability types | `AgentCapability` enum (3-14) |
| `src/agents/claude_agent.rs` | Claude API integration | `ClaudeAgent` (42-107) |
| `src/agents/manager.rs` | Agent stats/history | `show_stats()`, `show_history()` |
| **Master Coder** | | |
| `src/master/mod.rs` | Orchestration core | `MasterCoder`, `AutonomyMode` |
| `src/master/planner.rs` | Task analysis | `analyze_task()` (28-61), `TaskAnalysis` |
| `src/master/orchestrator.rs` | Execution engine | `execute_plan()` (68-127) |
| `src/master/agent_factory.rs` | Agent creation | `create_agent()` (15-33) |
| **Database** | | |
| `src/db/schema.rs` | Table definitions | `CREATE_TABLES` SQL |
| `src/db/mod.rs` | Database operations | `save_task_execution()`, `get_agent_stats()` |
| **Analyzer** | | |
| `src/analyzer/session_parser.rs` | Session parsing | `parse()` (86-152), `SessionData` |
| `src/analyzer/optimizer.rs` | Optimization detection | `analyze_session()` (34-53) |
| **Config** | | |
| `src/config/mod.rs` | Config management | `load()`, `save()`, `Config` struct |
| `src/config/auth.rs` | Authentication | `get_session_token()` (61-99) |
| **Status Line** | | |
| `src/statusline/usage_tracker.rs` | Usage tracking | `fetch_usage()` (56-65), `Usage` |
| `src/statusline/mod.rs` | Display formatting | `show_status()` (44-90) |
| **TUI** | | |
| `src/tui/mod.rs` | Terminal setup | `setup_terminal()` (18-48) |
| `src/tui/app.rs` | TUI application | `App` struct, `run()` (48-53) |

### Configuration Files

| File | Location | Purpose |
|------|----------|---------|
| Config | `~/.config/claude-helper/config.toml` | User configuration |
| Database | `~/.config/claude-helper/db/claude-helper.db` | SQLite database |
| Sessions | `~/.claude/sessions/*.jsonl` | Claude Code sessions |
| Logs | `~/.cache/claude-helper/logs/` | Application logs |

### Documentation

| File | Purpose |
|------|---------|
| `README.md` | User-facing documentation, features, usage |
| `QUICKSTART.md` | Quick start guide, first-time setup |
| `TESTING.md` | Testing status report, manual test results |
| `CLAUDE.md` | This file - AI assistant guide |

---

## Workflow Guidelines for AI Assistants

### When Working on This Codebase

1. **Always run `cargo fmt` and `cargo clippy` before committing**
   ```bash
   cargo fmt
   cargo clippy -- -D warnings
   ```

2. **Use proper error handling with context**
   ```rust
   use anyhow::{Result, Context};

   fs::read_to_string(path)
       .context(format!("Failed to read config from {:?}", path))?
   ```

3. **Keep functions focused and modular**
   - Single responsibility principle
   - Functions should be < 50 lines ideally
   - Extract complex logic into helper functions

4. **Add documentation for public APIs**
   - Use `///` for public items
   - Include examples when helpful
   - Document panics, errors, and safety requirements

5. **Prefer async for I/O operations**
   - Use `tokio::fs` instead of `std::fs`
   - Use `reqwest` for HTTP
   - Use `sqlx` for database (already async)

6. **Test with mock data first**
   - Most features can be tested without real API access
   - Use mock data in `usage_tracker.rs:119-134` as example
   - Add unit tests for business logic

7. **Update TESTING.md when testing new features**
   - Document what was tested
   - Note any issues or limitations
   - Update the "Not Yet Tested" section

### Making Changes to Core Systems

#### Modifying Agent System
- **DO**: Add new capabilities following the pattern in capabilities.rs
- **DO**: Update all match statements when adding capabilities
- **DON'T**: Break the `Agent` trait interface without careful consideration
- **TEST**: Ensure agent creation and prompt generation still work

#### Modifying Master Coder
- **DO**: Maintain backward compatibility with existing autonomy modes
- **DO**: Keep orchestration logic separate from agent logic
- **DON'T**: Hardcode agent types - use the capability system
- **TEST**: Test with different autonomy modes and task complexities

#### Modifying Database
- **DO**: Create migrations for schema changes (future feature)
- **DO**: Add indexes for frequently queried columns
- **DON'T**: Remove existing tables/columns (breaks existing DBs)
- **TEST**: Test with fresh DB and existing DB

#### Modifying Configuration
- **DO**: Provide sensible defaults for new options
- **DO**: Document new options in README and QUICKSTART
- **DON'T**: Remove existing config options without deprecation period
- **TEST**: Test config loading with missing/invalid values

### Common Pitfalls to Avoid

1. **Blocking in async context**
   ```rust
   // BAD
   async fn bad_example() {
       std::thread::sleep(Duration::from_secs(1)); // Blocks the runtime!
   }

   // GOOD
   async fn good_example() {
       tokio::time::sleep(Duration::from_secs(1)).await; // Async sleep
   }
   ```

2. **Not using context with errors**
   ```rust
   // BAD
   let content = fs::read_to_string(path)?; // What file failed?

   // GOOD
   let content = fs::read_to_string(&path)
       .context(format!("Failed to read {:?}", path))?;
   ```

3. **Hardcoding paths**
   ```rust
   // BAD
   let db = "/home/user/.config/claude-helper/db/claude-helper.db";

   // GOOD
   let db = config::db_file()?; // Uses dirs crate for platform-specific paths
   ```

4. **Ignoring clippy warnings**
   - Always address or explicitly allow warnings
   - Use `#[allow(clippy::lint_name)]` with justification comment
   - Run `cargo clippy -- -D warnings` to catch all issues

5. **Not handling edge cases**
   - Empty inputs
   - Missing files
   - Network failures
   - Invalid JSON
   - Database connection errors

### Debugging Tips

1. **Enable verbose logging**
   ```bash
   RUST_LOG=debug cargo run -- <command>
   RUST_LOG=trace cargo run -- <command>  # Even more verbose
   ```

2. **Use structured logging**
   ```rust
   use tracing::{info, debug, warn, error};

   debug!(agent_id = %agent.id(), "Executing agent task");
   info!(tokens_used = result.tokens_used, "Agent completed successfully");
   ```

3. **Database inspection**
   ```bash
   sqlite3 ~/.config/claude-helper/db/claude-helper.db
   .tables
   .schema task_executions
   SELECT * FROM agent_executions LIMIT 10;
   ```

4. **Test with mock data**
   - See `usage_tracker.rs:119-134` for mock usage data
   - Add mock responses for HTTP calls using mockito
   - Use tempfile for filesystem tests

### Performance Considerations

1. **Database queries**
   - Use indexes for frequently queried columns
   - Limit result sets with LIMIT clause
   - Use prepared statements (sqlx does this automatically)

2. **Parallel execution**
   - Respect the semaphore limits
   - Don't spawn unbounded tasks
   - Use `tokio::spawn` for CPU-bound work

3. **Memory usage**
   - Be cautious with large vectors
   - Stream large files instead of loading entirely
   - Use references instead of cloning when possible

4. **Binary size**
   - The release profile already optimizes aggressively
   - Avoid pulling in large dependencies
   - Consider feature flags for optional dependencies

### Contributing Workflow

1. **Before starting**
   - Check existing issues and PRs
   - Discuss major changes in an issue first
   - Read CONTRIBUTING.md (when it exists)

2. **Development**
   - Create feature branch from main
   - Make focused, atomic commits
   - Write clear commit messages
   - Test thoroughly

3. **Before committing**
   ```bash
   cargo fmt
   cargo clippy
   cargo test
   cargo build --release
   ```

4. **Commit message format**
   ```
   category: Brief description

   Longer explanation if needed.

   - Bullet points for multiple changes
   - Another change

   Fixes #123
   ```

   Categories: feat, fix, docs, style, refactor, test, chore

5. **Pull request**
   - Clear description of changes
   - Link to related issues
   - Include testing notes
   - Update TESTING.md if applicable

---

## Quick Reference

### Build Commands

```bash
cargo build                    # Debug build
cargo build --release         # Release build (optimized)
cargo run -- status           # Run with arguments
cargo test                    # Run tests
cargo fmt                     # Format code
cargo clippy                  # Lint
cargo doc --open              # Generate and view docs
```

### File Locations

```
~/.config/claude-helper/config.toml       # Configuration
~/.config/claude-helper/db/               # Database
~/.cache/claude-helper/logs/              # Logs
~/.claude/sessions/                       # Claude Code sessions
```

### Important Constants

| Constant | Value | Location |
|----------|-------|----------|
| Default model | claude-sonnet-4-5-20250929 | agents/claude_agent.rs |
| Max tokens/request | 8192 | agents/claude_agent.rs |
| 5-hour limit | 20,000 tokens | statusline/usage_tracker.rs |
| 7-day limit | 200,000 tokens | statusline/usage_tracker.rs |
| Input token cost | $3/M | statusline/usage_tracker.rs |
| Output token cost | $15/M | statusline/usage_tracker.rs |

### Key Dependencies

| Crate | Version | Purpose |
|-------|---------|---------|
| tokio | 1.41 | Async runtime |
| clap | 4.5 | CLI parsing |
| sqlx | 0.8 | Async SQLite |
| reqwest | 0.12 | HTTP client |
| anyhow | 1.0 | Error handling |
| serde | 1.0 | Serialization |
| ratatui | 0.29 | Terminal UI |

---

## Version History

| Version | Date | Major Changes |
|---------|------|---------------|
| 0.1.0 | 2025-11-14 | Initial release, core features implemented |

---

## Additional Resources

- **Repository**: https://github.com/Metroseksuaali/Claude-helper
- **Issues**: https://github.com/Metroseksuaali/Claude-helper/issues
- **Discussions**: https://github.com/Metroseksuaali/Claude-helper/discussions
- **Rust Book**: https://doc.rust-lang.org/book/
- **Tokio Tutorial**: https://tokio.rs/tokio/tutorial
- **sqlx Documentation**: https://docs.rs/sqlx/

---

**This document is maintained for AI assistants working on the Claude Helper codebase. When making significant architectural changes, update this file accordingly.**
