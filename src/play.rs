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
};

use crate::board::{
    self,
    constants::{self, DEFAULT_BOARD, END_OF_GAME_CHARACTER},
};
use board::{Action, Board};

const INITIAL_BLOCK_COUNT: u64 = 2;

#[allow(dead_code)]
pub fn stdin_reader() -> Action {
    // Read a byte from stdin
    let mut buffer = [0_u8; 1];
    if let Err(_) = io::stdin().read_exact(&mut buffer) {
        return Action::Shutdown;
    }
    Action::parse(buffer[0])
}

fn fuzz_cleanup(buf: &mut String, board: &Board) -> Result<(), ()> {
    if let Err(_) = board.draw_score(buf) {
        return Err(());
    }
    // Write final score to terminal
    print!("{}", buf);
    Ok(())
}

fn shutdown(buf: &mut String, board: &Board, fd: i32, ios: &libc::termios) -> Result<(), ()> {
    // Buffer final score
    if cfg!(fuzzing) {
        fuzz_cleanup(buf, board)?;
    }

    // Restore initial board state
    unsafe {
        libc::tcsetattr(fd, libc::TCSANOW, ios);
    }

    // Success!
    Ok(())
}

fn startup(input: &Input) -> (i32, libc::termios) {
    let fd = libc::STDIN_FILENO;
    let return_ios: libc::termios;
    unsafe {
        let mut ios = mem::zeroed();
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
    let mut board = DEFAULT_BOARD.clone();
    let mut gen: board::Generation = 0;
    let mut update;
    let mut has_space;
    let mut action: Action;

    // Startup
    let (fd, termios) = startup(input);

    // Clear screen and draw intial board
    for _ in 0..INITIAL_BLOCK_COUNT {
        board.create_new_tile(gen);
    }

    let mut stdout = io::stdout().lock();
    let mut buffer = String::with_capacity(board::constants::DISPLAY_BUFFER_SIZE);

    board
        .draw(&mut buffer, &mut stdout)
        .expect(constants::GAME_FAILURE_MESSAGE);

    // Pre-setup for slice input
    let mut iter = Default::default();
    match input {
        Input::Slice(slice) => {
            iter = slice.iter();
        }
        _ => {}
    }

    loop {
        action = match input {
            Input::Slice(_) => Action::parse(*iter.next().unwrap_or(&END_OF_GAME_CHARACTER)),
            Input::Interactive(f) => f(),
        };

        // Increment generation
        gen += 1;

        // Read input and take action
        match action {
            Action::Direction(direction) => {
                update = board.update(direction, gen);
            }
            Action::Continue => {
                continue;
            }
            Action::Shutdown => {
                print!(
                    "{}{}",
                    board::constants::LEFT_SPACE,
                    board::constants::GOODBYE
                );
                // Handle graceful shutdown
                return shutdown(&mut buffer, &board, fd, &termios);
            }
        };

        // Add new starting tile if possible
        has_space = board.has_space();
        if has_space {
            board.create_new_tile(gen);
        }

        // Draw new board state
        if let Err(_) = board.draw(&mut buffer, &mut stdout) {
            return Err(());
        }

        // Has the player used their last move?
        if !update && !has_space {
            print!(
                "{}{}",
                board::constants::LEFT_SPACE,
                board::constants::GAME_OVER
            );
            return shutdown(&mut buffer, &board, fd, &termios);
        }
    }
}
