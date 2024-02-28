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

if pgrep "${FUZZER_NAME}"; then 
	echo "Fuzzer '${FUZZER_NAME}' is already running: kill above PIDs or use 'killall ${FUZZER_NAME}' to continue";
	exit 1;
fi

echo "Starting fuzzer launch script" 
./util/fuzz.sh

echo 'Waiting for fuzzer warmup'
sleep 4
echo 'Collecting flamegraph samples: press Control-C to stop'

sh -c "set -x; flamegraph --deterministic --root --open --pid $(pgrep ${FUZZER_NAME})"

