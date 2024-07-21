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

use super::tile::{Power, Tile};
use super::Board;
use std::io;

// Internal constants
/// Text printed for an empty [`Tile`] in the board
pub(super) const EMPTY_TILE: &str = "      ";
/// Text printed to indicate score
pub(super) const SCORE_TEXT: &str = "score is ";
/// Message printed after merging a [`Tile`] [`WIN_POWER`] times
pub(super) const WIN_MESSAGE: &str = "─────── you win! ───────\r\n";
/// Power-of-two required to win the game
pub(super) const WIN_POWER: Power = 11;
/// The width of a single [`Tile`]
pub(super) const TILES_WIDTH: usize = 4;
/// The number of [Tile]s in a given [`Board`] row or column
pub(super) const TILES_PER_LINE: usize = 4;
/// The area of playable tiles
pub(super) const TILES_AREA: usize = TILES_PER_LINE * TILES_PER_LINE;
// Length of a complete output line, as-displayed, and including borders and whitespace
pub(super) const PRINT_LINE_LEN: usize = (TILES_PER_LINE * TILES_WIDTH) + PRE_MARGIN.len();

// Public constants
/// Text printed to the left of the board
pub const PRE_MARGIN: &str = "\r\r      ";
/// Text printed to mark the end of a game
pub const GAME_OVER: &str = "─── the game is over ───\r\n";
// Input character which ends gameplay
pub const END_OF_GAME_CHARACTER: u8 = 3;

/// Default starting [`Board`] structure
pub const DEFAULT_BOARD: &Board = &Board {
    tiles: [Tile::Empty(); TILES_AREA],
    open_tiles: 16,
    score: 0,
    max_tile: 0,
};

// Print output to stdout
#[cfg(not(feature = "sink"))]
/// Function to create a writer object
pub const WRITER: fn() -> io::Stdout = io::stdout;

// Send all output to a null sink
#[cfg(feature = "sink")]
/// Function to create a writer object
pub const WRITER: fn() -> io::Sink = io::sink;

/// Size of a complete buffer needed to print a [`Board`] frame to the terminal
pub(super) const DISPLAY_BUFFER_SIZE: usize = (TILES_PER_LINE + 6) * (TILES_PER_LINE + 4);
/// Message printed on various game failures
pub const GAME_FAILURE_MESSAGE: &str = " an ongoing game problem \r\n";
