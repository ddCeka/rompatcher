#![no_main]
use libfuzzer_sys::fuzz_target;
use rompatcher_formats::bdf::BdfPatcher;
use rompatcher_core::PatchFormat;

fuzz_target!(|data: &[u8]| {
    // 1. Fuzz validation
    let _ = BdfPatcher::validate(data);
    
    // 2. Fuzz metadata
    let _ = BdfPatcher::metadata(data);

    // 3. Fuzz application
    let mut rom = vec![0u8; 256];
    let patcher = BdfPatcher;
    let _ = patcher.apply(&mut rom, data);
});
