use anyhow::{Context, Result};
use serde::{Deserialize, Serialize};
use std::path::{Path, PathBuf};
use std::fs;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SessionData {
    pub session_id: String,
    pub messages: Vec<Message>,
    pub tool_calls: Vec<ToolCall>,
    pub file_accesses: Vec<FileAccess>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Message {
    pub role: String,
    pub content: String,
    pub timestamp: Option<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ToolCall {
    pub tool_name: String,
    pub parameters: serde_json::Value,
    pub timestamp: Option<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct FileAccess {
    pub file_path: String,
    pub operation: String, // "read", "write", "edit"
    pub line_count: Option<usize>,
}

pub struct SessionParser {}

impl SessionParser {
    pub fn new() -> Self {
        Self {}
    }

    /// Find recent Claude Code sessions
    pub fn find_recent_sessions(&self, count: usize) -> Result<Vec<PathBuf>> {
        let sessions_dir = self.get_sessions_dir()?;

        if !sessions_dir.exists() {
            return Ok(Vec::new());
        }

        let mut sessions = Vec::new();

        for entry in fs::read_dir(&sessions_dir)
            .context("Failed to read sessions directory")?
        {
            let entry = entry?;
            let path = entry.path();

            if path.is_file() && path.extension().map_or(false, |ext| ext == "jsonl") {
                sessions.push(path);
            }
        }

        // Sort by modification time (newest first)
        sessions.sort_by_key(|path| {
            fs::metadata(path)
                .and_then(|m| m.modified())
                .ok()
        });
        sessions.reverse();

        Ok(sessions.into_iter().take(count).collect())
    }

    /// Find session by ID
    pub fn find_session_by_id(&self, session_id: &str) -> Result<PathBuf> {
        let sessions_dir = self.get_sessions_dir()?;
        let session_file = sessions_dir.join(format!("{}.jsonl", session_id));

        if !session_file.exists() {
            anyhow::bail!("Session not found: {}", session_id);
        }

        Ok(session_file)
    }

    /// Parse a session file
    pub fn parse_session(&self, path: &Path) -> Result<SessionData> {
        let content = fs::read_to_string(path)
            .context("Failed to read session file")?;

        let mut messages = Vec::new();
        let mut tool_calls = Vec::new();
        let mut file_accesses = Vec::new();

        // Parse JSONL (one JSON object per line)
        for line in content.lines() {
            if line.trim().is_empty() {
                continue;
            }

            let json: serde_json::Value = serde_json::from_str(line)
                .context("Failed to parse JSON line")?;

            // Extract messages
            if let Some(role) = json.get("role").and_then(|r| r.as_str()) {
                if let Some(content) = json.get("content").and_then(|c| c.as_str()) {
                    messages.push(Message {
                        role: role.to_string(),
                        content: content.to_string(),
                        timestamp: json.get("timestamp").and_then(|t| t.as_str()).map(|s| s.to_string()),
                    });
                }
            }

            // Extract tool calls
            if let Some(tool_use) = json.get("tool_use") {
                if let Some(name) = tool_use.get("name").and_then(|n| n.as_str()) {
                    tool_calls.push(ToolCall {
                        tool_name: name.to_string(),
                        parameters: tool_use.get("input").cloned().unwrap_or(serde_json::Value::Null),
                        timestamp: json.get("timestamp").and_then(|t| t.as_str()).map(|s| s.to_string()),
                    });

                    // Track file accesses
                    if name == "Read" || name == "Write" || name == "Edit" {
                        if let Some(file_path) = tool_use.get("input")
                            .and_then(|i| i.get("file_path"))
                            .and_then(|fp| fp.as_str())
                        {
                            file_accesses.push(FileAccess {
                                file_path: file_path.to_string(),
                                operation: name.to_lowercase(),
                                line_count: None,
                            });
                        }
                    }
                }
            }
        }

        let session_id = path.file_stem()
            .and_then(|s| s.to_str())
            .unwrap_or("unknown")
            .to_string();

        Ok(SessionData {
            session_id,
            messages,
            tool_calls,
            file_accesses,
        })
    }

    fn get_sessions_dir(&self) -> Result<PathBuf> {
        let home = dirs::home_dir()
            .context("Could not find home directory")?;

        Ok(home.join(".claude").join("sessions"))
    }
}
