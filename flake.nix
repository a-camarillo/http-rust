{
  description = "Building containers with Rust";

  inputs = {
    nixpkgs.url = "github:nixos/nixpkgs/nixos-unstable";
    flake-parts.url = "github:hercules-ci/flake-parts";
  };

  outputs = inputs@{ self, nixpkgs, flake-parts, ... }:
    flake-parts.lib.mkFlake { inherit inputs; } {
      systems = [ "x86_64-linux" ];
      perSystem = { config, self', inputs', pkgs, system, ... }: 
      {
        devShells = {
          default = pkgs.mkShell {
            packages = [ 
              pkgs.rustc
              pkgs.cargo
            ];
            nativeBuildInputs = with pkgs; [
              pkgs.pkg-config
              pkgs.openssl
            ];
          };
        };
      };
  };
}
