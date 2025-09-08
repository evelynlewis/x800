// Copyright (c) 2025 Evelyn Lewis
// SPDX-License-Identifier: MIT

use std::{env, fs, path, process};

const ARG_COUNT: usize = 1;
const HEADER_SIZE: usize = std::mem::size_of::<u64>();

fn main() {
    // Name of the binary
    let binary_path = env::args().next().unwrap().clone();
    let binary_name = path::Path::new(&binary_path)
        .file_name()
        .unwrap()
        .to_str()
        .unwrap();

    // Usage utility
    let usage = || {
        println!(
            "
usage:
    {binary_name} FILE

    FILE: path to x800 save",
        );
    };

    // Fatal error handler
    let error = |msg: String| -> ! {
        println!("{binary_name}: ERROR: {msg}");
        usage();
        process::exit(1);
    };

    // Usable arguments: args[0] is the binary name
    let args: Vec<String> = env::args().skip(1).collect();

    // Fail on missing file
    if ARG_COUNT != args.len() {
        error(format!(
            "Exepected {ARG_COUNT} arguments but found {}",
            args.len()
        ));
    }

    // The input file
    let in_path = path::Path::new(&args[0]);
    let read = fs::read(in_path);
    let mut filtered: Vec<u8> = vec![];

    // Handle the files
    match read {
        Err(e) => {
            error(format!(
                "Failed to read FILE '{}': {}",
                in_path.display(),
                e
            ));
        }
        Ok(d) => {
            if filtered.len() < 8 {
                return;
            }

            // Copy header and valid moves
            filtered.extend(&d[0..HEADER_SIZE]);
            filtered.extend(
                d.iter()
                    .skip(HEADER_SIZE)
                    .filter(|b| **b == b'a' || **b == b's' || **b == b'd' || **b == b'w')
                    .collect::<Vec<_>>(),
            );

            // Create the updated file name
            let mut out_name = in_path.file_name().unwrap().to_owned();
            out_name.push(".mini");

            // Write out in case of change
            if filtered.len() != d.len() {
                fs::write(in_path.with_file_name(out_name), filtered).unwrap();
            }
        }
    }
}
