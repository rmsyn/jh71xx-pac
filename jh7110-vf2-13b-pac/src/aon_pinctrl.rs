#[doc = r"Register block"]
#[repr(C)]
pub struct RegisterBlock {
    #[doc = "0x00 - AON IOMUX CFG SAIF SYSCFG FMUX 0"]
    pub aon_iomux_cfgsaif_syscfg_fmux0: AON_IOMUX_CFGSAIF_SYSCFG_FMUX0,
    #[doc = "0x04 - AON IOMUX CFG SAIF SYSCFG FMUX 1"]
    pub aon_iomux_cfgsaif_syscfg_fmux1: AON_IOMUX_CFGSAIF_SYSCFG_FMUX1,
    #[doc = "0x08 - AON IOMUX CFG SAIF SYSCFG FMUX 2"]
    pub aon_iomux_cfgsaif_syscfg_fmux2: AON_IOMUX_CFGSAIF_SYSCFG_FMUX2,
    #[doc = "0x0c - AON IOMUX CFG SAIF SYSCFG FMUX 3"]
    pub aon_iomux_cfgsaif_syscfg_fmux3: AON_IOMUX_CFGSAIF_SYSCFG_FMUX3,
    #[doc = "0x10 - AON IOMUX CFG SAIF SYSCFG IOIRQ 4"]
    pub aon_iomux_cfgsaif_syscfg_ioirq4: AON_IOMUX_CFGSAIF_SYSCFG_IOIRQ4,
    #[doc = "0x14 - AON IOMUX CFG SAIF SYSCFG IOIRQ 5"]
    pub aon_iomux_cfgsaif_syscfg_ioirq5: AON_IOMUX_CFGSAIF_SYSCFG_IOIRQ5,
    #[doc = "0x18 - AON IOMUX CFG SAIF SYSCFG IOIRQ 6"]
    pub aon_iomux_cfgsaif_syscfg_ioirq6: AON_IOMUX_CFGSAIF_SYSCFG_IOIRQ6,
    #[doc = "0x1c - AON IOMUX CFG SAIF SYSCFG IOIRQ 7"]
    pub aon_iomux_cfgsaif_syscfg_ioirq7: AON_IOMUX_CFGSAIF_SYSCFG_IOIRQ7,
    #[doc = "0x20 - AON IOMUX CFG SAIF SYSCFG IOIRQ 8"]
    pub aon_iomux_cfgsaif_syscfg_ioirq8: AON_IOMUX_CFGSAIF_SYSCFG_IOIRQ8,
    #[doc = "0x24 - AON IOMUX CFG SAIF SYSCFG IOIRQ 9"]
    pub aon_iomux_cfgsaif_syscfg_ioirq9: AON_IOMUX_CFGSAIF_SYSCFG_IOIRQ9,
    #[doc = "0x28 - AON IOMUX CFG SAIF SYSCFG IOIRQ 10"]
    pub aon_iomux_cfgsaif_syscfg_ioirq10: AON_IOMUX_CFGSAIF_SYSCFG_IOIRQ10,
    #[doc = "0x2c - AON IOMUX CFG SAIF SYSCFG IOIRQ 11"]
    pub aon_iomux_cfgsaif_syscfg_ioirq11: AON_IOMUX_CFGSAIF_SYSCFG_IOIRQ11,
    #[doc = "0x30 - AON IOMUX CFG SAIF SYSCFG 48"]
    pub aon_iomux_cfgsaif_syscfg48: AON_IOMUX_CFGSAIF_SYSCFG48,
    #[doc = "0x34 - AON IOMUX CFG SAIF SYSCFG 52"]
    pub aon_iomux_cfgsaif_syscfg52: AON_IOMUX_CFGSAIF_SYSCFG52,
    #[doc = "0x38 - AON IOMUX CFG SAIF SYSCFG 56"]
    pub aon_iomux_cfgsaif_syscfg56: AON_IOMUX_CFGSAIF_SYSCFG56,
    #[doc = "0x3c - AON IOMUX CFG SAIF SYSCFG 60"]
    pub aon_iomux_cfgsaif_syscfg60: AON_IOMUX_CFGSAIF_SYSCFG60,
    #[doc = "0x40 - AON IOMUX CFG SAIF SYSCFG 64"]
    pub aon_iomux_cfgsaif_syscfg64: AON_IOMUX_CFGSAIF_SYSCFG64,
    #[doc = "0x44 - AON IOMUX CFG SAIF SYSCFG 68"]
    pub aon_iomux_cfgsaif_syscfg68: AON_IOMUX_CFGSAIF_SYSCFG68,
    _reserved18: [u8; 0x04],
    #[doc = "0x4c - AON IOMUX CFG SAIF SYSCFG 76"]
    pub aon_iomux_cfgsaif_syscfg76: AON_IOMUX_CFGSAIF_SYSCFG76,
    _reserved19: [u8; 0x04],
    #[doc = "0x54 - AON IOMUX CFG SAIF SYSCFG 84"]
    pub aon_iomux_cfgsaif_syscfg84: AON_IOMUX_CFGSAIF_SYSCFG84,
    #[doc = "0x58 - AON IOMUX CFG SAIF SYSCFG 88"]
    pub aon_iomux_cfgsaif_syscfg88: AON_IOMUX_CFGSAIF_SYSCFG88,
    #[doc = "0x5c - AON IOMUX CFG SAIF SYSCFG 92"]
    pub aon_iomux_cfgsaif_syscfg92: AON_IOMUX_CFGSAIF_SYSCFG92,
    #[doc = "0x60 - AON IOMUX CFG SAIF SYSCFG 96"]
    pub aon_iomux_cfgsaif_syscfg96: AON_IOMUX_CFGSAIF_SYSCFG96,
    #[doc = "0x64 - AON IOMUX CFG SAIF SYSCFG 100"]
    pub aon_iomux_cfgsaif_syscfg100: AON_IOMUX_CFGSAIF_SYSCFG100,
    #[doc = "0x68 - AON IOMUX CFG SAIF SYSCFG 104"]
    pub aon_iomux_cfgsaif_syscfg104: AON_IOMUX_CFGSAIF_SYSCFG104,
    #[doc = "0x6c - AON IOMUX CFG SAIF SYSCFG 108"]
    pub aon_iomux_cfgsaif_syscfg108: AON_IOMUX_CFGSAIF_SYSCFG108,
    #[doc = "0x70 - AON IOMUX CFG SAIF SYSCFG 112"]
    pub aon_iomux_cfgsaif_syscfg112: AON_IOMUX_CFGSAIF_SYSCFG112,
    #[doc = "0x74 - AON IOMUX CFG SAIF SYSCFG 116"]
    pub aon_iomux_cfgsaif_syscfg116: AON_IOMUX_CFGSAIF_SYSCFG116,
    #[doc = "0x78 - AON IOMUX CFG SAIF SYSCFG 120"]
    pub aon_iomux_cfgsaif_syscfg120: AON_IOMUX_CFGSAIF_SYSCFG120,
    #[doc = "0x7c - AON IOMUX CFG SAIF SYSCFG 124"]
    pub aon_iomux_cfgsaif_syscfg124: AON_IOMUX_CFGSAIF_SYSCFG124,
    #[doc = "0x80 - AON IOMUX CFG SAIF SYSCFG 128"]
    pub aon_iomux_cfgsaif_syscfg128: AON_IOMUX_CFGSAIF_SYSCFG128,
    #[doc = "0x84 - AON IOMUX CFG SAIF SYSCFG 132"]
    pub aon_iomux_cfgsaif_syscfg132: AON_IOMUX_CFGSAIF_SYSCFG132,
    #[doc = "0x88 - AON IOMUX CFG SAIF SYSCFG 136"]
    pub aon_iomux_cfgsaif_syscfg136: AON_IOMUX_CFGSAIF_SYSCFG136,
    #[doc = "0x8c - AON IOMUX CFG SAIF SYSCFG 140"]
    pub aon_iomux_cfgsaif_syscfg140: AON_IOMUX_CFGSAIF_SYSCFG140,
    #[doc = "0x90 - AON IOMUX CFG SAIF SYSCFG 144"]
    pub aon_iomux_cfgsaif_syscfg144: AON_IOMUX_CFGSAIF_SYSCFG144,
}
#[doc = "aon_iomux_cfgsaif_syscfg_fmux0 (rw) register accessor: AON IOMUX CFG SAIF SYSCFG FMUX 0\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`aon_iomux_cfgsaif_syscfg_fmux0::R`].  You can [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`aon_iomux_cfgsaif_syscfg_fmux0::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`aon_iomux_cfgsaif_syscfg_fmux0`]
module"]
pub type AON_IOMUX_CFGSAIF_SYSCFG_FMUX0 =
    crate::Reg<aon_iomux_cfgsaif_syscfg_fmux0::AON_IOMUX_CFGSAIF_SYSCFG_FMUX0_SPEC>;
#[doc = "AON IOMUX CFG SAIF SYSCFG FMUX 0"]
pub mod aon_iomux_cfgsaif_syscfg_fmux0;
#[doc = "aon_iomux_cfgsaif_syscfg_fmux1 (rw) register accessor: AON IOMUX CFG SAIF SYSCFG FMUX 1\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`aon_iomux_cfgsaif_syscfg_fmux1::R`].  You can [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`aon_iomux_cfgsaif_syscfg_fmux1::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`aon_iomux_cfgsaif_syscfg_fmux1`]
module"]
pub type AON_IOMUX_CFGSAIF_SYSCFG_FMUX1 =
    crate::Reg<aon_iomux_cfgsaif_syscfg_fmux1::AON_IOMUX_CFGSAIF_SYSCFG_FMUX1_SPEC>;
#[doc = "AON IOMUX CFG SAIF SYSCFG FMUX 1"]
pub mod aon_iomux_cfgsaif_syscfg_fmux1;
#[doc = "aon_iomux_cfgsaif_syscfg_fmux2 (rw) register accessor: AON IOMUX CFG SAIF SYSCFG FMUX 2\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`aon_iomux_cfgsaif_syscfg_fmux2::R`].  You can [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`aon_iomux_cfgsaif_syscfg_fmux2::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`aon_iomux_cfgsaif_syscfg_fmux2`]
module"]
pub type AON_IOMUX_CFGSAIF_SYSCFG_FMUX2 =
    crate::Reg<aon_iomux_cfgsaif_syscfg_fmux2::AON_IOMUX_CFGSAIF_SYSCFG_FMUX2_SPEC>;
#[doc = "AON IOMUX CFG SAIF SYSCFG FMUX 2"]
pub mod aon_iomux_cfgsaif_syscfg_fmux2;
#[doc = "aon_iomux_cfgsaif_syscfg_fmux3 (rw) register accessor: AON IOMUX CFG SAIF SYSCFG FMUX 3\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`aon_iomux_cfgsaif_syscfg_fmux3::R`].  You can [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`aon_iomux_cfgsaif_syscfg_fmux3::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`aon_iomux_cfgsaif_syscfg_fmux3`]
module"]
pub type AON_IOMUX_CFGSAIF_SYSCFG_FMUX3 =
    crate::Reg<aon_iomux_cfgsaif_syscfg_fmux3::AON_IOMUX_CFGSAIF_SYSCFG_FMUX3_SPEC>;
#[doc = "AON IOMUX CFG SAIF SYSCFG FMUX 3"]
pub mod aon_iomux_cfgsaif_syscfg_fmux3;
#[doc = "aon_iomux_cfgsaif_syscfg_ioirq4 (rw) register accessor: AON IOMUX CFG SAIF SYSCFG IOIRQ 4\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`aon_iomux_cfgsaif_syscfg_ioirq4::R`].  You can [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`aon_iomux_cfgsaif_syscfg_ioirq4::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`aon_iomux_cfgsaif_syscfg_ioirq4`]
module"]
pub type AON_IOMUX_CFGSAIF_SYSCFG_IOIRQ4 =
    crate::Reg<aon_iomux_cfgsaif_syscfg_ioirq4::AON_IOMUX_CFGSAIF_SYSCFG_IOIRQ4_SPEC>;
#[doc = "AON IOMUX CFG SAIF SYSCFG IOIRQ 4"]
pub mod aon_iomux_cfgsaif_syscfg_ioirq4;
#[doc = "aon_iomux_cfgsaif_syscfg_ioirq5 (rw) register accessor: AON IOMUX CFG SAIF SYSCFG IOIRQ 5\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`aon_iomux_cfgsaif_syscfg_ioirq5::R`].  You can [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`aon_iomux_cfgsaif_syscfg_ioirq5::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`aon_iomux_cfgsaif_syscfg_ioirq5`]
module"]
pub type AON_IOMUX_CFGSAIF_SYSCFG_IOIRQ5 =
    crate::Reg<aon_iomux_cfgsaif_syscfg_ioirq5::AON_IOMUX_CFGSAIF_SYSCFG_IOIRQ5_SPEC>;
#[doc = "AON IOMUX CFG SAIF SYSCFG IOIRQ 5"]
pub mod aon_iomux_cfgsaif_syscfg_ioirq5;
#[doc = "aon_iomux_cfgsaif_syscfg_ioirq6 (rw) register accessor: AON IOMUX CFG SAIF SYSCFG IOIRQ 6\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`aon_iomux_cfgsaif_syscfg_ioirq6::R`].  You can [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`aon_iomux_cfgsaif_syscfg_ioirq6::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`aon_iomux_cfgsaif_syscfg_ioirq6`]
module"]
pub type AON_IOMUX_CFGSAIF_SYSCFG_IOIRQ6 =
    crate::Reg<aon_iomux_cfgsaif_syscfg_ioirq6::AON_IOMUX_CFGSAIF_SYSCFG_IOIRQ6_SPEC>;
#[doc = "AON IOMUX CFG SAIF SYSCFG IOIRQ 6"]
pub mod aon_iomux_cfgsaif_syscfg_ioirq6;
#[doc = "aon_iomux_cfgsaif_syscfg_ioirq7 (rw) register accessor: AON IOMUX CFG SAIF SYSCFG IOIRQ 7\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`aon_iomux_cfgsaif_syscfg_ioirq7::R`].  You can [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`aon_iomux_cfgsaif_syscfg_ioirq7::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`aon_iomux_cfgsaif_syscfg_ioirq7`]
module"]
pub type AON_IOMUX_CFGSAIF_SYSCFG_IOIRQ7 =
    crate::Reg<aon_iomux_cfgsaif_syscfg_ioirq7::AON_IOMUX_CFGSAIF_SYSCFG_IOIRQ7_SPEC>;
#[doc = "AON IOMUX CFG SAIF SYSCFG IOIRQ 7"]
pub mod aon_iomux_cfgsaif_syscfg_ioirq7;
#[doc = "aon_iomux_cfgsaif_syscfg_ioirq8 (rw) register accessor: AON IOMUX CFG SAIF SYSCFG IOIRQ 8\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`aon_iomux_cfgsaif_syscfg_ioirq8::R`].  You can [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`aon_iomux_cfgsaif_syscfg_ioirq8::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`aon_iomux_cfgsaif_syscfg_ioirq8`]
module"]
pub type AON_IOMUX_CFGSAIF_SYSCFG_IOIRQ8 =
    crate::Reg<aon_iomux_cfgsaif_syscfg_ioirq8::AON_IOMUX_CFGSAIF_SYSCFG_IOIRQ8_SPEC>;
#[doc = "AON IOMUX CFG SAIF SYSCFG IOIRQ 8"]
pub mod aon_iomux_cfgsaif_syscfg_ioirq8;
#[doc = "aon_iomux_cfgsaif_syscfg_ioirq9 (rw) register accessor: AON IOMUX CFG SAIF SYSCFG IOIRQ 9\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`aon_iomux_cfgsaif_syscfg_ioirq9::R`].  You can [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`aon_iomux_cfgsaif_syscfg_ioirq9::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`aon_iomux_cfgsaif_syscfg_ioirq9`]
module"]
pub type AON_IOMUX_CFGSAIF_SYSCFG_IOIRQ9 =
    crate::Reg<aon_iomux_cfgsaif_syscfg_ioirq9::AON_IOMUX_CFGSAIF_SYSCFG_IOIRQ9_SPEC>;
#[doc = "AON IOMUX CFG SAIF SYSCFG IOIRQ 9"]
pub mod aon_iomux_cfgsaif_syscfg_ioirq9;
#[doc = "aon_iomux_cfgsaif_syscfg_ioirq10 (rw) register accessor: AON IOMUX CFG SAIF SYSCFG IOIRQ 10\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`aon_iomux_cfgsaif_syscfg_ioirq10::R`].  You can [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`aon_iomux_cfgsaif_syscfg_ioirq10::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`aon_iomux_cfgsaif_syscfg_ioirq10`]
module"]
pub type AON_IOMUX_CFGSAIF_SYSCFG_IOIRQ10 =
    crate::Reg<aon_iomux_cfgsaif_syscfg_ioirq10::AON_IOMUX_CFGSAIF_SYSCFG_IOIRQ10_SPEC>;
#[doc = "AON IOMUX CFG SAIF SYSCFG IOIRQ 10"]
pub mod aon_iomux_cfgsaif_syscfg_ioirq10;
#[doc = "aon_iomux_cfgsaif_syscfg_ioirq11 (rw) register accessor: AON IOMUX CFG SAIF SYSCFG IOIRQ 11\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`aon_iomux_cfgsaif_syscfg_ioirq11::R`].  You can [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`aon_iomux_cfgsaif_syscfg_ioirq11::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`aon_iomux_cfgsaif_syscfg_ioirq11`]
module"]
pub type AON_IOMUX_CFGSAIF_SYSCFG_IOIRQ11 =
    crate::Reg<aon_iomux_cfgsaif_syscfg_ioirq11::AON_IOMUX_CFGSAIF_SYSCFG_IOIRQ11_SPEC>;
#[doc = "AON IOMUX CFG SAIF SYSCFG IOIRQ 11"]
pub mod aon_iomux_cfgsaif_syscfg_ioirq11;
#[doc = "aon_iomux_cfgsaif_syscfg48 (rw) register accessor: AON IOMUX CFG SAIF SYSCFG 48\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`aon_iomux_cfgsaif_syscfg48::R`].  You can [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`aon_iomux_cfgsaif_syscfg48::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`aon_iomux_cfgsaif_syscfg48`]
module"]
pub type AON_IOMUX_CFGSAIF_SYSCFG48 =
    crate::Reg<aon_iomux_cfgsaif_syscfg48::AON_IOMUX_CFGSAIF_SYSCFG48_SPEC>;
#[doc = "AON IOMUX CFG SAIF SYSCFG 48"]
pub mod aon_iomux_cfgsaif_syscfg48;
#[doc = "aon_iomux_cfgsaif_syscfg52 (rw) register accessor: AON IOMUX CFG SAIF SYSCFG 52\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`aon_iomux_cfgsaif_syscfg52::R`].  You can [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`aon_iomux_cfgsaif_syscfg52::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`aon_iomux_cfgsaif_syscfg52`]
module"]
pub type AON_IOMUX_CFGSAIF_SYSCFG52 =
    crate::Reg<aon_iomux_cfgsaif_syscfg52::AON_IOMUX_CFGSAIF_SYSCFG52_SPEC>;
#[doc = "AON IOMUX CFG SAIF SYSCFG 52"]
pub mod aon_iomux_cfgsaif_syscfg52;
#[doc = "aon_iomux_cfgsaif_syscfg56 (rw) register accessor: AON IOMUX CFG SAIF SYSCFG 56\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`aon_iomux_cfgsaif_syscfg56::R`].  You can [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`aon_iomux_cfgsaif_syscfg56::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`aon_iomux_cfgsaif_syscfg56`]
module"]
pub type AON_IOMUX_CFGSAIF_SYSCFG56 =
    crate::Reg<aon_iomux_cfgsaif_syscfg56::AON_IOMUX_CFGSAIF_SYSCFG56_SPEC>;
#[doc = "AON IOMUX CFG SAIF SYSCFG 56"]
pub mod aon_iomux_cfgsaif_syscfg56;
#[doc = "aon_iomux_cfgsaif_syscfg60 (rw) register accessor: AON IOMUX CFG SAIF SYSCFG 60\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`aon_iomux_cfgsaif_syscfg60::R`].  You can [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`aon_iomux_cfgsaif_syscfg60::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`aon_iomux_cfgsaif_syscfg60`]
module"]
pub type AON_IOMUX_CFGSAIF_SYSCFG60 =
    crate::Reg<aon_iomux_cfgsaif_syscfg60::AON_IOMUX_CFGSAIF_SYSCFG60_SPEC>;
#[doc = "AON IOMUX CFG SAIF SYSCFG 60"]
pub mod aon_iomux_cfgsaif_syscfg60;
#[doc = "aon_iomux_cfgsaif_syscfg64 (rw) register accessor: AON IOMUX CFG SAIF SYSCFG 64\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`aon_iomux_cfgsaif_syscfg64::R`].  You can [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`aon_iomux_cfgsaif_syscfg64::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`aon_iomux_cfgsaif_syscfg64`]
module"]
pub type AON_IOMUX_CFGSAIF_SYSCFG64 =
    crate::Reg<aon_iomux_cfgsaif_syscfg64::AON_IOMUX_CFGSAIF_SYSCFG64_SPEC>;
#[doc = "AON IOMUX CFG SAIF SYSCFG 64"]
pub mod aon_iomux_cfgsaif_syscfg64;
#[doc = "aon_iomux_cfgsaif_syscfg68 (rw) register accessor: AON IOMUX CFG SAIF SYSCFG 68\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`aon_iomux_cfgsaif_syscfg68::R`].  You can [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`aon_iomux_cfgsaif_syscfg68::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`aon_iomux_cfgsaif_syscfg68`]
module"]
pub type AON_IOMUX_CFGSAIF_SYSCFG68 =
    crate::Reg<aon_iomux_cfgsaif_syscfg68::AON_IOMUX_CFGSAIF_SYSCFG68_SPEC>;
#[doc = "AON IOMUX CFG SAIF SYSCFG 68"]
pub mod aon_iomux_cfgsaif_syscfg68;
#[doc = "aon_iomux_cfgsaif_syscfg76 (rw) register accessor: AON IOMUX CFG SAIF SYSCFG 76\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`aon_iomux_cfgsaif_syscfg76::R`].  You can [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`aon_iomux_cfgsaif_syscfg76::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`aon_iomux_cfgsaif_syscfg76`]
module"]
pub type AON_IOMUX_CFGSAIF_SYSCFG76 =
    crate::Reg<aon_iomux_cfgsaif_syscfg76::AON_IOMUX_CFGSAIF_SYSCFG76_SPEC>;
#[doc = "AON IOMUX CFG SAIF SYSCFG 76"]
pub mod aon_iomux_cfgsaif_syscfg76;
#[doc = "aon_iomux_cfgsaif_syscfg84 (rw) register accessor: AON IOMUX CFG SAIF SYSCFG 84\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`aon_iomux_cfgsaif_syscfg84::R`].  You can [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`aon_iomux_cfgsaif_syscfg84::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`aon_iomux_cfgsaif_syscfg84`]
module"]
pub type AON_IOMUX_CFGSAIF_SYSCFG84 =
    crate::Reg<aon_iomux_cfgsaif_syscfg84::AON_IOMUX_CFGSAIF_SYSCFG84_SPEC>;
#[doc = "AON IOMUX CFG SAIF SYSCFG 84"]
pub mod aon_iomux_cfgsaif_syscfg84;
#[doc = "aon_iomux_cfgsaif_syscfg88 (rw) register accessor: AON IOMUX CFG SAIF SYSCFG 88\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`aon_iomux_cfgsaif_syscfg88::R`].  You can [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`aon_iomux_cfgsaif_syscfg88::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`aon_iomux_cfgsaif_syscfg88`]
module"]
pub type AON_IOMUX_CFGSAIF_SYSCFG88 =
    crate::Reg<aon_iomux_cfgsaif_syscfg88::AON_IOMUX_CFGSAIF_SYSCFG88_SPEC>;
#[doc = "AON IOMUX CFG SAIF SYSCFG 88"]
pub mod aon_iomux_cfgsaif_syscfg88;
#[doc = "aon_iomux_cfgsaif_syscfg92 (rw) register accessor: AON IOMUX CFG SAIF SYSCFG 92\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`aon_iomux_cfgsaif_syscfg92::R`].  You can [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`aon_iomux_cfgsaif_syscfg92::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`aon_iomux_cfgsaif_syscfg92`]
module"]
pub type AON_IOMUX_CFGSAIF_SYSCFG92 =
    crate::Reg<aon_iomux_cfgsaif_syscfg92::AON_IOMUX_CFGSAIF_SYSCFG92_SPEC>;
#[doc = "AON IOMUX CFG SAIF SYSCFG 92"]
pub mod aon_iomux_cfgsaif_syscfg92;
#[doc = "aon_iomux_cfgsaif_syscfg96 (rw) register accessor: AON IOMUX CFG SAIF SYSCFG 96\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`aon_iomux_cfgsaif_syscfg96::R`].  You can [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`aon_iomux_cfgsaif_syscfg96::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`aon_iomux_cfgsaif_syscfg96`]
module"]
pub type AON_IOMUX_CFGSAIF_SYSCFG96 =
    crate::Reg<aon_iomux_cfgsaif_syscfg96::AON_IOMUX_CFGSAIF_SYSCFG96_SPEC>;
#[doc = "AON IOMUX CFG SAIF SYSCFG 96"]
pub mod aon_iomux_cfgsaif_syscfg96;
#[doc = "aon_iomux_cfgsaif_syscfg100 (rw) register accessor: AON IOMUX CFG SAIF SYSCFG 100\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`aon_iomux_cfgsaif_syscfg100::R`].  You can [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`aon_iomux_cfgsaif_syscfg100::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`aon_iomux_cfgsaif_syscfg100`]
module"]
pub type AON_IOMUX_CFGSAIF_SYSCFG100 =
    crate::Reg<aon_iomux_cfgsaif_syscfg100::AON_IOMUX_CFGSAIF_SYSCFG100_SPEC>;
#[doc = "AON IOMUX CFG SAIF SYSCFG 100"]
pub mod aon_iomux_cfgsaif_syscfg100;
#[doc = "aon_iomux_cfgsaif_syscfg104 (rw) register accessor: AON IOMUX CFG SAIF SYSCFG 104\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`aon_iomux_cfgsaif_syscfg104::R`].  You can [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`aon_iomux_cfgsaif_syscfg104::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`aon_iomux_cfgsaif_syscfg104`]
module"]
pub type AON_IOMUX_CFGSAIF_SYSCFG104 =
    crate::Reg<aon_iomux_cfgsaif_syscfg104::AON_IOMUX_CFGSAIF_SYSCFG104_SPEC>;
#[doc = "AON IOMUX CFG SAIF SYSCFG 104"]
pub mod aon_iomux_cfgsaif_syscfg104;
#[doc = "aon_iomux_cfgsaif_syscfg108 (rw) register accessor: AON IOMUX CFG SAIF SYSCFG 108\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`aon_iomux_cfgsaif_syscfg108::R`].  You can [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`aon_iomux_cfgsaif_syscfg108::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`aon_iomux_cfgsaif_syscfg108`]
module"]
pub type AON_IOMUX_CFGSAIF_SYSCFG108 =
    crate::Reg<aon_iomux_cfgsaif_syscfg108::AON_IOMUX_CFGSAIF_SYSCFG108_SPEC>;
#[doc = "AON IOMUX CFG SAIF SYSCFG 108"]
pub mod aon_iomux_cfgsaif_syscfg108;
#[doc = "aon_iomux_cfgsaif_syscfg112 (rw) register accessor: AON IOMUX CFG SAIF SYSCFG 112\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`aon_iomux_cfgsaif_syscfg112::R`].  You can [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`aon_iomux_cfgsaif_syscfg112::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`aon_iomux_cfgsaif_syscfg112`]
module"]
pub type AON_IOMUX_CFGSAIF_SYSCFG112 =
    crate::Reg<aon_iomux_cfgsaif_syscfg112::AON_IOMUX_CFGSAIF_SYSCFG112_SPEC>;
#[doc = "AON IOMUX CFG SAIF SYSCFG 112"]
pub mod aon_iomux_cfgsaif_syscfg112;
#[doc = "aon_iomux_cfgsaif_syscfg116 (rw) register accessor: AON IOMUX CFG SAIF SYSCFG 116\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`aon_iomux_cfgsaif_syscfg116::R`].  You can [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`aon_iomux_cfgsaif_syscfg116::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`aon_iomux_cfgsaif_syscfg116`]
module"]
pub type AON_IOMUX_CFGSAIF_SYSCFG116 =
    crate::Reg<aon_iomux_cfgsaif_syscfg116::AON_IOMUX_CFGSAIF_SYSCFG116_SPEC>;
#[doc = "AON IOMUX CFG SAIF SYSCFG 116"]
pub mod aon_iomux_cfgsaif_syscfg116;
#[doc = "aon_iomux_cfgsaif_syscfg120 (rw) register accessor: AON IOMUX CFG SAIF SYSCFG 120\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`aon_iomux_cfgsaif_syscfg120::R`].  You can [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`aon_iomux_cfgsaif_syscfg120::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`aon_iomux_cfgsaif_syscfg120`]
module"]
pub type AON_IOMUX_CFGSAIF_SYSCFG120 =
    crate::Reg<aon_iomux_cfgsaif_syscfg120::AON_IOMUX_CFGSAIF_SYSCFG120_SPEC>;
#[doc = "AON IOMUX CFG SAIF SYSCFG 120"]
pub mod aon_iomux_cfgsaif_syscfg120;
#[doc = "aon_iomux_cfgsaif_syscfg124 (rw) register accessor: AON IOMUX CFG SAIF SYSCFG 124\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`aon_iomux_cfgsaif_syscfg124::R`].  You can [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`aon_iomux_cfgsaif_syscfg124::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`aon_iomux_cfgsaif_syscfg124`]
module"]
pub type AON_IOMUX_CFGSAIF_SYSCFG124 =
    crate::Reg<aon_iomux_cfgsaif_syscfg124::AON_IOMUX_CFGSAIF_SYSCFG124_SPEC>;
#[doc = "AON IOMUX CFG SAIF SYSCFG 124"]
pub mod aon_iomux_cfgsaif_syscfg124;
#[doc = "aon_iomux_cfgsaif_syscfg128 (rw) register accessor: AON IOMUX CFG SAIF SYSCFG 128\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`aon_iomux_cfgsaif_syscfg128::R`].  You can [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`aon_iomux_cfgsaif_syscfg128::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`aon_iomux_cfgsaif_syscfg128`]
module"]
pub type AON_IOMUX_CFGSAIF_SYSCFG128 =
    crate::Reg<aon_iomux_cfgsaif_syscfg128::AON_IOMUX_CFGSAIF_SYSCFG128_SPEC>;
#[doc = "AON IOMUX CFG SAIF SYSCFG 128"]
pub mod aon_iomux_cfgsaif_syscfg128;
#[doc = "aon_iomux_cfgsaif_syscfg132 (rw) register accessor: AON IOMUX CFG SAIF SYSCFG 132\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`aon_iomux_cfgsaif_syscfg132::R`].  You can [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`aon_iomux_cfgsaif_syscfg132::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`aon_iomux_cfgsaif_syscfg132`]
module"]
pub type AON_IOMUX_CFGSAIF_SYSCFG132 =
    crate::Reg<aon_iomux_cfgsaif_syscfg132::AON_IOMUX_CFGSAIF_SYSCFG132_SPEC>;
#[doc = "AON IOMUX CFG SAIF SYSCFG 132"]
pub mod aon_iomux_cfgsaif_syscfg132;
#[doc = "aon_iomux_cfgsaif_syscfg136 (rw) register accessor: AON IOMUX CFG SAIF SYSCFG 136\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`aon_iomux_cfgsaif_syscfg136::R`].  You can [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`aon_iomux_cfgsaif_syscfg136::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`aon_iomux_cfgsaif_syscfg136`]
module"]
pub type AON_IOMUX_CFGSAIF_SYSCFG136 =
    crate::Reg<aon_iomux_cfgsaif_syscfg136::AON_IOMUX_CFGSAIF_SYSCFG136_SPEC>;
#[doc = "AON IOMUX CFG SAIF SYSCFG 136"]
pub mod aon_iomux_cfgsaif_syscfg136;
#[doc = "aon_iomux_cfgsaif_syscfg140 (rw) register accessor: AON IOMUX CFG SAIF SYSCFG 140\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`aon_iomux_cfgsaif_syscfg140::R`].  You can [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`aon_iomux_cfgsaif_syscfg140::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`aon_iomux_cfgsaif_syscfg140`]
module"]
pub type AON_IOMUX_CFGSAIF_SYSCFG140 =
    crate::Reg<aon_iomux_cfgsaif_syscfg140::AON_IOMUX_CFGSAIF_SYSCFG140_SPEC>;
#[doc = "AON IOMUX CFG SAIF SYSCFG 140"]
pub mod aon_iomux_cfgsaif_syscfg140;
#[doc = "aon_iomux_cfgsaif_syscfg144 (rw) register accessor: AON IOMUX CFG SAIF SYSCFG 144\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`aon_iomux_cfgsaif_syscfg144::R`].  You can [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`aon_iomux_cfgsaif_syscfg144::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`aon_iomux_cfgsaif_syscfg144`]
module"]
pub type AON_IOMUX_CFGSAIF_SYSCFG144 =
    crate::Reg<aon_iomux_cfgsaif_syscfg144::AON_IOMUX_CFGSAIF_SYSCFG144_SPEC>;
#[doc = "AON IOMUX CFG SAIF SYSCFG 144"]
pub mod aon_iomux_cfgsaif_syscfg144;
