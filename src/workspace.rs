use std::path::PathBuf;

use anyhow::Result;
use serde::{Deserialize, Serialize};

use crate::{config::Config, repo::Repo, util::default_true};

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct Workspace {
    pub repo_name: String,

    #[serde(skip)]
    pub repo: Option<Repo>,

    pub branch: String,
    pub path: PathBuf,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct WorkspaceSettings {
    #[serde(default = "default_true")]
    pub submodules: bool,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub hook: Option<String>,
}

impl Workspace {
    /// Hydrate the workspace with the full Repo object from Config
    pub fn hydrate(&mut self, config: &Config) -> Result<()> {
        let repo = config
            .repos
            .get(&self.repo_name)
            .ok_or_else(|| anyhow::anyhow!("Repo '{}' not found in config", self.repo_name))?;
        self.repo = Some(repo.clone());
        Ok(())
    }

    /// Get a reference to the repo, returning an error if not hydrated
    pub fn get_repo(&self) -> Result<&Repo> {
        self.repo
            .as_ref()
            .ok_or_else(|| anyhow::anyhow!("Workspace not hydrated with repo"))
    }

    pub fn delete(&self) -> Result<()> {
        println!("Deleting worktree...");
        let repo = self.get_repo()?;
        repo.delete_worktree(self)?;
        Ok(())
    }

    pub fn setup(&self) -> Result<()> {
        let selected_repo = self.get_repo()?;

        // Initialize submodules if enabled in workspace settings
        if let Some(workspace_settings) = &selected_repo.workspace
            && workspace_settings.submodules
        {
            selected_repo.init_submodules(self)?;
        }

        // Setup direnv if configured
        if let Some(direnv) = &selected_repo.direnv {
            direnv.create(self)?;
            direnv.trust(self)?;
        }

        // Execute workspace hook if defined
        match selected_repo.execute_workspace_hook(self) {
            Ok(_) => {}
            Err(e) => println!("Failed to execute workspace hook: {}", e),
        }

        Ok(())
    }
}
