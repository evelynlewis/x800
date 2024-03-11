#!/bin/sh
set -e

exists() {
    command -v "${1}" >/dev/null
}

# Prefer the explicitly GNU tools
if exists gbase32; then
    BASE32="gbase32"
elif exists base32; then
    BASE32="base32"
else
    exit 1
fi

if exists gtr; then
    TR="gtr"
elif exists tr; then
    TR="tr"
else
    exit 1
fi

if exists gdd; then
    DD="gdd"
elif exists dd; then
    DD="dd"
else
    exit 1
fi

# Build
echo "Building x800 executable."
cargo build --release

# Other params
BS="2048"
SLEEP="16"
echo "Running benchmark for ${SLEEP} seconds."

# Configure cleanup
trap 'kill $(pgrep -P $$) 2> /dev/null' EXIT INT

# Run worker
${BASE32} </dev/urandom |
    ${TR} -dC 'ASDW' |
    ${DD} conv=lcase bs=${BS} status=progress |
    sh -c 'while true; do ./target/release/x800 >/dev/null; done' &

# Sleep and exit
sleep ${SLEEP}
exit 0
