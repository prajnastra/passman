# Passman
Simple password generator in Rust


## Prerequisites

### Clone the source code

First clone the source code:

```sh
git clone https://github.com/prajnastra/passman.git
cd passman
```

### Install the Rust compiler with `rustup`

1. Install [`rustup.rs`](https://rustup.rs/).

3. To make sure you have the right Rust compiler installed, run

   ```sh
   rustup override set stable
   rustup update stable
   ```

## Building

### Linux 

```sh
cargo build --release
```

### Install
```sh
mkdir -p ~/.local/bin
cp target/release/passman ~/.local/bin/
```

## Usage
```bash
passman
```
