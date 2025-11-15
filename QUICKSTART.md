# Claude Helper - Quick Start Guide

Get Claude Code supercharged in 5 minutes!

**Status**: Alpha v0.1.0 - Core features working, some features still in development

## What Works Now

✅ **Ready to use:**
- Session analysis and optimization detection
- `/optimize` and `/token-usage` slash commands
- Database storage and CLI tools
- Session hooks for automatic tracking

🚧 **In development:**
- Master Coder orchestration (structure complete, needs testing)
- Real-time token tracking (API endpoint verification needed)
- TUI dashboard (shows mock data currently)

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
# This sets up slash commands, hooks, and status line framework
claude-helper install-claude-integration
```

You'll see:
```
📦 Installing Claude Code integration...

✓ Created directory structure
✓ Updated settings.json (existing settings preserved)
✓ Installed /master
✓ Installed /optimize
✓ Installed /token-usage

✨ Claude Code integration installed successfully!

Next time you run 'claude', you'll have:
  • Status line showing token usage (🚧 in development)
  • /optimize - Session optimization suggestions (✅ working)
  • /token-usage - Detailed token breakdown (✅ working)
  • /master - Master Coder orchestration (🚧 in development)

Configuration: ~/.claude/settings.json
Commands: ~/.claude/commands/
```

**Note**: If you had existing settings, they were preserved and backed up to `settings.json.backup`.

### Step 3: Start Claude Code

```bash
claude
```

You'll now have access to:
- **Two working slash commands**: `/optimize`, `/token-usage`
- **Automatic tracking**: Every interaction logged for optimization analysis
- **Session hooks**: Runs after each response to detect optimization opportunities

**Note**: Status line and `/master` command are installed but still in development (API endpoint verification needed).

That's it! You're ready to analyze your sessions and get optimization suggestions.

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

### Status Line 🚧 (Framework Installed, Not Yet Functional)

**Status**: Configuration installed, real-time API tracking coming in v0.2.0.

When fully functional (v0.2.0), the status line will appear at the bottom:

```
[5h: 14k/20k 70%] [7d: 130k/200k 65%] $0.15/hr
```

Planned updates every 5 seconds showing:
- **5h**: Current rolling 5-hour window (Claude Code's rate limit)
- **7d**: Current week's total usage
- **Burn rate**: Cost per hour based on usage patterns

**Current workaround**: Use `/token-usage` slash command for session token analysis.

### Slash Command: /optimize ✅

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

### Slash Command: /token-usage ✅

View detailed token breakdown from your current Claude Code session:

```
/token-usage
```

Shows:
- Input/output tokens with costs
- Cache reads (free) and cache writes (cost)
- Session duration and message count
- Average tokens per message

**Note**: Uses Claude Code session data. Real-time API tracking coming in v0.2.0.

### Slash Command: /master 🚧

**Status**: In development. Command structure exists but orchestration needs testing.

```
/master "Your task here"
```

**Planned features** (v0.2.0):
- Task complexity analysis
- Dynamic agent team creation
- Parallel/sequential execution
- Multiple autonomy modes

**Current recommendation**: Use standard Claude Code for complex tasks until v0.2.0.

## Standalone CLI Usage

You can also use claude-helper outside Claude Code:

### Analyze Past Sessions ✅

```bash
# Analyze recent sessions
claude-helper analyze --last 10

# Get optimization suggestions
claude-helper optimize --last 5
```

### Agent Management ✅

```bash
# List available agent types
claude-helper agents list

# View statistics (from database)
claude-helper agents stats

# Show execution history
claude-helper agents history --last 20
```

### Check Token Usage 🚧

```bash
# Quick status check (API endpoint needs verification)
claude-helper status

# Watch in real-time (🚧 in development)
claude-helper watch
```

### Interactive TUI 🚧

```bash
# TUI with mock data (wiring to database in progress)
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

### Workflow 1: Session Optimization (✅ Working)

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

### Workflow 2: Database Exploration (✅ Working)

```bash
# View what agents have been created
claude-helper agents list

# Check database stats
claude-helper agents stats

# See execution history
claude-helper agents history

# Launch interactive TUI (shows mock data currently)
claude-helper tui
```

### Workflow 3: Configuration Management (✅ Working)

```bash
# View current settings
claude-helper config show

# Edit configuration
claude-helper config edit

# Reset to defaults if needed
claude-helper config reset
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

## Performance Tips (For Current Alpha Version)

1. **Use session analysis regularly**
   - Run `claude-helper analyze` weekly
   - Review optimization suggestions
   - Apply bash chain and file merge recommendations

2. **Monitor database growth**
   - Check `~/.config/claude-helper/db/` size periodically
   - Older sessions can be cleaned if needed

3. **Configure appropriately**
   - Set `min_savings_threshold` to filter minor optimizations
   - Adjust `history_depth` based on your usage patterns

4. **Leverage slash commands in Claude Code**
   - Use `/optimize` during long sessions
   - Check `/token-usage` to understand patterns

## Advanced Usage (Coming in v0.2.0+)

The following features are planned but not yet implemented:

### Custom Agent Templates 🚧

Custom YAML-based agent configurations planned for v0.3.0.

### Batch Processing 🚧

Batch task processing planned once Master Coder orchestration is stable (v0.2.0).

### Integration with CI/CD 🚧

CI/CD integration planned for v1.0.0.

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

## Tips for Success (Alpha Version)

- ✅ Focus on session analysis - it works great!
- ✅ Use `/optimize` regularly during Claude Code sessions
- ✅ Review and apply optimization suggestions
- ✅ Check database stats with `claude-helper agents stats`
- ✅ Report bugs on GitHub - this is alpha software
- ✅ Star the repo and watch for v0.2.0 updates

### What to Expect in v0.2.0

- ✅ Verified real-time token tracking
- ✅ Fully tested Master Coder orchestration
- ✅ TUI wired to real database
- ✅ Bug fixes from TODO.md
- ✅ Comprehensive test coverage

Happy coding! 🚀

**Remember**: This is alpha software. Session analysis works well, but some features are still being polished. Check the README for current status.
