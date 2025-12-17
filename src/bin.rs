// Copyright (c) 2024 Evelyn Lewis
// SPDX-License-Identifier: MIT

//! A minimal program entry point

mod board;
mod clock;
mod colour;
mod play;

/// The main program entry point
fn main() {
    play::play(&play::Input::Stdin);
}
