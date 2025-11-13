use std::path::PathBuf;

use anyhow::{Result, bail};
use chrono::Utc;
use inquire::{Confirm, Select, Text};

use crate::{config::Config, session::Session, util::truncate_path};

pub fn open(config: &Config) -> Result<()> {
    let mut sessions = Session::load_all()?;

    if sessions.is_empty() {
        bail!(
            "No sessions found. Create a session first using 'zs session new <path>' or 'zs workspace'."
        );
    }

    // Create a list of session choices
    let session_list: Vec<(&String, &Session)> = sessions.iter().collect();
    let choices: Vec<String> = session_list
        .iter()
        .map(|(_, session)| {
            let name = session.name.as_deref().unwrap_or("Unnamed");
            let desc = session.description.as_deref().unwrap_or("");
            if desc.is_empty() {
                format!("{} - {}", session.id, name)
            } else {
                format!("{} - {} ({})", session.id, name, desc)
            }
        })
        .collect();

    // Prompt user to select a session
    let selection = Select::new("Select a session:", choices).prompt()?;

    // Find the selected session key by matching the choice string
    let selected_key = session_list
        .iter()
        .enumerate()
        .find(|(i, _)| {
            let name = session_list[*i].1.name.as_deref().unwrap_or("Unnamed");
            let desc = session_list[*i].1.description.as_deref().unwrap_or("");
            let choice = if desc.is_empty() {
                format!("{} - {}", session_list[*i].1.id, name)
            } else {
                format!("{} - {} ({})", session_list[*i].1.id, name, desc)
            };
            choice == selection
        })
        .map(|(_, (key, _))| (*key).clone())
        .expect("Selected session should exist");

    // Update last_opened timestamp
    if let Some(session) = sessions.get_mut(&selected_key) {
        session.last_opened = Some(Utc::now());
    }

    // Save updated sessions
    Session::save_all(&sessions)?;

    // Get the session to open (immutable reference after saving)
    let selected_session = sessions
        .get(&selected_key)
        .expect("Selected session should exist");

    // Open the session using the configured multiplexer
    config.multiplexer.open(selected_session)?;

    Ok(())
}

pub fn new(config: &Config, path: PathBuf) -> Result<()> {
    // Ensure the path exists
    if !path.exists() {
        bail!("Path does not exist: {}", path.display());
    }

    if !path.is_dir() {
        bail!("Path is not a directory: {}", path.display());
    }

    // Get absolute path
    let abs_path = path.canonicalize()?;

    // Generate session ID from truncated path
    let session_id = truncate_path(&abs_path);

    // Check if session already exists
    let mut sessions = Session::load_all()?;
    if sessions.contains_key(&session_id) {
        bail!("Session already exists with ID: {}", session_id);
    }

    // Prompt for session name
    let session_name = Text::new("Session name:")
        .with_default(&session_id)
        .prompt()?;

    // Prompt for session description
    let session_description = Text::new("Description (optional):")
        .with_default("")
        .prompt()?;

    let description = if session_description.is_empty() {
        None
    } else {
        Some(session_description)
    };

    // Create session with initial timestamp
    let session = Session {
        id: session_id.clone(),
        path: abs_path,
        name: Some(session_name),
        description,
        workspace: None,
        last_opened: Some(Utc::now()),
    };

    // Save session
    sessions.insert(session_id.clone(), session);
    Session::save_all(&sessions)?;

    println!("Session '{}' created successfully!", session_id);

    // Open the newly created session
    let created_session = sessions.get(&session_id).expect("Session should exist");
    config.multiplexer.open(created_session)?;

    Ok(())
}

pub fn remove(config: &Config) -> Result<()> {
    let mut sessions = Session::load_all()?;

    if sessions.is_empty() {
        bail!("No sessions found.");
    }

    // Create a list of session choices
    let session_list: Vec<(&String, &Session)> = sessions.iter().collect();
    let choices: Vec<String> = session_list
        .iter()
        .map(|(_, session)| {
            let name = session.name.as_deref().unwrap_or("Unnamed");
            let desc = session.description.as_deref().unwrap_or("");
            if desc.is_empty() {
                format!("{} - {}", session.id, name)
            } else {
                format!("{} - {} ({})", session.id, name, desc)
            }
        })
        .collect();

    // Prompt user to select a session
    let selection = Select::new("Select a session to remove:", choices).prompt()?;

    // Find the selected session key by matching the choice string
    let selected_key = session_list
        .iter()
        .enumerate()
        .find(|(i, _)| {
            let name = session_list[*i].1.name.as_deref().unwrap_or("Unnamed");
            let desc = session_list[*i].1.description.as_deref().unwrap_or("");
            let choice = if desc.is_empty() {
                format!("{} - {}", session_list[*i].1.id, name)
            } else {
                format!("{} - {} ({})", session_list[*i].1.id, name, desc)
            };
            choice == selection
        })
        .map(|(_, (key, _))| (*key).clone())
        .expect("Selected session should exist");

    // Confirm deletion
    let confirm = Confirm::new(&format!(
        "Are you sure you want to delete session '{}'?",
        selected_key
    ))
    .with_default(false)
    .prompt()?;

    if !confirm {
        println!("Deletion cancelled.");
        return Ok(());
    }

    // Get the session to delete
    let session_to_delete = sessions
        .get(&selected_key)
        .expect("Selected session should exist");

    // Delete worktree if this session belongs to a repo
    if let Some(workspace) = &session_to_delete.workspace {
        workspace.delete(config)?;
    }

    // Delete session from multiplexer
    config.multiplexer.delete(session_to_delete)?;

    // Remove from session list
    sessions.remove(&selected_key);
    Session::save_all(&sessions)?;

    println!("Session '{}' deleted successfully!", selected_key);

    Ok(())
}
