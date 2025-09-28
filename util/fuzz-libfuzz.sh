#!/bin/sh

# Copyright (c) 2024-2025 Evelyn Lewis
# SPDX-License-Identifier: MIT

#
# Fuzz driver for roger, a libfuzzer driver
# Builds and starts roger in background
#
# usage:
# ./util/fuzz-libfuzz [ARGS]
#
set -e
trap '' HUP

# shellcheck source=util/fuzz-env.sh
. ./util/fuzz-env.sh

test -d ./fuzz

# Do a blocking build first
fuzz_build "$@"

# Reduce eventual child process priority by nice-ing ourselves
renice -n 8 $$

# Run fuzzer using cargo-fuzz
fuzz_run "$@" 2>>./fuzz/target/roger-output.txt &
