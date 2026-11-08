// Copyright (c) 2025-2026 Evelyn Lewis
// SPDX-License-Identifier: MIT

//! Time utilites

#![cfg(feature = "draw")]

/// CPU tick counter
/// Does not use its argument
#[inline]
#[cfg(all(target_arch = "aarch64", not(fuzzing)))]
pub(super) fn instant(_prev: u64) -> u64 {
    let mut tick;
    unsafe {
        std::arch::asm!(
            "mrs x0, cntvct_el0",
            out("x0") tick
        );
    }
    tick
}

/// CPU tick counterA
/// Does not use its argument
#[inline]
#[cfg(all(any(target_arch = "x86", target_arch = "x86_64"), not(fuzzing)))]
pub(super) fn instant(_prev: u64) -> u64 {
    let mut eax_accumulator: u32;
    let mut edx_data: u32;
    unsafe {
        std::arch::asm!("rdtsc", out("eax") eax_accumulator, out("edx") edx_data);
    }
    u64::from(eax_accumulator) | u64::from(edx_data) << 32
}

/// Stub CPU tick counter
/// Returns a value greater than its argument
#[inline]
#[cfg(any(
    fuzzing,
    not(any(target_arch = "x86", target_arch = "x86_64", target_arch = "aarch64"))
))]
pub(super) fn instant(prev: u64) -> u64 {
    prev + 1
}
