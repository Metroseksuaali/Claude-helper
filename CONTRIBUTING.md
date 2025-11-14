# Contributing to Claude Helper

Thank you for your interest in contributing to Claude Helper! This document provides guidelines and information to help you contribute effectively.

## Table of Contents

- [Code of Conduct](#code-of-conduct)
- [Getting Started](#getting-started)
- [Development Setup](#development-setup)
- [How to Contribute](#how-to-contribute)
- [Coding Standards](#coding-standards)
- [Testing Guidelines](#testing-guidelines)
- [Pull Request Process](#pull-request-process)
- [Project Architecture](#project-architecture)
- [Common Development Tasks](#common-development-tasks)

## Code of Conduct

We are committed to providing a welcoming and inclusive environment for all contributors. Please be respectful, constructive, and professional in all interactions.

### Expected Behavior

- Use welcoming and inclusive language
- Be respectful of differing viewpoints and experiences
- Gracefully accept constructive criticism
- Focus on what is best for the community
- Show empathy towards other community members

## Getting Started

### Prerequisites

Before you begin, ensure you have the following installed:

- **Rust**: Version 1.70 or higher ([install via rustup](https://rustup.rs/))
- **Git**: For version control
- **SQLite**: System SQLite library (usually pre-installed on Linux/macOS)

### Fork and Clone

1. Fork the repository on GitHub
2. Clone your fork locally:
   ```bash
   git clone https://github.com/YOUR_USERNAME/Claude-helper.git
   cd Claude-helper
   ```
3. Add the upstream repository:
   ```bash
   git remote add upstream https://github.com/Metroseksuaali/Claude-helper.git
   ```

## Development Setup

### Initial Build

```bash
# Build in debug mode
cargo build

# Run tests
cargo test

# Run with debug logging
RUST_LOG=debug cargo run -- status

# Build optimized release version
cargo build --release
```

### Development Tools

We recommend using the following tools during development:

```bash
# Format code (REQUIRED before committing)
cargo fmt

# Run linter (REQUIRED before committing)
cargo clippy

# Check for common issues with warnings as errors
cargo clippy -- -D warnings

# Generate and view documentation
cargo doc --open

# Run specific tests
cargo test test_name

# Run tests with output
cargo test -- --nocapture
```

### IDE Setup

**Recommended IDEs:**

- **VSCode**: Install the [rust-analyzer](https://marketplace.visualstudio.com/items?itemName=rust-lang.rust-analyzer) extension
- **IntelliJ IDEA**: Install the Rust plugin
- **Vim/Neovim**: Use rust.vim + coc-rust-analyzer

**Recommended VSCode Settings:**

```json
{
  "rust-analyzer.checkOnSave.command": "clippy",
  "editor.formatOnSave": true,
  "[rust]": {
    "editor.defaultFormatter": "rust-lang.rust-analyzer"
  }
}
```

## How to Contribute

### Types of Contributions

We welcome various types of contributions:

1. **Bug Reports**: Found a bug? Please report it!
2. **Feature Requests**: Have an idea? We'd love to hear it!
3. **Code Contributions**: Bug fixes, new features, optimizations
4. **Documentation**: Improvements to README, guides, code comments
5. **Testing**: Writing tests, manual testing, CI/CD improvements
6. **Performance**: Optimizations and benchmarks

### Before You Start

For **major changes** (new features, architectural changes):
1. Open an issue to discuss your proposal
2. Wait for maintainer feedback
3. Get approval before starting implementation

For **minor changes** (bug fixes, documentation, small improvements):
1. Feel free to submit a PR directly
2. Reference any related issues

### Finding Issues to Work On

- Browse the [issue tracker](https://github.com/Metroseksuaali/Claude-helper/issues)
- Look for issues labeled `good first issue` or `help wanted`
- Check the project roadmap in TESTING.md for planned features

## Coding Standards

### Naming Conventions

| Type | Convention | Example |
|------|------------|---------|
| Structs | PascalCase | `MasterCoder`, `AgentFactory` |
| Traits | PascalCase | `Agent` |
| Enums | PascalCase | `AgentCapability::CodeWriting` |
| Functions | snake_case | `analyze_task`, `execute_plan` |
| Constants | SCREAMING_SNAKE_CASE | `CREATE_TABLES`, `DEFAULT_BUDGET` |
| Modules | snake_case | `agent_factory.rs` |

### Code Style

We follow the [Rust Style Guide](https://doc.rust-lang.org/nightly/style-guide/). Key points:

1. **Use `cargo fmt`** - All code must be formatted before committing
2. **Pass `cargo clippy`** - No warnings allowed in PRs
3. **Keep functions focused** - Single responsibility, ideally < 50 lines
4. **Use meaningful names** - Clear, descriptive variable and function names
5. **Add documentation** - Public APIs require doc comments

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

**Use `thiserror` for library errors:**

```rust
use thiserror::Error;

#[derive(Error, Debug)]
pub enum AgentError {
    #[error("Agent execution failed: {0}")]
    ExecutionFailed(String),
}
```

### Async/Await

**Always use async for I/O operations:**

```rust
// Good ✓
pub async fn fetch_usage() -> Result<Usage> {
    let client = reqwest::Client::new();
    let response = client.get(url).send().await?;
    Ok(response.json().await?)
}

// Bad ✗ - blocking in async context
pub async fn bad_example() -> Result<Data> {
    std::thread::sleep(Duration::from_secs(1)); // Don't do this!
    Ok(data)
}
```

**Use tokio equivalents:**
- `tokio::fs` instead of `std::fs`
- `tokio::time::sleep` instead of `std::thread::sleep`
- `tokio::spawn` for concurrent tasks

### Documentation

**All public APIs require documentation:**

```rust
/// Represents an AI agent that can execute tasks.
///
/// # Examples
///
/// ```
/// let agent = ClaudeAgent::new("agent-1", AgentCapability::CodeWriting)?;
/// let result = agent.execute("Write a function").await?;
/// ```
///
/// # Errors
///
/// Returns an error if the agent fails to execute the task.
pub trait Agent: Send + Sync {
    /// Returns the unique identifier for this agent.
    fn id(&self) -> &str;

    // ... more methods
}
```

**Use doc comments:**
- `///` for items (functions, structs, enums)
- `//!` for module-level documentation
- Include examples when helpful
- Document errors, panics, and safety requirements

## Testing Guidelines

### Test Organization

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

### What to Test

**High Priority:**

1. **Business Logic**
   - Task complexity calculation
   - Capability detection
   - Dependency resolution
   - Cost calculations
   - Optimization detection

2. **Data Operations**
   - Database save/retrieve operations
   - Configuration loading/saving
   - Session parsing

3. **Edge Cases**
   - Empty inputs
   - Missing files
   - Network failures
   - Invalid JSON
   - Database errors

### Testing Best Practices

- **Write tests for new features** - All new code should include tests
- **Test edge cases** - Don't just test the happy path
- **Use descriptive test names** - `test_handles_empty_input_gracefully()`
- **Keep tests focused** - One test per behavior
- **Use mock data** - Most features can be tested without API access
- **Document test setup** - Explain complex test scenarios

### Running Tests

```bash
# Run all tests
cargo test

# Run specific test
cargo test test_name

# Run tests with output
cargo test -- --nocapture

# Run tests with logging
RUST_LOG=debug cargo test
```

## Pull Request Process

### Before Submitting

1. **Sync with upstream:**
   ```bash
   git fetch upstream
   git rebase upstream/main
   ```

2. **Run pre-submission checks:**
   ```bash
   cargo fmt
   cargo clippy -- -D warnings
   cargo test
   cargo build --release
   ```

3. **Ensure your changes:**
   - Follow the coding standards
   - Include tests for new functionality
   - Update documentation as needed
   - Don't break existing functionality

### Commit Message Format

Use clear, descriptive commit messages:

```
category: Brief description (50 chars or less)

More detailed explanation if needed (wrap at 72 chars).

- Bullet points for multiple changes
- Another change
- Reference issues like #123

Fixes #123
Closes #456
```

**Categories:**
- `feat`: New feature
- `fix`: Bug fix
- `docs`: Documentation changes
- `style`: Code style changes (formatting, etc.)
- `refactor`: Code refactoring
- `test`: Adding or updating tests
- `chore`: Maintenance tasks
- `perf`: Performance improvements

**Examples:**

```
feat: Add new optimization detection for file merging

Implements detection for frequently co-accessed files that
could be merged to reduce context switching.

- Add file access pattern tracking
- Implement merge suggestion algorithm
- Add tests for edge cases

Closes #42
```

```
fix: Resolve database connection pool exhaustion

The connection pool was not properly releasing connections
after failed queries, leading to exhaustion under load.

Fixes #78
```

### Pull Request Guidelines

1. **Create a descriptive PR:**
   - Clear title summarizing the change
   - Detailed description of what and why
   - Link to related issues
   - Include testing notes

2. **PR Description Template:**
   ```markdown
   ## Description
   Brief summary of changes

   ## Motivation
   Why is this change needed?

   ## Changes
   - List of specific changes
   - Another change

   ## Testing
   - How was this tested?
   - Manual testing steps
   - New test coverage

   ## Related Issues
   Fixes #123
   Related to #456

   ## Checklist
   - [ ] Code follows project style guidelines
   - [ ] Tests pass locally
   - [ ] Documentation updated
   - [ ] Commit messages are clear
   ```

3. **Respond to feedback:**
   - Address all review comments
   - Ask questions if anything is unclear
   - Push additional commits to address feedback

4. **Keep PRs focused:**
   - One feature or fix per PR
   - Avoid mixing unrelated changes
   - Split large changes into multiple PRs

### After Your PR is Merged

1. **Delete your branch:**
   ```bash
   git branch -d feature-branch
   git push origin --delete feature-branch
   ```

2. **Update your fork:**
   ```bash
   git checkout main
   git pull upstream main
   git push origin main
   ```

## Project Architecture

### Overview

Claude Helper is structured into seven main modules:

```
src/
├── agents/          # Agent system (trait, implementations)
├── analyzer/        # Session analysis and optimization
├── config/          # Configuration management
├── db/              # Database layer (SQLite)
├── master/          # Master Coder orchestration
├── statusline/      # Token usage tracking
└── tui/             # Terminal UI
```

### Key Design Patterns

- **Trait-Based Polymorphism**: `Agent` trait for extensible agent system
- **Factory Pattern**: Dynamic agent creation
- **Repository Pattern**: Database abstraction
- **Strategy Pattern**: Autonomy modes, capabilities
- **Async-First**: Tokio throughout for I/O operations

For detailed architecture information, see [CLAUDE.md](CLAUDE.md).

## Common Development Tasks

### Adding a New Agent Capability

1. Add to `src/agents/capabilities.rs`:
   ```rust
   pub enum AgentCapability {
       // ... existing ...
       YourNewCapability,
   }
   ```

2. Add description and emoji in the same file

3. Add system prompt in `src/master/agent_factory.rs`

4. Update keyword detection in `src/master/planner.rs`

See [CLAUDE.md - Common Development Tasks](CLAUDE.md#adding-a-new-agent-capability) for details.

### Adding a New CLI Command

1. Add to CLI enum in `src/main.rs`:
   ```rust
   #[derive(Parser)]
   enum Cli {
       /// Your command description
       YourCommand {
           #[arg(short, long)]
           option: Option<String>,
       },
   }
   ```

2. Add handler in main match statement

See [CLAUDE.md - Adding a New CLI Command](CLAUDE.md#adding-a-new-cli-command) for details.

### Modifying Database Schema

1. Update `src/db/schema.rs` with new table/columns
2. Add to initialization in `src/db/mod.rs`
3. Add CRUD operations as needed
4. Consider migration path for existing databases

**Important:** Don't remove existing tables/columns without a deprecation period!

## Common Pitfalls to Avoid

### 1. Blocking in Async Context

```rust
// Bad ✗
async fn bad_example() {
    std::thread::sleep(Duration::from_secs(1)); // Blocks runtime!
}

// Good ✓
async fn good_example() {
    tokio::time::sleep(Duration::from_secs(1)).await;
}
```

### 2. Missing Error Context

```rust
// Bad ✗
let content = fs::read_to_string(path)?; // What file failed?

// Good ✓
let content = fs::read_to_string(&path)
    .context(format!("Failed to read {:?}", path))?;
```

### 3. Hardcoding Paths

```rust
// Bad ✗
let db = "/home/user/.config/claude-helper/db/claude-helper.db";

// Good ✓
let db = config::db_file()?; // Platform-specific paths
```

### 4. Ignoring Clippy Warnings

- Always address or explicitly allow warnings
- Use `#[allow(clippy::lint_name)]` with justification comment
- Run `cargo clippy -- -D warnings` to catch all issues

## Debugging Tips

### Enable Verbose Logging

```bash
# Debug level
RUST_LOG=debug cargo run -- status

# Trace level (very verbose)
RUST_LOG=trace cargo run -- status

# Module-specific logging
RUST_LOG=claude_helper::agents=debug cargo run
```

### Database Inspection

```bash
sqlite3 ~/.config/claude-helper/db/claude-helper.db

# Common queries
.tables
.schema task_executions
SELECT * FROM agent_executions ORDER BY created_at DESC LIMIT 10;
```

### Use Structured Logging

```rust
use tracing::{info, debug, warn, error};

debug!(agent_id = %agent.id(), "Executing agent task");
info!(tokens_used = result.tokens_used, "Agent completed");
```

## Getting Help

- **Documentation**: Read [README.md](README.md), [QUICKSTART.md](QUICKSTART.md), and [CLAUDE.md](CLAUDE.md)
- **Issues**: Search [existing issues](https://github.com/Metroseksuaali/Claude-helper/issues)
- **Discussions**: Ask questions in [GitHub Discussions](https://github.com/Metroseksuaali/Claude-helper/discussions)
- **Code Examples**: Browse the codebase for patterns and examples

## Recognition

Contributors will be recognized in:
- GitHub contributors page
- Release notes for significant contributions
- Special thanks in README for major features

## License

By contributing to Claude Helper, you agree that your contributions will be licensed under the same license as the project (see [LICENSE](LICENSE) file).

---

**Thank you for contributing to Claude Helper! Your efforts help make this project better for everyone.**
