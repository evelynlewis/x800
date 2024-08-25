// Copyright (c) 2024 Evelyn Lewis
// SPDX-License-Identifier: MIT

//! Simple terminal colours for x800
//!
//! Uses 256 colour ANSI escape codes. See:
//! <https://en.wikipedia.org/wiki/ANSI_escape_code#8-bit>
//!
//! We use a box-shaped segment of colours, beginning from
//! the top right corner. Chosen order is to move down the
//! right-hand column, across the bottom, up the left side,
//! and across the top.

use super::board::tile::Power;
use std::fmt;

/// Information which fully specifies a given Colour
#[derive(Debug, PartialEq, Eq, Default)]
pub struct Colour {
    row: Power,
    column: Power,
}

impl Colour {
    /// Generate a colour based on a Tile's power-of-two
    pub const fn from_power(power: Power) -> Self {
        let limited = power % 18;
        let offset = (power % 3) * 2;
        match limited {
            // first, left-most column, upwards (ie. /|\)
            0..=2 => Colour {
                row: 5 - offset,
                column: COLOUR_LEFT_COLUMN,
            },
            // top-most row (ie. '>->')
            3..=5 => Colour {
                row: 0,
                column: COLOUR_LEFT_COLUMN + offset,
            },
            // right-most column, downwards (ie. '\|/')
            6..=8 => Colour {
                row: offset,
                column: COLOUR_RIGHT_COLUMN,
            },
            // bottom-most row (ie. '<-<')
            9..=11 => Colour {
                row: 5,
                column: COLOUR_RIGHT_COLUMN - offset,
            },
            // finally, cut across the win square
            12..=17 => Colour {
                row: 5 - (limited - 12),
                column: COLOUR_WIN_COLUMN + (limited - 12),
            },
            _ => Self::from_power(0 as Power),
        }
    }
}

/// <https://en.wikipedia.org/wiki/ANSI_escape_code#8-bit>
const COLOUR_LEFT_COLUMN: Power = 34;

/// <https://en.wikipedia.org/wiki/ANSI_escape_code#8-bit>
const COLOUR_WIN_COLUMN: Power = COLOUR_LEFT_COLUMN + 6;

/// <https://en.wikipedia.org/wiki/ANSI_escape_code#8-bit>
const COLOUR_RIGHT_COLUMN: Power = COLOUR_LEFT_COLUMN + 5;

impl fmt::Display for Colour {
    /// Generate and print foreground and background escape codes for a Colour
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let (bg, fg) = if *self == Self::default() {
            (0_u32, 15_u32)
        } else {
            (self.column + (self.row * 36_u32), 0_u32)
        };

        write!(f, "\u{1B}[48;5;{bg}m\u{1B}[38;5;{fg}m")
    }
}

#[cfg(test)]
mod tests {
    use super::{Colour, Power, COLOUR_LEFT_COLUMN, COLOUR_RIGHT_COLUMN, COLOUR_WIN_COLUMN};

    #[test]
    fn cycle_colours() {
        for power in 0..=u16::MAX {
            println!("{power}");
            assert_eq!(
                Colour::from_power(power as Power),
                Colour::from_power(power as Power % 18),
            );
        }
    }

    #[test]
    fn unique_colours() {
        for power in 0..u16::MAX {
            for skip in 1..=17 {
                assert_ne!(
                    Colour::from_power(power as Power),
                    Colour::from_power(power as Power + skip),
                    "With {power}, skipping {skip}",
                );
            }
        }
    }

    #[test]
    fn parameterized_colour_values() {
        let pairs = std::collections::HashMap::from([
            (
                0,
                Colour {
                    row: 5,
                    column: COLOUR_LEFT_COLUMN,
                },
            ),
            (
                1,
                Colour {
                    row: 3,
                    column: COLOUR_LEFT_COLUMN,
                },
            ),
            (
                2,
                Colour {
                    row: 1,
                    column: COLOUR_LEFT_COLUMN,
                },
            ),
            (
                3,
                Colour {
                    row: 0,
                    column: COLOUR_LEFT_COLUMN,
                },
            ),
            (
                5,
                Colour {
                    row: 0,
                    column: COLOUR_LEFT_COLUMN + 4,
                },
            ),
            (
                6,
                Colour {
                    row: 0,
                    column: COLOUR_RIGHT_COLUMN,
                },
            ),
            (
                8,
                Colour {
                    row: 4,
                    column: COLOUR_RIGHT_COLUMN,
                },
            ),
            (
                9,
                Colour {
                    row: 5,
                    column: COLOUR_RIGHT_COLUMN,
                },
            ),
            (
                11,
                Colour {
                    row: 5,
                    column: COLOUR_RIGHT_COLUMN - 4,
                },
            ),
            (
                12,
                Colour {
                    row: 5,
                    column: COLOUR_WIN_COLUMN,
                },
            ),
            (
                13,
                Colour {
                    row: 4,
                    column: COLOUR_WIN_COLUMN + 1,
                },
            ),
            (
                17,
                Colour {
                    row: 0,
                    column: COLOUR_WIN_COLUMN + 5,
                },
            ),
        ]);
        for (v, c) in pairs {
            assert_eq!(Colour::from_power(v), c);
        }
    }
}
