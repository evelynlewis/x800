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

if [ $# -gt 0 ]; then
    LOOPS=$1
else
    LOOPS=10000
fi

test_ryan() {
    plus_header "Building the ryan driver in $1 mode"
    cargo build --release --features="$1" --bin ryan
    echo
    echo "Running ${LOOPS} iterations:"
    # Allow for best of a few iterations
    i=0
    while [ $i -lt 8 ]; do
	echo
        time -p ./target/release/ryan ./fuzz/corpus/roger/gold "${LOOPS}" >/dev/null
        i=$((i + 1))
    done
    echo
    plus_ok
    echo
}

test_ryan default
test_ryan sink
test_ryan mini
