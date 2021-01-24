#![no_std]

#[cfg(all(feature = "bootloader", feature = "application"))]
compile_error!("Only 1 of bootloader or application may be selected at once");
