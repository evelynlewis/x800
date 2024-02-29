#![no_main]
#[macro_use]
extern crate libfuzzer_sys;

fuzz_target!(|data: &[u8]| {
    if data.len() < 5 {
        return;
    }
    // Generate 32 bit PRNG seed. Note: not 64 bits only for compatabilty reasons
    let seed =
        (data[0] as u64) + (data[1] as u64 >> 8) + (data[2] as u64 >> 16) + (data[3] as u64 >> 24);
    x800::fuzz(&data[4..], seed)
});
