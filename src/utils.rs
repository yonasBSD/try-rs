use std::path::PathBuf;

// Helper function to replace "~" with the actual home path
pub fn expand_path(path_str: &str) -> PathBuf {
    if (path_str.starts_with("~/") || (cfg!(windows) && path_str.starts_with("~\\")))
        && let Some(home) = dirs::home_dir()
    {
        // Remove "~/" (first 2 chars) and join with home
        return home.join(&path_str[2..]);
    }
    PathBuf::from(path_str)
}

// Checks if the string looks like a Git URL
pub fn is_git_url(s: &str) -> bool {
    s.starts_with("http://")
        || s.starts_with("https://")
        || s.starts_with("git@")
        || s.starts_with("ssh://")
        || s.ends_with(".git")
}

// Extracts a clean repository name (e.g., "github.com/tobi/try.git" -> "try")
pub fn extract_repo_name(url: &str) -> String {
    // Remove trailing slash and .git suffix
    let clean_url = url.trim_end_matches('/').trim_end_matches(".git");

    // Get the last part after the '/' or ':' (common in ssh)
    if let Some(last_part) = clean_url.rsplit(['/', ':']).next()
        && !last_part.is_empty()
    {
        return last_part.to_string();
    }
    // Generic name if detection fails
    "cloned-repo".to_string()
}
