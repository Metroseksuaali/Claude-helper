use super::orchestrator::ExecutionPlan;
use crate::agents::{Agent, AgentCapability, ClaudeAgent};
use crate::config::Config;
use anyhow::Result;

pub struct AgentFactory {
    config: Config,
}

impl AgentFactory {
    pub fn new(config: Config) -> Self {
        Self { config }
    }

    /// Create agents based on execution plan
    pub async fn create_agents(&self, plan: &ExecutionPlan) -> Result<Vec<Box<dyn Agent>>> {
        let mut agents = Vec::new();

        for phase in &plan.phases {
            for spec in &phase.agents {
                let agent = self
                    .create_agent(&spec.id, &spec.agent_type, &spec.capability, &spec.task)
                    .await?;

                agents.push(agent);
            }
        }

        Ok(agents)
    }

    async fn create_agent(
        &self,
        id: &str,
        agent_type: &str,
        capability: &AgentCapability,
        _task_description: &str,
    ) -> Result<Box<dyn Agent>> {
        // Create a system prompt based on agent type and capability
        let system_prompt = self.generate_system_prompt(agent_type, capability);

        // Create Claude agent with specialized prompt
        let agent = ClaudeAgent::new(
            id.to_string(),
            agent_type.to_string(),
            capability.clone(),
            system_prompt,
            self.config.clone(),
        )
        .await?;

        Ok(Box::new(agent))
    }

    fn generate_system_prompt(&self, agent_type: &str, capability: &AgentCapability) -> String {
        let base = format!(
            "You are {}, a specialized AI agent with expertise in {}.",
            agent_type,
            capability.description()
        );

        let specific = match capability {
            AgentCapability::Architecture => {
                "Your role is to design system architecture and create implementation plans. \
                Focus on:\n\
                - System design and component interaction\n\
                - Technology selection and trade-offs\n\
                - Scalability and maintainability\n\
                - Clear documentation of architectural decisions\n\n\
                Provide a comprehensive design document with diagrams (using text/ASCII) where helpful."
            }

            AgentCapability::CodeWriting => {
                "Your role is to write high-quality, production-ready code. \
                Focus on:\n\
                - Clean, readable, and maintainable code\n\
                - Following best practices and design patterns\n\
                - Proper error handling\n\
                - Code comments where necessary\n\
                - Type safety and correctness\n\n\
                Write complete, working code that can be directly used."
            }

            AgentCapability::Testing => {
                "Your role is to write comprehensive tests. \
                Focus on:\n\
                - Unit tests for individual functions/methods\n\
                - Integration tests for component interaction\n\
                - Edge cases and error conditions\n\
                - Test coverage and quality\n\
                - Clear test descriptions\n\n\
                Write tests that are thorough, maintainable, and catch potential bugs."
            }

            AgentCapability::Security => {
                "Your role is to audit code for security vulnerabilities. \
                Focus on:\n\
                - OWASP Top 10 vulnerabilities\n\
                - Input validation and sanitization\n\
                - Authentication and authorization\n\
                - Data encryption and secure storage\n\
                - Security best practices\n\n\
                Provide detailed security analysis with specific recommendations for fixes."
            }

            AgentCapability::Documentation => {
                "Your role is to create comprehensive documentation. \
                Focus on:\n\
                - Clear API documentation\n\
                - Usage examples and tutorials\n\
                - Architecture overview\n\
                - Installation and setup instructions\n\
                - Troubleshooting guides\n\n\
                Write documentation that is clear, complete, and helpful for developers."
            }

            AgentCapability::Debugging => {
                "Your role is to find and fix bugs. \
                Focus on:\n\
                - Systematic debugging approach\n\
                - Root cause analysis\n\
                - Minimal, targeted fixes\n\
                - Preventing similar bugs\n\
                - Testing the fix\n\n\
                Provide clear explanation of the bug and why your fix resolves it."
            }

            AgentCapability::Performance => {
                "Your role is to optimize performance. \
                Focus on:\n\
                - Identifying bottlenecks\n\
                - Algorithm and data structure optimization\n\
                - Resource usage (CPU, memory, I/O)\n\
                - Benchmarking and profiling\n\
                - Caching strategies\n\n\
                Provide measurable performance improvements with before/after metrics."
            }

            AgentCapability::Migration => {
                "Your role is to plan and execute migrations. \
                Focus on:\n\
                - Migration strategy and planning\n\
                - Data preservation and integrity\n\
                - Backward compatibility where needed\n\
                - Rollback procedures\n\
                - Testing migration thoroughly\n\n\
                Provide a safe, well-tested migration path with clear steps."
            }

            AgentCapability::Review => {
                "Your role is to review code for quality. \
                Focus on:\n\
                - Code quality and maintainability\n\
                - Best practices adherence\n\
                - Potential bugs or issues\n\
                - Performance considerations\n\
                - Consistency with codebase\n\n\
                Provide constructive feedback with specific suggestions for improvement."
            }
        };

        format!("{}\n\n{}", base, specific)
    }
}
