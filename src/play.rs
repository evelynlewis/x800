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
    io::{self, Read},
    mem,
    sync::{atomic, Arc, RwLock},
    thread,
};

use crate::board::{
    self,
    constants::{self, DEFAULT_BOARD, END_OF_GAME_CHARACTER},
};
use board::{Action, Board};

const INITIAL_TILES_COUNT: u64 = 2;

#[allow(dead_code)]
pub fn stdin_reader() -> Action {
    // Read a byte from stdin
    let mut buffer = [0_u8; 1];
    if let Err(_) = io::stdin().read_exact(&mut buffer) {
        return Action::Shutdown;
    }
    Action::parse(buffer[0])
}

fn fuzz_cleanup(board: &Board) -> Result<(), ()> {
    let mut output = String::with_capacity(constants::DISPLAY_LINE_LENGTH);
    if let Err(_) = board.draw_score(&mut output) {
        return Err(());
    }
    // Write final score to terminal
    print!("{}", output);
    Ok(())
}

fn startup(input: &Input) -> (i32, libc::termios) {
    let fd = libc::STDIN_FILENO;
    let return_ios: libc::termios;
    unsafe {
        let mut ios: libc::termios = mem::zeroed();
        // Read current configuration
        libc::tcgetattr(fd, &mut ios);
        // Copy
        return_ios = ios;

        // Enable and set raw mode in case of terminal
        if let Input::Interactive(_) = input {
            libc::cfmakeraw(&mut ios);
            libc::tcsetattr(fd, libc::TCSANOW, &ios);
        }
    }
    (fd, return_ios)
}

#[allow(dead_code)]
pub enum Input<'a> {
    Slice(&'a [u8]),
    Interactive(fn() -> Action),
}

#[inline]
pub fn play(input: &Input) -> Result<(), ()> {
    // Runtime storage
    let board = Arc::new(RwLock::new(DEFAULT_BOARD.clone()));
    let mut gen: board::Generation = 0;
    let mut action: Action;

    // Pre-setup for slice input
    let mut iter = Default::default();
    match input {
        Input::Slice(slice) => {
            iter = slice.iter();
        }
        _ => {}
    }

    // Startup
    let (fd, termios) = startup(input);

    // Closure for ending the game
    let shutdown = || {
        // Buffer final score
        if cfg!(fuzzing) {
            fuzz_cleanup(&board.read().unwrap())?;
        }

        // Restore initial board state
        unsafe {
            libc::tcsetattr(fd, libc::TCSANOW, &termios);
        }

        Ok(())
    };

    // Clear screen and draw intial board
    for _ in 0..INITIAL_TILES_COUNT {
        board.write().unwrap().create_new_tile(gen);
    }

    // Bookeeping for board-drawing thread
    let draw_board = Arc::clone(&board);
    let draw_done_reader = Arc::new(atomic::AtomicBool::new(false));
    let draw_done_writer = Arc::clone(&draw_done_reader);

    // Spawn board-drawing thread
    let draw_thread_joiner = thread::spawn(move || {
        board::draw(draw_board, &draw_done_reader).expect(constants::GAME_FAILURE_MESSAGE);
    });

    // We need the handle seperately
    let draw_thread = draw_thread_joiner.thread();

    // Do we need to redraw the board?
    let mut redraw = true;

    loop {
        // If there was any update, draw the board
        if redraw {
            draw_thread.unpark();
        }

        // Increment generation
        gen += 1;

        // First assume no re-draw
        redraw = false;

        action = match input {
            Input::Slice(_) => Action::parse(*iter.next().unwrap_or(&END_OF_GAME_CHARACTER)),
            Input::Interactive(f) => f(),
        };

        // Read input and take action
        match action {
            Action::Direction(direction) => {
                board.write().unwrap().update(direction, gen);
                redraw = true;
            }
            Action::Continue => {
                continue;
            }
            Action::Shutdown => {
                break;
            }
        };

        // Add new starting tile if possible
        // Has the player already used their last move?
        if !board.write().unwrap().create_new_tile(gen) {
            break;
        }
    }

    // Signal and join board-drawing thread
    draw_done_writer.store(true, atomic::Ordering::Relaxed);
    draw_thread.unpark();
    draw_thread_joiner.join().unwrap();

    // Print end-of-game message
    print!(
        "{}{}",
        board::constants::LEFT_SPACE,
        board::constants::GAME_OVER
    );

    // Handle graceful shutdown
    shutdown()
}
