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
test -f Cargo.toml

# shellcheck source=fenv.sh
. util/fenv.sh

# Corpus selection. Corpus can be passed as first argument
CORPUS="fuzz/corpus/${FUZZER_NAME}/"
if [ "$#" != 0 ]; then
	CORPUS=$1
fi
echo "Using corpus directory: '${CORPUS}'"

# Use HTML mode when invoked as such
USE_HTML=0
NAME="$(basename "$0")"
HTML_BIN_NAME="htmlcov.sh"
HTML_OUT_FILE="fuzz/target/roger.html"
HTML_ARGS=""
if [ "${NAME}" = "${HTML_BIN_NAME}" ]; then
	USE_HTML=1
	HTML_ARGS="-format=html"
fi

# Install dependencies
DEPS="rustfilt cargo-fuzz"
# shellcheck disable=SC2086
cargo install --quiet ${DEPS}

# Remove old coverage files
rm -rf ./fuzz/coverage/

# Build fuzz target and generate coverage
# shellcheck disable=SC2086
cargo fuzz coverage "${FUZZER_NAME}" ${FUZZER_BUILD_ARGS} "${CORPUS}" -- -use_value_profile=1 >/dev/null

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
		-instr-profile="${COVERAGE}" \
		"${HTML_ARGS}"
}

if [ "${USE_HTML}" ]; then
	# Save coverage to output file
	wrap_llvm_cov show >"${HTML_OUT_FILE}"
	echo "Saved coverage output in HTML format to ${HTML_OUT_FILE}"
else
	# Print coverage plaintext to terminal
	wrap_llvm_cov show
	# Print coverage report (summary) to terminal
	wrap_llvm_cov report
fi
