use std::path::PathBuf;

use anyhow::{Result, bail};
use serde::{Deserialize, Serialize};

use crate::config::Config;

#[derive(Serialize, Deserialize, Debug)]
pub struct Workspace {
    pub repo: String,
    pub branch: String,
    pub path: PathBuf,
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
