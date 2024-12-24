#!/bin/sh

# Copyright (c) 2024 Evelyn Lewis
# SPDX-License-Identifier: MIT

#
# Fuzz driver for roger, a libfuzzer driver
# Builds and starts roger in background.
# This variant is designed for benchmarking and does not use run arguments.
#
# usage:
# ./util/fuzzu.sh [ARGS]
#
set -e
trap '' HUP

test -d ./fuzz

# shellcheck source=fuzzenv.sh
. ./util/fuzzenv.sh

fuzz_build "$@"

# Run fuzzer using cargo-fuzz
fuzz_run_bench "$@" 2>>./fuzz/target/roger-output.txt &
