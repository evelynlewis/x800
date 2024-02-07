# `x800`: minimal and fast *2048* for POSIX
\[*x-eight-hundred*\]: a standards-compliant implementation of the [*2048*](https://en.wikipedia.org/wiki/2048_(video_game)) game

## Introduction

This project was created in part as a hands-on spike project for the author to learn [rustlang](rust-lang.org) as an experienced C and C++ systems software developer.

The name `x800` is the number `2048` in base sixteen, modified to comply with Cargo's package naming rules which disallow a leading digit in package names.

It was also an exercise in creating a working program of substantial size with minimal external dependencies. It doesn't use ncurses. The only dependency is the `libc` crate, which is already included in rustlang's `std` – so depending how you count it, `x800` has either zero dependencies or one.

With the above in mind, choices were made to write the code in an idiomatic as possible style, and in general follow the principle of least surprise.

## Preview
![x800 screenshotmar](screenshot.jpg)

## Quick start
`x800` should run on any POSIX compatible host targeted by a Rust toolchain supporting the Rust 2021 epoch. 

The [rustup](https://rustup.rs) tool can be used to install the required Rust toolchain on your build host.

### Running `x800` with `cargo run`

Check out the repository and use `cargo run`:

```sh
git clone 'https://github.com/evelynlewis/x800.git'
cd x800
cargo run --release
```

### Installation with Cargo

If you wish to install the program in your home directory, while still managed by Cargo, that can be done with:

```sh
git clone 'https://github.com/evelynlewis/x800.git'
cargo install --path x800
```

Then the binary can be run from any directory with:

```sh
x800
```

### Uninstallation with Cargo

To uninstall the binary:

```sh
cargo uninstall x800
```

## Cross-compilation with `cross` crate

In case you wanted to play 2048 on a somewhat more underpowered or exotic platform, the `cross` crate can be used to cross-compile from a better-supported host.

Here's an example which I tested on an x86-64 Linux box to build `x800` targeting a Raspberry Pi Zero W:

```sh
cargo install cross
cross build --release --target=arm-unknown-linux-gnueabihf
```

### Compatibility note

Docker is used by `cross` behind the scenes, so a working installation and a build host supported by the appropriate `cross` crate Docker image is required. 

Note that with the default `cross` setup, Macs with ARM seem not to be supported as a build host. I didn't look into it much further at the time since I had another machine on hand.

```sh
# On an M1 MacOS host, this fails like so:
cross build --release --target=arm-unknown-linux-gnueabihf
Unable to find image 'ghcr.io/cross-rs/arm-unknown-linux-gnueabihf:0.2.5' locally
0.2.5: Pulling from cross-rs/arm-unknown-linux-gnueabihf
docker: no matching manifest for linux/arm64/v8 in the manifest list entries.
See 'docker run --help'.
```

## License
[MIT License](LICENSE.txt)

## References
- [*ANSI escape code*: wikipedia.org](https://en.wikipedia.org/wiki/ANSI_escape_code)
- [*The TTY demystified*: linusakesson.net](http://www.linusakesson.net/programming/tty/)
- [*Zero-dependency random number generation in Rust* – Orhun Parmaksız: orhun.dev](https://blog.orhun.dev/zero-deps-random-in-rust/)
- [*termios(3) — Linux manual page*: man7.org](https://man7.org/linux/man-pages/man3/termios.3.html)