# Cowsay-rs

[![built with nix](https://builtwithnix.org/badge.svg)](https://builtwithnix.org)
[![CI](https://github.com/franco-grobler/cowsay-rs/actions/workflows/ci.yaml/badge.svg)](https://github.com/franco-grobler/cowsay-rs/actions/workflows/ci.yaml)

A Rust implementation of the classic cowsay program.

## Installation

### From Source

You can build it from the source code:

```bash
git clone https://github.com/franco-grobler/cowsay-rs.git
cd cowsay-rs
cargo install --path crates/cowsay-cli
```

## Usage

The command-line interface is available via the `cowsay-rs` binary.

### Basic Usage

To have a cow say something, simply provide a message:

```bash
cowsay-rs "Moo!"
```

```text
   ______
  < Moo! >
   ------
          \   ^__^
           \  (oo)\_______
              (__)\       )\/\
                  ||----w |
                  ||     ||
```

### Listing Available Cows

You can list all the available cowfiles:

```bash
cowsay-rs -l
```

### Choosing a Cow

Use the `-f` flag to select a different character:

```bash
cowsay-rs -f tux "Hello from Tux!"
```

```text
   _________________
  < Hello from Tux! >
   -----------------
          \
           \
              .--.
             |o_o |
             |:_/ |
            //   \ \
           (|     | )
          /'\_   _/`\
          \___)=(___/
```

### Other Options

There are various flags to alter the cow's appearance. For example, to make the
cow look "greedy":

```bash
cowsay-rs -g "I want more grass"
```

```text
   ___________________
  < I want more grass >
   -------------------
          \   ^__^
           \  ($$)\_______
              (__)\       )\/\
                  ||----w |
                  ||     ||
```

For a full list of options, you can use the `--help` flag:

```bash
cowsay-rs --help
```

## Contributing

Contributions are welcome! If you'd like to help, please feel free to open an
issue or submit a pull request.
