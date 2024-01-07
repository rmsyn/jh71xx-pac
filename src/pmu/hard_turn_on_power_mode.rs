#[doc = "Register `hard_turn_on_power_mode` reader"]
pub type R = crate::R<HARD_TURN_ON_POWER_MODE_SPEC>;
#[doc = "Register `hard_turn_on_power_mode` writer"]
pub type W = crate::W<HARD_TURN_ON_POWER_MODE_SPEC>;
#[doc = "Field `systop_power_mode` reader - SYSTOP turn-on power mode."]
pub type SYSTOP_POWER_MODE_R = crate::BitReader;
#[doc = "Field `systop_power_mode` writer - SYSTOP turn-on power mode."]
pub type SYSTOP_POWER_MODE_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `cpu_power_mode` reader - CPU turn-on power mode."]
pub type CPU_POWER_MODE_R = crate::BitReader;
#[doc = "Field `cpu_power_mode` writer - CPU turn-on power mode."]
pub type CPU_POWER_MODE_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `gpua_power_mode` reader - GPUA turn-on power mode."]
pub type GPUA_POWER_MODE_R = crate::BitReader;
#[doc = "Field `gpua_power_mode` writer - GPUA turn-on power mode."]
pub type GPUA_POWER_MODE_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `vdec_power_mode` reader - VDEC turn-on power mode."]
pub type VDEC_POWER_MODE_R = crate::BitReader;
#[doc = "Field `vdec_power_mode` writer - VDEC turn-on power mode."]
pub type VDEC_POWER_MODE_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `vout_power_mode` reader - VOUT turn-on power mode."]
pub type VOUT_POWER_MODE_R = crate::BitReader;
#[doc = "Field `vout_power_mode` writer - VOUT turn-on power mode."]
pub type VOUT_POWER_MODE_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `isp_power_mode` reader - ISP turn-on power mode."]
pub type ISP_POWER_MODE_R = crate::BitReader;
#[doc = "Field `isp_power_mode` writer - ISP turn-on power mode."]
pub type ISP_POWER_MODE_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `venc_power_mode` reader - VENC turn-on power mode."]
pub type VENC_POWER_MODE_R = crate::BitReader;
#[doc = "Field `venc_power_mode` writer - VENC turn-on power mode."]
pub type VENC_POWER_MODE_W<'a, REG> = crate::BitWriter<'a, REG>;
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
    pub fn systop_power_mode(&mut self) -> SYSTOP_POWER_MODE_W<HARD_TURN_ON_POWER_MODE_SPEC> {
        SYSTOP_POWER_MODE_W::new(self, 0)
    }
    #[doc = "Bit 1 - CPU turn-on power mode."]
    #[inline(always)]
    #[must_use]
    pub fn cpu_power_mode(&mut self) -> CPU_POWER_MODE_W<HARD_TURN_ON_POWER_MODE_SPEC> {
        CPU_POWER_MODE_W::new(self, 1)
    }
    #[doc = "Bit 2 - GPUA turn-on power mode."]
    #[inline(always)]
    #[must_use]
    pub fn gpua_power_mode(&mut self) -> GPUA_POWER_MODE_W<HARD_TURN_ON_POWER_MODE_SPEC> {
        GPUA_POWER_MODE_W::new(self, 2)
    }
    #[doc = "Bit 3 - VDEC turn-on power mode."]
    #[inline(always)]
    #[must_use]
    pub fn vdec_power_mode(&mut self) -> VDEC_POWER_MODE_W<HARD_TURN_ON_POWER_MODE_SPEC> {
        VDEC_POWER_MODE_W::new(self, 3)
    }
    #[doc = "Bit 4 - VOUT turn-on power mode."]
    #[inline(always)]
    #[must_use]
    pub fn vout_power_mode(&mut self) -> VOUT_POWER_MODE_W<HARD_TURN_ON_POWER_MODE_SPEC> {
        VOUT_POWER_MODE_W::new(self, 4)
    }
    #[doc = "Bit 5 - ISP turn-on power mode."]
    #[inline(always)]
    #[must_use]
    pub fn isp_power_mode(&mut self) -> ISP_POWER_MODE_W<HARD_TURN_ON_POWER_MODE_SPEC> {
        ISP_POWER_MODE_W::new(self, 5)
    }
    #[doc = "Bit 6 - VENC turn-on power mode."]
    #[inline(always)]
    #[must_use]
    pub fn venc_power_mode(&mut self) -> VENC_POWER_MODE_W<HARD_TURN_ON_POWER_MODE_SPEC> {
        VENC_POWER_MODE_W::new(self, 6)
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
#[doc = "Hardware Turn-On Power Mode\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`hard_turn_on_power_mode::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`hard_turn_on_power_mode::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct HARD_TURN_ON_POWER_MODE_SPEC;
impl crate::RegisterSpec for HARD_TURN_ON_POWER_MODE_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`hard_turn_on_power_mode::R`](R) reader structure"]
impl crate::Readable for HARD_TURN_ON_POWER_MODE_SPEC {}
#[doc = "`write(|w| ..)` method takes [`hard_turn_on_power_mode::W`](W) writer structure"]
impl crate::Writable for HARD_TURN_ON_POWER_MODE_SPEC {
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets hard_turn_on_power_mode to value 0"]
impl crate::Resettable for HARD_TURN_ON_POWER_MODE_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
