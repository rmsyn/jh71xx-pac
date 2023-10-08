#[doc = "Register `soft_turn_on_power_mode` reader"]
pub type R = crate::R<SOFT_TURN_ON_POWER_MODE_SPEC>;
#[doc = "Register `soft_turn_on_power_mode` writer"]
pub type W = crate::W<SOFT_TURN_ON_POWER_MODE_SPEC>;
#[doc = "Field `systop_power_mode` reader - SYSTOP turn-on power mode."]
pub type SYSTOP_POWER_MODE_R = crate::BitReader;
#[doc = "Field `systop_power_mode` writer - SYSTOP turn-on power mode."]
pub type SYSTOP_POWER_MODE_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O>;
#[doc = "Field `cpu_power_mode` reader - CPU turn-on power mode."]
pub type CPU_POWER_MODE_R = crate::BitReader;
#[doc = "Field `cpu_power_mode` writer - CPU turn-on power mode."]
pub type CPU_POWER_MODE_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O>;
#[doc = "Field `gpua_power_mode` reader - GPUA turn-on power mode."]
pub type GPUA_POWER_MODE_R = crate::BitReader;
#[doc = "Field `gpua_power_mode` writer - GPUA turn-on power mode."]
pub type GPUA_POWER_MODE_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O>;
#[doc = "Field `vdec_power_mode` reader - VDEC turn-on power mode."]
pub type VDEC_POWER_MODE_R = crate::BitReader;
#[doc = "Field `vdec_power_mode` writer - VDEC turn-on power mode."]
pub type VDEC_POWER_MODE_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O>;
#[doc = "Field `vout_power_mode` reader - VOUT turn-on power mode."]
pub type VOUT_POWER_MODE_R = crate::BitReader;
#[doc = "Field `vout_power_mode` writer - VOUT turn-on power mode."]
pub type VOUT_POWER_MODE_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O>;
#[doc = "Field `isp_power_mode` reader - ISP turn-on power mode."]
pub type ISP_POWER_MODE_R = crate::BitReader;
#[doc = "Field `isp_power_mode` writer - ISP turn-on power mode."]
pub type ISP_POWER_MODE_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O>;
#[doc = "Field `venc_power_mode` reader - VENC turn-on power mode."]
pub type VENC_POWER_MODE_R = crate::BitReader;
#[doc = "Field `venc_power_mode` writer - VENC turn-on power mode."]
pub type VENC_POWER_MODE_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O>;
impl R {
    #[doc = "Bit 0 - SYSTOP turn-on power mode."]
    #[inline(always)]
    pub fn systop_power_mode(&self) -> SYSTOP_POWER_MODE_R {
        SYSTOP_POWER_MODE_R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - CPU turn-on power mode."]
    #[inline(always)]
    pub fn cpu_power_mode(&self) -> CPU_POWER_MODE_R {
        CPU_POWER_MODE_R::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - GPUA turn-on power mode."]
    #[inline(always)]
    pub fn gpua_power_mode(&self) -> GPUA_POWER_MODE_R {
        GPUA_POWER_MODE_R::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3 - VDEC turn-on power mode."]
    #[inline(always)]
    pub fn vdec_power_mode(&self) -> VDEC_POWER_MODE_R {
        VDEC_POWER_MODE_R::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bit 4 - VOUT turn-on power mode."]
    #[inline(always)]
    pub fn vout_power_mode(&self) -> VOUT_POWER_MODE_R {
        VOUT_POWER_MODE_R::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bit 5 - ISP turn-on power mode."]
    #[inline(always)]
    pub fn isp_power_mode(&self) -> ISP_POWER_MODE_R {
        ISP_POWER_MODE_R::new(((self.bits >> 5) & 1) != 0)
    }
    #[doc = "Bit 6 - VENC turn-on power mode."]
    #[inline(always)]
    pub fn venc_power_mode(&self) -> VENC_POWER_MODE_R {
        VENC_POWER_MODE_R::new(((self.bits >> 6) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - SYSTOP turn-on power mode."]
    #[inline(always)]
    #[must_use]
    pub fn systop_power_mode(&mut self) -> SYSTOP_POWER_MODE_W<SOFT_TURN_ON_POWER_MODE_SPEC, 0> {
        SYSTOP_POWER_MODE_W::new(self)
    }
    #[doc = "Bit 1 - CPU turn-on power mode."]
    #[inline(always)]
    #[must_use]
    pub fn cpu_power_mode(&mut self) -> CPU_POWER_MODE_W<SOFT_TURN_ON_POWER_MODE_SPEC, 1> {
        CPU_POWER_MODE_W::new(self)
    }
    #[doc = "Bit 2 - GPUA turn-on power mode."]
    #[inline(always)]
    #[must_use]
    pub fn gpua_power_mode(&mut self) -> GPUA_POWER_MODE_W<SOFT_TURN_ON_POWER_MODE_SPEC, 2> {
        GPUA_POWER_MODE_W::new(self)
    }
    #[doc = "Bit 3 - VDEC turn-on power mode."]
    #[inline(always)]
    #[must_use]
    pub fn vdec_power_mode(&mut self) -> VDEC_POWER_MODE_W<SOFT_TURN_ON_POWER_MODE_SPEC, 3> {
        VDEC_POWER_MODE_W::new(self)
    }
    #[doc = "Bit 4 - VOUT turn-on power mode."]
    #[inline(always)]
    #[must_use]
    pub fn vout_power_mode(&mut self) -> VOUT_POWER_MODE_W<SOFT_TURN_ON_POWER_MODE_SPEC, 4> {
        VOUT_POWER_MODE_W::new(self)
    }
    #[doc = "Bit 5 - ISP turn-on power mode."]
    #[inline(always)]
    #[must_use]
    pub fn isp_power_mode(&mut self) -> ISP_POWER_MODE_W<SOFT_TURN_ON_POWER_MODE_SPEC, 5> {
        ISP_POWER_MODE_W::new(self)
    }
    #[doc = "Bit 6 - VENC turn-on power mode."]
    #[inline(always)]
    #[must_use]
    pub fn venc_power_mode(&mut self) -> VENC_POWER_MODE_W<SOFT_TURN_ON_POWER_MODE_SPEC, 6> {
        VENC_POWER_MODE_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.bits = bits;
        self
    }
}
#[doc = "Software Turn-On Power Mode\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`soft_turn_on_power_mode::R`](R).  You can [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`soft_turn_on_power_mode::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct SOFT_TURN_ON_POWER_MODE_SPEC;
impl crate::RegisterSpec for SOFT_TURN_ON_POWER_MODE_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`soft_turn_on_power_mode::R`](R) reader structure"]
impl crate::Readable for SOFT_TURN_ON_POWER_MODE_SPEC {}
#[doc = "`write(|w| ..)` method takes [`soft_turn_on_power_mode::W`](W) writer structure"]
impl crate::Writable for SOFT_TURN_ON_POWER_MODE_SPEC {
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
