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

use std::{fs::File, io::Read, mem};

mod board;
mod colour;
use board::{Action, Board};

const INITIAL_BLOCK_COUNT: u8 = 2;

fn input() -> Action {
    // Read a byte from stdin
    let mut stdin = File::open("/dev/stdin").unwrap();
    let mut buffer = [0u8; 1];
    stdin.read_exact(&mut buffer).unwrap();

    Action::parse(buffer[0] as char)
}

fn shutdown(fd: i32, ios: &libc::termios) {
    unsafe {
        // Restore initial board state
        libc::tcsetattr(fd, libc::TCSANOW, ios);
    }
    std::process::exit(libc::EXIT_SUCCESS);
}

fn startup() -> (i32, libc::termios) {
    let fd = libc::STDIN_FILENO;
    let return_ios: libc::termios;
    unsafe {
        let mut ios = mem::zeroed();
        // Read current configuration
        libc::tcgetattr(fd, &mut ios);
        // Copy
        return_ios = ios;
        // Enable and set raw mode
        libc::cfmakeraw(&mut ios);
        libc::tcsetattr(fd, libc::TCSANOW, &ios);
    }
    (fd, return_ios)
}

fn main() {
    // Runtime storage
    let mut board = Board::new();
    let mut gen: board::Generation = 0;
    let mut update;
    let mut has_space;

    // Startup
    let (fd, termios) = startup();

    // Clear screen and draw intial board
    for _ in 0..INITIAL_BLOCK_COUNT {
        board.create_block(gen);
    }
    board.draw();

    loop {
        // Assume no update
        update = false;

        // Increment generation
        gen += 1;

        // Read input and take action
        match input() {
            Action::Shutdown => {
                print!(
                    "{}{}",
                    board::constants::LEFT_SPACE,
                    board::constants::GOODBYE
                );
                shutdown(fd, &termios);
            }
            Action::Continue => {
                continue;
            }
            other => {
                update = board.update(other, gen);
            }
        };

        // Add new starting tile if possible
        has_space = board.has_space();
        if has_space {
            board.create_block(gen);
        }

        // Draw new board state
        board.draw();

        // Has the player used their last move?
        if !update && !has_space {
            print!(
                "{}{}",
                board::constants::LEFT_SPACE,
                board::constants::GAME_OVER
            );
            shutdown(fd, &termios);
        }
    }
}
