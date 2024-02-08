#!/bin/sh

if [ -z "$1" ]; then
	OUT="/dev/stdout"
else
	OUT="${1}"
fi

BS="8M"
COUNT="1"

# Check to see if we need a modifed variant of the command
if command -v base32; then
	BASE32="base32"
elif command -v gbase32; then
	BASE32="gbase32"
fi

if command -v "${BASE32}" && command -v tr && command -v dd; then
	"${BASE32}" /dev/urandom \
		| tr '[:upper:]' '[:lower:]' \
		| tr -dC asdw \
		| dd of="${OUT}" bs="${BS}" count="${COUNT}" \
		&& exit 0
fi

exit 1

