# Claude Helper - TODO List

**Generated**: 2025-11-14
**Status**: Code review findings - Issues to address

This document contains findings from a comprehensive code review. Items are organized by priority and category.

---

## 🔴 Critical Issues (Fix Soon)

### 1. Division by Zero Risk
**File**: `src/statusline/usage_tracker.rs:99-100`
**Issue**: Percentage calculations will crash if `limit` is 0
```rust
let five_hour_percent = ((response.usage.five_hour.used as f64 / response.usage.five_hour.limit as f64) * 100.0) as u8;
let seven_day_percent = ((response.usage.seven_day.used as f64 / response.usage.seven_day.limit as f64) * 100.0) as u8;
```
**Action**: Add validation to check if limit == 0 before division

### 2. Incorrect Dependency Logic
**File**: `src/master/planner.rs:246`
**Issue**: Code writers depend on `specs[0].id` which might not be an architect
```rust
dependencies: if specs.is_empty() { vec![] } else { vec![specs[0].id.clone()] },
```
**Action**: Code writers should explicitly depend on architect agents, not just first spec

### 3. Hardcoded AgentCapability
**File**: `src/db/mod.rs:153`
**Issue**: `get_agent_history()` always returns `CodeWriting` capability instead of parsing from database
```rust
capability: AgentCapability::CodeWriting, // Would parse from DB
```
**Action**: Parse capability string from database and convert to enum

### 4. Empty Capability Breakdown
**File**: `src/db/mod.rs:116`
**Issue**: `by_capability` HashMap initialized empty but never populated
```rust
let by_capability = std::collections::HashMap::new();
// This would query the actual capability distribution
// For now, using placeholder
```
**Action**: Implement actual capability distribution query

### 5. Percentage Overflow Risk
**File**: `src/statusline/usage_tracker.rs:100-101`
**Issue**: Using `u8` for percentages could overflow (max 255) if calculation exceeds 100
```rust
let five_hour_percent = (...) as u8;
```
**Action**: Clamp percentage to 0-100 range or use u16

---

## 🟡 Missing Implementations

### 6. Missing Capabilities in Planner
**File**: `src/master/planner.rs:200-299`
**Issue**: `plan_agents()` only handles 6 of 9 capabilities
- ✅ Architecture, CodeWriting, Security, Testing, Documentation, Migration
- ❌ Missing: Debugging, Performance, Review

**Action**: Add agent specs for Debugging, Performance, and Review capabilities

### 7. Review Capability Missing from Detection
**File**: `src/master/planner.rs:126-135`
**Issue**: Review capability exists in enum but has no keyword detection
**Action**: Add keywords for Review capability (e.g., "review", "audit", "assess", "check quality")

### 8. Input Validation Missing
**File**: `src/master/planner.rs:77-81`
**Issue**: Multiple TODOs for input validation that doesn't exist:
- Empty strings
- Extremely long inputs (>10,000 chars)
- Unicode handling
- Null bytes

**Action**: Implement input validation in `estimate_complexity()`

### 9. TUI Tabs Show Mock Data
**File**: `src/tui/app.rs:109-163`
**Issue**: All three tabs (`render_usage_tab`, `render_optimization_tab`, `render_agent_history_tab`) show hardcoded mock data instead of fetching real data
**Action**:
- Fetch usage data from `self.statusline`
- Fetch optimizations from `self.analyzer`
- Fetch agent history from `self.db`

### 10. Config Validation Missing
**File**: `src/config/mod.rs:18-24`
**Issue**: TODOs for config validation:
- `default_mode` must be one of: conservative, balanced, trust, interactive
- `max_parallel_agents` must be >= 1 and <= 100
- `token_budget` must be >= 1000 and reasonable (<= 1,000,000)

**Action**: Add validation methods and call in `Config::load()`

---

## 🟠 Incomplete/Simplified Code

### 11. Simplified Auth Implementation
**File**: `src/config/auth.rs:79-82`
**Issue**: Comments indicate this is simplified and missing production features:
- Parse session files properly
- Handle session refresh
- Validate session is still active

**Action**: Implement production-ready session handling

### 12. Uncertain API Endpoint
**File**: `src/statusline/usage_tracker.rs:70-71`
**Issue**: Comment says "This endpoint might not be the correct one"
```rust
// Note: This endpoint might not be the correct one
// You'd need to find the actual Claude usage API endpoint
```
**Action**: Verify correct Claude usage API endpoint

### 13. Inaccurate Cost Calculation
**File**: `src/statusline/usage_tracker.rs:109-111`
**Issue**: Uses 50/50 input/output assumption instead of actual token split
```rust
// TODO: Use actual input/output token split for accurate cost calculation
let avg_cost_per_million = 9.0; // Average of input and output
```
**Action**: Track input/output tokens separately and calculate accurate costs

### 14. Hardcoded Time Remaining
**File**: `src/statusline/usage_tracker.rs:119`
**Issue**: Minutes remaining hardcoded instead of calculating from `reset_at`
```rust
five_hour_minutes_remaining: 60, // Would calculate from reset_at
```
**Action**: Parse `reset_at` timestamp and calculate actual time remaining

### 15. Simplistic Keyword Extraction
**File**: `src/master/planner.rs:156-162`
**Issue**: Very simplistic extraction (first 10 words >3 chars) might miss context or include noise
```rust
fn extract_keywords(&self, task: &str) -> Vec<String> {
    task.split_whitespace()
        .filter(|word| word.len() > 3)
        .take(10)
        .map(|s| s.to_string())
        .collect()
}
```
**Action**: Implement better keyword extraction (stop words, stemming, TF-IDF)

---

## 📋 Documentation vs Implementation

### 16. Complexity Calculation Discrepancy
**File**: `CLAUDE.md` vs `src/master/planner.rs:98-101`
**Issue**: Documentation says +2 per keyword TYPE, but code adds +2 per MATCH (accumulates)
```rust
for keyword in &high_complexity_keywords {
    if task.contains(keyword) {
        complexity += 2; // Adds for EACH match
    }
}
```
**Action**: Either fix code or update documentation to match actual behavior

### 17. Case-Sensitive Keyword Matching
**File**: `src/master/planner.rs:97-102`
**Issue**: TODO says convert to lowercase, but matching is case-sensitive
```rust
// TODO: Convert to lowercase for case-insensitive matching
for keyword in &high_complexity_keywords {
    if task.contains(keyword) { // Case-sensitive
```
**Action**: Convert task to lowercase before matching

### 18. Stale Line References in Docs
**File**: `CLAUDE.md` (throughout)
**Issue**: Many line number references (e.g., "src/master/planner.rs:28-61") will become stale as code changes
**Action**: Consider automated doc generation or remove specific line numbers

---

## 🔧 Code Smells / Suspicious Patterns

### 19. Unused Config Parameter
**File**: `src/db/mod.rs:25`
**Issue**: `Database::new()` accepts `Config` but doesn't use it (prefixed with `_config`)
```rust
pub async fn new(_config: &Config) -> Result<Self> {
```
**Action**: Either use config or remove parameter

### 20. Nearly Empty Module
**File**: `src/tui/widgets.rs`
**Issue**: File has only 15 lines with a placeholder function
```rust
pub fn create_block(title: &str) -> Block {
    Block::default()
        .borders(Borders::ALL)
        .title(title)
        .style(Style::default().fg(Color::White))
}
```
**Action**: Either implement custom widgets or remove this module

### 21. Unused Dev-Dependency
**File**: `Cargo.toml:61`
**Issue**: `mockito = "1.5"` included but no HTTP mock tests exist
**Action**: Write HTTP mock tests or remove dependency

### 22. Silent Error Handling
**Files**: Multiple (db/mod.rs:146-148, 220-222, 248-250)
**Issue**: DateTime parsing failures silently default to `Utc::now()`, hiding corrupt data
```rust
let timestamp = DateTime::parse_from_rfc3339(&row.6)
    .unwrap_or_else(|_| Utc::now().into())
    .with_timezone(&Utc);
```
**Action**: Log warnings when parsing fails for debugging purposes

### 23. Progress Bar Unwrap
**File**: `src/master/orchestrator.rs:150-156`
**Issue**: Template parsing uses `unwrap()` which will panic if invalid
```rust
ProgressStyle::default_bar()
    .template("  {spinner:.cyan} [{bar:40.cyan/blue}] {msg}")
    .unwrap() // Will panic if template is invalid
```
**Action**: Use `expect()` with descriptive message or handle error gracefully

---

## 🧪 Test Coverage

### 24. Low Overall Test Coverage
**Status**:
- ✅ Unit tests in `planner.rs` (24 tests for complexity and dependency resolution)
- ✅ One integration test in `database_tests.rs`
- ❌ Most modules untested

**Action**: Expand test coverage for:
- `statusline/usage_tracker.rs`
- `analyzer/optimizer.rs`
- `agents/claude_agent.rs`
- `config/auth.rs`
- `master/orchestrator.rs`

### 25. Large Test TODO Blocks
**File**: `src/master/planner.rs:391-401`
**Issue**: Comprehensive test scenarios listed but not implemented:
- Empty agents vector
- Linear chain (A → B → C)
- Diamond dependency (A → B,C → D)
- Fully parallel agents
- Circular dependency
- Self-dependency
- Missing dependency
- Large graph (100+ agents)

**Action**: Implement these test scenarios

---

## ⚠️ Minor / Needs Verification

### 26. Session Parser Assumptions
**File**: `src/analyzer/session_parser.rs:105-113`
**Issue**: Only extracts messages if BOTH role AND content exist as strings. Tool responses might not have content field
**Action**: Verify against actual Claude Code JSONL format

### 27. Tool Call Format Assumptions
**File**: `src/analyzer/session_parser.rs:116-122`
**Issue**: Assumes `tool_use.name` and `tool_use.input` structure
**Action**: Verify this matches actual Claude Code JSONL schema

### 28. DateTime Format Compatibility
**File**: `src/db/schema.rs:12, 24, 34`
**Issue**: SQLite `DATETIME DEFAULT CURRENT_TIMESTAMP` format may not match Rust chrono parsing
**Action**: Test datetime roundtrip (save → read → parse)

### 29. Fragile Test/Build Detection
**File**: `src/analyzer/optimizer.rs:94-95`
**Issue**: Uses simple string contains which could have false positives
```rust
let has_test = bash_calls.iter().any(|cmd| cmd.contains("test")); // "latest" contains "test"
```
**Action**: Use more precise pattern matching or regex

### 30. Status vs Statusline Commands
**File**: `src/main.rs:47-61`
**Issue**: Status has `detailed` flag, Statusline doesn't
**Status**: ✅ This is intentional (Status = full CLI, Statusline = one-line integration)

---

## 📚 Implementation Completeness

### 31. Optimizer.rs Incomplete Read
**File**: `src/analyzer/optimizer.rs`
**Issue**: Only first 100 lines reviewed during code analysis
**Action**: Review remainder of implementation (detect_file_patterns, detect_large_files, detect_tool_repetition)

### 32. Agent Module Exports
**File**: `src/agents/mod.rs`
**Issue**: Need to verify all public exports are complete
**Action**: Check that Agent, AgentCapability, ClaudeAgent, AgentManager are all properly exported

### 33. Circular Dependency Algorithm
**File**: `src/master/planner.rs:311`
**Issue**: `max_iterations = remaining_specs.len() * 2` - verify this is sufficient for all edge cases
```rust
let max_iterations = remaining_specs.len() * 2; // Reasonable upper bound
```
**Action**: Consider pathological cases with deep dependency chains

### 34. Agent Removal Pattern
**File**: `src/master/orchestrator.rs:147`
**Issue**: `execute_parallel()` removes agents from vector during iteration
```rust
let mut agent = agents.remove(idx);
```
**Status**: ✅ This is intentional - each agent used once, no reuse

### 35. TUI App Run Method
**File**: `src/tui/app.rs:48-53`
**Issue**: Unusual pattern - `run()` takes `mut self` ownership but closure borrows self
```rust
pub async fn run(mut self) -> Result<()> {
    super::run_tui(|terminal| {
        self.draw(terminal)?;
        self.handle_events()
    })
}
```
**Status**: ✅ Pattern is correct, just unusual - FnMut closure can mutably borrow

---

## Priority Summary

| Priority | Count | Focus |
|----------|-------|-------|
| 🔴 Critical | 5 | Fix immediately (crashes, logic bugs) |
| 🟡 Missing Implementation | 5 | Core features not complete |
| 🟠 Incomplete/Simplified | 5 | Production readiness issues |
| 📋 Documentation | 3 | Docs don't match code |
| 🔧 Code Smells | 5 | Technical debt |
| 🧪 Testing | 2 | Expand coverage |
| ⚠️ Minor/Verify | 5 | Lower priority checks |
| 📚 Completeness | 5 | Review remaining items |

**Total Items**: 35

---

## Recommended Action Plan

### Phase 1 - Critical Fixes (1-2 hours)
1. Fix division by zero in usage tracker
2. Fix incorrect dependency logic in planner
3. Fix hardcoded capability in database
4. Implement capability distribution query
5. Add percentage clamping

### Phase 2 - Complete Missing Features (3-4 hours)
6. Add Debugging, Performance, Review agent planning
7. Add Review capability keyword detection
8. Implement input validation
9. Wire up real data to TUI tabs
10. Add config validation

### Phase 3 - Production Readiness (4-6 hours)
11. Improve auth session handling
12. Verify API endpoints
13. Fix cost calculation accuracy
14. Implement proper time remaining calculation
15. Improve keyword extraction

### Phase 4 - Testing & Polish (4-6 hours)
16. Expand test coverage to 60%+
17. Implement TODO test scenarios
18. Add HTTP mock tests
19. Add logging for silent errors
20. Update documentation to match code

---

**Note**: This TODO list was generated from automated code review on 2025-11-14. Line numbers may shift as code is modified. Always verify current state before making changes.
