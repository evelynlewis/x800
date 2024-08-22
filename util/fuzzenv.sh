# Copyright (c) 2024 Evelyn Lewis
# SPDX-License-Identifier: MIT

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
-max_len=8192 \
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

# Run fuzzer using cargo-fuzz without args
fuzz_run_bench() {
	# shellcheck disable=SC2086
	cargo fuzz run "$@" ${FUZZER_BUILD_ARGS} "${FUZZER_NAME}"
}

# Run fuzzer using cargo-fuzz
fuzz_check() {
	# shellcheck disable=SC2086
	cargo fuzz run ${FUZZER_BUILD_ARGS} "${FUZZER_NAME}" "$@"
}
