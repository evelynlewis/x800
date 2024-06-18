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

use std::env;
use std::fs;
use std::path;
use std::process;

use x800::one_shot;

fn main() {
    let args: Vec<String> = env::args().collect();
    let usage = || {
        println!("usage:\n    {} FILE [COUNT]", args[0]);
    };
    let error = |msg: String| -> ! {
        println!("{bin}: ERROR: {msg}", bin = args[0]);
        usage();
        process::exit(1);
    };
    const WANT_ARG_COUNT: usize = 1;
    let len = args.len();
    let count;
    match len {
        0 | 1 => {
            error(format!(
                "Need at least {WANT_ARG_COUNT} arguments, but {} arguments found",
                len - 1
            ));
        }
        2 => {
            count = 1;
        }
        3.. => match args[2].parse::<usize>() {
            Err(_) => {
                error(format!("Failed to parse numeric COUNT"));
            }
            Ok(n) => count = n,
        },
    }
    let name = path::Path::new(&args[1]);
    let contents = fs::read(name);
    match contents {
        Err(e) => {
            error(format!(
                "Failed to read file: {}\nError: {}",
                name.display(),
                e
            ));
        }
        Ok(d) => {
            for _ in 0..count {
                one_shot(&d);
            }
        }
    }
}
