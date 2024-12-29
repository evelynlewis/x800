#!/bin/sh

# Copyright (c) 2024 Evelyn Lewis
# SPDX-License-Identifier: MIT

#
# Run ryan benchmarks using various feature-sets
#
# usage:
# ./util/benchr.sh [LOOPS]
#
set -e

test -d ./fuzz/corpus/roger
test -d ./util
# shellcheck source=common.sh
. ./util/common.sh

LOOPS=10000
WARMUP=12
RUNS=24

test_ryan() {
    plus_header "Building the ryan driver in $1 mode"
    sh -xc "cargo build --release --features=$1 --bin ryan"
    echo
    echo "GAMES_PER_RUN: ${LOOPS}"
    sh -xc "hyperfine -N --warmup ${WARMUP} --runs ${RUNS} \"./target/release/ryan ./fuzz/corpus/roger/gold ${LOOPS}\""
    echo
    plus_ok
    echo
}

plus_header "Checking for \`hyperfine\` in path"

if ! command -v hyperfine > /dev/null; then
    plus_header "\`hyperfine\` not found: please install with \`cargo install hyperfine\`"
else
    plus_ok
fi

plus_header "Pre-cleaning build directory"
sh -xc "cargo clean"
plus_ok

test_ryan default
test_ryan sink
test_ryan mini
