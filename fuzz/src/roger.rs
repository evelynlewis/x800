// Copyright (c) 2024 Evelyn Lewis
// SPDX-License-Identifier: MIT

#![no_main]
#[macro_use]
extern crate libfuzzer_sys;

// Call the shared fuzz driver
fuzz_target!(|data: &[u8]| x800::one_shot(data));
