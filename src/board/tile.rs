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

/// The power-of-two of a [`Tile`]
pub type Power = u32;
/// The move number which created a [`Tile`]
pub type Generation = u64;

use super::constants::{
    BOARD_AREA, BOARD_DIMENSION, BOTTOM_EDGE, BOTTOM_LEFT_CORNER, BOTTOM_RIGHT_CORNER, EMPTY_TILE,
    LEFT_EDGE, LEFT_SPACE, RIGHT_EDGE, TOP_EDGE, TOP_LEFT_CORNER, TOP_RIGHT_CORNER,
};
use crate::colour::Colour;
use std::fmt;
use std::ops::{Index, IndexMut};

/// Types of [`Edge`]
#[derive(Clone, Copy, PartialEq, Eq)]
pub enum Edge {
    Top,
    Bottom,
    Left,
    Right,
}

/// Types of [`Corner`]
#[derive(Clone, Copy, PartialEq, Eq)]
pub enum Corner {
    TopLeft,
    TopRight,
    BottomLeft,
    BottomRight,
}

/// Basic [`super::Board`] block
#[derive(Clone, Copy, PartialEq, Eq)]
pub enum Tile {
    Empty(),
    Number(Power, Generation),
    Edge(Edge),
    Corner(Corner),
}

/// Unit tuple containing all [`Tile`]
#[derive(Clone)]
pub struct Tiles(pub [Tile; BOARD_AREA]);

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
            Self::Number(num, _) => {
                write!(
                    f,
                    "{}{:^width$}{}",
                    Colour::from_power(*num),
                    u32::pow(2, *num),
                    Colour::default(),
                    width = BOARD_DIMENSION,
                )
            }
            Self::Edge(Edge::Left) => {
                write!(f, "{LEFT_SPACE}{LEFT_EDGE}")
            }
            Self::Edge(Edge::Right) => {
                write!(f, "{RIGHT_EDGE}")
            }
            Self::Edge(Edge::Top) => {
                write!(f, "{TOP_EDGE}")
            }
            Self::Edge(Edge::Bottom) => {
                write!(f, "{BOTTOM_EDGE}")
            }
            Self::Corner(Corner::TopLeft) => {
                write!(f, "{LEFT_SPACE}{TOP_LEFT_CORNER}")
            }
            Self::Corner(Corner::TopRight) => {
                write!(f, "{TOP_RIGHT_CORNER}")
            }
            Self::Corner(Corner::BottomLeft) => {
                write!(f, "{LEFT_SPACE}{BOTTOM_LEFT_CORNER}")
            }
            Self::Corner(Corner::BottomRight) => {
                write!(f, "{BOTTOM_RIGHT_CORNER}")
            }
            Self::Empty() => {
                write!(f, "{EMPTY_TILE}")
            }
        }
    }
}
