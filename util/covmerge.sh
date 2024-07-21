#!/bin/sh

# Copyright (c) 2024 Evelyn Lewis
# SPDX-License-Identifier: MIT

#
# Merge archive corpus with main corpus
#
# usage:
# ./util/covmerge.sh
#
set -ex
test -d ./fuzz
test -d ./util

MAIN_DIR="./fuzz/corpus/roger/"
ARCHIVE_DIR="./fuzz/corpus/roger-archive/"

mkdir -p "${MAIN_DIR}" "${ARCHIVE_DIR}"
rsync -av "${MAIN_DIR}" "$@" "${ARCHIVE_DIR}"

# Merge corpus directories
EXTRA_FUZZER_RUN_ARGS="-merge=1 ${MAIN_DIR} ${ARCHIVE_DIR} $*"

# shellcheck source=fuzzenv.sh
. util/fuzzenv.sh

# Merge all corpus directories
# shellcheck disable=SC2119
fuzz_run
