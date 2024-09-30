// Copyright (c) 2024 Evelyn Lewis
// SPDX-License-Identifier: MIT

//! Implementation of x800 core event loop

use std::{
    io::{self, Read},
    mem, slice, sync, thread, time,
};

#[cfg(feature = "save")]
use std::io::Write;

use crate::board::{
    self, constants,
    tile::{Generation, Tile},
    Action, Board,
};

/// The number of tiles to generate at startup
const INITIAL_TILES_COUNT: u32 = 2;

/// Minimum wait between non-final draws
const MIN_DRAW_DELAY: time::Duration = time::Duration::from_millis(4);

/// Configures terminal as needed
fn io_config(input: &Input) -> Option<Io> {
    if cfg!(feature = "mini") || *input != Input::Interactive {
        return None;
    }
    const FD: libc::c_int = libc::STDIN_FILENO;
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
    let board = sync::Arc::new(sync::Mutex::new(Board::default()));

    // Game save setup
    #[cfg(feature = "save")]
    let mut save: Vec<u8> = Vec::new();
    #[cfg(feature = "save")]
    save.extend(fastrand::get_seed().to_le_bytes());

    // Closure called for program input
    let mut stdin = io::stdin().lock();
    let mut buffer = [0_u8; 1];
    let mut stdin_reader = || -> Action {
        // Read one byte from stdin
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

    // Limit scope for lock
    {
        let mut unlocked = board.lock().unwrap();
        for _ in 0..INITIAL_TILES_COUNT {
            unlocked.spawn_tile(generation);
        }
    }

    // Bookkeeping for board-drawing thread
    let draw_quit = sync::Arc::new(sync::atomic::AtomicBool::new(false));
    let draw_scan = sync::Arc::new(sync::atomic::AtomicBool::new(false));
    // Spawn board-drawing thread
    let draw_join = if cfg!(not(feature = "mini")) {
        Some(thread::spawn({
            let board = board.clone();
            let draw_quit = draw_quit.clone();
            let draw_scan = draw_scan.clone();
            move || {
                board::draw(&board, &draw_quit, &draw_scan).expect(constants::GAME_FAILURE_MESSAGE);
            }
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
        draw_scan.store(true, sync::atomic::Ordering::Relaxed);
        t.unpark();
    }

    // Storage used during board operations
    let mut work_line: Vec<Tile> = vec![];
    // Loop storage
    let mut action: Action;
    let mut moved;

    // The main event loop
    loop {
        generation += 1;
        action = match input {
            Input::Interactive => stdin_reader(),
            Input::Slice(_) => Action::parse(
                iter.next()
                    .copied()
                    .unwrap_or(constants::END_OF_GAME_CHARACTER),
            ),
        };

        // Read input and take action
        match action {
            Action::Direction(direction) => {
                let mut unlocked = board.lock().unwrap();
                moved = unlocked.update(direction, generation, &mut work_line);

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
                draw_scan.store(true, sync::atomic::Ordering::Relaxed);
                t.unpark();
                last_draw = time::Instant::now();
            }
        }
    }

    // Signal and join board-drawing thread, if it exists
    if let Some(t) = draw_thread {
        draw_scan.store(true, sync::atomic::Ordering::Relaxed);
        draw_quit.store(true, sync::atomic::Ordering::Release);
        t.unpark();
        if let Some(h) = draw_join {
            h.join().unwrap();
        }
    }

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
    println!("{}", board.lock().unwrap().score());
}
