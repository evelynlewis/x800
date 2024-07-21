// Copyright (c) 2024 Evelyn Lewis
// SPDX-License-Identifier: MIT

// Include README.md docs inline
#![doc = include_str!("./README.md")]

use std::{env, fs, hint, path, process};

const RYAN_MINIMUM_ARG_COUNT: usize = 1;

fn main() {
    // Used in debug messages
    let bin_name = || env::args().next().expect("Failed to get binary name");
    // Print program usage message
    let usage = || {
        println!(
            "
usage:
    {bin} FILE [LOOPS]

    FILE: path to x800 save
    LOOPS: iterate LOOPS times",
            bin = bin_name()
        );
    };
    // Fatal error handler
    let error = |msg: String| -> ! {
        println!("{bin}: ERROR: {msg}", bin = bin_name());
        usage();
        process::exit(1);
    };

    // The usable command-line arguments. argv[0] is the binary name, so skip it.
    let args: Vec<String> = env::args().skip(1).collect();

    // Command-line parsing bookkeeping
    let len = args.len();
    let loops;

    match len {
        0 => {
            assert_eq!(RYAN_MINIMUM_ARG_COUNT, 1);
            error(format!(
                "Need at least {RYAN_MINIMUM_ARG_COUNT} arguments, but {len} found"
            ));
        }
        1 => {
            loops = 1;
        }
        2.. => match args[1].parse::<usize>() {
            Err(_) => {
                error(format!(
                    "Bad LOOPS argument '{}': need positive integer",
                    args[1]
                ));
            }
            Ok(n) => loops = n,
        },
    }

    // The input file
    let name = path::Path::new(&args[0]);
    let contents = fs::read(name);
    match contents {
        Err(e) => {
            error(format!("Failed to read FILE '{}': {}", name.display(), e));
        }
        Ok(d) => {
            // Repeat `loops` times
            for _ in 0..loops {
                // Run x800 with the file contents
                // Use `black_box` to avoid optimizing away the loop
                #[allow(clippy::unit_arg)]
                hint::black_box(x800::one_shot(&d));
            }
        }
    }
}
