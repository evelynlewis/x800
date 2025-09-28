#!/bin/sh

# Copyright (c) 2024-2025 Evelyn Lewis
# SPDX-License-Identifier: MIT

#
# usage:
# ./util/cov.sh
#
set -e

# Ensure correct working directory
test -d ./util/
test -d ./fuzz/
test -f Cargo.toml

# shellcheck source=util/fuzz-env.sh
. util/fuzz-env.sh

# Corpus selection. Corpus can be passed as first argument
CORPUS="fuzz/corpus/${FUZZER_NAME}/"
if [ "$#" != 0 ]; then
	CORPUS=$1
fi
echo "Using corpus directory: '${CORPUS}'"

# Use HTML mode when invoked as such
USE_HTML=0
NAME="$(basename "$0")"
HTML_BIN_NAME="cov-html.sh"
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
cargo fuzz coverage "${FUZZER_NAME}" ${FUZZER_BUILD_ARGS} --features=default "${CORPUS}" -- -use_value_profile=1 >/dev/null

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
		${HTML_ARGS}
}

if [ "${USE_HTML}" -ne 0 ]; then
	# Save coverage to output file
	wrap_llvm_cov show >"${HTML_OUT_FILE}"
	echo "Saved coverage output in HTML format to ${HTML_OUT_FILE}"
else
	# Print coverage plaintext to terminal
	wrap_llvm_cov show
	# Print coverage report (summary) to terminal
	wrap_llvm_cov report
fi
