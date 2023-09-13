#[doc = "Register `u7mc_rtc_toggle` reader"]
pub type R = crate::R<U7MC_RTC_TOGGLE_SPEC>;
#[doc = "Register `u7mc_rtc_toggle` writer"]
pub type W = crate::W<U7MC_RTC_TOGGLE_SPEC>;
#[doc = "Field `clk_divcfg` reader - Clock divider coefficient: Max=6, Default=6, Min=6, Typical=6"]
pub type CLK_DIVCFG_R = crate::FieldReader<u32>;
#[doc = "Field `clk_divcfg` writer - Clock divider coefficient: Max=6, Default=6, Min=6, Typical=6"]
pub type CLK_DIVCFG_W<'a, REG, const O: u8> = crate::FieldWriter<'a, REG, 24, O, u32>;
impl R {
    #[doc = "Bits 0:23 - Clock divider coefficient: Max=6, Default=6, Min=6, Typical=6"]
    #[inline(always)]
    pub fn clk_divcfg(&self) -> CLK_DIVCFG_R {
        CLK_DIVCFG_R::new(self.bits & 0x00ff_ffff)
    }
}
impl W {
    #[doc = "Bits 0:23 - Clock divider coefficient: Max=6, Default=6, Min=6, Typical=6"]
    #[inline(always)]
    #[must_use]
    pub fn clk_divcfg(&mut self) -> CLK_DIVCFG_W<U7MC_RTC_TOGGLE_SPEC, 0> {
        CLK_DIVCFG_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.bits = bits;
        self
    }
}
#[doc = "U7MC RTC Toggle\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`u7mc_rtc_toggle::R`](R).  You can [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`u7mc_rtc_toggle::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct U7MC_RTC_TOGGLE_SPEC;
impl crate::RegisterSpec for U7MC_RTC_TOGGLE_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`u7mc_rtc_toggle::R`](R) reader structure"]
impl crate::Readable for U7MC_RTC_TOGGLE_SPEC {}
#[doc = "`write(|w| ..)` method takes [`u7mc_rtc_toggle::W`](W) writer structure"]
impl crate::Writable for U7MC_RTC_TOGGLE_SPEC {
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
