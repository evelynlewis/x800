// Copyright (c) 2024 Evelyn Lewis
// SPDX-License-Identifier: MIT

//! Implementation of [Board] drawing

use super::super::colour::Colour;
use super::constants;
use super::Board;
use std::{fmt::Write as _, io::Write as _, sync, thread};

/**
 * Board drawing worker function
 *
 * Note: `spin::Mutex` is used in place of `std::sync::Mutex` as the draw
 * thread requires only very brief access to the shared data, so we would
 * prefer to avoid putting the main loop thread to sleep for this duration.
 */
pub(crate) fn draw(
    board: &sync::Arc<spin::Mutex<Board>>,
    done: &sync::Arc<sync::atomic::AtomicBool>,
    scan: &sync::Arc<sync::atomic::AtomicBool>,
) {
    // Use one buffer for program duration
    let buffer = &mut String::with_capacity(constants::DISPLAY_BUFFER_SIZE);

    // Validate constants
    assert_eq!(
        constants::BOUNDARY_MARKER_LINE.chars().count(),
        constants::PRINT_LINE_LEN
    );

    // Preferred output stream
    let mut output = constants::WRITER();

    // Our own personal local board
    let mut local_board: Board = Board::default();

    // Make a clean exit
    let mut exit_after = false;

    loop {
        // Wait for wakeup
        thread::park();

        // Avoid spurious wakeups
        // First check if we should print and exit
        if done.load(sync::atomic::Ordering::Acquire) {
            // Print and then exit
            exit_after = true;
        } else if !scan.swap(false, sync::atomic::Ordering::Acquire) {
            // Spurious wakeup - continue and re-park the thread
            continue;
        }

        // Clone for drawing
        local_board.clone_from(&board.lock());

        // Write out to string
        local_board.draw_header(buffer);
        local_board.draw_tiles(buffer);
        local_board.draw_score(buffer);

        // Write out framebuffer
        output
            .write_all(buffer.as_bytes())
            .expect("Failed to render");
        output.flush().expect("failed to flush board render");
        buffer.clear();

        // Exit after to ensure we have always written prior
        if exit_after {
            // Print end-of-game message first
            write!(output, "{}{}", constants::PRE_MARGIN, constants::GAME_OVER)
                .expect("Failed to write to output stream");
            break;
        }
    }
}

impl Board {
    /// Clear the game screen and draw the game header to the output String
    /// Writing fmt output to a String should never fail, so in that case panic
    #[inline]
    fn draw_header(&self, output: &mut String) {
        write!(
            output,
            "{}",
            format_args!(
                "{}{}{}{}{:<colour_len$}{}\r\n{}{}\r\n",
                // Clear screen control character
                Colour::default(),
                "\u{1B}[2H\u{1B}[2J",
                // Header
                constants::PRE_MARGIN,
                // Print top banner
                Colour::from_power(self.max_tile),
                String::new(),
                // Top boundary line
                Colour::default(),
                constants::PRE_MARGIN,
                constants::BOUNDARY_MARKER_LINE,
                // Banner length
                colour_len = constants::PRINT_LINE_LEN
            ),
        )
        .unwrap();
    }

    /// Draw all [`Board`] Tiles to the output String
    /// Writing fmt output to a String should never fail, so in that case panic
    #[inline]
    fn draw_tiles(&self, output: &mut String) {
        // Iterate over each row and column, then print
        output.write_str("      ").unwrap();
        for (i, tile) in self.tiles.iter().enumerate() {
            write!(output, "{tile}").unwrap();
            if i % constants::TILES_PER_LINE == (constants::TILES_PER_LINE - 1) {
                output.write_str("\r\n      ").unwrap();
            }
        }
    }

    /// Draw the current score to the output String
    /// Writing fmt output to a String should never fail, so in that case panic
    fn draw_score(&self, output: &mut String) {
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
            constants::BOUNDARY_MARKER_LINE,
            score = self.score,
            before = format_args!("{SPACE}{header}{SPACE}{score_colour}{SCORE_TEXT}"),
            after = format_args!("{no_colour}\r\n"),
        )
        .unwrap();
    }
}
