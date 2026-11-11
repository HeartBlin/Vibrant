use anyhow::{Context, Result};
use std::{env, fs, path::PathBuf};

/// Ensure a state directory exists.
///
/// Checks `XDG_STATE_HOME`. If unset, pivots to checking `HOME`, in the directory
/// `.local/state`. If one of the ENVVARs is valid, creates a state directory at that path
/// and returns it.
///
/// # Errors
///
/// This function will return an error in the following situations:
/// * Both `XDG_STATE_HOME` and `HOME` cannot be found or used
/// * The state directory cannot be created at the choosen location
pub fn ensure_state_directory() -> Result<PathBuf> {
    let mut state_dir = None;

    // Try XDG_STATE_HOME
    if let Some(xdg_state) = env::var_os("XDG_STATE_HOME") {
        let path = PathBuf::from(xdg_state);
        if path.is_absolute() {
            state_dir = Some(path.join("Vibrant"));
        }
    }

    // Try falling back to $HOME/.local/state
    if state_dir.is_none()
        && let Some(home) = env::var_os("HOME")
    {
        let path = PathBuf::from(home);
        if path.is_absolute() {
            state_dir = Some(path.join(".local/state").join("Vibrant"));
        }
    }

    // Get the resolved path
    let path = state_dir
        .context("Could not resolve state directory from either '$XDG_STATE_HOME' or '$HOME'")?;

    // Ensure the folder exists
    fs::create_dir_all(&path)
        .with_context(|| format!("Failed to create state directory at `{}`", path.display()))?;

    Ok(path)
}
