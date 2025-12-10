{
  pkgs,
  ...
}:

{
  # https://devenv.sh/packages/
  packages = [
    # Testing and coverage tools
    pkgs.cargo-audit
    pkgs.cargo-dist
    pkgs.cargo-nextest
    # pkgs.cargo-llvm-cov
    pkgs.cargo-shear

    # WASM tools
    pkgs.wasm-pack

    # Utilities
    pkgs.git-cliff
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
      toolchainFile = ./rust-toolchain.toml;
    };
  };

  enterShell = ''
    unset DEVELOPER_DIR;
  '';
}
