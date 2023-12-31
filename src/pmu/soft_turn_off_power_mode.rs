#[doc = "Register `soft_turn_off_power_mode` reader"]
pub type R = crate::R<SOFT_TURN_OFF_POWER_MODE_SPEC>;
#[doc = "Register `soft_turn_off_power_mode` writer"]
pub type W = crate::W<SOFT_TURN_OFF_POWER_MODE_SPEC>;
#[doc = "Field `systop_power_mode` reader - SYSTOP turn-off power mode."]
pub type SYSTOP_POWER_MODE_R = crate::BitReader;
#[doc = "Field `systop_power_mode` writer - SYSTOP turn-off power mode."]
pub type SYSTOP_POWER_MODE_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `cpu_power_mode` reader - CPU turn-off power mode."]
pub type CPU_POWER_MODE_R = crate::BitReader;
#[doc = "Field `cpu_power_mode` writer - CPU turn-off power mode."]
pub type CPU_POWER_MODE_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `gpua_power_mode` reader - GPUA turn-off power mode."]
pub type GPUA_POWER_MODE_R = crate::BitReader;
#[doc = "Field `gpua_power_mode` writer - GPUA turn-off power mode."]
pub type GPUA_POWER_MODE_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `vdec_power_mode` reader - VDEC turn-off power mode."]
pub type VDEC_POWER_MODE_R = crate::BitReader;
#[doc = "Field `vdec_power_mode` writer - VDEC turn-off power mode."]
pub type VDEC_POWER_MODE_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `vout_power_mode` reader - VOUT turn-off power mode."]
pub type VOUT_POWER_MODE_R = crate::BitReader;
#[doc = "Field `vout_power_mode` writer - VOUT turn-off power mode."]
pub type VOUT_POWER_MODE_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `isp_power_mode` reader - ISP turn-off power mode."]
pub type ISP_POWER_MODE_R = crate::BitReader;
#[doc = "Field `isp_power_mode` writer - ISP turn-off power mode."]
pub type ISP_POWER_MODE_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `venc_power_mode` reader - VENC turn-off power mode."]
pub type VENC_POWER_MODE_R = crate::BitReader;
#[doc = "Field `venc_power_mode` writer - VENC turn-off power mode."]
pub type VENC_POWER_MODE_W<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bit 0 - SYSTOP turn-off power mode."]
    #[inline(always)]
    pub fn systop_power_mode(&self) -> SYSTOP_POWER_MODE_R {
        SYSTOP_POWER_MODE_R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - CPU turn-off power mode."]
    #[inline(always)]
    pub fn cpu_power_mode(&self) -> CPU_POWER_MODE_R {
        CPU_POWER_MODE_R::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - GPUA turn-off power mode."]
    #[inline(always)]
    pub fn gpua_power_mode(&self) -> GPUA_POWER_MODE_R {
        GPUA_POWER_MODE_R::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3 - VDEC turn-off power mode."]
    #[inline(always)]
    pub fn vdec_power_mode(&self) -> VDEC_POWER_MODE_R {
        VDEC_POWER_MODE_R::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bit 4 - VOUT turn-off power mode."]
    #[inline(always)]
    pub fn vout_power_mode(&self) -> VOUT_POWER_MODE_R {
        VOUT_POWER_MODE_R::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bit 5 - ISP turn-off power mode."]
    #[inline(always)]
    pub fn isp_power_mode(&self) -> ISP_POWER_MODE_R {
        ISP_POWER_MODE_R::new(((self.bits >> 5) & 1) != 0)
    }
    #[doc = "Bit 6 - VENC turn-off power mode."]
    #[inline(always)]
    pub fn venc_power_mode(&self) -> VENC_POWER_MODE_R {
        VENC_POWER_MODE_R::new(((self.bits >> 6) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - SYSTOP turn-off power mode."]
    #[inline(always)]
    #[must_use]
    pub fn systop_power_mode(&mut self) -> SYSTOP_POWER_MODE_W<SOFT_TURN_OFF_POWER_MODE_SPEC> {
        SYSTOP_POWER_MODE_W::new(self, 0)
    }
    #[doc = "Bit 1 - CPU turn-off power mode."]
    #[inline(always)]
    #[must_use]
    pub fn cpu_power_mode(&mut self) -> CPU_POWER_MODE_W<SOFT_TURN_OFF_POWER_MODE_SPEC> {
        CPU_POWER_MODE_W::new(self, 1)
    }
    #[doc = "Bit 2 - GPUA turn-off power mode."]
    #[inline(always)]
    #[must_use]
    pub fn gpua_power_mode(&mut self) -> GPUA_POWER_MODE_W<SOFT_TURN_OFF_POWER_MODE_SPEC> {
        GPUA_POWER_MODE_W::new(self, 2)
    }
    #[doc = "Bit 3 - VDEC turn-off power mode."]
    #[inline(always)]
    #[must_use]
    pub fn vdec_power_mode(&mut self) -> VDEC_POWER_MODE_W<SOFT_TURN_OFF_POWER_MODE_SPEC> {
        VDEC_POWER_MODE_W::new(self, 3)
    }
    #[doc = "Bit 4 - VOUT turn-off power mode."]
    #[inline(always)]
    #[must_use]
    pub fn vout_power_mode(&mut self) -> VOUT_POWER_MODE_W<SOFT_TURN_OFF_POWER_MODE_SPEC> {
        VOUT_POWER_MODE_W::new(self, 4)
    }
    #[doc = "Bit 5 - ISP turn-off power mode."]
    #[inline(always)]
    #[must_use]
    pub fn isp_power_mode(&mut self) -> ISP_POWER_MODE_W<SOFT_TURN_OFF_POWER_MODE_SPEC> {
        ISP_POWER_MODE_W::new(self, 5)
    }
    #[doc = "Bit 6 - VENC turn-off power mode."]
    #[inline(always)]
    #[must_use]
    pub fn venc_power_mode(&mut self) -> VENC_POWER_MODE_W<SOFT_TURN_OFF_POWER_MODE_SPEC> {
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
#[doc = "Software Turn-Off Power Mode\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`soft_turn_off_power_mode::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`soft_turn_off_power_mode::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct SOFT_TURN_OFF_POWER_MODE_SPEC;
impl crate::RegisterSpec for SOFT_TURN_OFF_POWER_MODE_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`soft_turn_off_power_mode::R`](R) reader structure"]
impl crate::Readable for SOFT_TURN_OFF_POWER_MODE_SPEC {}
#[doc = "`write(|w| ..)` method takes [`soft_turn_off_power_mode::W`](W) writer structure"]
impl crate::Writable for SOFT_TURN_OFF_POWER_MODE_SPEC {
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets soft_turn_off_power_mode to value 0"]
impl crate::Resettable for SOFT_TURN_OFF_POWER_MODE_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
