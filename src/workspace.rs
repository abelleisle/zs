use std::path::PathBuf;

use anyhow::{Result, bail};
use serde::{Deserialize, Serialize};

use crate::{config::Config, util::default_true};

#[derive(Serialize, Deserialize, Debug)]
pub struct Workspace {
    pub repo: String,
    pub branch: String,
    pub path: PathBuf,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct WorkspaceSettings {
    #[serde(default = "default_true")]
    pub submodules: bool,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub hook: Option<String>,
}

impl Workspace {
    pub fn delete(&self, config: &Config) -> Result<()> {
        if let Some(repo) = config.repos.get(&self.repo) {
            println!("Deleting worktree...");
            repo.delete_worktree(self)?;
        }

        bail!("Cannot delete workspace, because repo doesn't exists");
    }
}
