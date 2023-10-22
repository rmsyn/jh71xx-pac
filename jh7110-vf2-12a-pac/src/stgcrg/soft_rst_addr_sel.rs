#[doc = "Register `soft_rst_addr_sel` reader"]
pub type R = crate::R<SOFT_RST_ADDR_SEL_SPEC>;
#[doc = "Register `soft_rst_addr_sel` writer"]
pub type W = crate::W<SOFT_RST_ADDR_SEL_SPEC>;
#[doc = "Field `rstn_u0_stg_syscon_presetn` reader - 1: Assert reset, 0: De-assert reset"]
pub type RSTN_U0_STG_SYSCON_PRESETN_R = crate::BitReader;
#[doc = "Field `rstn_u0_stg_syscon_presetn` writer - 1: Assert reset, 0: De-assert reset"]
pub type RSTN_U0_STG_SYSCON_PRESETN_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O>;
#[doc = "Field `rst_u0_hifi4_rst_core` reader - 1: Assert reset, 0: De-assert reset"]
pub type RST_U0_HIFI4_RST_CORE_R = crate::BitReader;
#[doc = "Field `rst_u0_hifi4_rst_core` writer - 1: Assert reset, 0: De-assert reset"]
pub type RST_U0_HIFI4_RST_CORE_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O>;
#[doc = "Field `rst_u0_hifi4_rst_axi` reader - 1: Assert reset, 0: De-assert reset"]
pub type RST_U0_HIFI4_RST_AXI_R = crate::BitReader;
#[doc = "Field `rst_u0_hifi4_rst_axi` writer - 1: Assert reset, 0: De-assert reset"]
pub type RST_U0_HIFI4_RST_AXI_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O>;
#[doc = "Field `rstn_u0_sec_top_hreesetn` reader - 1: Assert reset, 0: De-assert reset"]
pub type RSTN_U0_SEC_TOP_HREESETN_R = crate::BitReader;
#[doc = "Field `rstn_u0_sec_top_hreesetn` writer - 1: Assert reset, 0: De-assert reset"]
pub type RSTN_U0_SEC_TOP_HREESETN_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O>;
#[doc = "Field `rst_u0_e2_sft7110_rst_core` reader - 1: Assert reset, 0: De-assert reset"]
pub type RST_U0_E2_SFT7110_RST_CORE_R = crate::BitReader;
#[doc = "Field `rst_u0_e2_sft7110_rst_core` writer - 1: Assert reset, 0: De-assert reset"]
pub type RST_U0_E2_SFT7110_RST_CORE_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O>;
#[doc = "Field `rstn_u0_dma1p_8ch_56hs_rstn_axi` reader - 1: Assert reset, 0: De-assert reset"]
pub type RSTN_U0_DMA1P_8CH_56HS_RSTN_AXI_R = crate::BitReader;
#[doc = "Field `rstn_u0_dma1p_8ch_56hs_rstn_axi` writer - 1: Assert reset, 0: De-assert reset"]
pub type RSTN_U0_DMA1P_8CH_56HS_RSTN_AXI_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O>;
#[doc = "Field `rstn_u0_dma1p_8ch_56hs_rstn_ahb` reader - 1: Assert reset, 0: De-assert reset"]
pub type RSTN_U0_DMA1P_8CH_56HS_RSTN_AHB_R = crate::BitReader;
#[doc = "Field `rstn_u0_dma1p_8ch_56hs_rstn_ahb` writer - 1: Assert reset, 0: De-assert reset"]
pub type RSTN_U0_DMA1P_8CH_56HS_RSTN_AHB_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O>;
#[doc = "Field `rstn_u0_cdn_usb_rstn_axi` reader - 1: Assert reset, 0: De-assert reset"]
pub type RSTN_U0_CDN_USB_RSTN_AXI_R = crate::BitReader;
#[doc = "Field `rstn_u0_cdn_usb_rstn_axi` writer - 1: Assert reset, 0: De-assert reset"]
pub type RSTN_U0_CDN_USB_RSTN_AXI_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O>;
#[doc = "Field `rstn_u0_cdn_usb_rstn_usb_apb` reader - 1: Assert reset, 0: De-assert reset"]
pub type RSTN_U0_CDN_USB_RSTN_USB_APB_R = crate::BitReader;
#[doc = "Field `rstn_u0_cdn_usb_rstn_usb_apb` writer - 1: Assert reset, 0: De-assert reset"]
pub type RSTN_U0_CDN_USB_RSTN_USB_APB_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O>;
#[doc = "Field `rstn_u0_cdn_usb_rstn_utmi_apb` reader - 1: Assert reset, 0: De-assert reset"]
pub type RSTN_U0_CDN_USB_RSTN_UTMI_APB_R = crate::BitReader;
#[doc = "Field `rstn_u0_cdn_usb_rstn_utmi_apb` writer - 1: Assert reset, 0: De-assert reset"]
pub type RSTN_U0_CDN_USB_RSTN_UTMI_APB_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O>;
#[doc = "Field `rstn_u0_cdn_usb_rstn_pwrup` reader - 1: Assert reset, 0: De-assert reset"]
pub type RSTN_U0_CDN_USB_RSTN_PWRUP_R = crate::BitReader;
#[doc = "Field `rstn_u0_cdn_usb_rstn_pwrup` writer - 1: Assert reset, 0: De-assert reset"]
pub type RSTN_U0_CDN_USB_RSTN_PWRUP_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O>;
#[doc = "Field `rstn_u0_plda_pcie_rstn_axi_mst0` reader - 1: Assert reset, 0: De-assert reset"]
pub type RSTN_U0_PLDA_PCIE_RSTN_AXI_MST0_R = crate::BitReader;
#[doc = "Field `rstn_u0_plda_pcie_rstn_axi_mst0` writer - 1: Assert reset, 0: De-assert reset"]
pub type RSTN_U0_PLDA_PCIE_RSTN_AXI_MST0_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O>;
#[doc = "Field `rstn_u0_plda_pcie_rstn_axi_slv0` reader - 1: Assert reset, 0: De-assert reset"]
pub type RSTN_U0_PLDA_PCIE_RSTN_AXI_SLV0_R = crate::BitReader;
#[doc = "Field `rstn_u0_plda_pcie_rstn_axi_slv0` writer - 1: Assert reset, 0: De-assert reset"]
pub type RSTN_U0_PLDA_PCIE_RSTN_AXI_SLV0_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O>;
#[doc = "Field `rstn_u0_plda_pcie_rstn_axi_slv` reader - 1: Assert reset, 0: De-assert reset"]
pub type RSTN_U0_PLDA_PCIE_RSTN_AXI_SLV_R = crate::BitReader;
#[doc = "Field `rstn_u0_plda_pcie_rstn_axi_slv` writer - 1: Assert reset, 0: De-assert reset"]
pub type RSTN_U0_PLDA_PCIE_RSTN_AXI_SLV_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O>;
#[doc = "Field `rstn_u0_plda_pci_rstn_brg` reader - 1: Assert reset, 0: De-assert reset"]
pub type RSTN_U0_PLDA_PCI_RSTN_BRG_R = crate::BitReader;
#[doc = "Field `rstn_u0_plda_pci_rstn_brg` writer - 1: Assert reset, 0: De-assert reset"]
pub type RSTN_U0_PLDA_PCI_RSTN_BRG_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O>;
#[doc = "Field `rstn_u0_plda_pcie_rstn_pcie` reader - 1: Assert reset, 0: De-assert reset"]
pub type RSTN_U0_PLDA_PCIE_RSTN_PCIE_R = crate::BitReader;
#[doc = "Field `rstn_u0_plda_pcie_rstn_pcie` writer - 1: Assert reset, 0: De-assert reset"]
pub type RSTN_U0_PLDA_PCIE_RSTN_PCIE_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O>;
#[doc = "Field `rstn_u0_plda_pcie_rstn_apb` reader - 1: Assert reset, 0: De-assert reset"]
pub type RSTN_U0_PLDA_PCIE_RSTN_APB_R = crate::BitReader;
#[doc = "Field `rstn_u0_plda_pcie_rstn_apb` writer - 1: Assert reset, 0: De-assert reset"]
pub type RSTN_U0_PLDA_PCIE_RSTN_APB_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O>;
#[doc = "Field `rstn_u1_plda_pcie_rstn_axi_mst0` reader - 1: Assert reset, 0: De-assert reset"]
pub type RSTN_U1_PLDA_PCIE_RSTN_AXI_MST0_R = crate::BitReader;
#[doc = "Field `rstn_u1_plda_pcie_rstn_axi_mst0` writer - 1: Assert reset, 0: De-assert reset"]
pub type RSTN_U1_PLDA_PCIE_RSTN_AXI_MST0_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O>;
#[doc = "Field `rstn_u1_plda_pcie_rstn_axi_slv0` reader - 1: Assert reset, 0: De-assert reset"]
pub type RSTN_U1_PLDA_PCIE_RSTN_AXI_SLV0_R = crate::BitReader;
#[doc = "Field `rstn_u1_plda_pcie_rstn_axi_slv0` writer - 1: Assert reset, 0: De-assert reset"]
pub type RSTN_U1_PLDA_PCIE_RSTN_AXI_SLV0_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O>;
#[doc = "Field `rstn_u1_plda_pcie_rstn_axi_slv` reader - 1: Assert reset, 0: De-assert reset"]
pub type RSTN_U1_PLDA_PCIE_RSTN_AXI_SLV_R = crate::BitReader;
#[doc = "Field `rstn_u1_plda_pcie_rstn_axi_slv` writer - 1: Assert reset, 0: De-assert reset"]
pub type RSTN_U1_PLDA_PCIE_RSTN_AXI_SLV_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O>;
#[doc = "Field `rstn_u1_plda_pcie_rstn_brg` reader - 1: Assert reset, 0: De-assert reset"]
pub type RSTN_U1_PLDA_PCIE_RSTN_BRG_R = crate::BitReader;
#[doc = "Field `rstn_u1_plda_pcie_rstn_brg` writer - 1: Assert reset, 0: De-assert reset"]
pub type RSTN_U1_PLDA_PCIE_RSTN_BRG_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O>;
#[doc = "Field `rstn_u1_plda_pcie_rstn_pcie` reader - 1: Assert reset, 0: De-assert reset"]
pub type RSTN_U1_PLDA_PCIE_RSTN_PCIE_R = crate::BitReader;
#[doc = "Field `rstn_u1_plda_pcie_rstn_pcie` writer - 1: Assert reset, 0: De-assert reset"]
pub type RSTN_U1_PLDA_PCIE_RSTN_PCIE_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O>;
#[doc = "Field `rstn_u1_plda_pcie_rstn_apb` reader - 1: Assert reset, 0: De-assert reset"]
pub type RSTN_U1_PLDA_PCIE_RSTN_APB_R = crate::BitReader;
#[doc = "Field `rstn_u1_plda_pcie_rstn_apb` writer - 1: Assert reset, 0: De-assert reset"]
pub type RSTN_U1_PLDA_PCIE_RSTN_APB_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O>;
impl R {
    #[doc = "Bit 0 - 1: Assert reset, 0: De-assert reset"]
    #[inline(always)]
    pub fn rstn_u0_stg_syscon_presetn(&self) -> RSTN_U0_STG_SYSCON_PRESETN_R {
        RSTN_U0_STG_SYSCON_PRESETN_R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - 1: Assert reset, 0: De-assert reset"]
    #[inline(always)]
    pub fn rst_u0_hifi4_rst_core(&self) -> RST_U0_HIFI4_RST_CORE_R {
        RST_U0_HIFI4_RST_CORE_R::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - 1: Assert reset, 0: De-assert reset"]
    #[inline(always)]
    pub fn rst_u0_hifi4_rst_axi(&self) -> RST_U0_HIFI4_RST_AXI_R {
        RST_U0_HIFI4_RST_AXI_R::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3 - 1: Assert reset, 0: De-assert reset"]
    #[inline(always)]
    pub fn rstn_u0_sec_top_hreesetn(&self) -> RSTN_U0_SEC_TOP_HREESETN_R {
        RSTN_U0_SEC_TOP_HREESETN_R::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bit 4 - 1: Assert reset, 0: De-assert reset"]
    #[inline(always)]
    pub fn rst_u0_e2_sft7110_rst_core(&self) -> RST_U0_E2_SFT7110_RST_CORE_R {
        RST_U0_E2_SFT7110_RST_CORE_R::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bit 5 - 1: Assert reset, 0: De-assert reset"]
    #[inline(always)]
    pub fn rstn_u0_dma1p_8ch_56hs_rstn_axi(&self) -> RSTN_U0_DMA1P_8CH_56HS_RSTN_AXI_R {
        RSTN_U0_DMA1P_8CH_56HS_RSTN_AXI_R::new(((self.bits >> 5) & 1) != 0)
    }
    #[doc = "Bit 6 - 1: Assert reset, 0: De-assert reset"]
    #[inline(always)]
    pub fn rstn_u0_dma1p_8ch_56hs_rstn_ahb(&self) -> RSTN_U0_DMA1P_8CH_56HS_RSTN_AHB_R {
        RSTN_U0_DMA1P_8CH_56HS_RSTN_AHB_R::new(((self.bits >> 6) & 1) != 0)
    }
    #[doc = "Bit 7 - 1: Assert reset, 0: De-assert reset"]
    #[inline(always)]
    pub fn rstn_u0_cdn_usb_rstn_axi(&self) -> RSTN_U0_CDN_USB_RSTN_AXI_R {
        RSTN_U0_CDN_USB_RSTN_AXI_R::new(((self.bits >> 7) & 1) != 0)
    }
    #[doc = "Bit 8 - 1: Assert reset, 0: De-assert reset"]
    #[inline(always)]
    pub fn rstn_u0_cdn_usb_rstn_usb_apb(&self) -> RSTN_U0_CDN_USB_RSTN_USB_APB_R {
        RSTN_U0_CDN_USB_RSTN_USB_APB_R::new(((self.bits >> 8) & 1) != 0)
    }
    #[doc = "Bit 9 - 1: Assert reset, 0: De-assert reset"]
    #[inline(always)]
    pub fn rstn_u0_cdn_usb_rstn_utmi_apb(&self) -> RSTN_U0_CDN_USB_RSTN_UTMI_APB_R {
        RSTN_U0_CDN_USB_RSTN_UTMI_APB_R::new(((self.bits >> 9) & 1) != 0)
    }
    #[doc = "Bit 10 - 1: Assert reset, 0: De-assert reset"]
    #[inline(always)]
    pub fn rstn_u0_cdn_usb_rstn_pwrup(&self) -> RSTN_U0_CDN_USB_RSTN_PWRUP_R {
        RSTN_U0_CDN_USB_RSTN_PWRUP_R::new(((self.bits >> 10) & 1) != 0)
    }
    #[doc = "Bit 11 - 1: Assert reset, 0: De-assert reset"]
    #[inline(always)]
    pub fn rstn_u0_plda_pcie_rstn_axi_mst0(&self) -> RSTN_U0_PLDA_PCIE_RSTN_AXI_MST0_R {
        RSTN_U0_PLDA_PCIE_RSTN_AXI_MST0_R::new(((self.bits >> 11) & 1) != 0)
    }
    #[doc = "Bit 12 - 1: Assert reset, 0: De-assert reset"]
    #[inline(always)]
    pub fn rstn_u0_plda_pcie_rstn_axi_slv0(&self) -> RSTN_U0_PLDA_PCIE_RSTN_AXI_SLV0_R {
        RSTN_U0_PLDA_PCIE_RSTN_AXI_SLV0_R::new(((self.bits >> 12) & 1) != 0)
    }
    #[doc = "Bit 13 - 1: Assert reset, 0: De-assert reset"]
    #[inline(always)]
    pub fn rstn_u0_plda_pcie_rstn_axi_slv(&self) -> RSTN_U0_PLDA_PCIE_RSTN_AXI_SLV_R {
        RSTN_U0_PLDA_PCIE_RSTN_AXI_SLV_R::new(((self.bits >> 13) & 1) != 0)
    }
    #[doc = "Bit 14 - 1: Assert reset, 0: De-assert reset"]
    #[inline(always)]
    pub fn rstn_u0_plda_pci_rstn_brg(&self) -> RSTN_U0_PLDA_PCI_RSTN_BRG_R {
        RSTN_U0_PLDA_PCI_RSTN_BRG_R::new(((self.bits >> 14) & 1) != 0)
    }
    #[doc = "Bit 15 - 1: Assert reset, 0: De-assert reset"]
    #[inline(always)]
    pub fn rstn_u0_plda_pcie_rstn_pcie(&self) -> RSTN_U0_PLDA_PCIE_RSTN_PCIE_R {
        RSTN_U0_PLDA_PCIE_RSTN_PCIE_R::new(((self.bits >> 15) & 1) != 0)
    }
    #[doc = "Bit 16 - 1: Assert reset, 0: De-assert reset"]
    #[inline(always)]
    pub fn rstn_u0_plda_pcie_rstn_apb(&self) -> RSTN_U0_PLDA_PCIE_RSTN_APB_R {
        RSTN_U0_PLDA_PCIE_RSTN_APB_R::new(((self.bits >> 16) & 1) != 0)
    }
    #[doc = "Bit 17 - 1: Assert reset, 0: De-assert reset"]
    #[inline(always)]
    pub fn rstn_u1_plda_pcie_rstn_axi_mst0(&self) -> RSTN_U1_PLDA_PCIE_RSTN_AXI_MST0_R {
        RSTN_U1_PLDA_PCIE_RSTN_AXI_MST0_R::new(((self.bits >> 17) & 1) != 0)
    }
    #[doc = "Bit 18 - 1: Assert reset, 0: De-assert reset"]
    #[inline(always)]
    pub fn rstn_u1_plda_pcie_rstn_axi_slv0(&self) -> RSTN_U1_PLDA_PCIE_RSTN_AXI_SLV0_R {
        RSTN_U1_PLDA_PCIE_RSTN_AXI_SLV0_R::new(((self.bits >> 18) & 1) != 0)
    }
    #[doc = "Bit 19 - 1: Assert reset, 0: De-assert reset"]
    #[inline(always)]
    pub fn rstn_u1_plda_pcie_rstn_axi_slv(&self) -> RSTN_U1_PLDA_PCIE_RSTN_AXI_SLV_R {
        RSTN_U1_PLDA_PCIE_RSTN_AXI_SLV_R::new(((self.bits >> 19) & 1) != 0)
    }
    #[doc = "Bit 20 - 1: Assert reset, 0: De-assert reset"]
    #[inline(always)]
    pub fn rstn_u1_plda_pcie_rstn_brg(&self) -> RSTN_U1_PLDA_PCIE_RSTN_BRG_R {
        RSTN_U1_PLDA_PCIE_RSTN_BRG_R::new(((self.bits >> 20) & 1) != 0)
    }
    #[doc = "Bit 21 - 1: Assert reset, 0: De-assert reset"]
    #[inline(always)]
    pub fn rstn_u1_plda_pcie_rstn_pcie(&self) -> RSTN_U1_PLDA_PCIE_RSTN_PCIE_R {
        RSTN_U1_PLDA_PCIE_RSTN_PCIE_R::new(((self.bits >> 21) & 1) != 0)
    }
    #[doc = "Bit 22 - 1: Assert reset, 0: De-assert reset"]
    #[inline(always)]
    pub fn rstn_u1_plda_pcie_rstn_apb(&self) -> RSTN_U1_PLDA_PCIE_RSTN_APB_R {
        RSTN_U1_PLDA_PCIE_RSTN_APB_R::new(((self.bits >> 22) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - 1: Assert reset, 0: De-assert reset"]
    #[inline(always)]
    #[must_use]
    pub fn rstn_u0_stg_syscon_presetn(
        &mut self,
    ) -> RSTN_U0_STG_SYSCON_PRESETN_W<SOFT_RST_ADDR_SEL_SPEC, 0> {
        RSTN_U0_STG_SYSCON_PRESETN_W::new(self)
    }
    #[doc = "Bit 1 - 1: Assert reset, 0: De-assert reset"]
    #[inline(always)]
    #[must_use]
    pub fn rst_u0_hifi4_rst_core(&mut self) -> RST_U0_HIFI4_RST_CORE_W<SOFT_RST_ADDR_SEL_SPEC, 1> {
        RST_U0_HIFI4_RST_CORE_W::new(self)
    }
    #[doc = "Bit 2 - 1: Assert reset, 0: De-assert reset"]
    #[inline(always)]
    #[must_use]
    pub fn rst_u0_hifi4_rst_axi(&mut self) -> RST_U0_HIFI4_RST_AXI_W<SOFT_RST_ADDR_SEL_SPEC, 2> {
        RST_U0_HIFI4_RST_AXI_W::new(self)
    }
    #[doc = "Bit 3 - 1: Assert reset, 0: De-assert reset"]
    #[inline(always)]
    #[must_use]
    pub fn rstn_u0_sec_top_hreesetn(
        &mut self,
    ) -> RSTN_U0_SEC_TOP_HREESETN_W<SOFT_RST_ADDR_SEL_SPEC, 3> {
        RSTN_U0_SEC_TOP_HREESETN_W::new(self)
    }
    #[doc = "Bit 4 - 1: Assert reset, 0: De-assert reset"]
    #[inline(always)]
    #[must_use]
    pub fn rst_u0_e2_sft7110_rst_core(
        &mut self,
    ) -> RST_U0_E2_SFT7110_RST_CORE_W<SOFT_RST_ADDR_SEL_SPEC, 4> {
        RST_U0_E2_SFT7110_RST_CORE_W::new(self)
    }
    #[doc = "Bit 5 - 1: Assert reset, 0: De-assert reset"]
    #[inline(always)]
    #[must_use]
    pub fn rstn_u0_dma1p_8ch_56hs_rstn_axi(
        &mut self,
    ) -> RSTN_U0_DMA1P_8CH_56HS_RSTN_AXI_W<SOFT_RST_ADDR_SEL_SPEC, 5> {
        RSTN_U0_DMA1P_8CH_56HS_RSTN_AXI_W::new(self)
    }
    #[doc = "Bit 6 - 1: Assert reset, 0: De-assert reset"]
    #[inline(always)]
    #[must_use]
    pub fn rstn_u0_dma1p_8ch_56hs_rstn_ahb(
        &mut self,
    ) -> RSTN_U0_DMA1P_8CH_56HS_RSTN_AHB_W<SOFT_RST_ADDR_SEL_SPEC, 6> {
        RSTN_U0_DMA1P_8CH_56HS_RSTN_AHB_W::new(self)
    }
    #[doc = "Bit 7 - 1: Assert reset, 0: De-assert reset"]
    #[inline(always)]
    #[must_use]
    pub fn rstn_u0_cdn_usb_rstn_axi(
        &mut self,
    ) -> RSTN_U0_CDN_USB_RSTN_AXI_W<SOFT_RST_ADDR_SEL_SPEC, 7> {
        RSTN_U0_CDN_USB_RSTN_AXI_W::new(self)
    }
    #[doc = "Bit 8 - 1: Assert reset, 0: De-assert reset"]
    #[inline(always)]
    #[must_use]
    pub fn rstn_u0_cdn_usb_rstn_usb_apb(
        &mut self,
    ) -> RSTN_U0_CDN_USB_RSTN_USB_APB_W<SOFT_RST_ADDR_SEL_SPEC, 8> {
        RSTN_U0_CDN_USB_RSTN_USB_APB_W::new(self)
    }
    #[doc = "Bit 9 - 1: Assert reset, 0: De-assert reset"]
    #[inline(always)]
    #[must_use]
    pub fn rstn_u0_cdn_usb_rstn_utmi_apb(
        &mut self,
    ) -> RSTN_U0_CDN_USB_RSTN_UTMI_APB_W<SOFT_RST_ADDR_SEL_SPEC, 9> {
        RSTN_U0_CDN_USB_RSTN_UTMI_APB_W::new(self)
    }
    #[doc = "Bit 10 - 1: Assert reset, 0: De-assert reset"]
    #[inline(always)]
    #[must_use]
    pub fn rstn_u0_cdn_usb_rstn_pwrup(
        &mut self,
    ) -> RSTN_U0_CDN_USB_RSTN_PWRUP_W<SOFT_RST_ADDR_SEL_SPEC, 10> {
        RSTN_U0_CDN_USB_RSTN_PWRUP_W::new(self)
    }
    #[doc = "Bit 11 - 1: Assert reset, 0: De-assert reset"]
    #[inline(always)]
    #[must_use]
    pub fn rstn_u0_plda_pcie_rstn_axi_mst0(
        &mut self,
    ) -> RSTN_U0_PLDA_PCIE_RSTN_AXI_MST0_W<SOFT_RST_ADDR_SEL_SPEC, 11> {
        RSTN_U0_PLDA_PCIE_RSTN_AXI_MST0_W::new(self)
    }
    #[doc = "Bit 12 - 1: Assert reset, 0: De-assert reset"]
    #[inline(always)]
    #[must_use]
    pub fn rstn_u0_plda_pcie_rstn_axi_slv0(
        &mut self,
    ) -> RSTN_U0_PLDA_PCIE_RSTN_AXI_SLV0_W<SOFT_RST_ADDR_SEL_SPEC, 12> {
        RSTN_U0_PLDA_PCIE_RSTN_AXI_SLV0_W::new(self)
    }
    #[doc = "Bit 13 - 1: Assert reset, 0: De-assert reset"]
    #[inline(always)]
    #[must_use]
    pub fn rstn_u0_plda_pcie_rstn_axi_slv(
        &mut self,
    ) -> RSTN_U0_PLDA_PCIE_RSTN_AXI_SLV_W<SOFT_RST_ADDR_SEL_SPEC, 13> {
        RSTN_U0_PLDA_PCIE_RSTN_AXI_SLV_W::new(self)
    }
    #[doc = "Bit 14 - 1: Assert reset, 0: De-assert reset"]
    #[inline(always)]
    #[must_use]
    pub fn rstn_u0_plda_pci_rstn_brg(
        &mut self,
    ) -> RSTN_U0_PLDA_PCI_RSTN_BRG_W<SOFT_RST_ADDR_SEL_SPEC, 14> {
        RSTN_U0_PLDA_PCI_RSTN_BRG_W::new(self)
    }
    #[doc = "Bit 15 - 1: Assert reset, 0: De-assert reset"]
    #[inline(always)]
    #[must_use]
    pub fn rstn_u0_plda_pcie_rstn_pcie(
        &mut self,
    ) -> RSTN_U0_PLDA_PCIE_RSTN_PCIE_W<SOFT_RST_ADDR_SEL_SPEC, 15> {
        RSTN_U0_PLDA_PCIE_RSTN_PCIE_W::new(self)
    }
    #[doc = "Bit 16 - 1: Assert reset, 0: De-assert reset"]
    #[inline(always)]
    #[must_use]
    pub fn rstn_u0_plda_pcie_rstn_apb(
        &mut self,
    ) -> RSTN_U0_PLDA_PCIE_RSTN_APB_W<SOFT_RST_ADDR_SEL_SPEC, 16> {
        RSTN_U0_PLDA_PCIE_RSTN_APB_W::new(self)
    }
    #[doc = "Bit 17 - 1: Assert reset, 0: De-assert reset"]
    #[inline(always)]
    #[must_use]
    pub fn rstn_u1_plda_pcie_rstn_axi_mst0(
        &mut self,
    ) -> RSTN_U1_PLDA_PCIE_RSTN_AXI_MST0_W<SOFT_RST_ADDR_SEL_SPEC, 17> {
        RSTN_U1_PLDA_PCIE_RSTN_AXI_MST0_W::new(self)
    }
    #[doc = "Bit 18 - 1: Assert reset, 0: De-assert reset"]
    #[inline(always)]
    #[must_use]
    pub fn rstn_u1_plda_pcie_rstn_axi_slv0(
        &mut self,
    ) -> RSTN_U1_PLDA_PCIE_RSTN_AXI_SLV0_W<SOFT_RST_ADDR_SEL_SPEC, 18> {
        RSTN_U1_PLDA_PCIE_RSTN_AXI_SLV0_W::new(self)
    }
    #[doc = "Bit 19 - 1: Assert reset, 0: De-assert reset"]
    #[inline(always)]
    #[must_use]
    pub fn rstn_u1_plda_pcie_rstn_axi_slv(
        &mut self,
    ) -> RSTN_U1_PLDA_PCIE_RSTN_AXI_SLV_W<SOFT_RST_ADDR_SEL_SPEC, 19> {
        RSTN_U1_PLDA_PCIE_RSTN_AXI_SLV_W::new(self)
    }
    #[doc = "Bit 20 - 1: Assert reset, 0: De-assert reset"]
    #[inline(always)]
    #[must_use]
    pub fn rstn_u1_plda_pcie_rstn_brg(
        &mut self,
    ) -> RSTN_U1_PLDA_PCIE_RSTN_BRG_W<SOFT_RST_ADDR_SEL_SPEC, 20> {
        RSTN_U1_PLDA_PCIE_RSTN_BRG_W::new(self)
    }
    #[doc = "Bit 21 - 1: Assert reset, 0: De-assert reset"]
    #[inline(always)]
    #[must_use]
    pub fn rstn_u1_plda_pcie_rstn_pcie(
        &mut self,
    ) -> RSTN_U1_PLDA_PCIE_RSTN_PCIE_W<SOFT_RST_ADDR_SEL_SPEC, 21> {
        RSTN_U1_PLDA_PCIE_RSTN_PCIE_W::new(self)
    }
    #[doc = "Bit 22 - 1: Assert reset, 0: De-assert reset"]
    #[inline(always)]
    #[must_use]
    pub fn rstn_u1_plda_pcie_rstn_apb(
        &mut self,
    ) -> RSTN_U1_PLDA_PCIE_RSTN_APB_W<SOFT_RST_ADDR_SEL_SPEC, 22> {
        RSTN_U1_PLDA_PCIE_RSTN_APB_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.bits = bits;
        self
    }
}
#[doc = "Software RESET Address Selector\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`soft_rst_addr_sel::R`](R).  You can [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`soft_rst_addr_sel::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct SOFT_RST_ADDR_SEL_SPEC;
impl crate::RegisterSpec for SOFT_RST_ADDR_SEL_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`soft_rst_addr_sel::R`](R) reader structure"]
impl crate::Readable for SOFT_RST_ADDR_SEL_SPEC {}
#[doc = "`write(|w| ..)` method takes [`soft_rst_addr_sel::W`](W) writer structure"]
impl crate::Writable for SOFT_RST_ADDR_SEL_SPEC {
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}