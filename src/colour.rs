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

/// colour.rs
///
/// Simple terminal colours for c2048
///
/// Uses 256 colour ANSI escape codes. See:
/// <https://en.wikipedia.org/wiki/ANSI_escape_code#8-bit>
///
/// We use a box-shaped segment of colours, beginning from
/// the top right corner. Chosen order is to move down the
/// right-hand column, across the bottom, up the left side,
/// and across the top.
use std::{default::Default, fmt};

#[derive(Clone, Debug, PartialEq, Eq)]
pub struct Colour {
    pub empty: bool,
    pub row: u8,
    pub column: u8,
}

impl Default for Colour {
    fn default() -> Self {
        Colour {
            empty: true,
            row: 0,
            column: 0,
        }
    }
}

#[test]
fn power_to_colour_zero_test() {
    assert_eq!(Colour::from_power(0), Colour::default());
}

impl Colour {
    pub fn from_power(power: u8) -> Self {
        match power {
            // empty colour
            0 => Colour {
                empty: true,
                row: 0,
                column: 0,
            },
            // first, left-most column, upwards (ie. /|\)
            1..=3 => Colour {
                empty: false,
                row: 5 - ((power - 1) * 2),
                column: COLOUR_LEFT_COLUMN,
            },
            // top-most row (ie. '>->')
            4..=6 => Colour {
                empty: false,
                row: 0,
                column: COLOUR_LEFT_COLUMN + ((power - 4) * 2),
            },
            // right-most column, downwards (ie. '\|/')
            7..=9 => Colour {
                empty: false,
                row: (power - 7) * 2,
                column: COLOUR_RIGHT_COLUMN,
            },
            // bottom-most row (ie. '<-<')
            10..=12 => Colour {
                empty: false,
                row: 5,
                column: COLOUR_RIGHT_COLUMN + ((power - 10) * 2),
            },
            // above is a cycle, so use modulus
            _ => Self::from_power(power % 13),
        }
    }
}

const COLOUR_LEFT_COLUMN: u8 = 34 - 16;
const COLOUR_RIGHT_COLUMN: u8 = COLOUR_LEFT_COLUMN + 5;

#[test]
fn power_to_colour_cycle_test() {
    for power in 0..=255u8 {
        assert_eq!(Colour::from_power(power), Colour::from_power(power % 13));
    }
}

impl fmt::Display for Colour {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let bg: u32;
        let fg: u32;
        if self.empty {
            bg = 0;
            fg = 15;
        } else {
            bg = 16 + u32::from(self.column) + (u32::from(self.row) * 36);
            fg = 0;
        }
        write!(f, "{esc}[48;5;{}m{esc}[38;5;{}m", bg, fg, esc = '\u{1B}')
    }
}
