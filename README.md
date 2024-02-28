# `x800`: a fast and portable *2048* for POSIX

x800: [ɛks eɪt ˈhʌndrəd]

## Introduction

How fast? A sustained update rate of around five-and-a-quarter million moves per second on a 4th Generation Intel Pentium from 2013 (see methodology [below](#intel-pentium-g3220t-at-260ghz-with-linux-65-shell-benchmark)).

The project has few external dependencies. It doesn't use ncurses, or a Rust terminal library such as [ratatui](https://crates.io/crates/ratatui). It instead relies on simple frame-buffering and standard [ANSI escape codes](https://en.wikipedia.org/wiki/ANSI_escape_code). The only build dependency aside from the Rust Standard Library is the [`fastrand`](https://crates.io/crates/fastrand) Crate, a simple PRNG without child dependencies. The program also requires a POSIX-compatible `libc` crate, which is included in Rust's `std` when building for POSIX targets.

The project was created in part as a hands-on spike project for the author to learn [rustlang](rust-lang.org) as a systems software developer experienced in C and C++.

The name `x800` is the number 2048 in base sixteen, modified to comply with Cargo's package naming rules which disallow a leading digit.

## Preview

![`x800` gameplay](util/screenshot.jpeg)

## Gameplay

A description of *2048* gameplay, from [Wikipedia](https://en.wikipedia.org/wiki/2048_(video_game)):

> 2048 is played on a plain 4×4 grid, with numbered tiles that slide when a player moves them using the four arrow keys. The game begins with two tiles already in the grid, having a value of either 2 or 4, and another such tile appears in a random empty space after each turn. Tiles slide as far as possible in the chosen direction until they are stopped by either another tile or the edge of the grid. If two tiles of the same number collide while moving, they will merge into a tile with the total value of the two tiles that collided.

`x800` gameplay is quite similar to the original *2048*, with the minor difference of using only letter in place of arrow keys for movement. Since *2048* supports both letter and arrow keys, this a focused implementation of the *2048* concept.

The keys ('w', 'a', 's', 'd') are used for (up, left, down, right) moves respectively.

## Compatibility

`x800` has the following requirements:

1. A Rust toolchain supporting the Rust 2021 Edition, meaning version 1.56.0 or later.
2. The toolchain supports the target system's target-triple [[1]](#compatibility-notes).
3. The target's `libc` is POSIX-conforming [[2]](#compatibility-notes).

This means that Linux, MacOS, other BSDs, QNX, and MinGW should all work on a variety of architectures.

### Compatibility notes

1. For straightforward cross-compilation using `cross`, see the related [section](#cross-compilation-with-cross-crate) below.
2. Refer to the Rust Platform Support [page](https://doc.rust-lang.org/rustc/platform-support.html) and Rust `libc` [documentation](https://docs.rs/libc/latest/libc/). For the latter, the platform selection dropdown at the top of the page is useful.

## Quick start

### Pre-setup

The [rustup](https://rustup.rs) tool can be used to install the required Rust toolchain including `cargo` on your machine.

### To install

To install the program locally:

```sh
cargo install x800 --git 'https://github.com/evelynlewis/x800.git'
```

To run:

```sh
x800
```

### To uninstall

To uninstall the binary:

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

> Note that as of February 2024, the latest released version of `cross`, `v0.2.5`, has an ongoing problem with supporting some non-x86_64 build hosts. As of the time of writing the best workaround for this is to install directly from the `main` branch on Github as shown above. This is also the [recommended](https://github.com/cross-rs/cross##installation) installation method in the `cross` project documentation.

## Local development

To set up `x800` for local testing and experimentation:

```sh
git clone 'https://github.com/evelynlewis/x800.git'
cd x800
cargo build
```

See-also the development shell scripts in the `util/` directory.

## Mini-benchmarks

Since `x800` takes input from standard input, or `stdin`, and exits at the completion of a game, random games can be played by sending a stream of random moves to `stdin`. Monitoring the speed of characters being read from standard input and the typical time required to finish a game provides a reasonable performance benchmark.

### Notes on benchmark results and their interpretation

  1. The `pv` tool output of `[ n MiB/s]` corresponds to `n × 1024²`, or `n × 1048576` discrete directional moves processed by `x800` in a given second. In the measurements below, the first number is the instantaneous rate, and the second the average. Measurements were taken until the the average rate had stabilized for at least several seconds.

  2. The `hyperfine` latency numbers correspond to the duration of a complete randomly-run game, beginning-to-end. Statistical reasoning of the number of moves this typically corresponds to is left as an exercise for the reader.

### Mini-benchmarks using the `hyperfine` tool

`hyperfine` is [described](https://nnethercote.github.io/perf-book/benchmarking.html) by *The Rust Performance Book* as "an excellent general-purpose benchmarking tool." It seems to deliver.

These benchmarks require GNU `base32`, `tr`, `pv`, and a recent version of [`hyperfine`](https://github.com/sharkdp/hyperfine).

> Note: The `hyperfine` binary can be installed via your system package manager or with `cargo install hyperfine`. Your package manager's version may be too old.

#### M1 MacBook Air with MacOS hyperfine benchmark

```sh
./util/bench.sh
+ test -d ./util/
+ test -f ./util/gen-moves.sh
+ cargo build -p x800 --release
    Finished release [optimized] target(s) in 0.03s
+ touch /tmp/moves
+ hyperfine --prepare './util/gen-moves.sh /tmp/moves' --warmup=256 --runs=256 --input=/tmp/moves -N ./target/release/x800
Benchmark 1: ./target/release/x800
  Time (mean ± σ):       1.4 ms ±   0.1 ms    [User: 0.7 ms, System: 0.4 ms]
  Range (min … max):     1.2 ms …   1.7 ms    256 runs
```

#### Intel Pentium G3220T at 2.60GHz with Linux 6.5 hyperfine benchmark

```sh
./util/bench.sh
+ test -d ./util/
+ test -f ./util/gen-moves.sh
+ cargo build -p x800 --release
    Finished release [optimized] target(s) in 0.07s
+ touch /tmp/moves
+ hyperfine --prepare ./util/gen-moves.sh /tmp/moves --warmup=256 --runs=256 --input=/tmp/moves -N ./target/release/x800
Benchmark 1: ./target/release/x800
  Time (mean ± σ):       1.4 ms ±   0.1 ms    [User: 1.1 ms, System: 0.2 ms]
  Range (min … max):     1.1 ms …   1.9 ms    256 runs
```

### Mini-benchmarks using shell tools

Requires GNU `base32`, `tr`, `dash`, `grep`, and `pv`.

> Note: MacOS arguments and tool names modified slightly, as provided below.

```sh
cat /dev/urandom \
    | base32 \
    | tr -s '[:upper:]' '[:lower:]' | tr -dC 'asdw' \
    | pv --wait --rate --average-rate \
    | dd bs=256 \
    | dash -c 'while true; do ./target/release/x800; done' > /dev/null

```

### M1 MacBook Air with MacOS shell benchmark

```sh
cat /dev/urandom \
    | gbase32 \
    | tr -s '[:upper:]' '[:lower:]' | tr -dC 'asdw' \
    | dd bs=256 \
    | pv --wait --rate --average-rate \
    | dash -c 'while true; do ./target/release/x800; done' > /dev/null
[2.66MiB/s] [2.64MiB/s]
```

### Intel Pentium G3220T at 2.60GHz with Linux 6.5 shell benchmark

```sh
cat /dev/urandom \
    | base32 \
    | tr -s '[:upper:]' '[:lower:]' | tr -dC 'asdw' \
    | dd bs=256 \
    | pv --wait --rate --average-rate \
    | dash -c 'while true; do ./target/release/x800; done' > /dev/null
[5.35MiB/s] [5.01MiB/s]
```

## Fuzzing

A fuzzer named `roger`, after the rabbit, is provided in the `fuzz/` directory. It is build using [`cargo-fuzz`](https://github.com/rust-fuzz/cargo-fuzz), which uses [LibFuzzer](https://llvm.org/docs/LibFuzzer.html), part of the LLVM project. A convenience script is provided under `util/fuzz.sh`. Both a small dictionary and a minimized corpus with good coverage are checked into the repository.

## Flame graphs

Flame graphs, first created by [Brendan Gregg](https://www.brendangregg.com/flamegraphs.html), are a powerful and intuitive performance-visualization tool. `x800` makes use of the Rust [flamegraph](https://github.com/flamegraph-rs/flamegraph) Crate for its straightforward flame graph support. A helper script at `util/flamegraph.sh` allows for easy sampling of `x800`. In its default configuration, the script uses the `roger` fuzzer as the binary under test.

## License

[MIT License](LICENSE.txt)

## References

- [*ANSI escape code*, Wikipedia](https://en.wikipedia.org/wiki/ANSI_escape_code)
- [*The TTY demystified*, Linus Åkesson](http://www.linusakesson.net/programming/tty/)
- [*Zero-dependency random number generation in Rust*, Orhun Parmaksız](https://blog.orhun.dev/zero-deps-random-in-rust/)
- [*termios(3) — Linux manual page*, man7.org](https://man7.org/linux/man-pages/man3/termios.3.html)
- [*The Rust Performance Book*, Nicholas Nethercote](https://nnethercote.github.io/perf-book)
- [*Clippy's Lints*, Clippy Documentation](https://doc.rust-lang.org/stable/clippy/lints.html)
- [`hyperfine`, GitHub](https://github.com/sharkdp/hyperfine)
- [LibFuzzer, llvm.org](https://llvm.org/docs/LibFuzzer.html)
- [*Rust Fuzz Book*, The Rust Fuzzing Authority](https://rust-fuzz.github.io/book)
- [`cargo-fuzz`, GitHub](https://github.com/rust-fuzz/cargo-fuzz)
- [*Flame Graphs*, Brendan Gregg](https://www.brendangregg.com/flamegraphs.html)
