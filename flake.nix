{
  description = "A small helper for nix";
  inputs.nixpkgs.url = "https://channels.nixos.org/nixpkgs-unstable/nixexprs.tar.zst";

  outputs = inputs: let
    systems = [ "aarch64-linux" "x86_64-linux" ];
    forAllSystems = f: inputs.nixpkgs.lib.genAttrs systems (
      system: f inputs.nixpkgs.legacyPackages.${system}
    );
  in {
    nixosModules.default = import ./nix/module.nix;
    packages = forAllSystems (pkgs: rec {
      vibrant = pkgs.callPackage ./nix { };
      default = vibrant;
    });
  };
}
