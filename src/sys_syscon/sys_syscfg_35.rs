#[doc = "Register `sys_syscfg_35` reader"]
pub type R = crate::R<SYS_SYSCFG_35_SPEC>;
#[doc = "Register `sys_syscfg_35` writer"]
pub type W = crate::W<SYS_SYSCFG_35_SPEC>;
#[doc = "Field `can_ctrl_host_if_1` reader - can_ctrl_host_if_1"]
pub type CAN_CTRL_HOST_IF_1_R = crate::FieldReader<u32>;
#[doc = "Field `u1_gmac5_axi64_scfg_ram_cfg_slp` reader - SRAM/ROM configuration. SLP: sleep enable, high active, default is low."]
pub type U1_GMAC5_AXI64_SCFG_RAM_CFG_SLP_R = crate::BitReader;
#[doc = "Field `u1_gmac5_axi64_scfg_ram_cfg_slp` writer - SRAM/ROM configuration. SLP: sleep enable, high active, default is low."]
pub type U1_GMAC5_AXI64_SCFG_RAM_CFG_SLP_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `u1_gmac5_axi64_scfg_ram_cfg_sd` reader - SRAM/ROM configuration. SD: shutdown enable, high active, default is low."]
pub type U1_GMAC5_AXI64_SCFG_RAM_CFG_SD_R = crate::BitReader;
#[doc = "Field `u1_gmac5_axi64_scfg_ram_cfg_sd` writer - SRAM/ROM configuration. SD: shutdown enable, high active, default is low."]
pub type U1_GMAC5_AXI64_SCFG_RAM_CFG_SD_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `u1_gmac5_axi64_scfg_ram_cfg_rtsel` reader - SRAM/ROM configuration. RTSEL: timing setting for debug purpose, default is 2'b01."]
pub type U1_GMAC5_AXI64_SCFG_RAM_CFG_RTSEL_R = crate::FieldReader;
#[doc = "Field `u1_gmac5_axi64_scfg_ram_cfg_rtsel` writer - SRAM/ROM configuration. RTSEL: timing setting for debug purpose, default is 2'b01."]
pub type U1_GMAC5_AXI64_SCFG_RAM_CFG_RTSEL_W<'a, REG> = crate::FieldWriter<'a, REG, 2>;
#[doc = "Field `u1_gmac5_axi64_scfg_ram_cfg_ptsel` reader - SRAM/ROM configuration. PTSEL: timing setting for debug purpose, default is 2'b01."]
pub type U1_GMAC5_AXI64_SCFG_RAM_CFG_PTSEL_R = crate::FieldReader;
#[doc = "Field `u1_gmac5_axi64_scfg_ram_cfg_ptsel` writer - SRAM/ROM configuration. PTSEL: timing setting for debug purpose, default is 2'b01."]
pub type U1_GMAC5_AXI64_SCFG_RAM_CFG_PTSEL_W<'a, REG> = crate::FieldWriter<'a, REG, 2>;
#[doc = "Field `u1_gmac5_axi64_scfg_ram_cfg_trb` reader - SRAM/ROM configuration. TRB: timing setting for debug purpose, default is 2'b01."]
pub type U1_GMAC5_AXI64_SCFG_RAM_CFG_TRB_R = crate::FieldReader;
#[doc = "Field `u1_gmac5_axi64_scfg_ram_cfg_trb` writer - SRAM/ROM configuration. TRB: timing setting for debug purpose, default is 2'b01."]
pub type U1_GMAC5_AXI64_SCFG_RAM_CFG_TRB_W<'a, REG> = crate::FieldWriter<'a, REG, 2>;
#[doc = "Field `u1_gmac5_axi64_scfg_ram_cfg_wtsel` reader - SRAM/ROM configuration. WTSEL: timing setting for debug purpose, default is 2'b01."]
pub type U1_GMAC5_AXI64_SCFG_RAM_CFG_WTSEL_R = crate::FieldReader;
#[doc = "Field `u1_gmac5_axi64_scfg_ram_cfg_wtsel` writer - SRAM/ROM configuration. WTSEL: timing setting for debug purpose, default is 2'b01."]
pub type U1_GMAC5_AXI64_SCFG_RAM_CFG_WTSEL_W<'a, REG> = crate::FieldWriter<'a, REG, 2>;
#[doc = "Field `u1_gmac5_axi64_scfg_ram_cfg_vs` reader - SRAM/ROM configuration. VS: timing setting for debug purpose, default is 1'b1."]
pub type U1_GMAC5_AXI64_SCFG_RAM_CFG_VS_R = crate::BitReader;
#[doc = "Field `u1_gmac5_axi64_scfg_ram_cfg_vs` writer - SRAM/ROM configuration. VS: timing setting for debug purpose, default is 1'b1."]
pub type U1_GMAC5_AXI64_SCFG_RAM_CFG_VS_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `u1_gmac5_axi64_scfg_ram_cfg_vg` reader - SRAM/ROM configuration. VG: timing setting for debug purpose, default is 1'b1."]
pub type U1_GMAC5_AXI64_SCFG_RAM_CFG_VG_R = crate::BitReader;
#[doc = "Field `u1_gmac5_axi64_scfg_ram_cfg_vg` writer - SRAM/ROM configuration. VG: timing setting for debug purpose, default is 1'b1."]
pub type U1_GMAC5_AXI64_SCFG_RAM_CFG_VG_W<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bits 0:18 - can_ctrl_host_if_1"]
    #[inline(always)]
    pub fn can_ctrl_host_if_1(&self) -> CAN_CTRL_HOST_IF_1_R {
        CAN_CTRL_HOST_IF_1_R::new(self.bits & 0x0007_ffff)
    }
    #[doc = "Bit 19 - SRAM/ROM configuration. SLP: sleep enable, high active, default is low."]
    #[inline(always)]
    pub fn u1_gmac5_axi64_scfg_ram_cfg_slp(&self) -> U1_GMAC5_AXI64_SCFG_RAM_CFG_SLP_R {
        U1_GMAC5_AXI64_SCFG_RAM_CFG_SLP_R::new(((self.bits >> 19) & 1) != 0)
    }
    #[doc = "Bit 20 - SRAM/ROM configuration. SD: shutdown enable, high active, default is low."]
    #[inline(always)]
    pub fn u1_gmac5_axi64_scfg_ram_cfg_sd(&self) -> U1_GMAC5_AXI64_SCFG_RAM_CFG_SD_R {
        U1_GMAC5_AXI64_SCFG_RAM_CFG_SD_R::new(((self.bits >> 20) & 1) != 0)
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
    ) -> U1_GMAC5_AXI64_SCFG_RAM_CFG_SLP_W<SYS_SYSCFG_35_SPEC> {
        U1_GMAC5_AXI64_SCFG_RAM_CFG_SLP_W::new(self, 19)
    }
    #[doc = "Bit 20 - SRAM/ROM configuration. SD: shutdown enable, high active, default is low."]
    #[inline(always)]
    #[must_use]
    pub fn u1_gmac5_axi64_scfg_ram_cfg_sd(
        &mut self,
    ) -> U1_GMAC5_AXI64_SCFG_RAM_CFG_SD_W<SYS_SYSCFG_35_SPEC> {
        U1_GMAC5_AXI64_SCFG_RAM_CFG_SD_W::new(self, 20)
    }
    #[doc = "Bits 21:22 - SRAM/ROM configuration. RTSEL: timing setting for debug purpose, default is 2'b01."]
    #[inline(always)]
    #[must_use]
    pub fn u1_gmac5_axi64_scfg_ram_cfg_rtsel(
        &mut self,
    ) -> U1_GMAC5_AXI64_SCFG_RAM_CFG_RTSEL_W<SYS_SYSCFG_35_SPEC> {
        U1_GMAC5_AXI64_SCFG_RAM_CFG_RTSEL_W::new(self, 21)
    }
    #[doc = "Bits 23:24 - SRAM/ROM configuration. PTSEL: timing setting for debug purpose, default is 2'b01."]
    #[inline(always)]
    #[must_use]
    pub fn u1_gmac5_axi64_scfg_ram_cfg_ptsel(
        &mut self,
    ) -> U1_GMAC5_AXI64_SCFG_RAM_CFG_PTSEL_W<SYS_SYSCFG_35_SPEC> {
        U1_GMAC5_AXI64_SCFG_RAM_CFG_PTSEL_W::new(self, 23)
    }
    #[doc = "Bits 25:26 - SRAM/ROM configuration. TRB: timing setting for debug purpose, default is 2'b01."]
    #[inline(always)]
    #[must_use]
    pub fn u1_gmac5_axi64_scfg_ram_cfg_trb(
        &mut self,
    ) -> U1_GMAC5_AXI64_SCFG_RAM_CFG_TRB_W<SYS_SYSCFG_35_SPEC> {
        U1_GMAC5_AXI64_SCFG_RAM_CFG_TRB_W::new(self, 25)
    }
    #[doc = "Bits 27:28 - SRAM/ROM configuration. WTSEL: timing setting for debug purpose, default is 2'b01."]
    #[inline(always)]
    #[must_use]
    pub fn u1_gmac5_axi64_scfg_ram_cfg_wtsel(
        &mut self,
    ) -> U1_GMAC5_AXI64_SCFG_RAM_CFG_WTSEL_W<SYS_SYSCFG_35_SPEC> {
        U1_GMAC5_AXI64_SCFG_RAM_CFG_WTSEL_W::new(self, 27)
    }
    #[doc = "Bit 29 - SRAM/ROM configuration. VS: timing setting for debug purpose, default is 1'b1."]
    #[inline(always)]
    #[must_use]
    pub fn u1_gmac5_axi64_scfg_ram_cfg_vs(
        &mut self,
    ) -> U1_GMAC5_AXI64_SCFG_RAM_CFG_VS_W<SYS_SYSCFG_35_SPEC> {
        U1_GMAC5_AXI64_SCFG_RAM_CFG_VS_W::new(self, 29)
    }
    #[doc = "Bit 30 - SRAM/ROM configuration. VG: timing setting for debug purpose, default is 1'b1."]
    #[inline(always)]
    #[must_use]
    pub fn u1_gmac5_axi64_scfg_ram_cfg_vg(
        &mut self,
    ) -> U1_GMAC5_AXI64_SCFG_RAM_CFG_VG_W<SYS_SYSCFG_35_SPEC> {
        U1_GMAC5_AXI64_SCFG_RAM_CFG_VG_W::new(self, 30)
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
#[doc = "SYS SYSCONSAIF SYSCFG 140\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`sys_syscfg_35::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`sys_syscfg_35::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct SYS_SYSCFG_35_SPEC;
impl crate::RegisterSpec for SYS_SYSCFG_35_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`sys_syscfg_35::R`](R) reader structure"]
impl crate::Readable for SYS_SYSCFG_35_SPEC {}
#[doc = "`write(|w| ..)` method takes [`sys_syscfg_35::W`](W) writer structure"]
impl crate::Writable for SYS_SYSCFG_35_SPEC {
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets sys_syscfg_35 to value 0"]
impl crate::Resettable for SYS_SYSCFG_35_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
