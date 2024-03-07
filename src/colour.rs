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

use super::board::Power;
/// Simple terminal colours for x800
///
/// Uses 256 colour ANSI escape codes. See:
/// <https://en.wikipedia.org/wiki/ANSI_escape_code#8-bit>
///
/// We use a box-shaped segment of colours, beginning from
/// the top right corner. Chosen order is to move down the
/// right-hand column, across the bottom, up the left side,
/// and across the top.
use std::fmt;

#[derive(Debug, PartialEq, Eq, Default)]
pub struct Colour {
    pub row: Power,
    pub column: Power,
}

impl Colour {
    #[inline(always)]
    pub const fn from_power(power: Power) -> Self {
        match power {
            // Blank colour
            0 => Colour { row: 0, column: 0 },
            // first, left-most column, upwards (ie. /|\)
            1..=3 => Colour {
                row: 5 - ((power - 1) * 2),
                column: COLOUR_LEFT_COLUMN,
            },
            // top-most row (ie. '>->')
            4..=6 => Colour {
                row: 0,
                column: COLOUR_LEFT_COLUMN + ((power - 4) * 2),
            },
            // right-most column, downwards (ie. '\|/')
            7..=9 => Colour {
                row: (power - 7) * 2,
                column: COLOUR_RIGHT_COLUMN,
            },
            // bottom-most row (ie. '<-<')
            10..=12 => Colour {
                row: 5,
                column: COLOUR_RIGHT_COLUMN + ((power - 10) * 2),
            },
            // above is a cycle, so use modulus
            _ => Self::from_power(power % 13),
        }
    }
}

const COLOUR_LEFT_COLUMN: Power = 34 - 16;
const COLOUR_RIGHT_COLUMN: Power = COLOUR_LEFT_COLUMN + 5;

#[test]
fn power_to_colour_zero_test() {
    assert_eq!(Colour::from_power(0), Colour::default());
}

#[test]
fn power_to_colour_cycle_test() {
    for power in 0..=255_u8 {
        assert_eq!(
            Colour::from_power(power as Power),
            Colour::from_power(power as Power % 13)
        );
    }
}

impl fmt::Display for Colour {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let (bg, fg) = if *self == Self::default() {
            (0_u32, 15_u32)
        } else {
            (
                16_u32 + self.column as u32 + (self.row as u32 * 36_u32),
                0_u32,
            )
        };

        write!(f, "\u{1B}[48;5;{bg}m\u{1B}[38;5;{fg}m")
    }
}
