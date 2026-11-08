// Copyright (c) 2024 Evelyn Lewis
// SPDX-License-Identifier: MIT

//! Implementation of [Board] drawing

// Module is draw-only
#![cfg(feature = "draw")]

use std::{fmt::Write as _, io::Write as _};

use super::super::colour::Colour;
use super::Board;
use super::constants;

impl Board {
    /// Draw a normal board frame
    pub(crate) fn draw_frame(&self, frame: &mut String) {
        // Validate constants
        assert_eq!(
            constants::BOUNDARY_MARKER_LINE.chars().count(),
            constants::PRINT_LINE_LEN
        );

        // Preferred output stream
        let mut output = constants::WRITER();

        // Write out to string
        self.draw_header(frame);
        self.draw_tiles(frame);
        self.draw_score(frame);

        // Write out framebuffer
        output
            .write_all(frame.as_bytes())
            .expect("Failed to render");
        output.flush().expect("failed to flush board render");
        frame.clear();
    }

    /// Draw the final board frame
    pub(crate) fn draw_quit(&mut self, frame: &mut String) {
        // Normal draw first
        self.draw_frame(frame);
        // Print end-of-game message
        write!(
            constants::WRITER(),
            "{}{}",
            constants::PRE_MARGIN,
            constants::GAME_OVER
        )
        .expect("Failed to write to output stream");
    }

    /// Clear the game screen and draw the game header to the output String
    /// Writing fmt output to a String should never fail, so in that case panic
    #[inline]
    fn draw_header(&self, frame: &mut String) {
        write!(
            frame,
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
    fn draw_tiles(&self, frame: &mut String) {
        // Iterate over each row and column, then print
        frame.write_str("      ").unwrap();
        for (i, tile) in self.tiles.iter().enumerate() {
            write!(frame, "{tile}").unwrap();
            if i % constants::TILES_PER_LINE == (constants::TILES_PER_LINE - 1) {
                frame.write_str("\r\n      ").unwrap();
            }
        }
    }

    /// Draw the current score to the output String
    /// Writing fmt output to a String should never fail, so in that case panic
    fn draw_score(&self, frame: &mut String) {
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
            frame,
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
