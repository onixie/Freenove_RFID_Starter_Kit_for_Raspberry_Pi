with import (builtins.fetchTarball {
  # get an older version of nixos-22.11 to match the glibc version in raspbian
  # ref: https://lazamar.co.uk/nix-versions/?package=glibc&version=2.31&fullName=glibc-2.31&keyName=glibc&revision=3913f6a514fa3eb29e34af744cc97d0b0f93c35c&channel=nixos-22.11#instructions
  url = "https://github.com/NixOS/nixpkgs/archive/3913f6a514fa3eb29e34af744cc97d0b0f93c35c.tar.gz";
}) {
  crossSystem = (import <nixpkgs/lib>).systems.examples.raspberryPi // {
    # override the config to armv7l
    config = "armv7l-unknown-linux-gnueabihf";
  };
};

mkShell {
  nativeBuildInputs = [];
  buildInputs = [];
}
