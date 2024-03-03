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
use std::{cmp::max, fmt::Write, ops};

pub mod constants;
pub use draw::draw;

mod draw;
mod tile;

use super::colour::Colour;
use tile::Tile;

// Promote Generation type to public within this module
pub type Generation = tile::Generation;
use constants::{BOARD_DIMENSION, NUMBER_BLOCKS_PER_LINE};

type Row = [tile::Tile; BOARD_DIMENSION];

#[derive(Clone)]
pub struct Board {
    rows: [Row; BOARD_DIMENSION],
    open_blocks: u64,
    score: u64,
    max_block: u64,
}

#[derive(PartialEq, Eq, Clone, Copy)]
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
        match input as char {
            'w' => Action::Direction(Direction::Up),
            'a' => Action::Direction(Direction::Left),
            'd' => Action::Direction(Direction::Right),
            's' => Action::Direction(Direction::Down),
            '\u{3}' => Action::Shutdown,
            _ => Action::Continue,
        }
    }
}

struct UpdateStatus {
    made_move: bool,
    go_again: bool,
}

impl Board {
    // Clear screen
    pub fn draw_clear(&self, output: &mut String) -> fmt::Result {
        write!(
            output,
            "{}",
            format_args!("{}\u{1B}[2H\u{1B}[2J", Colour::default())
        )
    }

    pub fn has_space(&self) -> bool {
        self.open_blocks != 0
    }

    /// Draw board display
    #[inline]
    fn merge(
        &mut self,
        row: usize,
        column: usize,
        direction: Direction,
        gen: u64,
        update: &mut UpdateStatus,
    ) {
        let (r0, r1, c0, c1) = match direction {
            Direction::Down => (row - 1, row, column, column),
            Direction::Up => (row, row - 1, column, column),
            Direction::Right => (row, row, column - 1, column),
            Direction::Left => (row, row, column, column - 1),
        };

        let merged: Option<Tile> = self.rows[r0][c0].merge(&self.rows[r1][c1], gen);
        let previous: &Tile = &self.rows[r0][c0];
        let result = match merged {
            Some(Tile::Number(n, _)) => {
                if let Tile::Number(m, _) = previous {
                    if *m != n {
                        self.open_blocks += 1;
                        self.score += 1 << u64::from(n);
                        self.max_block = std::cmp::max(self.max_block, n);
                    }
                }
                self.rows[r0][c0] = Tile::Empty();
                self.rows[r1][c1] = merged.unwrap();
                true
            }
            Some(_) | None => false,
        };

        update.go_again |= result;
        update.made_move |= result;
    }

    // Carry out an action on the board
    #[inline]
    pub fn update(&mut self, direction: Direction, gen: tile::Generation) -> bool {
        const RANGE: ops::Range<usize> = 1..BOARD_DIMENSION;

        // Determine if we have iterated over the entire board without any updates
        // And also if the board has been changed over the course of this move
        let mut update = UpdateStatus {
            // We start without having moved the board
            made_move: false,
            // We always need a first iteration
            go_again: true,
        };

        let reverse;
        // Take appropriate action
        match direction {
            Direction::Down | Direction::Left => {
                reverse = false;
            }
            Direction::Up | Direction::Right => {
                reverse = true;
            }
        }

        if reverse {
            while update.go_again {
                update.go_again = false;
                for r in RANGE.rev() {
                    for c in RANGE.rev() {
                        self.merge(r, c, direction, gen, &mut update);
                    }
                }
            }
        } else {
            while update.go_again {
                update.go_again = false;
                for r in RANGE {
                    for c in RANGE {
                        self.merge(r, c, direction, gen, &mut update);
                    }
                }
            }
        }
        update.made_move
    }

    // Create a new '2' or '4' starting number tile
    pub fn create_new_tile(&mut self, gen: tile::Generation) {
        const CHANCE_OF_FOUR_BLOCK: u32 = 4;
        assert!(self.open_blocks > 0);

        // Collect random numbers
        // Note: use u32 for backwards compatability
        let insert_index = fastrand::u32(..self.open_blocks as u32);
        let insert_value = if fastrand::u32(..CHANCE_OF_FOUR_BLOCK) == (CHANCE_OF_FOUR_BLOCK - 1) {
            2 // '4' tile
        } else {
            1 // '2' tile
        };

        let mut current_index = 0;

        // Brute force isn't great, but it's an exceptionally small board (about 16 ops maximum)
        for r in 1..BOARD_DIMENSION {
            for c in 1..BOARD_DIMENSION {
                if self.rows[r][c] == Tile::Empty() {
                    if current_index == insert_index {
                        self.rows[r][c] = Tile::Number(insert_value, gen);
                        self.open_blocks -= 1;
                        self.max_block = max(insert_value, self.max_block);
                        // Early exit
                        return;
                    }
                    current_index += 1;
                }
            }
        }
    }

    pub fn draw_score(&self, buffer: &mut String) -> fmt::Result {
        let space = constants::LEFT_SPACE;
        let score_colour = Colour::from_power(self.max_block);
        let score_text = constants::SCORE_TEXT;
        let length = constants::DISPLAY_LINE_LENGTH;
        let no_colour = Colour::default();
        let header = if self.max_block >= constants::WIN_POWER {
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
                Colour::from_power(self.max_block),
                "",
                Colour::default(),
                colour_len = (NUMBER_BLOCKS_PER_LINE * constants::BLOCK_WIDTH)
                    + (2 * constants::LR_EDGE_WIDTH)
                    + constants::LEFT_SPACE.len()
            )
        )
    }

    fn draw_blocks(&self, buffer: &mut String) -> fmt::Result {
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
