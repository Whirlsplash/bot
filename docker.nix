{ system ? builtins.currentSystem }:

let
  sources = import ./nix/sources.nix;
  pkgs = import sources.nixpkgs { };
  callPackage = pkgs.lib.callPackageWith pkgs;
  bot = callPackage ./default.nix { };

  dockerImage = pkg:
    pkgs.dockerTools.buildImage {
      name = "Whirlsplash/bot";
      tag = "latest";
      created = "now";

      fromImage = pkgs.dockerTools.pullImage {
        imageName = "alpine";
        imageDigest = "sha256:def822f9851ca422481ec6fee59a9966f12b351c62ccb9aca841526ffaa9f748";
        # https://nixos.wiki/wiki/Docker
        #
        # The above article didn't even work for me, ROFL.
        # `nix-build docker.nix` threw an error about the sha256 being
        # incorrect, but it also spat our the expected sha256...
        #
        # so I just replaced it...
        sha256 = "1z6fh6ry14m5cpcjfg88vn2m36garmgdagr4vfc3pm1z3kph639n";
        finalImageTag = "alpine";
        finalImageName = "3.13.5";
      };

      contents = [ pkg ];

      config = {
        WorkingDir = "/";
        Env = [ "DISCORD_TOKEN" ];
        EntryPoint = [ "/bin/bot" ];
      };
    };

in dockerImage bot
