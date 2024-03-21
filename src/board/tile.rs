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

pub(super) type Power = u32;

use super::constants::{
    BOTTOM_EDGE, BOTTOM_LEFT_CORNER, BOTTOM_RIGHT_CORNER, BOTTOM_RIGHT_CORNER_SPACE, EMPTY_TILE,
    LEFT_EDGE, LEFT_SPACE, RIGHT_EDGE, TOP_EDGE, TOP_LEFT_CORNER, TOP_RIGHT_CORNER,
};
use super::BOARD_DIMENSION;
use crate::colour::Colour;
use std::fmt;
use std::ops::{Index, IndexMut};

#[derive(Clone, Copy, PartialEq, Eq)]
pub(super) enum EdgeSide {
    Top,
    Bottom,
    Left,
    Right,
}

#[derive(Clone, Copy, PartialEq, Eq)]
pub(super) enum CornerSide {
    TopLeft,
    TopRight,
    BottomLeft,
    BottomRight,
}

#[derive(Clone, Copy, PartialEq, Eq)]
pub enum Tile {
    Empty(),
    Number(Power),
    Edge(EdgeSide),
    Corner(CornerSide),
}

#[derive(Clone)]
pub struct Tiles(pub [Tile; BOARD_DIMENSION * BOARD_DIMENSION]);

impl Index<(usize, usize)> for Tiles {
    type Output = Tile;

    fn index(&self, index: (usize, usize)) -> &Self::Output {
        &(self.0[index.0 * BOARD_DIMENSION + index.1])
    }
}

impl IndexMut<(usize, usize)> for Tiles {
    fn index_mut(&mut self, index: (usize, usize)) -> &mut Self::Output {
        &mut (self.0[index.0 * BOARD_DIMENSION + index.1])
    }
}

impl fmt::Display for Tile {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::Number(num) => {
                write!(
                    f,
                    "{}{:^width$}{}",
                    Colour::from_power(*num),
                    u32::pow(2, *num),
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
                write!(f, "{EMPTY_TILE}")
            }
        }
    }
}
