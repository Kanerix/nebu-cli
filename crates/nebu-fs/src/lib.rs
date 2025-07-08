use std::path::{Path, PathBuf};

/// Takes a path and expands the home directory if it starts with `~` or `~/`.
pub fn expand_home_dir<P: AsRef<Path>>(path: P) -> Option<PathBuf> {
    let path = path.as_ref();
    
    if let Some(path_str) = path.to_str() {
        if path_str.starts_with("~/") {
            return home::home_dir().map(|home| home.join(&path_str[2..]));
        } else if path_str == "~" {
            return home::home_dir();
        }
    }
    
    Some(path.to_path_buf())
}
