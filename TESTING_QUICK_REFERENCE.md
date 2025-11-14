# Claude Helper - Testing Quick Reference Guide

## At a Glance

| Category | Status | Count |
|----------|--------|-------|
| **Automated Tests** | ❌ None | 0 |
| **Test Modules** | ❌ Missing | 0 |
| **Untested Functions** | ⚠️ Critical | 12+ |
| **Manual Tests** | ✅ Completed | CLI, Config, DB Schema |
| **Test Dependencies** | ✅ Available | mockito, tempfile |

---

## Priority Matrix: What to Test First

### CRITICAL (Start Here!)

1. **`planner.rs:76-109` - `estimate_complexity()`**
   - Complexity: HIGH
   - Impact: Task analysis foundation
   - Estimated test count: 8-10 tests
   - Time to implement: 2-3 hours

2. **`planner.rs:292-341` - `create_phases()`**
   - Complexity: VERY HIGH  
   - Impact: Execution planning, orchestration
   - Estimated test count: 9-11 tests
   - Time to implement: 3-4 hours

3. **`db/mod.rs:73-108` - `get_agent_stats()`**
   - Complexity: MEDIUM
   - Impact: Statistics and learning
   - Estimated test count: 5-6 tests
   - Time to implement: 2-3 hours

### HIGH (Do Next Week)

4. **`optimizer.rs:55-101` - `detect_bash_chains()`**
   - Test count: 5 tests, 2 hours

5. **`optimizer.rs:103-146` - `detect_file_patterns()`**
   - Test count: 5 tests, 2 hours

6. **`db/mod.rs:176-207` - `get_hourly_breakdown()`**
   - Test count: 4 tests, 2 hours
   - ⚠️ WARNING: SQL injection vulnerability in line 177-186!

---

## Test Categories by Type

### Unit Tests (Can run without database/API)

**In `src/master/planner.rs`:**
```rust
#[cfg(test)]
mod tests {
    #[test]
    fn test_estimate_complexity_low() { }
    
    #[test]
    fn test_detect_capabilities_multiple() { }
    
    #[test]
    fn test_create_phases_topological_sort() { }
}
```

**In `src/statusline/usage_tracker.rs`:**
```rust
#[test]
fn test_percentage_calculation() { }

#[test]
fn test_burn_rate_calculation() { }
```

**In `src/analyzer/optimizer.rs`:**
```rust
#[test]
fn test_detect_bash_chains_git() { }

#[test]
fn test_detect_file_patterns_same_dir() { }
```

### Integration Tests (Need database)

**Create `tests/integration/database_tests.rs`:**
```rust
#[tokio::test]
async fn test_save_and_retrieve_task_execution() { }

#[tokio::test]
async fn test_agent_stats_aggregation() { }

#[tokio::test]
async fn test_hourly_breakdown_time_series() { }
```

### System Tests (Full workflow)

**Create `tests/integration/orchestration_tests.rs`:**
```rust
#[tokio::test]
async fn test_execute_plan_with_mock_agents() { }

#[tokio::test]
async fn test_parallel_execution_respects_semaphore() { }

#[tokio::test]
async fn test_autonomy_mode_conservative() { }
```

---

## Test Data Fixtures

### Mock Data Helpers

**Location**: `tests/common/mod.rs`

```rust
// Create reusable test data
pub fn sample_task_analysis() -> TaskAnalysis {
    TaskAnalysis {
        task_description: "Write tests".to_string(),
        complexity: 5,
        estimated_files: 3,
        estimated_tokens: 6000,
        estimated_time_min: 5,
        estimated_time_max: 15,
        required_capabilities: vec![AgentCapability::Testing],
        keywords: vec!["write", "tests".to_string()],
    }
}

pub fn sample_execution_plan() -> ExecutionPlan {
    ExecutionPlan {
        phases: vec![/* ... */],
    }
}

pub async fn setup_test_db() -> Database {
    // Create temp database for testing
}
```

### Sample JSONL Session Files

**Location**: `tests/fixtures/sessions/`

```json
{"role": "user", "content": "Write a function", "timestamp": "2025-11-14T10:00:00Z"}
{"tool_use": {"name": "Read", "input": {"file_path": "/src/main.rs"}}, "timestamp": "2025-11-14T10:00:01Z"}
{"tool_use": {"name": "Bash", "input": {"command": "git add ."}}, "timestamp": "2025-11-14T10:00:02Z"}
```

---

## Edge Cases & Boundary Values to Test

### Complexity Calculation
- [ ] Empty string: `""` → expects 3 (base)
- [ ] Single character: `"a"` → expects 3 (base)
- [ ] No keywords: `"write a function"` → expects 4 (base + "write")
- [ ] Multiple high keywords: `"refactor authenticate and migrate"` → should cap at 10
- [ ] All low keywords: basic + implement + create → expects 5

### Token Calculations
- [ ] Zero tokens: 0 used / 20000 limit → 0%
- [ ] Half limit: 10000 / 20000 → 50%
- [ ] Over limit: 21000 / 20000 → 100% (capped)
- [ ] Very small: 1 / 20000 → rounds to 0% vs 1%?
- [ ] All limits zero: division by zero protection?

### Database Queries
- [ ] Empty database: should return zero stats, not crash
- [ ] Single record: accurate averages
- [ ] Large dataset: 10000+ records, performance
- [ ] Null values: how handled in aggregation?
- [ ] DateTime parsing: invalid formats

### File Patterns
- [ ] No file accesses: empty vector
- [ ] Single file: no pattern
- [ ] Same file 10 times: frequency detection
- [ ] Different directories: should not merge
- [ ] Path with special chars: `/home/user/My Projects/file.rs`

### Dependency Resolution
- [ ] No agents: empty plan
- [ ] Single agent: single phase
- [ ] Linear chain: A → B → C → 3 phases
- [ ] Diamond: A → B,C → D → 3 phases
- [ ] Fully parallel: no dependencies → 1 phase
- [ ] Circular: A → B → C → A → error handling?

---

## Testing Tools Already Available

### In Cargo.toml

```toml
[dev-dependencies]
mockito = "1.5"          # HTTP request mocking
tempfile = "3.13"        # Temporary file/directory creation
```

### Recommended Additions

```bash
# Code coverage
cargo install cargo-tarpaulin

# Run with coverage
cargo tarpaulin --out Html

# Property-based testing (optional)
# Add to Cargo.toml: proptest = "1.0"
```

---

## How to Run Tests

### Once tests are implemented:

```bash
# Run all tests
cargo test

# Run tests with output
cargo test -- --nocapture

# Run specific test file
cargo test --test database_tests

# Run specific test
cargo test test_estimate_complexity_low

# Run with coverage report
cargo tarpaulin --out Html --output-dir coverage
open coverage/index.html
```

---

## Known Issues to Fix During Testing

### Database Security
- **File**: `src/db/mod.rs:177-186`
- **Issue**: SQL injection vulnerability in `get_hourly_breakdown()`
- **Current**: String formatting in SQL query
- **Fix**: Use parameterized queries
- **Priority**: CRITICAL

### Missing Validation
- **File**: `src/config/mod.rs:122`
- **Issue**: No validation of loaded config values
- **Fix**: Validate mode, max_agents > 0, timeouts reasonable
- **Priority**: HIGH

### Error Recovery
- **File**: `src/master/orchestrator.rs:306-314`
- **Issue**: Circular dependencies cause infinite loop
- **Fix**: Add iteration limit and proper error message
- **Priority**: HIGH

### DateTime Parsing
- **File**: `src/db/mod.rs:125-127, 195-197, 223-225`
- **Issue**: Silent failure on invalid datetime
- **Fix**: Log warnings, handle gracefully
- **Priority**: MEDIUM

---

## Test Implementation Checklist

### Phase 1: Unit Tests (This Week)
- [ ] Task complexity calculation (10 tests)
- [ ] Capability detection (8 tests)
- [ ] Phase creation/dependency resolution (10 tests)
- [ ] **Total**: ~28 unit tests

### Phase 2: Integration Tests (Next Week)
- [ ] Database operations (8 tests)
- [ ] Configuration loading (5 tests)
- [ ] Session parsing (6 tests)
- [ ] **Total**: ~19 integration tests

### Phase 3: System Tests (2 Weeks)
- [ ] Orchestration workflow (7 tests)
- [ ] Optimizer algorithms (8 tests)
- [ ] Error scenarios (5 tests)
- [ ] **Total**: ~20 system tests

### Phase 4: Coverage & Quality (3 Weeks)
- [ ] Reach 80% line coverage
- [ ] Reach 90% function coverage
- [ ] Fix all security issues
- [ ] Document test patterns

---

## Resource Links

**From CLAUDE.md:**
- Task Planning: `src/master/planner.rs:7-17`
- Capability Detection: `src/master/planner.rs:111-142`
- Dependency Resolution: `src/master/planner.rs:292-341`
- Database Queries: `src/db/mod.rs:41-237`
- Optimization Detection: `src/analyzer/optimizer.rs:34-53`

**Rust Testing Patterns:**
- https://doc.rust-lang.org/book/ch11-00-testing.html
- https://doc.rust-lang.org/book/ch11-03-test-organization.html

**Async Testing with Tokio:**
- https://tokio.rs/#fn-examples

---

**Last Updated**: 2025-11-14
**Next Review**: After Phase 1 tests complete
**Questions?** See TEST_COVERAGE_ANALYSIS.md for detailed information
