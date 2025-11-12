use anyhow::{Result, bail};
use chrono::Utc;
use inquire::{Select, Text};

use crate::{config::Config, session::Session, util::truncate_path};

pub fn run(config: &Config) -> Result<()> {
    // Check if there are any repos defined
    if config.repos.is_empty() {
        bail!("No repos defined in config. Please add repos to ~/.config/zs/zs.toml");
    }

    // Get list of repo names for selection
    let repo_names: Vec<&String> = config.repos.keys().collect();

    // Prompt user to select a repo
    let selected_repo_name = Select::new("Select a workspace:", repo_names).prompt()?;

    let selected_repo = config
        .repos
        .get(selected_repo_name)
        .expect("Selected repo should exist");

    println!(
        "Selected workspace: {} at {}",
        selected_repo_name,
        selected_repo.path.display()
    );

    // Ensure the repo path exists
    selected_repo.ensure_path_exists()?;
    println!("Ensured path exists at: {}", selected_repo.path.display());

    // Clone the repo if it doesn't exist
    selected_repo.ensure_cloned()?;

    // Update the repo to get the latest changes
    selected_repo.update()?;

    // Prompt for workspace name
    let workspace_name = Text::new("Workspace name:").prompt()?;

    // Prompt for branch name
    let branch_name = Text::new("Branch name:").prompt()?;

    // Prompt for workspace description
    let workspace_description = Text::new("Description (optional):")
        .with_default("")
        .prompt()?;

    let description = if workspace_description.is_empty() {
        None
    } else {
        Some(workspace_description)
    };

    // Create workspace path: <repo.path>/workspaces/<name>
    let workspace_path = selected_repo.path.join("workspaces").join(&workspace_name);

    // Create the worktree
    selected_repo.create_worktree(&branch_name, &workspace_path)?;
    println!("Created worktree at: {}", workspace_path.display());

    // Execute workspace hook if defined
    selected_repo.execute_workspace_hook(&workspace_path)?;

    // Generate session ID from truncated path
    let session_id = truncate_path(&workspace_path);

    // Create session with initial timestamp
    let session = Session {
        id: session_id.clone(),
        path: workspace_path,
        name: Some(workspace_name.clone()),
        description,
        repo: Some(selected_repo_name.to_string()),
        last_opened: Some(Utc::now()),
    };

    // Load existing sessions, add new one, and save
    let mut sessions = Session::load_all()?;
    sessions.insert(session_id.clone(), session);
    Session::save_all(&sessions)?;

    println!("Session '{}' created successfully!", session_id);

    // Open the newly created session
    let created_session = sessions.get(&session_id).expect("Session should exist");
    config.multiplexer.open(created_session)?;

    Ok(())
}
