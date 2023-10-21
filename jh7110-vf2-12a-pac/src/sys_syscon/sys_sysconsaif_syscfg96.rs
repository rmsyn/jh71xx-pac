#[doc = "Register `sys_sysconsaif_syscfg96` reader"]
pub type R = crate::R<SYS_SYSCONSAIF_SYSCFG96_SPEC>;
#[doc = "Register `sys_sysconsaif_syscfg96` writer"]
pub type W = crate::W<SYS_SYSCONSAIF_SYSCFG96_SPEC>;
#[doc = "Field `u0_tdm16slot_clkpol` reader - u0_tdm16slot_clkpol"]
pub type U0_TDM16SLOT_CLKPOL_R = crate::BitReader;
#[doc = "Field `u0_tdm16slot_pcm_ms` reader - u0_tdm16slot_pcm_ms"]
pub type U0_TDM16SLOT_PCM_MS_R = crate::BitReader;
#[doc = "Field `u0_trace_mtx_scfg_c0_in0_ctl` reader - u0_trace_mtx_scfg_c0_in0_ctl"]
pub type U0_TRACE_MTX_SCFG_C0_IN0_CTL_R = crate::FieldReader;
#[doc = "Field `u0_trace_mtx_scfg_c0_in0_ctl` writer - u0_trace_mtx_scfg_c0_in0_ctl"]
pub type U0_TRACE_MTX_SCFG_C0_IN0_CTL_W<'a, REG, const O: u8> = crate::FieldWriter<'a, REG, 5, O>;
#[doc = "Field `u0_trace_mtx_scfg_c0_in1_ctl` reader - u0_trace_mtx_scfg_c0_in1_ctl"]
pub type U0_TRACE_MTX_SCFG_C0_IN1_CTL_R = crate::FieldReader;
#[doc = "Field `u0_trace_mtx_scfg_c0_in1_ctl` writer - u0_trace_mtx_scfg_c0_in1_ctl"]
pub type U0_TRACE_MTX_SCFG_C0_IN1_CTL_W<'a, REG, const O: u8> = crate::FieldWriter<'a, REG, 5, O>;
#[doc = "Field `u0_trace_mtx_scfg_c1_in0_ctl` reader - u0_trace_mtx_scfg_c1_in0_ctl"]
pub type U0_TRACE_MTX_SCFG_C1_IN0_CTL_R = crate::FieldReader;
#[doc = "Field `u0_trace_mtx_scfg_c1_in0_ctl` writer - u0_trace_mtx_scfg_c1_in0_ctl"]
pub type U0_TRACE_MTX_SCFG_C1_IN0_CTL_W<'a, REG, const O: u8> = crate::FieldWriter<'a, REG, 5, O>;
#[doc = "Field `u0_trace_mtx_scfg_c1_in1_ctl` reader - u0_trace_mtx_scfg_c1_in1_ctl"]
pub type U0_TRACE_MTX_SCFG_C1_IN1_CTL_R = crate::FieldReader;
#[doc = "Field `u0_trace_mtx_scfg_c1_in1_ctl` writer - u0_trace_mtx_scfg_c1_in1_ctl"]
pub type U0_TRACE_MTX_SCFG_C1_IN1_CTL_W<'a, REG, const O: u8> = crate::FieldWriter<'a, REG, 5, O>;
#[doc = "Field `u0_trace_mtx_scfg_c2_in0_ctl` reader - u0_trace_mtx_scfg_c2_in0_ctl"]
pub type U0_TRACE_MTX_SCFG_C2_IN0_CTL_R = crate::FieldReader;
#[doc = "Field `u0_trace_mtx_scfg_c2_in0_ctl` writer - u0_trace_mtx_scfg_c2_in0_ctl"]
pub type U0_TRACE_MTX_SCFG_C2_IN0_CTL_W<'a, REG, const O: u8> = crate::FieldWriter<'a, REG, 5, O>;
#[doc = "Field `u0_trace_mtx_scfg_c2_in1_ctl` reader - u0_trace_mtx_scfg_c2_in1_ctl"]
pub type U0_TRACE_MTX_SCFG_C2_IN1_CTL_R = crate::FieldReader;
#[doc = "Field `u0_trace_mtx_scfg_c2_in1_ctl` writer - u0_trace_mtx_scfg_c2_in1_ctl"]
pub type U0_TRACE_MTX_SCFG_C2_IN1_CTL_W<'a, REG, const O: u8> = crate::FieldWriter<'a, REG, 5, O>;
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
    ) -> U0_TRACE_MTX_SCFG_C0_IN0_CTL_W<SYS_SYSCONSAIF_SYSCFG96_SPEC, 2> {
        U0_TRACE_MTX_SCFG_C0_IN0_CTL_W::new(self)
    }
    #[doc = "Bits 7:11 - u0_trace_mtx_scfg_c0_in1_ctl"]
    #[inline(always)]
    #[must_use]
    pub fn u0_trace_mtx_scfg_c0_in1_ctl(
        &mut self,
    ) -> U0_TRACE_MTX_SCFG_C0_IN1_CTL_W<SYS_SYSCONSAIF_SYSCFG96_SPEC, 7> {
        U0_TRACE_MTX_SCFG_C0_IN1_CTL_W::new(self)
    }
    #[doc = "Bits 12:16 - u0_trace_mtx_scfg_c1_in0_ctl"]
    #[inline(always)]
    #[must_use]
    pub fn u0_trace_mtx_scfg_c1_in0_ctl(
        &mut self,
    ) -> U0_TRACE_MTX_SCFG_C1_IN0_CTL_W<SYS_SYSCONSAIF_SYSCFG96_SPEC, 12> {
        U0_TRACE_MTX_SCFG_C1_IN0_CTL_W::new(self)
    }
    #[doc = "Bits 17:21 - u0_trace_mtx_scfg_c1_in1_ctl"]
    #[inline(always)]
    #[must_use]
    pub fn u0_trace_mtx_scfg_c1_in1_ctl(
        &mut self,
    ) -> U0_TRACE_MTX_SCFG_C1_IN1_CTL_W<SYS_SYSCONSAIF_SYSCFG96_SPEC, 17> {
        U0_TRACE_MTX_SCFG_C1_IN1_CTL_W::new(self)
    }
    #[doc = "Bits 22:26 - u0_trace_mtx_scfg_c2_in0_ctl"]
    #[inline(always)]
    #[must_use]
    pub fn u0_trace_mtx_scfg_c2_in0_ctl(
        &mut self,
    ) -> U0_TRACE_MTX_SCFG_C2_IN0_CTL_W<SYS_SYSCONSAIF_SYSCFG96_SPEC, 22> {
        U0_TRACE_MTX_SCFG_C2_IN0_CTL_W::new(self)
    }
    #[doc = "Bits 27:31 - u0_trace_mtx_scfg_c2_in1_ctl"]
    #[inline(always)]
    #[must_use]
    pub fn u0_trace_mtx_scfg_c2_in1_ctl(
        &mut self,
    ) -> U0_TRACE_MTX_SCFG_C2_IN1_CTL_W<SYS_SYSCONSAIF_SYSCFG96_SPEC, 27> {
        U0_TRACE_MTX_SCFG_C2_IN1_CTL_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.bits = bits;
        self
    }
}
#[doc = "SYS SYSCONSAIF SYSCFG 96\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`sys_sysconsaif_syscfg96::R`](R).  You can [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`sys_sysconsaif_syscfg96::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct SYS_SYSCONSAIF_SYSCFG96_SPEC;
impl crate::RegisterSpec for SYS_SYSCONSAIF_SYSCFG96_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`sys_sysconsaif_syscfg96::R`](R) reader structure"]
impl crate::Readable for SYS_SYSCONSAIF_SYSCFG96_SPEC {}
#[doc = "`write(|w| ..)` method takes [`sys_sysconsaif_syscfg96::W`](W) writer structure"]
impl crate::Writable for SYS_SYSCONSAIF_SYSCFG96_SPEC {
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
