# `ryan`: a lean, fast, and reproducible `x800` driver

## Introduction

```sh
usage:
    ryan FILE [LOOPS]

    FILE: path to x800 save
    LOOPS: iterate LOOPS times
```

Given some `x800` save file `FILE`, `ryan` will run a deterministic `x800` game using the contents of `FILE`.

The optional `LOOPS` argument runs complete games using the contents of `FILE` repeatedly, `LOOPS` times.

### Example

```sh
ryan ./ig-save.bin 2048
```

## `x800` save files

Building `x800` with the `save` feature, for example, using `cargo build --features save`, will cause `x800` to create a save file at program exit.

This file will be named according to the regular expression `x800-game-save-[0-9]+.bin`, and contains the initial game seed and all subsequent moves. In this way, the save file allows fully reproducing a complete game.

### Interoperability

The `x800` save file format is also used for all `x800` fuzzers. This allows for:

- Easily checking the result of a fuzz test case with `ryan`.
- Creating new seed cases for the fuzz corpus using the `x800` `save` feature.
- Re-running previous `x800` games with `ryan`.
