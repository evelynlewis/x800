#!/bin/sh

# Copyright (c) 2024 Evelyn Lewis
# SPDX-License-Identifier: MIT

#
# Flamegraph generation script
#
# usage:
# ./util/flamegraph.sh SCRIPT
#
# Example:
# ./util/flamegraph.sh fuzz
#

set -e

test -d ./util/
test -d ./fuzz/

# Require one argument
if [ $# -eq 0 ]; then
	echo "Please provide an argument. An example:"
	echo "$0 fuzzu"
	exit 1
elif [ $# -gt 1 ]; then
	echo "Too many arguments provided"
	exit 1
fi
# Script to launch the target
LAUNCH_SCRIPT="./util/$1.sh"
echo "Launching fuzzer with script:"
if command -v "${LAUNCH_SCRIPT}"; then
	echo "Script is valid"
else
	echo "Script \"$1\" at \"${LAUNCH_SCRIPT}\" is invalid"
	exit 1
fi

# Setup fuzzer environment variables
# shellcheck source=fuzzenv.sh
. ./util/fuzzenv.sh
WARMUP_DURATION="4"

if pgrep "${FUZZER_NAME}"; then
	echo "A process named '${FUZZER_NAME}' is already running. Kill above PIDs to continue. You may also try: 
killall ${FUZZER_NAME}"
	exit 1
fi

echo "
Starting '${FUZZER_NAME}' launch script
"
sh -cx "${LAUNCH_SCRIPT}"

echo "
Waiting ${WARMUP_DURATION}s for process warmup."
sleep ${WARMUP_DURATION}

echo "
Sampling '${FUZZER_NAME}' process with flamegraph: press CTRL-C to stop.
"
sh -cx "flamegraph --deterministic --root --open --pid $(pgrep ${FUZZER_NAME})"
