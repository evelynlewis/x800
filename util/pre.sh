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
# Pre-push tests
#
# usage:
# ./util/pre.sh
#

set -e

xcall() {
    sh -cx "cargo clean"
    sh -cx "$*"
}

echo ++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++
echo "Starting pre-flight validation"
echo ++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++
echo "Now validating environment"
echo
test -d ./fuzz
test -d ./util
echo "OK"
echo ++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++

echo "Now validating build"
echo
xcall cargo build
echo "OK"
echo ++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++

echo "Now validating test"
echo
xcall cargo test
echo "OK"
echo ++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++

echo "Now validating docs"
echo
xcall cargo doc --document-private-items
echo "OK"
echo ++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++

# Build and run criterion benchmarks
echo "Now validating criterion benchmarks"
echo
xcall cargo bench --bench timed
echo "OK"
echo ++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++

# Run fuzzer on known inputs
echo "Now validating fuzzer in default mode"
echo
xcall cargo fuzz run roger --sanitizer none --release -- -runs=0
echo "OK"
echo ++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++

echo "Now validating fuzzer in mini mode"
xcall cargo fuzz run roger --sanitizer none --release --features mini -- -runs=0
echo "OK"
echo ++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++
echo

echo ++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++
echo "ALL OK"
echo "Passed all pre-flight validation"
echo ++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++
