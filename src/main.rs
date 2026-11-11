mod build;
mod state;
mod switch;
use anyhow::{Context, Result};
use std::{env, fs, path::PathBuf};

fn main() -> Result<()> {
    // Get the path of the flake
    let flake_path = env::var_os("VIBRANT_DIR")
        .map(PathBuf::from)
        .context("Environment variable 'VIBRANT_DIR' must be set")?;

    // Check if we can find a `flake.nix`
    let flake_file = flake_path.join("flake.nix");
    anyhow::ensure!(
        flake_file.is_file(),
        "Path `{}` does not contain a flake.nix file",
        flake_path.display()
    );

    // Get the host name
    let host = fs::read_to_string("/etc/hostname")
        .context("Failed to read hostname file")?
        .trim()
        .to_string();

    // Get the state folder ready
    let state = state::ensure_state_directory()?;

    // Build the new generation
    let symlink = build::run_nix_build(&flake_path, &state, &host)?;

    // Show the diff of current generation against the new one
    build::show_system_diff(&symlink)?;

    // Activate the generation
    switch::activate_generation(&symlink)?;

    Ok(())
}
