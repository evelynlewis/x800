/*
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
*/

//! Data structures and functions needed to define, index, and display a [Tile]

use super::constants;
use crate::colour::Colour;
use std::fmt;
use std::hash::Hash;

/// The power-of-two of a [`Tile`]
pub type Power = u32;
/// The move number which created a [`Tile`]
pub type Generation = u64;

/// Basic [`super::Board`] block
#[derive(Clone, Copy, PartialEq, Eq, Debug, Hash)]
pub enum Tile {
    Empty(),
    Number(Power, Generation),
}

impl fmt::Display for Tile {
    /// Format [`Tile`] to be displayed
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
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
            Self::Empty() => {
                write!(f, "{EMPTY_TILE}")
            }
        }
    }
}
