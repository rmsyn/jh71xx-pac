#[doc = r"Register block"]
#[repr(C)]
pub struct RegisterBlock {
    clk_osc: CLK_OSC,
    clk_aon_apb: CLK_AON_APB,
    clk_ahb_gmac5: CLK_AHB_GMAC5,
    clk_axi_gmac5: CLK_AXI_GMAC5,
    clk_gmac0_rmii_rtx: CLK_GMAC0_RMII_RTX,
    clk_gmac5_axi64_tx: CLK_GMAC5_AXI64_TX,
    clk_gmac5_axi64_txi: CLK_GMAC5_AXI64_TXI,
    clk_gmac5_axi64_rx: CLK_GMAC5_AXI64_RX,
    clk_gmac5_axi64_rxi: CLK_GMAC5_AXI64_RXI,
    clk_optc_apb: CLK_OPTC_APB,
    clk_rtc_hms_apb: CLK_RTC_HMS_APB,
    clk_rtc_internal: CLK_RTC_INTERNAL,
    clk_rtc_hms_osc32k: CLK_RTC_HMS_OSC32K,
    clk_rtc_hms_cal: CLK_RTC_HMS_CAL,
    soft_rst_addr_sel: SOFT_RST_ADDR_SEL,
    aoncrg_rst_status: AONCRG_RST_STATUS,
}
impl RegisterBlock {
    #[doc = "0x00 - Oscillator Clock"]
    #[inline(always)]
    pub const fn clk_osc(&self) -> &CLK_OSC {
        &self.clk_osc
    }
    #[doc = "0x04 - AON APB Function Clock"]
    #[inline(always)]
    pub const fn clk_aon_apb(&self) -> &CLK_AON_APB {
        &self.clk_aon_apb
    }
    #[doc = "0x08 - AHB GMAC5 Clock"]
    #[inline(always)]
    pub const fn clk_ahb_gmac5(&self) -> &CLK_AHB_GMAC5 {
        &self.clk_ahb_gmac5
    }
    #[doc = "0x0c - AXI GMAC5 Clock"]
    #[inline(always)]
    pub const fn clk_axi_gmac5(&self) -> &CLK_AXI_GMAC5 {
        &self.clk_axi_gmac5
    }
    #[doc = "0x10 - GMAC0 RMII RTX Clock"]
    #[inline(always)]
    pub const fn clk_gmac0_rmii_rtx(&self) -> &CLK_GMAC0_RMII_RTX {
        &self.clk_gmac0_rmii_rtx
    }
    #[doc = "0x14 - GMAC5 AXI64 Clock Transmitter"]
    #[inline(always)]
    pub const fn clk_gmac5_axi64_tx(&self) -> &CLK_GMAC5_AXI64_TX {
        &self.clk_gmac5_axi64_tx
    }
    #[doc = "0x18 - GMAC5 AXI64 Clock Transmission Inverter"]
    #[inline(always)]
    pub const fn clk_gmac5_axi64_txi(&self) -> &CLK_GMAC5_AXI64_TXI {
        &self.clk_gmac5_axi64_txi
    }
    #[doc = "0x1c - GMAC5 AXI64 Clock Receiver"]
    #[inline(always)]
    pub const fn clk_gmac5_axi64_rx(&self) -> &CLK_GMAC5_AXI64_RX {
        &self.clk_gmac5_axi64_rx
    }
    #[doc = "0x20 - GMAC5 AXI64 Clock Receiving Inverter"]
    #[inline(always)]
    pub const fn clk_gmac5_axi64_rxi(&self) -> &CLK_GMAC5_AXI64_RXI {
        &self.clk_gmac5_axi64_rxi
    }
    #[doc = "0x24 - OPTC APB Clock"]
    #[inline(always)]
    pub const fn clk_optc_apb(&self) -> &CLK_OPTC_APB {
        &self.clk_optc_apb
    }
    #[doc = "0x28 - RTC HMS APB Clock"]
    #[inline(always)]
    pub const fn clk_rtc_hms_apb(&self) -> &CLK_RTC_HMS_APB {
        &self.clk_rtc_hms_apb
    }
    #[doc = "0x2c - RTC Internal Clock"]
    #[inline(always)]
    pub const fn clk_rtc_internal(&self) -> &CLK_RTC_INTERNAL {
        &self.clk_rtc_internal
    }
    #[doc = "0x30 - RTC HMS Clock Oscillator 32K"]
    #[inline(always)]
    pub const fn clk_rtc_hms_osc32k(&self) -> &CLK_RTC_HMS_OSC32K {
        &self.clk_rtc_hms_osc32k
    }
    #[doc = "0x34 - RTC HMS Clock Calculator"]
    #[inline(always)]
    pub const fn clk_rtc_hms_cal(&self) -> &CLK_RTC_HMS_CAL {
        &self.clk_rtc_hms_cal
    }
    #[doc = "0x38 - Software RESET Address Selector"]
    #[inline(always)]
    pub const fn soft_rst_addr_sel(&self) -> &SOFT_RST_ADDR_SEL {
        &self.soft_rst_addr_sel
    }
    #[doc = "0x3c - AONCRG RESET Status"]
    #[inline(always)]
    pub const fn aoncrg_rst_status(&self) -> &AONCRG_RST_STATUS {
        &self.aoncrg_rst_status
    }
}
#[doc = "clk_osc (rw) register accessor: Oscillator Clock\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`clk_osc::R`].  You can [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`clk_osc::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@clk_osc`]
module"]
pub type CLK_OSC = crate::Reg<clk_osc::CLK_OSC_SPEC>;
#[doc = "Oscillator Clock"]
pub mod clk_osc;
#[doc = "clk_aon_apb (rw) register accessor: AON APB Function Clock\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`clk_aon_apb::R`].  You can [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`clk_aon_apb::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@clk_aon_apb`]
module"]
pub type CLK_AON_APB = crate::Reg<clk_aon_apb::CLK_AON_APB_SPEC>;
#[doc = "AON APB Function Clock"]
pub mod clk_aon_apb;
#[doc = "clk_ahb_gmac5 (rw) register accessor: AHB GMAC5 Clock\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`clk_ahb_gmac5::R`].  You can [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`clk_ahb_gmac5::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@clk_ahb_gmac5`]
module"]
pub type CLK_AHB_GMAC5 = crate::Reg<clk_ahb_gmac5::CLK_AHB_GMAC5_SPEC>;
#[doc = "AHB GMAC5 Clock"]
pub mod clk_ahb_gmac5;
#[doc = "clk_axi_gmac5 (rw) register accessor: AXI GMAC5 Clock\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`clk_axi_gmac5::R`].  You can [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`clk_axi_gmac5::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@clk_axi_gmac5`]
module"]
pub type CLK_AXI_GMAC5 = crate::Reg<clk_axi_gmac5::CLK_AXI_GMAC5_SPEC>;
#[doc = "AXI GMAC5 Clock"]
pub mod clk_axi_gmac5;
#[doc = "clk_gmac0_rmii_rtx (rw) register accessor: GMAC0 RMII RTX Clock\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`clk_gmac0_rmii_rtx::R`].  You can [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`clk_gmac0_rmii_rtx::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@clk_gmac0_rmii_rtx`]
module"]
pub type CLK_GMAC0_RMII_RTX = crate::Reg<clk_gmac0_rmii_rtx::CLK_GMAC0_RMII_RTX_SPEC>;
#[doc = "GMAC0 RMII RTX Clock"]
pub mod clk_gmac0_rmii_rtx;
#[doc = "clk_gmac5_axi64_tx (rw) register accessor: GMAC5 AXI64 Clock Transmitter\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`clk_gmac5_axi64_tx::R`].  You can [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`clk_gmac5_axi64_tx::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@clk_gmac5_axi64_tx`]
module"]
pub type CLK_GMAC5_AXI64_TX = crate::Reg<clk_gmac5_axi64_tx::CLK_GMAC5_AXI64_TX_SPEC>;
#[doc = "GMAC5 AXI64 Clock Transmitter"]
pub mod clk_gmac5_axi64_tx;
#[doc = "clk_gmac5_axi64_txi (rw) register accessor: GMAC5 AXI64 Clock Transmission Inverter\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`clk_gmac5_axi64_txi::R`].  You can [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`clk_gmac5_axi64_txi::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@clk_gmac5_axi64_txi`]
module"]
pub type CLK_GMAC5_AXI64_TXI = crate::Reg<clk_gmac5_axi64_txi::CLK_GMAC5_AXI64_TXI_SPEC>;
#[doc = "GMAC5 AXI64 Clock Transmission Inverter"]
pub mod clk_gmac5_axi64_txi;
#[doc = "clk_gmac5_axi64_rx (rw) register accessor: GMAC5 AXI64 Clock Receiver\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`clk_gmac5_axi64_rx::R`].  You can [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`clk_gmac5_axi64_rx::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@clk_gmac5_axi64_rx`]
module"]
pub type CLK_GMAC5_AXI64_RX = crate::Reg<clk_gmac5_axi64_rx::CLK_GMAC5_AXI64_RX_SPEC>;
#[doc = "GMAC5 AXI64 Clock Receiver"]
pub mod clk_gmac5_axi64_rx;
#[doc = "clk_gmac5_axi64_rxi (rw) register accessor: GMAC5 AXI64 Clock Receiving Inverter\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`clk_gmac5_axi64_rxi::R`].  You can [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`clk_gmac5_axi64_rxi::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@clk_gmac5_axi64_rxi`]
module"]
pub type CLK_GMAC5_AXI64_RXI = crate::Reg<clk_gmac5_axi64_rxi::CLK_GMAC5_AXI64_RXI_SPEC>;
#[doc = "GMAC5 AXI64 Clock Receiving Inverter"]
pub mod clk_gmac5_axi64_rxi;
#[doc = "clk_optc_apb (rw) register accessor: OPTC APB Clock\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`clk_optc_apb::R`].  You can [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`clk_optc_apb::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@clk_optc_apb`]
module"]
pub type CLK_OPTC_APB = crate::Reg<clk_optc_apb::CLK_OPTC_APB_SPEC>;
#[doc = "OPTC APB Clock"]
pub mod clk_optc_apb;
#[doc = "clk_rtc_hms_apb (rw) register accessor: RTC HMS APB Clock\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`clk_rtc_hms_apb::R`].  You can [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`clk_rtc_hms_apb::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@clk_rtc_hms_apb`]
module"]
pub type CLK_RTC_HMS_APB = crate::Reg<clk_rtc_hms_apb::CLK_RTC_HMS_APB_SPEC>;
#[doc = "RTC HMS APB Clock"]
pub mod clk_rtc_hms_apb;
#[doc = "clk_rtc_internal (rw) register accessor: RTC Internal Clock\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`clk_rtc_internal::R`].  You can [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`clk_rtc_internal::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@clk_rtc_internal`]
module"]
pub type CLK_RTC_INTERNAL = crate::Reg<clk_rtc_internal::CLK_RTC_INTERNAL_SPEC>;
#[doc = "RTC Internal Clock"]
pub mod clk_rtc_internal;
#[doc = "clk_rtc_hms_osc32k (rw) register accessor: RTC HMS Clock Oscillator 32K\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`clk_rtc_hms_osc32k::R`].  You can [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`clk_rtc_hms_osc32k::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@clk_rtc_hms_osc32k`]
module"]
pub type CLK_RTC_HMS_OSC32K = crate::Reg<clk_rtc_hms_osc32k::CLK_RTC_HMS_OSC32K_SPEC>;
#[doc = "RTC HMS Clock Oscillator 32K"]
pub mod clk_rtc_hms_osc32k;
#[doc = "clk_rtc_hms_cal (rw) register accessor: RTC HMS Clock Calculator\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`clk_rtc_hms_cal::R`].  You can [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`clk_rtc_hms_cal::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@clk_rtc_hms_cal`]
module"]
pub type CLK_RTC_HMS_CAL = crate::Reg<clk_rtc_hms_cal::CLK_RTC_HMS_CAL_SPEC>;
#[doc = "RTC HMS Clock Calculator"]
pub mod clk_rtc_hms_cal;
#[doc = "soft_rst_addr_sel (rw) register accessor: Software RESET Address Selector\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`soft_rst_addr_sel::R`].  You can [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`soft_rst_addr_sel::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@soft_rst_addr_sel`]
module"]
pub type SOFT_RST_ADDR_SEL = crate::Reg<soft_rst_addr_sel::SOFT_RST_ADDR_SEL_SPEC>;
#[doc = "Software RESET Address Selector"]
pub mod soft_rst_addr_sel;
#[doc = "aoncrg_rst_status (rw) register accessor: AONCRG RESET Status\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`aoncrg_rst_status::R`].  You can [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`aoncrg_rst_status::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@aoncrg_rst_status`]
module"]
pub type AONCRG_RST_STATUS = crate::Reg<aoncrg_rst_status::AONCRG_RST_STATUS_SPEC>;
#[doc = "AONCRG RESET Status"]
pub mod aoncrg_rst_status;
