{
  pkgs,
  ...
}:

{
  # https://devenv.sh/packages/
  packages = [
    # Testing and coverage tools
    pkgs.cargo-nextest
    pkgs.cargo-llvm-cov

    # Utilities
    pkgs.just
  ];

  # https://devenv.sh/languages/
  languages.rust = {
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
}
