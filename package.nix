{ rustPlatform, lib }:

rustPlatform.buildRustPackage {
  pname = "vibrant";
  version = "0.1.0";
  cargoLock.lockFile = ./Cargo.lock;
  src = lib.cleanSource ./.;
}
