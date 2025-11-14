use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub enum AgentCapability {
    Architecture,
    CodeWriting,
    Testing,
    Security,
    Documentation,
    Debugging,
    Performance,
    Migration,
    Review,
}

impl AgentCapability {
    pub fn description(&self) -> &str {
        match self {
            Self::Architecture => "system design and architecture",
            Self::CodeWriting => "writing production-quality code",
            Self::Testing => "comprehensive testing and quality assurance",
            Self::Security => "security auditing and vulnerability detection",
            Self::Documentation => "technical documentation and guides",
            Self::Debugging => "debugging and bug fixing",
            Self::Performance => "performance optimization and profiling",
            Self::Migration => "code and data migration",
            Self::Review => "code review and quality assessment",
        }
    }

    pub fn emoji(&self) -> &str {
        match self {
            Self::Architecture => "🏗️",
            Self::CodeWriting => "💻",
            Self::Testing => "🧪",
            Self::Security => "🔒",
            Self::Documentation => "📚",
            Self::Debugging => "🐛",
            Self::Performance => "⚡",
            Self::Migration => "🔄",
            Self::Review => "👁️",
        }
    }

    /// Parse capability from Debug string representation
    /// Returns None for invalid/unknown capability strings
    pub fn from_str(s: &str) -> Option<Self> {
        match s {
            "Architecture" => Some(Self::Architecture),
            "CodeWriting" => Some(Self::CodeWriting),
            "Testing" => Some(Self::Testing),
            "Security" => Some(Self::Security),
            "Documentation" => Some(Self::Documentation),
            "Debugging" => Some(Self::Debugging),
            "Performance" => Some(Self::Performance),
            "Migration" => Some(Self::Migration),
            "Review" => Some(Self::Review),
            _ => None,
        }
    }
}
