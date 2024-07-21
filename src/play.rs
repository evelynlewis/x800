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

//! Implementation of x800 core event loop

use std::{
    io::{self, Read, Write},
    mem, slice, sync, thread, time,
};

use crate::board::{self, constants, tile::Generation, Action};

/// The number of tiles to generate at startup
const INITIAL_TILES_COUNT: u32 = 2;

/// Minimum wait between non-terminal draws
const MIN_DRAW_DELAY: time::Duration = time::Duration::from_millis(4);

/// Configures terminal as needed
fn io_config(input: &Input) -> Option<Io> {
    if cfg!(feature = "mini") {
        return None;
    }
    let fd = libc::STDIN_FILENO;
    let return_ios: libc::termios;
    unsafe {
        let mut ios: libc::termios = mem::zeroed();
        // Read current configuration
        libc::tcgetattr(fd, &mut ios);
        // Copy
        return_ios = ios;

        // Enable and set raw mode in case of terminal
        if let Input::Interactive = input {
            libc::cfmakeraw(&mut ios);
            libc::tcsetattr(fd, libc::TCSANOW, &ios);
        }
    }
    Some(Io(fd, return_ios))
}

/// Defines input mode
#[allow(dead_code)]
#[derive(PartialEq, Eq)]
pub enum Input<'a> {
    Slice(&'a [u8]),
    Interactive,
}

/// Wrapper for handling IO-related data structures
struct Io(libc::c_int, libc::termios);

/// Main program event loop
#[inline]
#[allow(clippy::too_many_lines)]
pub fn play(input: &Input) {
    // Runtime storage
    let board = sync::Arc::new(sync::Mutex::new(constants::DEFAULT_BOARD.clone()));
    let mut stdin = io::stdin();

    // Game save setup
    #[cfg(feature = "save")]
    let mut save: Vec<u8> = Vec::with_capacity(2048);
    #[cfg(feature = "save")]
    save.extend(fastrand::get_seed().to_ne_bytes());

    // Closure called for program input
    let mut stdin_reader = || -> Action {
        // Read one byte from stdin
        let mut buffer = [0_u8; 1];
        match stdin.read_exact(&mut buffer) {
            Err(_) => Action::Shutdown,
            Ok(()) => {
                #[cfg(feature = "save")]
                save.push(buffer[0]);
                Action::parse(buffer[0])
            }
        }
    };

    // Pre-setup for slice input
    let mut iter = match input {
        Input::Slice(slice) => slice.iter(),
        Input::Interactive => slice::Iter::default(),
    };

    let io = io_config(input);

    // Clear screen and generate initial tiles
    let mut generation: Generation = 0;
    {
        let mut unlocked = board.lock().unwrap();
        for _ in 0..INITIAL_TILES_COUNT {
            unlocked.spawn_tile(generation);
        }
    }

    // Bookkeeping for board-drawing thread
    let draw_quit = sync::Arc::new(sync::atomic::AtomicBool::new(false));
    let board_arg = sync::Arc::clone(&board);
    let quit_arg = sync::Arc::clone(&draw_quit);
    // Spawn board-drawing thread
    let draw_join = if cfg!(not(feature = "mini")) {
        Some(thread::spawn(move || {
            board::draw(&board_arg, &quit_arg).expect(constants::GAME_FAILURE_MESSAGE);
        }))
    } else {
        None
    };
    // We need the thread handle separately
    let draw_thread = draw_join.as_ref().map(thread::JoinHandle::thread);

    // Draw timestamp
    let mut last_draw = time::Instant::now();
    // Initially draw the board
    if let Some(t) = draw_thread {
        t.unpark();
    }

    // The main event loop
    loop {
        generation += 1;
        let action = match input {
            Input::Slice(_) => Action::parse(
                iter.next()
                    .copied()
                    .unwrap_or(constants::END_OF_GAME_CHARACTER),
            ),
            Input::Interactive => stdin_reader(),
        };

        // Read input and take action
        match action {
            Action::Direction(direction) => {
                let mut unlocked = board.lock().unwrap();
                let moved = unlocked.update(direction, generation);

                // Skip adding a tile after invalid move
                if !moved && unlocked.has_space() {
                    continue;
                }

                // Add new tile if the board has space
                if !unlocked.spawn_tile(generation) {
                    break;
                }
            }
            Action::Continue => {
                continue;
            }
            Action::Shutdown => {
                break;
            }
        };

        // In case of update, draw the board
        if let Some(t) = draw_thread {
            if last_draw.elapsed() > MIN_DRAW_DELAY {
                t.unpark();
                last_draw = time::Instant::now();
            }
        }
    }

    // Handle graceful shutdown
    // Signal and join board-drawing thread, if it exists
    draw_quit.store(true, sync::atomic::Ordering::Relaxed);
    if let Some(t) = draw_thread {
        t.unpark();
    }

    if let Some(h) = draw_join {
        h.join().unwrap();
    }

    // Print end-of-game message
    write!(
        constants::WRITER(),
        "{}{}",
        board::constants::PRE_MARGIN,
        board::constants::GAME_OVER
    )
    .expect("Failed to write to output stream");

    // Reset terminal
    if io.is_some() {
        let io = io.unwrap();
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
}
