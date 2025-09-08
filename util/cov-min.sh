#!/bin/sh

# Copyright (c) 2024-2025 Evelyn Lewis
# SPDX-License-Identifier: MIT

#
# Minimize fuzzer coverage
#
# usage:
# ./util/cov-min.sh [DIR]
#
set -ex

test -d ./fuzz
test -d ./util

MAIN_DIR="./fuzz/corpus/roger/"
ARCHIVE_DIR="./fuzz/corpus/roger-archive/"
TMP_DIR="./fuzz/corpus/roger-tmp/"

mkdir -p "${MAIN_DIR}" "${ARCHIVE_DIR}"
rsync -a "${MAIN_DIR}" "${ARCHIVE_DIR}"

rm -rf "${TMP_DIR}"
rsync -a "${MAIN_DIR}" "${TMP_DIR}"

# Clean out main dir
rm -rf "${MAIN_DIR}"
mkdir -p "${MAIN_DIR}"

# Merge corpus directories
EXTRA_FUZZER_RUN_ARGS="-merge=1 ${MAIN_DIR} ${TMP_DIR}"

# shellcheck source=fuzz-env.sh
. util/fuzz-env.sh

# Merge all corpus directories
# shellcheck disable=SC2119
fuzz_run
