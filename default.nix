{ system ? builtins.currentSystem }:

let
  sources = import ./nix/sources.nix;
  pkgs = import sources.nixpkgs { };
  bot = import ./bot.nix { inherit sources pkgs; };

  name = "Whirlsplash/bot";
  tag = "latest";

in pkgs.dockerTools.buildLayeredImage {
  inherit name tag;
  contents = [ bot ];

  config = {
    Cmd = [ "/bin/bot" ];
    Env = [ ];
    WorkingDir = "/";
  };
}
