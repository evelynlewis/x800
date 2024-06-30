# `x800`: a fast and portable 2048 for POSIX

x800 : /ˈɛks ˈeɪt ˈhəndɹəd/

## Introduction

2048 is played by merging a four-by-four grid of power-of-two tiles, to maximize the sum of resulting tile values.

How fast? In one second, a Linux machine with an Intel Pentium CPU from 2013 can play 4300 complete loops of a four thousand move `x800` game. This gives an effective throughput of 16 million moves-per-second. In the same test, an MacOS M1 MacBook Air can complete 35 million moves-per-second. When disabling rendering by using the `mini` build feature, `x800` on the M1 machine processes 55 million moves-per-second, or 18 nanoseconds per move. This duration is the time needed for light to travel 5.5 metres: the [height](https://en.wikipedia.org/wiki/Giraffe#Anatomy) of a giraffe, and less than half the [length](https://olympics.com/en/news/badminton-court-size-dimension-measurement-length-width-net-height-service-line) of a standard badminton court.

See also the benchmark [section](#benchmarks) below, and the [util/rbench.sh](./util/rbench.sh) script.

The project has few build dependencies. It doesn't use ncurses or an external Rust terminal library. It instead uses simple output buffering and standard [ANSI escape codes](https://en.wikipedia.org/wiki/ANSI_escape_code). The required POSIX-compatible [`libc`](https://docs.rs/libc/latest/libc/) crate is part of Rust's standard library on POSIX targets. The only other build dependency is [`fastrand`](https://crates.io/crates/fastrand), a simple non-cryptographic pseudorandom number generator crate without child dependencies.

The project started in part as a hands-on spike project for the author to learn [Rust](https://rust-lang.org) as a systems software developer experienced in C and C++. The name `x800` is the number 2048 in base sixteen, modified to comply with Cargo's package naming rules which disallow a leading digit.

## Preview

![`x800` gameplay](util/screenshot.jpeg)

## Quick start

Use the [`rustup`](https://rustup.rs) tool to install the required Rust toolchain including `cargo` on your machine.

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

## Gameplay

A description of 2048 gameplay, from [Wikipedia](<https://en.wikipedia.org/wiki/2048_(video_game)>):

> 2048 is played on a plain 4×4 grid, with numbered tiles that slide when a player moves them using the four arrow keys. The game begins with two tiles already in the grid, having a value of either 2 or 4, and another such tile appears in a random empty space after each turn. Tiles slide as far as possible in the chosen direction until they are stopped by either another tile or the edge of the grid. If two tiles of the same number collide while moving, they will merge into a tile with the total value of the two tiles that collided. The resulting tile cannot merge with another tile again in the same move. Higher-scoring tiles emit a soft glow; the largest possible tile is 131,072.
>
> …
>
> The game is won when a tile with a value of 2048 appears on the board. Players can continue beyond that to reach higher scores. When the player has no legal moves (there are no empty spaces and no adjacent tiles with the same value), the game ends.

`x800` gameplay is quite similar to the original 2048, with the minor difference of using only letter keys in place of arrow keys for movement. This decision was made to allow every input move to be exactly one byte, whereas directional arrow keys map to multi-byte escape codes. Since the original 2048 supports both letter and arrow keys, this a focused implementation of the 2048 concept.

`x800` uses the 'w', 'a', 's', and 'd' keys for up, left, down, and right moves respectively.

## Compatibility

`x800` has the following requirements:

1. A Rust toolchain supporting the target system's target-triple, including `std` support. See [notes 1 and 2](#notes).
2. The target's Rust `libc` crate is POSIX-conforming. See [notes 2 and 3](#notes).
3. A Minimum Supported Rust Version (MSRV) of 1.56. This version stabilized support for the 2021 Edition.

There are no other known requirements.

This means that Linux, MacOS, and other BSDs should all work on a variety of architectures.

### Notes

1. For straightforward cross-compilation using `cross`, see the related [section](#cross-compilation-with-cross-crate).
2. For information on Rust platform support see the appropriate [page](https://doc.rust-lang.org/rustc/platform-support.html) in the Rust documentation.
3. Refer to the Rust `libc` [documentation](https://docs.rs/libc/latest/libc/). The platform selection dropdown at the top of the page is useful.

### Cross-compilation with `cross` crate

In case you wanted to play 2048 on a somewhat more underpowered or unique platform, the [`cross`](https://github.com/cross-rs/cross) crate can be used to cross-compile for the target on a better-supported host.

This example was tested on both an x86-64 Linux box and an arm64 MacOS host. It builds `x800` to target a Raspberry Pi Zero W with target triple `arm-unknown-linux-gnueabihf`:

```sh
cargo install cross --git 'https://github.com/cross-rs/cross'
cross build --release --target=arm-unknown-linux-gnueabihf
```

> Note that as of February 2024, the latest released version of `cross`, `v0.2.5`, has an ongoing problem with supporting some non-x86_64 build hosts. As of the time of writing the best workaround for this is to install directly from the `main` branch on GitHub, as shown. This is also the [recommended](https://github.com/cross-rs/cross##installation) installation method in the `cross` project documentation.

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

_The Rust Performance Book_ [describes](<(https://nnethercote.github.io/perf-book/benchmarking.html)>) `hyperfine` as "an excellent general-purpose benchmarking tool." It seems to deliver.

The `hyperfine` latency numbers correspond to the duration of a complete game run beginning-to-end with random input. Statistical reasoning of the number of moves this typically corresponds to is left as an exercise for the reader.

#### Requirements

These benchmarks require GNU `base32`, `tr`, `dd`, and a recent version of [`hyperfine`](https://github.com/sharkdp/hyperfine).

> The required GNU shell tools are part of GNU `coreutils` and preinstalled on most Linux systems. The `hyperfine` binary can be installed via your system package manager or with `cargo install hyperfine`. Your package manager's version may be too old.

#### `hyperfine` benchmark: Intel Pentium G3220T at 2.60GHz with Linux 6.5

```sh
./util/hyperfine.sh
[...]
++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++
Running with random input

Benchmark 1: ./target/release/x800
  Time (mean ± σ):       1.1 ms ±   0.1 ms    [User: 0.4 ms, System: 0.6 ms]
  Range (min … max):     1.0 ms …   1.5 ms    256 runs

++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++
Running with gold file input

Size, resolved path, and loop count are:
4.0K	./fuzz/corpus/roger/0385ba3512e77623555fabd0ec9340830cfbf947 1000

Benchmark 1: ./target/release/ryan ./fuzz/corpus/roger/0385ba3512e77623555fabd0ec9340830cfbf947 1000
  Time (mean ± σ):     260.0 ms ±   2.9 ms    [User: 230.6 ms, System: 34.6 ms]
  Range (min … max):   253.6 ms … 263.0 ms    32 runs
```

#### `hyperfine` benchmark: M1 MacBook Air with MacOS 14

```sh
./util/hyperfine.sh
[...]
++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++
Running with random input

Benchmark 1: ./target/release/x800
  Time (mean ± σ):       1.2 ms ±   0.1 ms    [User: 0.6 ms, System: 0.4 ms]
  Range (min … max):     1.1 ms …   1.6 ms    256 runs

++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++
Running with gold file input

Size, resolved path, and loop count are:
4.0K	./fuzz/corpus/roger/0385ba3512e77623555fabd0ec9340830cfbf947 1000

Benchmark 1: ./target/release/ryan ./fuzz/corpus/roger/0385ba3512e77623555fabd0ec9340830cfbf947 1000
  Time (mean ± σ):     120.6 ms ±   0.3 ms    [User: 101.4 ms, System: 27.7 ms]
  Range (min … max):   120.1 ms … 121.6 ms    32 runs
```

### Benchmarks using shell tools

Requires POSIX `sh`, and GNU `base32`, `dd`, `tr`, and `grep`.

> The required GNU shell tools are part of GNU `coreutils` and preinstalled on most Linux systems. Also note that in this configuration, these benchmarks may be close to the performance limit for the shell tools themselves.

```sh
./util/bench.sh
```

### Shell benchmark: Intel Pentium G3220T at 2.60GHz with Linux 6.5

```sh
./util/bench.sh
Building x800 executable.
    Finished `release` profile [optimized] target(s) in 0.01s
Running benchmark for 16 seconds.
119332864 bytes (119 MB, 114 MiB) copied, 16 s, 7.5 MB/s
```

### Shell benchmark: M1 MacBook Air with MacOS 14

```sh
./util/bench.sh
Building x800 executable.
    Finished `release` profile [optimized] target(s) in 0.01s
Running benchmark for 16 seconds.
103407616 bytes (103 MB, 99 MiB) copied, 16 s, 6.5 MB/s
```

## Fuzzing

A fuzzer named `roger`, after the rabbit, is provided in the `fuzz/` directory. It's built using [`cargo-fuzz`](https://github.com/rust-fuzz/cargo-fuzz), which uses [LibFuzzer](https://llvm.org/docs/LibFuzzer.html), part of the LLVM project. A convenience script is under `util/fuzz.sh`.

A variant of `roger` using [honggfuzz](https://honggfuzz.dev) and [honggfuzz-rs](https://github.com/rust-fuzz/honggfuzz-rs) is also included. It's named `hroger`, and there's a convenience script under `util/hfuzz.sh`.

Both a small dictionary and a minimized corpus with good coverage are checked into the repository. Both are compatible with each fuzzer.

## Flame graphs

Flame graphs, first created by [Brendan Gregg](https://www.brendangregg.com/flamegraphs.html), are a powerful and intuitive performance-visualization tool. This project makes use of the Rust [`flamegraph`](https://github.com/flamegraph-rs/flamegraph) crate for its straightforward flame graph support.

The helper script at `util/flamegraph.sh` allows for easy sampling of `x800`. In its default configuration, the script uses the `roger` fuzzer as the binary under test.

## License

MIT License

Copyright (c) 2024 Evelyn Lewis

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

## References

- [_ANSI escape code_, Wikipedia](https://en.wikipedia.org/wiki/ANSI_escape_code)
- [_The TTY demystified_, Linus Åkesson](http://www.linusakesson.net/programming/tty/)
- [_Zero-dependency random number generation in Rust_, Orhun Parmaksız](https://blog.orhun.dev/zero-deps-random-in-rust/)
- [_termios(3) — Linux manual page_, man7.org](https://man7.org/linux/man-pages/man3/termios.3.html)
- [_The Rust Performance Book_, Nicholas Nethercote](https://nnethercote.github.io/perf-book)
- [_Clippy's Lints_, Clippy Documentation](https://doc.rust-lang.org/stable/clippy/lints.html)
- [_hyperfine_, GitHub](https://github.com/sharkdp/hyperfine)
- [_Rust Fuzz Book_, The Rust Fuzzing Authority](https://rust-fuzz.github.io/book)
- [_LibFuzzer_, llvm.org](https://llvm.org/docs/LibFuzzer.html)
- [_cargo-fuzz_, GitHub](https://github.com/rust-fuzz/cargo-fuzz)
- [_honggfuzz_, honggfuzz.dev](https://honggfuzz.dev)
- [_honggfuzz-rs_, GitHub](https://github.com/rust-fuzz/honggfuzz-rs)
- [_Flame Graphs_, Brendan Gregg](https://www.brendangregg.com/flamegraphs.html)
