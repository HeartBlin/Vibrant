use anyhow::{Context, Result};
use std::path::Path;
use std::process::Command;

/// Registers and activates a NixOS generation.
///
/// Invokes the external `nix-env` and `switch-to-configuration`, the latter coming
/// from `symlink`, in two steps. The `nix-env` step assures that the profile, or generation,
/// is properly registered with NixOS, as to allow booting into it. The `switch-to-configuration`,
/// step commits the actual activation of the generation (either an implicit `switch`, or a `boot`
/// if the flag `-b/--boot` is provided)
///
/// # Errors
///
/// This function will return an error in the following situations:
/// * `symlink` does not exist or does not resolve to a directory.
/// * The `nix-env` binary cannot be found in `PATH` or fails to spawn.
/// * The `nix-env` command fails to register the generation.
/// * The `switch-to-configuration` binary cannot be found in `PATH` or fails to spawn.
/// * The `switch-to-configuration` fails to activate the new generation
pub fn activate_generation(symlink: &Path, boot: bool) -> Result<()> {
    // TOCTOU
    anyhow::ensure!(symlink.is_dir(), "Result directory is missing",);

    // Generation registration
    let env_status = Command::new("sudo")
        .arg("nix-env")
        .arg("--profile")
        .arg("/nix/var/nix/profiles/system")
        .arg("--set")
        .arg(symlink)
        .status()
        .context("Failed to run `sudo nix-env`")?;

    anyhow::ensure!(
        env_status.success(),
        "'nix-env' failed to register the generation: {}",
        env_status
    );

    // Determine if we want `switch` or `boot`
    let action = if boot { "boot" } else { "switch" };

    // Activate the new configuration
    let switch_status = Command::new("sudo")
        .arg(symlink.join("bin/switch-to-configuration"))
        .arg(action)
        .status()
        .with_context(|| format!("Failed to spawn activation script ({})", action))?;

    anyhow::ensure!(
        switch_status.success(),
        "Failed to `{}` to the new generation: {}",
        action,
        switch_status
    );

    Ok(())
}
