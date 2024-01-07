#[doc = "Register `stg_syscfg_125` reader"]
pub type R = crate::R<STG_SYSCFG_125_SPEC>;
#[doc = "Register `stg_syscfg_125` writer"]
pub type W = crate::W<STG_SYSCFG_125_SPEC>;
#[doc = "Field `u0_pcie_tx_pattern` reader - u0_pcie_tx_pattern"]
pub type U0_PCIE_TX_PATTERN_R = crate::FieldReader;
#[doc = "Field `u0_pcie_tx_pattern` writer - u0_pcie_tx_pattern"]
pub type U0_PCIE_TX_PATTERN_W<'a, REG> = crate::FieldWriter<'a, REG, 2>;
#[doc = "Field `u0_pcie_usb3_bus_width` reader - u0_pcie_usb3_bus_width"]
pub type U0_PCIE_USB3_BUS_WIDTH_R = crate::FieldReader;
#[doc = "Field `u0_pcie_usb3_bus_width` writer - u0_pcie_usb3_bus_width"]
pub type U0_PCIE_USB3_BUS_WIDTH_W<'a, REG> = crate::FieldWriter<'a, REG, 2>;
#[doc = "Field `u0_pcie_usb3_phy_enable` reader - u0_pcie_usb3_phy_enable"]
pub type U0_PCIE_USB3_PHY_ENABLE_R = crate::BitReader;
#[doc = "Field `u0_pcie_usb3_phy_enable` writer - u0_pcie_usb3_phy_enable"]
pub type U0_PCIE_USB3_PHY_ENABLE_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `u0_pcie_usb3_rate` reader - u0_pcie_usb3_rate"]
pub type U0_PCIE_USB3_RATE_R = crate::FieldReader;
#[doc = "Field `u0_pcie_usb3_rate` writer - u0_pcie_usb3_rate"]
pub type U0_PCIE_USB3_RATE_W<'a, REG> = crate::FieldWriter<'a, REG, 2>;
#[doc = "Field `u0_pcie_usb3_rx_standby` reader - u0_pcie_usb3_rx_standby"]
pub type U0_PCIE_USB3_RX_STANDBY_R = crate::BitReader;
#[doc = "Field `u0_pcie_usb3_rx_standby` writer - u0_pcie_usb3_rx_standby"]
pub type U0_PCIE_USB3_RX_STANDBY_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `u0_pcie_xwdecerr` reader - u0_pcie_xwdecerr"]
pub type U0_PCIE_XWDECERR_R = crate::BitReader;
#[doc = "Field `u0_pcie_xwerrclr` reader - u0_pcie_xwerrclr"]
pub type U0_PCIE_XWERRCLR_R = crate::BitReader;
#[doc = "Field `u0_pcie_xwerrclr` writer - u0_pcie_xwerrclr"]
pub type U0_PCIE_XWERRCLR_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `u0_pcie_xwslverr` reader - u0_pcie_xwslverr"]
pub type U0_PCIE_XWSLVERR_R = crate::BitReader;
#[doc = "Field `u0_sec_top_sramcfg_slp` reader - SRAM/ROM configuration. SLP: sleep enable, high active, default is low."]
pub type U0_SEC_TOP_SRAMCFG_SLP_R = crate::BitReader;
#[doc = "Field `u0_sec_top_sramcfg_slp` writer - SRAM/ROM configuration. SLP: sleep enable, high active, default is low."]
pub type U0_SEC_TOP_SRAMCFG_SLP_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `u0_sec_top_sramcfg_sd` reader - SRAM/ROM configuration. SD: shutdown enable, high active, default is low."]
pub type U0_SEC_TOP_SRAMCFG_SD_R = crate::BitReader;
#[doc = "Field `u0_sec_top_sramcfg_sd` writer - SRAM/ROM configuration. SD: shutdown enable, high active, default is low."]
pub type U0_SEC_TOP_SRAMCFG_SD_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `u0_sec_top_sramcfg_rtsel` reader - SRAM/ROM configuration. RTSEL: timing setting for debug purpose, default is 2'b01."]
pub type U0_SEC_TOP_SRAMCFG_RTSEL_R = crate::FieldReader;
#[doc = "Field `u0_sec_top_sramcfg_rtsel` writer - SRAM/ROM configuration. RTSEL: timing setting for debug purpose, default is 2'b01."]
pub type U0_SEC_TOP_SRAMCFG_RTSEL_W<'a, REG> = crate::FieldWriter<'a, REG, 2>;
#[doc = "Field `u0_sec_top_sramcfg_ptsel` reader - SRAM/ROM configuration. PTSEL: timing setting for debug purpose, default is 2'b01."]
pub type U0_SEC_TOP_SRAMCFG_PTSEL_R = crate::FieldReader;
#[doc = "Field `u0_sec_top_sramcfg_ptsel` writer - SRAM/ROM configuration. PTSEL: timing setting for debug purpose, default is 2'b01."]
pub type U0_SEC_TOP_SRAMCFG_PTSEL_W<'a, REG> = crate::FieldWriter<'a, REG, 2>;
#[doc = "Field `u0_sec_top_sramcfg_trb` reader - SRAM/ROM configuration. TRB: timing setting for debug purpose, default is 2'b01."]
pub type U0_SEC_TOP_SRAMCFG_TRB_R = crate::FieldReader;
#[doc = "Field `u0_sec_top_sramcfg_trb` writer - SRAM/ROM configuration. TRB: timing setting for debug purpose, default is 2'b01."]
pub type U0_SEC_TOP_SRAMCFG_TRB_W<'a, REG> = crate::FieldWriter<'a, REG, 2>;
#[doc = "Field `u0_sec_top_sramcfg_wtsel` reader - SRAM/ROM configuration. WTSEL: timing setting for debug purpose, default is 2'b01."]
pub type U0_SEC_TOP_SRAMCFG_WTSEL_R = crate::FieldReader;
#[doc = "Field `u0_sec_top_sramcfg_wtsel` writer - SRAM/ROM configuration. WTSEL: timing setting for debug purpose, default is 2'b01."]
pub type U0_SEC_TOP_SRAMCFG_WTSEL_W<'a, REG> = crate::FieldWriter<'a, REG, 2>;
#[doc = "Field `u0_sec_top_sramcfg_vs` reader - SRAM/ROM configuration. VS: timing setting for debug purpose, default is 1'b1."]
pub type U0_SEC_TOP_SRAMCFG_VS_R = crate::BitReader;
#[doc = "Field `u0_sec_top_sramcfg_vs` writer - SRAM/ROM configuration. VS: timing setting for debug purpose, default is 1'b1."]
pub type U0_SEC_TOP_SRAMCFG_VS_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `u0_sec_top_sramcfg_vg` reader - SRAM/ROM configuration. VG: timing setting for debug purpose, default is 1'b1."]
pub type U0_SEC_TOP_SRAMCFG_VG_R = crate::BitReader;
#[doc = "Field `u0_sec_top_sramcfg_vg` writer - SRAM/ROM configuration. VG: timing setting for debug purpose, default is 1'b1."]
pub type U0_SEC_TOP_SRAMCFG_VG_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `u0_plda_pcie_align_detect` reader - u0_plda_pcie_align_detect"]
pub type U0_PLDA_PCIE_ALIGN_DETECT_R = crate::BitReader;
impl R {
    #[doc = "Bits 0:1 - u0_pcie_tx_pattern"]
    #[inline(always)]
    pub fn u0_pcie_tx_pattern(&self) -> U0_PCIE_TX_PATTERN_R {
        U0_PCIE_TX_PATTERN_R::new((self.bits & 3) as u8)
    }
    #[doc = "Bits 2:3 - u0_pcie_usb3_bus_width"]
    #[inline(always)]
    pub fn u0_pcie_usb3_bus_width(&self) -> U0_PCIE_USB3_BUS_WIDTH_R {
        U0_PCIE_USB3_BUS_WIDTH_R::new(((self.bits >> 2) & 3) as u8)
    }
    #[doc = "Bit 4 - u0_pcie_usb3_phy_enable"]
    #[inline(always)]
    pub fn u0_pcie_usb3_phy_enable(&self) -> U0_PCIE_USB3_PHY_ENABLE_R {
        U0_PCIE_USB3_PHY_ENABLE_R::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bits 5:6 - u0_pcie_usb3_rate"]
    #[inline(always)]
    pub fn u0_pcie_usb3_rate(&self) -> U0_PCIE_USB3_RATE_R {
        U0_PCIE_USB3_RATE_R::new(((self.bits >> 5) & 3) as u8)
    }
    #[doc = "Bit 7 - u0_pcie_usb3_rx_standby"]
    #[inline(always)]
    pub fn u0_pcie_usb3_rx_standby(&self) -> U0_PCIE_USB3_RX_STANDBY_R {
        U0_PCIE_USB3_RX_STANDBY_R::new(((self.bits >> 7) & 1) != 0)
    }
    #[doc = "Bit 8 - u0_pcie_xwdecerr"]
    #[inline(always)]
    pub fn u0_pcie_xwdecerr(&self) -> U0_PCIE_XWDECERR_R {
        U0_PCIE_XWDECERR_R::new(((self.bits >> 8) & 1) != 0)
    }
    #[doc = "Bit 9 - u0_pcie_xwerrclr"]
    #[inline(always)]
    pub fn u0_pcie_xwerrclr(&self) -> U0_PCIE_XWERRCLR_R {
        U0_PCIE_XWERRCLR_R::new(((self.bits >> 9) & 1) != 0)
    }
    #[doc = "Bit 10 - u0_pcie_xwslverr"]
    #[inline(always)]
    pub fn u0_pcie_xwslverr(&self) -> U0_PCIE_XWSLVERR_R {
        U0_PCIE_XWSLVERR_R::new(((self.bits >> 10) & 1) != 0)
    }
    #[doc = "Bit 11 - SRAM/ROM configuration. SLP: sleep enable, high active, default is low."]
    #[inline(always)]
    pub fn u0_sec_top_sramcfg_slp(&self) -> U0_SEC_TOP_SRAMCFG_SLP_R {
        U0_SEC_TOP_SRAMCFG_SLP_R::new(((self.bits >> 11) & 1) != 0)
    }
    #[doc = "Bit 12 - SRAM/ROM configuration. SD: shutdown enable, high active, default is low."]
    #[inline(always)]
    pub fn u0_sec_top_sramcfg_sd(&self) -> U0_SEC_TOP_SRAMCFG_SD_R {
        U0_SEC_TOP_SRAMCFG_SD_R::new(((self.bits >> 12) & 1) != 0)
    }
    #[doc = "Bits 13:14 - SRAM/ROM configuration. RTSEL: timing setting for debug purpose, default is 2'b01."]
    #[inline(always)]
    pub fn u0_sec_top_sramcfg_rtsel(&self) -> U0_SEC_TOP_SRAMCFG_RTSEL_R {
        U0_SEC_TOP_SRAMCFG_RTSEL_R::new(((self.bits >> 13) & 3) as u8)
    }
    #[doc = "Bits 15:16 - SRAM/ROM configuration. PTSEL: timing setting for debug purpose, default is 2'b01."]
    #[inline(always)]
    pub fn u0_sec_top_sramcfg_ptsel(&self) -> U0_SEC_TOP_SRAMCFG_PTSEL_R {
        U0_SEC_TOP_SRAMCFG_PTSEL_R::new(((self.bits >> 15) & 3) as u8)
    }
    #[doc = "Bits 17:18 - SRAM/ROM configuration. TRB: timing setting for debug purpose, default is 2'b01."]
    #[inline(always)]
    pub fn u0_sec_top_sramcfg_trb(&self) -> U0_SEC_TOP_SRAMCFG_TRB_R {
        U0_SEC_TOP_SRAMCFG_TRB_R::new(((self.bits >> 17) & 3) as u8)
    }
    #[doc = "Bits 19:20 - SRAM/ROM configuration. WTSEL: timing setting for debug purpose, default is 2'b01."]
    #[inline(always)]
    pub fn u0_sec_top_sramcfg_wtsel(&self) -> U0_SEC_TOP_SRAMCFG_WTSEL_R {
        U0_SEC_TOP_SRAMCFG_WTSEL_R::new(((self.bits >> 19) & 3) as u8)
    }
    #[doc = "Bit 21 - SRAM/ROM configuration. VS: timing setting for debug purpose, default is 1'b1."]
    #[inline(always)]
    pub fn u0_sec_top_sramcfg_vs(&self) -> U0_SEC_TOP_SRAMCFG_VS_R {
        U0_SEC_TOP_SRAMCFG_VS_R::new(((self.bits >> 21) & 1) != 0)
    }
    #[doc = "Bit 22 - SRAM/ROM configuration. VG: timing setting for debug purpose, default is 1'b1."]
    #[inline(always)]
    pub fn u0_sec_top_sramcfg_vg(&self) -> U0_SEC_TOP_SRAMCFG_VG_R {
        U0_SEC_TOP_SRAMCFG_VG_R::new(((self.bits >> 22) & 1) != 0)
    }
    #[doc = "Bit 23 - u0_plda_pcie_align_detect"]
    #[inline(always)]
    pub fn u0_plda_pcie_align_detect(&self) -> U0_PLDA_PCIE_ALIGN_DETECT_R {
        U0_PLDA_PCIE_ALIGN_DETECT_R::new(((self.bits >> 23) & 1) != 0)
    }
}
impl W {
    #[doc = "Bits 0:1 - u0_pcie_tx_pattern"]
    #[inline(always)]
    #[must_use]
    pub fn u0_pcie_tx_pattern(&mut self) -> U0_PCIE_TX_PATTERN_W<STG_SYSCFG_125_SPEC> {
        U0_PCIE_TX_PATTERN_W::new(self, 0)
    }
    #[doc = "Bits 2:3 - u0_pcie_usb3_bus_width"]
    #[inline(always)]
    #[must_use]
    pub fn u0_pcie_usb3_bus_width(&mut self) -> U0_PCIE_USB3_BUS_WIDTH_W<STG_SYSCFG_125_SPEC> {
        U0_PCIE_USB3_BUS_WIDTH_W::new(self, 2)
    }
    #[doc = "Bit 4 - u0_pcie_usb3_phy_enable"]
    #[inline(always)]
    #[must_use]
    pub fn u0_pcie_usb3_phy_enable(&mut self) -> U0_PCIE_USB3_PHY_ENABLE_W<STG_SYSCFG_125_SPEC> {
        U0_PCIE_USB3_PHY_ENABLE_W::new(self, 4)
    }
    #[doc = "Bits 5:6 - u0_pcie_usb3_rate"]
    #[inline(always)]
    #[must_use]
    pub fn u0_pcie_usb3_rate(&mut self) -> U0_PCIE_USB3_RATE_W<STG_SYSCFG_125_SPEC> {
        U0_PCIE_USB3_RATE_W::new(self, 5)
    }
    #[doc = "Bit 7 - u0_pcie_usb3_rx_standby"]
    #[inline(always)]
    #[must_use]
    pub fn u0_pcie_usb3_rx_standby(&mut self) -> U0_PCIE_USB3_RX_STANDBY_W<STG_SYSCFG_125_SPEC> {
        U0_PCIE_USB3_RX_STANDBY_W::new(self, 7)
    }
    #[doc = "Bit 9 - u0_pcie_xwerrclr"]
    #[inline(always)]
    #[must_use]
    pub fn u0_pcie_xwerrclr(&mut self) -> U0_PCIE_XWERRCLR_W<STG_SYSCFG_125_SPEC> {
        U0_PCIE_XWERRCLR_W::new(self, 9)
    }
    #[doc = "Bit 11 - SRAM/ROM configuration. SLP: sleep enable, high active, default is low."]
    #[inline(always)]
    #[must_use]
    pub fn u0_sec_top_sramcfg_slp(&mut self) -> U0_SEC_TOP_SRAMCFG_SLP_W<STG_SYSCFG_125_SPEC> {
        U0_SEC_TOP_SRAMCFG_SLP_W::new(self, 11)
    }
    #[doc = "Bit 12 - SRAM/ROM configuration. SD: shutdown enable, high active, default is low."]
    #[inline(always)]
    #[must_use]
    pub fn u0_sec_top_sramcfg_sd(&mut self) -> U0_SEC_TOP_SRAMCFG_SD_W<STG_SYSCFG_125_SPEC> {
        U0_SEC_TOP_SRAMCFG_SD_W::new(self, 12)
    }
    #[doc = "Bits 13:14 - SRAM/ROM configuration. RTSEL: timing setting for debug purpose, default is 2'b01."]
    #[inline(always)]
    #[must_use]
    pub fn u0_sec_top_sramcfg_rtsel(&mut self) -> U0_SEC_TOP_SRAMCFG_RTSEL_W<STG_SYSCFG_125_SPEC> {
        U0_SEC_TOP_SRAMCFG_RTSEL_W::new(self, 13)
    }
    #[doc = "Bits 15:16 - SRAM/ROM configuration. PTSEL: timing setting for debug purpose, default is 2'b01."]
    #[inline(always)]
    #[must_use]
    pub fn u0_sec_top_sramcfg_ptsel(&mut self) -> U0_SEC_TOP_SRAMCFG_PTSEL_W<STG_SYSCFG_125_SPEC> {
        U0_SEC_TOP_SRAMCFG_PTSEL_W::new(self, 15)
    }
    #[doc = "Bits 17:18 - SRAM/ROM configuration. TRB: timing setting for debug purpose, default is 2'b01."]
    #[inline(always)]
    #[must_use]
    pub fn u0_sec_top_sramcfg_trb(&mut self) -> U0_SEC_TOP_SRAMCFG_TRB_W<STG_SYSCFG_125_SPEC> {
        U0_SEC_TOP_SRAMCFG_TRB_W::new(self, 17)
    }
    #[doc = "Bits 19:20 - SRAM/ROM configuration. WTSEL: timing setting for debug purpose, default is 2'b01."]
    #[inline(always)]
    #[must_use]
    pub fn u0_sec_top_sramcfg_wtsel(&mut self) -> U0_SEC_TOP_SRAMCFG_WTSEL_W<STG_SYSCFG_125_SPEC> {
        U0_SEC_TOP_SRAMCFG_WTSEL_W::new(self, 19)
    }
    #[doc = "Bit 21 - SRAM/ROM configuration. VS: timing setting for debug purpose, default is 1'b1."]
    #[inline(always)]
    #[must_use]
    pub fn u0_sec_top_sramcfg_vs(&mut self) -> U0_SEC_TOP_SRAMCFG_VS_W<STG_SYSCFG_125_SPEC> {
        U0_SEC_TOP_SRAMCFG_VS_W::new(self, 21)
    }
    #[doc = "Bit 22 - SRAM/ROM configuration. VG: timing setting for debug purpose, default is 1'b1."]
    #[inline(always)]
    #[must_use]
    pub fn u0_sec_top_sramcfg_vg(&mut self) -> U0_SEC_TOP_SRAMCFG_VG_W<STG_SYSCFG_125_SPEC> {
        U0_SEC_TOP_SRAMCFG_VG_W::new(self, 22)
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
#[doc = "STG SYSCONSAIF SYSCFG 500\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`stg_syscfg_125::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`stg_syscfg_125::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct STG_SYSCFG_125_SPEC;
impl crate::RegisterSpec for STG_SYSCFG_125_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`stg_syscfg_125::R`](R) reader structure"]
impl crate::Readable for STG_SYSCFG_125_SPEC {}
#[doc = "`write(|w| ..)` method takes [`stg_syscfg_125::W`](W) writer structure"]
impl crate::Writable for STG_SYSCFG_125_SPEC {
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets stg_syscfg_125 to value 0"]
impl crate::Resettable for STG_SYSCFG_125_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
