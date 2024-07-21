// Copyright (c) 2024 Evelyn Lewis
// SPDX-License-Identifier: MIT

//! Implementation of [Board] drawing

use super::super::colour::Colour;
use super::constants;
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
    board: &sync::Arc<sync::Mutex<Board>>,
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
        Board::draw_clear(buffer)?;
        cloned.draw_header(buffer)?;
        cloned.draw_tiles(buffer)?;
        cloned.draw_score(buffer)?;

        // Write out framebuffer
        write!(output, "{buffer}").expect("Failed to render");
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
    pub fn draw_clear(output: &mut String) -> fmt::Result {
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
                "{}{}{:<colour_len$}{}\r\n{}{}\r\n",
                constants::PRE_MARGIN,
                Colour::from_power(self.max_tile),
                String::new(),
                Colour::default(),
                constants::PRE_MARGIN,
                "─".repeat(constants::PRINT_LINE_LEN),
                colour_len = constants::PRINT_LINE_LEN
            )
        )
    }

    /// Draw all [`Board`] Tiles to the output String
    fn draw_tiles(&self, output: &mut String) -> fmt::Result {
        // Allow bounds-checking elision
        assert_eq!(self.tiles.len(), constants::TILES_AREA);
        // Iterate over each row and column, then print
        write!(output, "      ")?;
        for i in 0..constants::TILES_PER_LINE {
            for j in 0..constants::TILES_PER_LINE {
                write!(output, "{}", self.tiles[constants::TILES_PER_LINE * i + j])?;
            }
            write!(output, "\r\n      ")?;
        }
        Ok(())
    }

    /// Draw the current score to the output String
    pub fn draw_score(&self, output: &mut String) -> fmt::Result {
        const LENGTH: usize = constants::PRINT_LINE_LEN - SCORE_TEXT.len();
        const SPACE: &str = constants::PRE_MARGIN;
        const SCORE_TEXT: &str = constants::SCORE_TEXT;
        let score_colour = Colour::from_power(self.max_tile);
        let no_colour = Colour::default();
        let header = if self.max_tile >= constants::WIN_POWER {
            constants::WIN_MESSAGE
        } else {
            "\r\n"
        };

        write!(
            output,
            "{}{}{before}{score:<LENGTH$}{after}",
            constants::PRE_MARGIN,
            "─".repeat(constants::PRINT_LINE_LEN),
            score = self.score,
            before = format_args!("{SPACE}{header}{SPACE}{score_colour}{SCORE_TEXT}"),
            after = format_args!("{no_colour}\r\n"),
        )
    }
}
