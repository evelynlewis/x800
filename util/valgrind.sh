#!/bin/sh

# Copyright (c) 2024-2026 Evelyn Lewis
# SPDX-License-Identifier: MIT

# valgrind.sh
# Run Valgrind tools on the `roger` fuzzer
# Using the -a flag prints annotation output
#
# usage:
# ./util/valgrind.sh
#
set -e

# Ensure we are in the root directory
if ! test -d fuzz; then
	echo "Error: must be run from root directory"
	echo "usage:"
	echo "./util/valgrind.sh"
	exit 1
fi

# shellcheck source=util/fuzz-env.sh
. util/fuzz-env.sh

cargo fuzz build --release --sanitizer none roger

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
