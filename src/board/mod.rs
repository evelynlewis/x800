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
use std::{cmp, collections::BTreeSet, fmt::Write};

pub mod constants;
pub use draw::draw;

mod draw;
mod tile;

use self::constants::END_OF_GAME_CHARACTER;

use super::colour::Colour;
use tile::Tile;

// Promote Power type to public within this module
pub type Power = tile::Power;
pub type Generation = tile::Power;

use constants::{BOARD_DIMENSION, NUMBER_TILES_RANGE};

#[derive(Clone)]
pub struct Board {
    tiles: tile::Tiles,
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

    fn pair(major: usize, minor: usize, direction: Direction) -> ((usize, usize), (usize, usize)) {
        match direction {
            Direction::Left => ((major, minor), (major, minor + 1)),
            Direction::Right => ((major, minor), (major, minor - 1)),
            Direction::Up => ((minor, major), (minor + 1, major)),
            Direction::Down => ((minor, major), (minor - 1, major)),
        }
    }

    #[inline(always)]
    fn collect(
        &mut self,
        major: usize,
        minor: usize,
        direction: Direction,
        set: &mut BTreeSet<(usize, usize)>,
    ) -> bool {
        let current = Self::pair(major, minor, direction).0;
        let mut moved = false;

        match self.tiles[current] {
            Tile::Empty() => {
                set.insert(current);
            }
            Tile::Number(_, _) => {
                if !set.is_empty() {
                    let index = match direction {
                        Direction::Up | Direction::Left => set.pop_first(),
                        Direction::Down | Direction::Right => set.pop_last(),
                    }
                    .expect("Malformed set");

                    self.tiles[index] = self.tiles[current];
                    self.tiles[current] = Tile::Empty();
                    set.insert(current);
                    moved = true;
                }
            }
            Tile::Corner(_) | Tile::Edge(_) => {}
        }
        moved
    }

    #[inline(always)]
    fn merge(
        &mut self,
        major: usize,
        minor: usize,
        direction: Direction,
        set: &mut BTreeSet<(usize, usize)>,
        generation: Generation,
    ) -> bool {
        let (current, next) = Self::pair(major, minor, direction);

        match (self.tiles[current], self.tiles[next]) {
            (Tile::Number(n, a), Tile::Number(m, b)) => {
                if (n == m) && (a != generation && b != generation) {
                    self.score += 1 << (n + 1);
                    self.max_tile = cmp::max(self.max_tile, n + 1);
                    self.tiles[current] = Tile::Number(n + 1, generation);
                    self.tiles[next] = Tile::Empty();
                    set.insert(next);
                    self.open_tiles += 1;
                    return true;
                }
            }
            (Tile::Empty(), Tile::Number(_, _)) => {
                set.insert(current);
                let index = match direction {
                    Direction::Up | Direction::Left => set.pop_first().unwrap(),
                    Direction::Down | Direction::Right => set.pop_last().unwrap(),
                };
                self.tiles[index] = self.tiles[next];
                self.tiles[next] = Tile::Empty();
            }
            (Tile::Edge(_), _)
            | (Tile::Corner(_), _)
            | (_, Tile::Empty())
            | (_, Tile::Edge(_))
            | (_, Tile::Corner(_)) => {}
        }
        false
    }

    #[inline(always)]
    pub fn update(
        &mut self,
        direction: Direction,
        generation: Generation,
        set: &mut BTreeSet<(usize, usize)>,
    ) -> bool {
        let mut moved = false;

        // Iterate in row or column major order. First scan removes slack; second does one merge
        match direction {
            Direction::Left | Direction::Up => {
                const MIDDLE_TILE: usize = 2;
                for major in NUMBER_TILES_RANGE {
                    set.clear();

                    for minor in NUMBER_TILES_RANGE {
                        moved |= self.collect(major, minor, direction, set);
                    }
                    for minor in NUMBER_TILES_RANGE {
                        moved |= self.merge(major, minor, direction, set, generation);
                    }
                    // Allow a final middle merge
                    moved |= self.merge(major, MIDDLE_TILE, direction, set, generation);
                }
            }
            Direction::Right | Direction::Down => {
                const MIDDLE_TILE: usize = 3;
                for major in NUMBER_TILES_RANGE.rev() {
                    set.clear();

                    for minor in NUMBER_TILES_RANGE.rev() {
                        moved |= self.collect(major, minor, direction, set);
                    }
                    for minor in NUMBER_TILES_RANGE.rev() {
                        moved |= self.merge(major, minor, direction, set, generation);
                    }
                    // Allow a final middle merge
                    moved |= self.merge(major, MIDDLE_TILE, direction, set, generation);
                }
            }
        }
        moved
    }

    // Create a new '2' or '4' number tile in a blank space
    #[inline]
    pub fn spawn_tile(&mut self, generation: Generation) -> bool {
        if !self.has_space() {
            return false;
        }

        // Collect random numbers
        const CHANCE_OF_FOUR_TILES: u64 = 4;
        let insert_index = fastrand::u64(..self.open_tiles as u64);
        let insert_value = if fastrand::u64(..CHANCE_OF_FOUR_TILES) == (CHANCE_OF_FOUR_TILES - 1) {
            2 // '4' tile
        } else {
            1 // '2' tile
        };

        let mut cursor = 0;

        // Brute force isn't great, but it's an exceptionally small board (about 16 loops maximum)
        for r in NUMBER_TILES_RANGE {
            for c in NUMBER_TILES_RANGE {
                if (self.tiles[(r, c)]) == Tile::Empty() {
                    if cursor == insert_index {
                        self.tiles[(r, c)] = Tile::Number(insert_value, generation);
                        self.open_tiles -= 1;
                        self.max_tile = cmp::max(insert_value, self.max_tile);
                        // Return early
                        return true;
                    }
                    cursor += 1;
                }
            }
        }
        unreachable!("Failed to spawn tile");
    }
}
