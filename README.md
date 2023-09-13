# JH71xx Peripheral Access Crate

Low-level access crates for JH71xx-based SoCs and development boards.

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
# For JH7110 VisionFive2 v1.2a:
./scripts/regen.sh jh7110-vf2-12a-pac

# For JH7110 VisionFive2 v1.3b:
./scripts/regen.sh jh7110-vf2-13b-pac
``` 
