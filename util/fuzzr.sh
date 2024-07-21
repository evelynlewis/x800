#!/bin/sh

# Copyright (c) 2024 Evelyn Lewis
# SPDX-License-Identifier: MIT

#
# Fuzz driver for roger, a libfuzzer driver
# Builds and starts roger in background
#
# usage:
# ./util/fuzzr.sh [ARGS]
#
set -e
trap '' HUP

# shellcheck source=fuzzenv.sh
. ./util/fuzzenv.sh

test -d ./fuzz

# Do a blocking build first
fuzz_build "$@"

# Reduce eventual child process priority by nice-ing ourselves
renice -n 8 $$

# Run fuzzer using cargo-fuzz
fuzz_run "$@" 2>>./fuzz/target/roger-output.txt &
