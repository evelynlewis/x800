// Copyright (c) 2024 Evelyn Lewis
// SPDX-License-Identifier: MIT

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
