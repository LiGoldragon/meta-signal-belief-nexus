{
  description = "meta-signal-belief-nexus — Belief Nexus meta signal contract";

  inputs = {
    base.url = "github:LiGoldragon/build-base";
    ethos-zero = {
      url = "github:LiGoldragon/ethos-zero";
      inputs.base.follows = "base";
    };
  };

  # Cargo.nix is generated from Cargo.lock by the build base's crate2nix and
  # committed, so evaluation imports no derivation:
  #   nix shell github:LiGoldragon/build-base#toolchain github:LiGoldragon/build-base#crate2nix -c crate2nix generate
  outputs =
    { self, base, ethos-zero }:
    base.lib.workspaceOutputs {
      cargoNix = ./Cargo.nix;

      # Every test runs with every feature on.
      memberChecks = { check, ... }: { meta-signal-belief-nexus = check "meta-signal-belief-nexus" [ "default" "datom" ]; };

      extraChecks =
        { pkgs, toolchain, check, doc, clippy, ... }:
        let
          manifest = builtins.fromTOML (builtins.readFile ./Cargo.toml);
          rust = pkgs.lib.fileset.toSource {
            root = ./.;
            fileset = pkgs.lib.fileset.unions [ ./src ./tests ];
          };
        in
        {
          # The tests under the default feature set alone.
          test-default = check "meta-signal-belief-nexus" [ "default" ];
          generated = pkgs.runCommand "meta-signal-belief-nexus-generated" { generator = ethos-zero.packages.${pkgs.stdenv.hostPlatform.system}.default; declaration = ./signal.ethos; committed = ./src/lib.rs; } ''
            temporary="$(${pkgs.coreutils}/bin/mktemp -d)"
            "$generator/bin/ethos-zero" "Generate.{ «$declaration» «$temporary» }"
            generated="$(${pkgs.findutils}/bin/find "$temporary" -maxdepth 1 -name '*.rs' ! -name 'committed-generated.rs' -print -quit)"
            ${pkgs.coreutils}/bin/head -c "$(${pkgs.coreutils}/bin/stat -c %s "$generated")" "$committed" > "$temporary/committed-generated.rs"
            ${pkgs.diffutils}/bin/cmp "$generated" "$temporary/committed-generated.rs"
            touch "$out"
          '';
          fmt = pkgs.runCommand "meta-signal-belief-nexus-fmt" { nativeBuildInputs = [ toolchain ]; } ''
            rustfmt --check --edition ${manifest.package.edition} ${rust}/src/lib.rs ${rust}/tests/*.rs
            touch $out
          '';
          clippy = clippy "meta-signal-belief-nexus" [ "default" "datom" ] [ "-D" "warnings" ];
          doc = doc "meta-signal-belief-nexus" [ "default" "datom" ] [ "-D" "warnings" ];
        };
    };
}
