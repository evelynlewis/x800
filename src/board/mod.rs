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

use std::{
    cmp::max,
    fmt, fs,
    io::{self, Read},
};

pub(super) mod constants;
mod tile;

use super::colour::Colour;
use tile::Tile;

// Promote Generation type to public within this module
pub type Generation = tile::Generation;

const NUMBER_BLOCKS_PER_LINE: usize = 4;
const BOARD_DIMENSION: usize = NUMBER_BLOCKS_PER_LINE + 2;
type Row = [tile::Tile; BOARD_DIMENSION];

#[derive(Debug, Clone)]
pub struct Board {
    rows: [Row; BOARD_DIMENSION],
    open_blocks: u8,
    score: u64,
    max_block: u8,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub(super) enum Action {
    Up,
    Down,
    Left,
    Right,
    Continue,
    Shutdown,
}

impl Action {
    pub(super) fn parse(input: char) -> Self {
        match input {
            'w' => Action::Up,
            'a' => Action::Left,
            'd' => Action::Right,
            's' => Action::Down,
            '\u{3}' => Action::Shutdown,
            _ => Action::Continue,
        }
    }
}

#[test]
fn action_parse_test() {
    for c in 0..=255u8 {
        let action = match c as char {
            'w' => Action::Up,
            'a' => Action::Left,
            's' => Action::Down,
            'd' => Action::Right,
            '\u{3}' => Action::Shutdown,
            _ => Action::Continue,
        };
        assert_eq!(Action::parse(c as char), action);
    }
}
struct UpdateStatus {
    made_move: bool,
    go_again: bool,
}

impl Board {
    /// Create a new default Board instance
    pub fn new() -> Board {
        let mut board = constants::DEFAULT_BOARD.clone();

        assert_eq!(board.rows.len(), BOARD_DIMENSION);
        assert_eq!(board.rows[0].len(), BOARD_DIMENSION);
        board.open_blocks = 0;
        board.max_block = 0;

        for i in 1..BOARD_DIMENSION {
            for j in 1..BOARD_DIMENSION {
                match board.rows[i][j] {
                    Tile::Empty() => {
                        board.open_blocks += 1;
                    }
                    Tile::Number(n, _) => {
                        board.max_block = std::cmp::max(board.max_block, n);
                    }
                    _ => {
                        continue;
                    }
                }
            }
        }
        board
    }

    // Clear screen
    #[inline]
    pub fn clear() {
        print!("{}{esc}[2H{esc}[2J", Colour::default(), esc = '\u{1B}');
    }

    #[inline]
    pub fn has_space(&self) -> bool {
        self.open_blocks != 0
    }

    /// Draw board display
    pub fn draw(&self) {
        Self::clear();
        print!("{self}");
    }

    fn merge(
        &mut self,
        r0: usize,
        c0: usize,
        r1: usize,
        c1: usize,
        gen: u64,
        update: &mut UpdateStatus,
    ) {
        let merged = self.rows[r0][c0].merge(&self.rows[r1][c1], gen);
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
    pub fn update(&mut self, action: Action, gen: tile::Generation) -> bool {
        // Determine if we have iterated over the entire board without any updates
        // And also if the board has been changed over the course of this move
        let mut update = UpdateStatus {
            // We start without having moved the board
            made_move: false,
            // We always need a first iteration
            go_again: true,
        };

        // Take appropriate action
        match action {
            Action::Left => {
                while update.go_again {
                    update.go_again = false;
                    for r in 1..BOARD_DIMENSION {
                        for c in 1..BOARD_DIMENSION {
                            self.merge(r, c, r, c - 1, gen, &mut update);
                        }
                    }
                }
            }
            Action::Up => {
                while update.go_again {
                    update.go_again = false;
                    for r in (1..BOARD_DIMENSION).rev() {
                        for c in (1..BOARD_DIMENSION).rev() {
                            self.merge(r, c, r - 1, c, gen, &mut update);
                        }
                    }
                }
            }
            Action::Right => {
                while update.go_again {
                    update.go_again = false;
                    for r in (1..BOARD_DIMENSION).rev() {
                        for c in (1..BOARD_DIMENSION).rev() {
                            self.merge(r, c - 1, r, c, gen, &mut update);
                        }
                    }
                }
            }
            Action::Down => {
                while update.go_again {
                    update.go_again = false;
                    for r in 1..BOARD_DIMENSION {
                        for c in 1..BOARD_DIMENSION {
                            self.merge(r - 1, c, r, c, gen, &mut update);
                        }
                    }
                }
            }

            Action::Continue | Action::Shutdown => {
                unreachable!("Reached invalid Action: {:?}", action);
            }
        }
        update.made_move
    }

    // Create a new '2' or '4' starting number tile
    pub fn create_new_tile(&mut self, gen: tile::Generation) -> io::Result<()> {
        const CHANCE_OF_FOUR_BLOCK: u8 = 4;
        assert!(self.open_blocks > 0);

        // Collect random numbers
        let mut rng = fs::File::open("/dev/urandom")?;
        let mut buffer = [0u8; 1];
        rng.read_exact(&mut buffer)?;
        let new_index = buffer[0] % self.open_blocks;
        rng.read_exact(&mut buffer)?;
        let new_value = if (buffer[0] % CHANCE_OF_FOUR_BLOCK) == (CHANCE_OF_FOUR_BLOCK - 1) {
            2 // '4' tile
        } else {
            1 // '2' tile
        };

        let mut scan_index: u8 = 0;

        // Brute force isn't great, but it's an exceptionally small board (about 36 ops)
        for r in 1..BOARD_DIMENSION {
            for c in 1..BOARD_DIMENSION {
                if self.rows[r][c] == Tile::Empty() {
                    if scan_index == new_index {
                        self.rows[r][c] = Tile::Number(new_value, gen);
                        self.open_blocks -= 1;
                        self.max_block = max(new_value, self.max_block);
                        return Ok(());
                    }
                    scan_index += 1;
                }
            }
        }
        Ok(())
    }

    fn fmt_score(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        if self.max_block >= constants::WIN_POWER {
            write!(f, "{}{}", constants::LEFT_SPACE, constants::WIN_MESSAGE)?;
        } else {
            write!(f, "\r\n")?;
        }
        write!(
            f,
            "{}{}{score}{:<colour_len$}{}\r\n",
            constants::LEFT_SPACE,
            Colour::from_power(self.max_block),
            self.score,
            Colour::default(),
            score = constants::SCORE,
            colour_len = (NUMBER_BLOCKS_PER_LINE * constants::BLOCK_WIDTH)
                + (2 * constants::LR_EDGE_WIDTH)
                + constants::LEFT_SPACE.len()
                - constants::SCORE.len()
        )?;
        Ok(())
    }

    fn fmt_header(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(
            f,
            "{}{}{:<colour_len$}{}\r\n\n",
            constants::LEFT_SPACE,
            Colour::from_power(self.max_block),
            "",
            Colour::default(),
            colour_len = (NUMBER_BLOCKS_PER_LINE * constants::BLOCK_WIDTH)
                + (2 * constants::LR_EDGE_WIDTH)
                + constants::LEFT_SPACE.len()
        )
    }

    fn fmt_blocks(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        // Iterate over each row and column, then print
        for i in 0..BOARD_DIMENSION {
            for j in 0..BOARD_DIMENSION {
                write!(f, "{}", self.rows[i][j])?;
            }
        }
        Ok(())
    }
}

impl fmt::Display for Board {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        self.fmt_header(f)?;
        self.fmt_blocks(f)?;
        self.fmt_score(f)?;
        Ok(())
    }
}
