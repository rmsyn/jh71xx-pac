#[doc = r"Register block"]
#[repr(C)]
pub struct RegisterBlock {
    #[doc = "0x00 - Clock HIFI4 Core"]
    pub clk_hifi4_core: CLK_HIFI4_CORE,
    #[doc = "0x04 - Clock USB APB"]
    pub clk_usb_apb: CLK_USB_APB,
    #[doc = "0x08 - Clock USB UTMI APB"]
    pub clk_usb_utmi_apb: CLK_USB_UTMI_APB,
    #[doc = "0x0c - Clock USB AXI"]
    pub clk_usb_axi: CLK_USB_AXI,
    #[doc = "0x10 - Clock USB AXI"]
    pub clk_usb_ipm: CLK_USB_IPM,
    #[doc = "0x14 - Clock USB STB"]
    pub clk_usb_stb: CLK_USB_STB,
    #[doc = "0x18 - Clock USB APP 125"]
    pub clk_usb_app125: CLK_USB_APP125,
    #[doc = "0x1c - Clock USB Reference Clock"]
    pub clk_usb_refclk: CLK_USB_REFCLK,
    #[doc = "0x20 - U0 Clock PCIe AXI MST 0"]
    pub clk_u0_pcie_axi_mst0: CLK_U0_PCIE_AXI_MST0,
    #[doc = "0x24 - U0 Clock PCIe APB"]
    pub clk_u0_pcie_apb: CLK_U0_PCIE_APB,
    #[doc = "0x28 - U0 Clock PCIe TL"]
    pub clk_u0_pcie_tl: CLK_U0_PCIE_TL,
    #[doc = "0x2c - U1 Clock PCIe AXI MST 0"]
    pub clk_u1_pcie_axi_mst0: CLK_U1_PCIE_AXI_MST0,
    #[doc = "0x30 - U1 Clock PCIe APB"]
    pub clk_u1_pcie_apb: CLK_U1_PCIE_APB,
    #[doc = "0x34 - U1 Clock PCIe TL"]
    pub clk_u1_pcie_tl: CLK_U1_PCIE_TL,
    #[doc = "0x38 - Clock PCIe 01 SLV DEC Main"]
    pub clk_pcie01_slv_dec_main: CLK_PCIE01_SLV_DEC_MAIN,
    #[doc = "0x3c - Clock Security HCLK"]
    pub clk_sec_hclk: CLK_SEC_HCLK,
    #[doc = "0x40 - Clock Security Miscellaneous AHB"]
    pub clk_sec_misc_ahb: CLK_SEC_MISC_AHB,
    #[doc = "0x44 - Clock STG MTRX Group 0 Main"]
    pub clk_stg_mtrx_group0_main: CLK_STG_MTRX_GROUP0_MAIN,
    #[doc = "0x48 - Clock STG MTRX Group 0 Bus"]
    pub clk_stg_mtrx_group0_bus: CLK_STG_MTRX_GROUP0_BUS,
    #[doc = "0x4c - Clock STG MTRX Group 0 STG"]
    pub clk_stg_mtrx_group0_stg: CLK_STG_MTRX_GROUP0_STG,
    #[doc = "0x50 - Clock STG MTRX Group 1 Main"]
    pub clk_stg_mtrx_group1_main: CLK_STG_MTRX_GROUP1_MAIN,
    #[doc = "0x54 - Clock STG MTRX Group 1 Bus"]
    pub clk_stg_mtrx_group1_bus: CLK_STG_MTRX_GROUP1_BUS,
    #[doc = "0x58 - Clock STG MTRX Group 1 STG"]
    pub clk_stg_mtrx_group1_stg: CLK_STG_MTRX_GROUP1_STG,
    #[doc = "0x5c - Clock STG MTRX Group 1 HIFI"]
    pub clk_stg_mtrx_group1_hifi: CLK_STG_MTRX_GROUP1_HIFI,
    #[doc = "0x60 - Clock E2 RTC"]
    pub clk_e2_rtc: CLK_E2_RTC,
    #[doc = "0x64 - Clock E2 Core"]
    pub clk_e2_core: CLK_E2_CORE,
    #[doc = "0x68 - Clock E2 DBG"]
    pub clk_e2_dbg: CLK_E2_DBG,
    #[doc = "0x6c - Clock DMA AXI"]
    pub clk_dma_axi: CLK_DMA_AXI,
    #[doc = "0x70 - Clock DMA AHB"]
    pub clk_dma_ahb: CLK_DMA_AHB,
    #[doc = "0x74 - Software RESET Address Selector"]
    pub soft_rst_addr_sel: SOFT_RST_ADDR_SEL,
    #[doc = "0x78 - STGCRG RESET Status"]
    pub stgcrg_rst_stat: STGCRG_RST_STAT,
}
#[doc = "clk_hifi4_core (rw) register accessor: Clock HIFI4 Core\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`clk_hifi4_core::R`].  You can [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`clk_hifi4_core::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`clk_hifi4_core`]
module"]
pub type CLK_HIFI4_CORE = crate::Reg<clk_hifi4_core::CLK_HIFI4_CORE_SPEC>;
#[doc = "Clock HIFI4 Core"]
pub mod clk_hifi4_core;
#[doc = "clk_usb_apb (rw) register accessor: Clock USB APB\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`clk_usb_apb::R`].  You can [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`clk_usb_apb::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`clk_usb_apb`]
module"]
pub type CLK_USB_APB = crate::Reg<clk_usb_apb::CLK_USB_APB_SPEC>;
#[doc = "Clock USB APB"]
pub mod clk_usb_apb;
#[doc = "clk_usb_utmi_apb (rw) register accessor: Clock USB UTMI APB\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`clk_usb_utmi_apb::R`].  You can [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`clk_usb_utmi_apb::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`clk_usb_utmi_apb`]
module"]
pub type CLK_USB_UTMI_APB = crate::Reg<clk_usb_utmi_apb::CLK_USB_UTMI_APB_SPEC>;
#[doc = "Clock USB UTMI APB"]
pub mod clk_usb_utmi_apb;
#[doc = "clk_usb_axi (rw) register accessor: Clock USB AXI\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`clk_usb_axi::R`].  You can [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`clk_usb_axi::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`clk_usb_axi`]
module"]
pub type CLK_USB_AXI = crate::Reg<clk_usb_axi::CLK_USB_AXI_SPEC>;
#[doc = "Clock USB AXI"]
pub mod clk_usb_axi;
#[doc = "clk_usb_ipm (rw) register accessor: Clock USB AXI\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`clk_usb_ipm::R`].  You can [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`clk_usb_ipm::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`clk_usb_ipm`]
module"]
pub type CLK_USB_IPM = crate::Reg<clk_usb_ipm::CLK_USB_IPM_SPEC>;
#[doc = "Clock USB AXI"]
pub mod clk_usb_ipm;
#[doc = "clk_usb_stb (rw) register accessor: Clock USB STB\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`clk_usb_stb::R`].  You can [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`clk_usb_stb::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`clk_usb_stb`]
module"]
pub type CLK_USB_STB = crate::Reg<clk_usb_stb::CLK_USB_STB_SPEC>;
#[doc = "Clock USB STB"]
pub mod clk_usb_stb;
#[doc = "clk_usb_app125 (rw) register accessor: Clock USB APP 125\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`clk_usb_app125::R`].  You can [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`clk_usb_app125::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`clk_usb_app125`]
module"]
pub type CLK_USB_APP125 = crate::Reg<clk_usb_app125::CLK_USB_APP125_SPEC>;
#[doc = "Clock USB APP 125"]
pub mod clk_usb_app125;
#[doc = "clk_usb_refclk (rw) register accessor: Clock USB Reference Clock\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`clk_usb_refclk::R`].  You can [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`clk_usb_refclk::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`clk_usb_refclk`]
module"]
pub type CLK_USB_REFCLK = crate::Reg<clk_usb_refclk::CLK_USB_REFCLK_SPEC>;
#[doc = "Clock USB Reference Clock"]
pub mod clk_usb_refclk;
#[doc = "clk_u0_pcie_axi_mst0 (rw) register accessor: U0 Clock PCIe AXI MST 0\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`clk_u0_pcie_axi_mst0::R`].  You can [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`clk_u0_pcie_axi_mst0::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`clk_u0_pcie_axi_mst0`]
module"]
pub type CLK_U0_PCIE_AXI_MST0 = crate::Reg<clk_u0_pcie_axi_mst0::CLK_U0_PCIE_AXI_MST0_SPEC>;
#[doc = "U0 Clock PCIe AXI MST 0"]
pub mod clk_u0_pcie_axi_mst0;
#[doc = "clk_u0_pcie_apb (rw) register accessor: U0 Clock PCIe APB\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`clk_u0_pcie_apb::R`].  You can [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`clk_u0_pcie_apb::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`clk_u0_pcie_apb`]
module"]
pub type CLK_U0_PCIE_APB = crate::Reg<clk_u0_pcie_apb::CLK_U0_PCIE_APB_SPEC>;
#[doc = "U0 Clock PCIe APB"]
pub mod clk_u0_pcie_apb;
#[doc = "clk_u0_pcie_tl (rw) register accessor: U0 Clock PCIe TL\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`clk_u0_pcie_tl::R`].  You can [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`clk_u0_pcie_tl::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`clk_u0_pcie_tl`]
module"]
pub type CLK_U0_PCIE_TL = crate::Reg<clk_u0_pcie_tl::CLK_U0_PCIE_TL_SPEC>;
#[doc = "U0 Clock PCIe TL"]
pub mod clk_u0_pcie_tl;
#[doc = "clk_u1_pcie_axi_mst0 (rw) register accessor: U1 Clock PCIe AXI MST 0\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`clk_u1_pcie_axi_mst0::R`].  You can [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`clk_u1_pcie_axi_mst0::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`clk_u1_pcie_axi_mst0`]
module"]
pub type CLK_U1_PCIE_AXI_MST0 = crate::Reg<clk_u1_pcie_axi_mst0::CLK_U1_PCIE_AXI_MST0_SPEC>;
#[doc = "U1 Clock PCIe AXI MST 0"]
pub mod clk_u1_pcie_axi_mst0;
#[doc = "clk_u1_pcie_apb (rw) register accessor: U1 Clock PCIe APB\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`clk_u1_pcie_apb::R`].  You can [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`clk_u1_pcie_apb::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`clk_u1_pcie_apb`]
module"]
pub type CLK_U1_PCIE_APB = crate::Reg<clk_u1_pcie_apb::CLK_U1_PCIE_APB_SPEC>;
#[doc = "U1 Clock PCIe APB"]
pub mod clk_u1_pcie_apb;
#[doc = "clk_u1_pcie_tl (rw) register accessor: U1 Clock PCIe TL\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`clk_u1_pcie_tl::R`].  You can [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`clk_u1_pcie_tl::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`clk_u1_pcie_tl`]
module"]
pub type CLK_U1_PCIE_TL = crate::Reg<clk_u1_pcie_tl::CLK_U1_PCIE_TL_SPEC>;
#[doc = "U1 Clock PCIe TL"]
pub mod clk_u1_pcie_tl;
#[doc = "clk_pcie01_slv_dec_main (rw) register accessor: Clock PCIe 01 SLV DEC Main\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`clk_pcie01_slv_dec_main::R`].  You can [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`clk_pcie01_slv_dec_main::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`clk_pcie01_slv_dec_main`]
module"]
pub type CLK_PCIE01_SLV_DEC_MAIN =
    crate::Reg<clk_pcie01_slv_dec_main::CLK_PCIE01_SLV_DEC_MAIN_SPEC>;
#[doc = "Clock PCIe 01 SLV DEC Main"]
pub mod clk_pcie01_slv_dec_main;
#[doc = "clk_sec_hclk (rw) register accessor: Clock Security HCLK\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`clk_sec_hclk::R`].  You can [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`clk_sec_hclk::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`clk_sec_hclk`]
module"]
pub type CLK_SEC_HCLK = crate::Reg<clk_sec_hclk::CLK_SEC_HCLK_SPEC>;
#[doc = "Clock Security HCLK"]
pub mod clk_sec_hclk;
#[doc = "clk_sec_misc_ahb (rw) register accessor: Clock Security Miscellaneous AHB\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`clk_sec_misc_ahb::R`].  You can [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`clk_sec_misc_ahb::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`clk_sec_misc_ahb`]
module"]
pub type CLK_SEC_MISC_AHB = crate::Reg<clk_sec_misc_ahb::CLK_SEC_MISC_AHB_SPEC>;
#[doc = "Clock Security Miscellaneous AHB"]
pub mod clk_sec_misc_ahb;
#[doc = "clk_stg_mtrx_group0_main (rw) register accessor: Clock STG MTRX Group 0 Main\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`clk_stg_mtrx_group0_main::R`].  You can [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`clk_stg_mtrx_group0_main::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`clk_stg_mtrx_group0_main`]
module"]
pub type CLK_STG_MTRX_GROUP0_MAIN =
    crate::Reg<clk_stg_mtrx_group0_main::CLK_STG_MTRX_GROUP0_MAIN_SPEC>;
#[doc = "Clock STG MTRX Group 0 Main"]
pub mod clk_stg_mtrx_group0_main;
#[doc = "clk_stg_mtrx_group0_bus (rw) register accessor: Clock STG MTRX Group 0 Bus\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`clk_stg_mtrx_group0_bus::R`].  You can [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`clk_stg_mtrx_group0_bus::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`clk_stg_mtrx_group0_bus`]
module"]
pub type CLK_STG_MTRX_GROUP0_BUS =
    crate::Reg<clk_stg_mtrx_group0_bus::CLK_STG_MTRX_GROUP0_BUS_SPEC>;
#[doc = "Clock STG MTRX Group 0 Bus"]
pub mod clk_stg_mtrx_group0_bus;
#[doc = "clk_stg_mtrx_group0_stg (rw) register accessor: Clock STG MTRX Group 0 STG\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`clk_stg_mtrx_group0_stg::R`].  You can [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`clk_stg_mtrx_group0_stg::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`clk_stg_mtrx_group0_stg`]
module"]
pub type CLK_STG_MTRX_GROUP0_STG =
    crate::Reg<clk_stg_mtrx_group0_stg::CLK_STG_MTRX_GROUP0_STG_SPEC>;
#[doc = "Clock STG MTRX Group 0 STG"]
pub mod clk_stg_mtrx_group0_stg;
#[doc = "clk_stg_mtrx_group1_main (rw) register accessor: Clock STG MTRX Group 1 Main\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`clk_stg_mtrx_group1_main::R`].  You can [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`clk_stg_mtrx_group1_main::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`clk_stg_mtrx_group1_main`]
module"]
pub type CLK_STG_MTRX_GROUP1_MAIN =
    crate::Reg<clk_stg_mtrx_group1_main::CLK_STG_MTRX_GROUP1_MAIN_SPEC>;
#[doc = "Clock STG MTRX Group 1 Main"]
pub mod clk_stg_mtrx_group1_main;
#[doc = "clk_stg_mtrx_group1_bus (rw) register accessor: Clock STG MTRX Group 1 Bus\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`clk_stg_mtrx_group1_bus::R`].  You can [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`clk_stg_mtrx_group1_bus::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`clk_stg_mtrx_group1_bus`]
module"]
pub type CLK_STG_MTRX_GROUP1_BUS =
    crate::Reg<clk_stg_mtrx_group1_bus::CLK_STG_MTRX_GROUP1_BUS_SPEC>;
#[doc = "Clock STG MTRX Group 1 Bus"]
pub mod clk_stg_mtrx_group1_bus;
#[doc = "clk_stg_mtrx_group1_stg (rw) register accessor: Clock STG MTRX Group 1 STG\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`clk_stg_mtrx_group1_stg::R`].  You can [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`clk_stg_mtrx_group1_stg::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`clk_stg_mtrx_group1_stg`]
module"]
pub type CLK_STG_MTRX_GROUP1_STG =
    crate::Reg<clk_stg_mtrx_group1_stg::CLK_STG_MTRX_GROUP1_STG_SPEC>;
#[doc = "Clock STG MTRX Group 1 STG"]
pub mod clk_stg_mtrx_group1_stg;
#[doc = "clk_stg_mtrx_group1_hifi (rw) register accessor: Clock STG MTRX Group 1 HIFI\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`clk_stg_mtrx_group1_hifi::R`].  You can [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`clk_stg_mtrx_group1_hifi::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`clk_stg_mtrx_group1_hifi`]
module"]
pub type CLK_STG_MTRX_GROUP1_HIFI =
    crate::Reg<clk_stg_mtrx_group1_hifi::CLK_STG_MTRX_GROUP1_HIFI_SPEC>;
#[doc = "Clock STG MTRX Group 1 HIFI"]
pub mod clk_stg_mtrx_group1_hifi;
#[doc = "clk_e2_rtc (rw) register accessor: Clock E2 RTC\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`clk_e2_rtc::R`].  You can [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`clk_e2_rtc::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`clk_e2_rtc`]
module"]
pub type CLK_E2_RTC = crate::Reg<clk_e2_rtc::CLK_E2_RTC_SPEC>;
#[doc = "Clock E2 RTC"]
pub mod clk_e2_rtc;
#[doc = "clk_e2_core (rw) register accessor: Clock E2 Core\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`clk_e2_core::R`].  You can [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`clk_e2_core::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`clk_e2_core`]
module"]
pub type CLK_E2_CORE = crate::Reg<clk_e2_core::CLK_E2_CORE_SPEC>;
#[doc = "Clock E2 Core"]
pub mod clk_e2_core;
#[doc = "clk_e2_dbg (rw) register accessor: Clock E2 DBG\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`clk_e2_dbg::R`].  You can [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`clk_e2_dbg::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`clk_e2_dbg`]
module"]
pub type CLK_E2_DBG = crate::Reg<clk_e2_dbg::CLK_E2_DBG_SPEC>;
#[doc = "Clock E2 DBG"]
pub mod clk_e2_dbg;
#[doc = "clk_dma_axi (rw) register accessor: Clock DMA AXI\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`clk_dma_axi::R`].  You can [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`clk_dma_axi::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`clk_dma_axi`]
module"]
pub type CLK_DMA_AXI = crate::Reg<clk_dma_axi::CLK_DMA_AXI_SPEC>;
#[doc = "Clock DMA AXI"]
pub mod clk_dma_axi;
#[doc = "clk_dma_ahb (rw) register accessor: Clock DMA AHB\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`clk_dma_ahb::R`].  You can [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`clk_dma_ahb::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`clk_dma_ahb`]
module"]
pub type CLK_DMA_AHB = crate::Reg<clk_dma_ahb::CLK_DMA_AHB_SPEC>;
#[doc = "Clock DMA AHB"]
pub mod clk_dma_ahb;
#[doc = "soft_rst_addr_sel (rw) register accessor: Software RESET Address Selector\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`soft_rst_addr_sel::R`].  You can [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`soft_rst_addr_sel::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`soft_rst_addr_sel`]
module"]
pub type SOFT_RST_ADDR_SEL = crate::Reg<soft_rst_addr_sel::SOFT_RST_ADDR_SEL_SPEC>;
#[doc = "Software RESET Address Selector"]
pub mod soft_rst_addr_sel;
#[doc = "stgcrg_rst_stat (rw) register accessor: STGCRG RESET Status\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`stgcrg_rst_stat::R`].  You can [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`stgcrg_rst_stat::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`stgcrg_rst_stat`]
module"]
pub type STGCRG_RST_STAT = crate::Reg<stgcrg_rst_stat::STGCRG_RST_STAT_SPEC>;
#[doc = "STGCRG RESET Status"]
pub mod stgcrg_rst_stat;
