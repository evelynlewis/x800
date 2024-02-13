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

pub(super) type Generation = u64;
pub(super) type Power = u8;

use super::constants::{
    BOTTOM_EDGE, BOTTOM_LEFT_CORNER, BOTTOM_RIGHT_CORNER, BOTTOM_RIGHT_CORNER_SPACE, EMPTY_BLOCK,
    LEFT_EDGE, LEFT_SPACE, RIGHT_EDGE, TOP_EDGE, TOP_LEFT_CORNER, TOP_RIGHT_CORNER,
};
use super::BOARD_DIMENSION;
use crate::colour::Colour;
use std::fmt;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub(super) enum Tile {
    Empty(),
    Number(Power, Generation),
    Edge(EdgeSide),
    Corner(CornerSide),
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub(super) enum EdgeSide {
    Top,
    Bottom,
    Left,
    Right,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub(super) enum CornerSide {
    TopLeft,
    TopRight,
    BottomLeft,
    BottomRight,
}

impl Tile {
    pub(super) fn merge(self, other: &Self, gen: Generation) -> Option<Self> {
        match (self, other) {
            // Merge two similar number blocks
            (Self::Number(n, j), Self::Number(m, k)) if n == *m && ((j != gen) && (*k != gen)) => {
                Some(Self::Number(n + 1, gen))
            }
            // Shift number tile to fill space
            (Self::Number(n, j), Self::Empty()) => Some(Self::Number(n, j)),
            // Anything else
            (_, _) => None,
        }
    }
}

impl fmt::Display for Tile {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::Number(num, _) => {
                write!(
                    f,
                    "{}{:^width$}{}",
                    Colour::from_power(*num),
                    u64::pow(2, u32::from(*num)),
                    Colour::default(),
                    width = BOARD_DIMENSION,
                )
            }
            Self::Edge(EdgeSide::Left) => {
                write!(f, "{LEFT_SPACE}{LEFT_EDGE}")
            }
            Self::Edge(EdgeSide::Right) => {
                write!(f, "{RIGHT_EDGE}")
            }
            Self::Edge(EdgeSide::Top) => {
                write!(f, "{TOP_EDGE}")
            }
            Self::Edge(EdgeSide::Bottom) => {
                write!(f, "{BOTTOM_EDGE}")
            }
            Self::Corner(CornerSide::TopLeft) => {
                write!(f, "{LEFT_SPACE}{TOP_LEFT_CORNER}")
            }
            Self::Corner(CornerSide::TopRight) => {
                write!(f, "{TOP_RIGHT_CORNER}")
            }
            Self::Corner(CornerSide::BottomLeft) => {
                write!(f, "{LEFT_SPACE}{BOTTOM_LEFT_CORNER}")
            }
            Self::Corner(CornerSide::BottomRight) => {
                write!(f, "{BOTTOM_RIGHT_CORNER}{BOTTOM_RIGHT_CORNER_SPACE}")
            }
            Self::Empty() => {
                write!(f, "{EMPTY_BLOCK}")
            }
        }
    }
}
