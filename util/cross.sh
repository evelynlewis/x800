#!/bin/sh

# Copyright (c) 2025 Evelyn Lewis
# SPDX-License-Identifier: MIT

set -eux

ARCH="
    aarch64-unknown-linux-gnu
    aarch64-unknown-linux-musl
    arm-unknown-linux-gnueabi
    arm-unknown-linux-gnueabihf
    arm-unknown-linux-musleabi
    arm-unknown-linux-musleabihf
    armv5te-unknown-linux-gnueabi
    armv5te-unknown-linux-musleabi
    armv7-unknown-linux-gnueabi
    armv7-unknown-linux-gnueabihf
    armv7-unknown-linux-musleabi
    armv7-unknown-linux-musleabihf
    i586-unknown-linux-gnu
    i586-unknown-linux-musl
    i686-unknown-linux-gnu
    i686-unknown-linux-musl
    powerpc-unknown-linux-gnu
    powerpc64-unknown-linux-gnu
    powerpc64le-unknown-linux-gnu
    riscv64gc-unknown-linux-gnu
    s390x-unknown-linux-gnu
    sparc64-unknown-linux-gnu
    thumbv7neon-unknown-linux-gnueabihf
    x86_64-unknown-linux-gnu
    x86_64-unknown-linux-musl
"

for a in $ARCH; do
    cross build --release --verbose --target "$a"
    sh -x ./util/gold.sh "$a"
done

