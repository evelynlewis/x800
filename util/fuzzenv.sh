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
# Fuzzer environment configuration
# This file should be sourced not run
#
# usage:
# . fuzzenv.sh
#

# shellcheck shell=sh

FUZZER_NAME="roger"

FUZZER_BUILD_ARGS="\
--sanitizer none \
--release \
${EXTRA_FUZZER_BUILD_ARGS} \
"

FUZZER_RUN_ARGS="\
-dict=./fuzz/dictionary.txt \
-max_len=32768 \
-use_value_profile=1 \
${EXTRA_FUZZER_RUN_ARGS} \
"

# Do a blocking build
fuzz_build() {
	# shellcheck disable=SC2086
	cargo fuzz build "$@" ${FUZZER_BUILD_ARGS} "${FUZZER_NAME}"
}

# Run fuzzer using cargo-fuzz
fuzz_run() {
	# shellcheck disable=SC2086
	cargo fuzz run "$@" ${FUZZER_BUILD_ARGS} "${FUZZER_NAME}" -- ${FUZZER_RUN_ARGS}
}

# Run fuzzer using cargo-fuzz
fuzz_check() {
	# shellcheck disable=SC2086
	cargo fuzz run ${FUZZER_BUILD_ARGS} "${FUZZER_NAME}" "$@"
}
