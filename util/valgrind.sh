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

# valgrind.sh
# Run Valgrind tools on the `roger` fuzzer
# Using the -a flag prints annotation output

set -ex

cargo fuzz build --release --no-cfg-fuzzing --sanitizer none roger

BIN="$(find fuzz/target -name roger | grep release)"
VALGRIND="valgrind"
ANNOTATE="callgrind_annotate"
OUT="callgrind.out"

# Ensure binaries are valid, and print message otherwise
if_none() {
	command -v "$1" >/dev/null && return 0
	echo "$2"
	return 1
}
if_none "${BIN}" "Could not find binary"
if_none "${VALGRIND}" "Could not find valgrind tool"
if_none "${ANNOTATE}" "Could not find valgrind annotator"

${VALGRIND} --tool=callgrind --simulate-cache=yes --callgrind-out-file="${OUT}" "${BIN}" -runs=0 fuzz/corpus/roger >/dev/null

if [ "$1" = '-a' ]; then
	${ANNOTATE} --auto=yes "${OUT}"
fi
