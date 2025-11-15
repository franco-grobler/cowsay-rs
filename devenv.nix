{
  pkgs,
  ...
}:

{
  # https://devenv.sh/packages/
  packages = [
    # Testing and coverage tools
    pkgs.cargo-audit
    pkgs.cargo-nextest
    # pkgs.cargo-llvm-cov
    pkgs.cargo-shear

    # Utilities
    pkgs.pre-commit
  ];

  # https://devenv.sh/languages/
  languages.rust = {
    enable = true;
    components = [
      "rustc"
      "cargo"
      "clippy"
      "rustfmt"
      "rust-analyzer"
    ];
    toolchainFile = ./rust-toolchain.toml;
  };

  enterShell = ''
    unset DEVELOPER_DIR;
  '';
}
