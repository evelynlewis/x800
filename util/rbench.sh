#!/bin/sh

# MIT License
#
# Copyright (c) 2024 Evelyn Lewis
#
# Permission is hereby granted, free of charge, to any person obtaining a copy
# of this software and associated documentation files (the "Software"), to deal
# in the Software without restriction, including without limitation the rights
# to use, copy, modify, merge, publish, distribute, sublicense, and/or sell
# copies of the Software, and to permit persons to whom the Software is
# furnished to do so, subject to the following conditions:
#
# The above copyright notice and this permission notice shall be included in all
# copies or substantial portions of the Software.
#
# THE SOFTWARE IS PROVIDED "AS IS", WITHOUT WARRANTY OF ANY KIND, EXPRESS OR
# IMPLIED, INCLUDING BUT NOT LIMITED TO THE WARRANTIES OF MERCHANTABILITY,
# FITNESS FOR A PARTICULAR PURPOSE AND NONINFRINGEMENT. IN NO EVENT SHALL THE
# AUTHORS OR COPYRIGHT HOLDERS BE LIABLE FOR ANY CLAIM, DAMAGES OR OTHER
# LIABILITY, WHETHER IN AN ACTION OF CONTRACT, TORT OR OTHERWISE, ARISING FROM,
# OUT OF OR IN CONNECTION WITH THE SOFTWARE OR THE USE OR OTHER DEALINGS IN THE
# SOFTWARE.

#
# Run ryan benchmarks using various feature-sets
#
# usage:
# ./util/rbench.sh [LOOPS]
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
    while [ $i -lt 5 ]; do
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
