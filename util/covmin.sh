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
# Minimize fuzzer coverage
#
# usage:
# ./util/covmin.sh [DIR]
#
set -ex

test -d ./fuzz
test -d ./util

MAIN_DIR="./fuzz/corpus/roger/"
ARCHIVE_DIR="./fuzz/corpus/roger-archive/"
TMP_DIR="./fuzz/corpus/roger-tmp/"

mkdir -p "${MAIN_DIR}" "${ARCHIVE_DIR}"
rsync -a "${MAIN_DIR}" "${ARCHIVE_DIR}"

rm -rf "${TMP_DIR}"
rsync -a "${MAIN_DIR}" "${TMP_DIR}"

# Clean out main dir
rm -rf "${MAIN_DIR}"
mkdir -p "${MAIN_DIR}"

# Merge corpus directories
EXTRA_FUZZER_RUN_ARGS="-merge=1 ${MAIN_DIR} ${TMP_DIR}"

# shellcheck source=fuzzenv.sh
. util/fuzzenv.sh

# Merge all corpus directories
# shellcheck disable=SC2119
fuzz_run
