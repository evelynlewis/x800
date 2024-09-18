#!/bin/sh

# Copyright (c) 2024 Evelyn Lewis
# SPDX-License-Identifier: MIT

#
# A minimal x800 benchmark script
#
# usage:
# benchu.sh INPUT
#
set -e

if [ $# -lt 1 ]; then
    echo "Missing INPUT argument"
    echo "usage: $0 INPUT"
    exit 1
else
    INPUT="$1"
fi

# Many loops per iteration
LOOPS=10000
# Allow for the best of several iterations
TIMES=16

echo "Binary: $(command -V ryan)"
echo "Input: ${INPUT}"

echo
i=0
echo "Running ${LOOPS} loops, ${TIMES} times"
while [ $i -lt "${TIMES}" ]; do
    echo
    time -p ryan "${INPUT}" "${LOOPS}" >/dev/null
    i=$((i + 1))
done
echo
