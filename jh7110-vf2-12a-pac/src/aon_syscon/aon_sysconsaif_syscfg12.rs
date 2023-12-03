#[doc = "Register `aon_sysconsaif_syscfg12` reader"]
pub type R = crate::R<AON_SYSCONSAIF_SYSCFG12_SPEC>;
#[doc = "Register `aon_sysconsaif_syscfg12` writer"]
pub type W = crate::W<AON_SYSCONSAIF_SYSCFG12_SPEC>;
#[doc = "Field `u0_boot_ctrl_boot_vector_35_32` reader - u0_boot_ctrl_boot_vector_35_32"]
pub type U0_BOOT_CTRL_BOOT_VECTOR_35_32_R = crate::FieldReader;
#[doc = "Field `gmac5_axi64_scfg_ram_cfg_slp` reader - SRAM/ROM configuration. SLP: sleep enable, high active, default is low."]
pub type GMAC5_AXI64_SCFG_RAM_CFG_SLP_R = crate::BitReader;
#[doc = "Field `gmac5_axi64_scfg_ram_cfg_slp` writer - SRAM/ROM configuration. SLP: sleep enable, high active, default is low."]
pub type GMAC5_AXI64_SCFG_RAM_CFG_SLP_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `gmac5_axi64_scfg_ram_cfg_sd` reader - SRAM/ROM configuration. SD: shutdown enable, high active, default is low."]
pub type GMAC5_AXI64_SCFG_RAM_CFG_SD_R = crate::BitReader;
#[doc = "Field `gmac5_axi64_scfg_ram_cfg_sd` writer - SRAM/ROM configuration. SD: shutdown enable, high active, default is low."]
pub type GMAC5_AXI64_SCFG_RAM_CFG_SD_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `gmac5_axi64_scfg_ram_cfg_rtsel` reader - SRAM/ROM configuration. RTSEL: timing setting for debug purpose, default is 2'b01."]
pub type GMAC5_AXI64_SCFG_RAM_CFG_RTSEL_R = crate::FieldReader;
#[doc = "Field `gmac5_axi64_scfg_ram_cfg_rtsel` writer - SRAM/ROM configuration. RTSEL: timing setting for debug purpose, default is 2'b01."]
pub type GMAC5_AXI64_SCFG_RAM_CFG_RTSEL_W<'a, REG> = crate::FieldWriter<'a, REG, 2>;
#[doc = "Field `gmac5_axi64_scfg_ram_cfg_ptsel` reader - SRAM/ROM configuration. PTSEL: timing setting for debug purpose, default is 2'b01."]
pub type GMAC5_AXI64_SCFG_RAM_CFG_PTSEL_R = crate::FieldReader;
#[doc = "Field `gmac5_axi64_scfg_ram_cfg_ptsel` writer - SRAM/ROM configuration. PTSEL: timing setting for debug purpose, default is 2'b01."]
pub type GMAC5_AXI64_SCFG_RAM_CFG_PTSEL_W<'a, REG> = crate::FieldWriter<'a, REG, 2>;
#[doc = "Field `gmac5_axi64_scfg_ram_cfg_trb` reader - SRAM/ROM configuration. TRB: timing setting for debug purpose, default is 2'b01."]
pub type GMAC5_AXI64_SCFG_RAM_CFG_TRB_R = crate::FieldReader;
#[doc = "Field `gmac5_axi64_scfg_ram_cfg_trb` writer - SRAM/ROM configuration. TRB: timing setting for debug purpose, default is 2'b01."]
pub type GMAC5_AXI64_SCFG_RAM_CFG_TRB_W<'a, REG> = crate::FieldWriter<'a, REG, 2>;
#[doc = "Field `gmac5_axi64_scfg_ram_cfg_wtsel` reader - SRAM/ROM configuration. WTSEL: timing setting for debug purpose, default is 2'b01."]
pub type GMAC5_AXI64_SCFG_RAM_CFG_WTSEL_R = crate::FieldReader;
#[doc = "Field `gmac5_axi64_scfg_ram_cfg_wtsel` writer - SRAM/ROM configuration. WTSEL: timing setting for debug purpose, default is 2'b01."]
pub type GMAC5_AXI64_SCFG_RAM_CFG_WTSEL_W<'a, REG> = crate::FieldWriter<'a, REG, 2>;
#[doc = "Field `gmac5_axi64_scfg_ram_cfg_vs` reader - SRAM/ROM configuration. VS: timing setting for debug purpose, default is 1'b1."]
pub type GMAC5_AXI64_SCFG_RAM_CFG_VS_R = crate::BitReader;
#[doc = "Field `gmac5_axi64_scfg_ram_cfg_vs` writer - SRAM/ROM configuration. VS: timing setting for debug purpose, default is 1'b1."]
pub type GMAC5_AXI64_SCFG_RAM_CFG_VS_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `gmac5_axi64_scfg_ram_cfg_vg` reader - SRAM/ROM configuration. VG: timing setting for debug purpose, default is 1'b1."]
pub type GMAC5_AXI64_SCFG_RAM_CFG_VG_R = crate::BitReader;
#[doc = "Field `gmac5_axi64_scfg_ram_cfg_vg` writer - SRAM/ROM configuration. VG: timing setting for debug purpose, default is 1'b1."]
pub type GMAC5_AXI64_SCFG_RAM_CFG_VG_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `gmac5_axi64_mac_speed_o` reader - gmac5_axi64_mac_speed_o"]
pub type GMAC5_AXI64_MAC_SPEED_O_R = crate::FieldReader;
#[doc = "Field `gmac5_axi64_phy_intf_sel_i` reader - Active PHY Selected. When you have multiple GMAC PHY interfaces in your configuration, this field indicates the sampled value of the PHY selector during reset de-assertion. Values: 0x0 - GMII or MII, 0x1 - RGMII, 0x2 - SGMII, 0x3 - TBI, 0x4 - RMII, 0x5 - RTBI, 0x6 - SMII, 0x7 - REVMII"]
pub type GMAC5_AXI64_PHY_INTF_SEL_I_R = crate::FieldReader;
#[doc = "Field `gmac5_axi64_phy_intf_sel_i` writer - Active PHY Selected. When you have multiple GMAC PHY interfaces in your configuration, this field indicates the sampled value of the PHY selector during reset de-assertion. Values: 0x0 - GMII or MII, 0x1 - RGMII, 0x2 - SGMII, 0x3 - TBI, 0x4 - RMII, 0x5 - RTBI, 0x6 - SMII, 0x7 - REVMII"]
pub type GMAC5_AXI64_PHY_INTF_SEL_I_W<'a, REG> = crate::FieldWriter<'a, REG, 3>;
impl R {
    #[doc = "Bits 0:3 - u0_boot_ctrl_boot_vector_35_32"]
    #[inline(always)]
    pub fn u0_boot_ctrl_boot_vector_35_32(&self) -> U0_BOOT_CTRL_BOOT_VECTOR_35_32_R {
        U0_BOOT_CTRL_BOOT_VECTOR_35_32_R::new((self.bits & 0x0f) as u8)
    }
    #[doc = "Bit 4 - SRAM/ROM configuration. SLP: sleep enable, high active, default is low."]
    #[inline(always)]
    pub fn gmac5_axi64_scfg_ram_cfg_slp(&self) -> GMAC5_AXI64_SCFG_RAM_CFG_SLP_R {
        GMAC5_AXI64_SCFG_RAM_CFG_SLP_R::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bit 5 - SRAM/ROM configuration. SD: shutdown enable, high active, default is low."]
    #[inline(always)]
    pub fn gmac5_axi64_scfg_ram_cfg_sd(&self) -> GMAC5_AXI64_SCFG_RAM_CFG_SD_R {
        GMAC5_AXI64_SCFG_RAM_CFG_SD_R::new(((self.bits >> 5) & 1) != 0)
    }
    #[doc = "Bits 6:7 - SRAM/ROM configuration. RTSEL: timing setting for debug purpose, default is 2'b01."]
    #[inline(always)]
    pub fn gmac5_axi64_scfg_ram_cfg_rtsel(&self) -> GMAC5_AXI64_SCFG_RAM_CFG_RTSEL_R {
        GMAC5_AXI64_SCFG_RAM_CFG_RTSEL_R::new(((self.bits >> 6) & 3) as u8)
    }
    #[doc = "Bits 8:9 - SRAM/ROM configuration. PTSEL: timing setting for debug purpose, default is 2'b01."]
    #[inline(always)]
    pub fn gmac5_axi64_scfg_ram_cfg_ptsel(&self) -> GMAC5_AXI64_SCFG_RAM_CFG_PTSEL_R {
        GMAC5_AXI64_SCFG_RAM_CFG_PTSEL_R::new(((self.bits >> 8) & 3) as u8)
    }
    #[doc = "Bits 10:11 - SRAM/ROM configuration. TRB: timing setting for debug purpose, default is 2'b01."]
    #[inline(always)]
    pub fn gmac5_axi64_scfg_ram_cfg_trb(&self) -> GMAC5_AXI64_SCFG_RAM_CFG_TRB_R {
        GMAC5_AXI64_SCFG_RAM_CFG_TRB_R::new(((self.bits >> 10) & 3) as u8)
    }
    #[doc = "Bits 12:13 - SRAM/ROM configuration. WTSEL: timing setting for debug purpose, default is 2'b01."]
    #[inline(always)]
    pub fn gmac5_axi64_scfg_ram_cfg_wtsel(&self) -> GMAC5_AXI64_SCFG_RAM_CFG_WTSEL_R {
        GMAC5_AXI64_SCFG_RAM_CFG_WTSEL_R::new(((self.bits >> 12) & 3) as u8)
    }
    #[doc = "Bit 14 - SRAM/ROM configuration. VS: timing setting for debug purpose, default is 1'b1."]
    #[inline(always)]
    pub fn gmac5_axi64_scfg_ram_cfg_vs(&self) -> GMAC5_AXI64_SCFG_RAM_CFG_VS_R {
        GMAC5_AXI64_SCFG_RAM_CFG_VS_R::new(((self.bits >> 14) & 1) != 0)
    }
    #[doc = "Bit 15 - SRAM/ROM configuration. VG: timing setting for debug purpose, default is 1'b1."]
    #[inline(always)]
    pub fn gmac5_axi64_scfg_ram_cfg_vg(&self) -> GMAC5_AXI64_SCFG_RAM_CFG_VG_R {
        GMAC5_AXI64_SCFG_RAM_CFG_VG_R::new(((self.bits >> 15) & 1) != 0)
    }
    #[doc = "Bits 16:17 - gmac5_axi64_mac_speed_o"]
    #[inline(always)]
    pub fn gmac5_axi64_mac_speed_o(&self) -> GMAC5_AXI64_MAC_SPEED_O_R {
        GMAC5_AXI64_MAC_SPEED_O_R::new(((self.bits >> 16) & 3) as u8)
    }
    #[doc = "Bits 18:20 - Active PHY Selected. When you have multiple GMAC PHY interfaces in your configuration, this field indicates the sampled value of the PHY selector during reset de-assertion. Values: 0x0 - GMII or MII, 0x1 - RGMII, 0x2 - SGMII, 0x3 - TBI, 0x4 - RMII, 0x5 - RTBI, 0x6 - SMII, 0x7 - REVMII"]
    #[inline(always)]
    pub fn gmac5_axi64_phy_intf_sel_i(&self) -> GMAC5_AXI64_PHY_INTF_SEL_I_R {
        GMAC5_AXI64_PHY_INTF_SEL_I_R::new(((self.bits >> 18) & 7) as u8)
    }
}
impl W {
    #[doc = "Bit 4 - SRAM/ROM configuration. SLP: sleep enable, high active, default is low."]
    #[inline(always)]
    #[must_use]
    pub fn gmac5_axi64_scfg_ram_cfg_slp(
        &mut self,
    ) -> GMAC5_AXI64_SCFG_RAM_CFG_SLP_W<AON_SYSCONSAIF_SYSCFG12_SPEC> {
        GMAC5_AXI64_SCFG_RAM_CFG_SLP_W::new(self, 4)
    }
    #[doc = "Bit 5 - SRAM/ROM configuration. SD: shutdown enable, high active, default is low."]
    #[inline(always)]
    #[must_use]
    pub fn gmac5_axi64_scfg_ram_cfg_sd(
        &mut self,
    ) -> GMAC5_AXI64_SCFG_RAM_CFG_SD_W<AON_SYSCONSAIF_SYSCFG12_SPEC> {
        GMAC5_AXI64_SCFG_RAM_CFG_SD_W::new(self, 5)
    }
    #[doc = "Bits 6:7 - SRAM/ROM configuration. RTSEL: timing setting for debug purpose, default is 2'b01."]
    #[inline(always)]
    #[must_use]
    pub fn gmac5_axi64_scfg_ram_cfg_rtsel(
        &mut self,
    ) -> GMAC5_AXI64_SCFG_RAM_CFG_RTSEL_W<AON_SYSCONSAIF_SYSCFG12_SPEC> {
        GMAC5_AXI64_SCFG_RAM_CFG_RTSEL_W::new(self, 6)
    }
    #[doc = "Bits 8:9 - SRAM/ROM configuration. PTSEL: timing setting for debug purpose, default is 2'b01."]
    #[inline(always)]
    #[must_use]
    pub fn gmac5_axi64_scfg_ram_cfg_ptsel(
        &mut self,
    ) -> GMAC5_AXI64_SCFG_RAM_CFG_PTSEL_W<AON_SYSCONSAIF_SYSCFG12_SPEC> {
        GMAC5_AXI64_SCFG_RAM_CFG_PTSEL_W::new(self, 8)
    }
    #[doc = "Bits 10:11 - SRAM/ROM configuration. TRB: timing setting for debug purpose, default is 2'b01."]
    #[inline(always)]
    #[must_use]
    pub fn gmac5_axi64_scfg_ram_cfg_trb(
        &mut self,
    ) -> GMAC5_AXI64_SCFG_RAM_CFG_TRB_W<AON_SYSCONSAIF_SYSCFG12_SPEC> {
        GMAC5_AXI64_SCFG_RAM_CFG_TRB_W::new(self, 10)
    }
    #[doc = "Bits 12:13 - SRAM/ROM configuration. WTSEL: timing setting for debug purpose, default is 2'b01."]
    #[inline(always)]
    #[must_use]
    pub fn gmac5_axi64_scfg_ram_cfg_wtsel(
        &mut self,
    ) -> GMAC5_AXI64_SCFG_RAM_CFG_WTSEL_W<AON_SYSCONSAIF_SYSCFG12_SPEC> {
        GMAC5_AXI64_SCFG_RAM_CFG_WTSEL_W::new(self, 12)
    }
    #[doc = "Bit 14 - SRAM/ROM configuration. VS: timing setting for debug purpose, default is 1'b1."]
    #[inline(always)]
    #[must_use]
    pub fn gmac5_axi64_scfg_ram_cfg_vs(
        &mut self,
    ) -> GMAC5_AXI64_SCFG_RAM_CFG_VS_W<AON_SYSCONSAIF_SYSCFG12_SPEC> {
        GMAC5_AXI64_SCFG_RAM_CFG_VS_W::new(self, 14)
    }
    #[doc = "Bit 15 - SRAM/ROM configuration. VG: timing setting for debug purpose, default is 1'b1."]
    #[inline(always)]
    #[must_use]
    pub fn gmac5_axi64_scfg_ram_cfg_vg(
        &mut self,
    ) -> GMAC5_AXI64_SCFG_RAM_CFG_VG_W<AON_SYSCONSAIF_SYSCFG12_SPEC> {
        GMAC5_AXI64_SCFG_RAM_CFG_VG_W::new(self, 15)
    }
    #[doc = "Bits 18:20 - Active PHY Selected. When you have multiple GMAC PHY interfaces in your configuration, this field indicates the sampled value of the PHY selector during reset de-assertion. Values: 0x0 - GMII or MII, 0x1 - RGMII, 0x2 - SGMII, 0x3 - TBI, 0x4 - RMII, 0x5 - RTBI, 0x6 - SMII, 0x7 - REVMII"]
    #[inline(always)]
    #[must_use]
    pub fn gmac5_axi64_phy_intf_sel_i(
        &mut self,
    ) -> GMAC5_AXI64_PHY_INTF_SEL_I_W<AON_SYSCONSAIF_SYSCFG12_SPEC> {
        GMAC5_AXI64_PHY_INTF_SEL_I_W::new(self, 18)
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
#[doc = "AON SYSCONSAIF SYSCFG 12\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`aon_sysconsaif_syscfg12::R`](R).  You can [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`aon_sysconsaif_syscfg12::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct AON_SYSCONSAIF_SYSCFG12_SPEC;
impl crate::RegisterSpec for AON_SYSCONSAIF_SYSCFG12_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`aon_sysconsaif_syscfg12::R`](R) reader structure"]
impl crate::Readable for AON_SYSCONSAIF_SYSCFG12_SPEC {}
#[doc = "`write(|w| ..)` method takes [`aon_sysconsaif_syscfg12::W`](W) writer structure"]
impl crate::Writable for AON_SYSCONSAIF_SYSCFG12_SPEC {
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
