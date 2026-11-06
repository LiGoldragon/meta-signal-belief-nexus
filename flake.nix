{
  description = "Belief Nexus meta signal contract";
  inputs = {
    nixpkgs.url = "github:NixOS/nixpkgs/nixpkgs-unstable";
    flake-utils.url = "github:numtide/flake-utils";
    rust-build = { url = "github:LiGoldragon/rust-build"; inputs.nixpkgs.follows = "nixpkgs"; };
    ethos-zero.url = "github:LiGoldragon/ethos-zero";
  };
  outputs = { self, nixpkgs, flake-utils, rust-build, ethos-zero }:
    flake-utils.lib.eachDefaultSystem (system:
      let
        pkgs = import nixpkgs { inherit system; };
        rust = rust-build.lib.${system}.fromPkgs pkgs;
        source = rust.cleanSource { root = ./.; extraFilters = [ (path: type: type == "regular" && pkgs.lib.hasSuffix ".ethos" (toString path)) ]; };
        cargoArtifacts = rust.craneLib.buildDepsOnly { src = source; cargoExtraArgs = "--all-features"; };
      in {
        packages.default = rust.craneLib.buildPackage { pname = "meta-signal-belief-nexus"; version = "0.3.0"; src = source; inherit cargoArtifacts; cargoExtraArgs = "--all-features"; };
        checks = {
          test = rust.craneLib.cargoTest { pname = "meta-signal-belief-nexus-test"; version = "0.3.0"; src = source; inherit cargoArtifacts; cargoTestExtraArgs = "--all-features"; };
          test-default = rust.craneLib.cargoTest { pname = "meta-signal-belief-nexus-test-default"; version = "0.3.0"; src = source; inherit cargoArtifacts; };
          test-datom = rust.craneLib.cargoTest { pname = "meta-signal-belief-nexus-test-datom"; version = "0.3.0"; src = source; inherit cargoArtifacts; cargoTestExtraArgs = "--features datom --test behavior"; };
          generated = pkgs.runCommand "meta-signal-belief-nexus-generated" { generator = ethos-zero.packages.${system}.default; declaration = ./signal.ethos; committed = ./src/lib.rs; } ''
            temporary="$(${pkgs.coreutils}/bin/mktemp -d)"
            "$generator/bin/ethos-zero" "Generate.{ «$declaration» «$temporary» }"
            generated="$(${pkgs.findutils}/bin/find "$temporary" -maxdepth 1 -name '*.rs' ! -name 'committed-generated.rs' -print -quit)"
            ${pkgs.coreutils}/bin/head -c "$(${pkgs.coreutils}/bin/stat -c %s "$generated")" "$committed" > "$temporary/committed-generated.rs"
            ${pkgs.diffutils}/bin/cmp "$generated" "$temporary/committed-generated.rs"
            touch "$out"
          '';
        };
        devShells.default = pkgs.mkShell { packages = [ rust.toolchain ]; };
      });
}
