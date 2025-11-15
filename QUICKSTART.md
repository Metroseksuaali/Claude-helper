# Claude Helper - Quick Start Guide

Get Claude Code supercharged in 3 minutes!

## Installation & Setup

### Step 1: Install Claude Helper

```bash
# Prerequisites: Rust 1.70+
git clone https://github.com/Metroseksuaali/Claude-helper.git
cd Claude-helper
cargo build --release
sudo cp target/release/claude-helper /usr/local/bin/
```

### Step 2: Install Claude Code Integration

```bash
# This sets up status line, hooks, and slash commands
claude-helper install-claude-integration
```

You'll see:
```
📦 Installing Claude Code integration...

✓ Created directory structure
✓ Installed settings.json
✓ Installed /master
✓ Installed /optimize
✓ Installed /token-usage

✨ Claude Code integration installed successfully!

Next time you run 'claude', you'll have:
  • Status line showing token usage (updates every 5s)
  • /master - Run Master Coder orchestration
  • /optimize - Get session optimization suggestions
  • /token-usage - View detailed token breakdown

Configuration: ~/.claude/settings.json
Commands: ~/.claude/commands/
```

### Step 3: Start Claude Code

```bash
claude
```

You'll immediately see:
- **Status line at bottom**: `[5h: 14k/20k 70%] [7d: 130k/200k 65%] $0.15/hr`
- **Three new slash commands**: `/master`, `/optimize`, `/token-usage`
- **Automatic tracking**: Every interaction logged for optimization

That's it! You're ready to go.

## Authentication

Claude Helper works with:

### Claude Code Pro/Max (Recommended)
If you have Claude Code installed, authentication is automatic!

### Claude API Key
If you have an Anthropic API key:

```bash
claude-helper config set-api-key
# Follow the prompts
```

## Using Claude Helper in Claude Code

### Status Line (Automatic)

After installation, the status line appears automatically at the bottom:

```
[5h: 14k/20k 70%] [7d: 130k/200k 65%] $0.15/hr
```

It updates every 5 seconds showing:
- **5h**: Current rolling 5-hour window (Claude Code's rate limit)
- **7d**: Current week's total usage
- **Burn rate**: Cost per hour based on usage patterns

### Slash Command: /master

Run Master Coder orchestration directly in Claude Code:

```
/master "Implement OAuth2 authentication with Google and GitHub providers"
```

The Master Coder will:
1. ✅ Analyze task complexity
2. ✅ Create specialized agent team (Code Writers, Security Auditor, Test Engineer, etc.)
3. ✅ Execute with parallel/sequential orchestration
4. ✅ Only ask for confirmation on major changes (not every step!)
5. ✅ Show real-time progress

**Autonomy modes** (use with `/master --mode <mode>`):
- `conservative` - Frequent confirmations
- `balanced` - Smart gates at important points (default)
- `trust` - Fully automatic execution
- `interactive` - Full control every step

Example:
```
/master --mode trust "Refactor database layer with comprehensive tests"
```

### Slash Command: /optimize

Get instant optimization suggestions for your current session:

```
/optimize
```

Output example:
```
Found 3 optimization opportunities:

1. ⚡ Quick Command
   Combine git operations → Save ~600 tokens
   Suggestion: git add . && git commit -m "msg" && git push

2. 🔗 File Merge
   auth.ts + user.ts accessed together → Save ~400 tokens
   Suggestion: Consider merging into auth-user.ts

3. 🎯 Context Pruning
   Redundant file reads → Save ~900 tokens
   Suggestion: Use more specific Grep patterns
```

The analyzer detects:
- Command batching opportunities
- Files frequently accessed together
- Redundant tool calls
- Context optimization

### Slash Command: /token-usage

View detailed token breakdown:

```
/token-usage
```

Shows:
- Input/output tokens with costs
- Cache reads (free) and cache writes (cost)
- Session duration and message count
- Average tokens per message
- Burn rate and time until limits

## Standalone CLI Usage

You can also use claude-helper outside Claude Code:

### Check Token Usage

```bash
# Quick status check
claude-helper status

# Watch in real-time
claude-helper watch
```

### Run Master Coder from CLI

```bash
claude-helper run "Implement OAuth2 authentication"

# With options
claude-helper run \
  --mode balanced \
  --max-agents 5 \
  --token-budget 30000 \
  "Add login with tests and docs"
```

### Analyze Past Sessions

```bash
# Analyze recent sessions
claude-helper analyze --last 10

# Get optimization suggestions
claude-helper optimize --last 5
```

### Agent Management

```bash
# List available agent types
claude-helper agents list

# View statistics
claude-helper agents stats

# Show execution history
claude-helper agents history --last 20
```

### Interactive TUI

```bash
claude-helper tui
```

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
