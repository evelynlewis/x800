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
