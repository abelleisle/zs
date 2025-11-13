use std::fs;

use anyhow::{Context, Result};
use serde::{Deserialize, Serialize};

use crate::workspace::Workspace;

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct Direnv {
    #[serde(default)]
    trust: bool,

    #[serde(skip_serializing_if = "Option::is_none")]
    envrc: Option<String>,
}

impl Direnv {
    /// Trust the workspace directory with direnv if trust is enabled
    pub fn trust(&self, workspace: &Workspace) -> Result<()> {
        if !self.trust {
            return Ok(());
        }

        println!("Trusting workspace with direnv...");

        let status = std::process::Command::new("direnv")
            .arg("allow")
            .arg(&workspace.path)
            .status()
            .context("Failed to run direnv allow")?;

        if !status.success() {
            anyhow::bail!("direnv allow failed with exit code: {:?}", status.code());
        }

        println!("Workspace trusted with direnv");
        Ok(())
    }

    /// Create .envrc file in the workspace if envrc content is specified
    pub fn create(&self, workspace: &Workspace) -> Result<()> {
        if let Some(envrc_content) = &self.envrc {
            println!("Creating .envrc file...");

            let envrc_path = workspace.path.join(".envrc");
            fs::write(&envrc_path, envrc_content).context("Failed to write .envrc file")?;

            println!("Created .envrc at: {}", envrc_path.display());
        }

        Ok(())
    }
}
