//! Peripheral access crates for JH71xx-based devices.

#![no_std]

#[cfg(any(feature = "visionfive2-12a", feature = "visionfive2-12a-rt", feature = "visionfive2-12a-rts"))]
pub extern crate jh7110_vf2_12a_pac;

#[cfg(any(feature = "visionfive2-13b", feature = "visionfive2-13b-rt", feature = "visionfive2-13b-rts"))]
pub extern crate jh7110_vf2_13b_pac;
