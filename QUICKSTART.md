# Claude Helper - Quick Start Guide

## Installation

### Option 1: Quick Install (Recommended)
```bash
curl -fsSL https://raw.githubusercontent.com/Metroseksuaali/Claude-helper/main/install.sh | sh
```

### Option 2: Build from Source
```bash
# Prerequisites: Rust 1.70+
git clone https://github.com/Metroseksuaali/Claude-helper.git
cd Claude-helper
cargo build --release
sudo cp target/release/claude-helper /usr/local/bin/
```

## First Time Setup

### 1. Configure Authentication

Choose your authentication method:

#### Option A: Claude Code Pro/Max (Recommended)
If you have Claude Code installed, authentication is automatic!

```bash
# Verify it's configured
claude-helper config show
```

#### Option B: API Key
If you have an Anthropic API key:

```bash
claude-helper config set-api-key
# Follow the prompts to enter your API key
```

### 2. Verify Installation

```bash
# Check version
claude-helper --version
# Should output: claude-helper 0.1.0

# Check available commands
claude-helper --help

# Test database creation
claude-helper agents list
```

## Basic Usage

### Check Token Usage

```bash
# Detailed view
claude-helper status

# Compact format (for status bars)
claude-helper statusline
```

Output:
```
Current 5-Hour Block:
  [████████████████████████████░░░░░░░░░░░░]
  Used: 14000 / 20000 tokens (70%)
  Time remaining: 120 minutes

7-Day Total:
  [██████████████████████████░░░░░░░░░░░░░░]
  Used: 130000 / 200000 tokens (65%)

Cost Information:
  Burn rate: $0.15/hour
  Estimated 7-day cost: $1.17
```

### Master Coder Orchestration

Run complex tasks with intelligent agent coordination:

```bash
# Basic usage
claude-helper run "Implement OAuth2 authentication"

# With options
claude-helper run \
  --mode balanced \
  --max-agents 5 \
  --token-budget 30000 \
  "Add login feature with tests and documentation"
```

**Autonomy Modes:**
- `conservative` - Asks for approval frequently
- `balanced` - Approval gates at important points (default)
- `trust` - Fully automatic
- `interactive` - Control every step

### Analyze and Optimize Sessions

```bash
# Analyze recent sessions
claude-helper analyze --last 10

# Get optimization suggestions
claude-helper optimize --last 5

# Analyze specific session
claude-helper optimize --session <session-id>
```

### Agent Management

```bash
# List available agent types
claude-helper agents list

# View agent statistics
claude-helper agents stats

# See agent execution history
claude-helper agents history --last 20
```

### Interactive TUI

```bash
# Open interactive dashboard
claude-helper tui

# Use arrow keys to navigate tabs:
# - Usage: Token tracking
# - Optimizations: Suggestions
# - Agent History: Recent executions

# Press 'q' to quit
```

## Integration with Claude Code

Install status line integration:

```bash
claude-helper install-statusline
```

This updates your `~/.claude/settings.json` to show real-time token usage in Claude Code's status bar.

## Configuration

### View Current Config
```bash
claude-helper config show
```

### Edit Configuration
```bash
claude-helper config edit
# Opens in $EDITOR (or nano)
```

### Configuration File Location
`~/.config/claude-helper/config.toml`

### Key Settings

```toml
[master_coder]
default_mode = "balanced"          # conservative, balanced, trust, interactive
max_parallel_agents = 5            # Maximum concurrent agents
token_budget = 50000               # Default token limit per task
enable_learning = true             # Learn from past sessions

[statusline]
update_interval = 30               # Seconds between updates
show_costs = true                  # Display cost estimates
api_endpoint = "https://claude.ai/api"

[analyzer]
history_depth = 50                 # Sessions to analyze
min_savings_threshold = 500        # Minimum tokens to suggest optimization
```

## Example Workflows

### Workflow 1: Feature Development

```bash
# Run Master Coder with balanced autonomy
claude-helper run --mode balanced \
  "Implement user authentication with OAuth2, including:
   - Google and GitHub providers
   - JWT token management
   - Session handling
   - Comprehensive tests
   - Security audit
   - API documentation"

# Master Coder will:
# 1. Create Architect agent for design
# 2. Spawn 3 Code Writer agents (providers, middleware, UI)
# 3. Run Security Auditor and Test Engineer in parallel
# 4. Generate documentation
```

### Workflow 2: Optimization Session

```bash
# Check current usage
claude-helper status

# Analyze recent sessions
claude-helper analyze --last 20

# Get specific optimizations
claude-helper optimize --last 10

# Apply suggestions manually or use interactive TUI
claude-helper tui
```

### Workflow 3: Daily Monitoring

```bash
# Watch token usage in real-time
claude-helper watch

# Or add to status bar
echo 'export PS1="$(claude-helper statusline) $PS1"' >> ~/.bashrc
```

## Troubleshooting

### Database Issues

```bash
# Reset database (careful: deletes history!)
rm ~/.config/claude-helper/db/claude-helper.db
claude-helper agents list  # Recreates DB
```

### Configuration Issues

```bash
# Reset to defaults
claude-helper config reset

# Verify settings
claude-helper config show
```

### API Connection Issues

```bash
# For Claude Code mode:
# 1. Ensure Claude Code is installed
# 2. Check you're logged in
# 3. Verify session exists: ls ~/.claude/sessions/

# For API Key mode:
# 1. Verify key is set: claude-helper config show
# 2. Test with simple command: claude-helper status
```

### Verbose Logging

```bash
# Enable verbose output for debugging
claude-helper --verbose <command>

# Example:
claude-helper --verbose run "test task"
```

## Performance Tips

1. **Use appropriate autonomy mode**
   - Start with `balanced` for safety
   - Use `trust` for repetitive tasks
   - Use `interactive` for critical operations

2. **Set token budgets**
   - Prevents runaway costs
   - Example: `--token-budget 10000`

3. **Optimize agent count**
   - More agents = faster but more complex
   - Start with default (5), adjust as needed

4. **Monitor regularly**
   - Use `watch` command
   - Check `status` before big tasks
   - Review `optimize` suggestions weekly

## Advanced Usage

### Custom Agent Templates

Create custom agent configurations in `~/.config/claude-helper/agents/`:

```yaml
# custom-reviewer.yaml
name: "Custom Code Reviewer"
capability: "Review"
system_prompt: |
  You are a specialized code reviewer focusing on:
  - Performance optimization
  - Security best practices
  - Code maintainability

  Provide actionable feedback with specific examples.
```

### Batch Processing

```bash
# Process multiple tasks
for task in "feature1" "feature2" "feature3"; do
  claude-helper run --mode trust "$task"
done
```

### Integration with CI/CD

```bash
# In your CI pipeline
claude-helper run --mode trust --token-budget 20000 \
  "Review PR and suggest improvements"
```

## Getting Help

```bash
# General help
claude-helper --help

# Command-specific help
claude-helper run --help
claude-helper config --help
claude-helper agents --help

# View documentation
cat ~/.config/claude-helper/README.md
```

## What's Next?

1. ⭐ **Star the repo**: https://github.com/Metroseksuaali/Claude-helper
2. 📖 **Read full docs**: See README.md
3. 🐛 **Report bugs**: GitHub Issues
4. 💡 **Request features**: GitHub Discussions
5. 🤝 **Contribute**: See CONTRIBUTING.md

## Tips for Success

- ✅ Start with small tasks to learn the system
- ✅ Use `--mode balanced` until comfortable
- ✅ Monitor token usage regularly
- ✅ Review optimization suggestions
- ✅ Keep token budgets reasonable
- ✅ Use the TUI for interactive exploration

Happy coding! 🚀
