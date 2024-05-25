{
  description = "The backend core that runs amethyst";

  inputs = {
    nixpkgs.url = "github:NixOS/nixpkgs/nixos-unstable";
    devenv.url = "github:cachix/devenv";
    nix2container.url = "github:nlewo/nix2container";
    nix2container.inputs.nixpkgs.follows = "nixpkgs";
    mk-shell-bin.url = "github:rrbutani/nix-mk-shell-bin";
    fenix = {
      url = "github:nix-community/fenix";
      inputs.nixpkgs.follows = "nixpkgs";
    };
  };

  nixConfig = {
    extra-trusted-public-keys = "devenv.cachix.org-1:w1cLUi8dv3hnoSPGAuibQv+f9TZLr6cv/Hm9XgU50cw=";
    extra-substituters = "https://devenv.cachix.org";
  };

  outputs =
    inputs@{ flake-parts, nixpkgs, ... }:
    flake-parts.lib.mkFlake { inherit inputs; } {
      imports = [ inputs.devenv.flakeModule ];
      systems = [
        "x86_64-linux"
        "i686-linux"
        "x86_64-darwin"
        "aarch64-linux"
        "aarch64-darwin"
      ];

      perSystem =
        {
          config,
          self',
          inputs',
          lib,
          pkgs,
          system,
          ...
        }:
        {
          devenv.shells.default = {
            name = "Amethyst-core/core";
            difftastic.enable = true;
            imports = [ ];
            packages =
              lib.optionals pkgs.stdenv.isDarwin (
                with pkgs.darwin.apple_sdk.frameworks;
                [
                  Security
                  SystemConfiguration
                ]
              )
              ++ ([ pkgs.cargo-watch ]);

            env = {

            };

            languages.rust = {
              enable = true;
              channel = "stable";
              components = [
                "rustc"
                "cargo"
                "clippy"
                "rustfmt"
                "rust-analyzer"
              ];
            };

            languages.javascript = {
              enable = true;
              pnpm = {
                enable = true;
              };
            };

            languages.typescript.enable = true;

            pre-commit.hooks = {
              nixfmt.package = pkgs.nixfmt-rfc-style;
              nixfmt.enable = true;
              clippy.enable = true;
              clippy.files = "/backend/*";
            };

            dotenv.enable = true;
          };
        };
      flake = { };
    };
}
