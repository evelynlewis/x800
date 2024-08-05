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
        let limited = power % 12;
        match limited {
            // first, left-most column, upwards (ie. /|\)
            0..=2 => Colour {
                row: 5 - (limited * 2),
                column: COLOUR_LEFT_COLUMN,
            },
            // top-most row (ie. '>->')
            3..=5 => Colour {
                row: 0,
                column: COLOUR_LEFT_COLUMN + ((limited - 3) * 2),
            },
            // right-most column, downwards (ie. '\|/')
            6..=8 => Colour {
                row: (limited - 6) * 2,
                column: COLOUR_RIGHT_COLUMN,
            },
            // bottom-most row (ie. '<-<')
            9..=11 => Colour {
                row: 5,
                column: COLOUR_RIGHT_COLUMN + ((limited - 9) * 2),
            },
            _ => Self::from_power(0 as Power),
        }
    }
}

/// See <https://en.wikipedia.org/wiki/ANSI_escape_code#8-bit>
const COLOUR_LEFT_COLUMN: Power = 34 - 16;
/// See <https://en.wikipedia.org/wiki/ANSI_escape_code#8-bit>
const COLOUR_RIGHT_COLUMN: Power = COLOUR_LEFT_COLUMN + 5;

#[test]
fn power_to_colour_cycle_test() {
    for power in 0..=u16::MAX {
        assert_eq!(
            Colour::from_power(power as Power),
            Colour::from_power(power as Power % 12)
        );
    }
}

#[test]
fn power_to_colour_difference() {
    for power in 0..u16::MAX {
        assert_ne!(
            Colour::from_power(power as Power),
            Colour::from_power(power as Power + 1)
        );
    }
}

impl fmt::Display for Colour {
    /// Generate and print foreground and background escape codes for a Colour
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let (bg, fg) = if *self == Self::default() {
            (0_u32, 15_u32)
        } else {
            (16_u32 + self.column + (self.row * 36_u32), 0_u32)
        };

        write!(f, "\u{1B}[48;5;{bg}m\u{1B}[38;5;{fg}m")
    }
}
