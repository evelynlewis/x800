#!/bin/sh

# Copyright (c) 2024 Evelyn Lewis
# SPDX-License-Identifier: MIT

#
# Fuzz driver for hroger, a Honggfuzz driver
# Builds and starts hroger in background
#
# usage:
# ./util/fuzzh.sh
#
set -e

# See usage: should be run from x800 root
if ! cd fuzz 2>/dev/null; then
    echo "Error: must be run from root directory"
    echo "usage:"
    echo "./util/fuzzh.sh"
    exit 1
fi

# Install required tooling
cargo install honggfuzz

# Used by honggfuzz
export HFUZZ_INPUT="./corpus/roger/"
export HFUZZ_RUN_ARGS="$@"

# Build first
cargo hfuzz build

cargo hfuzz run hroger

