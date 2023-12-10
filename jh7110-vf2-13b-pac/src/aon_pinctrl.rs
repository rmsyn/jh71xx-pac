#[doc = r"Register block"]
#[repr(C)]
pub struct RegisterBlock {
    fmux_0: FMUX_0,
    fmux_1: FMUX_1,
    fmux_2: FMUX_2,
    fmux_3: FMUX_3,
    ioirq_0: IOIRQ_0,
    ioirq_1: IOIRQ_1,
    ioirq_2: IOIRQ_2,
    ioirq_3: IOIRQ_3,
    ioirq_4: IOIRQ_4,
    ioirq_5: IOIRQ_5,
    ioirq_6: IOIRQ_6,
    ioirq_7: IOIRQ_7,
    testen: TESTEN,
    rgpio_0: RGPIO_0,
    rgpio_1: RGPIO_1,
    rgpio_2: RGPIO_2,
    rgpio_3: RGPIO_3,
    rstn: RSTN,
    _reserved18: [u8; 0x04],
    rtc: RTC,
    _reserved19: [u8; 0x04],
    osc: OSC,
    gmac0_mdc: GMAC0_MDC,
    gmac0_mdio: GMAC0_MDIO,
    gmac0_rxd0: GMAC0_RXD0,
    gmac0_rxd1: GMAC0_RXD1,
    gmac0_rxd2: GMAC0_RXD2,
    gmac0_rxd3: GMAC0_RXD3,
    gmac0_rxdv: GMAC0_RXDV,
    gmac0_rxc: GMAC0_RXC,
    gmac0_txd0: GMAC0_TXD0,
    gmac0_txd1: GMAC0_TXD1,
    gmac0_txd2: GMAC0_TXD2,
    gmac0_txd3: GMAC0_TXD3,
    gmac0_txen: GMAC0_TXEN,
    gmac0_txc: GMAC0_TXC,
    gmac0_rxc_func_sel: GMAC0_RXC_FUNC_SEL,
}
impl RegisterBlock {
    #[doc = "0x00 - AON IOMUX CFG SAIF SYSCFG FMUX 0"]
    #[inline(always)]
    pub const fn fmux_0(&self) -> &FMUX_0 {
        &self.fmux_0
    }
    #[doc = "0x04 - AON IOMUX CFG SAIF SYSCFG FMUX 4"]
    #[inline(always)]
    pub const fn fmux_1(&self) -> &FMUX_1 {
        &self.fmux_1
    }
    #[doc = "0x08 - AON IOMUX CFG SAIF SYSCFG FMUX 8"]
    #[inline(always)]
    pub const fn fmux_2(&self) -> &FMUX_2 {
        &self.fmux_2
    }
    #[doc = "0x0c - AON IOMUX CFG SAIF SYSCFG FMUX 12"]
    #[inline(always)]
    pub const fn fmux_3(&self) -> &FMUX_3 {
        &self.fmux_3
    }
    #[doc = "0x10 - AON IOMUX CFG SAIF SYSCFG IOIRQ 16"]
    #[inline(always)]
    pub const fn ioirq_0(&self) -> &IOIRQ_0 {
        &self.ioirq_0
    }
    #[doc = "0x14 - AON IOMUX CFG SAIF SYSCFG IOIRQ 20"]
    #[inline(always)]
    pub const fn ioirq_1(&self) -> &IOIRQ_1 {
        &self.ioirq_1
    }
    #[doc = "0x18 - AON IOMUX CFG SAIF SYSCFG IOIRQ 24"]
    #[inline(always)]
    pub const fn ioirq_2(&self) -> &IOIRQ_2 {
        &self.ioirq_2
    }
    #[doc = "0x1c - AON IOMUX CFG SAIF SYSCFG IOIRQ 28"]
    #[inline(always)]
    pub const fn ioirq_3(&self) -> &IOIRQ_3 {
        &self.ioirq_3
    }
    #[doc = "0x20 - AON IOMUX CFG SAIF SYSCFG IOIRQ 32"]
    #[inline(always)]
    pub const fn ioirq_4(&self) -> &IOIRQ_4 {
        &self.ioirq_4
    }
    #[doc = "0x24 - AON IOMUX CFG SAIF SYSCFG IOIRQ 36"]
    #[inline(always)]
    pub const fn ioirq_5(&self) -> &IOIRQ_5 {
        &self.ioirq_5
    }
    #[doc = "0x28 - AON IOMUX CFG SAIF SYSCFG IOIRQ 40"]
    #[inline(always)]
    pub const fn ioirq_6(&self) -> &IOIRQ_6 {
        &self.ioirq_6
    }
    #[doc = "0x2c - AON IOMUX CFG SAIF SYSCFG IOIRQ 44"]
    #[inline(always)]
    pub const fn ioirq_7(&self) -> &IOIRQ_7 {
        &self.ioirq_7
    }
    #[doc = "0x30 - AON IOMUX CFG SAIF SYSCFG 48"]
    #[inline(always)]
    pub const fn testen(&self) -> &TESTEN {
        &self.testen
    }
    #[doc = "0x34 - AON IOMUX CFG SAIF SYSCFG 52"]
    #[inline(always)]
    pub const fn rgpio_0(&self) -> &RGPIO_0 {
        &self.rgpio_0
    }
    #[doc = "0x38 - AON IOMUX CFG SAIF SYSCFG 56"]
    #[inline(always)]
    pub const fn rgpio_1(&self) -> &RGPIO_1 {
        &self.rgpio_1
    }
    #[doc = "0x3c - AON IOMUX CFG SAIF SYSCFG 60"]
    #[inline(always)]
    pub const fn rgpio_2(&self) -> &RGPIO_2 {
        &self.rgpio_2
    }
    #[doc = "0x40 - AON IOMUX CFG SAIF SYSCFG 64"]
    #[inline(always)]
    pub const fn rgpio_3(&self) -> &RGPIO_3 {
        &self.rgpio_3
    }
    #[doc = "0x44 - AON IOMUX CFG SAIF SYSCFG 68"]
    #[inline(always)]
    pub const fn rstn(&self) -> &RSTN {
        &self.rstn
    }
    #[doc = "0x4c - AON IOMUX CFG SAIF SYSCFG 76"]
    #[inline(always)]
    pub const fn rtc(&self) -> &RTC {
        &self.rtc
    }
    #[doc = "0x54 - AON IOMUX CFG SAIF SYSCFG 84"]
    #[inline(always)]
    pub const fn osc(&self) -> &OSC {
        &self.osc
    }
    #[doc = "0x58 - AON IOMUX CFG SAIF SYSCFG 88"]
    #[inline(always)]
    pub const fn gmac0_mdc(&self) -> &GMAC0_MDC {
        &self.gmac0_mdc
    }
    #[doc = "0x5c - AON IOMUX CFG SAIF SYSCFG 92"]
    #[inline(always)]
    pub const fn gmac0_mdio(&self) -> &GMAC0_MDIO {
        &self.gmac0_mdio
    }
    #[doc = "0x60 - AON IOMUX CFG SAIF SYSCFG 96"]
    #[inline(always)]
    pub const fn gmac0_rxd0(&self) -> &GMAC0_RXD0 {
        &self.gmac0_rxd0
    }
    #[doc = "0x64 - AON IOMUX CFG SAIF SYSCFG 100"]
    #[inline(always)]
    pub const fn gmac0_rxd1(&self) -> &GMAC0_RXD1 {
        &self.gmac0_rxd1
    }
    #[doc = "0x68 - AON IOMUX CFG SAIF SYSCFG 104"]
    #[inline(always)]
    pub const fn gmac0_rxd2(&self) -> &GMAC0_RXD2 {
        &self.gmac0_rxd2
    }
    #[doc = "0x6c - AON IOMUX CFG SAIF SYSCFG 108"]
    #[inline(always)]
    pub const fn gmac0_rxd3(&self) -> &GMAC0_RXD3 {
        &self.gmac0_rxd3
    }
    #[doc = "0x70 - AON IOMUX CFG SAIF SYSCFG 112"]
    #[inline(always)]
    pub const fn gmac0_rxdv(&self) -> &GMAC0_RXDV {
        &self.gmac0_rxdv
    }
    #[doc = "0x74 - AON IOMUX CFG SAIF SYSCFG 116"]
    #[inline(always)]
    pub const fn gmac0_rxc(&self) -> &GMAC0_RXC {
        &self.gmac0_rxc
    }
    #[doc = "0x78 - AON IOMUX CFG SAIF SYSCFG 120"]
    #[inline(always)]
    pub const fn gmac0_txd0(&self) -> &GMAC0_TXD0 {
        &self.gmac0_txd0
    }
    #[doc = "0x7c - AON IOMUX CFG SAIF SYSCFG 124"]
    #[inline(always)]
    pub const fn gmac0_txd1(&self) -> &GMAC0_TXD1 {
        &self.gmac0_txd1
    }
    #[doc = "0x80 - AON IOMUX CFG SAIF SYSCFG 128"]
    #[inline(always)]
    pub const fn gmac0_txd2(&self) -> &GMAC0_TXD2 {
        &self.gmac0_txd2
    }
    #[doc = "0x84 - AON IOMUX CFG SAIF SYSCFG 132"]
    #[inline(always)]
    pub const fn gmac0_txd3(&self) -> &GMAC0_TXD3 {
        &self.gmac0_txd3
    }
    #[doc = "0x88 - AON IOMUX CFG SAIF SYSCFG 136"]
    #[inline(always)]
    pub const fn gmac0_txen(&self) -> &GMAC0_TXEN {
        &self.gmac0_txen
    }
    #[doc = "0x8c - AON IOMUX CFG SAIF SYSCFG 140"]
    #[inline(always)]
    pub const fn gmac0_txc(&self) -> &GMAC0_TXC {
        &self.gmac0_txc
    }
    #[doc = "0x90 - AON IOMUX CFG SAIF SYSCFG 144"]
    #[inline(always)]
    pub const fn gmac0_rxc_func_sel(&self) -> &GMAC0_RXC_FUNC_SEL {
        &self.gmac0_rxc_func_sel
    }
}
#[doc = "fmux_0 (rw) register accessor: AON IOMUX CFG SAIF SYSCFG FMUX 0\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`fmux_0::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`fmux_0::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@fmux_0`]
module"]
pub type FMUX_0 = crate::Reg<fmux_0::FMUX_0_SPEC>;
#[doc = "AON IOMUX CFG SAIF SYSCFG FMUX 0"]
pub mod fmux_0;
#[doc = "fmux_1 (rw) register accessor: AON IOMUX CFG SAIF SYSCFG FMUX 4\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`fmux_1::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`fmux_1::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@fmux_1`]
module"]
pub type FMUX_1 = crate::Reg<fmux_1::FMUX_1_SPEC>;
#[doc = "AON IOMUX CFG SAIF SYSCFG FMUX 4"]
pub mod fmux_1;
#[doc = "fmux_2 (rw) register accessor: AON IOMUX CFG SAIF SYSCFG FMUX 8\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`fmux_2::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`fmux_2::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@fmux_2`]
module"]
pub type FMUX_2 = crate::Reg<fmux_2::FMUX_2_SPEC>;
#[doc = "AON IOMUX CFG SAIF SYSCFG FMUX 8"]
pub mod fmux_2;
#[doc = "fmux_3 (rw) register accessor: AON IOMUX CFG SAIF SYSCFG FMUX 12\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`fmux_3::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`fmux_3::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@fmux_3`]
module"]
pub type FMUX_3 = crate::Reg<fmux_3::FMUX_3_SPEC>;
#[doc = "AON IOMUX CFG SAIF SYSCFG FMUX 12"]
pub mod fmux_3;
#[doc = "ioirq_0 (rw) register accessor: AON IOMUX CFG SAIF SYSCFG IOIRQ 16\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`ioirq_0::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`ioirq_0::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@ioirq_0`]
module"]
pub type IOIRQ_0 = crate::Reg<ioirq_0::IOIRQ_0_SPEC>;
#[doc = "AON IOMUX CFG SAIF SYSCFG IOIRQ 16"]
pub mod ioirq_0;
#[doc = "ioirq_1 (rw) register accessor: AON IOMUX CFG SAIF SYSCFG IOIRQ 20\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`ioirq_1::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`ioirq_1::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@ioirq_1`]
module"]
pub type IOIRQ_1 = crate::Reg<ioirq_1::IOIRQ_1_SPEC>;
#[doc = "AON IOMUX CFG SAIF SYSCFG IOIRQ 20"]
pub mod ioirq_1;
#[doc = "ioirq_2 (rw) register accessor: AON IOMUX CFG SAIF SYSCFG IOIRQ 24\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`ioirq_2::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`ioirq_2::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@ioirq_2`]
module"]
pub type IOIRQ_2 = crate::Reg<ioirq_2::IOIRQ_2_SPEC>;
#[doc = "AON IOMUX CFG SAIF SYSCFG IOIRQ 24"]
pub mod ioirq_2;
#[doc = "ioirq_3 (rw) register accessor: AON IOMUX CFG SAIF SYSCFG IOIRQ 28\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`ioirq_3::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`ioirq_3::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@ioirq_3`]
module"]
pub type IOIRQ_3 = crate::Reg<ioirq_3::IOIRQ_3_SPEC>;
#[doc = "AON IOMUX CFG SAIF SYSCFG IOIRQ 28"]
pub mod ioirq_3;
#[doc = "ioirq_4 (rw) register accessor: AON IOMUX CFG SAIF SYSCFG IOIRQ 32\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`ioirq_4::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`ioirq_4::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@ioirq_4`]
module"]
pub type IOIRQ_4 = crate::Reg<ioirq_4::IOIRQ_4_SPEC>;
#[doc = "AON IOMUX CFG SAIF SYSCFG IOIRQ 32"]
pub mod ioirq_4;
#[doc = "ioirq_5 (rw) register accessor: AON IOMUX CFG SAIF SYSCFG IOIRQ 36\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`ioirq_5::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`ioirq_5::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@ioirq_5`]
module"]
pub type IOIRQ_5 = crate::Reg<ioirq_5::IOIRQ_5_SPEC>;
#[doc = "AON IOMUX CFG SAIF SYSCFG IOIRQ 36"]
pub mod ioirq_5;
#[doc = "ioirq_6 (rw) register accessor: AON IOMUX CFG SAIF SYSCFG IOIRQ 40\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`ioirq_6::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`ioirq_6::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@ioirq_6`]
module"]
pub type IOIRQ_6 = crate::Reg<ioirq_6::IOIRQ_6_SPEC>;
#[doc = "AON IOMUX CFG SAIF SYSCFG IOIRQ 40"]
pub mod ioirq_6;
#[doc = "ioirq_7 (rw) register accessor: AON IOMUX CFG SAIF SYSCFG IOIRQ 44\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`ioirq_7::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`ioirq_7::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@ioirq_7`]
module"]
pub type IOIRQ_7 = crate::Reg<ioirq_7::IOIRQ_7_SPEC>;
#[doc = "AON IOMUX CFG SAIF SYSCFG IOIRQ 44"]
pub mod ioirq_7;
#[doc = "testen (rw) register accessor: AON IOMUX CFG SAIF SYSCFG 48\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`testen::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`testen::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@testen`]
module"]
pub type TESTEN = crate::Reg<testen::TESTEN_SPEC>;
#[doc = "AON IOMUX CFG SAIF SYSCFG 48"]
pub mod testen;
#[doc = "rgpio_0 (rw) register accessor: AON IOMUX CFG SAIF SYSCFG 52\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`rgpio_0::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`rgpio_0::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@rgpio_0`]
module"]
pub type RGPIO_0 = crate::Reg<rgpio_0::RGPIO_0_SPEC>;
#[doc = "AON IOMUX CFG SAIF SYSCFG 52"]
pub mod rgpio_0;
#[doc = "rgpio_1 (rw) register accessor: AON IOMUX CFG SAIF SYSCFG 56\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`rgpio_1::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`rgpio_1::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@rgpio_1`]
module"]
pub type RGPIO_1 = crate::Reg<rgpio_1::RGPIO_1_SPEC>;
#[doc = "AON IOMUX CFG SAIF SYSCFG 56"]
pub mod rgpio_1;
#[doc = "rgpio_2 (rw) register accessor: AON IOMUX CFG SAIF SYSCFG 60\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`rgpio_2::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`rgpio_2::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@rgpio_2`]
module"]
pub type RGPIO_2 = crate::Reg<rgpio_2::RGPIO_2_SPEC>;
#[doc = "AON IOMUX CFG SAIF SYSCFG 60"]
pub mod rgpio_2;
#[doc = "rgpio_3 (rw) register accessor: AON IOMUX CFG SAIF SYSCFG 64\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`rgpio_3::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`rgpio_3::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@rgpio_3`]
module"]
pub type RGPIO_3 = crate::Reg<rgpio_3::RGPIO_3_SPEC>;
#[doc = "AON IOMUX CFG SAIF SYSCFG 64"]
pub mod rgpio_3;
#[doc = "rstn (rw) register accessor: AON IOMUX CFG SAIF SYSCFG 68\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`rstn::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`rstn::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@rstn`]
module"]
pub type RSTN = crate::Reg<rstn::RSTN_SPEC>;
#[doc = "AON IOMUX CFG SAIF SYSCFG 68"]
pub mod rstn;
#[doc = "rtc (rw) register accessor: AON IOMUX CFG SAIF SYSCFG 76\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`rtc::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`rtc::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@rtc`]
module"]
pub type RTC = crate::Reg<rtc::RTC_SPEC>;
#[doc = "AON IOMUX CFG SAIF SYSCFG 76"]
pub mod rtc;
#[doc = "osc (rw) register accessor: AON IOMUX CFG SAIF SYSCFG 84\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`osc::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`osc::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@osc`]
module"]
pub type OSC = crate::Reg<osc::OSC_SPEC>;
#[doc = "AON IOMUX CFG SAIF SYSCFG 84"]
pub mod osc;
#[doc = "gmac0_mdc (rw) register accessor: AON IOMUX CFG SAIF SYSCFG 88\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`gmac0_mdc::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`gmac0_mdc::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@gmac0_mdc`]
module"]
pub type GMAC0_MDC = crate::Reg<gmac0_mdc::GMAC0_MDC_SPEC>;
#[doc = "AON IOMUX CFG SAIF SYSCFG 88"]
pub mod gmac0_mdc;
#[doc = "gmac0_mdio (rw) register accessor: AON IOMUX CFG SAIF SYSCFG 92\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`gmac0_mdio::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`gmac0_mdio::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@gmac0_mdio`]
module"]
pub type GMAC0_MDIO = crate::Reg<gmac0_mdio::GMAC0_MDIO_SPEC>;
#[doc = "AON IOMUX CFG SAIF SYSCFG 92"]
pub mod gmac0_mdio;
#[doc = "gmac0_rxd0 (rw) register accessor: AON IOMUX CFG SAIF SYSCFG 96\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`gmac0_rxd0::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`gmac0_rxd0::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@gmac0_rxd0`]
module"]
pub type GMAC0_RXD0 = crate::Reg<gmac0_rxd0::GMAC0_RXD0_SPEC>;
#[doc = "AON IOMUX CFG SAIF SYSCFG 96"]
pub mod gmac0_rxd0;
#[doc = "gmac0_rxd1 (rw) register accessor: AON IOMUX CFG SAIF SYSCFG 100\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`gmac0_rxd1::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`gmac0_rxd1::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@gmac0_rxd1`]
module"]
pub type GMAC0_RXD1 = crate::Reg<gmac0_rxd1::GMAC0_RXD1_SPEC>;
#[doc = "AON IOMUX CFG SAIF SYSCFG 100"]
pub mod gmac0_rxd1;
#[doc = "gmac0_rxd2 (rw) register accessor: AON IOMUX CFG SAIF SYSCFG 104\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`gmac0_rxd2::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`gmac0_rxd2::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@gmac0_rxd2`]
module"]
pub type GMAC0_RXD2 = crate::Reg<gmac0_rxd2::GMAC0_RXD2_SPEC>;
#[doc = "AON IOMUX CFG SAIF SYSCFG 104"]
pub mod gmac0_rxd2;
#[doc = "gmac0_rxd3 (rw) register accessor: AON IOMUX CFG SAIF SYSCFG 108\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`gmac0_rxd3::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`gmac0_rxd3::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@gmac0_rxd3`]
module"]
pub type GMAC0_RXD3 = crate::Reg<gmac0_rxd3::GMAC0_RXD3_SPEC>;
#[doc = "AON IOMUX CFG SAIF SYSCFG 108"]
pub mod gmac0_rxd3;
#[doc = "gmac0_rxdv (rw) register accessor: AON IOMUX CFG SAIF SYSCFG 112\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`gmac0_rxdv::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`gmac0_rxdv::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@gmac0_rxdv`]
module"]
pub type GMAC0_RXDV = crate::Reg<gmac0_rxdv::GMAC0_RXDV_SPEC>;
#[doc = "AON IOMUX CFG SAIF SYSCFG 112"]
pub mod gmac0_rxdv;
#[doc = "gmac0_rxc (rw) register accessor: AON IOMUX CFG SAIF SYSCFG 116\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`gmac0_rxc::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`gmac0_rxc::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@gmac0_rxc`]
module"]
pub type GMAC0_RXC = crate::Reg<gmac0_rxc::GMAC0_RXC_SPEC>;
#[doc = "AON IOMUX CFG SAIF SYSCFG 116"]
pub mod gmac0_rxc;
#[doc = "gmac0_txd0 (rw) register accessor: AON IOMUX CFG SAIF SYSCFG 120\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`gmac0_txd0::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`gmac0_txd0::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@gmac0_txd0`]
module"]
pub type GMAC0_TXD0 = crate::Reg<gmac0_txd0::GMAC0_TXD0_SPEC>;
#[doc = "AON IOMUX CFG SAIF SYSCFG 120"]
pub mod gmac0_txd0;
#[doc = "gmac0_txd1 (rw) register accessor: AON IOMUX CFG SAIF SYSCFG 124\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`gmac0_txd1::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`gmac0_txd1::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@gmac0_txd1`]
module"]
pub type GMAC0_TXD1 = crate::Reg<gmac0_txd1::GMAC0_TXD1_SPEC>;
#[doc = "AON IOMUX CFG SAIF SYSCFG 124"]
pub mod gmac0_txd1;
#[doc = "gmac0_txd2 (rw) register accessor: AON IOMUX CFG SAIF SYSCFG 128\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`gmac0_txd2::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`gmac0_txd2::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@gmac0_txd2`]
module"]
pub type GMAC0_TXD2 = crate::Reg<gmac0_txd2::GMAC0_TXD2_SPEC>;
#[doc = "AON IOMUX CFG SAIF SYSCFG 128"]
pub mod gmac0_txd2;
#[doc = "gmac0_txd3 (rw) register accessor: AON IOMUX CFG SAIF SYSCFG 132\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`gmac0_txd3::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`gmac0_txd3::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@gmac0_txd3`]
module"]
pub type GMAC0_TXD3 = crate::Reg<gmac0_txd3::GMAC0_TXD3_SPEC>;
#[doc = "AON IOMUX CFG SAIF SYSCFG 132"]
pub mod gmac0_txd3;
#[doc = "gmac0_txen (rw) register accessor: AON IOMUX CFG SAIF SYSCFG 136\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`gmac0_txen::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`gmac0_txen::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@gmac0_txen`]
module"]
pub type GMAC0_TXEN = crate::Reg<gmac0_txen::GMAC0_TXEN_SPEC>;
#[doc = "AON IOMUX CFG SAIF SYSCFG 136"]
pub mod gmac0_txen;
#[doc = "gmac0_txc (rw) register accessor: AON IOMUX CFG SAIF SYSCFG 140\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`gmac0_txc::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`gmac0_txc::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@gmac0_txc`]
module"]
pub type GMAC0_TXC = crate::Reg<gmac0_txc::GMAC0_TXC_SPEC>;
#[doc = "AON IOMUX CFG SAIF SYSCFG 140"]
pub mod gmac0_txc;
#[doc = "gmac0_rxc_func_sel (rw) register accessor: AON IOMUX CFG SAIF SYSCFG 144\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`gmac0_rxc_func_sel::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`gmac0_rxc_func_sel::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@gmac0_rxc_func_sel`]
module"]
pub type GMAC0_RXC_FUNC_SEL = crate::Reg<gmac0_rxc_func_sel::GMAC0_RXC_FUNC_SEL_SPEC>;
#[doc = "AON IOMUX CFG SAIF SYSCFG 144"]
pub mod gmac0_rxc_func_sel;
