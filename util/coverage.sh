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

# Errors are fatal, and trace execution
set -e

# Ensure correct working directory
test -d ./util/
test -d ./fuzz/

# Corpus selection
CORPUS="fuzz/corpus/roger/"
if [ ${#} -gt 0 ]; then
	CORPUS=$1
fi
echo "Using corpus directory: '${CORPUS}'"

DEPS="rustfilt cargo-fuzz"
FUZZ_ARGS="--release --sanitizer none --no-cfg-fuzzing roger"
COV_ARGS='-Xdemangler=rustfilt -ignore-filename-regex='.cargo' -ignore-filename-regex='/rustc/.+' -use-color -instr-profile=fuzz/coverage/roger/coverage.profdata'

# Install dependencies
# shellcheck disable=SC2086
cargo install --quiet ${DEPS}

# Build fuzz target and generate coverage
# shellcheck disable=SC2086
cargo fuzz coverage ${FUZZ_ARGS} "${CORPUS}" -- -use_value_profile=1 >/dev/null

# Show coverage
# shellcheck disable=SC2086
"$(rustc --print=target-libdir)/../bin/llvm-cov" show "$(find target -name roger | grep coverage)" ${COV_ARGS}

# Generate report
# shellcheck disable=SC2086
"$(rustc --print=target-libdir)/../bin/llvm-cov" report "$(find target -name roger | grep coverage)" ${COV_ARGS}
