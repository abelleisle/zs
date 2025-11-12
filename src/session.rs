use std::{collections::HashMap, path::PathBuf};

use anyhow::{Context, Result};
use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Debug)]
pub struct Session {
    pub id: String,
    pub path: PathBuf,
    pub name: Option<String>,
    pub description: Option<String>,
    pub repo: Option<String>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub last_opened: Option<DateTime<Utc>>,
}

pub type SessionMap = HashMap<String, Session>;

impl Session {
    pub fn load_all() -> Result<SessionMap> {
        let sessions_path = Self::sessions_path()?;

        if !sessions_path.exists() {
            return Ok(HashMap::new());
        }

        let contents = std::fs::read_to_string(&sessions_path).context(format!(
            "Failed to read sessions file at {}",
            sessions_path.display()
        ))?;

        let sessions: SessionMap =
            serde_json::from_str(&contents).context("Failed to parse sessions file")?;

        Ok(sessions)
    }

    pub fn save_all(sessions: &SessionMap) -> Result<()> {
        let sessions_path = Self::sessions_path()?;

        // Ensure the parent directory exists
        if let Some(parent) = sessions_path.parent() {
            std::fs::create_dir_all(parent).context("Failed to create sessions directory")?;
        }

        let contents =
            serde_json::to_string_pretty(sessions).context("Failed to serialize sessions")?;

        std::fs::write(&sessions_path, contents).context(format!(
            "Failed to write sessions file at {}",
            sessions_path.display()
        ))?;

        Ok(())
    }

    pub fn sessions_path() -> Result<PathBuf> {
        let cache_dir = std::env::var("XDG_CACHE_HOME")
            .ok()
            .map(PathBuf::from)
            .or_else(|| dirs::home_dir().map(|home| home.join(".cache")))
            .context("Failed to determine cache directory")?;

        Ok(cache_dir.join("zs").join("sessions.json"))
    }
}
