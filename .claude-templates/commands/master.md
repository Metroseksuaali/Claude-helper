---
description: Run Master Coder to orchestrate specialized agents for complex tasks
---

# Master Coder Agent Orchestration

This command analyzes your task and dynamically creates a team of specialized agents to complete it efficiently.

## Usage

```
/master "your task description here"
```

## What It Does

1. **Task Analysis**: Analyzes complexity and requirements
2. **Team Creation**: Dynamically creates specialized agents (Code Writers, Security Auditor, Test Engineer, etc.)
3. **Orchestration**: Manages parallel and sequential execution
4. **Quality Control**: Ensures consistency and correctness
5. **Progress Reporting**: Shows real-time progress

## Agents Created Based on Task

- **Architect**: For design and planning
- **Code Writer (Alpha, Beta, Gamma...)**: Multiple writers for complex tasks
- **Security Auditor**: Reviews for vulnerabilities
- **Test Engineer**: Writes comprehensive tests
- **Documentation Writer**: Creates docs
- **Performance Optimizer**: Optimizes code
- **Migration Specialist**: Handles migrations

## Examples

```
/master "Implement OAuth2 authentication with Google and GitHub providers"
/master "Refactor database layer with comprehensive tests and documentation"
/master "Add GraphQL API with security audit"
```

## Execution

```bash
claude-helper run --mode balanced "$@"
```
