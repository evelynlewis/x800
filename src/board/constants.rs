// Copyright (c) 2024-2026 Evelyn Lewis
// SPDX-License-Identifier: MIT

//! Various constants used by [`super::Board`]

// Needed since some constants are only used interactively
#![cfg_attr(not(feature = "draw"), allow(unused_imports))]
#![cfg_attr(not(feature = "draw"), allow(dead_code))]

use super::tile::Power;

// Internal constants
/// Text printed for an empty [`super::Tile`] in the board
pub(super) const EMPTY_TILE: &str = "      ";
/// Text printed to indicate score
pub(super) const SCORE_TEXT: &str = "score is ";
/// Message printed after merging a [`super::Tile`] [`WIN_POWER`] times
pub(super) const WIN_MESSAGE: &str = "─────── you win! ───────\r\n";
/// Power-of-two required to win the game
pub(super) const WIN_POWER: Power = 11;
/// The width of a single [`super::Tile`]
pub(super) const TILES_WIDTH: usize = 4;
/// The number of [`super::Tile`]s in a given [`super::Board`] row or column
pub(super) const TILES_PER_LINE: usize = 4;
/// The area of playable tiles
pub(super) const TILES_AREA: usize = TILES_PER_LINE * TILES_PER_LINE;
/// Length of a complete output line, as-displayed, and including borders and whitespace
pub(super) const PRINT_LINE_LEN: usize = (TILES_PER_LINE * TILES_WIDTH) + PRE_MARGIN.len();
/// Marker line used at top and bottom
pub(super) const BOUNDARY_MARKER_LINE: &str = "────────────────────────";
/// Text printed to the left of the board
pub(super) const PRE_MARGIN: &str = "\r\r      ";
/// Text printed to mark the end of a game
pub(super) const GAME_OVER: &str = "─── the game is over ───\r\n";

// Print output to stdout
#[cfg(not(feature = "sink"))]
/// Function to create a writer object
pub(super) const WRITER: fn() -> std::io::Stdout = std::io::stdout;

// Send all output to a null sink
#[cfg(feature = "sink")]
/// Function to create a writer object
pub(super) const WRITER: fn() -> std::io::Sink = std::io::sink;

// Public constants
/// Input character which ends gameplay
pub(crate) const END_OF_GAME_CHARACTER: u8 = 3;
/// Size of a complete buffer needed to print a [`super::Board`] frame to the terminal
pub(crate) const DISPLAY_BUFFER_SIZE: usize = (TILES_PER_LINE + 6) * (TILES_PER_LINE + 4);
/// Message printed on various game failures
pub(crate) const GAME_FAILURE_MESSAGE: &str = " an ongoing game problem \r\n";
