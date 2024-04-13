# `x800`: A fast and portable 2048 for POSIX

x800: [ɛks eɪt ˈhʌndrəd]

## Introduction

How fast? A sustained update [rate](#shell-benchmark-intel-pentium-g3220t-at-260ghz-with-linux-65) of 7.1 million moves per second on a 2.6GHz Intel Pentium CPU from 2013.

The project has few dependencies. It doesn't use ncurses or an external Rust terminal library: it instead relies on simple output buffering and standard [ANSI escape codes](https://en.wikipedia.org/wiki/ANSI_escape_code). The required POSIX-compatible [`libc`](https://docs.rs/libc/latest/libc/) Crate is included in Rust's standard library on POSIX targets. The only other build dependency is [`fastrand`](https://crates.io/crates/fastrand), a simple pseudo-random number generator Crate without child dependencies.

The project started in part as a hands-on spike project for the author to learn [rustlang](rust-lang.org) as a systems software developer experienced in C and C++. The name `x800` is the number 2048 in base sixteen, modified to comply with Cargo's package naming rules which disallow a leading digit.

## Preview

![`x800` gameplay](util/screenshot.jpeg)

## Quick start

The [rustup](https://rustup.rs) tool can be used to install the required Rust toolchain including `cargo` on your machine.

### Install

To install or update `x800`:

```sh
cargo install x800 --git 'https://github.com/evelynlewis/x800.git'
```

To run:

```sh
x800
```

### Uninstall

To uninstall `x800`:

```sh
cargo uninstall x800
```

### Cross-compilation with `cross` Crate

In case you wanted to play 2048 on a somewhat more underpowered or unique platform, the [`cross`](https://github.com/cross-rs/cross) Crate can be used to cross-compile for the target on a better-supported host.

This example was tested on both an x86-64 Linux box and an arm64 MacOS host. It builds `x800` to target a Raspberry Pi Zero W with target triple `arm-unknown-linux-gnueabihf`:

```sh
cargo install cross --git 'https://github.com/cross-rs/cross'
cross build --release --target=arm-unknown-linux-gnueabihf
```

> Note that as of February 2024, the latest released version of `cross`, `v0.2.5`, has an ongoing problem with supporting some non-x86_64 build hosts. As of the time of writing the best workaround for this is to install directly from the `main` branch on GitHub as shown above. This is also the [recommended](https://github.com/cross-rs/cross##installation) installation method in the `cross` project documentation.

## Compatibility

`x800` has the following requirements:

1. A Rust toolchain supporting the Rust 2021 Edition, meaning version 1.56.0 or later.
2. The toolchain supports the target system's target-triple (see [note 1](#compatibility-notes)).
3. The target's `libc` is POSIX-conforming (see [note 2](#compatibility-notes)).

This means that Linux, MacOS, other BSDs, and QNX should all work on a variety of architectures.

### Compatibility notes

1. For straightforward cross-compilation using `cross`, see the related [section](#cross-compilation-with-cross-crate) above.
2. Refer to the Rust Platform Support [page](https://doc.rust-lang.org/rustc/platform-support.html) and Rust `libc` [documentation](https://docs.rs/libc/latest/libc/). For the latter, the platform selection dropdown at the top of the page is useful.

## Gameplay

A description of 2048 gameplay, from [Wikipedia](https://en.wikipedia.org/wiki/2048_(video_game)):

> 2048 is played on a plain 4×4 grid, with numbered tiles that slide when a player moves them using the four arrow keys. The game begins with two tiles already in the grid, having a value of either 2 or 4, and another such tile appears in a random empty space after each turn. Tiles slide as far as possible in the chosen direction until they are stopped by either another tile or the edge of the grid. If two tiles of the same number collide while moving, they will merge into a tile with the total value of the two tiles that collided.

`x800` gameplay is quite similar to the original 2048, with the minor difference of using only letter keys in place of arrow keys for movement. This decision was made to allow every input move to be exactly one byte, whereas directional arrow keys map to multi-byte escape codes. Since the original 2048 supports both letter and arrow keys, this a focused implementation of the 2048 concept.

The keys ('w', 'a', 's', 'd') are used for (up, left, down, right) moves respectively.

## Local development

To set up `x800` for local testing and experimentation:

```sh
git clone 'https://github.com/evelynlewis/x800.git'
cd x800
cargo build
```

See-also the development shell scripts in the `util/` directory.

## Benchmarks

Since `x800` takes input from standard input, or `stdin`, and exits at the completion of a game, random games can be played by sending a stream of random moves to `stdin`. Monitoring the speed of characters being read from standard input and the typical time required to finish a game provides a reasonable performance benchmark.

### Benchmarks using the `hyperfine` tool

*The Rust Performance Book* [describes]((https://nnethercote.github.io/perf-book/benchmarking.html)) `hyperfine` as "an excellent general-purpose benchmarking tool." It seems to deliver.

The `hyperfine` latency numbers correspond to the duration of a complete game run beginning-to-end with random input. Statistical reasoning of the number of moves this typically corresponds to is left as an exercise for the reader.

#### Requirements

These benchmarks require GNU `base32`, `tr`, `dd`, and a recent version of [`hyperfine`](https://github.com/sharkdp/hyperfine).

> On most Linux systems the required GNU shell tools will be preinstalled. The `hyperfine` binary can be installed via your system package manager or with `cargo install hyperfine`. Your package manager's version may be too old.

#### `hyperfine` benchmark: Intel Pentium G3220T at 2.60GHz with Linux 6.5

```sh
./util/hyperfine.sh
    Finished release [optimized] target(s) in 0.02s
Benchmark 1: ./target/release/x800
  Time (mean ± σ):       1.1 ms ±   0.1 ms    [User: 0.8 ms, System: 0.1 ms]
  Range (min … max):     1.0 ms …   1.3 ms    256 runs
```

#### `hyperfine` benchmark: M1 MacBook Air with MacOS 14

```sh
./util/hyperfine.sh
    Finished release [optimized] target(s) in 0.01s
Benchmark 1: ./target/release/x800
  Time (mean ± σ):       1.2 ms ±   0.1 ms    [User: 0.5 ms, System: 0.4 ms]
  Range (min … max):     1.1 ms …   1.4 ms    256 runs
```

### Benchmarks using shell tools

Requires POSIX `sh`, and GNU `base32`, `dd`, `tr`, and `grep`.

> On most Linux systems the required GNU shell tools will be preinstalled. Also note that in this configuration, these benchmarks may be close to the performance limit for the shell tools themselves.

```sh
./util/bench.sh
```

### Shell benchmark: Intel Pentium G3220T at 2.60GHz with Linux 6.5

```sh
./util/bench.sh
Building x800 executable.
    Finished release [optimized] target(s) in 0.01s
Running benchmark for 16 seconds.
113035264 bytes (113 MB, 108 MiB) copied, 16 s, 7.1 MB/s
```

### Shell benchmark: M1 MacBook Air with MacOS 14

```sh
./util/bench.sh
Building x800 executable.
    Finished release [optimized] target(s) in 0.01s
Running benchmark for 16 seconds.
103557120 bytes (104 MB, 99 MiB) copied, 16 s, 6.5 MB/s
```

## Fuzzing

A fuzzer named `roger`, after the rabbit, is provided in the `fuzz/` directory. It's built using [`cargo-fuzz`](https://github.com/rust-fuzz/cargo-fuzz), which uses [LibFuzzer](https://llvm.org/docs/LibFuzzer.html), part of the LLVM project. A convenience script is under `util/fuzz.sh`.

A variant of `roger` using [honggfuzz](https://honggfuzz.dev) and [honggfuzz-rs](https://github.com/rust-fuzz/honggfuzz-rs) is also included. It's named `hroger`, and there's a convenience script under `util/hfuzz.sh`.

Both a small dictionary and a minimized corpus with good coverage are checked into the repository. Both are compatible with each fuzzer.

## Flame graphs

Flame graphs, first created by [Brendan Gregg](https://www.brendangregg.com/flamegraphs.html), are a powerful and intuitive performance-visualization tool. This project makes use of the Rust [`flamegraph`](https://github.com/flamegraph-rs/flamegraph) Crate for its straightforward flame graph support.

The helper script at `util/flamegraph.sh` allows for easy sampling of `x800`. In its default configuration, the script uses the `roger` fuzzer as the binary under test, with `cfg!(fuzzing)` enabled. The `util/flamegraph-ncf.sh` script is similar, but turns off `cfg!(fuzzing)`, which provides a more realistic `x800` gameplay workload.

## License

[MIT License](LICENSE.txt)

## References

- [*ANSI escape code*, Wikipedia](https://en.wikipedia.org/wiki/ANSI_escape_code)
- [*The TTY demystified*, Linus Åkesson](http://www.linusakesson.net/programming/tty/)
- [*Zero-dependency random number generation in Rust*, Orhun Parmaksız](https://blog.orhun.dev/zero-deps-random-in-rust/)
- [*termios(3) — Linux manual page*, man7.org](https://man7.org/linux/man-pages/man3/termios.3.html)
- [*The Rust Performance Book*, Nicholas Nethercote](https://nnethercote.github.io/perf-book)
- [*Clippy's Lints*, Clippy Documentation](https://doc.rust-lang.org/stable/clippy/lints.html)
- [*hyperfine*, GitHub](https://github.com/sharkdp/hyperfine)
- [*Rust Fuzz Book*, The Rust Fuzzing Authority](https://rust-fuzz.github.io/book)
- [*LibFuzzer*, llvm.org](https://llvm.org/docs/LibFuzzer.html)
- [*cargo-fuzz*, GitHub](https://github.com/rust-fuzz/cargo-fuzz)
- [*honggfuzz*, honggfuzz.dev](https://honggfuzz.dev)
- [*honggfuzz-rs*, GitHub](https://github.com/rust-fuzz/honggfuzz-rs)
- [*Flame Graphs*, Brendan Gregg](https://www.brendangregg.com/flamegraphs.html)
