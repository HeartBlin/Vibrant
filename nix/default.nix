{ lib, rustPlatform, makeWrapper, nix-output-monitor, nvd }:

rustPlatform.buildRustPackage {
  pname = "vibrant";
  version = "1.1.0";
  cargoLock.lockFile = ../Cargo.lock;
  src = lib.cleanSource ../.;

  nativeBuildInputs = [ makeWrapper ];
  postInstall = ''
    wrapProgram $out/bin/vibrant \
      --prefix PATH : ${lib.makeBinPath [ nix-output-monitor nvd ]}
  '';
}
