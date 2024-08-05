// Copyright (c) 2024 Evelyn Lewis
// SPDX-License-Identifier: MIT

fn main() {
    loop {
        honggfuzz::fuzz!(|data: &[u8]| { x800::one_shot(data) });
    }
}
