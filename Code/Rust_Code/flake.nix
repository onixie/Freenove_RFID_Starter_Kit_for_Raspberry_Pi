{
  description = "flake for cross compile to armv7l using gcc + glibc 2.31";

  inputs = {
    # get an older version of nixos-22.11 to match the glibc version in raspbian
    # ref: https://lazamar.co.uk/nix-versions/?package=glibc&version=2.31&fullName=glibc-2.31&keyName=glibc&revision=3913f6a514fa3eb29e34af744cc97d0b0f93c35c&channel=nixos-22.11#instructions
    nixpkgs.url = "github:NixOS/nixpkgs?rev=3913f6a514fa3eb29e34af744cc97d0b0f93c35c";
    flake-utils.url = "github:numtide/flake-utils";
  };

  outputs = { self, nixpkgs, flake-utils }:
  flake-utils.lib.eachSystem ["x86_64-linux"] (system:
  let
    pkgs = import nixpkgs {
      crossSystem = {
        config = "armv7l-unknown-linux-gnueabihf";
      };
    };
  in {
    devShell = pkgs.mkShell {
      nativeBuildInputs = with pkgs; [
      ];
      buildInputs = with pkgs; [
      ];
    };
  });
}
