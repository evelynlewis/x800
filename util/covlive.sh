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

#
# usage:
# ./util/covlive.sh
#
set -e

test -d ./util/
# shellcheck source=common.sh
. ./util/common.sh

rm -r /tmp/roger/

PREVIOUS=""
while true; do
	rsync -a fuzz/corpus/roger/ /tmp/roger/
	CURRENT="$(stat -c %Y fuzz/corpus/roger)"
	if [ "${CURRENT}" != "${PREVIOUS}" ]; then
		plus_header "Changes detected: processing..."
		./util/covhtml.sh /tmp/roger

		PREVIOUS=${CURRENT}
		plus_header "Waiting for changes..."
	fi
	# Sleeping is not ideal, but the coverage script
	# takes around 3 seconds to run, so a one second
	# maximum additional delay is okay her
	sleep 1
done
