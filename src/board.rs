// Copyright (c) 2024-2025 Evelyn Lewis
// SPDX-License-Identifier: MIT

//! Data structures and functions needed to implement the x800 board

use std::{cmp, vec};

pub(super) mod constants;
pub(super) mod tile;

mod draw;
use constants::TILES_AREA;

#[cfg(feature = "interactive")]
pub(super) use draw::draw;

use tile::{Generation, Tile};

/// Storage for an x800 board
#[derive(Clone)]
pub(super) struct Board {
    /// All tiles on a board, in row-major order
    tiles: [Tile; constants::TILES_AREA],
    /// Number of unoccupied tile spaces
    open_tiles: usize,
    /// Current game score
    score: u32,
    /// Value of the highest-value number tile
    max_tile: tile::Power,
    /// Generation of the board
    pub(super) generation: Generation,
}

/// Default starting [`Board`] structure
impl Board {
    pub(super) const fn default() -> Board {
        Board {
            tiles: [Tile::Empty(); constants::TILES_AREA],
            score: 0,
            open_tiles: 16,
            max_tile: 0,
            generation: 0,
        }
    }
    #[inline]
    #[cfg(feature = "interactive")]
    pub(super) fn clone_from(&mut self, source: &Self) {
        if self.generation != source.generation {
            *self = source.clone();
        }
    }
}

/// Directions the player can move
#[derive(Clone, Copy, PartialEq, Eq)]
pub(super) enum Direction {
    Up,
    Down,
    Left,
    Right,
}

/// [`Board`] operations
#[derive(Clone, Copy, PartialEq, Eq)]
pub(super) enum Action {
    Direction(Direction),
    Continue,
    Shutdown,
}

impl Action {
    #[inline]
    pub(super) const fn parse(input: u8) -> Self {
        match input {
            b'w' => Action::Direction(Direction::Up),
            b'a' => Action::Direction(Direction::Left),
            b'd' => Action::Direction(Direction::Right),
            b's' => Action::Direction(Direction::Down),
            constants::END_OF_GAME_CHARACTER => Action::Shutdown,
            _ => Action::Continue,
        }
    }
}

impl Board {
    /// Update the [`Board`], given a move [`Direction`] and its current state.
    #[allow(clippy::needless_continue)] // Avoiding `continue` below is a perf regression
    pub(super) fn update(&mut self, direction: Direction, work_line: &mut vec::Vec<Tile>) -> bool {
        const LEFT_INDEX: [[usize; constants::TILES_PER_LINE]; constants::TILES_PER_LINE] =
            [[0, 1, 2, 3], [4, 5, 6, 7], [8, 9, 10, 11], [12, 13, 14, 15]];
        const RIGHT_INDEX: [[usize; constants::TILES_PER_LINE]; constants::TILES_PER_LINE] =
            [[3, 2, 1, 0], [7, 6, 5, 4], [11, 10, 9, 8], [15, 14, 13, 12]];
        const UP_INDEX: [[usize; constants::TILES_PER_LINE]; constants::TILES_PER_LINE] =
            [[0, 4, 8, 12], [1, 5, 9, 13], [2, 6, 10, 14], [3, 7, 11, 15]];
        const DOWN_INDEX: [[usize; constants::TILES_PER_LINE]; constants::TILES_PER_LINE] =
            [[12, 8, 4, 0], [13, 9, 5, 1], [14, 10, 6, 2], [15, 11, 7, 3]];

        let index_table = match direction {
            Direction::Up => UP_INDEX,
            Direction::Down => DOWN_INDEX,
            Direction::Left => LEFT_INDEX,
            Direction::Right => RIGHT_INDEX,
        };

        // Loop bookkeeping
        let mut state_changed = false;
        let mut work_len: usize;

        for index_line in index_table {
            // Clear the vector to begin
            work_line.clear();

            // Get the effective line
            for sub_index in index_line {
                if let Tile::Number(_, _) = self.tiles[sub_index] {
                    work_line.push(self.tiles[sub_index]);
                }
            }

            // Number of tile merged
            let mut cleared: usize = 0;
            work_len = work_line.len().saturating_sub(1);

            if 0 != work_len {
                let mut a_tile: &Tile;
                let mut b_tile: &Tile;

                for a_index in 0..work_len {
                    if a_index + cleared >= work_len {
                        break;
                    }
                    a_tile = &work_line[a_index];
                    b_tile = &work_line[a_index + 1];
                    match (a_tile, b_tile) {
                        (Tile::Empty(), _) | (_, Tile::Empty()) => {
                            unreachable!();
                        }
                        (Tile::Number(a_pow, a_gen), Tile::Number(b_pow, b_gen)) => {
                            if a_pow != b_pow {
                                // Allow improved branch prediction
                                continue;
                            } else if (*a_gen != self.generation) && (*b_gen != self.generation) {
                                // New next-power block
                                let next_pow = a_pow + 1;
                                work_line[a_index] = Tile::Number(next_pow, self.generation);

                                // One fewer element
                                work_line.remove(a_index + 1);
                                cleared += 1;

                                // Update statistics
                                self.open_tiles += 1;
                                self.score += 1 << next_pow;
                                self.max_tile = cmp::max(next_pow, self.max_tile);
                            }
                        }
                    }
                }
            }

            // Re-add empty tiles to the end of the vector
            work_line.resize(constants::TILES_PER_LINE, Tile::Empty());

            // Apply to tiles
            let mut tile: Tile;
            for (line_i, tiles_i) in index_line.into_iter().enumerate() {
                tile = work_line[line_i];
                if self.tiles[tiles_i] != tile {
                    state_changed = true;
                    self.tiles[tiles_i] = tile;
                }
            }
        }
        // Clear so clone_from is trivial
        work_line.clear();

        state_changed
    }

    /// Check if the [`Board`] has one or more [`Tile::Empty`] [`Tile`]s
    #[inline]
    pub(super) fn has_space(&self) -> bool {
        self.open_tiles != 0
    }

    /// Create a new '2' or '4' number tile in a blank space
    pub(super) fn spawn_tile(&mut self) -> bool {
        // Values
        const CHANCE_OF_FOUR_TILES: u64 = 4;

        if !self.has_space() {
            return false;
        }

        // Collect random numbers
        #[allow(clippy::cast_possible_truncation)]
        let insert_index = fastrand::u64(..self.open_tiles as u64) as usize;
        let insert_value = if fastrand::u64(..CHANCE_OF_FOUR_TILES) == (CHANCE_OF_FOUR_TILES - 1) {
            2 // '4' tile
        } else {
            1 // '2' tile
        };

        // Insert at location
        let mut skipped: usize = 0;
        for index in 0..TILES_AREA {
            if let Tile::Empty() = self.tiles[index] {
                if skipped == insert_index {
                    self.tiles[index] = Tile::Number(insert_value, self.generation);
                    // Update bookkeeping variables
                    self.generation += 1;
                    self.open_tiles -= 1;
                    self.max_tile = cmp::max(insert_value, self.max_tile);
                    return true;
                }
                skipped += 1;
            }
        }

        // Should never happen since insert_index is less than open_tiles
        unreachable!("Failed to insert tile at offset {}", insert_index);
    }

    #[allow(dead_code)]
    pub(super) fn score(&self) -> u32 {
        self.score
    }
}
