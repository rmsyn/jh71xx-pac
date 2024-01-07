#[doc = "Register `soft_rst_addr_sel` reader"]
pub type R = crate::R<SOFT_RST_ADDR_SEL_SPEC>;
#[doc = "Register `soft_rst_addr_sel` writer"]
pub type W = crate::W<SOFT_RST_ADDR_SEL_SPEC>;
#[doc = "Field `gmac5_axi64_axi` reader - 1: Assert reset, 0: De-assert reset"]
pub type GMAC5_AXI64_AXI_R = crate::BitReader;
#[doc = "Field `gmac5_axi64_axi` writer - 1: Assert reset, 0: De-assert reset"]
pub type GMAC5_AXI64_AXI_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `gmac5_axi64_ahb` reader - 1: Assert reset, 0: De-assert reset"]
pub type GMAC5_AXI64_AHB_R = crate::BitReader;
#[doc = "Field `gmac5_axi64_ahb` writer - 1: Assert reset, 0: De-assert reset"]
pub type GMAC5_AXI64_AHB_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `aon_iomux_presetn` reader - 1: Assert reset, 0: De-assert reset"]
pub type AON_IOMUX_PRESETN_R = crate::BitReader;
#[doc = "Field `aon_iomux_presetn` writer - 1: Assert reset, 0: De-assert reset"]
pub type AON_IOMUX_PRESETN_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `pmu_apb` reader - 1: Assert reset, 0: De-assert reset"]
pub type PMU_APB_R = crate::BitReader;
#[doc = "Field `pmu_apb` writer - 1: Assert reset, 0: De-assert reset"]
pub type PMU_APB_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `pmu_wkup` reader - 1: Assert reset, 0: De-assert reset"]
pub type PMU_WKUP_R = crate::BitReader;
#[doc = "Field `pmu_wkup` writer - 1: Assert reset, 0: De-assert reset"]
pub type PMU_WKUP_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `rtc_hms_apb` reader - 1: Assert reset, 0: De-assert reset"]
pub type RTC_HMS_APB_R = crate::BitReader;
#[doc = "Field `rtc_hms_apb` writer - 1: Assert reset, 0: De-assert reset"]
pub type RTC_HMS_APB_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `rtc_hms_cal` reader - 1: Assert reset, 0: De-assert reset"]
pub type RTC_HMS_CAL_R = crate::BitReader;
#[doc = "Field `rtc_hms_cal` writer - 1: Assert reset, 0: De-assert reset"]
pub type RTC_HMS_CAL_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `rtc_hms_osc32k` reader - 1: Assert reset, 0: De-assert reset"]
pub type RTC_HMS_OSC32K_R = crate::BitReader;
#[doc = "Field `rtc_hms_osc32k` writer - 1: Assert reset, 0: De-assert reset"]
pub type RTC_HMS_OSC32K_W<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bit 0 - 1: Assert reset, 0: De-assert reset"]
    #[inline(always)]
    pub fn gmac5_axi64_axi(&self) -> GMAC5_AXI64_AXI_R {
        GMAC5_AXI64_AXI_R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - 1: Assert reset, 0: De-assert reset"]
    #[inline(always)]
    pub fn gmac5_axi64_ahb(&self) -> GMAC5_AXI64_AHB_R {
        GMAC5_AXI64_AHB_R::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - 1: Assert reset, 0: De-assert reset"]
    #[inline(always)]
    pub fn aon_iomux_presetn(&self) -> AON_IOMUX_PRESETN_R {
        AON_IOMUX_PRESETN_R::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3 - 1: Assert reset, 0: De-assert reset"]
    #[inline(always)]
    pub fn pmu_apb(&self) -> PMU_APB_R {
        PMU_APB_R::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bit 4 - 1: Assert reset, 0: De-assert reset"]
    #[inline(always)]
    pub fn pmu_wkup(&self) -> PMU_WKUP_R {
        PMU_WKUP_R::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bit 5 - 1: Assert reset, 0: De-assert reset"]
    #[inline(always)]
    pub fn rtc_hms_apb(&self) -> RTC_HMS_APB_R {
        RTC_HMS_APB_R::new(((self.bits >> 5) & 1) != 0)
    }
    #[doc = "Bit 6 - 1: Assert reset, 0: De-assert reset"]
    #[inline(always)]
    pub fn rtc_hms_cal(&self) -> RTC_HMS_CAL_R {
        RTC_HMS_CAL_R::new(((self.bits >> 6) & 1) != 0)
    }
    #[doc = "Bit 7 - 1: Assert reset, 0: De-assert reset"]
    #[inline(always)]
    pub fn rtc_hms_osc32k(&self) -> RTC_HMS_OSC32K_R {
        RTC_HMS_OSC32K_R::new(((self.bits >> 7) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - 1: Assert reset, 0: De-assert reset"]
    #[inline(always)]
    #[must_use]
    pub fn gmac5_axi64_axi(&mut self) -> GMAC5_AXI64_AXI_W<SOFT_RST_ADDR_SEL_SPEC> {
        GMAC5_AXI64_AXI_W::new(self, 0)
    }
    #[doc = "Bit 1 - 1: Assert reset, 0: De-assert reset"]
    #[inline(always)]
    #[must_use]
    pub fn gmac5_axi64_ahb(&mut self) -> GMAC5_AXI64_AHB_W<SOFT_RST_ADDR_SEL_SPEC> {
        GMAC5_AXI64_AHB_W::new(self, 1)
    }
    #[doc = "Bit 2 - 1: Assert reset, 0: De-assert reset"]
    #[inline(always)]
    #[must_use]
    pub fn aon_iomux_presetn(&mut self) -> AON_IOMUX_PRESETN_W<SOFT_RST_ADDR_SEL_SPEC> {
        AON_IOMUX_PRESETN_W::new(self, 2)
    }
    #[doc = "Bit 3 - 1: Assert reset, 0: De-assert reset"]
    #[inline(always)]
    #[must_use]
    pub fn pmu_apb(&mut self) -> PMU_APB_W<SOFT_RST_ADDR_SEL_SPEC> {
        PMU_APB_W::new(self, 3)
    }
    #[doc = "Bit 4 - 1: Assert reset, 0: De-assert reset"]
    #[inline(always)]
    #[must_use]
    pub fn pmu_wkup(&mut self) -> PMU_WKUP_W<SOFT_RST_ADDR_SEL_SPEC> {
        PMU_WKUP_W::new(self, 4)
    }
    #[doc = "Bit 5 - 1: Assert reset, 0: De-assert reset"]
    #[inline(always)]
    #[must_use]
    pub fn rtc_hms_apb(&mut self) -> RTC_HMS_APB_W<SOFT_RST_ADDR_SEL_SPEC> {
        RTC_HMS_APB_W::new(self, 5)
    }
    #[doc = "Bit 6 - 1: Assert reset, 0: De-assert reset"]
    #[inline(always)]
    #[must_use]
    pub fn rtc_hms_cal(&mut self) -> RTC_HMS_CAL_W<SOFT_RST_ADDR_SEL_SPEC> {
        RTC_HMS_CAL_W::new(self, 6)
    }
    #[doc = "Bit 7 - 1: Assert reset, 0: De-assert reset"]
    #[inline(always)]
    #[must_use]
    pub fn rtc_hms_osc32k(&mut self) -> RTC_HMS_OSC32K_W<SOFT_RST_ADDR_SEL_SPEC> {
        RTC_HMS_OSC32K_W::new(self, 7)
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
#[doc = "Software RESET Address Selector\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`soft_rst_addr_sel::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`soft_rst_addr_sel::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
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
#[doc = "`reset()` method sets soft_rst_addr_sel to value 0"]
impl crate::Resettable for SOFT_RST_ADDR_SEL_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
