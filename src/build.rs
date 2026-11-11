use anyhow::{Context, Result};
use std::path::{Path, PathBuf};
use std::process::{Command, Stdio};

/// Runs a nix build.
///
/// Invokes the external `nix` command to build a new generation of the flake and host
/// specified by `flake_path` and `host`. If `nix-output-monitor`, here `nom`, exits with
/// a non-zero exit code, a warning is printed to `stderr`, and the function continues.
///
/// # Errors
///
/// This function will return an error in the following situations:
/// * `state` does not exist or does not resolve to a directory.
/// * The `nix` binary cannot be found in `PATH` or fails to spawn.
/// * The `stderr` of `nix` cannot be captured.
/// * The `nom` binary cannot be found in `PATH` or fails to spawn.
/// * Failure to wait for either `nix` or `nom`.
/// * If the nix build command returns a non-zero exit code
/// * `store_path` cannot be canonicalized
pub fn run_nix_build(flake_path: &Path, state: &Path, host: &str) -> Result<PathBuf> {
    println!("# Building '{}'", host);

    // TOCTOU
    anyhow::ensure!(state.is_dir(), "State directory is missing",);

    // Get the folder to leave the build in
    let out_link = state.join("result");

    // Build the command
    let command = format!(
        "{}#nixosConfigurations.{}.config.system.build.toplevel",
        flake_path.display(),
        host
    );

    // Set up the nix build
    let mut nix_child = Command::new("nix")
        .arg("build")
        .arg(command)
        .arg("--out-link")
        .arg(&out_link)
        .arg("--log-format")
        .arg("internal-json")
        .stderr(Stdio::piped())
        .stdout(Stdio::null())
        .spawn()
        .context("Failed to spawn 'nix` process")?;

    // For reasons, nix build outputs to stderr
    let nix_stderr = nix_child
        .stderr
        .take()
        .context("Failed to capture nix stderr")?;

    // Set up nix-output-monitor
    let mut nom_child = Command::new("nom")
        .arg("--json")
        .stdin(nix_stderr)
        .stdout(Stdio::inherit())
        .stderr(Stdio::inherit())
        .spawn()
        .context("Failed to spawn 'nix-output-monitor' process")?;

    // Did they stop?
    let nix_status = nix_child.wait().context("Failed to wait on nix process")?;
    let nom_status = nom_child.wait().context("Failed to wait on nom process")?;

    // Did they fail?
    anyhow::ensure!(
        nix_status.success(),
        "'nix' failed with status: {}",
        nix_status
    );

    if !nom_status.success() {
        eprintln!("Warning: 'nix-output-monitor' exited with status: {nom_status}");
    }

    // Canonicalize it
    let store_path = out_link
        .canonicalize()
        .context("Failed to resolve result symlink after build")?;

    Ok(store_path)
}

/// Displays a diff between the current system and a newly built one.
///
/// Invokes the external `nvd` command to compare `/run/current-system` against
/// the store path referenced by `symlink`. If `nvd` exits with a non-zero status
/// code, a warning is printed to `stderr`, and `Ok(())` is still returned.
///
/// # Errors
///
/// This function will return an error in the following situations:
/// * `symlink` does not exist or does not resolve to a directory.
/// * The `nvd` binary cannot be found in `PATH` or fails to spawn.
pub fn show_system_diff(symlink: &Path) -> Result<()> {
    // TOCTOU
    anyhow::ensure!(symlink.is_dir(), "Result directory is missing",);

    // Set up nvd
    let status = Command::new("nvd")
        .arg("diff")
        .arg("/run/current-system")
        .arg(symlink)
        .status()
        .context("Failed to execute 'nvd'")?;

    if !status.success() {
        eprintln!("Warning: nvd exited with status {}", status);
    }

    Ok(())
}
