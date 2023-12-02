#[doc = "Register `stg_sysconsaif_syscfg4` reader"]
pub type R = crate::R<STG_SYSCONSAIF_SYSCFG4_SPEC>;
#[doc = "Register `stg_sysconsaif_syscfg4` writer"]
pub type W = crate::W<STG_SYSCONSAIF_SYSCFG4_SPEC>;
#[doc = "Field `u0_cdn_usb_lowest_belt` reader - LTM interface to software"]
pub type U0_CDN_USB_LOWEST_BELT_R = crate::FieldReader<u16>;
#[doc = "Field `u0_cdn_usb_ltm_host_req` reader - LTM interface to software"]
pub type U0_CDN_USB_LTM_HOST_REQ_R = crate::BitReader;
#[doc = "Field `u0_cdn_usb_ltm_host_req_halt` reader - LTM interface to software"]
pub type U0_CDN_USB_LTM_HOST_REQ_HALT_R = crate::BitReader;
#[doc = "Field `u0_cdn_usb_ltm_host_req_halt` writer - LTM interface to software"]
pub type U0_CDN_USB_LTM_HOST_REQ_HALT_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `u0_cdn_usb_mdctrl_clk_sel` reader - u0_cdn_usb_mdctrl_clk_sel"]
pub type U0_CDN_USB_MDCTRL_CLK_SEL_R = crate::BitReader;
#[doc = "Field `u0_cdn_usb_mdctrl_clk_sel` writer - u0_cdn_usb_mdctrl_clk_sel"]
pub type U0_CDN_USB_MDCTRL_CLK_SEL_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `u0_cdn_usb_mdctrl_clk_status` reader - u0_cdn_usb_mdctrl_clk_status"]
pub type U0_CDN_USB_MDCTRL_CLK_STATUS_R = crate::BitReader;
#[doc = "Field `u0_cdn_usb_mode_strap` reader - Can onlly be changed when pwrup_rst_n is low"]
pub type U0_CDN_USB_MODE_STRAP_R = crate::FieldReader;
#[doc = "Field `u0_cdn_usb_mode_strap` writer - Can onlly be changed when pwrup_rst_n is low"]
pub type U0_CDN_USB_MODE_STRAP_W<'a, REG> = crate::FieldWriter<'a, REG, 3>;
#[doc = "Field `u0_cdn_usb_otg_suspendm` reader - u0_cdn_usb_otg_suspendm"]
pub type U0_CDN_USB_OTG_SUSPENDM_R = crate::BitReader;
#[doc = "Field `u0_cdn_usb_otg_suspendm` writer - u0_cdn_usb_otg_suspendm"]
pub type U0_CDN_USB_OTG_SUSPENDM_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `u0_cdn_usb_otg_suspendm_byps` reader - u0_cdn_usb_otg_suspendm_byps"]
pub type U0_CDN_USB_OTG_SUSPENDM_BYPS_R = crate::BitReader;
#[doc = "Field `u0_cdn_usb_otg_suspendm_byps` writer - u0_cdn_usb_otg_suspendm_byps"]
pub type U0_CDN_USB_OTG_SUSPENDM_BYPS_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `u0_cdn_usb_phy_bvalid` reader - u0_cdn_usb_phy_bvalid"]
pub type U0_CDN_USB_PHY_BVALID_R = crate::BitReader;
#[doc = "Field `u0_cdn_usb_pll_en` reader - u0_cdn_usb_pll_en"]
pub type U0_CDN_USB_PLL_EN_R = crate::BitReader;
#[doc = "Field `u0_cdn_usb_pll_en` writer - u0_cdn_usb_pll_en"]
pub type U0_CDN_USB_PLL_EN_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `u0_cdn_usb_refclk_mode` reader - u0_cdn_usb_refclk_mode"]
pub type U0_CDN_USB_REFCLK_MODE_R = crate::BitReader;
#[doc = "Field `u0_cdn_usb_refclk_mode` writer - u0_cdn_usb_refclk_mode"]
pub type U0_CDN_USB_REFCLK_MODE_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `u0_cdn_usb_rid_a_comp_sts` reader - u0_cdn_usb_rid_a_comp_sts"]
pub type U0_CDN_USB_RID_A_COMP_STS_R = crate::BitReader;
#[doc = "Field `u0_cdn_usb_rid_a_comp_sts` writer - u0_cdn_usb_rid_a_comp_sts"]
pub type U0_CDN_USB_RID_A_COMP_STS_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `u0_cdn_usb_rid_b_comp_sts` reader - u0_cdn_usb_rid_b_comp_sts"]
pub type U0_CDN_USB_RID_B_COMP_STS_R = crate::BitReader;
#[doc = "Field `u0_cdn_usb_rid_b_comp_sts` writer - u0_cdn_usb_rid_b_comp_sts"]
pub type U0_CDN_USB_RID_B_COMP_STS_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `u0_cdn_usb_rid_c_comp_sts` reader - u0_cdn_usb_rid_c_comp_sts"]
pub type U0_CDN_USB_RID_C_COMP_STS_R = crate::BitReader;
#[doc = "Field `u0_cdn_usb_rid_c_comp_sts` writer - u0_cdn_usb_rid_c_comp_sts"]
pub type U0_CDN_USB_RID_C_COMP_STS_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `u0_cdn_usb_rid_float_comp_en` reader - u0_cdn_usb_rid_float_comp_en"]
pub type U0_CDN_USB_RID_FLOAT_COMP_EN_R = crate::BitReader;
#[doc = "Field `u0_cdn_usb_rid_float_comp_sts` reader - u0_cdn_usb_rid_float_comp_sts"]
pub type U0_CDN_USB_RID_FLOAT_COMP_STS_R = crate::BitReader;
#[doc = "Field `u0_cdn_usb_rid_float_comp_sts` writer - u0_cdn_usb_rid_float_comp_sts"]
pub type U0_CDN_USB_RID_FLOAT_COMP_STS_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `u0_cdn_usb_rid_gnd_comp_sts` reader - u0_cdn_usb_rid_gnd_comp_sts"]
pub type U0_CDN_USB_RID_GND_COMP_STS_R = crate::BitReader;
#[doc = "Field `u0_cdn_usb_rid_gnd_comp_sts` writer - u0_cdn_usb_rid_gnd_comp_sts"]
pub type U0_CDN_USB_RID_GND_COMP_STS_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `u0_cdn_usb_rid_nonfloat_comp_en` reader - u0_cdn_usb_rid_nonfloat_comp_en"]
pub type U0_CDN_USB_RID_NONFLOAT_COMP_EN_R = crate::BitReader;
#[doc = "Field `u0_cdn_usb_rx_dm` reader - u0_cdn_usb_rx_dm"]
pub type U0_CDN_USB_RX_DM_R = crate::BitReader;
impl R {
    #[doc = "Bits 0:11 - LTM interface to software"]
    #[inline(always)]
    pub fn u0_cdn_usb_lowest_belt(&self) -> U0_CDN_USB_LOWEST_BELT_R {
        U0_CDN_USB_LOWEST_BELT_R::new((self.bits & 0x0fff) as u16)
    }
    #[doc = "Bit 12 - LTM interface to software"]
    #[inline(always)]
    pub fn u0_cdn_usb_ltm_host_req(&self) -> U0_CDN_USB_LTM_HOST_REQ_R {
        U0_CDN_USB_LTM_HOST_REQ_R::new(((self.bits >> 12) & 1) != 0)
    }
    #[doc = "Bit 13 - LTM interface to software"]
    #[inline(always)]
    pub fn u0_cdn_usb_ltm_host_req_halt(&self) -> U0_CDN_USB_LTM_HOST_REQ_HALT_R {
        U0_CDN_USB_LTM_HOST_REQ_HALT_R::new(((self.bits >> 13) & 1) != 0)
    }
    #[doc = "Bit 14 - u0_cdn_usb_mdctrl_clk_sel"]
    #[inline(always)]
    pub fn u0_cdn_usb_mdctrl_clk_sel(&self) -> U0_CDN_USB_MDCTRL_CLK_SEL_R {
        U0_CDN_USB_MDCTRL_CLK_SEL_R::new(((self.bits >> 14) & 1) != 0)
    }
    #[doc = "Bit 15 - u0_cdn_usb_mdctrl_clk_status"]
    #[inline(always)]
    pub fn u0_cdn_usb_mdctrl_clk_status(&self) -> U0_CDN_USB_MDCTRL_CLK_STATUS_R {
        U0_CDN_USB_MDCTRL_CLK_STATUS_R::new(((self.bits >> 15) & 1) != 0)
    }
    #[doc = "Bits 16:18 - Can onlly be changed when pwrup_rst_n is low"]
    #[inline(always)]
    pub fn u0_cdn_usb_mode_strap(&self) -> U0_CDN_USB_MODE_STRAP_R {
        U0_CDN_USB_MODE_STRAP_R::new(((self.bits >> 16) & 7) as u8)
    }
    #[doc = "Bit 19 - u0_cdn_usb_otg_suspendm"]
    #[inline(always)]
    pub fn u0_cdn_usb_otg_suspendm(&self) -> U0_CDN_USB_OTG_SUSPENDM_R {
        U0_CDN_USB_OTG_SUSPENDM_R::new(((self.bits >> 19) & 1) != 0)
    }
    #[doc = "Bit 20 - u0_cdn_usb_otg_suspendm_byps"]
    #[inline(always)]
    pub fn u0_cdn_usb_otg_suspendm_byps(&self) -> U0_CDN_USB_OTG_SUSPENDM_BYPS_R {
        U0_CDN_USB_OTG_SUSPENDM_BYPS_R::new(((self.bits >> 20) & 1) != 0)
    }
    #[doc = "Bit 21 - u0_cdn_usb_phy_bvalid"]
    #[inline(always)]
    pub fn u0_cdn_usb_phy_bvalid(&self) -> U0_CDN_USB_PHY_BVALID_R {
        U0_CDN_USB_PHY_BVALID_R::new(((self.bits >> 21) & 1) != 0)
    }
    #[doc = "Bit 22 - u0_cdn_usb_pll_en"]
    #[inline(always)]
    pub fn u0_cdn_usb_pll_en(&self) -> U0_CDN_USB_PLL_EN_R {
        U0_CDN_USB_PLL_EN_R::new(((self.bits >> 22) & 1) != 0)
    }
    #[doc = "Bit 23 - u0_cdn_usb_refclk_mode"]
    #[inline(always)]
    pub fn u0_cdn_usb_refclk_mode(&self) -> U0_CDN_USB_REFCLK_MODE_R {
        U0_CDN_USB_REFCLK_MODE_R::new(((self.bits >> 23) & 1) != 0)
    }
    #[doc = "Bit 24 - u0_cdn_usb_rid_a_comp_sts"]
    #[inline(always)]
    pub fn u0_cdn_usb_rid_a_comp_sts(&self) -> U0_CDN_USB_RID_A_COMP_STS_R {
        U0_CDN_USB_RID_A_COMP_STS_R::new(((self.bits >> 24) & 1) != 0)
    }
    #[doc = "Bit 25 - u0_cdn_usb_rid_b_comp_sts"]
    #[inline(always)]
    pub fn u0_cdn_usb_rid_b_comp_sts(&self) -> U0_CDN_USB_RID_B_COMP_STS_R {
        U0_CDN_USB_RID_B_COMP_STS_R::new(((self.bits >> 25) & 1) != 0)
    }
    #[doc = "Bit 26 - u0_cdn_usb_rid_c_comp_sts"]
    #[inline(always)]
    pub fn u0_cdn_usb_rid_c_comp_sts(&self) -> U0_CDN_USB_RID_C_COMP_STS_R {
        U0_CDN_USB_RID_C_COMP_STS_R::new(((self.bits >> 26) & 1) != 0)
    }
    #[doc = "Bit 27 - u0_cdn_usb_rid_float_comp_en"]
    #[inline(always)]
    pub fn u0_cdn_usb_rid_float_comp_en(&self) -> U0_CDN_USB_RID_FLOAT_COMP_EN_R {
        U0_CDN_USB_RID_FLOAT_COMP_EN_R::new(((self.bits >> 27) & 1) != 0)
    }
    #[doc = "Bit 28 - u0_cdn_usb_rid_float_comp_sts"]
    #[inline(always)]
    pub fn u0_cdn_usb_rid_float_comp_sts(&self) -> U0_CDN_USB_RID_FLOAT_COMP_STS_R {
        U0_CDN_USB_RID_FLOAT_COMP_STS_R::new(((self.bits >> 28) & 1) != 0)
    }
    #[doc = "Bit 29 - u0_cdn_usb_rid_gnd_comp_sts"]
    #[inline(always)]
    pub fn u0_cdn_usb_rid_gnd_comp_sts(&self) -> U0_CDN_USB_RID_GND_COMP_STS_R {
        U0_CDN_USB_RID_GND_COMP_STS_R::new(((self.bits >> 29) & 1) != 0)
    }
    #[doc = "Bit 30 - u0_cdn_usb_rid_nonfloat_comp_en"]
    #[inline(always)]
    pub fn u0_cdn_usb_rid_nonfloat_comp_en(&self) -> U0_CDN_USB_RID_NONFLOAT_COMP_EN_R {
        U0_CDN_USB_RID_NONFLOAT_COMP_EN_R::new(((self.bits >> 30) & 1) != 0)
    }
    #[doc = "Bit 31 - u0_cdn_usb_rx_dm"]
    #[inline(always)]
    pub fn u0_cdn_usb_rx_dm(&self) -> U0_CDN_USB_RX_DM_R {
        U0_CDN_USB_RX_DM_R::new(((self.bits >> 31) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 13 - LTM interface to software"]
    #[inline(always)]
    #[must_use]
    pub fn u0_cdn_usb_ltm_host_req_halt(
        &mut self,
    ) -> U0_CDN_USB_LTM_HOST_REQ_HALT_W<STG_SYSCONSAIF_SYSCFG4_SPEC> {
        U0_CDN_USB_LTM_HOST_REQ_HALT_W::new(self, 13)
    }
    #[doc = "Bit 14 - u0_cdn_usb_mdctrl_clk_sel"]
    #[inline(always)]
    #[must_use]
    pub fn u0_cdn_usb_mdctrl_clk_sel(
        &mut self,
    ) -> U0_CDN_USB_MDCTRL_CLK_SEL_W<STG_SYSCONSAIF_SYSCFG4_SPEC> {
        U0_CDN_USB_MDCTRL_CLK_SEL_W::new(self, 14)
    }
    #[doc = "Bits 16:18 - Can onlly be changed when pwrup_rst_n is low"]
    #[inline(always)]
    #[must_use]
    pub fn u0_cdn_usb_mode_strap(
        &mut self,
    ) -> U0_CDN_USB_MODE_STRAP_W<STG_SYSCONSAIF_SYSCFG4_SPEC> {
        U0_CDN_USB_MODE_STRAP_W::new(self, 16)
    }
    #[doc = "Bit 19 - u0_cdn_usb_otg_suspendm"]
    #[inline(always)]
    #[must_use]
    pub fn u0_cdn_usb_otg_suspendm(
        &mut self,
    ) -> U0_CDN_USB_OTG_SUSPENDM_W<STG_SYSCONSAIF_SYSCFG4_SPEC> {
        U0_CDN_USB_OTG_SUSPENDM_W::new(self, 19)
    }
    #[doc = "Bit 20 - u0_cdn_usb_otg_suspendm_byps"]
    #[inline(always)]
    #[must_use]
    pub fn u0_cdn_usb_otg_suspendm_byps(
        &mut self,
    ) -> U0_CDN_USB_OTG_SUSPENDM_BYPS_W<STG_SYSCONSAIF_SYSCFG4_SPEC> {
        U0_CDN_USB_OTG_SUSPENDM_BYPS_W::new(self, 20)
    }
    #[doc = "Bit 22 - u0_cdn_usb_pll_en"]
    #[inline(always)]
    #[must_use]
    pub fn u0_cdn_usb_pll_en(&mut self) -> U0_CDN_USB_PLL_EN_W<STG_SYSCONSAIF_SYSCFG4_SPEC> {
        U0_CDN_USB_PLL_EN_W::new(self, 22)
    }
    #[doc = "Bit 23 - u0_cdn_usb_refclk_mode"]
    #[inline(always)]
    #[must_use]
    pub fn u0_cdn_usb_refclk_mode(
        &mut self,
    ) -> U0_CDN_USB_REFCLK_MODE_W<STG_SYSCONSAIF_SYSCFG4_SPEC> {
        U0_CDN_USB_REFCLK_MODE_W::new(self, 23)
    }
    #[doc = "Bit 24 - u0_cdn_usb_rid_a_comp_sts"]
    #[inline(always)]
    #[must_use]
    pub fn u0_cdn_usb_rid_a_comp_sts(
        &mut self,
    ) -> U0_CDN_USB_RID_A_COMP_STS_W<STG_SYSCONSAIF_SYSCFG4_SPEC> {
        U0_CDN_USB_RID_A_COMP_STS_W::new(self, 24)
    }
    #[doc = "Bit 25 - u0_cdn_usb_rid_b_comp_sts"]
    #[inline(always)]
    #[must_use]
    pub fn u0_cdn_usb_rid_b_comp_sts(
        &mut self,
    ) -> U0_CDN_USB_RID_B_COMP_STS_W<STG_SYSCONSAIF_SYSCFG4_SPEC> {
        U0_CDN_USB_RID_B_COMP_STS_W::new(self, 25)
    }
    #[doc = "Bit 26 - u0_cdn_usb_rid_c_comp_sts"]
    #[inline(always)]
    #[must_use]
    pub fn u0_cdn_usb_rid_c_comp_sts(
        &mut self,
    ) -> U0_CDN_USB_RID_C_COMP_STS_W<STG_SYSCONSAIF_SYSCFG4_SPEC> {
        U0_CDN_USB_RID_C_COMP_STS_W::new(self, 26)
    }
    #[doc = "Bit 28 - u0_cdn_usb_rid_float_comp_sts"]
    #[inline(always)]
    #[must_use]
    pub fn u0_cdn_usb_rid_float_comp_sts(
        &mut self,
    ) -> U0_CDN_USB_RID_FLOAT_COMP_STS_W<STG_SYSCONSAIF_SYSCFG4_SPEC> {
        U0_CDN_USB_RID_FLOAT_COMP_STS_W::new(self, 28)
    }
    #[doc = "Bit 29 - u0_cdn_usb_rid_gnd_comp_sts"]
    #[inline(always)]
    #[must_use]
    pub fn u0_cdn_usb_rid_gnd_comp_sts(
        &mut self,
    ) -> U0_CDN_USB_RID_GND_COMP_STS_W<STG_SYSCONSAIF_SYSCFG4_SPEC> {
        U0_CDN_USB_RID_GND_COMP_STS_W::new(self, 29)
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
#[doc = "STG SYSCONSAIF SYSCFG 4\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`stg_sysconsaif_syscfg4::R`](R).  You can [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`stg_sysconsaif_syscfg4::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct STG_SYSCONSAIF_SYSCFG4_SPEC;
impl crate::RegisterSpec for STG_SYSCONSAIF_SYSCFG4_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`stg_sysconsaif_syscfg4::R`](R) reader structure"]
impl crate::Readable for STG_SYSCONSAIF_SYSCFG4_SPEC {}
#[doc = "`write(|w| ..)` method takes [`stg_sysconsaif_syscfg4::W`](W) writer structure"]
impl crate::Writable for STG_SYSCONSAIF_SYSCFG4_SPEC {
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
