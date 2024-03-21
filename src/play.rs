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
    collections::BTreeSet,
    io::{self, Read},
    mem,
    sync::{atomic, Arc, Mutex},
    thread,
};

use crate::board::{
    self,
    constants::{self, DEFAULT_BOARD, END_OF_GAME_CHARACTER},
};
use board::Action;

const INITIAL_TILES_COUNT: u32 = 2;
const EXPECT_NOT_FUZZING: &str = "Expected cfg!(not(fuzzing))";

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
    assert_eq!(cfg!(not(fuzzing)), io.is_some());

    // Clear screen and draw initial board
    {
        let mut unlocked = board.lock().unwrap();
        for _ in 0..INITIAL_TILES_COUNT {
            unlocked.spawn_tile();
        }
    }

    // Bookkeeping for board-drawing thread
    let draw_quit = Arc::new(atomic::AtomicBool::new(false));
    let draw_join;
    let draw_thread;

    // Provision zero or one drawing threads
    if cfg!(fuzzing) {
        // Empty in case of fuzzing
        draw_join = None;
        draw_thread = None;
    } else {
        // The usual case
        assert!(cfg!(not(fuzzing)));

        // Spawn board-drawing thread
        let board_arg = Arc::clone(&board);
        let quit_arg = Arc::clone(&draw_quit);
        draw_join = Some(thread::spawn(move || {
            board::draw(board_arg, &quit_arg).expect(constants::GAME_FAILURE_MESSAGE);
        }));

        // We need the thread handle separately
        draw_thread = Some(draw_join.as_ref().expect(EXPECT_NOT_FUZZING).thread());

        // Initially draw the board
        draw_thread.as_ref().expect(EXPECT_NOT_FUZZING).unpark();
    }

    let mut moved;
    let mut empty_set = BTreeSet::new();

    // The main event loop
    loop {
        let action = match input {
            Input::Slice(_) => Action::parse(*iter.next().unwrap_or(&END_OF_GAME_CHARACTER)),
            Input::Interactive => stdin_reader(),
        };

        // Read input and take action
        match action {
            Action::Direction(direction) => {
                let mut unlocked = board.lock().unwrap();
                moved = unlocked.update(direction, &mut empty_set);

                // If the move had no effect on a non-full board,
                // skip adding a new tile
                if !moved && unlocked.has_space() {
                    continue;
                }

                // Add new starting tile if possible
                // Has the player already used their last move?
                if !unlocked.spawn_tile() {
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

        // In case of update while not fuzzing, draw the board
        if cfg!(not(fuzzing)) {
            draw_thread.as_ref().expect(EXPECT_NOT_FUZZING).unpark();
        }
    }

    // Handle graceful shutdown
    if cfg!(not(fuzzing)) {
        // Signal and join board-drawing thread, if it exists
        draw_quit.store(true, atomic::Ordering::Relaxed);
        draw_thread.as_ref().expect(EXPECT_NOT_FUZZING).unpark();
        draw_join.unwrap().join().unwrap();

        // Print end-of-game message
        print!(
            "{}{}",
            board::constants::LEFT_SPACE,
            board::constants::GAME_OVER
        );

        // Reset terminal
        let io = io.expect("shutdown failed");
        unsafe {
            libc::tcsetattr(io.0, libc::TCSANOW, &io.1);
        }
    }

    Ok(())
}
