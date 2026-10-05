{ lib, rustPlatform }:

let
  cargoToml = lib.importTOML ./Cargo.toml;
in
rustPlatform.buildRustPackage {
  pname   = cargoToml.package.name;
  version = cargoToml.package.version;

  src = lib.fileset.toSource {
    root    = ./.;
    fileset = lib.fileset.unions [
      ./Cargo.toml
      ./Cargo.lock
      ./src
    ];
  };

  cargoLock.lockFile = ./Cargo.lock;

  meta = {
    description = "Terminal piggy bank tracker";
    homepage    = "https://github.com/TestkaJakub/piggy";
    mainProgram = "piggy";
    platforms   = lib.platforms.all;
  };
}