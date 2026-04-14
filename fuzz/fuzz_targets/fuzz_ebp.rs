#![no_main]
use libfuzzer_sys::fuzz_target;
use rompatcher_formats::ebp::EbpPatcher;
use rompatcher_core::PatchFormat;

fuzz_target!(|data: &[u8]| {
    let _ = EbpPatcher::validate(data);
    let _ = EbpPatcher::metadata(data);

    let mut rom = vec![0u8; 1024];
    let patcher = EbpPatcher;
    let _ = patcher.apply(&mut rom, data);
});
