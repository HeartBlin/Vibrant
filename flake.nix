{
  description = "A small helper for nix";
  inputs.nixpkgs.url = "https://channels.nixos.org/nixpkgs-unstable/nixexprs.tar.zst";
  outputs = inputs: let
    forAllSystems = inputs.nixpkgs.lib.genAttrs [
      "x86_64-linux"
      "aarch64-linux"
    ];
  in {
    packages = forAllSystems (system: let
      pkgs = inputs.nixpkgs.legacyPackages.${system};
    in rec {
      default = vibrant;
      vibrant = pkgs.callPackage ./nix/package.nix { };
    });

    nixosModules = rec {
      default = vibrant;
      vibrant = import ./nix/module.nix;
    };
  };
}
