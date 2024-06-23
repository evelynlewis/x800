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

set -e

test -d ./util/
test -f ./util/gen-moves.sh

# shellcheck source=common.sh
. ./util/common.sh

plus_header "Building project"
cargo build -p x800 --release --features sink
plus_ok

plus_header "Installing hyperfine"
cargo install --quiet hyperfine
plus_ok

plus_header "Running with random input"
touch /tmp/moves
hyperfine \
	--prepare './util/gen-moves.sh /tmp/moves' \
	--warmup=256 \
	--runs=256 \
	--input='/tmp/moves' \
	-N \
	'./target/release/x800'
plus_ok

GOLD_FILE="$(realpath ./fuzz/corpus/roger/gold)"
GOLD_LOOPS="1000"
plus_header "Running with gold file input"
echo "Size, resolved path, and loop count are:"
printf "%s %s\n\n" "$(du -sh "${GOLD_FILE}")" "${GOLD_LOOPS}"

hyperfine \
	--warmup=32 \
	--runs=32 \
	-N \
	"./target/release/ryan ${GOLD_FILE} ${GOLD_LOOPS}"
plus_ok
