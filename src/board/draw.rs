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

//! Implementation of [Board] drawing

use super::super::colour::Colour;
use super::constants::{self, BOARD_AREA, BOARD_DIMENSION, NUMBER_TILES_PER_LINE};
use super::Board;
use std::{
    fmt,
    fmt::Write as FmtWrite,
    io::Write as IoWrite,
    sync::{self, atomic},
    thread,
};

/// Board drawing worker function
pub fn draw(
    board: sync::Arc<sync::Mutex<Board>>,
    done: &sync::Arc<atomic::AtomicBool>,
) -> fmt::Result {
    // Use one buffer for program duration
    let buffer = &mut String::with_capacity(constants::DISPLAY_BUFFER_SIZE);

    // Preferred output stream
    let mut output = constants::WRITER();

    // Our own personal local board
    let mut cloned;

    loop {
        // Wait for wakeup
        thread::park();

        // Clone for drawing
        cloned = board.lock().unwrap().clone();

        // Write out to string
        cloned.draw_clear(buffer)?;
        cloned.draw_header(buffer)?;
        cloned.draw_tiles(buffer)?;
        cloned.draw_score(buffer)?;

        // Write out framebuffer
        write!(output, "{}", buffer).expect("Failed to render");
        output.flush().expect("failed to flush board render");
        buffer.clear();

        // Leave the loop
        if done.load(atomic::Ordering::Relaxed) {
            break;
        }
    }

    Ok(())
}

impl Board {
    /// Draw a 'clear screen' control character to the output String
    pub fn draw_clear(&self, output: &mut String) -> fmt::Result {
        write!(
            output,
            "{}",
            format_args!("{}\u{1B}[2H\u{1B}[2J", Colour::default())
        )
    }

    /// Draw the game header to the output String
    fn draw_header(&self, output: &mut String) -> fmt::Result {
        write!(
            output,
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

    /// Draw all [`Board`] Tiles to the output String
    fn draw_tiles(&self, output: &mut String) -> fmt::Result {
        // Allow bounds-checking elision
        assert_eq!(self.tiles.0.len(), BOARD_AREA);

        // Iterate over each row and column, then print
        for i in 0..BOARD_DIMENSION {
            for j in 0..BOARD_DIMENSION {
                write!(output, "{}", self.tiles[(i, j)])?;
            }
        }
        Ok(())
    }

    /// Draw the current score to the output String
    pub fn draw_score(&self, output: &mut String) -> fmt::Result {
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
            output,
            "{before}{score:<length$}{after}",
            score = self.score,
            before = format_args!("{space}{header}{space}{score_colour}{score_text}"),
            after = format_args!("{no_colour}\r\n"),
        )
    }
}
