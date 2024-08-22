// Copyright (c) 2024 Evelyn Lewis
// SPDX-License-Identifier: MIT

//! Data structures and functions needed to implement the x800 board

use std::{cmp, vec};

pub mod constants;
mod draw;
pub use draw::draw;
pub mod tile;
use tile::{Generation, Tile};

/// Storage for an x800 board
#[derive(Clone)]
pub struct Board {
    /// All tiles on a board, in row-major order
    tiles: [Tile; constants::TILES_AREA],
    /// Storage used during board operations
    line: vec::Vec<Tile>,
    /// Current game score
    score: u64,
    /// Number of unoccupied tile spaces
    open_tiles: usize,
    /// Value of the highest-value number tile
    max_tile: tile::Power,
}

/// Default starting [`Board`] structure
impl Board {
    pub const fn default() -> Board {
        Board {
            tiles: [Tile::Empty(); constants::TILES_AREA],
            line: vec![],
            score: 0,
            open_tiles: 16,
            max_tile: 0,
        }
    }
}

/// Directions the player can move
#[derive(Clone, Copy, PartialEq, Eq, Debug)]
pub(super) enum Direction {
    Up,
    Down,
    Left,
    Right,
}

/// [`Board`] operations
#[derive(PartialEq, Eq)]
pub(super) enum Action {
    Direction(Direction),
    Continue,
    Shutdown,
}

impl Action {
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
    /// Update the [`Board`], given a move [`Direction`] and [`Generation`], and its current state.
    pub fn update(&mut self, direction: Direction, generation: Generation) -> bool {
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
        let mut end_index: usize;
        let mut index: usize;
        let mut line_tile: Tile;

        for index_line in index_table {
            // Clear the vector to begin
            self.line.clear();

            // Get the effective line
            for sub_index in index_line {
                if let Tile::Number(_, _) = self.tiles[sub_index] {
                    self.line.push(self.tiles[sub_index]);
                }
            }

            end_index = self.line.len().saturating_sub(1);
            index = 0;

            // While there are still elements at (index, index + 1)
            while index < end_index {
                let (a_index, b_index) = (index, index + 1);
                match (self.line[a_index], self.line[b_index]) {
                    (Tile::Empty(), _) | (_, Tile::Empty()) => {
                        unreachable!();
                    }
                    (Tile::Number(a_pow, a_gen), Tile::Number(b_pow, b_gen)) => {
                        if a_pow == b_pow && ((a_gen != generation) && (b_gen != generation)) {
                            // New next-power block
                            let next_pow = a_pow + 1;
                            self.line[a_index] = Tile::Number(next_pow, generation);

                            // One fewer element
                            self.line.remove(b_index);
                            end_index -= 1;

                            // Update statistics
                            self.open_tiles += 1;
                            self.score += 1 << next_pow;
                            self.max_tile = cmp::max(next_pow, self.max_tile);
                        }
                    }
                }
                index += 1;
            }

            // Re-add empty tiles to the end of the vector
            self.line.resize(constants::TILES_PER_LINE, Tile::Empty());

            // Apply to tiles
            for (line_i, tiles_i) in index_line.into_iter().enumerate() {
                line_tile = self.line[line_i];
                if self.tiles[tiles_i] != line_tile {
                    state_changed = true;
                    self.tiles[tiles_i] = line_tile;
                }
            }
        }
        // Clear so clone_from is trivial
        self.line.clear();

        state_changed
    }

    /// Check if the [`Board`] has one or more [`Tile::Empty`] [`Tile`]s
    #[inline]
    pub fn has_space(&self) -> bool {
        self.open_tiles != 0
    }

    /// Create a new '2' or '4' number tile in a blank space
    pub fn spawn_tile(&mut self, generation: Generation) -> bool {
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
        let mut offset: usize = 0;
        for tile in &mut self.tiles {
            if *tile == Tile::Empty() {
                if offset == insert_index {
                    *tile = Tile::Number(insert_value, generation);
                    self.max_tile = cmp::max(insert_value, self.max_tile);
                    self.open_tiles -= 1;
                    return true;
                }
                offset += 1;
            }
        }

        // Should never happen since insert_index is less than open_tiles
        unreachable!("Failed to insert tile at offset {}", insert_index);
    }

    #[allow(dead_code)]
    pub fn score(&self) -> u64 {
        self.score
    }
}
