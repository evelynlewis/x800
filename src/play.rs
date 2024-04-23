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

use std::{
    collections::VecDeque,
    io::{self, Read, Write},
    mem,
    sync::{atomic, Arc, Mutex},
    thread,
};

use crate::board::{
    self,
    constants::{self, DEFAULT_BOARD, END_OF_GAME_CHARACTER, NUMBER_TILES_PER_LINE},
    Generation,
};
use board::Action;

const INITIAL_TILES_COUNT: u32 = 2;

fn startup(input: &Input) -> Option<Io> {
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

#[allow(dead_code)]
pub enum Input<'a> {
    Slice(&'a [u8]),
    Interactive,
}

struct Io(libc::c_int, libc::termios);

#[inline(always)]
pub fn play(input: &Input) -> Result<(), ()> {
    // Runtime storage
    let board = Arc::new(Mutex::new(DEFAULT_BOARD.clone()));

    // Closure called for program input
    let stdin_reader = || -> Action {
        // Read a byte from stdin
        let mut buffer = [0_u8; 1];
        match io::stdin().read_exact(&mut buffer) {
            Err(_) => Action::Shutdown,
            Ok(_) => Action::parse(buffer[0]),
        }
    };

    // Pre-setup for slice input
    let mut iter = match input {
        Input::Slice(slice) => slice.iter(),
        Input::Interactive => Default::default(),
    };

    let io = startup(input);

    // Clear screen and generate initial tiles
    let mut generation: Generation = 0;
    {
        let mut unlocked = board.lock().unwrap();
        for _ in 0..INITIAL_TILES_COUNT {
            unlocked.spawn_tile(generation);
        }
    }

    // Bookkeeping for board-drawing thread
    let draw_quit = Arc::new(atomic::AtomicBool::new(false));
    let draw_join;
    let draw_thread;

    // Spawn board-drawing thread
    let board_arg = Arc::clone(&board);
    let quit_arg = Arc::clone(&draw_quit);
    draw_join = if cfg!(not(feature = "mini")) {
        Some(thread::spawn(move || {
            board::draw(board_arg, &quit_arg).expect(constants::GAME_FAILURE_MESSAGE);
        }))
    } else {
        None
    };

    // We need the thread handle separately
    draw_thread = draw_join.as_ref().and_then(|t| Some(t.thread()));

    // Initially draw the board
    draw_thread.and_then(|t| Some(t.unpark()));

    let mut moved;
    let mut empty_queue = VecDeque::with_capacity(NUMBER_TILES_PER_LINE);

    // The main event loop
    loop {
        generation += 1;
        let action = match input {
            Input::Slice(_) => Action::parse(iter.next().copied().unwrap_or(END_OF_GAME_CHARACTER)),
            Input::Interactive => stdin_reader(),
        };

        // Read input and take action
        match action {
            Action::Direction(direction) => {
                let mut unlocked = board.lock().unwrap();
                moved = unlocked.update(direction, generation, &mut empty_queue);

                // If the move had no effect on a non-full board,
                // skip adding a new tile
                if !moved && unlocked.has_space() {
                    continue;
                }

                // Add new starting tile if possible
                // Has the player already used their last move?
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
        draw_thread.and_then(|t| Some(t.unpark()));
    }

    // Handle graceful shutdown
    // Signal and join board-drawing thread, if it exists
    draw_quit.store(true, atomic::Ordering::Relaxed);
    draw_thread.and_then(|t| Some(t.unpark()));
    draw_join.and_then(|h| Some(h.join().unwrap()));

    // Print end-of-game message
    write!(
        constants::WRITER(),
        "{}{}",
        board::constants::LEFT_SPACE,
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

    Ok(())
}
