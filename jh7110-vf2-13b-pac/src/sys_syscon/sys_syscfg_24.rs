#[doc = "Register `sys_syscfg_24` reader"]
pub type R = crate::R<SYS_SYSCFG_24_SPEC>;
#[doc = "Register `sys_syscfg_24` writer"]
pub type W = crate::W<SYS_SYSCFG_24_SPEC>;
#[doc = "Field `u0_tdm16slot_clkpol` reader - u0_tdm16slot_clkpol"]
pub type U0_TDM16SLOT_CLKPOL_R = crate::BitReader;
#[doc = "Field `u0_tdm16slot_pcm_ms` reader - u0_tdm16slot_pcm_ms"]
pub type U0_TDM16SLOT_PCM_MS_R = crate::BitReader;
#[doc = "Field `u0_trace_mtx_scfg_c0_in0_ctl` reader - u0_trace_mtx_scfg_c0_in0_ctl"]
pub type U0_TRACE_MTX_SCFG_C0_IN0_CTL_R = crate::FieldReader;
#[doc = "Field `u0_trace_mtx_scfg_c0_in0_ctl` writer - u0_trace_mtx_scfg_c0_in0_ctl"]
pub type U0_TRACE_MTX_SCFG_C0_IN0_CTL_W<'a, REG> = crate::FieldWriter<'a, REG, 5>;
#[doc = "Field `u0_trace_mtx_scfg_c0_in1_ctl` reader - u0_trace_mtx_scfg_c0_in1_ctl"]
pub type U0_TRACE_MTX_SCFG_C0_IN1_CTL_R = crate::FieldReader;
#[doc = "Field `u0_trace_mtx_scfg_c0_in1_ctl` writer - u0_trace_mtx_scfg_c0_in1_ctl"]
pub type U0_TRACE_MTX_SCFG_C0_IN1_CTL_W<'a, REG> = crate::FieldWriter<'a, REG, 5>;
#[doc = "Field `u0_trace_mtx_scfg_c1_in0_ctl` reader - u0_trace_mtx_scfg_c1_in0_ctl"]
pub type U0_TRACE_MTX_SCFG_C1_IN0_CTL_R = crate::FieldReader;
#[doc = "Field `u0_trace_mtx_scfg_c1_in0_ctl` writer - u0_trace_mtx_scfg_c1_in0_ctl"]
pub type U0_TRACE_MTX_SCFG_C1_IN0_CTL_W<'a, REG> = crate::FieldWriter<'a, REG, 5>;
#[doc = "Field `u0_trace_mtx_scfg_c1_in1_ctl` reader - u0_trace_mtx_scfg_c1_in1_ctl"]
pub type U0_TRACE_MTX_SCFG_C1_IN1_CTL_R = crate::FieldReader;
#[doc = "Field `u0_trace_mtx_scfg_c1_in1_ctl` writer - u0_trace_mtx_scfg_c1_in1_ctl"]
pub type U0_TRACE_MTX_SCFG_C1_IN1_CTL_W<'a, REG> = crate::FieldWriter<'a, REG, 5>;
#[doc = "Field `u0_trace_mtx_scfg_c2_in0_ctl` reader - u0_trace_mtx_scfg_c2_in0_ctl"]
pub type U0_TRACE_MTX_SCFG_C2_IN0_CTL_R = crate::FieldReader;
#[doc = "Field `u0_trace_mtx_scfg_c2_in0_ctl` writer - u0_trace_mtx_scfg_c2_in0_ctl"]
pub type U0_TRACE_MTX_SCFG_C2_IN0_CTL_W<'a, REG> = crate::FieldWriter<'a, REG, 5>;
#[doc = "Field `u0_trace_mtx_scfg_c2_in1_ctl` reader - u0_trace_mtx_scfg_c2_in1_ctl"]
pub type U0_TRACE_MTX_SCFG_C2_IN1_CTL_R = crate::FieldReader;
#[doc = "Field `u0_trace_mtx_scfg_c2_in1_ctl` writer - u0_trace_mtx_scfg_c2_in1_ctl"]
pub type U0_TRACE_MTX_SCFG_C2_IN1_CTL_W<'a, REG> = crate::FieldWriter<'a, REG, 5>;
impl R {
    #[doc = "Bit 0 - u0_tdm16slot_clkpol"]
    #[inline(always)]
    pub fn u0_tdm16slot_clkpol(&self) -> U0_TDM16SLOT_CLKPOL_R {
        U0_TDM16SLOT_CLKPOL_R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - u0_tdm16slot_pcm_ms"]
    #[inline(always)]
    pub fn u0_tdm16slot_pcm_ms(&self) -> U0_TDM16SLOT_PCM_MS_R {
        U0_TDM16SLOT_PCM_MS_R::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bits 2:6 - u0_trace_mtx_scfg_c0_in0_ctl"]
    #[inline(always)]
    pub fn u0_trace_mtx_scfg_c0_in0_ctl(&self) -> U0_TRACE_MTX_SCFG_C0_IN0_CTL_R {
        U0_TRACE_MTX_SCFG_C0_IN0_CTL_R::new(((self.bits >> 2) & 0x1f) as u8)
    }
    #[doc = "Bits 7:11 - u0_trace_mtx_scfg_c0_in1_ctl"]
    #[inline(always)]
    pub fn u0_trace_mtx_scfg_c0_in1_ctl(&self) -> U0_TRACE_MTX_SCFG_C0_IN1_CTL_R {
        U0_TRACE_MTX_SCFG_C0_IN1_CTL_R::new(((self.bits >> 7) & 0x1f) as u8)
    }
    #[doc = "Bits 12:16 - u0_trace_mtx_scfg_c1_in0_ctl"]
    #[inline(always)]
    pub fn u0_trace_mtx_scfg_c1_in0_ctl(&self) -> U0_TRACE_MTX_SCFG_C1_IN0_CTL_R {
        U0_TRACE_MTX_SCFG_C1_IN0_CTL_R::new(((self.bits >> 12) & 0x1f) as u8)
    }
    #[doc = "Bits 17:21 - u0_trace_mtx_scfg_c1_in1_ctl"]
    #[inline(always)]
    pub fn u0_trace_mtx_scfg_c1_in1_ctl(&self) -> U0_TRACE_MTX_SCFG_C1_IN1_CTL_R {
        U0_TRACE_MTX_SCFG_C1_IN1_CTL_R::new(((self.bits >> 17) & 0x1f) as u8)
    }
    #[doc = "Bits 22:26 - u0_trace_mtx_scfg_c2_in0_ctl"]
    #[inline(always)]
    pub fn u0_trace_mtx_scfg_c2_in0_ctl(&self) -> U0_TRACE_MTX_SCFG_C2_IN0_CTL_R {
        U0_TRACE_MTX_SCFG_C2_IN0_CTL_R::new(((self.bits >> 22) & 0x1f) as u8)
    }
    #[doc = "Bits 27:31 - u0_trace_mtx_scfg_c2_in1_ctl"]
    #[inline(always)]
    pub fn u0_trace_mtx_scfg_c2_in1_ctl(&self) -> U0_TRACE_MTX_SCFG_C2_IN1_CTL_R {
        U0_TRACE_MTX_SCFG_C2_IN1_CTL_R::new(((self.bits >> 27) & 0x1f) as u8)
    }
}
impl W {
    #[doc = "Bits 2:6 - u0_trace_mtx_scfg_c0_in0_ctl"]
    #[inline(always)]
    #[must_use]
    pub fn u0_trace_mtx_scfg_c0_in0_ctl(
        &mut self,
    ) -> U0_TRACE_MTX_SCFG_C0_IN0_CTL_W<SYS_SYSCFG_24_SPEC> {
        U0_TRACE_MTX_SCFG_C0_IN0_CTL_W::new(self, 2)
    }
    #[doc = "Bits 7:11 - u0_trace_mtx_scfg_c0_in1_ctl"]
    #[inline(always)]
    #[must_use]
    pub fn u0_trace_mtx_scfg_c0_in1_ctl(
        &mut self,
    ) -> U0_TRACE_MTX_SCFG_C0_IN1_CTL_W<SYS_SYSCFG_24_SPEC> {
        U0_TRACE_MTX_SCFG_C0_IN1_CTL_W::new(self, 7)
    }
    #[doc = "Bits 12:16 - u0_trace_mtx_scfg_c1_in0_ctl"]
    #[inline(always)]
    #[must_use]
    pub fn u0_trace_mtx_scfg_c1_in0_ctl(
        &mut self,
    ) -> U0_TRACE_MTX_SCFG_C1_IN0_CTL_W<SYS_SYSCFG_24_SPEC> {
        U0_TRACE_MTX_SCFG_C1_IN0_CTL_W::new(self, 12)
    }
    #[doc = "Bits 17:21 - u0_trace_mtx_scfg_c1_in1_ctl"]
    #[inline(always)]
    #[must_use]
    pub fn u0_trace_mtx_scfg_c1_in1_ctl(
        &mut self,
    ) -> U0_TRACE_MTX_SCFG_C1_IN1_CTL_W<SYS_SYSCFG_24_SPEC> {
        U0_TRACE_MTX_SCFG_C1_IN1_CTL_W::new(self, 17)
    }
    #[doc = "Bits 22:26 - u0_trace_mtx_scfg_c2_in0_ctl"]
    #[inline(always)]
    #[must_use]
    pub fn u0_trace_mtx_scfg_c2_in0_ctl(
        &mut self,
    ) -> U0_TRACE_MTX_SCFG_C2_IN0_CTL_W<SYS_SYSCFG_24_SPEC> {
        U0_TRACE_MTX_SCFG_C2_IN0_CTL_W::new(self, 22)
    }
    #[doc = "Bits 27:31 - u0_trace_mtx_scfg_c2_in1_ctl"]
    #[inline(always)]
    #[must_use]
    pub fn u0_trace_mtx_scfg_c2_in1_ctl(
        &mut self,
    ) -> U0_TRACE_MTX_SCFG_C2_IN1_CTL_W<SYS_SYSCFG_24_SPEC> {
        U0_TRACE_MTX_SCFG_C2_IN1_CTL_W::new(self, 27)
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
#[doc = "SYS SYSCONSAIF SYSCFG 96\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`sys_syscfg_24::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`sys_syscfg_24::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct SYS_SYSCFG_24_SPEC;
impl crate::RegisterSpec for SYS_SYSCFG_24_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`sys_syscfg_24::R`](R) reader structure"]
impl crate::Readable for SYS_SYSCFG_24_SPEC {}
#[doc = "`write(|w| ..)` method takes [`sys_syscfg_24::W`](W) writer structure"]
impl crate::Writable for SYS_SYSCFG_24_SPEC {
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets sys_syscfg_24 to value 0"]
impl crate::Resettable for SYS_SYSCFG_24_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
