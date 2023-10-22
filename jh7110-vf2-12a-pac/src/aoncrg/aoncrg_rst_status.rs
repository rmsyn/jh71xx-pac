#[doc = "Register `aoncrg_rst_status` reader"]
pub type R = crate::R<AONCRG_RST_STATUS_SPEC>;
#[doc = "Register `aoncrg_rst_status` writer"]
pub type W = crate::W<AONCRG_RST_STATUS_SPEC>;
#[doc = "Field `gmac5_axi64_rstn_axi` reader - 1: Assert reset, 0: De-assert reset"]
pub type GMAC5_AXI64_RSTN_AXI_R = crate::BitReader;
#[doc = "Field `gmac5_axi64_rstn_axi` writer - 1: Assert reset, 0: De-assert reset"]
pub type GMAC5_AXI64_RSTN_AXI_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O>;
#[doc = "Field `gmac5_axi64_rstn_ahb` reader - 1: Assert reset, 0: De-assert reset"]
pub type GMAC5_AXI64_RSTN_AHB_R = crate::BitReader;
#[doc = "Field `gmac5_axi64_rstn_ahb` writer - 1: Assert reset, 0: De-assert reset"]
pub type GMAC5_AXI64_RSTN_AHB_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O>;
#[doc = "Field `aon_iomux_presetn` reader - 1: Assert reset, 0: De-assert reset"]
pub type AON_IOMUX_PRESETN_R = crate::BitReader;
#[doc = "Field `aon_iomux_presetn` writer - 1: Assert reset, 0: De-assert reset"]
pub type AON_IOMUX_PRESETN_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O>;
#[doc = "Field `pmu_rstn_apb` reader - 1: Assert reset, 0: De-assert reset"]
pub type PMU_RSTN_APB_R = crate::BitReader;
#[doc = "Field `pmu_rstn_apb` writer - 1: Assert reset, 0: De-assert reset"]
pub type PMU_RSTN_APB_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O>;
#[doc = "Field `pmu_rstn_wkup` reader - 1: Assert reset, 0: De-assert reset"]
pub type PMU_RSTN_WKUP_R = crate::BitReader;
#[doc = "Field `pmu_rstn_wkup` writer - 1: Assert reset, 0: De-assert reset"]
pub type PMU_RSTN_WKUP_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O>;
#[doc = "Field `rtc_hms_rstn_apb` reader - 1: Assert reset, 0: De-assert reset"]
pub type RTC_HMS_RSTN_APB_R = crate::BitReader;
#[doc = "Field `rtc_hms_rstn_apb` writer - 1: Assert reset, 0: De-assert reset"]
pub type RTC_HMS_RSTN_APB_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O>;
#[doc = "Field `rtc_hms_rstn_cal` reader - 1: Assert reset, 0: De-assert reset"]
pub type RTC_HMS_RSTN_CAL_R = crate::BitReader;
#[doc = "Field `rtc_hms_rstn_cal` writer - 1: Assert reset, 0: De-assert reset"]
pub type RTC_HMS_RSTN_CAL_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O>;
#[doc = "Field `rtc_hms_rstn_osc32k` reader - 1: Assert reset, 0: De-assert reset"]
pub type RTC_HMS_RSTN_OSC32K_R = crate::BitReader;
#[doc = "Field `rtc_hms_rstn_osc32k` writer - 1: Assert reset, 0: De-assert reset"]
pub type RTC_HMS_RSTN_OSC32K_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O>;
impl R {
    #[doc = "Bit 0 - 1: Assert reset, 0: De-assert reset"]
    #[inline(always)]
    pub fn gmac5_axi64_rstn_axi(&self) -> GMAC5_AXI64_RSTN_AXI_R {
        GMAC5_AXI64_RSTN_AXI_R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - 1: Assert reset, 0: De-assert reset"]
    #[inline(always)]
    pub fn gmac5_axi64_rstn_ahb(&self) -> GMAC5_AXI64_RSTN_AHB_R {
        GMAC5_AXI64_RSTN_AHB_R::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - 1: Assert reset, 0: De-assert reset"]
    #[inline(always)]
    pub fn aon_iomux_presetn(&self) -> AON_IOMUX_PRESETN_R {
        AON_IOMUX_PRESETN_R::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3 - 1: Assert reset, 0: De-assert reset"]
    #[inline(always)]
    pub fn pmu_rstn_apb(&self) -> PMU_RSTN_APB_R {
        PMU_RSTN_APB_R::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bit 4 - 1: Assert reset, 0: De-assert reset"]
    #[inline(always)]
    pub fn pmu_rstn_wkup(&self) -> PMU_RSTN_WKUP_R {
        PMU_RSTN_WKUP_R::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bit 5 - 1: Assert reset, 0: De-assert reset"]
    #[inline(always)]
    pub fn rtc_hms_rstn_apb(&self) -> RTC_HMS_RSTN_APB_R {
        RTC_HMS_RSTN_APB_R::new(((self.bits >> 5) & 1) != 0)
    }
    #[doc = "Bit 6 - 1: Assert reset, 0: De-assert reset"]
    #[inline(always)]
    pub fn rtc_hms_rstn_cal(&self) -> RTC_HMS_RSTN_CAL_R {
        RTC_HMS_RSTN_CAL_R::new(((self.bits >> 6) & 1) != 0)
    }
    #[doc = "Bit 7 - 1: Assert reset, 0: De-assert reset"]
    #[inline(always)]
    pub fn rtc_hms_rstn_osc32k(&self) -> RTC_HMS_RSTN_OSC32K_R {
        RTC_HMS_RSTN_OSC32K_R::new(((self.bits >> 7) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - 1: Assert reset, 0: De-assert reset"]
    #[inline(always)]
    #[must_use]
    pub fn gmac5_axi64_rstn_axi(&mut self) -> GMAC5_AXI64_RSTN_AXI_W<AONCRG_RST_STATUS_SPEC, 0> {
        GMAC5_AXI64_RSTN_AXI_W::new(self)
    }
    #[doc = "Bit 1 - 1: Assert reset, 0: De-assert reset"]
    #[inline(always)]
    #[must_use]
    pub fn gmac5_axi64_rstn_ahb(&mut self) -> GMAC5_AXI64_RSTN_AHB_W<AONCRG_RST_STATUS_SPEC, 1> {
        GMAC5_AXI64_RSTN_AHB_W::new(self)
    }
    #[doc = "Bit 2 - 1: Assert reset, 0: De-assert reset"]
    #[inline(always)]
    #[must_use]
    pub fn aon_iomux_presetn(&mut self) -> AON_IOMUX_PRESETN_W<AONCRG_RST_STATUS_SPEC, 2> {
        AON_IOMUX_PRESETN_W::new(self)
    }
    #[doc = "Bit 3 - 1: Assert reset, 0: De-assert reset"]
    #[inline(always)]
    #[must_use]
    pub fn pmu_rstn_apb(&mut self) -> PMU_RSTN_APB_W<AONCRG_RST_STATUS_SPEC, 3> {
        PMU_RSTN_APB_W::new(self)
    }
    #[doc = "Bit 4 - 1: Assert reset, 0: De-assert reset"]
    #[inline(always)]
    #[must_use]
    pub fn pmu_rstn_wkup(&mut self) -> PMU_RSTN_WKUP_W<AONCRG_RST_STATUS_SPEC, 4> {
        PMU_RSTN_WKUP_W::new(self)
    }
    #[doc = "Bit 5 - 1: Assert reset, 0: De-assert reset"]
    #[inline(always)]
    #[must_use]
    pub fn rtc_hms_rstn_apb(&mut self) -> RTC_HMS_RSTN_APB_W<AONCRG_RST_STATUS_SPEC, 5> {
        RTC_HMS_RSTN_APB_W::new(self)
    }
    #[doc = "Bit 6 - 1: Assert reset, 0: De-assert reset"]
    #[inline(always)]
    #[must_use]
    pub fn rtc_hms_rstn_cal(&mut self) -> RTC_HMS_RSTN_CAL_W<AONCRG_RST_STATUS_SPEC, 6> {
        RTC_HMS_RSTN_CAL_W::new(self)
    }
    #[doc = "Bit 7 - 1: Assert reset, 0: De-assert reset"]
    #[inline(always)]
    #[must_use]
    pub fn rtc_hms_rstn_osc32k(&mut self) -> RTC_HMS_RSTN_OSC32K_W<AONCRG_RST_STATUS_SPEC, 7> {
        RTC_HMS_RSTN_OSC32K_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.bits = bits;
        self
    }
}
#[doc = "AONCRG RESET Status\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`aoncrg_rst_status::R`](R).  You can [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`aoncrg_rst_status::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct AONCRG_RST_STATUS_SPEC;
impl crate::RegisterSpec for AONCRG_RST_STATUS_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`aoncrg_rst_status::R`](R) reader structure"]
impl crate::Readable for AONCRG_RST_STATUS_SPEC {}
#[doc = "`write(|w| ..)` method takes [`aoncrg_rst_status::W`](W) writer structure"]
impl crate::Writable for AONCRG_RST_STATUS_SPEC {
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
