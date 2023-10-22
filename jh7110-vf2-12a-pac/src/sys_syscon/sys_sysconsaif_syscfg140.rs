#[doc = "Register `sys_sysconsaif_syscfg140` reader"]
pub type R = crate::R<SYS_SYSCONSAIF_SYSCFG140_SPEC>;
#[doc = "Register `sys_sysconsaif_syscfg140` writer"]
pub type W = crate::W<SYS_SYSCONSAIF_SYSCFG140_SPEC>;
#[doc = "Field `u1_can_ctrl_host_if` reader - u1_can_ctrl_host_if"]
pub type U1_CAN_CTRL_HOST_IF_R = crate::FieldReader<u32>;
#[doc = "Field `u1_gmac5_axi64_scfg_ram_cfg_slp` reader - SRAM/ROM configuration. SLP: sleep enable, high active, default is low."]
pub type U1_GMAC5_AXI64_SCFG_RAM_CFG_SLP_R = crate::BitReader;
#[doc = "Field `u1_gmac5_axi64_scfg_ram_cfg_slp` writer - SRAM/ROM configuration. SLP: sleep enable, high active, default is low."]
pub type U1_GMAC5_AXI64_SCFG_RAM_CFG_SLP_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O>;
#[doc = "Field `u1_gmac5_axi64_scfg_ram_cfg_sram_config_sd` reader - SRAM/ROM configuration. SD: shutdown enable, high active, default is low."]
pub type U1_GMAC5_AXI64_SCFG_RAM_CFG_SRAM_CONFIG_SD_R = crate::BitReader;
#[doc = "Field `u1_gmac5_axi64_scfg_ram_cfg_sram_config_sd` writer - SRAM/ROM configuration. SD: shutdown enable, high active, default is low."]
pub type U1_GMAC5_AXI64_SCFG_RAM_CFG_SRAM_CONFIG_SD_W<'a, REG, const O: u8> =
    crate::BitWriter<'a, REG, O>;
#[doc = "Field `u1_gmac5_axi64_scfg_ram_cfg_rtsel` reader - SRAM/ROM configuration. RTSEL: timing setting for debug purpose, default is 2'b01."]
pub type U1_GMAC5_AXI64_SCFG_RAM_CFG_RTSEL_R = crate::FieldReader;
#[doc = "Field `u1_gmac5_axi64_scfg_ram_cfg_rtsel` writer - SRAM/ROM configuration. RTSEL: timing setting for debug purpose, default is 2'b01."]
pub type U1_GMAC5_AXI64_SCFG_RAM_CFG_RTSEL_W<'a, REG, const O: u8> =
    crate::FieldWriter<'a, REG, 2, O>;
#[doc = "Field `u1_gmac5_axi64_scfg_ram_cfg_ptsel` reader - SRAM/ROM configuration. PTSEL: timing setting for debug purpose, default is 2'b01."]
pub type U1_GMAC5_AXI64_SCFG_RAM_CFG_PTSEL_R = crate::FieldReader;
#[doc = "Field `u1_gmac5_axi64_scfg_ram_cfg_ptsel` writer - SRAM/ROM configuration. PTSEL: timing setting for debug purpose, default is 2'b01."]
pub type U1_GMAC5_AXI64_SCFG_RAM_CFG_PTSEL_W<'a, REG, const O: u8> =
    crate::FieldWriter<'a, REG, 2, O>;
#[doc = "Field `u1_gmac5_axi64_scfg_ram_cfg_trb` reader - SRAM/ROM configuration. TRB: timing setting for debug purpose, default is 2'b01."]
pub type U1_GMAC5_AXI64_SCFG_RAM_CFG_TRB_R = crate::FieldReader;
#[doc = "Field `u1_gmac5_axi64_scfg_ram_cfg_trb` writer - SRAM/ROM configuration. TRB: timing setting for debug purpose, default is 2'b01."]
pub type U1_GMAC5_AXI64_SCFG_RAM_CFG_TRB_W<'a, REG, const O: u8> =
    crate::FieldWriter<'a, REG, 2, O>;
#[doc = "Field `u1_gmac5_axi64_scfg_ram_cfg_wtsel` reader - SRAM/ROM configuration. WTSEL: timing setting for debug purpose, default is 2'b01."]
pub type U1_GMAC5_AXI64_SCFG_RAM_CFG_WTSEL_R = crate::FieldReader;
#[doc = "Field `u1_gmac5_axi64_scfg_ram_cfg_wtsel` writer - SRAM/ROM configuration. WTSEL: timing setting for debug purpose, default is 2'b01."]
pub type U1_GMAC5_AXI64_SCFG_RAM_CFG_WTSEL_W<'a, REG, const O: u8> =
    crate::FieldWriter<'a, REG, 2, O>;
#[doc = "Field `u1_gmac5_axi64_scfg_ram_cfg_vs` reader - SRAM/ROM configuration. VS: timing setting for debug purpose, default is 1'b1."]
pub type U1_GMAC5_AXI64_SCFG_RAM_CFG_VS_R = crate::BitReader;
#[doc = "Field `u1_gmac5_axi64_scfg_ram_cfg_vs` writer - SRAM/ROM configuration. VS: timing setting for debug purpose, default is 1'b1."]
pub type U1_GMAC5_AXI64_SCFG_RAM_CFG_VS_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O>;
#[doc = "Field `u1_gmac5_axi64_scfg_ram_cfg_vg` reader - SRAM/ROM configuration. VG: timing setting for debug purpose, default is 1'b1."]
pub type U1_GMAC5_AXI64_SCFG_RAM_CFG_VG_R = crate::BitReader;
#[doc = "Field `u1_gmac5_axi64_scfg_ram_cfg_vg` writer - SRAM/ROM configuration. VG: timing setting for debug purpose, default is 1'b1."]
pub type U1_GMAC5_AXI64_SCFG_RAM_CFG_VG_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O>;
impl R {
    #[doc = "Bits 0:18 - u1_can_ctrl_host_if"]
    #[inline(always)]
    pub fn u1_can_ctrl_host_if(&self) -> U1_CAN_CTRL_HOST_IF_R {
        U1_CAN_CTRL_HOST_IF_R::new(self.bits & 0x0007_ffff)
    }
    #[doc = "Bit 19 - SRAM/ROM configuration. SLP: sleep enable, high active, default is low."]
    #[inline(always)]
    pub fn u1_gmac5_axi64_scfg_ram_cfg_slp(&self) -> U1_GMAC5_AXI64_SCFG_RAM_CFG_SLP_R {
        U1_GMAC5_AXI64_SCFG_RAM_CFG_SLP_R::new(((self.bits >> 19) & 1) != 0)
    }
    #[doc = "Bit 20 - SRAM/ROM configuration. SD: shutdown enable, high active, default is low."]
    #[inline(always)]
    pub fn u1_gmac5_axi64_scfg_ram_cfg_sram_config_sd(
        &self,
    ) -> U1_GMAC5_AXI64_SCFG_RAM_CFG_SRAM_CONFIG_SD_R {
        U1_GMAC5_AXI64_SCFG_RAM_CFG_SRAM_CONFIG_SD_R::new(((self.bits >> 20) & 1) != 0)
    }
    #[doc = "Bits 21:22 - SRAM/ROM configuration. RTSEL: timing setting for debug purpose, default is 2'b01."]
    #[inline(always)]
    pub fn u1_gmac5_axi64_scfg_ram_cfg_rtsel(&self) -> U1_GMAC5_AXI64_SCFG_RAM_CFG_RTSEL_R {
        U1_GMAC5_AXI64_SCFG_RAM_CFG_RTSEL_R::new(((self.bits >> 21) & 3) as u8)
    }
    #[doc = "Bits 23:24 - SRAM/ROM configuration. PTSEL: timing setting for debug purpose, default is 2'b01."]
    #[inline(always)]
    pub fn u1_gmac5_axi64_scfg_ram_cfg_ptsel(&self) -> U1_GMAC5_AXI64_SCFG_RAM_CFG_PTSEL_R {
        U1_GMAC5_AXI64_SCFG_RAM_CFG_PTSEL_R::new(((self.bits >> 23) & 3) as u8)
    }
    #[doc = "Bits 25:26 - SRAM/ROM configuration. TRB: timing setting for debug purpose, default is 2'b01."]
    #[inline(always)]
    pub fn u1_gmac5_axi64_scfg_ram_cfg_trb(&self) -> U1_GMAC5_AXI64_SCFG_RAM_CFG_TRB_R {
        U1_GMAC5_AXI64_SCFG_RAM_CFG_TRB_R::new(((self.bits >> 25) & 3) as u8)
    }
    #[doc = "Bits 27:28 - SRAM/ROM configuration. WTSEL: timing setting for debug purpose, default is 2'b01."]
    #[inline(always)]
    pub fn u1_gmac5_axi64_scfg_ram_cfg_wtsel(&self) -> U1_GMAC5_AXI64_SCFG_RAM_CFG_WTSEL_R {
        U1_GMAC5_AXI64_SCFG_RAM_CFG_WTSEL_R::new(((self.bits >> 27) & 3) as u8)
    }
    #[doc = "Bit 29 - SRAM/ROM configuration. VS: timing setting for debug purpose, default is 1'b1."]
    #[inline(always)]
    pub fn u1_gmac5_axi64_scfg_ram_cfg_vs(&self) -> U1_GMAC5_AXI64_SCFG_RAM_CFG_VS_R {
        U1_GMAC5_AXI64_SCFG_RAM_CFG_VS_R::new(((self.bits >> 29) & 1) != 0)
    }
    #[doc = "Bit 30 - SRAM/ROM configuration. VG: timing setting for debug purpose, default is 1'b1."]
    #[inline(always)]
    pub fn u1_gmac5_axi64_scfg_ram_cfg_vg(&self) -> U1_GMAC5_AXI64_SCFG_RAM_CFG_VG_R {
        U1_GMAC5_AXI64_SCFG_RAM_CFG_VG_R::new(((self.bits >> 30) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 19 - SRAM/ROM configuration. SLP: sleep enable, high active, default is low."]
    #[inline(always)]
    #[must_use]
    pub fn u1_gmac5_axi64_scfg_ram_cfg_slp(
        &mut self,
    ) -> U1_GMAC5_AXI64_SCFG_RAM_CFG_SLP_W<SYS_SYSCONSAIF_SYSCFG140_SPEC, 19> {
        U1_GMAC5_AXI64_SCFG_RAM_CFG_SLP_W::new(self)
    }
    #[doc = "Bit 20 - SRAM/ROM configuration. SD: shutdown enable, high active, default is low."]
    #[inline(always)]
    #[must_use]
    pub fn u1_gmac5_axi64_scfg_ram_cfg_sram_config_sd(
        &mut self,
    ) -> U1_GMAC5_AXI64_SCFG_RAM_CFG_SRAM_CONFIG_SD_W<SYS_SYSCONSAIF_SYSCFG140_SPEC, 20> {
        U1_GMAC5_AXI64_SCFG_RAM_CFG_SRAM_CONFIG_SD_W::new(self)
    }
    #[doc = "Bits 21:22 - SRAM/ROM configuration. RTSEL: timing setting for debug purpose, default is 2'b01."]
    #[inline(always)]
    #[must_use]
    pub fn u1_gmac5_axi64_scfg_ram_cfg_rtsel(
        &mut self,
    ) -> U1_GMAC5_AXI64_SCFG_RAM_CFG_RTSEL_W<SYS_SYSCONSAIF_SYSCFG140_SPEC, 21> {
        U1_GMAC5_AXI64_SCFG_RAM_CFG_RTSEL_W::new(self)
    }
    #[doc = "Bits 23:24 - SRAM/ROM configuration. PTSEL: timing setting for debug purpose, default is 2'b01."]
    #[inline(always)]
    #[must_use]
    pub fn u1_gmac5_axi64_scfg_ram_cfg_ptsel(
        &mut self,
    ) -> U1_GMAC5_AXI64_SCFG_RAM_CFG_PTSEL_W<SYS_SYSCONSAIF_SYSCFG140_SPEC, 23> {
        U1_GMAC5_AXI64_SCFG_RAM_CFG_PTSEL_W::new(self)
    }
    #[doc = "Bits 25:26 - SRAM/ROM configuration. TRB: timing setting for debug purpose, default is 2'b01."]
    #[inline(always)]
    #[must_use]
    pub fn u1_gmac5_axi64_scfg_ram_cfg_trb(
        &mut self,
    ) -> U1_GMAC5_AXI64_SCFG_RAM_CFG_TRB_W<SYS_SYSCONSAIF_SYSCFG140_SPEC, 25> {
        U1_GMAC5_AXI64_SCFG_RAM_CFG_TRB_W::new(self)
    }
    #[doc = "Bits 27:28 - SRAM/ROM configuration. WTSEL: timing setting for debug purpose, default is 2'b01."]
    #[inline(always)]
    #[must_use]
    pub fn u1_gmac5_axi64_scfg_ram_cfg_wtsel(
        &mut self,
    ) -> U1_GMAC5_AXI64_SCFG_RAM_CFG_WTSEL_W<SYS_SYSCONSAIF_SYSCFG140_SPEC, 27> {
        U1_GMAC5_AXI64_SCFG_RAM_CFG_WTSEL_W::new(self)
    }
    #[doc = "Bit 29 - SRAM/ROM configuration. VS: timing setting for debug purpose, default is 1'b1."]
    #[inline(always)]
    #[must_use]
    pub fn u1_gmac5_axi64_scfg_ram_cfg_vs(
        &mut self,
    ) -> U1_GMAC5_AXI64_SCFG_RAM_CFG_VS_W<SYS_SYSCONSAIF_SYSCFG140_SPEC, 29> {
        U1_GMAC5_AXI64_SCFG_RAM_CFG_VS_W::new(self)
    }
    #[doc = "Bit 30 - SRAM/ROM configuration. VG: timing setting for debug purpose, default is 1'b1."]
    #[inline(always)]
    #[must_use]
    pub fn u1_gmac5_axi64_scfg_ram_cfg_vg(
        &mut self,
    ) -> U1_GMAC5_AXI64_SCFG_RAM_CFG_VG_W<SYS_SYSCONSAIF_SYSCFG140_SPEC, 30> {
        U1_GMAC5_AXI64_SCFG_RAM_CFG_VG_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.bits = bits;
        self
    }
}
#[doc = "SYS SYSCONSAIF SYSCFG 140\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`sys_sysconsaif_syscfg140::R`](R).  You can [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`sys_sysconsaif_syscfg140::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct SYS_SYSCONSAIF_SYSCFG140_SPEC;
impl crate::RegisterSpec for SYS_SYSCONSAIF_SYSCFG140_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`sys_sysconsaif_syscfg140::R`](R) reader structure"]
impl crate::Readable for SYS_SYSCONSAIF_SYSCFG140_SPEC {}
#[doc = "`write(|w| ..)` method takes [`sys_sysconsaif_syscfg140::W`](W) writer structure"]
impl crate::Writable for SYS_SYSCONSAIF_SYSCFG140_SPEC {
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}