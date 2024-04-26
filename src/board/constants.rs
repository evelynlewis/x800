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

//! Various constants used by [Board]

use super::tile::Power;
use super::tile::{Corner, Edge, Tile, Tiles};
use super::Board;

// Internal constants
/// Text printed for an empty [`Tile`] in the board
pub(super) const EMPTY_TILE: &str = "      ";
/// Text printed for the left edge of the board
pub(super) const LEFT_EDGE: &str = "│";
/// Text printed for the right edge of the board
pub(super) const RIGHT_EDGE: &str = "│\r\n";
/// Text printed for the top edge of the board
pub(super) const TOP_EDGE: &str = "──────";
/// Text printed for the bottom edge of the board
pub(super) const BOTTOM_EDGE: &str = "──────";
/// Text printed at the bottom-right corner
pub(super) const BOTTOM_RIGHT_CORNER: &str = "┘\r\n";
/// Text printed at the top-right corner
pub(super) const TOP_RIGHT_CORNER: &str = "┐\r\n";
/// Text printed at the bottom-left corner
pub(super) const BOTTOM_LEFT_CORNER: &str = "└";
/// Text printed at the top-left corner
pub(super) const TOP_LEFT_CORNER: &str = "┌";
/// Width of the left and right [`Board`] edges
pub(super) const LR_EDGE_WIDTH: usize = 1;
/// Text printed to indicate score
pub(super) const SCORE_TEXT: &str = "score is ";
/// Message printed after merging a [`Tile`] WIN_POWER times
pub(super) const WIN_MESSAGE: &str = "- - - - you win!! - - - -\r\n";
/// Power-of-two required to win the game
pub(super) const WIN_POWER: Power = 11;
/// The width of a single [`Tile`]
pub(super) const TILES_WIDTH: usize = 4;
/// The number of [Tile]s in a given [`Board`] row or column
pub(super) const BOARD_DIMENSION: usize = NUMBER_TILES_PER_LINE + 2 * LR_EDGE_WIDTH;
/// The area of playable tiles
pub(super) const BOARD_AREA: usize = BOARD_DIMENSION * BOARD_DIMENSION;
pub(super) const NUMBER_TILES_RANGE: std::ops::Range<usize> =
    LR_EDGE_WIDTH..(LR_EDGE_WIDTH + NUMBER_TILES_PER_LINE);

// Public constants
/// The number of [`Tile`]s per row or column
pub const NUMBER_TILES_PER_LINE: usize = 4;
/// Text printed to the left of the board
pub const LEFT_SPACE: &str = "\r      ";
/// Text printed to mark the end of a game
pub const GAME_OVER: &str = "- - - - game over - - - -\r\n";
// Input character which ends gameplay
pub const END_OF_GAME_CHARACTER: u8 = '\u{3}' as u8;
// Length of a complete output line, as-displayed, and including borders and whitespace
pub const DISPLAY_LINE_LENGTH: usize =
    (NUMBER_TILES_PER_LINE * TILES_WIDTH) + (2 * LR_EDGE_WIDTH) + LEFT_SPACE.len()
        - SCORE_TEXT.len();

/// Default starting [`Board`] structure
pub const DEFAULT_BOARD: &Board = &Board {
    tiles: Tiles([
        Tile::Corner(Corner::TopLeft),
        Tile::Edge(Edge::Top),
        Tile::Edge(Edge::Top),
        Tile::Edge(Edge::Top),
        Tile::Edge(Edge::Top),
        Tile::Corner(Corner::TopRight),
        Tile::Edge(Edge::Left),
        Tile::Empty(),
        Tile::Empty(),
        Tile::Empty(),
        Tile::Empty(),
        Tile::Edge(Edge::Right),
        Tile::Edge(Edge::Left),
        Tile::Empty(),
        Tile::Empty(),
        Tile::Empty(),
        Tile::Empty(),
        Tile::Edge(Edge::Right),
        Tile::Edge(Edge::Left),
        Tile::Empty(),
        Tile::Empty(),
        Tile::Empty(),
        Tile::Empty(),
        Tile::Edge(Edge::Right),
        Tile::Edge(Edge::Left),
        Tile::Empty(),
        Tile::Empty(),
        Tile::Empty(),
        Tile::Empty(),
        Tile::Edge(Edge::Right),
        Tile::Corner(Corner::BottomLeft),
        Tile::Edge(Edge::Bottom),
        Tile::Edge(Edge::Bottom),
        Tile::Edge(Edge::Bottom),
        Tile::Edge(Edge::Bottom),
        Tile::Corner(Corner::BottomRight),
    ]),
    open_tiles: 16,
    score: 0,
    max_tile: 0,
};

// Print output to stdout
#[cfg(not(feature = "sink"))]
/// Function to create a writer object
pub const WRITER: fn() -> std::io::Stdout = std::io::stdout;

// Send all output to a null sink
#[cfg(feature = "sink")]
/// Function to create a writer object
pub const WRITER: fn() -> std::io::Sink = std::io::sink;

/// Size of a complete buffer needed to print a [`Board`] frame to the terminal
pub(super) const DISPLAY_BUFFER_SIZE: usize = (BOARD_DIMENSION + 6) * (BOARD_DIMENSION + 4);
/// Message printed on various game failures
pub const GAME_FAILURE_MESSAGE: &str = " an ongoing game problem \r\n";
