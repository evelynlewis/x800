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

# Reduce eventual child process priority by nice-ing ourselves
renice -n 8 $$

# Run fuzzer using cargo-fuzz
fuzz_run_bench "$@" 2>>./fuzz/target/roger-output.txt &
