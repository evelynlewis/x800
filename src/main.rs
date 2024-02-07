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

mod board;
mod colour;
use board::{Action, Board};

const INITIAL_BLOCK_COUNT: u8 = 2;

fn input() -> io::Result<Action> {
    // Read a byte from stdin
    let mut buffer = [0u8; 1];
    io::stdin().read_exact(&mut buffer)?;

    Ok(Action::parse(buffer[0] as char))
}

fn shutdown(fd: i32, ios: &libc::termios) {
    unsafe {
        // Restore initial board state
        libc::tcsetattr(fd, libc::TCSANOW, ios);
    }
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

    let handle_tile_err = |e: &io::Error| {
        println!("{}creating new tile: {}", board::constants::LEFT_SPACE, e);
        shutdown(fd, &termios);
    };

    // Clear screen and draw intial board
    for _ in 0..INITIAL_BLOCK_COUNT {
        if let Err(e) = board.create_new_tile(gen) {
            return handle_tile_err(&e);
        }
    }
    board.draw();

    loop {
        // Increment generation
        gen += 1;

        // Read input and take action
        match input() {
            Err(_) | Ok(Action::Shutdown) => {
                print!(
                    "{}{}",
                    board::constants::LEFT_SPACE,
                    board::constants::GOODBYE
                );
                // Handle graceful shutdown
                return shutdown(fd, &termios);
            }
            Ok(Action::Continue) => {
                continue;
            }
            Ok(other) => {
                update = board.update(other, gen);
            }
        };

        // Add new starting tile if possible
        has_space = board.has_space();
        if has_space {
            if let Err(e) = board.create_new_tile(gen) {
                return handle_tile_err(&e);
            }
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
            return shutdown(fd, &termios);
        }
    }
}
