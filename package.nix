{ lib, rustPlatform, makeWrapper, nix-output-monitor }:

rustPlatform.buildRustPackage {
  pname = "vibrant";
  version = "0.1.1";
  cargoLock.lockFile = ./Cargo.lock;
  src = lib.cleanSource ./.;

  nativeBuildInputs = [ makeWrapper ];
  postInstall = ''
    wrapProgram $out/bin/vibrant \
      --prefix PATH : ${lib.makeBinPath [ nix-output-monitor ]}
  '';
}
