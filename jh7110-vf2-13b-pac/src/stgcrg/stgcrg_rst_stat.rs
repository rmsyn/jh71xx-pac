#[doc = "Register `stgcrg_rst_stat` reader"]
pub type R = crate::R<STGCRG_RST_STAT_SPEC>;
#[doc = "Register `stgcrg_rst_stat` writer"]
pub type W = crate::W<STGCRG_RST_STAT_SPEC>;
#[doc = "Field `u0_stg_syscon_presetn` reader - 1: Assert reset, 0: De-assert reset"]
pub type U0_STG_SYSCON_PRESETN_R = crate::BitReader;
#[doc = "Field `u0_stg_syscon_presetn` writer - 1: Assert reset, 0: De-assert reset"]
pub type U0_STG_SYSCON_PRESETN_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `u0_hifi4_core` reader - 1: Assert reset, 0: De-assert reset"]
pub type U0_HIFI4_CORE_R = crate::BitReader;
#[doc = "Field `u0_hifi4_core` writer - 1: Assert reset, 0: De-assert reset"]
pub type U0_HIFI4_CORE_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `u0_hifi4_axi` reader - 1: Assert reset, 0: De-assert reset"]
pub type U0_HIFI4_AXI_R = crate::BitReader;
#[doc = "Field `u0_hifi4_axi` writer - 1: Assert reset, 0: De-assert reset"]
pub type U0_HIFI4_AXI_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `u0_sec_top_hreesetn` reader - 1: Assert reset, 0: De-assert reset"]
pub type U0_SEC_TOP_HREESETN_R = crate::BitReader;
#[doc = "Field `u0_sec_top_hreesetn` writer - 1: Assert reset, 0: De-assert reset"]
pub type U0_SEC_TOP_HREESETN_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `u0_e2_core` reader - 1: Assert reset, 0: De-assert reset"]
pub type U0_E2_CORE_R = crate::BitReader;
#[doc = "Field `u0_e2_core` writer - 1: Assert reset, 0: De-assert reset"]
pub type U0_E2_CORE_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `u0_dma_axi` reader - 1: Assert reset, 0: De-assert reset"]
pub type U0_DMA_AXI_R = crate::BitReader;
#[doc = "Field `u0_dma_axi` writer - 1: Assert reset, 0: De-assert reset"]
pub type U0_DMA_AXI_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `u0_dma_ahb` reader - 1: Assert reset, 0: De-assert reset"]
pub type U0_DMA_AHB_R = crate::BitReader;
#[doc = "Field `u0_dma_ahb` writer - 1: Assert reset, 0: De-assert reset"]
pub type U0_DMA_AHB_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `u0_usb_axi` reader - 1: Assert reset, 0: De-assert reset"]
pub type U0_USB_AXI_R = crate::BitReader;
#[doc = "Field `u0_usb_axi` writer - 1: Assert reset, 0: De-assert reset"]
pub type U0_USB_AXI_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `u0_usb_apb` reader - 1: Assert reset, 0: De-assert reset"]
pub type U0_USB_APB_R = crate::BitReader;
#[doc = "Field `u0_usb_apb` writer - 1: Assert reset, 0: De-assert reset"]
pub type U0_USB_APB_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `u0_usb_utmi_apb` reader - 1: Assert reset, 0: De-assert reset"]
pub type U0_USB_UTMI_APB_R = crate::BitReader;
#[doc = "Field `u0_usb_utmi_apb` writer - 1: Assert reset, 0: De-assert reset"]
pub type U0_USB_UTMI_APB_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `u0_usb_pwrup` reader - 1: Assert reset, 0: De-assert reset"]
pub type U0_USB_PWRUP_R = crate::BitReader;
#[doc = "Field `u0_usb_pwrup` writer - 1: Assert reset, 0: De-assert reset"]
pub type U0_USB_PWRUP_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `u0_pcie_axi_mst0` reader - 1: Assert reset, 0: De-assert reset"]
pub type U0_PCIE_AXI_MST0_R = crate::BitReader;
#[doc = "Field `u0_pcie_axi_mst0` writer - 1: Assert reset, 0: De-assert reset"]
pub type U0_PCIE_AXI_MST0_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `u0_pcie_axi_slv0` reader - 1: Assert reset, 0: De-assert reset"]
pub type U0_PCIE_AXI_SLV0_R = crate::BitReader;
#[doc = "Field `u0_pcie_axi_slv0` writer - 1: Assert reset, 0: De-assert reset"]
pub type U0_PCIE_AXI_SLV0_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `u0_pcie_axi_slv` reader - 1: Assert reset, 0: De-assert reset"]
pub type U0_PCIE_AXI_SLV_R = crate::BitReader;
#[doc = "Field `u0_pcie_axi_slv` writer - 1: Assert reset, 0: De-assert reset"]
pub type U0_PCIE_AXI_SLV_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `u0_pci_brg` reader - 1: Assert reset, 0: De-assert reset"]
pub type U0_PCI_BRG_R = crate::BitReader;
#[doc = "Field `u0_pci_brg` writer - 1: Assert reset, 0: De-assert reset"]
pub type U0_PCI_BRG_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `u0_pcie_pcie` reader - 1: Assert reset, 0: De-assert reset"]
pub type U0_PCIE_PCIE_R = crate::BitReader;
#[doc = "Field `u0_pcie_pcie` writer - 1: Assert reset, 0: De-assert reset"]
pub type U0_PCIE_PCIE_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `u0_pcie_apb` reader - 1: Assert reset, 0: De-assert reset"]
pub type U0_PCIE_APB_R = crate::BitReader;
#[doc = "Field `u0_pcie_apb` writer - 1: Assert reset, 0: De-assert reset"]
pub type U0_PCIE_APB_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `u1_pcie_axi_mst0` reader - 1: Assert reset, 0: De-assert reset"]
pub type U1_PCIE_AXI_MST0_R = crate::BitReader;
#[doc = "Field `u1_pcie_axi_mst0` writer - 1: Assert reset, 0: De-assert reset"]
pub type U1_PCIE_AXI_MST0_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `u1_pcie_axi_slv0` reader - 1: Assert reset, 0: De-assert reset"]
pub type U1_PCIE_AXI_SLV0_R = crate::BitReader;
#[doc = "Field `u1_pcie_axi_slv0` writer - 1: Assert reset, 0: De-assert reset"]
pub type U1_PCIE_AXI_SLV0_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `u1_pcie_axi_slv` reader - 1: Assert reset, 0: De-assert reset"]
pub type U1_PCIE_AXI_SLV_R = crate::BitReader;
#[doc = "Field `u1_pcie_axi_slv` writer - 1: Assert reset, 0: De-assert reset"]
pub type U1_PCIE_AXI_SLV_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `u1_pcie_brg` reader - 1: Assert reset, 0: De-assert reset"]
pub type U1_PCIE_BRG_R = crate::BitReader;
#[doc = "Field `u1_pcie_brg` writer - 1: Assert reset, 0: De-assert reset"]
pub type U1_PCIE_BRG_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `u1_pcie_pcie` reader - 1: Assert reset, 0: De-assert reset"]
pub type U1_PCIE_PCIE_R = crate::BitReader;
#[doc = "Field `u1_pcie_pcie` writer - 1: Assert reset, 0: De-assert reset"]
pub type U1_PCIE_PCIE_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `u1_pcie_apb` reader - 1: Assert reset, 0: De-assert reset"]
pub type U1_PCIE_APB_R = crate::BitReader;
#[doc = "Field `u1_pcie_apb` writer - 1: Assert reset, 0: De-assert reset"]
pub type U1_PCIE_APB_W<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bit 0 - 1: Assert reset, 0: De-assert reset"]
    #[inline(always)]
    pub fn u0_stg_syscon_presetn(&self) -> U0_STG_SYSCON_PRESETN_R {
        U0_STG_SYSCON_PRESETN_R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - 1: Assert reset, 0: De-assert reset"]
    #[inline(always)]
    pub fn u0_hifi4_core(&self) -> U0_HIFI4_CORE_R {
        U0_HIFI4_CORE_R::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - 1: Assert reset, 0: De-assert reset"]
    #[inline(always)]
    pub fn u0_hifi4_axi(&self) -> U0_HIFI4_AXI_R {
        U0_HIFI4_AXI_R::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3 - 1: Assert reset, 0: De-assert reset"]
    #[inline(always)]
    pub fn u0_sec_top_hreesetn(&self) -> U0_SEC_TOP_HREESETN_R {
        U0_SEC_TOP_HREESETN_R::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bit 4 - 1: Assert reset, 0: De-assert reset"]
    #[inline(always)]
    pub fn u0_e2_core(&self) -> U0_E2_CORE_R {
        U0_E2_CORE_R::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bit 5 - 1: Assert reset, 0: De-assert reset"]
    #[inline(always)]
    pub fn u0_dma_axi(&self) -> U0_DMA_AXI_R {
        U0_DMA_AXI_R::new(((self.bits >> 5) & 1) != 0)
    }
    #[doc = "Bit 6 - 1: Assert reset, 0: De-assert reset"]
    #[inline(always)]
    pub fn u0_dma_ahb(&self) -> U0_DMA_AHB_R {
        U0_DMA_AHB_R::new(((self.bits >> 6) & 1) != 0)
    }
    #[doc = "Bit 7 - 1: Assert reset, 0: De-assert reset"]
    #[inline(always)]
    pub fn u0_usb_axi(&self) -> U0_USB_AXI_R {
        U0_USB_AXI_R::new(((self.bits >> 7) & 1) != 0)
    }
    #[doc = "Bit 8 - 1: Assert reset, 0: De-assert reset"]
    #[inline(always)]
    pub fn u0_usb_apb(&self) -> U0_USB_APB_R {
        U0_USB_APB_R::new(((self.bits >> 8) & 1) != 0)
    }
    #[doc = "Bit 9 - 1: Assert reset, 0: De-assert reset"]
    #[inline(always)]
    pub fn u0_usb_utmi_apb(&self) -> U0_USB_UTMI_APB_R {
        U0_USB_UTMI_APB_R::new(((self.bits >> 9) & 1) != 0)
    }
    #[doc = "Bit 10 - 1: Assert reset, 0: De-assert reset"]
    #[inline(always)]
    pub fn u0_usb_pwrup(&self) -> U0_USB_PWRUP_R {
        U0_USB_PWRUP_R::new(((self.bits >> 10) & 1) != 0)
    }
    #[doc = "Bit 11 - 1: Assert reset, 0: De-assert reset"]
    #[inline(always)]
    pub fn u0_pcie_axi_mst0(&self) -> U0_PCIE_AXI_MST0_R {
        U0_PCIE_AXI_MST0_R::new(((self.bits >> 11) & 1) != 0)
    }
    #[doc = "Bit 12 - 1: Assert reset, 0: De-assert reset"]
    #[inline(always)]
    pub fn u0_pcie_axi_slv0(&self) -> U0_PCIE_AXI_SLV0_R {
        U0_PCIE_AXI_SLV0_R::new(((self.bits >> 12) & 1) != 0)
    }
    #[doc = "Bit 13 - 1: Assert reset, 0: De-assert reset"]
    #[inline(always)]
    pub fn u0_pcie_axi_slv(&self) -> U0_PCIE_AXI_SLV_R {
        U0_PCIE_AXI_SLV_R::new(((self.bits >> 13) & 1) != 0)
    }
    #[doc = "Bit 14 - 1: Assert reset, 0: De-assert reset"]
    #[inline(always)]
    pub fn u0_pci_brg(&self) -> U0_PCI_BRG_R {
        U0_PCI_BRG_R::new(((self.bits >> 14) & 1) != 0)
    }
    #[doc = "Bit 15 - 1: Assert reset, 0: De-assert reset"]
    #[inline(always)]
    pub fn u0_pcie_pcie(&self) -> U0_PCIE_PCIE_R {
        U0_PCIE_PCIE_R::new(((self.bits >> 15) & 1) != 0)
    }
    #[doc = "Bit 16 - 1: Assert reset, 0: De-assert reset"]
    #[inline(always)]
    pub fn u0_pcie_apb(&self) -> U0_PCIE_APB_R {
        U0_PCIE_APB_R::new(((self.bits >> 16) & 1) != 0)
    }
    #[doc = "Bit 17 - 1: Assert reset, 0: De-assert reset"]
    #[inline(always)]
    pub fn u1_pcie_axi_mst0(&self) -> U1_PCIE_AXI_MST0_R {
        U1_PCIE_AXI_MST0_R::new(((self.bits >> 17) & 1) != 0)
    }
    #[doc = "Bit 18 - 1: Assert reset, 0: De-assert reset"]
    #[inline(always)]
    pub fn u1_pcie_axi_slv0(&self) -> U1_PCIE_AXI_SLV0_R {
        U1_PCIE_AXI_SLV0_R::new(((self.bits >> 18) & 1) != 0)
    }
    #[doc = "Bit 19 - 1: Assert reset, 0: De-assert reset"]
    #[inline(always)]
    pub fn u1_pcie_axi_slv(&self) -> U1_PCIE_AXI_SLV_R {
        U1_PCIE_AXI_SLV_R::new(((self.bits >> 19) & 1) != 0)
    }
    #[doc = "Bit 20 - 1: Assert reset, 0: De-assert reset"]
    #[inline(always)]
    pub fn u1_pcie_brg(&self) -> U1_PCIE_BRG_R {
        U1_PCIE_BRG_R::new(((self.bits >> 20) & 1) != 0)
    }
    #[doc = "Bit 21 - 1: Assert reset, 0: De-assert reset"]
    #[inline(always)]
    pub fn u1_pcie_pcie(&self) -> U1_PCIE_PCIE_R {
        U1_PCIE_PCIE_R::new(((self.bits >> 21) & 1) != 0)
    }
    #[doc = "Bit 22 - 1: Assert reset, 0: De-assert reset"]
    #[inline(always)]
    pub fn u1_pcie_apb(&self) -> U1_PCIE_APB_R {
        U1_PCIE_APB_R::new(((self.bits >> 22) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - 1: Assert reset, 0: De-assert reset"]
    #[inline(always)]
    #[must_use]
    pub fn u0_stg_syscon_presetn(&mut self) -> U0_STG_SYSCON_PRESETN_W<STGCRG_RST_STAT_SPEC> {
        U0_STG_SYSCON_PRESETN_W::new(self, 0)
    }
    #[doc = "Bit 1 - 1: Assert reset, 0: De-assert reset"]
    #[inline(always)]
    #[must_use]
    pub fn u0_hifi4_core(&mut self) -> U0_HIFI4_CORE_W<STGCRG_RST_STAT_SPEC> {
        U0_HIFI4_CORE_W::new(self, 1)
    }
    #[doc = "Bit 2 - 1: Assert reset, 0: De-assert reset"]
    #[inline(always)]
    #[must_use]
    pub fn u0_hifi4_axi(&mut self) -> U0_HIFI4_AXI_W<STGCRG_RST_STAT_SPEC> {
        U0_HIFI4_AXI_W::new(self, 2)
    }
    #[doc = "Bit 3 - 1: Assert reset, 0: De-assert reset"]
    #[inline(always)]
    #[must_use]
    pub fn u0_sec_top_hreesetn(&mut self) -> U0_SEC_TOP_HREESETN_W<STGCRG_RST_STAT_SPEC> {
        U0_SEC_TOP_HREESETN_W::new(self, 3)
    }
    #[doc = "Bit 4 - 1: Assert reset, 0: De-assert reset"]
    #[inline(always)]
    #[must_use]
    pub fn u0_e2_core(&mut self) -> U0_E2_CORE_W<STGCRG_RST_STAT_SPEC> {
        U0_E2_CORE_W::new(self, 4)
    }
    #[doc = "Bit 5 - 1: Assert reset, 0: De-assert reset"]
    #[inline(always)]
    #[must_use]
    pub fn u0_dma_axi(&mut self) -> U0_DMA_AXI_W<STGCRG_RST_STAT_SPEC> {
        U0_DMA_AXI_W::new(self, 5)
    }
    #[doc = "Bit 6 - 1: Assert reset, 0: De-assert reset"]
    #[inline(always)]
    #[must_use]
    pub fn u0_dma_ahb(&mut self) -> U0_DMA_AHB_W<STGCRG_RST_STAT_SPEC> {
        U0_DMA_AHB_W::new(self, 6)
    }
    #[doc = "Bit 7 - 1: Assert reset, 0: De-assert reset"]
    #[inline(always)]
    #[must_use]
    pub fn u0_usb_axi(&mut self) -> U0_USB_AXI_W<STGCRG_RST_STAT_SPEC> {
        U0_USB_AXI_W::new(self, 7)
    }
    #[doc = "Bit 8 - 1: Assert reset, 0: De-assert reset"]
    #[inline(always)]
    #[must_use]
    pub fn u0_usb_apb(&mut self) -> U0_USB_APB_W<STGCRG_RST_STAT_SPEC> {
        U0_USB_APB_W::new(self, 8)
    }
    #[doc = "Bit 9 - 1: Assert reset, 0: De-assert reset"]
    #[inline(always)]
    #[must_use]
    pub fn u0_usb_utmi_apb(&mut self) -> U0_USB_UTMI_APB_W<STGCRG_RST_STAT_SPEC> {
        U0_USB_UTMI_APB_W::new(self, 9)
    }
    #[doc = "Bit 10 - 1: Assert reset, 0: De-assert reset"]
    #[inline(always)]
    #[must_use]
    pub fn u0_usb_pwrup(&mut self) -> U0_USB_PWRUP_W<STGCRG_RST_STAT_SPEC> {
        U0_USB_PWRUP_W::new(self, 10)
    }
    #[doc = "Bit 11 - 1: Assert reset, 0: De-assert reset"]
    #[inline(always)]
    #[must_use]
    pub fn u0_pcie_axi_mst0(&mut self) -> U0_PCIE_AXI_MST0_W<STGCRG_RST_STAT_SPEC> {
        U0_PCIE_AXI_MST0_W::new(self, 11)
    }
    #[doc = "Bit 12 - 1: Assert reset, 0: De-assert reset"]
    #[inline(always)]
    #[must_use]
    pub fn u0_pcie_axi_slv0(&mut self) -> U0_PCIE_AXI_SLV0_W<STGCRG_RST_STAT_SPEC> {
        U0_PCIE_AXI_SLV0_W::new(self, 12)
    }
    #[doc = "Bit 13 - 1: Assert reset, 0: De-assert reset"]
    #[inline(always)]
    #[must_use]
    pub fn u0_pcie_axi_slv(&mut self) -> U0_PCIE_AXI_SLV_W<STGCRG_RST_STAT_SPEC> {
        U0_PCIE_AXI_SLV_W::new(self, 13)
    }
    #[doc = "Bit 14 - 1: Assert reset, 0: De-assert reset"]
    #[inline(always)]
    #[must_use]
    pub fn u0_pci_brg(&mut self) -> U0_PCI_BRG_W<STGCRG_RST_STAT_SPEC> {
        U0_PCI_BRG_W::new(self, 14)
    }
    #[doc = "Bit 15 - 1: Assert reset, 0: De-assert reset"]
    #[inline(always)]
    #[must_use]
    pub fn u0_pcie_pcie(&mut self) -> U0_PCIE_PCIE_W<STGCRG_RST_STAT_SPEC> {
        U0_PCIE_PCIE_W::new(self, 15)
    }
    #[doc = "Bit 16 - 1: Assert reset, 0: De-assert reset"]
    #[inline(always)]
    #[must_use]
    pub fn u0_pcie_apb(&mut self) -> U0_PCIE_APB_W<STGCRG_RST_STAT_SPEC> {
        U0_PCIE_APB_W::new(self, 16)
    }
    #[doc = "Bit 17 - 1: Assert reset, 0: De-assert reset"]
    #[inline(always)]
    #[must_use]
    pub fn u1_pcie_axi_mst0(&mut self) -> U1_PCIE_AXI_MST0_W<STGCRG_RST_STAT_SPEC> {
        U1_PCIE_AXI_MST0_W::new(self, 17)
    }
    #[doc = "Bit 18 - 1: Assert reset, 0: De-assert reset"]
    #[inline(always)]
    #[must_use]
    pub fn u1_pcie_axi_slv0(&mut self) -> U1_PCIE_AXI_SLV0_W<STGCRG_RST_STAT_SPEC> {
        U1_PCIE_AXI_SLV0_W::new(self, 18)
    }
    #[doc = "Bit 19 - 1: Assert reset, 0: De-assert reset"]
    #[inline(always)]
    #[must_use]
    pub fn u1_pcie_axi_slv(&mut self) -> U1_PCIE_AXI_SLV_W<STGCRG_RST_STAT_SPEC> {
        U1_PCIE_AXI_SLV_W::new(self, 19)
    }
    #[doc = "Bit 20 - 1: Assert reset, 0: De-assert reset"]
    #[inline(always)]
    #[must_use]
    pub fn u1_pcie_brg(&mut self) -> U1_PCIE_BRG_W<STGCRG_RST_STAT_SPEC> {
        U1_PCIE_BRG_W::new(self, 20)
    }
    #[doc = "Bit 21 - 1: Assert reset, 0: De-assert reset"]
    #[inline(always)]
    #[must_use]
    pub fn u1_pcie_pcie(&mut self) -> U1_PCIE_PCIE_W<STGCRG_RST_STAT_SPEC> {
        U1_PCIE_PCIE_W::new(self, 21)
    }
    #[doc = "Bit 22 - 1: Assert reset, 0: De-assert reset"]
    #[inline(always)]
    #[must_use]
    pub fn u1_pcie_apb(&mut self) -> U1_PCIE_APB_W<STGCRG_RST_STAT_SPEC> {
        U1_PCIE_APB_W::new(self, 22)
    }
    #[doc = r" Writes raw bits to the register."]
    #[doc = r""]
    #[doc = r" # Safety"]
    #[doc = r""]
    #[doc = r" Passing incorrect value can cause undefined behaviour. See reference manual"]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.bits = bits;
        self
    }
}
#[doc = "STGCRG RESET Status\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`stgcrg_rst_stat::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`stgcrg_rst_stat::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct STGCRG_RST_STAT_SPEC;
impl crate::RegisterSpec for STGCRG_RST_STAT_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`stgcrg_rst_stat::R`](R) reader structure"]
impl crate::Readable for STGCRG_RST_STAT_SPEC {}
#[doc = "`write(|w| ..)` method takes [`stgcrg_rst_stat::W`](W) writer structure"]
impl crate::Writable for STGCRG_RST_STAT_SPEC {
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets stgcrg_rst_stat to value 0"]
impl crate::Resettable for STGCRG_RST_STAT_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
