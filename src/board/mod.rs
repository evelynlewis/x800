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

use core::fmt;
use std::{cmp, fmt::Write};

pub mod constants;
pub use draw::draw;

mod draw;
mod tile;

use crate::board::constants::GAME_FAILURE_MESSAGE;

use self::constants::END_OF_GAME_CHARACTER;

use super::colour::Colour;
use tile::Tile;

// Promote Generation and Power types to public within this module
pub type Generation = tile::Generation;
pub type Power = tile::Power;

use constants::{BOARD_DIMENSION, NUMBER_TILES_PER_LINE, NUMBER_TILES_RANGE};

type Row = [tile::Tile; BOARD_DIMENSION];

#[derive(Clone)]
pub struct Board {
    rows: [Row; BOARD_DIMENSION],
    score: u32,
    open_tiles: u32,
    max_tile: Power,
}

#[derive(Clone, Copy, PartialEq, Eq)]
pub(super) enum Direction {
    Up,
    Down,
    Left,
    Right,
}

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
            END_OF_GAME_CHARACTER => Action::Shutdown,
            _ => Action::Continue,
        }
    }
}

impl Board {
    // Clear screen
    #[inline(always)]
    pub fn draw_clear(&self, output: &mut String) -> fmt::Result {
        write!(
            output,
            "{}",
            format_args!("{}\u{1B}[2H\u{1B}[2J", Colour::default())
        )
    }

    #[inline(always)]
    pub fn has_space(&self) -> bool {
        self.open_tiles != 0
    }

    /// Draw board display
    #[inline(always)]
    fn merge(&mut self, row: usize, column: usize, direction: Direction, gen: Generation) -> bool {
        let (r0, r1, c0, c1) = match direction {
            Direction::Down => (row - 1, row, column, column),
            Direction::Up => (row, row - 1, column, column),
            Direction::Right => (row, row, column - 1, column),
            Direction::Left => (row, row, column, column - 1),
        };

        let merged: Option<Tile> = self.rows[r0][c0].merge(&self.rows[r1][c1], gen);
        match merged {
            Some(Tile::Number(n, _)) => {
                let previous: &Tile = &self.rows[r0][c0];
                if let Tile::Number(m, _) = previous {
                    if *m != n {
                        self.open_tiles += 1;
                        self.score += 1 << n;
                        self.max_tile = std::cmp::max(self.max_tile, n);
                    }
                }
                self.rows[r0][c0] = Tile::Empty();
                // In this branch, merged is Some()
                self.rows[r1][c1] = merged.unwrap();
                true
            }
            _ => false,
        }
    }

    // Update the board based on a directional move
    #[inline(always)]
    pub fn update(&mut self, direction: Direction, gen: tile::Generation) {
        // Determine if we have iterated over the entire board without any updates
        // And also if the board has been changed over the course of this move
        let mut again;

        // Take appropriate action
        match direction {
            Direction::Down | Direction::Left => loop {
                again = false;
                for r in NUMBER_TILES_RANGE {
                    for c in NUMBER_TILES_RANGE {
                        again |= self.merge(r, c, direction, gen);
                    }
                }
                if !again {
                    break;
                }
            },
            Direction::Up | Direction::Right => loop {
                again = false;
                for r in NUMBER_TILES_RANGE.rev() {
                    for c in NUMBER_TILES_RANGE.rev() {
                        again |= self.merge(r, c, direction, gen);
                    }
                }
                if !again {
                    break;
                }
            },
        }
    }

    // Create a new '2' or '4' number tile in a blank space
    pub fn spawn_tile(&mut self, gen: tile::Generation) -> bool {
        if !self.has_space() {
            return false;
        }

        const CHANCE_OF_FOUR_TILES: u32 = 4;
        assert_ne!(self.open_tiles, 0);

        // Collect random numbers
        // Note: use u32 for backwards compatability
        let insert_index = fastrand::u32(..self.open_tiles as u32);
        let insert_value = if fastrand::u32(..CHANCE_OF_FOUR_TILES) == (CHANCE_OF_FOUR_TILES - 1) {
            2 // '4' tile
        } else {
            1 // '2' tile
        };

        let mut cursor = 0;

        // Brute force isn't great, but it's an exceptionally small board (about 16 loops maximum)
        for r in NUMBER_TILES_RANGE {
            for c in NUMBER_TILES_RANGE {
                if self.rows[r][c] == Tile::Empty() {
                    if cursor == insert_index {
                        self.rows[r][c] = Tile::Number(insert_value, gen);
                        self.open_tiles -= 1;
                        self.max_tile = cmp::max(insert_value, self.max_tile);
                        // Early exit
                        return true;
                    }
                    cursor += 1;
                }
            }
        }
        unreachable!("{}", GAME_FAILURE_MESSAGE);
    }

    pub fn draw_score(&self, buffer: &mut String) -> fmt::Result {
        let space = constants::LEFT_SPACE;
        let score_colour = Colour::from_power(self.max_tile);
        let score_text = constants::SCORE_TEXT;
        let length = constants::DISPLAY_LINE_LENGTH;
        let no_colour = Colour::default();
        let header = if self.max_tile >= constants::WIN_POWER {
            constants::WIN_MESSAGE
        } else {
            "\r\n"
        };

        write!(
            buffer,
            "{before}{score:<length$}{after}",
            score = self.score,
            before = format_args!("{space}{header}{space}{score_colour}{score_text}"),
            after = format_args!("{no_colour}\r\n"),
        )
    }

    fn draw_header(&self, buffer: &mut String) -> fmt::Result {
        write!(
            buffer,
            "{}",
            format_args!(
                "{}{}{:<colour_len$}{}\r\n\n",
                constants::LEFT_SPACE,
                Colour::from_power(self.max_tile),
                "",
                Colour::default(),
                colour_len = (NUMBER_TILES_PER_LINE * constants::TILES_WIDTH)
                    + (2 * constants::LR_EDGE_WIDTH)
                    + constants::LEFT_SPACE.len()
            )
        )
    }

    fn draw_tiles(&self, buffer: &mut String) -> fmt::Result {
        // Allow bounds-checking elision
        assert_eq!(self.rows.len(), BOARD_DIMENSION);
        assert_eq!(self.rows[0].len(), BOARD_DIMENSION);

        // Iterate over each row and column, then print
        for i in 0..BOARD_DIMENSION {
            for j in 0..BOARD_DIMENSION {
                write!(buffer, "{}", self.rows[i][j])?;
            }
        }
        Ok(())
    }
}
