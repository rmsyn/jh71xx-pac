#[doc = "Register `stg_sysconsaif_syscfg68` reader"]
pub type R = crate::R<STG_SYSCONSAIF_SYSCFG68_SPEC>;
#[doc = "Register `stg_sysconsaif_syscfg68` writer"]
pub type W = crate::W<STG_SYSCONSAIF_SYSCFG68_SPEC>;
#[doc = "Field `u0_hifi4_scfg_sram_config_slp` reader - SRAM/ROM configuration. SLP: sleep enable, high active, default is low."]
pub type U0_HIFI4_SCFG_SRAM_CONFIG_SLP_R = crate::BitReader;
#[doc = "Field `u0_hifi4_scfg_sram_config_slp` writer - SRAM/ROM configuration. SLP: sleep enable, high active, default is low."]
pub type U0_HIFI4_SCFG_SRAM_CONFIG_SLP_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O>;
#[doc = "Field `u0_hifi4_scfg_sram_config_sram_config_sd` reader - SRAM/ROM configuration. SD: shutdown enable, high active, default is low."]
pub type U0_HIFI4_SCFG_SRAM_CONFIG_SRAM_CONFIG_SD_R = crate::BitReader;
#[doc = "Field `u0_hifi4_scfg_sram_config_sram_config_sd` writer - SRAM/ROM configuration. SD: shutdown enable, high active, default is low."]
pub type U0_HIFI4_SCFG_SRAM_CONFIG_SRAM_CONFIG_SD_W<'a, REG, const O: u8> =
    crate::BitWriter<'a, REG, O>;
#[doc = "Field `u0_hifi4_scfg_sram_config_rtsel` reader - SRAM/ROM configuration. RTSEL: timing setting for debug purpose, default is 2'b01."]
pub type U0_HIFI4_SCFG_SRAM_CONFIG_RTSEL_R = crate::FieldReader;
#[doc = "Field `u0_hifi4_scfg_sram_config_rtsel` writer - SRAM/ROM configuration. RTSEL: timing setting for debug purpose, default is 2'b01."]
pub type U0_HIFI4_SCFG_SRAM_CONFIG_RTSEL_W<'a, REG, const O: u8> =
    crate::FieldWriter<'a, REG, 2, O>;
#[doc = "Field `u0_hifi4_scfg_sram_config_ptsel` reader - SRAM/ROM configuration. PTSEL: timing setting for debug purpose, default is 2'b01."]
pub type U0_HIFI4_SCFG_SRAM_CONFIG_PTSEL_R = crate::FieldReader;
#[doc = "Field `u0_hifi4_scfg_sram_config_ptsel` writer - SRAM/ROM configuration. PTSEL: timing setting for debug purpose, default is 2'b01."]
pub type U0_HIFI4_SCFG_SRAM_CONFIG_PTSEL_W<'a, REG, const O: u8> =
    crate::FieldWriter<'a, REG, 2, O>;
#[doc = "Field `u0_hifi4_scfg_sram_config_trb` reader - SRAM/ROM configuration. TRB: timing setting for debug purpose, default is 2'b01."]
pub type U0_HIFI4_SCFG_SRAM_CONFIG_TRB_R = crate::FieldReader;
#[doc = "Field `u0_hifi4_scfg_sram_config_trb` writer - SRAM/ROM configuration. TRB: timing setting for debug purpose, default is 2'b01."]
pub type U0_HIFI4_SCFG_SRAM_CONFIG_TRB_W<'a, REG, const O: u8> = crate::FieldWriter<'a, REG, 2, O>;
#[doc = "Field `u0_hifi4_scfg_sram_config_wtsel` reader - SRAM/ROM configuration. WTSEL: timing setting for debug purpose, default is 2'b01."]
pub type U0_HIFI4_SCFG_SRAM_CONFIG_WTSEL_R = crate::FieldReader;
#[doc = "Field `u0_hifi4_scfg_sram_config_wtsel` writer - SRAM/ROM configuration. WTSEL: timing setting for debug purpose, default is 2'b01."]
pub type U0_HIFI4_SCFG_SRAM_CONFIG_WTSEL_W<'a, REG, const O: u8> =
    crate::FieldWriter<'a, REG, 2, O>;
#[doc = "Field `u0_hifi4_scfg_sram_config_vs` reader - SRAM/ROM configuration. VS: timing setting for debug purpose, default is 1'b1."]
pub type U0_HIFI4_SCFG_SRAM_CONFIG_VS_R = crate::BitReader;
#[doc = "Field `u0_hifi4_scfg_sram_config_vs` writer - SRAM/ROM configuration. VS: timing setting for debug purpose, default is 1'b1."]
pub type U0_HIFI4_SCFG_SRAM_CONFIG_VS_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O>;
#[doc = "Field `u0_hifi4_scfg_sram_config_vg` reader - SRAM/ROM configuration. VG: timing setting for debug purpose, default is 1'b1."]
pub type U0_HIFI4_SCFG_SRAM_CONFIG_VG_R = crate::BitReader;
#[doc = "Field `u0_hifi4_scfg_sram_config_vg` writer - SRAM/ROM configuration. VG: timing setting for debug purpose, default is 1'b1."]
pub type U0_HIFI4_SCFG_SRAM_CONFIG_VG_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O>;
#[doc = "Field `u0_hifi4_statvectorsel` reader - When the value is 1, it indicates that the AltResetVec is valid"]
pub type U0_HIFI4_STATVECTORSEL_R = crate::BitReader;
#[doc = "Field `u0_hifi4_statvectorsel` writer - When the value is 1, it indicates that the AltResetVec is valid"]
pub type U0_HIFI4_STATVECTORSEL_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O>;
#[doc = "Field `u0_hifi4_trigin_idma` reader - DMA port trigger"]
pub type U0_HIFI4_TRIGIN_IDMA_R = crate::BitReader;
#[doc = "Field `u0_hifi4_trigin_idma` writer - DMA port trigger"]
pub type U0_HIFI4_TRIGIN_IDMA_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O>;
#[doc = "Field `u0_hifi4_trigout_idma` reader - DMA port trigger"]
pub type U0_HIFI4_TRIGOUT_IDMA_R = crate::BitReader;
#[doc = "Field `u0_hifi4_xocdmode` reader - Debug signal"]
pub type U0_HIFI4_XOCDMODE_R = crate::BitReader;
#[doc = "Field `u0_plda_pcie_align_detect` reader - u0_plda_pcie_align_detect"]
pub type U0_PLDA_PCIE_ALIGN_DETECT_R = crate::BitReader;
impl R {
    #[doc = "Bit 0 - SRAM/ROM configuration. SLP: sleep enable, high active, default is low."]
    #[inline(always)]
    pub fn u0_hifi4_scfg_sram_config_slp(&self) -> U0_HIFI4_SCFG_SRAM_CONFIG_SLP_R {
        U0_HIFI4_SCFG_SRAM_CONFIG_SLP_R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - SRAM/ROM configuration. SD: shutdown enable, high active, default is low."]
    #[inline(always)]
    pub fn u0_hifi4_scfg_sram_config_sram_config_sd(
        &self,
    ) -> U0_HIFI4_SCFG_SRAM_CONFIG_SRAM_CONFIG_SD_R {
        U0_HIFI4_SCFG_SRAM_CONFIG_SRAM_CONFIG_SD_R::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bits 2:3 - SRAM/ROM configuration. RTSEL: timing setting for debug purpose, default is 2'b01."]
    #[inline(always)]
    pub fn u0_hifi4_scfg_sram_config_rtsel(&self) -> U0_HIFI4_SCFG_SRAM_CONFIG_RTSEL_R {
        U0_HIFI4_SCFG_SRAM_CONFIG_RTSEL_R::new(((self.bits >> 2) & 3) as u8)
    }
    #[doc = "Bits 4:5 - SRAM/ROM configuration. PTSEL: timing setting for debug purpose, default is 2'b01."]
    #[inline(always)]
    pub fn u0_hifi4_scfg_sram_config_ptsel(&self) -> U0_HIFI4_SCFG_SRAM_CONFIG_PTSEL_R {
        U0_HIFI4_SCFG_SRAM_CONFIG_PTSEL_R::new(((self.bits >> 4) & 3) as u8)
    }
    #[doc = "Bits 6:7 - SRAM/ROM configuration. TRB: timing setting for debug purpose, default is 2'b01."]
    #[inline(always)]
    pub fn u0_hifi4_scfg_sram_config_trb(&self) -> U0_HIFI4_SCFG_SRAM_CONFIG_TRB_R {
        U0_HIFI4_SCFG_SRAM_CONFIG_TRB_R::new(((self.bits >> 6) & 3) as u8)
    }
    #[doc = "Bits 8:9 - SRAM/ROM configuration. WTSEL: timing setting for debug purpose, default is 2'b01."]
    #[inline(always)]
    pub fn u0_hifi4_scfg_sram_config_wtsel(&self) -> U0_HIFI4_SCFG_SRAM_CONFIG_WTSEL_R {
        U0_HIFI4_SCFG_SRAM_CONFIG_WTSEL_R::new(((self.bits >> 8) & 3) as u8)
    }
    #[doc = "Bit 10 - SRAM/ROM configuration. VS: timing setting for debug purpose, default is 1'b1."]
    #[inline(always)]
    pub fn u0_hifi4_scfg_sram_config_vs(&self) -> U0_HIFI4_SCFG_SRAM_CONFIG_VS_R {
        U0_HIFI4_SCFG_SRAM_CONFIG_VS_R::new(((self.bits >> 10) & 1) != 0)
    }
    #[doc = "Bit 11 - SRAM/ROM configuration. VG: timing setting for debug purpose, default is 1'b1."]
    #[inline(always)]
    pub fn u0_hifi4_scfg_sram_config_vg(&self) -> U0_HIFI4_SCFG_SRAM_CONFIG_VG_R {
        U0_HIFI4_SCFG_SRAM_CONFIG_VG_R::new(((self.bits >> 11) & 1) != 0)
    }
    #[doc = "Bit 12 - When the value is 1, it indicates that the AltResetVec is valid"]
    #[inline(always)]
    pub fn u0_hifi4_statvectorsel(&self) -> U0_HIFI4_STATVECTORSEL_R {
        U0_HIFI4_STATVECTORSEL_R::new(((self.bits >> 12) & 1) != 0)
    }
    #[doc = "Bit 13 - DMA port trigger"]
    #[inline(always)]
    pub fn u0_hifi4_trigin_idma(&self) -> U0_HIFI4_TRIGIN_IDMA_R {
        U0_HIFI4_TRIGIN_IDMA_R::new(((self.bits >> 13) & 1) != 0)
    }
    #[doc = "Bit 14 - DMA port trigger"]
    #[inline(always)]
    pub fn u0_hifi4_trigout_idma(&self) -> U0_HIFI4_TRIGOUT_IDMA_R {
        U0_HIFI4_TRIGOUT_IDMA_R::new(((self.bits >> 14) & 1) != 0)
    }
    #[doc = "Bit 15 - Debug signal"]
    #[inline(always)]
    pub fn u0_hifi4_xocdmode(&self) -> U0_HIFI4_XOCDMODE_R {
        U0_HIFI4_XOCDMODE_R::new(((self.bits >> 15) & 1) != 0)
    }
    #[doc = "Bit 16 - u0_plda_pcie_align_detect"]
    #[inline(always)]
    pub fn u0_plda_pcie_align_detect(&self) -> U0_PLDA_PCIE_ALIGN_DETECT_R {
        U0_PLDA_PCIE_ALIGN_DETECT_R::new(((self.bits >> 16) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - SRAM/ROM configuration. SLP: sleep enable, high active, default is low."]
    #[inline(always)]
    #[must_use]
    pub fn u0_hifi4_scfg_sram_config_slp(
        &mut self,
    ) -> U0_HIFI4_SCFG_SRAM_CONFIG_SLP_W<STG_SYSCONSAIF_SYSCFG68_SPEC, 0> {
        U0_HIFI4_SCFG_SRAM_CONFIG_SLP_W::new(self)
    }
    #[doc = "Bit 1 - SRAM/ROM configuration. SD: shutdown enable, high active, default is low."]
    #[inline(always)]
    #[must_use]
    pub fn u0_hifi4_scfg_sram_config_sram_config_sd(
        &mut self,
    ) -> U0_HIFI4_SCFG_SRAM_CONFIG_SRAM_CONFIG_SD_W<STG_SYSCONSAIF_SYSCFG68_SPEC, 1> {
        U0_HIFI4_SCFG_SRAM_CONFIG_SRAM_CONFIG_SD_W::new(self)
    }
    #[doc = "Bits 2:3 - SRAM/ROM configuration. RTSEL: timing setting for debug purpose, default is 2'b01."]
    #[inline(always)]
    #[must_use]
    pub fn u0_hifi4_scfg_sram_config_rtsel(
        &mut self,
    ) -> U0_HIFI4_SCFG_SRAM_CONFIG_RTSEL_W<STG_SYSCONSAIF_SYSCFG68_SPEC, 2> {
        U0_HIFI4_SCFG_SRAM_CONFIG_RTSEL_W::new(self)
    }
    #[doc = "Bits 4:5 - SRAM/ROM configuration. PTSEL: timing setting for debug purpose, default is 2'b01."]
    #[inline(always)]
    #[must_use]
    pub fn u0_hifi4_scfg_sram_config_ptsel(
        &mut self,
    ) -> U0_HIFI4_SCFG_SRAM_CONFIG_PTSEL_W<STG_SYSCONSAIF_SYSCFG68_SPEC, 4> {
        U0_HIFI4_SCFG_SRAM_CONFIG_PTSEL_W::new(self)
    }
    #[doc = "Bits 6:7 - SRAM/ROM configuration. TRB: timing setting for debug purpose, default is 2'b01."]
    #[inline(always)]
    #[must_use]
    pub fn u0_hifi4_scfg_sram_config_trb(
        &mut self,
    ) -> U0_HIFI4_SCFG_SRAM_CONFIG_TRB_W<STG_SYSCONSAIF_SYSCFG68_SPEC, 6> {
        U0_HIFI4_SCFG_SRAM_CONFIG_TRB_W::new(self)
    }
    #[doc = "Bits 8:9 - SRAM/ROM configuration. WTSEL: timing setting for debug purpose, default is 2'b01."]
    #[inline(always)]
    #[must_use]
    pub fn u0_hifi4_scfg_sram_config_wtsel(
        &mut self,
    ) -> U0_HIFI4_SCFG_SRAM_CONFIG_WTSEL_W<STG_SYSCONSAIF_SYSCFG68_SPEC, 8> {
        U0_HIFI4_SCFG_SRAM_CONFIG_WTSEL_W::new(self)
    }
    #[doc = "Bit 10 - SRAM/ROM configuration. VS: timing setting for debug purpose, default is 1'b1."]
    #[inline(always)]
    #[must_use]
    pub fn u0_hifi4_scfg_sram_config_vs(
        &mut self,
    ) -> U0_HIFI4_SCFG_SRAM_CONFIG_VS_W<STG_SYSCONSAIF_SYSCFG68_SPEC, 10> {
        U0_HIFI4_SCFG_SRAM_CONFIG_VS_W::new(self)
    }
    #[doc = "Bit 11 - SRAM/ROM configuration. VG: timing setting for debug purpose, default is 1'b1."]
    #[inline(always)]
    #[must_use]
    pub fn u0_hifi4_scfg_sram_config_vg(
        &mut self,
    ) -> U0_HIFI4_SCFG_SRAM_CONFIG_VG_W<STG_SYSCONSAIF_SYSCFG68_SPEC, 11> {
        U0_HIFI4_SCFG_SRAM_CONFIG_VG_W::new(self)
    }
    #[doc = "Bit 12 - When the value is 1, it indicates that the AltResetVec is valid"]
    #[inline(always)]
    #[must_use]
    pub fn u0_hifi4_statvectorsel(
        &mut self,
    ) -> U0_HIFI4_STATVECTORSEL_W<STG_SYSCONSAIF_SYSCFG68_SPEC, 12> {
        U0_HIFI4_STATVECTORSEL_W::new(self)
    }
    #[doc = "Bit 13 - DMA port trigger"]
    #[inline(always)]
    #[must_use]
    pub fn u0_hifi4_trigin_idma(
        &mut self,
    ) -> U0_HIFI4_TRIGIN_IDMA_W<STG_SYSCONSAIF_SYSCFG68_SPEC, 13> {
        U0_HIFI4_TRIGIN_IDMA_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.bits = bits;
        self
    }
}
#[doc = "STG SYSCONSAIF SYSCFG 68\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`stg_sysconsaif_syscfg68::R`](R).  You can [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`stg_sysconsaif_syscfg68::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct STG_SYSCONSAIF_SYSCFG68_SPEC;
impl crate::RegisterSpec for STG_SYSCONSAIF_SYSCFG68_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`stg_sysconsaif_syscfg68::R`](R) reader structure"]
impl crate::Readable for STG_SYSCONSAIF_SYSCFG68_SPEC {}
#[doc = "`write(|w| ..)` method takes [`stg_sysconsaif_syscfg68::W`](W) writer structure"]
impl crate::Writable for STG_SYSCONSAIF_SYSCFG68_SPEC {
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
