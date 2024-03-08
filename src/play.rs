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
    sync::{atomic, Arc, Mutex},
    thread,
};

use crate::board::{
    self,
    constants::{self, DEFAULT_BOARD, END_OF_GAME_CHARACTER},
};
use board::{Action, Board};

const INITIAL_TILES_COUNT: u32 = 2;

#[inline(always)]
fn startup(input: &Input) -> Option<Io> {
    // Early exit
    if cfg!(fuzzing) {
        return None;
    }

    // In case of not fuzzing, we must return Some(_)
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
    let mut gen: board::Generation = 0;

    // Closure called for program input
    let stdin_reader = || -> Action {
        // Read a byte from stdin
        let mut buffer = [0_u8; 1];
        if let Err(_) = io::stdin().read_exact(&mut buffer) {
            return Action::Shutdown;
        }
        Action::parse(buffer[0])
    };

    // Pre-setup for slice input
    let mut iter = match input {
        Input::Slice(slice) => slice.iter(),
        _ => Default::default(),
    };

    let io: Option<Io> = startup(input);
    // Ensure the postcondition holds
    if !cfg!(fuzzing) {
        assert!(io.is_some());
    }

    // Clear screen and draw intial board
    {
        let mut unlocked = board.lock().unwrap();
        for _ in 0..INITIAL_TILES_COUNT {
            unlocked.spawn_tile(gen);
        }
    }

    // Bookeeping for board-drawing thread
    let draw_quit = Arc::new(atomic::AtomicBool::new(false));
    let draw_join;

    // Spawn board-drawing thread
    {
        let board_arg = Arc::clone(&board);
        let quit_arg = Arc::clone(&draw_quit);
        draw_join = thread::spawn(move || {
            board::draw(board_arg, &quit_arg).expect(constants::GAME_FAILURE_MESSAGE);
        });
    }

    // We need the handle seperately
    let draw_thread = draw_join.thread();

    // Draw the board initially
    draw_thread.unpark();

    loop {
        let action = match input {
            Input::Slice(_) => Action::parse(*iter.next().unwrap_or(&END_OF_GAME_CHARACTER)),
            Input::Interactive => stdin_reader(),
        };

        // Read input and take action
        match action {
            Action::Direction(direction) => {
                board.lock().unwrap().update(direction, gen);
            }
            Action::Continue => {
                continue;
            }
            Action::Shutdown => {
                break;
            }
        };

        // If there was any update, draw the board
        draw_thread.unpark();

        // Add new starting tile if possible
        // Has the player already used their last move?
        if !board.lock().unwrap().spawn_tile(gen) {
            break;
        }

        // Increment generation
        gen += 1;
    }

    // Signal and join board-drawing thread
    draw_quit.store(true, atomic::Ordering::Relaxed);
    draw_thread.unpark();
    draw_join.join().unwrap();

    // Print end-of-game message
    print!(
        "{}{}",
        board::constants::LEFT_SPACE,
        board::constants::GAME_OVER
    );

    let fuzz_shutdown = |board: &Board| {
        assert!(cfg!(fuzzing));
        let mut output = String::with_capacity(constants::DISPLAY_LINE_LENGTH);
        board.draw_score(&mut output).expect("could not draw score");
        print!("{}", output);
    };

    let interactive_shutdown = |io: &Io| {
        assert!(!cfg!(fuzzing));
        // Buffer final score
        // Restore initial board state
        unsafe {
            libc::tcsetattr(io.0, libc::TCSANOW, &io.1);
        }
    };

    // Handle graceful shutdown
    if cfg!(fuzzing) {
        fuzz_shutdown(&board.lock().unwrap());
    } else {
        interactive_shutdown(&io.expect("shutdown failed"));
    }

    Ok(())
}
