use std::path::Path;

/// Truncates a path by shortening all directory names except the last 2 to their first character.
/// Also replaces the home directory with ~.
///
/// Examples:
/// - `/home/user/projects/myrepo/src/main.rs` → `~/p/m/src/main.rs`
/// - `~/documents/work` → `~/documents/work` (2 or fewer parts, no truncation)
pub fn truncate_path(path: &Path) -> String {
    let path_str = path.to_string_lossy();

    // Replace home directory with ~
    let path_str = if let Some(home) = dirs::home_dir() {
        let home_str = home.to_string_lossy();
        if path_str.starts_with(home_str.as_ref()) {
            format!("~{}", &path_str[home_str.len()..])
        } else {
            path_str.to_string()
        }
    } else {
        path_str.to_string()
    };

    // Split path by / and filter out empty parts
    let parts: Vec<&str> = path_str.split('/').filter(|p| !p.is_empty()).collect();

    let num_parts = parts.len();

    // If 2 or fewer parts, return original
    if num_parts <= 2 {
        return path_str;
    }

    // Build result: truncate all but last 2 parts
    let mut result = String::new();

    for (i, part) in parts.iter().enumerate() {
        if i == 0 {
            // First part - keep full
            result.push_str(part);
        } else if i < num_parts - 2 {
            // Middle parts - truncate to first character
            result.push('/');
            if let Some(first_char) = part.chars().next() {
                result.push(first_char);
            }
        } else {
            // Last 2 parts - keep full
            result.push('/');
            result.push_str(part);
        }
    }

    result
}

pub fn default_true() -> bool {
    true
}

#[cfg(test)]
mod tests {
    use std::path::PathBuf;

    use super::*;

    #[test]
    fn test_truncate_short_path() {
        let path = PathBuf::from("~/documents");
        let result = truncate_path(&path);
        assert_eq!(result, "~/documents");
    }

    #[test]
    fn test_truncate_long_path() {
        let path = PathBuf::from("~/documents/work/projects/myrepo/src");
        let result = truncate_path(&path);
        assert_eq!(result, "~/d/w/p/myrepo/src");
    }

    #[test]
    fn test_truncate_home_path() {
        let path_str = if let Some(home) = dirs::home_dir() {
            format!(
                "{}/documents/work/projects/myrepo/src",
                home.to_string_lossy()
            )
        } else {
            return;
        };
        let path = PathBuf::from(path_str);
        let result = truncate_path(&path);
        assert_eq!(result, "~/d/w/p/myrepo/src");
    }
}
