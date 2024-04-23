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
# usage:
# ./util/coverage.sh
#
set -e

# Ensure correct working directory
test -d ./util/
test -d ./fuzz/

# shellcheck disable=SC1091
. util/fenv.sh

# Corpus selection. Corpus can be passed as first argument
CORPUS="fuzz/corpus/${FUZZER_NAME}/"
if [ "$#" != 0 ]; then
	CORPUS=$1
fi
echo "Using corpus directory: '${CORPUS}'"

# Install dependencies
DEPS="rustfilt cargo-fuzz"
# shellcheck disable=SC2086
cargo install --quiet ${DEPS}

# Build fuzz target and generate coverage
# shellcheck disable=SC2086
cargo fuzz coverage ${FUZZER_NAME} ${FUZZER_BUILD_ARGS} "${CORPUS}" -- -use_value_profile=1 >/dev/null

# Ensure there is exactly one fuzzer, and that it is valid
FUZZER="$(find target -type f -name "${FUZZER_NAME}" | grep coverage)"
command -v "${FUZZER}"

wrap_llvm_cov() {
	COVERAGE="fuzz/coverage/${FUZZER_NAME}/coverage.profdata"

	"$(rustc --print=target-libdir)/../bin/llvm-cov" \
		"$@" "${FUZZER}" \
		-Xdemangler=rustfilt \
		-ignore-filename-regex='.cargo' \
		-ignore-filename-regex='/rustc/.+' \
		-use-color \
		-instr-profile="${COVERAGE}"
}

# Show coverage
wrap_llvm_cov show
# Generate report
wrap_llvm_cov report
