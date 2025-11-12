use anyhow::Result;

use crate::{config::Config, session::Session};

pub fn run(_config: &Config) -> Result<()> {
    let sessions = Session::load_all()?;

    println!(
        "Loaded {} sessions from: {}",
        sessions.len(),
        Session::sessions_path()?.display()
    );

    for (key, session) in &sessions {
        println!("  [{}] {} - {}", key, session.id, session.path.display());
    }

    Ok(())
}
