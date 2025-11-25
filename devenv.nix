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
  languages = {
    javascript = {
      enable = true;
      bun = {
        enable = true;
      };
    };
    rust = {
      enable = true;
      channel = "stable";
      components = [
        "rustc"
        "cargo"
        "clippy"
        "rustfmt"
        "rust-analyzer"
      ];
      version = "1.91.0";
    };
  };

  enterShell = ''
    unset DEVELOPER_DIR;
  '';
}
