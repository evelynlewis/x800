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

use super::super::colour::Colour;
use super::constants::{self, BOARD_DIMENSION, NUMBER_TILES_PER_LINE};
use super::Board;
use std::{
    fmt,
    fmt::Write as FmtWrite,
    io::Write,
    sync::{atomic, Arc, Mutex},
    thread, time,
};

pub fn draw(board: Arc<Mutex<Board>>, done: &Arc<atomic::AtomicBool>) -> fmt::Result {
    // When fuzzing, this fn should not be called
    assert!(cfg!(not(fuzzing)));

    // Use one buffer for program duration
    let buffer = &mut String::with_capacity(constants::DISPLAY_BUFFER_SIZE);

    // Duration between draws. 2ms is 500Hz
    const DRAW_DURATION: time::Duration = time::Duration::from_millis(2);

    // If set, draw the board this time
    let mut force_draw = true;

    // Always draw the first time
    let mut timestamp = time::Instant::now();
    let mut exit_after = false;

    loop {
        // Wait for wakeup
        thread::park();

        // Check if we should exit this time
        if done.load(atomic::Ordering::Relaxed) {
            exit_after = true;
            force_draw = true;
        }

        // Continue waiting if time has not elapsed and a draw is not required
        // Note that we use the monotonic timestamp, time::Instant()
        if !force_draw && (time::Instant::now() - timestamp) < DRAW_DURATION {
            continue;
        } else {
            force_draw = false;
            timestamp = time::Instant::now();
        }

        // Limit scope to avoid holding the lock
        {
            // Alias for reading
            let board = board.lock().unwrap();

            // Write out to string
            board.draw_clear(buffer)?;
            board.draw_header(buffer)?;
            board.draw_tiles(buffer)?;
            board.draw_score(buffer)?;
        }

        // Write out framebuffer
        write!(std::io::stdout(), "{}", buffer).expect("failed to render");
        buffer.clear();

        // Leave the loop
        if exit_after {
            break;
        }
    }

    Ok(())
}

impl Board {
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
        assert_eq!(self.tiles.0.len(), BOARD_DIMENSION * BOARD_DIMENSION);

        // Iterate over each row and column, then print
        for i in 0..BOARD_DIMENSION {
            for j in 0..BOARD_DIMENSION {
                write!(buffer, "{}", self.tiles[(i, j)])?;
            }
        }
        Ok(())
    }
}
