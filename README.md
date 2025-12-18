# `x800`: a fast and portable 2048 for POSIX

x800 : /ˈɛks ˈeɪt ˈhəndɹəd/

## Introduction

2048 is played by merging a four-by-four grid of power-of-two tiles, to maximize the sum of merged tile values. `x800` is a fast and portable implementation.

`x800` can evaluate millions of moves per second, even on modest hardware. A Linux machine with an Intel Pentium CPU from 2013 was tested to play 4759 complete loops of a high-scoring 1080 move `x800` game in one second, or 4.86 million moves-per-second.

In the same test, a 2020 M1 MacBook Air with MacOS 26 completes 13.5 million moves-per-second, or 74.0 nanoseconds per move. In this duration, light travels 22.2 metres - a distance which Usain Bolt ran in 1.81 [seconds](https://www.ncbi.nlm.nih.gov/pmc/articles/PMC10669785) in 2009 to set the current 100m sprint record.

See also the benchmark [section](#benchmarks) below, and the [util/bench-ryan.sh](./util/bench-ryan.sh) script.

The project has few build dependencies. Rather than ncurses or an external Rust terminal library, `x800` uses simple output buffering with standard [ANSI escape codes](https://en.wikipedia.org/wiki/ANSI_escape_code).

- The required POSIX-compatible [`libc`](https://docs.rs/libc/latest/libc/) Crate is part of Rust's standard library on POSIX targets.
- The [fastrand](https://crates.io/crates/fastrand) Crate is a simple and fast non-cryptographic pseudorandom number generator.

The project CI builds and tests `x800` on 25 cross-compiled POSIX targets - that is, all `rust-std` POSIX targets supported by the latest stable version of the widely-used `cross` cross-compilation [tool](https://github.com/cross-rs/cross).

The project is also built and tested natively on all standard GitHub-hosted Linux or macOS [runners](https://docs.github.com/en/actions/using-github-hosted-runners/using-github-hosted-runners/about-github-hosted-runners#standard-github-hosted-runners-for-public-repositories). Binary tarballs for any cross-compiled or native platform can be downloaded from the latest GitHub Actions [`build-bot`](https://github.com/evelynlewis/x800/actions/workflows/ci.yml) run. See below for local build instructions.

The project started in part as a hands-on spike project for the author to learn [Rust](https://rust-lang.org) as a systems software developer experienced in C and C++. The name `x800` is the number 2048 in base sixteen, modified to comply with Cargo's package naming rules which disallow a leading digit.

## Quick start

Use the [`rustup`](https://rustup.rs) tool to install the required Rust toolchain including `cargo` on your machine.

To install or update `x800`:

```sh
cargo install x800 --git 'https://github.com/evelynlewis/x800.git'
```

To run `x800`:

```sh
x800
```

To uninstall `x800`:

```sh
cargo uninstall x800
```

## Preview

![`x800` gameplay](etc/screen.png)

## Gameplay

A description of 2048 gameplay, from [Wikipedia](<https://en.wikipedia.org/wiki/2048_(video_game)>):

> 2048 is played on a plain 4×4 grid, with numbered tiles that slide when a player moves them using the four arrow keys. The game begins with two tiles already in the grid, having a value of either 2 or 4, and another such tile appears in a random empty space after each turn. Tiles slide as far as possible in the chosen direction until they are stopped by either another tile or the edge of the grid. If two tiles of the same number collide while moving, they will merge into a tile with the total value of the two tiles that collided. The resulting tile cannot merge with another tile again in the same move.
>
> […]
>
> The game is won when a tile with a value of 2048 appears on the board. Players can continue beyond that to reach higher scores. When the player has no legal moves (there are no empty spaces and no adjacent tiles with the same value), the game ends.

`x800` gameplay is quite similar to the original 2048, with the minor difference of using only letter keys in place of arrow keys for movement. This decision was made to allow every input move to be exactly one byte, whereas directional arrow keys map to multi-byte escape codes. Since the original 2048 supports both letter and arrow keys, this a focused implementation of the 2048 concept.

`x800` uses the 'w', 'a', 's', and 'd' keys for up, left, down, and right moves respectively.

## Compatibility

`x800` has the following requirements:

1. A Rust toolchain supporting the target system's target-triple, including `std` support. See [notes 1 and 2](#notes).
2. The target's Rust `libc` crate is POSIX-conforming. See [notes 2 and 3](#notes).
3. A Minimum Supported Rust Version (MSRV) of 1.80.

There are no other known requirements.

This means that Linux, MacOS, and other BSDs should all work on a variety of architectures.

### Notes

1. For straightforward cross-compilation using `cross`, see the related [section](#cross-compilation-with-cross-crate).
2. For information on Rust platform support see the appropriate [page](https://doc.rust-lang.org/rustc/platform-support.html) in the Rust documentation.
3. Refer to the Rust `libc` [documentation](https://docs.rs/libc/latest/libc/). The platform selection dropdown at the top of the page is useful.

### Cross-compilation with `cross` crate

In case you wanted to play 2048 on a somewhat more underpowered or unique platform, the [`cross`](https://github.com/cross-rs/cross) crate can be used to cross-compile for the target on a better-supported host.

The project CI `compatibility` jobs use `cross` to build and test `x800` on a wide range of target architectures and operating systems - at the time of writing, all POSIX platforms supported by the latest release of `cross`, `v0.2.5`.

This example was tested on both an x86-64 Linux box and an arm64 MacOS host. It builds `x800` to target a Raspberry Pi Zero W with target triple `arm-unknown-linux-gnueabihf`:

```sh
cargo install cross --git 'https://github.com/cross-rs/cross'
cross build --release --target=arm-unknown-linux-gnueabihf
```

> Note that as of February 2024, the latest released version of `cross`, `v0.2.5`, has an ongoing problem with supporting some non-x86_64 build hosts. As of the time of writing the best workaround for this is to install directly from the `main` branch on GitHub, as shown. This is also the [recommended](https://github.com/cross-rs/cross##installation) installation method in the `cross` project documentation.

## Local development

To set up `x800` for local testing and experimentation:

```sh
# Packages to build on Ubuntu. These are likely already installed.
sudo apt install build-essential git
git clone 'https://github.com/evelynlewis/x800.git'
cd x800
cargo build
```

See-also the development shell scripts in the `util/` directory. In particular, the [`util/preflight.sh`](./util/preflight.sh) script runs a slew of tests starting from a clean build each time - useful to verify changes prior to integration. The [`util/bench-ryan.sh`](./util/bench-ryan.sh) script can also be used to benchmark candidate code changes.

## Benchmarks

The `ryan` benchmarks in [`util/bench-ryan.sh`](./util/bench-ryan.sh) are run using the the [`ryan`](./src/ryan/README.md) reproducible `x800` driver. They use the [`hyperfine`](https://github.com/sharkdp/hyperfine) tool to benchmark a large number of loops of a known high-scoring, or 'gold', `x800` save file.

The `hyperfine` tool can be installed via your system package manager or with:

```sh
cargo install hyperfine
```

The `ryan` benchmarks can be run with:

```sh
./util/bench-ryan.sh
```

Other benchmarks and tests can be run with:

```sh
cargo bench --all-targets
```

## Fuzzing

A fuzzer named `roger`, after the rabbit, is provided in the `fuzz/` directory. It's built using [`cargo-fuzz`](https://github.com/rust-fuzz/cargo-fuzz), which uses [LibFuzzer](https://llvm.org/docs/LibFuzzer.html), part of the LLVM project. A convenience script is under `util/fuzz-libfuzz`.

A variant of `roger` using [honggfuzz](https://honggfuzz.dev) and [honggfuzz-rs](https://github.com/rust-fuzz/honggfuzz-rs) is also included. It's named `hroger`, and there's a convenience script under `util/fuzz-hongg.sh`.

Both a small dictionary and a minimized corpus with good coverage are checked into the repository. Both are compatible with each fuzzer.

## License

MIT License

Copyright (c) 2024-2025 Evelyn Lewis

Permission is hereby granted, free of charge, to any person obtaining a copy
of this software and associated documentation files (the "Software"), to deal
in the Software without restriction, including without limitation the rights
to use, copy, modify, merge, publish, distribute, sublicense, and/or sell
copies of the Software, and to permit persons to whom the Software is
furnished to do so, subject to the following conditions:

The above copyright notice and this permission notice shall be included in all
copies or substantial portions of the Software.

THE SOFTWARE IS PROVIDED "AS IS", WITHOUT WARRANTY OF ANY KIND, EXPRESS OR
IMPLIED, INCLUDING BUT NOT LIMITED TO THE WARRANTIES OF MERCHANTABILITY,
FITNESS FOR A PARTICULAR PURPOSE AND NONINFRINGEMENT. IN NO EVENT SHALL THE
AUTHORS OR COPYRIGHT HOLDERS BE LIABLE FOR ANY CLAIM, DAMAGES OR OTHER
LIABILITY, WHETHER IN AN ACTION OF CONTRACT, TORT OR OTHERWISE, ARISING FROM,
OUT OF OR IN CONNECTION WITH THE SOFTWARE OR THE USE OR OTHER DEALINGS IN THE
SOFTWARE.

## Some references

- [_Multicomponent Velocity Measurement for Linear Sprinting: Usain Bolt’s 100 m World-Record Analysis_, _Bioengineering (Basel, Switzerland)_, National Library of Medicine](https://www.ncbi.nlm.nih.gov/pmc/articles/PMC10669785)
- [_ANSI escape code_, Wikipedia](https://en.wikipedia.org/wiki/ANSI_escape_code)
- [_The TTY demystified_, Linus Åkesson](http://www.linusakesson.net/programming/tty/)
- [_Zero-dependency random number generation in Rust_, Orhun Parmaksız](https://blog.orhun.dev/zero-deps-random-in-rust/)
- [_termios(3) — Linux manual page_, man7.org](https://man7.org/linux/man-pages/man3/termios.3.html)
- [_Performance Analysis and Tuning on Modern CPUs, 1st Edition_, Denis Bakhvalov](https://easyperf.net)
- [_The Rust Performance Book_, Nicholas Nethercote](https://nnethercote.github.io/perf-book)
- [_Rust Fuzz Book_, The Rust Fuzzing Authority](https://rust-fuzz.github.io/book)
- [_cross_, GitHub](https://github.com/cross-rs/cross)
- [_hyperfine_, GitHub](https://github.com/sharkdp/hyperfine)
- [_cargo-fuzz_, GitHub](https://github.com/rust-fuzz/cargo-fuzz)
- [_LibFuzzer_, llvm.org](https://llvm.org/docs/LibFuzzer.html)
- [_honggfuzz_, honggfuzz.dev](https://honggfuzz.dev)
- [_honggfuzz-rs_, GitHub](https://github.com/rust-fuzz/honggfuzz-rs)
- [_Flame Graphs_, Brendan Gregg](https://www.brendangregg.com/flamegraphs.html)
