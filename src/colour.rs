// Copyright (c) 2024-2026 Evelyn Lewis
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
use core::fmt;

/// Number of colours in use
const COLOUR_COUNT: Power = 18;

/// <https://en.wikipedia.org/wiki/ANSI_escape_code#8-bit>
const COLOUR_LEFT_COLUMN: Power = 34;

/// <https://en.wikipedia.org/wiki/ANSI_escape_code#8-bit>
const COLOUR_RIGHT_COLUMN: Power = COLOUR_LEFT_COLUMN + 5;

/// <https://en.wikipedia.org/wiki/ANSI_escape_code#8-bit>
const COLOUR_WIN_COLUMN: Power = COLOUR_LEFT_COLUMN + 6;

/// Information which fully specifies a given Colour
#[derive(PartialEq, Eq, Debug)]
pub(super) struct Colour {
    /// Column in ANSI colour chart
    column: Power,
    /// Row in ANSI colour chart
    row: Power,
}

impl Colour {
    /// Our own Default implementation to allow `const`
    pub(super) const fn default() -> Self {
        Self { row: 0, column: 0 }
    }

    /// Generate a colour based on a Tile's power-of-twok
    #[inline]
    pub(super) const fn from_power(power: Power) -> Self {
        let limited = power % COLOUR_COUNT;
        // Skip every second colour
        let offset = (power % 3) * 2;
        match limited {
            // First, left-most column, upwards (ie. /|\)
            0..=2 => Colour {
                row: 5 - offset,
                column: COLOUR_LEFT_COLUMN,
            },
            // Top-most row (ie. '>->')
            3..=5 => Colour {
                row: 0,
                column: COLOUR_LEFT_COLUMN + offset,
            },
            // Right-most column, downwards (ie. '\|/')
            6..=8 => Colour {
                row: offset,
                column: COLOUR_RIGHT_COLUMN,
            },
            // Bottom-most row (ie. '<-<')
            9..=11 => Colour {
                row: 5,
                column: COLOUR_RIGHT_COLUMN - offset,
            },
            // Across win row
            12..=14 => Colour {
                row: 5,
                column: COLOUR_WIN_COLUMN + offset,
            },
            // Up the other side
            15..=17 => Colour {
                row: 5 - offset,
                column: COLOUR_WIN_COLUMN + 5,
            },
            _ => Self::from_power(0 as Power),
        }
    }
}

impl fmt::Display for Colour {
    /// Generate and print foreground and background escape codes for a Colour
    #[inline]
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
    use super::{
        COLOUR_COUNT, COLOUR_LEFT_COLUMN, COLOUR_RIGHT_COLUMN, COLOUR_WIN_COLUMN, Colour, Power,
    };

    #[test]
    fn cycle_colours() {
        for power in 0..=u16::MAX {
            println!("{power}");
            assert_eq!(
                Colour::from_power(Power::from(power)),
                Colour::from_power(Power::from(power) % COLOUR_COUNT),
            );
        }
    }

    #[test]
    fn unique_colours() {
        for power in 0..u16::MAX {
            for skip in 1..COLOUR_COUNT {
                assert_ne!(
                    Colour::from_power(Power::from(power)),
                    Colour::from_power(Power::from(power) + skip),
                    "With {power}, skipping {skip}",
                );
            }
        }
    }

    #[test]
    #[allow(clippy::too_many_lines)]
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
                4,
                Colour {
                    row: 0,
                    column: COLOUR_LEFT_COLUMN + 2,
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
                7,
                Colour {
                    row: 2,
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
                10,
                Colour {
                    row: 5,
                    column: COLOUR_RIGHT_COLUMN - 2,
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
                    row: 5,
                    column: COLOUR_WIN_COLUMN + 2,
                },
            ),
            (
                14,
                Colour {
                    row: 5,
                    column: COLOUR_WIN_COLUMN + 4,
                },
            ),
            (
                15,
                Colour {
                    row: 5,
                    column: COLOUR_WIN_COLUMN + 5,
                },
            ),
            (
                16,
                Colour {
                    row: 3,
                    column: COLOUR_WIN_COLUMN + 5,
                },
            ),
            (
                17,
                Colour {
                    row: 1,
                    column: COLOUR_WIN_COLUMN + 5,
                },
            ),
        ]);
        for (v, c) in pairs {
            assert_eq!(Colour::from_power(v), c);
        }
    }
}
