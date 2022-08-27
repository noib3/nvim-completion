{
  description = "nvim-compleet";

  inputs = {
    nixpkgs.url = "github:NixOS/nixpkgs/nixos-unstable";

    devshell = {
      url = "github:numtide/devshell";
      inputs.nixpkgs.follows = "nixpkgs";
    };

    flake-utils = {
      url = "github:numtide/flake-utils";
      inputs.nixpkgs.follows = "nixpkgs";
    };

    rust-overlay = {
      url = "github:oxalica/rust-overlay";
      inputs.nixpkgs.follows = "nixpkgs";
    };
  };

  outputs = { self, ... }@inputs: with inputs;
    flake-utils.lib.eachDefaultSystem (system:
      let
        pkgs = import nixpkgs {
          inherit system;
          overlays = [
            devshell.overlay
            rust-overlay.overlays.default
          ];
        };
      in
      {
        devShells.default = pkgs.devshell.mkShell {
          name = "nvim-compleet";

          packages = with pkgs; [
            (rust-bin.selectLatestNightlyWith (toolchain: toolchain.default))
            neovim
          ];

          commands = [
            {
              name = "install";
              command = ''"$PRJ_ROOT/install.sh" "$@"'';
            }
          ];
        };
      }
    );
}
