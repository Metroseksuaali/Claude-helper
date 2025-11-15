---
description: Analyze current session and suggest token optimization opportunities
---

# Session Optimization Analysis

Analyzes your current Claude Code session in real-time and suggests ways to reduce token usage.

## Usage

```
/optimize
```

## What It Detects

### 1. Command Patterns
- Sequential bash commands that could be combined
- Repetitive workflows that could be scripted

### 2. File Access Patterns
- Files frequently accessed together (merge candidates)
- Large files that could be split
- Redundant file reads

### 3. Tool Usage
- Excessive Grep searches
- Duplicate Read operations
- Inefficient search patterns

### 4. Context Management
- Unnecessary context in prompts
- Repeated information
- Overly broad file reads

## Output

For each optimization found:
- **Type**: What kind of optimization
- **Description**: What was detected
- **Estimated Savings**: How many tokens you'd save
- **Suggestion**: Specific action to take

## Example Output

```
Found 3 optimization opportunities:

1. ⚡ Quick Command
   Combine git operations → Save ~600 tokens
   Suggestion: git add . && git commit -m "msg" && git push

2. 🔗 File Merge
   auth.ts + user.ts frequently accessed together → Save ~400 tokens
   Suggestion: Consider merging into auth-user.ts

3. 🎯 Context Pruning
   Redundant file reads detected → Save ~900 tokens
   Suggestion: Use more specific Grep patterns
```

## Execution

```bash
claude-helper optimize --session current
```
