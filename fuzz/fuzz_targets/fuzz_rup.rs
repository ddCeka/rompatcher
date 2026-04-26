#![no_main]
use libfuzzer_sys::fuzz_target;
use rompatcher_formats::rup::RupPatcher;
use rompatcher_core::PatchFormat;

fuzz_target!(|data: &[u8]| {
    let _ = RupPatcher::validate(data);
    let _ = RupPatcher::metadata(data);

    let mut rom = vec![0u8; 1024];
    let patcher = RupPatcher;
    let _ = patcher.apply(&mut rom, data);
});
