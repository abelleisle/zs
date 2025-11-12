use std::process::Command;

use anyhow::{Context, Result};
use serde::{Deserialize, Serialize};

use crate::session::Session;

#[derive(Serialize, Deserialize, Debug)]
#[serde(rename_all = "lowercase")]
pub enum Multiplexer {
    Zellij,
}

impl Multiplexer {
    pub fn open(&self, session: &Session) -> Result<()> {
        match self {
            Multiplexer::Zellij => self.open_zellij(session),
        }
    }

    pub fn delete(&self, session: &Session) -> Result<()> {
        match self {
            Multiplexer::Zellij => self.delete_zellij(session),
        }
    }

    fn open_zellij(&self, session: &Session) -> Result<()> {
        // Check if we're already inside a Zellij session
        if std::env::var("ZELLIJ").is_ok() {
            anyhow::bail!(
                "Already inside a Zellij session. Cannot nest sessions.\n\
                 Target session path: {}",
                session.path.display()
            );
        }

        // Convert session ID to Zellij-compatible session name
        let session_name = Self::zellij_session_name(&session.id);

        println!("Opening Zellij session: {}", session_name);
        println!("  Path: {}", session.path.display());

        // Build the command to open zellij
        let mut cmd = Command::new("zellij");
        cmd.arg("attach")
            .arg("--create")
            .arg(&session_name)
            .current_dir(&session.path);

        // Execute zellij
        let status = cmd.status().context("Failed to execute zellij command")?;

        if !status.success() {
            anyhow::bail!("Zellij exited with non-zero status");
        }

        Ok(())
    }

    fn delete_zellij(&self, session: &Session) -> Result<()> {
        // Convert session ID to Zellij-compatible session name
        let session_name = Self::zellij_session_name(&session.id);

        println!("Deleting Zellij session: {}", session_name);

        // Build the command to delete zellij session
        let mut cmd = Command::new("zellij");
        cmd.arg("delete-session").arg(&session_name);

        // Execute zellij delete
        let output = cmd
            .output()
            .context("Failed to execute zellij delete-session command")?;

        // It's okay if the session doesn't exist in zellij, just continue
        if !output.status.success() {
            let stderr = String::from_utf8_lossy(&output.stderr);
            println!("Warning: Could not delete Zellij session: {}", stderr);
        } else {
            println!("Deleted Zellij session: {}", session_name);
        }

        Ok(())
    }

    /// Convert a session ID to a Zellij-compatible session name.
    /// Zellij can't have '/' in session names, so we convert them to '.' for the last 2 parts.
    ///
    /// Algorithm from shell script:
    /// 1. Reverse the string
    /// 2. Replace first '/' with '§'
    /// 3. Replace second '/' with '§'
    /// 4. Remove all remaining '/'
    /// 5. Replace all '.' with '_'
    /// 6. Replace '§' with '.'
    /// 7. Reverse the string
    /// 8. Replace '~.' at beginning with '~'
    fn zellij_session_name(session_id: &str) -> String {
        let mut result: String = session_id.chars().rev().collect();

        // Replace first '/' with '§'
        if let Some(pos) = result.find('/') {
            result.replace_range(pos..=pos, "§");
        }

        // Replace second '/' with '§'
        if let Some(pos) = result.find('/') {
            result.replace_range(pos..=pos, "§");
        }

        // Remove all remaining '/'
        result = result.replace('/', "");

        // Replace all '.' with '_'
        result = result.replace('.', "_");

        // Replace '§' with '.'
        result = result.replace('§', ".");

        // Reverse back
        result = result.chars().rev().collect();

        // Replace '~.' at beginning with '~'
        if result.starts_with("~.") {
            result = format!("~{}", &result[2..]);
        }

        result
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_zellij_session_name() {
        let input = "~/p/m/workspaces/feature";
        let result = Multiplexer::zellij_session_name(input);
        assert_eq!(result, "~pm.workspaces.feature");
    }

    #[test]
    fn test_zellij_session_name_short() {
        let input = "~/workspace";
        let result = Multiplexer::zellij_session_name(input);
        assert_eq!(result, "~workspace");
    }

    #[test]
    fn test_zellij_session_name_with_dots() {
        let input = "~/p/m/workspaces/feature.test";
        let result = Multiplexer::zellij_session_name(input);
        assert_eq!(result, "~pm.workspaces.feature_test");
    }
}
