#!/bin/sh
# Copyright (c) 2024 Evelyn Lewis
# SPDX-License-Identifier: MIT

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
