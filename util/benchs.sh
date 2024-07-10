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
# x800 shell benchmark
#
# usage:
# ./util/benchs.sh [ARGS]
#
set -e

# source environmental compatibility script
# shellcheck source=compat.sh
. ./util/compat.sh

# Setup tools
BASE32=$(compat base32)
TR=$(compat tr)
DD=$(compat dd)

# Build
echo "Building x800 executable."
cargo build --release --features sink "${@}"

# Other params
BS="8192"
SLEEP="16"
echo "Running benchmark for ${SLEEP} seconds."

# Configure cleanup
trap 'kill $(pgrep -P $$) 2> /dev/null' EXIT INT

# Run worker
${BASE32} </dev/urandom |
    ${TR} -dC 'ASDW' |
    ${DD} conv=lcase bs=${BS} status=progress |
    sh -c 'while true; do ./target/release/x800; done' &

# Sleep and exit
sleep ${SLEEP}
exit 0
