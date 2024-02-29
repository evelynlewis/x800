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

set -e

test -d ./util/
test -d ./fuzz/

FUZZER_NAME="roger"
SCRIPT_ARGS="${*}"
WARMUP_DURATION="4"

if pgrep "${FUZZER_NAME}"; then
	echo "A process named '${FUZZER_NAME}' is already running. Kill above PIDs to continue. You may also try: 
killall ${FUZZER_NAME}"
	exit 1
fi

echo "
Starting '${FUZZER_NAME}' launch script with args: ${SCRIPT_ARGS}
"
sh -c "set -x; ./util/fuzz.sh ${SCRIPT_ARGS}"

echo "
Waiting ${WARMUP_DURATION}s for process warmup."
sleep ${WARMUP_DURATION}

echo "
Sampling '${FUZZER_NAME}' process with flamegraph: press CTRL-C to stop.
"
sh -c "set -x; flamegraph --deterministic --root --open --pid $(pgrep ${FUZZER_NAME})"
