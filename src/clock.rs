// Copyright (c) 2025 Evelyn Lewis
// SPDX-License-Identifier: MIT

//! Time utilites

#![cfg(feature = "interactive")]

#[cfg(any(target_arch = "x86", target_arch = "x86_64", target_arch = "aarch64"))]
use std::arch;

/// CPU tick counter
/// Does not use its argument
#[inline]
#[cfg(target_arch = "aarch64")]
pub fn instant(_prev: u64) -> u64 {
    let mut tick;
    unsafe {
        arch::asm!(
            "mrs x0, cntvct_el0",
            out("x0") tick
        );
    }
    tick
}

/// CPU tick counterA
/// Does not use its argument
#[inline]
#[cfg(any(target_arch = "x86", target_arch = "x86_64"))]
pub fn instant(_prev: u64) -> u64 {
    let mut eax: u32;
    let mut edx: u32;
    unsafe {
        arch::asm!("rdtsc", out("eax") eax, out("edx") edx);
    }
    (eax as u64) | (edx as u64) << 32
}

/// Stub CPU tick counter
/// Returns a value greater than its argument
#[inline]
#[cfg(not(any(target_arch = "x86", target_arch = "x86_64", target_arch = "aarch64")))]
pub fn instant(prev: u64) -> u64 {
    prev + 1
}
