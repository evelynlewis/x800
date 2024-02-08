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

use super::tile::{CornerSide, EdgeSide, Tile};
use super::Board;

pub(super) const EMPTY_BLOCK: &str = "      ";
pub(super) const LEFT_EDGE: &str = "│";
pub(super) const RIGHT_EDGE: &str = "│\r\n";
pub(super) const TOP_EDGE: &str = "──────";
pub(super) const BOTTOM_EDGE: &str = "──────";
pub(super) const BOTTOM_RIGHT_CORNER_SPACE: &str = "";
pub(super) const BOTTOM_RIGHT_CORNER: &str = "┘\r\n";
pub(super) const TOP_RIGHT_CORNER: &str = "┐\r\n";
pub(super) const BOTTOM_LEFT_CORNER: &str = "└";
pub(super) const TOP_LEFT_CORNER: &str = "┌";
pub const LR_EDGE_WIDTH: usize = 1;
pub const LEFT_SPACE: &str = "\r      ";
pub const SCORE: &str = "score is ";
pub const WIN_MESSAGE: &str = "- - - - you win!! - - - -\r\n";
pub const GAME_OVER: &str = "- - - - game over - - - -\r\n";
pub const GOODBYE: &str = "- - - - goodbye now - - -\r\n";
pub const WIN_POWER: u8 = 11u8;
pub const BLOCK_WIDTH: usize = 4;

pub const DEFAULT_BOARD: &Board = &Board {
    rows: [
        [
            Tile::Corner(CornerSide::TopLeft),
            Tile::Edge(EdgeSide::Top),
            Tile::Edge(EdgeSide::Top),
            Tile::Edge(EdgeSide::Top),
            Tile::Edge(EdgeSide::Top),
            Tile::Corner(CornerSide::TopRight),
        ],
        [
            Tile::Edge(EdgeSide::Left),
            Tile::Empty(),
            Tile::Empty(),
            Tile::Empty(),
            Tile::Empty(),
            Tile::Edge(EdgeSide::Right),
        ],
        [
            Tile::Edge(EdgeSide::Left),
            Tile::Empty(),
            Tile::Empty(),
            Tile::Empty(),
            Tile::Empty(),
            Tile::Edge(EdgeSide::Right),
        ],
        [
            Tile::Edge(EdgeSide::Left),
            Tile::Empty(),
            Tile::Empty(),
            Tile::Empty(),
            Tile::Empty(),
            Tile::Edge(EdgeSide::Right),
        ],
        [
            Tile::Edge(EdgeSide::Left),
            Tile::Empty(),
            Tile::Empty(),
            Tile::Empty(),
            Tile::Empty(),
            Tile::Edge(EdgeSide::Right),
        ],
        [
            Tile::Corner(CornerSide::BottomLeft),
            Tile::Edge(EdgeSide::Bottom),
            Tile::Edge(EdgeSide::Bottom),
            Tile::Edge(EdgeSide::Bottom),
            Tile::Edge(EdgeSide::Bottom),
            Tile::Corner(CornerSide::BottomRight),
        ],
    ],
    open_blocks: 0,
    score: 0,
    max_block: 0,
};
