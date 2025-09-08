#!/bin/sh
set -e

# Copyright (c) 2024-2025 Evelyn Lewis
# SPDX-License-Identifier: MIT

INPUT=fuzz/corpus/roger/gold
CANDIDATE=candidate.out.tmp
EXPECTED=fuzz/corpus/score/gold

if [ $# -lt 1 ]; then
	echo "$0: Error: TARGET required"
	echo
	echo "usage"
	echo "$0 TARGET"
	echo
	echo "Example:"
	echo "$0 aarch64-unknown-linux-gnu"
	exit 1
fi

cross run --target "$1" --features score --release --bin ryan "${INPUT}" 2>/dev/null >"${CANDIDATE}"

if [ "$(cat "${CANDIDATE}")" -ne "$(cat "${EXPECTED}")" ]; then
	printf "FAIL: WANT: %s FOUND: %s\n" "$(cat "${EXPECTED}")" "$(cat "${CANDIDATE}")"
	rm "${CANDIDATE}"
	exit 1
else
	printf "OK: %s\n" "$(cat "${CANDIDATE}")"
	rm "${CANDIDATE}"
	exit 0
fi
