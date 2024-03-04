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
    fmt,
    io::Write,
    sync::{atomic, Arc, RwLock},
    thread, time,
};

use super::{constants, Board};

pub fn draw(board: Arc<RwLock<Board>>, done: &Arc<atomic::AtomicBool>) -> fmt::Result {
    // When fuzzing this fn is a no-op
    if cfg!(fuzzing) {
        return Ok(());
    }

    // Use one buffer for program duration
    let buffer = &mut String::with_capacity(constants::DISPLAY_BUFFER_SIZE);

    // Duration between draws. 4ms is 500Hz
    const DRAW_DURATION: time::Duration = time::Duration::from_millis(2);

    // If set, draw the board this time
    let mut force_draw = true;

    // Always draw the first time
    let mut timestamp = time::Instant::now();
    let mut exit_after = false;

    loop {
        // Wait for wakeup
        thread::park();

        // Check if we should exit this time
        if done.load(atomic::Ordering::Relaxed) {
            exit_after = true;
            force_draw = true;
        }

        // Continue waiting if time has not elaped and a draw is not required
        // Note that we use the monotonic timestamp, time::Instant()
        if !force_draw && (time::Instant::now() - timestamp) < DRAW_DURATION {
            continue;
        } else {
            force_draw = false;
            timestamp = time::Instant::now();
        }

        // Limit scope to avoid holding the lock
        {
            // Alias for reading
            let board = board.read().unwrap();

            // Write out to string
            board.draw_clear(buffer)?;
            board.draw_header(buffer)?;
            board.draw_blocks(buffer)?;
            board.draw_score(buffer)?;
        }

        // Write out framebuffer
        write!(std::io::stdout(), "{}", buffer).expect("failed to write to screen");
        buffer.clear();

        // Leave the loop
        if exit_after {
            break;
        }
    }

    Ok(())
}
