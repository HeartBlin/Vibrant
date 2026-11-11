{
  description = "A small helper for nix";
  inputs.nixpkgs.url = "https://channels.nixos.org/nixpkgs-unstable/nixexprs.tar.zst";

  outputs = inputs: let
    inherit (inputs.nixpkgs) legacyPackages;
  in {
    nixosModules.default = import ./nix/module.nix;
    packages = {
      "aarch64-linux".vibrant = legacyPackages."aarch64-linux".callPackage ./nix { };
      "x86_64-linux".vibrant = legacyPackages."x86_64-linux".callPackage ./nix { };
    };
  };
}
