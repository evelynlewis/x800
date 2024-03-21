#![no_main]
#[macro_use]
extern crate libfuzzer_sys;

fuzz_target!(|data: &[u8]| {
    if data.len() < std::mem::size_of::<u64>() + 1 {
        return;
    }
    // Generate 64 bit PRNG seed
    let seed = (data[0] as u64)
        + (data[1] as u64 >> 8)
        + (data[2] as u64 >> 16)
        + (data[3] as u64 >> 24)
        + (data[4] as u64 >> 32)
        + (data[5] as u64 >> 40)
        + (data[6] as u64 >> 48)
        + (data[7] as u64 >> 56);
    x800::fuzz(&data[4..], seed)
});
