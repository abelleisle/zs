use std::{collections::HashMap, path::PathBuf};

use anyhow::{Context, Result};
use serde::{Deserialize, Serialize};

use crate::{multiplexer::Multiplexer, repo::Repo};

#[derive(Serialize, Deserialize, Debug)]
pub struct Config {
    pub multiplexer: Multiplexer,
    #[serde(default)]
    pub repos: HashMap<String, Repo>,
}

impl Config {
    pub fn load() -> Result<Self> {
        let config_path = Self::config_path()?;

        if !config_path.exists() {
            return Ok(Config {
                multiplexer: Multiplexer::Zellij,
                repos: HashMap::new(),
            });
        }

        let contents = std::fs::read_to_string(&config_path).context(format!(
            "Failed to read config file at {}",
            config_path.display()
        ))?;

        let mut config: Config =
            toml::from_str(&contents).context("Failed to parse config file")?;

        // Expand all repo paths
        for repo in config.repos.values_mut() {
            repo.expand_path()?;
        }

        Ok(config)
    }

    pub fn config_path() -> Result<PathBuf> {
        let config_dir = std::env::var("XDG_CONFIG_HOME")
            .ok()
            .map(PathBuf::from)
            .or_else(|| dirs::home_dir().map(|home| home.join(".config")))
            .context("Failed to determine config directory")?;

        Ok(config_dir.join("zs").join("zs.toml"))
    }
}
