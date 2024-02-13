use std::{
    io::{self, Read},
    mem,
};

use crate::board;
use board::{Action, Board};

const INITIAL_BLOCK_COUNT: u8 = 2;

#[allow(dead_code)]
pub fn stdin_reader() -> Action {
    // Read a byte from stdin
    let mut buffer = [0u8; 1];
    if io::stdin().read_exact(&mut buffer).is_err() {
        return Action::Shutdown;
    }
    Action::parse(Some(&buffer[0]))
}

fn shutdown(fd: i32, ios: &libc::termios) {
    unsafe {
        // Restore initial board state
        libc::tcsetattr(fd, libc::TCSANOW, ios);
    }
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

pub fn play(input: &Input) {
    // Runtime storage
    let mut board = Board::new();
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

    board.draw();

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
            Input::Slice(_) => Action::parse(iter.next()),
            Input::Interactive(f) => f(),
        };

        // Increment generation
        gen += 1;

        // Read input and take action
        match action {
            Action::Shutdown => {
                print!(
                    "{}{}",
                    board::constants::LEFT_SPACE,
                    board::constants::GOODBYE
                );
                // Handle graceful shutdown
                return shutdown(fd, &termios);
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
            board.create_new_tile(gen);
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
