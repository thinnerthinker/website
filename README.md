# Thinnerthinker's Personal Website

This codebase is equivalent to the live deployment of thinnerthinker's personal website at https://hopedge.com/.

## Building

### Prerequisites

To compile the website, ensure you have the following tools installed: Rust, Cargo, [Nix](https://nixos.org/download.html) (optional but recommended).

### Installation

Clone the repository:

```sh
git clone https://github.com/thinnerthinker/website.git
cd website
```

### Building with Cargo

You can compile on your native platform using Cargo:

```sh
cargo build --release
```

### Building with Nix

To build a binary inside the Nix store ready to run locally:

```sh
nix build .
```

### Cross-Compiling with Nix

#### Compiling for Linux

```sh
nix build .#x86_64-linux
```