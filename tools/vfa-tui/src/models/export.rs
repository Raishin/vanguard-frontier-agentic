use std::path::PathBuf;

use crate::error::TuiError;
use crate::security::validate::validate_argument;

/// Represents the selection criteria for export operations.
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum ExportSelection {
    /// Export all agents.
    All,
    /// Export agents for a specific role.
    Role(String),
    /// Export agents from a specific provider.
    Provider(String),
    /// Export specific agents by ID.
    Agents(Vec<String>),
}

/// Represents an export command to be executed as a subprocess.
#[derive(Debug, Clone)]
pub struct ExportCommand {
    pub platform: String,
    pub selection: ExportSelection,
    pub target_repo: PathBuf,
    pub dry_run: bool,
    pub force: bool,
    pub no_skills: bool,
}

impl ExportCommand {
    /// Create a new export command with default options (dry_run=true, force=false, no_skills=false).
    pub fn new(platform: String, selection: ExportSelection, target_repo: PathBuf) -> Self {
        Self {
            platform,
            selection,
            target_repo,
            dry_run: true,
            force: false,
            no_skills: false,
        }
    }

    /// Validate all user-provided arguments. Returns an error if any argument
    /// contains shell metacharacters or other forbidden characters.
    pub fn validate(&self) -> Result<(), TuiError> {
        validate_argument(&self.platform)?;

        match &self.selection {
            ExportSelection::All => {}
            ExportSelection::Role(role) => {
                validate_argument(role)?;
            }
            ExportSelection::Provider(provider) => {
                validate_argument(provider)?;
            }
            ExportSelection::Agents(ids) => {
                for id in ids {
                    validate_argument(id)?;
                }
            }
        }

        let target_str = self.target_repo.to_string_lossy();
        validate_argument(&target_str)?;

        Ok(())
    }

    /// Build the argument array for subprocess invocation.
    ///
    /// The command is: `node scripts/export-marketplace-agents.mjs <args>`
    pub fn to_args(&self) -> Vec<String> {
        let mut args = Vec::new();

        args.push("--platform".to_string());
        args.push(self.platform.clone());

        match &self.selection {
            ExportSelection::All => {
                args.push("--all".to_string());
            }
            ExportSelection::Role(role) => {
                args.push("--role".to_string());
                args.push(role.clone());
            }
            ExportSelection::Provider(provider) => {
                args.push("--provider".to_string());
                args.push(provider.clone());
            }
            ExportSelection::Agents(ids) => {
                args.push(format!("--agents={}", ids.join(",")));
            }
        }

        args.push("--repo".to_string());
        args.push(self.target_repo.to_string_lossy().to_string());

        if self.dry_run {
            args.push("--dry-run".to_string());
        }

        if self.force {
            args.push("--force".to_string());
        }

        if self.no_skills {
            args.push("--no-skills".to_string());
        }

        args
    }

    /// Render the full command string for display/preview.
    pub fn display_command(&self) -> String {
        format!(
            "node scripts/export-marketplace-agents.mjs {}",
            self.to_args().join(" ")
        )
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn to_args_all_selection() {
        let cmd = ExportCommand::new(
            "kiro".to_string(),
            ExportSelection::All,
            PathBuf::from("/tmp/target"),
        );
        let args = cmd.to_args();
        assert!(args.contains(&"--platform".to_string()));
        assert!(args.contains(&"kiro".to_string()));
        assert!(args.contains(&"--all".to_string()));
        assert!(args.contains(&"--repo".to_string()));
        assert!(args.contains(&"/tmp/target".to_string()));
        assert!(args.contains(&"--dry-run".to_string()));
        assert!(!args.contains(&"--force".to_string()));
        assert!(!args.contains(&"--no-skills".to_string()));
    }

    #[test]
    fn to_args_role_selection() {
        let cmd = ExportCommand::new(
            "cursor".to_string(),
            ExportSelection::Role("devops-engineer".to_string()),
            PathBuf::from("/tmp/target"),
        );
        let args = cmd.to_args();
        assert!(args.contains(&"--role".to_string()));
        assert!(args.contains(&"devops-engineer".to_string()));
    }

    #[test]
    fn to_args_provider_selection() {
        let cmd = ExportCommand::new(
            "claude".to_string(),
            ExportSelection::Provider("aws".to_string()),
            PathBuf::from("/tmp/target"),
        );
        let args = cmd.to_args();
        assert!(args.contains(&"--provider".to_string()));
        assert!(args.contains(&"aws".to_string()));
    }

    #[test]
    fn to_args_agents_selection() {
        let cmd = ExportCommand::new(
            "kiro".to_string(),
            ExportSelection::Agents(vec!["agent-a".to_string(), "agent-b".to_string()]),
            PathBuf::from("/tmp/target"),
        );
        let args = cmd.to_args();
        assert!(args.contains(&"agent-a".to_string()));
        assert!(args.contains(&"agent-b".to_string()));
        assert!(!args.contains(&"--all".to_string()));
        assert!(!args.contains(&"--role".to_string()));
        assert!(!args.contains(&"--provider".to_string()));
    }

    #[test]
    fn to_args_with_force_and_no_skills() {
        let mut cmd = ExportCommand::new(
            "kiro".to_string(),
            ExportSelection::All,
            PathBuf::from("/tmp/target"),
        );
        cmd.dry_run = false;
        cmd.force = true;
        cmd.no_skills = true;
        let args = cmd.to_args();
        assert!(!args.contains(&"--dry-run".to_string()));
        assert!(args.contains(&"--force".to_string()));
        assert!(args.contains(&"--no-skills".to_string()));
    }

    #[test]
    fn display_command_format() {
        let cmd = ExportCommand::new(
            "kiro".to_string(),
            ExportSelection::All,
            PathBuf::from("/tmp/target"),
        );
        let display = cmd.display_command();
        assert!(display.starts_with("node scripts/export-marketplace-agents.mjs"));
        assert!(display.contains("--platform"));
        assert!(display.contains("kiro"));
    }

    #[test]
    fn no_empty_strings_in_args() {
        let cmd = ExportCommand::new(
            "kiro".to_string(),
            ExportSelection::All,
            PathBuf::from("/tmp/target"),
        );
        for arg in cmd.to_args() {
            assert!(!arg.is_empty(), "found empty string in args");
        }
    }

    #[test]
    fn validate_accepts_safe_args() {
        let cmd = ExportCommand::new(
            "kiro".to_string(),
            ExportSelection::Role("devops-engineer".to_string()),
            PathBuf::from("/tmp/target"),
        );
        assert!(cmd.validate().is_ok());
    }

    #[test]
    fn validate_rejects_platform_with_metachar() {
        let cmd = ExportCommand::new(
            "kiro; rm -rf /".to_string(),
            ExportSelection::All,
            PathBuf::from("/tmp/target"),
        );
        assert!(cmd.validate().is_err());
    }

    #[test]
    fn validate_rejects_role_with_metachar() {
        let cmd = ExportCommand::new(
            "kiro".to_string(),
            ExportSelection::Role("role|cat /etc/passwd".to_string()),
            PathBuf::from("/tmp/target"),
        );
        assert!(cmd.validate().is_err());
    }

    #[test]
    fn validate_rejects_agent_id_with_metachar() {
        let cmd = ExportCommand::new(
            "kiro".to_string(),
            ExportSelection::Agents(vec!["good-agent".to_string(), "bad$(cmd)".to_string()]),
            PathBuf::from("/tmp/target"),
        );
        assert!(cmd.validate().is_err());
    }

    #[test]
    fn to_args_agents_selection_uses_agents_flag() {
        let cmd = ExportCommand::new(
            "kiro".to_string(),
            ExportSelection::Agents(vec!["agent-a".to_string(), "agent-b".to_string()]),
            PathBuf::from("/tmp/target"),
        );
        let args = cmd.to_args();
        assert!(args.iter().any(|arg| arg == "--agents=agent-a,agent-b"));
        assert!(!args.iter().any(|arg| arg == "agent-a"));
    }
}
