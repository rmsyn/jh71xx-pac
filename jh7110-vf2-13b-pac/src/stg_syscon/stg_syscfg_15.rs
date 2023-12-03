#[doc = "Register `stg_syscfg_15` reader"]
pub type R = crate::R<STG_SYSCFG_15_SPEC>;
#[doc = "Register `stg_syscfg_15` writer"]
pub type W = crate::W<STG_SYSCFG_15_SPEC>;
#[doc = "Field `u0_hifi4_scfg_dsp_mst_offset_master` reader - Indicates that master port remap address"]
pub type U0_HIFI4_SCFG_DSP_MST_OFFSET_MASTER_R = crate::FieldReader<u16>;
#[doc = "Field `u0_hifi4_scfg_dsp_mst_offset_master` writer - Indicates that master port remap address"]
pub type U0_HIFI4_SCFG_DSP_MST_OFFSET_MASTER_W<'a, REG> = crate::FieldWriter<'a, REG, 12, u16>;
#[doc = "Field `u0_hifi4_scfg_dsp_mst_offset_dma` reader - Indicates the DMA port remap address"]
pub type U0_HIFI4_SCFG_DSP_MST_OFFSET_DMA_R = crate::FieldReader<u16>;
#[doc = "Field `u0_hifi4_scfg_dsp_mst_offset_dma` writer - Indicates the DMA port remap address"]
pub type U0_HIFI4_SCFG_DSP_MST_OFFSET_DMA_W<'a, REG> = crate::FieldWriter<'a, REG, 12, u16>;
impl R {
    #[doc = "Bits 0:11 - Indicates that master port remap address"]
    #[inline(always)]
    pub fn u0_hifi4_scfg_dsp_mst_offset_master(&self) -> U0_HIFI4_SCFG_DSP_MST_OFFSET_MASTER_R {
        U0_HIFI4_SCFG_DSP_MST_OFFSET_MASTER_R::new((self.bits & 0x0fff) as u16)
    }
    #[doc = "Bits 16:27 - Indicates the DMA port remap address"]
    #[inline(always)]
    pub fn u0_hifi4_scfg_dsp_mst_offset_dma(&self) -> U0_HIFI4_SCFG_DSP_MST_OFFSET_DMA_R {
        U0_HIFI4_SCFG_DSP_MST_OFFSET_DMA_R::new(((self.bits >> 16) & 0x0fff) as u16)
    }
}
impl W {
    #[doc = "Bits 0:11 - Indicates that master port remap address"]
    #[inline(always)]
    #[must_use]
    pub fn u0_hifi4_scfg_dsp_mst_offset_master(
        &mut self,
    ) -> U0_HIFI4_SCFG_DSP_MST_OFFSET_MASTER_W<STG_SYSCFG_15_SPEC> {
        U0_HIFI4_SCFG_DSP_MST_OFFSET_MASTER_W::new(self, 0)
    }
    #[doc = "Bits 16:27 - Indicates the DMA port remap address"]
    #[inline(always)]
    #[must_use]
    pub fn u0_hifi4_scfg_dsp_mst_offset_dma(
        &mut self,
    ) -> U0_HIFI4_SCFG_DSP_MST_OFFSET_DMA_W<STG_SYSCFG_15_SPEC> {
        U0_HIFI4_SCFG_DSP_MST_OFFSET_DMA_W::new(self, 16)
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
#[doc = "STG SYSCONSAIF SYSCFG 60\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`stg_syscfg_15::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`stg_syscfg_15::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct STG_SYSCFG_15_SPEC;
impl crate::RegisterSpec for STG_SYSCFG_15_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`stg_syscfg_15::R`](R) reader structure"]
impl crate::Readable for STG_SYSCFG_15_SPEC {}
#[doc = "`write(|w| ..)` method takes [`stg_syscfg_15::W`](W) writer structure"]
impl crate::Writable for STG_SYSCFG_15_SPEC {
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets stg_syscfg_15 to value 0"]
impl crate::Resettable for STG_SYSCFG_15_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
