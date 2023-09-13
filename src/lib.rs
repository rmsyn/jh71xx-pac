//! Peripheral access crates for JH71xx-based devices.

#![no_std]

#[cfg(feature = "visionfive2-12a")]
pub extern crate jh7110_vf2_12a_pac;

#[cfg(feature = "visionfive2-13b")]
pub extern crate jh7110_vf2_13b_pac;
