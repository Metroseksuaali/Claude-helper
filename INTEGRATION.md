# Claude Code Integration Guide

This guide explains how Claude Helper integrates with Claude Code and how to customize the integration.

## Overview

Claude Helper enhances Claude Code through three integration points:

1. **Status Line**: Real-time token usage display (updates every 5s)
2. **Slash Commands**: `/master`, `/optimize`, `/token-usage`
3. **Session Hooks**: `sessionStart` and `afterResponse` for automatic tracking

## Installation

```bash
# Install the integration
claude-helper install-claude-integration
```

This creates:
```
~/.claude/
├── settings.json          # Claude Code configuration
└── commands/
    ├── master.md          # /master slash command
    ├── optimize.md        # /optimize slash command
    └── token-usage.md     # /token-usage slash command
```

## Configuration Files

### settings.json

Location: `~/.claude/settings.json`

```json
{
  "statusLine": "claude-helper statusline",
  "hooks": {
    "sessionStart": "claude-helper session-start",
    "afterResponse": "claude-helper log-usage"
  }
}
```

**Fields:**
- `statusLine`: Command executed to generate status line (every 5s)
- `hooks.sessionStart`: Command run when Claude Code session starts
- `hooks.afterResponse`: Command run after each Claude response

### Command Files

Commands are Markdown files with frontmatter and execution blocks.

#### Example: master.md

```markdown
---
description: Run Master Coder to orchestrate specialized agents for complex tasks
---

# Master Coder Agent Orchestration

This command analyzes your task and dynamically creates a team of specialized agents.

## Usage

```
/master "your task description here"
```

## Execution

```bash
claude-helper run --mode balanced "$@"
```
```

**Structure:**
- **Frontmatter** (`---`): Metadata like description
- **Content**: Help text shown to user
- **Execution block**: Command to run

## Status Line Details

### Format

```
[5h: 14k/20k 70%] [7d: 130k/200k 65%] $0.15/hr
```

**Components:**
- `5h`: Rolling 5-hour window (Claude Code rate limit)
- `7d`: Current week's usage
- `$0.15/hr`: Burn rate (cost per hour)

### How It Works

1. Claude Code calls `claude-helper statusline` every ~5 seconds
2. Claude Helper checks cache (5-second TTL)
3. If cache fresh: return cached data (~1ms)
4. If cache stale: fetch from API, cache, return (~8ms)
5. Output rendered at bottom of Claude Code window

### Performance

- **Cache hit**: ~1ms (most requests)
- **Cache miss**: ~8ms (every 5 seconds)
- **Zero impact** on Claude Code responsiveness

## Session Hooks

### sessionStart Hook

**Triggered**: When Claude Code session begins

**Command**: `claude-helper session-start`

**Actions**:
1. Log session start time to `~/.config/claude-helper/sessions.log`
2. Initialize session tracking in database
3. Prepare for optimization analysis

### afterResponse Hook

**Triggered**: After each Claude Code response

**Command**: `claude-helper log-usage`

**Actions**:
1. Get current token usage
2. Log to `~/.config/claude-helper/usage.log`
3. Analyze interaction for optimization opportunities
4. Update session statistics

## Slash Commands

### /master

**Purpose**: Run Master Coder orchestration

**Syntax**:
```
/master "task description"
/master --mode trust "task description"
/master --max-agents 3 --token-budget 20000 "task description"
```

**Parameters**:
- `--mode`: `conservative`, `balanced` (default), `trust`, `interactive`
- `--max-agents`: Maximum parallel agents (default: 5)
- `--token-budget`: Token limit for task (default: 50000)

**Behavior**:
1. Analyzes task complexity
2. Creates specialized agent team
3. Orchestrates parallel/sequential execution
4. Only asks for confirmation on major changes (balanced mode)
5. Shows real-time progress

**Agent Types Created**:
- **Architect**: High-level design
- **Code Writer (Alpha, Beta, ...)**: Specialized writers
- **Security Auditor**: Vulnerability scanning
- **Test Engineer**: Comprehensive testing
- **Documentation Writer**: Docs and comments
- **Performance Optimizer**: Profiling and optimization

### /optimize

**Purpose**: Analyze current session for optimization opportunities

**Syntax**:
```
/optimize
/optimize --session current
```

**Detects**:
- ⚡ **Quick Commands**: Sequential bash commands to combine
- 🔗 **File Merge**: Files frequently accessed together
- ✂️ **File Split**: Large files that could be split
- 🎯 **Context Pruning**: Redundant file reads
- 📦 **Tool Call Batching**: Inefficient tool usage

**Output Example**:
```
Found 3 optimization opportunities:

1. ⚡ Quick Command
   Combine git operations → Save ~600 tokens
   Suggestion: git add . && git commit -m "msg" && git push

2. 🔗 File Merge
   auth.ts + user.ts accessed together → Save ~400 tokens
   Suggestion: Consider merging into auth-user.ts
```

### /token-usage

**Purpose**: Show detailed token breakdown

**Syntax**:
```
/token-usage
```

**Shows**:
- Input/output tokens with costs
- Cache reads (free) and cache writes (cost)
- Total tokens and cost
- 5-hour and 7-day usage limits
- Burn rate and estimated time to limits
- Session duration and statistics

## Customization

### Custom Status Line Format

Edit `~/.claude/settings.json`:

```json
{
  "statusLine": "claude-helper statusline --format compact"
}
```

Future formats (planned):
- `compact`: Current format (default)
- `detailed`: More information
- `minimal`: Just percentage
- `custom`: User-defined template

### Custom Slash Commands

Create new command files in `~/.claude/commands/`:

**Example: security-audit.md**

```markdown
---
description: Run comprehensive security audit on codebase
---

# Security Audit

Runs security analysis including:
- Dependency vulnerability scanning
- Code pattern analysis
- Secret detection
- Permission checks

## Usage

```
/security-audit
/security-audit --severity high
```

## Execution

```bash
claude-helper run --mode balanced \
  "Run comprehensive security audit with:
   - Dependency vulnerability scan
   - Code pattern analysis
   - Secret detection
   - Permission verification
   Report findings with severity levels."
```
```

After creating the file:
```bash
# Restart Claude Code or reload configuration
claude  # The new /security-audit command is now available
```

### Disable Hooks

To disable session tracking:

```json
{
  "statusLine": "claude-helper statusline"
  // Remove or comment out hooks section
}
```

### Change Update Interval

The 5-second interval is hardcoded in Claude Code. To effectively change it:

1. Adjust cache TTL in `~/.config/claude-helper/config.toml`:

```toml
[statusline]
update_interval = 10  # Longer cache = fewer API calls
```

This reduces API calls but status may be up to 10s stale.

## Data Storage

### Database

Location: `~/.config/claude-helper/db/claude-helper.db`

**Tables**:
- `task_executions`: Master Coder task history
- `agent_executions`: Individual agent runs
- `sessions`: Session metadata
- `optimizations`: Detected optimization opportunities

**Purpose**:
- Learning from past executions
- Optimization pattern detection
- Performance statistics
- Token usage prediction

### Logs

**Session log**: `~/.config/claude-helper/sessions.log`
- Session start/end times
- Session IDs

**Usage log**: `~/.config/claude-helper/usage.log`
- Token usage per interaction
- Timestamps

**Purpose**:
- Real-time session analysis
- Optimization opportunity detection
- Historical trend analysis

## Troubleshooting

### Status Line Not Appearing

1. **Check installation**:
   ```bash
   cat ~/.claude/settings.json
   # Should contain "statusLine" field
   ```

2. **Test command manually**:
   ```bash
   claude-helper statusline
   # Should output: [5h: ...] [7d: ...] $...
   ```

3. **Check permissions**:
   ```bash
   ls -la ~/.claude/
   # All files should be readable
   ```

4. **Restart Claude Code**:
   ```bash
   # Kill any running Claude processes
   pkill -f claude
   # Start fresh
   claude
   ```

### Slash Commands Not Working

1. **Verify command files exist**:
   ```bash
   ls -la ~/.claude/commands/
   # Should show: master.md, optimize.md, token-usage.md
   ```

2. **Check command format**:
   ```bash
   cat ~/.claude/commands/master.md
   # Verify frontmatter and execution block
   ```

3. **Test command manually**:
   ```bash
   claude-helper run "test task"
   # Should execute without errors
   ```

### Hooks Not Running

1. **Check hooks configuration**:
   ```bash
   cat ~/.claude/settings.json | grep -A 4 hooks
   # Should show sessionStart and afterResponse
   ```

2. **Test hooks manually**:
   ```bash
   claude-helper session-start
   claude-helper log-usage
   # Should complete without errors
   ```

3. **Check log files**:
   ```bash
   tail ~/.config/claude-helper/sessions.log
   tail ~/.config/claude-helper/usage.log
   # Should show recent entries
   ```

### High API Usage

If you see high API usage:

1. **Increase cache TTL**:
   Edit `~/.config/claude-helper/config.toml`:
   ```toml
   [statusline]
   update_interval = 10  # Cache for 10 seconds instead of 5
   ```

2. **Disable unnecessary hooks**:
   Remove `afterResponse` hook if you don't need per-interaction tracking

3. **Use standalone mode**:
   Remove status line, use manual checks:
   ```bash
   claude-helper status
   ```

## Performance

### Benchmarks

- **Status line (cache hit)**: ~1ms
- **Status line (cache miss)**: ~8ms
- **sessionStart hook**: ~15ms
- **afterResponse hook**: ~20ms
- **Database operations**: ~5ms

### Impact on Claude Code

- **Negligible**: Status line cached, hooks async
- **Memory**: ~5-10MB for claude-helper process
- **CPU**: <1% during status updates
- **Disk**: ~1MB for database and logs

## Best Practices

### 1. Regular Optimization Review

```bash
# Weekly: Check for optimization opportunities
/optimize

# Or from CLI:
claude-helper optimize --last 50
```

### 2. Monitor Token Usage

Watch burn rate and adjust usage:
- High burn rate? Use `/optimize` to find savings
- Near limits? Check `/token-usage` for breakdown

### 3. Use Appropriate Autonomy Modes

- **balanced**: Default, good for most tasks
- **trust**: For repetitive/well-understood tasks
- **conservative**: For critical operations
- **interactive**: When learning Master Coder

### 4. Set Token Budgets

Prevent runaway costs:
```
/master --token-budget 20000 "task description"
```

### 5. Review Agent History

```bash
# See what agents worked well
claude-helper agents stats
claude-helper agents history --last 20
```

## Advanced Integration

### Custom Agent Templates

Create agent templates in `~/.config/claude-helper/agents/`:

```yaml
# performance-optimizer.yaml
name: "Performance Optimizer"
capability: "Optimization"
system_prompt: |
  You are a performance optimization specialist.
  Focus on:
  - Identifying bottlenecks
  - Profiling critical paths
  - Suggesting algorithmic improvements
  - Memory optimization
  - Cache efficiency

  Always provide benchmarks before and after.
```

Use in Master Coder:
```
/master --agents-config performance-optimizer.yaml "optimize API endpoints"
```

### Integration with Other Tools

**VSCode**: Add to tasks.json:
```json
{
  "label": "Check Token Usage",
  "type": "shell",
  "command": "claude-helper status"
}
```

**tmux**: Add to status bar:
```bash
set -g status-right '#(claude-helper statusline)'
```

**Shell prompt**: Add to .bashrc:
```bash
PS1='$(claude-helper statusline) $ '
```

## Security Considerations

### API Keys

- Never commit API keys to version control
- Store in config file with proper permissions:
  ```bash
  chmod 600 ~/.config/claude-helper/config.toml
  ```

### Database

- Database contains task descriptions and code snippets
- Ensure proper permissions:
  ```bash
  chmod 600 ~/.config/claude-helper/db/claude-helper.db
  ```

### Logs

- Logs may contain sensitive information
- Rotate regularly:
  ```bash
  # Add to crontab
  0 0 * * 0 rm ~/.config/claude-helper/*.log
  ```

## Support

- **Documentation**: [README.md](README.md)
- **Quick Start**: [QUICKSTART.md](QUICKSTART.md)
- **Issues**: [GitHub Issues](https://github.com/Metroseksuaali/Claude-helper/issues)
- **Discussions**: [GitHub Discussions](https://github.com/Metroseksuaali/Claude-helper/discussions)

## License

MIT License - See [LICENSE](LICENSE) for details
