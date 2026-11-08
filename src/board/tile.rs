// Copyright (c) 2024-2026 Evelyn Lewis
// SPDX-License-Identifier: MIT

//! Data structures and functions needed to define, index, and display a [Tile]

use super::constants;
use crate::colour::Colour;

/// The power-of-two of a [`Tile`]
pub(crate) type Power = u32;
/// The move number which created a [`Tile`]
pub(super) type Generation = u64;

/// Basic [`super::Board`] block
#[derive(Clone, Copy, PartialEq, Eq)]
pub(crate) enum Tile {
    Empty,
    Number(Power, Generation),
}

impl std::fmt::Display for Tile {
    /// Format [`Tile`] to be displayed
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        use constants::EMPTY_TILE;
        match self {
            Self::Number(num, _) => {
                write!(
                    f,
                    "{}{:^width$}{}",
                    Colour::from_power(*num),
                    u32::pow(2, *num),
                    Colour::default(),
                    width = EMPTY_TILE.len(),
                )
            }
            Self::Empty => {
                write!(f, "{EMPTY_TILE}")
            }
        }
    }
}
