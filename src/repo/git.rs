use std::process::Command;

use anyhow::{Context, Result, bail};
use serde::{Deserialize, Serialize};

fn default_true() -> bool {
    true
}

#[derive(Serialize, Deserialize, Debug)]
pub struct GitRepo {
    // Fields for GitRepo
    pub url: String,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub branch: Option<String>,

    #[serde(default)]
    pub shallow: bool,

    #[serde(default = "default_true")]
    pub submodules: bool,
}

impl GitRepo {
    pub fn ensure_cloned(&self, path: &std::path::Path) -> Result<()> {
        println!("Cloning repository from: {}", self.url);
        println!("  Branch: {}", self.branch.as_deref().unwrap_or("default"));
        println!("  Shallow: {}", self.shallow);
        println!("  Submodules: {}", self.submodules);

        let mut cmd = Command::new("git");
        cmd.arg("clone");

        // Add shallow clone flag if requested
        if self.shallow {
            cmd.arg("--depth").arg("1");
        }

        // Add branch if specified
        if let Some(branch) = &self.branch {
            cmd.arg("--branch").arg(branch);
        }

        // Add submodules flag if requested
        if self.submodules {
            cmd.arg("--recurse-submodules");
        }

        // Add URL and destination path
        cmd.arg(&self.url);
        cmd.arg(path);

        // Execute the clone command
        let output = cmd
            .output()
            .context("Failed to execute git clone command")?;

        if !output.status.success() {
            let stderr = String::from_utf8_lossy(&output.stderr);
            bail!("Git clone failed: {}", stderr);
        }

        println!("Successfully cloned to: {}", path.display());

        Ok(())
    }

    pub fn create_worktree(
        &self,
        repo_path: &std::path::Path,
        branch_name: &str,
        worktree_path: &std::path::Path,
    ) -> Result<()> {
        println!(
            "Creating worktree with branch '{}' at: {}",
            branch_name,
            worktree_path.display()
        );

        // Ensure parent directory exists
        if let Some(parent) = worktree_path.parent() {
            std::fs::create_dir_all(parent).context("Failed to create workspaces directory")?;
        }

        let mut cmd = Command::new("git");
        cmd.arg("-C")
            .arg(repo_path)
            .arg("worktree")
            .arg("add")
            .arg(worktree_path)
            .arg("-b")
            .arg(branch_name);

        // Execute the worktree add command
        let output = cmd
            .output()
            .context("Failed to execute git worktree add command")?;

        if !output.status.success() {
            let stderr = String::from_utf8_lossy(&output.stderr);
            bail!("Git worktree add failed: {}", stderr);
        }

        println!(
            "Successfully created worktree with branch '{}'",
            branch_name
        );

        Ok(())
    }
}
