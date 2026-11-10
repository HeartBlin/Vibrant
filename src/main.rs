use std::io::BufRead;

fn main() {
    // Get the path of the flake
    let flake_path = std::env::var_os("VIBRANT_DIR")
        .map(std::path::PathBuf::from)
        .expect("Environment variable 'VIBRANT_DIR' must be set");

    // Check if we can find a `flake.nix`
    let flake_file = flake_path.join("flake.nix");
    let metadata = std::fs::metadata(&flake_file).expect("Could not access `flake.nix`");

    if !metadata.is_file() {
        panic!(
            "Path `{}` does not contain a flake.nix file",
            flake_path.display()
        );
    }

    // Get the host name
    let host = std::fs::read_to_string("/etc/hostname")
        .expect("Failed to read hostname file")
        .trim()
        .to_string();

    // Get the state folder ready
    let state = ensure_state_directory().expect("Failed to resolve or create state directory");

    // Construct flake reference
    let flake_ref = format!(
        "{}#nixosConfigurations.{}.config.system.build.toplevel",
        flake_path.display(),
        host
    );

    run_nix_build(&flake_ref, &state, &host);
    register_nixos_generation(&state);
    switch_to_configuration(&state);
}

fn resolve_state_directory() -> Option<std::path::PathBuf> {
    // Try XDG_STATE_HOME
    if let Some(xdg_state) = std::env::var_os("XDG_STATE_HOME") {
        let path = std::path::PathBuf::from(xdg_state);
        if path.is_absolute() {
            return Some(path.join("Vibrant"));
        }
    }

    // Try falling back to $HOME/.local/state
    if let Some(home) = std::env::var_os("HOME") {
        let path = std::path::PathBuf::from(home);
        if path.is_absolute() {
            return Some(path.join(".local/state").join("Vibrant"));
        }
    }

    None
}

fn ensure_state_directory() -> std::io::Result<std::path::PathBuf> {
    // Try the path
    let path = resolve_state_directory().ok_or_else(|| {
        std::io::Error::new(
            std::io::ErrorKind::InvalidInput,
            "Could not resolve state directory",
        )
    })?;

    // Ensure the folder is there
    std::fs::create_dir_all(&path)?;

    Ok(path)
}

fn run_nix_build(flake_ref: &str, state: &std::path::Path, host: &str) {
    println!("# Building {}", host);

    let mut command = std::process::Command::new("nix")
        .arg("build")
        .arg(flake_ref)
        .arg("--out-link")
        .arg(state.join("result"))
        .arg("--log-format")
        .arg("bar-with-logs")
        .stderr(std::process::Stdio::piped())
        .stdout(std::process::Stdio::inherit())
        .spawn()
        .expect("Failed to spawn nix process");

    if let Some(stderr) = command.stderr.take() {
        let reader = std::io::BufReader::new(stderr);
        for text in reader.lines().map_while(Result::ok) {
            println!("{}", text);
        }
    }

    let status = command.wait().expect("Failed to wait on child process");
    if !status.success() {
        panic!("Nix build failed with {}", status);
    }

    println!("Nix build completed");
}

fn register_nixos_generation(state: &std::path::Path) {
    std::process::Command::new("sudo")
        .arg("nix-env")
        .arg("--profile")
        .arg("/nix/var/nix/profiles/system")
        .arg("--set")
        .arg(state.join("result"))
        .status()
        .expect("Failed to run `sudo nix-env`");
}

fn switch_to_configuration(state: &std::path::Path) {
    std::process::Command::new("sudo")
        .arg(state.join("result/bin/switch-to-configuration"))
        .arg("switch")
        .status()
        .expect("Failed to switch to new generation");
}
