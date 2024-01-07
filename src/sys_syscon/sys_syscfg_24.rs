#[doc = "Register `sys_syscfg_24` reader"]
pub type R = crate::R<SYS_SYSCFG_24_SPEC>;
#[doc = "Register `sys_syscfg_24` writer"]
pub type W = crate::W<SYS_SYSCFG_24_SPEC>;
#[doc = "Field `tdm16slot_clkpol` reader - tdm16slot_clkpol"]
pub type TDM16SLOT_CLKPOL_R = crate::BitReader;
#[doc = "Field `tdm16slot_pcm_ms` reader - tdm16slot_pcm_ms"]
pub type TDM16SLOT_PCM_MS_R = crate::BitReader;
#[doc = "Field `u0_trace_mtx_in0_0` reader - u0_trace_mtx_in0_0"]
pub type U0_TRACE_MTX_IN0_0_R = crate::FieldReader;
#[doc = "Field `u0_trace_mtx_in0_0` writer - u0_trace_mtx_in0_0"]
pub type U0_TRACE_MTX_IN0_0_W<'a, REG> = crate::FieldWriter<'a, REG, 5>;
#[doc = "Field `u0_trace_mtx_in1_0` reader - u0_trace_mtx_in1_0"]
pub type U0_TRACE_MTX_IN1_0_R = crate::FieldReader;
#[doc = "Field `u0_trace_mtx_in1_0` writer - u0_trace_mtx_in1_0"]
pub type U0_TRACE_MTX_IN1_0_W<'a, REG> = crate::FieldWriter<'a, REG, 5>;
#[doc = "Field `u0_trace_mtx_in0_1` reader - u0_trace_mtx_in0_1"]
pub type U0_TRACE_MTX_IN0_1_R = crate::FieldReader;
#[doc = "Field `u0_trace_mtx_in0_1` writer - u0_trace_mtx_in0_1"]
pub type U0_TRACE_MTX_IN0_1_W<'a, REG> = crate::FieldWriter<'a, REG, 5>;
#[doc = "Field `u0_trace_mtx_in1_1` reader - u0_trace_mtx_in1_1"]
pub type U0_TRACE_MTX_IN1_1_R = crate::FieldReader;
#[doc = "Field `u0_trace_mtx_in1_1` writer - u0_trace_mtx_in1_1"]
pub type U0_TRACE_MTX_IN1_1_W<'a, REG> = crate::FieldWriter<'a, REG, 5>;
#[doc = "Field `u0_trace_mtx_in0_2` reader - u0_trace_mtx_in0_2"]
pub type U0_TRACE_MTX_IN0_2_R = crate::FieldReader;
#[doc = "Field `u0_trace_mtx_in0_2` writer - u0_trace_mtx_in0_2"]
pub type U0_TRACE_MTX_IN0_2_W<'a, REG> = crate::FieldWriter<'a, REG, 5>;
#[doc = "Field `u0_trace_mtx_in1_2` reader - u0_trace_mtx_in1_2"]
pub type U0_TRACE_MTX_IN1_2_R = crate::FieldReader;
#[doc = "Field `u0_trace_mtx_in1_2` writer - u0_trace_mtx_in1_2"]
pub type U0_TRACE_MTX_IN1_2_W<'a, REG> = crate::FieldWriter<'a, REG, 5>;
impl R {
    #[doc = "Bit 0 - tdm16slot_clkpol"]
    #[inline(always)]
    pub fn tdm16slot_clkpol(&self) -> TDM16SLOT_CLKPOL_R {
        TDM16SLOT_CLKPOL_R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - tdm16slot_pcm_ms"]
    #[inline(always)]
    pub fn tdm16slot_pcm_ms(&self) -> TDM16SLOT_PCM_MS_R {
        TDM16SLOT_PCM_MS_R::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bits 2:6 - u0_trace_mtx_in0_0"]
    #[inline(always)]
    pub fn u0_trace_mtx_in0_0(&self) -> U0_TRACE_MTX_IN0_0_R {
        U0_TRACE_MTX_IN0_0_R::new(((self.bits >> 2) & 0x1f) as u8)
    }
    #[doc = "Bits 7:11 - u0_trace_mtx_in1_0"]
    #[inline(always)]
    pub fn u0_trace_mtx_in1_0(&self) -> U0_TRACE_MTX_IN1_0_R {
        U0_TRACE_MTX_IN1_0_R::new(((self.bits >> 7) & 0x1f) as u8)
    }
    #[doc = "Bits 12:16 - u0_trace_mtx_in0_1"]
    #[inline(always)]
    pub fn u0_trace_mtx_in0_1(&self) -> U0_TRACE_MTX_IN0_1_R {
        U0_TRACE_MTX_IN0_1_R::new(((self.bits >> 12) & 0x1f) as u8)
    }
    #[doc = "Bits 17:21 - u0_trace_mtx_in1_1"]
    #[inline(always)]
    pub fn u0_trace_mtx_in1_1(&self) -> U0_TRACE_MTX_IN1_1_R {
        U0_TRACE_MTX_IN1_1_R::new(((self.bits >> 17) & 0x1f) as u8)
    }
    #[doc = "Bits 22:26 - u0_trace_mtx_in0_2"]
    #[inline(always)]
    pub fn u0_trace_mtx_in0_2(&self) -> U0_TRACE_MTX_IN0_2_R {
        U0_TRACE_MTX_IN0_2_R::new(((self.bits >> 22) & 0x1f) as u8)
    }
    #[doc = "Bits 27:31 - u0_trace_mtx_in1_2"]
    #[inline(always)]
    pub fn u0_trace_mtx_in1_2(&self) -> U0_TRACE_MTX_IN1_2_R {
        U0_TRACE_MTX_IN1_2_R::new(((self.bits >> 27) & 0x1f) as u8)
    }
}
impl W {
    #[doc = "Bits 2:6 - u0_trace_mtx_in0_0"]
    #[inline(always)]
    #[must_use]
    pub fn u0_trace_mtx_in0_0(&mut self) -> U0_TRACE_MTX_IN0_0_W<SYS_SYSCFG_24_SPEC> {
        U0_TRACE_MTX_IN0_0_W::new(self, 2)
    }
    #[doc = "Bits 7:11 - u0_trace_mtx_in1_0"]
    #[inline(always)]
    #[must_use]
    pub fn u0_trace_mtx_in1_0(&mut self) -> U0_TRACE_MTX_IN1_0_W<SYS_SYSCFG_24_SPEC> {
        U0_TRACE_MTX_IN1_0_W::new(self, 7)
    }
    #[doc = "Bits 12:16 - u0_trace_mtx_in0_1"]
    #[inline(always)]
    #[must_use]
    pub fn u0_trace_mtx_in0_1(&mut self) -> U0_TRACE_MTX_IN0_1_W<SYS_SYSCFG_24_SPEC> {
        U0_TRACE_MTX_IN0_1_W::new(self, 12)
    }
    #[doc = "Bits 17:21 - u0_trace_mtx_in1_1"]
    #[inline(always)]
    #[must_use]
    pub fn u0_trace_mtx_in1_1(&mut self) -> U0_TRACE_MTX_IN1_1_W<SYS_SYSCFG_24_SPEC> {
        U0_TRACE_MTX_IN1_1_W::new(self, 17)
    }
    #[doc = "Bits 22:26 - u0_trace_mtx_in0_2"]
    #[inline(always)]
    #[must_use]
    pub fn u0_trace_mtx_in0_2(&mut self) -> U0_TRACE_MTX_IN0_2_W<SYS_SYSCFG_24_SPEC> {
        U0_TRACE_MTX_IN0_2_W::new(self, 22)
    }
    #[doc = "Bits 27:31 - u0_trace_mtx_in1_2"]
    #[inline(always)]
    #[must_use]
    pub fn u0_trace_mtx_in1_2(&mut self) -> U0_TRACE_MTX_IN1_2_W<SYS_SYSCFG_24_SPEC> {
        U0_TRACE_MTX_IN1_2_W::new(self, 27)
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
