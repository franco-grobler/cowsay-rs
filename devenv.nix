{
  pkgs,
  ...
}:

{
  # https://devenv.sh/packages/
  packages = [ pkgs.just ];

  # https://devenv.sh/languages/
  languages.rust = {
    enable = true;
    channel = "stable";
    version = "latest";
    components = [
      "rustc"
      "cargo"
      "clippy"
      "rustfmt"
      "rust-analyzer"
    ];
  };
}
