use anyhow::Result;
use std::collections::HashMap;
use super::session_parser::SessionData;

#[derive(Debug, Clone)]
pub enum OptimizationType {
    QuickCommand,
    ParameterizedScript,
    FileMerge,
    FileSplit,
    ContextPruning,
    ToolCallBatching,
}

#[derive(Debug, Clone)]
pub struct Optimization {
    pub opt_type: OptimizationType,
    pub title: String,
    pub description: String,
    pub estimated_savings: usize, // tokens
    pub examples: Vec<String>,
    pub suggestion: Option<String>,
}

pub struct Optimizer {
    min_savings_threshold: usize,
}

impl Optimizer {
    pub fn new(min_savings_threshold: usize) -> Self {
        Self { min_savings_threshold }
    }

    pub fn analyze(&self, session: &SessionData) -> Result<Vec<Optimization>> {
        let mut optimizations = Vec::new();

        // Detect bash command chains
        optimizations.extend(self.detect_bash_chains(session)?);

        // Detect frequently co-accessed files
        optimizations.extend(self.detect_file_patterns(session)?);

        // Detect large file reads
        optimizations.extend(self.detect_large_files(session)?);

        // Detect repeated tool calls
        optimizations.extend(self.detect_tool_repetition(session)?);

        // Filter by threshold
        optimizations.retain(|opt| opt.estimated_savings >= self.min_savings_threshold);

        Ok(optimizations)
    }

    fn detect_bash_chains(&self, session: &SessionData) -> Result<Vec<Optimization>> {
        let mut optimizations = Vec::new();

        // Find sequences of bash commands
        let bash_calls: Vec<&str> = session.tool_calls
            .iter()
            .filter(|tc| tc.tool_name == "Bash")
            .filter_map(|tc| tc.parameters.get("command").and_then(|c| c.as_str()))
            .collect();

        // Look for patterns like: git add . -> git commit -> git push
        if bash_calls.len() >= 3 {
            // Check for git workflows
            let git_commands: Vec<&str> = bash_calls.iter()
                .filter(|cmd| cmd.starts_with("git"))
                .copied()
                .collect();

            if git_commands.len() >= 3 {
                optimizations.push(Optimization {
                    opt_type: OptimizationType::QuickCommand,
                    title: "Combine git operations into single command".to_string(),
                    description: "Multiple sequential git commands can be combined with &&".to_string(),
                    estimated_savings: git_commands.len() * 200, // ~200 tokens per command
                    examples: git_commands.iter().take(3).map(|s| s.to_string()).collect(),
                    suggestion: Some("Use: git add . && git commit -m 'message' && git push".to_string()),
                });
            }

            // Check for test + build workflows
            let has_test = bash_calls.iter().any(|cmd| cmd.contains("test") || cmd.contains("jest") || cmd.contains("pytest"));
            let has_build = bash_calls.iter().any(|cmd| cmd.contains("build") || cmd.contains("compile"));

            if has_test && has_build {
                optimizations.push(Optimization {
                    opt_type: OptimizationType::ParameterizedScript,
                    title: "Create test-and-build script".to_string(),
                    description: "Frequently run test and build commands together".to_string(),
                    estimated_savings: 400,
                    examples: vec!["npm test".to_string(), "npm run build".to_string()],
                    suggestion: Some("Create package.json script: \"test-and-build\": \"npm test && npm run build\"".to_string()),
                });
            }
        }

        Ok(optimizations)
    }

    fn detect_file_patterns(&self, session: &SessionData) -> Result<Vec<Optimization>> {
        let mut optimizations = Vec::new();

        // Count file access frequency
        let mut file_counts: HashMap<String, usize> = HashMap::new();

        for access in &session.file_accesses {
            *file_counts.entry(access.file_path.clone()).or_insert(0) += 1;
        }

        // Find files accessed together frequently
        let frequently_accessed: Vec<_> = file_counts.iter()
            .filter(|(_, count)| **count >= 3)
            .collect();

        if frequently_accessed.len() >= 2 {
            // Check if they're related (e.g., same directory, similar names)
            let paths: Vec<&String> = frequently_accessed.iter().map(|(path, _)| *path).collect();

            // Simple heuristic: check if in same directory
            if let Some(first_path) = paths.first() {
                let first_dir = std::path::Path::new(first_path).parent();

                let same_dir_count = paths.iter()
                    .filter(|path| {
                        std::path::Path::new(path).parent() == first_dir
                    })
                    .count();

                if same_dir_count >= 2 {
                    optimizations.push(Optimization {
                        opt_type: OptimizationType::FileMerge,
                        title: "Consider merging frequently co-accessed files".to_string(),
                        description: "These files are often accessed together and could potentially be merged".to_string(),
                        estimated_savings: same_dir_count * 500,
                        examples: paths.iter().take(3).map(|s| s.to_string()).collect(),
                        suggestion: Some("Evaluate if these files should be combined into a single module".to_string()),
                    });
                }
            }
        }

        Ok(optimizations)
    }

    fn detect_large_files(&self, _session: &SessionData) -> Result<Vec<Optimization>> {
        let optimizations = Vec::new();

        // This would analyze file sizes and suggest splitting
        // For now, using simple heuristic

        // Placeholder optimization
        // In reality, you'd check actual file sizes

        Ok(optimizations)
    }

    fn detect_tool_repetition(&self, session: &SessionData) -> Result<Vec<Optimization>> {
        let mut optimizations = Vec::new();

        // Count tool usage
        let mut tool_counts: HashMap<String, usize> = HashMap::new();

        for tool_call in &session.tool_calls {
            *tool_counts.entry(tool_call.tool_name.clone()).or_insert(0) += 1;
        }

        // Look for excessive Grep calls
        if let Some(grep_count) = tool_counts.get("Grep") {
            if *grep_count > 5 {
                optimizations.push(Optimization {
                    opt_type: OptimizationType::ToolCallBatching,
                    title: "Reduce redundant Grep searches".to_string(),
                    description: format!("Found {} Grep calls - some might be redundant", grep_count),
                    estimated_savings: (grep_count - 2) * 100,
                    examples: vec![format!("{} Grep tool calls in session", grep_count)],
                    suggestion: Some("Use more specific patterns or combine searches".to_string()),
                });
            }
        }

        // Look for excessive Read calls
        if let Some(read_count) = tool_counts.get("Read") {
            if *read_count > 10 {
                optimizations.push(Optimization {
                    opt_type: OptimizationType::ContextPruning,
                    title: "Many file reads detected".to_string(),
                    description: format!("Found {} Read calls - consider if all are necessary", read_count),
                    estimated_savings: (read_count - 5) * 300,
                    examples: vec![format!("{} Read tool calls in session", read_count)],
                    suggestion: Some("Read only files that are directly relevant to the task".to_string()),
                });
            }
        }

        Ok(optimizations)
    }
}
