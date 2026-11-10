{ config, lib, pkgs, self, ... }:

let
  cfg = config.programs.vibrant;
in {
  options.programs.vibrant = {
    enable = lib.mkEnableOption "Enable `vibrant`";
    flake = lib.mkOption {
      type = lib.types.nullOr lib.types.str;
      default = null;
      description = "The folder that contains the `flake.nix` file";
    };
  };

  config = {
    environment = {
      sessionVariables.VIBRANT_DIR = lib.mkIf (cfg.flake != null) cfg.flake;
      systemPackages = lib.mkIf cfg.enable [
        self.packages.${pkgs.stdenv.hostPlatform.system}.vibrant
      ];
    };
  };
}
