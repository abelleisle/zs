use std::process::Command;

use anyhow::{Context, Result, bail};
use inquire::Confirm;
use serde::{Deserialize, Serialize};

use crate::{util::default_true, workspace::Workspace};

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
        workspace: &Workspace,
    ) -> Result<()> {
        println!(
            "Creating worktree with branch '{}' at: {}",
            workspace.branch,
            workspace.path.display()
        );

        // Ensure parent directory exists
        if let Some(parent) = workspace.path.parent() {
            std::fs::create_dir_all(parent).context("Failed to create workspaces directory")?;
        }

        let mut cmd = Command::new("git");
        cmd.arg("-C")
            .arg(repo_path)
            .arg("worktree")
            .arg("add")
            .arg(&workspace.path)
            .arg("-b")
            .arg(&workspace.branch);

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
            workspace.branch
        );

        Ok(())
    }

    pub fn delete_worktree(
        &self,
        repo_path: &std::path::Path,
        workspace: &Workspace,
    ) -> Result<()> {
        println!("Deleting worktree at: {}", workspace.path.display());

        let mut cmd = Command::new("git");
        cmd.arg("-C")
            .arg(repo_path)
            .arg("worktree")
            .arg("remove")
            .arg(&workspace.path);

        // Execute the worktree remove command
        let output = cmd
            .output()
            .context("Failed to execute git worktree remove command")?;

        if !output.status.success() {
            let stderr = String::from_utf8_lossy(&output.stderr);
            println!("Failed to remove worktree: {}", stderr);

            // Ask user if they want to force remove
            let force = Confirm::new("Would you like to force remove the worktree?")
                .with_default(false)
                .prompt()?;

            if !force {
                bail!("Worktree removal cancelled");
            }

            // Try again with --force flag
            println!("Force removing worktree...");
            let mut force_cmd = Command::new("git");
            force_cmd
                .arg("-C")
                .arg(repo_path)
                .arg("worktree")
                .arg("remove")
                .arg("--force")
                .arg(&workspace.path);

            let force_output = force_cmd
                .output()
                .context("Failed to execute git worktree remove --force command")?;

            if !force_output.status.success() {
                let force_stderr = String::from_utf8_lossy(&force_output.stderr);
                bail!("Git worktree remove --force failed: {}", force_stderr);
            }
        }

        {
            let mut cmd = Command::new("git");
            cmd.arg("-C")
                .arg(repo_path)
                .arg("branch")
                .arg("-d")
                .arg(&workspace.branch);

            // Execute the worktree remove command
            cmd.output()
                .context("Failed to remove old branch from primary repo")?;
        }

        println!("Successfully deleted worktree");

        Ok(())
    }

    pub fn update(&self, repo_path: &std::path::Path) -> Result<()> {
        println!("Updating repository at: {}", repo_path.display());

        let mut cmd = Command::new("git");
        cmd.arg("-C").arg(repo_path).arg("pull");

        // Execute the pull command
        let output = cmd.output().context("Failed to execute git pull command")?;

        if !output.status.success() {
            let stderr = String::from_utf8_lossy(&output.stderr);
            bail!("Git pull failed: {}", stderr);
        }

        let stdout = String::from_utf8_lossy(&output.stdout);
        if !stdout.trim().is_empty() {
            println!("{}", stdout.trim());
        }

        println!("Repository updated successfully");

        Ok(())
    }

    pub fn init_submodules(&self, workspace: &Workspace) -> Result<()> {
        println!("Initializing submodules...");

        let mut cmd = Command::new("git");
        cmd.arg("-C")
            .arg(&workspace.path)
            .arg("submodule")
            .arg("update")
            .arg("--init")
            .arg("--recursive");

        // Use status() instead of output() to inherit stdio and show live output
        let status = cmd
            .status()
            .context("Failed to execute git submodule update command")?;

        if !status.success() {
            bail!(
                "Git submodule update failed with exit code: {:?}",
                status.code()
            );
        }

        println!("Successfully initialized submodules");

        Ok(())
    }
}
