// Copyright (c) 2024-2025 Evelyn Lewis
// SPDX-License-Identifier: MIT

//! Implementation of x800 core event loop

use std::{
    io::{self, Read as _},
    mem,
};

#[cfg(feature = "draw")]
use crate::clock;

use crate::board::{constants, tile::Tile, Action, Board};

#[cfg(feature = "save")]
use std::io::Write;

/// The number of tiles to generate at startup
const INITIAL_TILES_COUNT: usize = 2;

/// Minimum wait in ticks between non-final draws
#[cfg(feature = "draw")]
const MIN_DRAW_DELAY: u64 = 1024 * 1024;

/// Configures terminal as needed
fn io_config(input: &Input) -> Option<Io> {
    const FD: libc::c_int = libc::STDIN_FILENO;
    if cfg!(not(feature = "draw")) || *input != Input::Interactive {
        return None;
    }
    let return_ios: libc::termios;
    unsafe {
        let mut ios: libc::termios = mem::zeroed();
        // Read current configuration
        libc::tcgetattr(FD, &mut ios);
        // Save initial value for later use
        return_ios = ios;

        // Enable and set raw mode in case of terminal
        libc::cfmakeraw(&mut ios);
        libc::tcsetattr(FD, libc::TCSANOW, &ios);
    }
    Some(Io(FD, return_ios))
}

/// Defines input mode
#[allow(dead_code)]
#[derive(PartialEq, Eq)]
pub(crate) enum Input<'a> {
    Slice(&'a [u8]),
    Interactive,
}

/// Wrapper for handling IO-related data structures
struct Io(libc::c_int, libc::termios);

/// Main program event loop
#[inline]
#[allow(clippy::too_many_lines)]
pub(crate) fn play(input: &Input) {
    // Runtime storage
    let mut board = Board::default();

    // Game save setup
    #[cfg(feature = "save")]
    let mut save: Vec<u8> = Vec::new();
    #[cfg(feature = "save")]
    save.extend(fastrand::get_seed().to_le_bytes());

    // Configure terminal
    let io = io_config(input);

    // Limit scope for lock
    {
        for _ in 0..INITIAL_TILES_COUNT {
            board.spawn_tile();
        }
    }

    // Next draw timestamp in CPU ticks
    #[cfg(feature = "draw")]
    let mut draw_time: u64 = clock::instant(0);
    // Current time in CPU ticks
    #[cfg(feature = "draw")]
    let mut current_time: u64;

    // Frame buffer
    #[cfg(feature = "draw")]
    let mut frame = String::with_capacity(constants::DISPLAY_BUFFER_SIZE);
    // Initially draw the board
    #[cfg(feature = "draw")]
    board.draw_frame(&mut frame);

    // Moves input iterator setup
    let mut moves_slice = if let Input::Slice(slice) = input {
        Some(slice.iter())
    } else {
        None
    };
    let mut moves_stdin = if let Input::Interactive = input {
        Some(io::stdin().lock().bytes())
    } else {
        None
    };

    // Storage used during board operations
    let mut work_line: Vec<Tile> = vec![];
    // Loop storage
    let mut action: Action;
    let mut moved;

    // The main event loop
    loop {
        action = match input {
            Input::Slice(_) => Action::parse(
                moves_slice
                    .as_mut()
                    .unwrap()
                    .next()
                    .copied()
                    .unwrap_or(constants::END_OF_GAME_CHARACTER),
            ),
            Input::Interactive => Action::parse({
                let move_byte = moves_stdin
                    .as_mut()
                    .unwrap()
                    .next()
                    .unwrap_or(Ok(constants::END_OF_GAME_CHARACTER))
                    .expect(constants::GAME_FAILURE_MESSAGE);
                #[cfg(feature = "save")]
                save.push(move_byte);
                move_byte
            }),
        };

        // Read input and take action
        match action {
            Action::Direction(direction) => {
                moved = board.update(direction, &mut work_line);
                // Skip adding a tile after invalid move
                if !moved && board.has_space() {
                    continue;
                }
                // Add new tile if the board has space
                if !board.spawn_tile() {
                    break;
                }
            }
            Action::Continue => {
                continue;
            }
            Action::Shutdown => {
                break;
            }
        }

        // In case of update, draw the board
        #[cfg(feature = "draw")]
        {
            current_time = clock::instant(draw_time);
            if current_time > draw_time {
                board.draw_frame(&mut frame);
            }
            draw_time = current_time.wrapping_add(MIN_DRAW_DELAY);
        }
    }

    #[cfg(feature = "draw")]
    board.draw_quit(&mut frame);

    // Reset terminal
    if let Some(io) = io {
        unsafe {
            libc::tcsetattr(io.0, libc::TCSANOW, &io.1);
        }
    }

    // All game activity must be done prior
    #[cfg(feature = "save")]
    if *input == Input::Interactive {
        let name = format!("x800-game-save-{}.bin", fastrand::u64(..));
        let mut save_file = std::fs::File::create(name).expect("Failed to open save file");
        save_file
            .write(&save)
            .expect("Failed to write game save file");
    }

    #[cfg(feature = "score")]
    println!("{}", board.score());
}
