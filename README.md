[![crates.io](https://img.shields.io/crates/d/jh71xx-pac.svg)](https://crates.io/crates/jh71xx-pac)
[![crates.io](https://img.shields.io/crates/v/jh71xx-pac.svg)](https://crates.io/crates/jh71xx-pac)
![Build Status](https://codeberg.org/weathered-steel/jh71xx-pac/workflows/CI/badge.svg)

# JH71xx Peripheral Access Crate

Development has moved to Codeberg: <https://codeberg.org/weathered-steel/jh71xx-pac>

Low-level access crates for JH71xx-based SoCs. Currently, only the JH7110 SoC is supported.

The JH7100 SoC is discontinued, and the next SoC in the JH-series will be the JH8110.

**WARNING** This crate is in the very earliest stages of development. SVD files used to generate the Rust code are partially hand-crafted from referencing the [JH7110 Technical Reference Manual, preliminary release V2](https://doc-en.rvspace.org/JH7110/PDF/JH7110_TRM_StarFive_Preliminary_V2.pdf). Any mistakes are my own, and reviews/contributions are welcome.

## Generating Rust Code from SVD

Code generation uses a modified fork of the `cmsis-svd-generator` tool from SiFive, with patches to generate StarFive peripherals.

Until changes are upstreamed, a development revision can be cloned from [rmsyn/cmsis-svd-generator#dev](https://github.com/rmsyn/cmsis-svd-generator/tree/dev). The tool requires [pydevicetree](https://pypi.org/project/pydevicetree/) to be installed in your python environment.

### Requirements:

- cargo-fmt
- pydevicetree
- svd2rust

To regenerate/update the Rust code:

```bash
./scripts/regen.sh
``` 
