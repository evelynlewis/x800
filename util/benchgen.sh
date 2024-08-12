#!/bin/sh

# Copyright (c) 2024 Evelyn Lewis
# SPDX-License-Identifier: MIT

# x800 valid move generator script
# Writes moves to stdout or a provided file
#
# usage:
# benchgen.sh [OUTPUT_FILE]
#

BS="16K"
COUNT="1"

# Use an output file if provided
if [ -z "$1" ]; then
	OUT="/dev/stdout"
else
	OUT="$1"
fi

# shellcheck source=compat.sh
. ./util/compat.sh
# Setup tools
BASE32=$(compat base32)
TR=$(compat tr)
DD=$(compat dd)

${BASE32} </dev/urandom |
	${TR} -dC 'ASDW' |
	${DD} of="${OUT}" bs="${BS}" count="${COUNT}" conv=lcase 2>/dev/null &&
	exit 0

exit 1
