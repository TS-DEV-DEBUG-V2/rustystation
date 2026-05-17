// PSX BIOS image embedded at compile time. The bytes live next to
// this file as bios.bin and are compiled directly into the
// executable's .rodata.
// this file is part of the RustyStation project
// Copyright 2025-2026 TS-DEV-DEBUG-V2
pub const BIOS_SIZE: usize = 524288;

pub static BIOS: &[u8; BIOS_SIZE] = include_bytes!("bios.bin");
