mod git;

use std::path::PathBuf;

use anyhow::{Context, Result};
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Debug)]
pub struct Repo {
    pub path: PathBuf,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub workspace_hook: Option<String>,

    #[serde(flatten)]
    pub repo: RepoType,
}

#[derive(Serialize, Deserialize, Debug)]
#[serde(tag = "type", rename_all = "lowercase")]
pub enum RepoType {
    Git(git::GitRepo),
}

impl Repo {
    pub fn expand_path(&mut self) -> Result<()> {
        let binding = self.path.to_string_lossy();
        let expanded = shellexpand::full(&binding).context("Failed to expand path")?;
        self.path = PathBuf::from(expanded.as_ref());
        Ok(())
    }

    pub fn ensure_path_exists(&self) -> Result<()> {
        if !self.path.exists() {
            std::fs::create_dir_all(&self.path)?;
        }

        Ok(())
    }

    // Make sure the repo is cloned in `path/primary`
    pub fn ensure_cloned(&self) -> Result<()> {
        let primary_path = self.path.join("primary");

        // Check if the primary directory exists and is not empty
        if primary_path.exists() && primary_path.read_dir()?.next().is_some() {
            println!("Repository already cloned at: {}", primary_path.display());
            return Ok(());
        }

        // Ensure the parent path exists
        self.ensure_path_exists()?;

        // Delegate to the specific repo type
        match &self.repo {
            RepoType::Git(git_repo) => {
                git_repo.ensure_cloned(&primary_path)?;
            }
        }

        Ok(())
    }

    // Create a worktree for this repo
    pub fn create_worktree(
        &self,
        branch_name: &str,
        worktree_path: &std::path::Path,
    ) -> Result<()> {
        let primary_path = self.path.join("primary");

        // Delegate to the specific repo type
        match &self.repo {
            RepoType::Git(git_repo) => {
                git_repo.create_worktree(&primary_path, branch_name, worktree_path)?;
            }
        }

        Ok(())
    }

    // Delete a worktree for this repo
    pub fn delete_worktree(&self, worktree_path: &std::path::Path) -> Result<()> {
        let primary_path = self.path.join("primary");

        // Delegate to the specific repo type
        match &self.repo {
            RepoType::Git(git_repo) => {
                git_repo.delete_worktree(&primary_path, worktree_path)?;
            }
        }

        Ok(())
    }

    // Execute workspace hook if defined
    pub fn execute_workspace_hook(&self, workspace_path: &std::path::Path) -> Result<()> {
        if let Some(hook) = &self.workspace_hook {
            println!("\n=== Executing workspace hook ===");
            println!("Working directory: {}", workspace_path.display());
            println!("\n{}", hook);
            println!("\n================================\n");

            let mut cmd = std::process::Command::new("sh");
            cmd.arg("-c")
                .arg(hook)
                .current_dir(workspace_path);

            // Use status() instead of output() to inherit stdio and show live output
            let status = cmd.status()
                .context("Failed to execute workspace hook")?;

            if !status.success() {
                anyhow::bail!("Workspace hook failed with exit code: {:?}", status.code());
            }

            println!("\n=== Workspace hook completed successfully ===\n");
        }

        Ok(())
    }

    // Update the repository to the latest version
    pub fn update(&self) -> Result<()> {
        let primary_path = self.path.join("primary");

        // Delegate to the specific repo type
        match &self.repo {
            RepoType::Git(git_repo) => {
                git_repo.update(&primary_path)?;
            }
        }

        Ok(())
    }
}
