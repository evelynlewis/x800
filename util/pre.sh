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

test -d ./util/
# shellcheck source=common.sh
. ./util/common.sh

plus_line
echo "Starting pre-flight validation"
plus_line
echo

plus_header "Now validating environment"
test -d ./fuzz
test -d ./util
plus_ok

plus_header "Now validating git"
if [ "$(git status --porcelain=v1 2>/dev/null | wc -l)" -ne 0 ]; then
	echo "ERROR: untracked files:"
	echo
	git status
	echo
	exit 1
fi
plus_ok

plus_header "Now validating build"
xcall cargo build
plus_ok

plus_header "Now validating build with sink feature"
xcall cargo build --features sink
plus_ok

plus_header "Now validating build with mini feature"
xcall cargo build --features mini
plus_ok

plus_header "Now validating build with save feature"
xcall cargo build --features save
plus_ok

plus_header "Now validating test"
xcall cargo test
plus_ok

plus_header "Now validating docs"
xcall cargo doc --document-private-items
plus_ok

# Build and run timed benchmarks
plus_header "Now validating timed benchmarks"
xcall cargo bench --bench timed
plus_ok

# Build and run hyperfine benchmarks
plus_header "Now validating hyperfine benchmarks"
xcall ./util/hyperfine.sh
plus_ok

# Build and run hyperfine benchmarks
plus_header "Now validating ryan benchmarks"
xcall ./util/rbench.sh
plus_ok

# Run fuzzer on known inputs
plus_header "Now validating fuzzer in default mode"
xcall cargo fuzz run roger --sanitizer none --release -- -runs=0
plus_ok

plus_header "Now validating fuzzer in mini mode"
xcall cargo fuzz run roger --sanitizer none --release --features mini -- -runs=0
plus_ok

echo
plus_line
echo "ALL OK"
echo "Passed all pre-flight validation"
plus_line
