#[doc = "Register `sys_syscfg_6` reader"]
pub type R = crate::R<SYS_SYSCFG_6_SPEC>;
#[doc = "Register `sys_syscfg_6` writer"]
pub type W = crate::W<SYS_SYSCFG_6_SPEC>;
#[doc = "Field `sdio_m_hbig_endian` reader - AHB master bus interface endianess: 1: Big-endian AHB bus interface, 0: Little-endian AHB bus interface"]
pub type SDIO_M_HBIG_ENDIAN_R = crate::BitReader;
#[doc = "Field `sdio_m_hbig_endian` writer - AHB master bus interface endianess: 1: Big-endian AHB bus interface, 0: Little-endian AHB bus interface"]
pub type SDIO_M_HBIG_ENDIAN_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `i2srx_adc_en` reader - i2srx_adc_en"]
pub type I2SRX_ADC_EN_R = crate::BitReader;
#[doc = "Field `i2srx_adc_en` writer - i2srx_adc_en"]
pub type I2SRX_ADC_EN_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `intmem_rom_sram_scfg_disable_rom` reader - intmem_rom_sram_scfg_disable_rom"]
pub type INTMEM_ROM_SRAM_SCFG_DISABLE_ROM_R = crate::BitReader;
#[doc = "Field `intmem_rom_sram_scfg_disable_rom` writer - intmem_rom_sram_scfg_disable_rom"]
pub type INTMEM_ROM_SRAM_SCFG_DISABLE_ROM_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `u0_intmem_rom_sram_sram_config_slp` reader - SRAM/ROM configuration. SLP: sleep enable, high active, default is low."]
pub type U0_INTMEM_ROM_SRAM_SRAM_CONFIG_SLP_R = crate::BitReader;
#[doc = "Field `u0_intmem_rom_sram_sram_config_slp` writer - SRAM/ROM configuration. SLP: sleep enable, high active, default is low."]
pub type U0_INTMEM_ROM_SRAM_SRAM_CONFIG_SLP_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `u0_intmem_rom_sram_sram_config_sd` reader - SRAM/ROM configuration. SD: shutdown enable, high active, default is low."]
pub type U0_INTMEM_ROM_SRAM_SRAM_CONFIG_SD_R = crate::BitReader;
#[doc = "Field `u0_intmem_rom_sram_sram_config_sd` writer - SRAM/ROM configuration. SD: shutdown enable, high active, default is low."]
pub type U0_INTMEM_ROM_SRAM_SRAM_CONFIG_SD_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `u0_intmem_rom_sram_sram_config_rtsel` reader - SRAM/ROM configuration. RTSEL: timing setting for debug purpose, default is 2'b01."]
pub type U0_INTMEM_ROM_SRAM_SRAM_CONFIG_RTSEL_R = crate::FieldReader;
#[doc = "Field `u0_intmem_rom_sram_sram_config_rtsel` writer - SRAM/ROM configuration. RTSEL: timing setting for debug purpose, default is 2'b01."]
pub type U0_INTMEM_ROM_SRAM_SRAM_CONFIG_RTSEL_W<'a, REG> = crate::FieldWriter<'a, REG, 2>;
#[doc = "Field `u0_intmem_rom_sram_sram_config_ptsel` reader - SRAM/ROM configuration. PTSEL: timing setting for debug purpose, default is 2'b01."]
pub type U0_INTMEM_ROM_SRAM_SRAM_CONFIG_PTSEL_R = crate::FieldReader;
#[doc = "Field `u0_intmem_rom_sram_sram_config_ptsel` writer - SRAM/ROM configuration. PTSEL: timing setting for debug purpose, default is 2'b01."]
pub type U0_INTMEM_ROM_SRAM_SRAM_CONFIG_PTSEL_W<'a, REG> = crate::FieldWriter<'a, REG, 2>;
#[doc = "Field `u0_intmem_rom_sram_sram_config_trb` reader - SRAM/ROM configuration. TRB: timing setting for debug purpose, default is 2'b01."]
pub type U0_INTMEM_ROM_SRAM_SRAM_CONFIG_TRB_R = crate::FieldReader;
#[doc = "Field `u0_intmem_rom_sram_sram_config_trb` writer - SRAM/ROM configuration. TRB: timing setting for debug purpose, default is 2'b01."]
pub type U0_INTMEM_ROM_SRAM_SRAM_CONFIG_TRB_W<'a, REG> = crate::FieldWriter<'a, REG, 2>;
#[doc = "Field `u0_intmem_rom_sram_sram_config_wtsel` reader - SRAM/ROM configuration. WTSEL: timing setting for debug purpose, default is 2'b01."]
pub type U0_INTMEM_ROM_SRAM_SRAM_CONFIG_WTSEL_R = crate::FieldReader;
#[doc = "Field `u0_intmem_rom_sram_sram_config_wtsel` writer - SRAM/ROM configuration. WTSEL: timing setting for debug purpose, default is 2'b01."]
pub type U0_INTMEM_ROM_SRAM_SRAM_CONFIG_WTSEL_W<'a, REG> = crate::FieldWriter<'a, REG, 2>;
#[doc = "Field `u0_intmem_rom_sram_sram_config_vs` reader - SRAM/ROM configuration. VS: timing setting for debug purpose, default is 1'b1."]
pub type U0_INTMEM_ROM_SRAM_SRAM_CONFIG_VS_R = crate::BitReader;
#[doc = "Field `u0_intmem_rom_sram_sram_config_vs` writer - SRAM/ROM configuration. VS: timing setting for debug purpose, default is 1'b1."]
pub type U0_INTMEM_ROM_SRAM_SRAM_CONFIG_VS_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `u0_intmem_rom_sram_sram_config_vg` reader - SRAM/ROM configuration. VG: timing setting for debug purpose, default is 1'b1."]
pub type U0_INTMEM_ROM_SRAM_SRAM_CONFIG_VG_R = crate::BitReader;
#[doc = "Field `u0_intmem_rom_sram_sram_config_vg` writer - SRAM/ROM configuration. VG: timing setting for debug purpose, default is 1'b1."]
pub type U0_INTMEM_ROM_SRAM_SRAM_CONFIG_VG_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `jtag_daisy_chain_en_0` reader - jtag_daisy_chain_en_0"]
pub type JTAG_DAISY_CHAIN_EN_0_R = crate::BitReader;
#[doc = "Field `jtag_daisy_chain_en_0` writer - jtag_daisy_chain_en_0"]
pub type JTAG_DAISY_CHAIN_EN_0_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `jtag_daisy_chain_en_1` reader - jtag_daisy_chain_en_1"]
pub type JTAG_DAISY_CHAIN_EN_1_R = crate::BitReader;
#[doc = "Field `jtag_daisy_chain_en_1` writer - jtag_daisy_chain_en_1"]
pub type JTAG_DAISY_CHAIN_EN_1_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `pdrstn_usbpipe_plugen` reader - pdrstn_usbpipe_plugen"]
pub type PDRSTN_USBPIPE_PLUGEN_R = crate::BitReader;
#[doc = "Field `pdrstn_usbpipe_plugen` writer - pdrstn_usbpipe_plugen"]
pub type PDRSTN_USBPIPE_PLUGEN_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `pll0_cpi_bias` reader - pll0_cpi_bias"]
pub type PLL0_CPI_BIAS_R = crate::FieldReader;
#[doc = "Field `pll0_cpi_bias` writer - pll0_cpi_bias"]
pub type PLL0_CPI_BIAS_W<'a, REG> = crate::FieldWriter<'a, REG, 3>;
#[doc = "Field `pll0_cpp_bias` reader - pll0_cpp_bias"]
pub type PLL0_CPP_BIAS_R = crate::FieldReader;
#[doc = "Field `pll0_cpp_bias` writer - pll0_cpp_bias"]
pub type PLL0_CPP_BIAS_W<'a, REG> = crate::FieldWriter<'a, REG, 3>;
#[doc = "Field `pll0_dacpd` reader - pll0_dacpd"]
pub type PLL0_DACPD_R = crate::BitReader;
#[doc = "Field `pll0_dacpd` writer - pll0_dacpd"]
pub type PLL0_DACPD_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `pll0_dsmpd` reader - pll0_dsmpd"]
pub type PLL0_DSMPD_R = crate::BitReader;
#[doc = "Field `pll0_dsmpd` writer - pll0_dsmpd"]
pub type PLL0_DSMPD_W<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bit 0 - AHB master bus interface endianess: 1: Big-endian AHB bus interface, 0: Little-endian AHB bus interface"]
    #[inline(always)]
    pub fn sdio_m_hbig_endian(&self) -> SDIO_M_HBIG_ENDIAN_R {
        SDIO_M_HBIG_ENDIAN_R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - i2srx_adc_en"]
    #[inline(always)]
    pub fn i2srx_adc_en(&self) -> I2SRX_ADC_EN_R {
        I2SRX_ADC_EN_R::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - intmem_rom_sram_scfg_disable_rom"]
    #[inline(always)]
    pub fn intmem_rom_sram_scfg_disable_rom(&self) -> INTMEM_ROM_SRAM_SCFG_DISABLE_ROM_R {
        INTMEM_ROM_SRAM_SCFG_DISABLE_ROM_R::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3 - SRAM/ROM configuration. SLP: sleep enable, high active, default is low."]
    #[inline(always)]
    pub fn u0_intmem_rom_sram_sram_config_slp(&self) -> U0_INTMEM_ROM_SRAM_SRAM_CONFIG_SLP_R {
        U0_INTMEM_ROM_SRAM_SRAM_CONFIG_SLP_R::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bit 4 - SRAM/ROM configuration. SD: shutdown enable, high active, default is low."]
    #[inline(always)]
    pub fn u0_intmem_rom_sram_sram_config_sd(&self) -> U0_INTMEM_ROM_SRAM_SRAM_CONFIG_SD_R {
        U0_INTMEM_ROM_SRAM_SRAM_CONFIG_SD_R::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bits 5:6 - SRAM/ROM configuration. RTSEL: timing setting for debug purpose, default is 2'b01."]
    #[inline(always)]
    pub fn u0_intmem_rom_sram_sram_config_rtsel(&self) -> U0_INTMEM_ROM_SRAM_SRAM_CONFIG_RTSEL_R {
        U0_INTMEM_ROM_SRAM_SRAM_CONFIG_RTSEL_R::new(((self.bits >> 5) & 3) as u8)
    }
    #[doc = "Bits 7:8 - SRAM/ROM configuration. PTSEL: timing setting for debug purpose, default is 2'b01."]
    #[inline(always)]
    pub fn u0_intmem_rom_sram_sram_config_ptsel(&self) -> U0_INTMEM_ROM_SRAM_SRAM_CONFIG_PTSEL_R {
        U0_INTMEM_ROM_SRAM_SRAM_CONFIG_PTSEL_R::new(((self.bits >> 7) & 3) as u8)
    }
    #[doc = "Bits 9:10 - SRAM/ROM configuration. TRB: timing setting for debug purpose, default is 2'b01."]
    #[inline(always)]
    pub fn u0_intmem_rom_sram_sram_config_trb(&self) -> U0_INTMEM_ROM_SRAM_SRAM_CONFIG_TRB_R {
        U0_INTMEM_ROM_SRAM_SRAM_CONFIG_TRB_R::new(((self.bits >> 9) & 3) as u8)
    }
    #[doc = "Bits 11:12 - SRAM/ROM configuration. WTSEL: timing setting for debug purpose, default is 2'b01."]
    #[inline(always)]
    pub fn u0_intmem_rom_sram_sram_config_wtsel(&self) -> U0_INTMEM_ROM_SRAM_SRAM_CONFIG_WTSEL_R {
        U0_INTMEM_ROM_SRAM_SRAM_CONFIG_WTSEL_R::new(((self.bits >> 11) & 3) as u8)
    }
    #[doc = "Bit 13 - SRAM/ROM configuration. VS: timing setting for debug purpose, default is 1'b1."]
    #[inline(always)]
    pub fn u0_intmem_rom_sram_sram_config_vs(&self) -> U0_INTMEM_ROM_SRAM_SRAM_CONFIG_VS_R {
        U0_INTMEM_ROM_SRAM_SRAM_CONFIG_VS_R::new(((self.bits >> 13) & 1) != 0)
    }
    #[doc = "Bit 14 - SRAM/ROM configuration. VG: timing setting for debug purpose, default is 1'b1."]
    #[inline(always)]
    pub fn u0_intmem_rom_sram_sram_config_vg(&self) -> U0_INTMEM_ROM_SRAM_SRAM_CONFIG_VG_R {
        U0_INTMEM_ROM_SRAM_SRAM_CONFIG_VG_R::new(((self.bits >> 14) & 1) != 0)
    }
    #[doc = "Bit 15 - jtag_daisy_chain_en_0"]
    #[inline(always)]
    pub fn jtag_daisy_chain_en_0(&self) -> JTAG_DAISY_CHAIN_EN_0_R {
        JTAG_DAISY_CHAIN_EN_0_R::new(((self.bits >> 15) & 1) != 0)
    }
    #[doc = "Bit 16 - jtag_daisy_chain_en_1"]
    #[inline(always)]
    pub fn jtag_daisy_chain_en_1(&self) -> JTAG_DAISY_CHAIN_EN_1_R {
        JTAG_DAISY_CHAIN_EN_1_R::new(((self.bits >> 16) & 1) != 0)
    }
    #[doc = "Bit 17 - pdrstn_usbpipe_plugen"]
    #[inline(always)]
    pub fn pdrstn_usbpipe_plugen(&self) -> PDRSTN_USBPIPE_PLUGEN_R {
        PDRSTN_USBPIPE_PLUGEN_R::new(((self.bits >> 17) & 1) != 0)
    }
    #[doc = "Bits 18:20 - pll0_cpi_bias"]
    #[inline(always)]
    pub fn pll0_cpi_bias(&self) -> PLL0_CPI_BIAS_R {
        PLL0_CPI_BIAS_R::new(((self.bits >> 18) & 7) as u8)
    }
    #[doc = "Bits 21:23 - pll0_cpp_bias"]
    #[inline(always)]
    pub fn pll0_cpp_bias(&self) -> PLL0_CPP_BIAS_R {
        PLL0_CPP_BIAS_R::new(((self.bits >> 21) & 7) as u8)
    }
    #[doc = "Bit 24 - pll0_dacpd"]
    #[inline(always)]
    pub fn pll0_dacpd(&self) -> PLL0_DACPD_R {
        PLL0_DACPD_R::new(((self.bits >> 24) & 1) != 0)
    }
    #[doc = "Bit 25 - pll0_dsmpd"]
    #[inline(always)]
    pub fn pll0_dsmpd(&self) -> PLL0_DSMPD_R {
        PLL0_DSMPD_R::new(((self.bits >> 25) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - AHB master bus interface endianess: 1: Big-endian AHB bus interface, 0: Little-endian AHB bus interface"]
    #[inline(always)]
    #[must_use]
    pub fn sdio_m_hbig_endian(&mut self) -> SDIO_M_HBIG_ENDIAN_W<SYS_SYSCFG_6_SPEC> {
        SDIO_M_HBIG_ENDIAN_W::new(self, 0)
    }
    #[doc = "Bit 1 - i2srx_adc_en"]
    #[inline(always)]
    #[must_use]
    pub fn i2srx_adc_en(&mut self) -> I2SRX_ADC_EN_W<SYS_SYSCFG_6_SPEC> {
        I2SRX_ADC_EN_W::new(self, 1)
    }
    #[doc = "Bit 2 - intmem_rom_sram_scfg_disable_rom"]
    #[inline(always)]
    #[must_use]
    pub fn intmem_rom_sram_scfg_disable_rom(
        &mut self,
    ) -> INTMEM_ROM_SRAM_SCFG_DISABLE_ROM_W<SYS_SYSCFG_6_SPEC> {
        INTMEM_ROM_SRAM_SCFG_DISABLE_ROM_W::new(self, 2)
    }
    #[doc = "Bit 3 - SRAM/ROM configuration. SLP: sleep enable, high active, default is low."]
    #[inline(always)]
    #[must_use]
    pub fn u0_intmem_rom_sram_sram_config_slp(
        &mut self,
    ) -> U0_INTMEM_ROM_SRAM_SRAM_CONFIG_SLP_W<SYS_SYSCFG_6_SPEC> {
        U0_INTMEM_ROM_SRAM_SRAM_CONFIG_SLP_W::new(self, 3)
    }
    #[doc = "Bit 4 - SRAM/ROM configuration. SD: shutdown enable, high active, default is low."]
    #[inline(always)]
    #[must_use]
    pub fn u0_intmem_rom_sram_sram_config_sd(
        &mut self,
    ) -> U0_INTMEM_ROM_SRAM_SRAM_CONFIG_SD_W<SYS_SYSCFG_6_SPEC> {
        U0_INTMEM_ROM_SRAM_SRAM_CONFIG_SD_W::new(self, 4)
    }
    #[doc = "Bits 5:6 - SRAM/ROM configuration. RTSEL: timing setting for debug purpose, default is 2'b01."]
    #[inline(always)]
    #[must_use]
    pub fn u0_intmem_rom_sram_sram_config_rtsel(
        &mut self,
    ) -> U0_INTMEM_ROM_SRAM_SRAM_CONFIG_RTSEL_W<SYS_SYSCFG_6_SPEC> {
        U0_INTMEM_ROM_SRAM_SRAM_CONFIG_RTSEL_W::new(self, 5)
    }
    #[doc = "Bits 7:8 - SRAM/ROM configuration. PTSEL: timing setting for debug purpose, default is 2'b01."]
    #[inline(always)]
    #[must_use]
    pub fn u0_intmem_rom_sram_sram_config_ptsel(
        &mut self,
    ) -> U0_INTMEM_ROM_SRAM_SRAM_CONFIG_PTSEL_W<SYS_SYSCFG_6_SPEC> {
        U0_INTMEM_ROM_SRAM_SRAM_CONFIG_PTSEL_W::new(self, 7)
    }
    #[doc = "Bits 9:10 - SRAM/ROM configuration. TRB: timing setting for debug purpose, default is 2'b01."]
    #[inline(always)]
    #[must_use]
    pub fn u0_intmem_rom_sram_sram_config_trb(
        &mut self,
    ) -> U0_INTMEM_ROM_SRAM_SRAM_CONFIG_TRB_W<SYS_SYSCFG_6_SPEC> {
        U0_INTMEM_ROM_SRAM_SRAM_CONFIG_TRB_W::new(self, 9)
    }
    #[doc = "Bits 11:12 - SRAM/ROM configuration. WTSEL: timing setting for debug purpose, default is 2'b01."]
    #[inline(always)]
    #[must_use]
    pub fn u0_intmem_rom_sram_sram_config_wtsel(
        &mut self,
    ) -> U0_INTMEM_ROM_SRAM_SRAM_CONFIG_WTSEL_W<SYS_SYSCFG_6_SPEC> {
        U0_INTMEM_ROM_SRAM_SRAM_CONFIG_WTSEL_W::new(self, 11)
    }
    #[doc = "Bit 13 - SRAM/ROM configuration. VS: timing setting for debug purpose, default is 1'b1."]
    #[inline(always)]
    #[must_use]
    pub fn u0_intmem_rom_sram_sram_config_vs(
        &mut self,
    ) -> U0_INTMEM_ROM_SRAM_SRAM_CONFIG_VS_W<SYS_SYSCFG_6_SPEC> {
        U0_INTMEM_ROM_SRAM_SRAM_CONFIG_VS_W::new(self, 13)
    }
    #[doc = "Bit 14 - SRAM/ROM configuration. VG: timing setting for debug purpose, default is 1'b1."]
    #[inline(always)]
    #[must_use]
    pub fn u0_intmem_rom_sram_sram_config_vg(
        &mut self,
    ) -> U0_INTMEM_ROM_SRAM_SRAM_CONFIG_VG_W<SYS_SYSCFG_6_SPEC> {
        U0_INTMEM_ROM_SRAM_SRAM_CONFIG_VG_W::new(self, 14)
    }
    #[doc = "Bit 15 - jtag_daisy_chain_en_0"]
    #[inline(always)]
    #[must_use]
    pub fn jtag_daisy_chain_en_0(&mut self) -> JTAG_DAISY_CHAIN_EN_0_W<SYS_SYSCFG_6_SPEC> {
        JTAG_DAISY_CHAIN_EN_0_W::new(self, 15)
    }
    #[doc = "Bit 16 - jtag_daisy_chain_en_1"]
    #[inline(always)]
    #[must_use]
    pub fn jtag_daisy_chain_en_1(&mut self) -> JTAG_DAISY_CHAIN_EN_1_W<SYS_SYSCFG_6_SPEC> {
        JTAG_DAISY_CHAIN_EN_1_W::new(self, 16)
    }
    #[doc = "Bit 17 - pdrstn_usbpipe_plugen"]
    #[inline(always)]
    #[must_use]
    pub fn pdrstn_usbpipe_plugen(&mut self) -> PDRSTN_USBPIPE_PLUGEN_W<SYS_SYSCFG_6_SPEC> {
        PDRSTN_USBPIPE_PLUGEN_W::new(self, 17)
    }
    #[doc = "Bits 18:20 - pll0_cpi_bias"]
    #[inline(always)]
    #[must_use]
    pub fn pll0_cpi_bias(&mut self) -> PLL0_CPI_BIAS_W<SYS_SYSCFG_6_SPEC> {
        PLL0_CPI_BIAS_W::new(self, 18)
    }
    #[doc = "Bits 21:23 - pll0_cpp_bias"]
    #[inline(always)]
    #[must_use]
    pub fn pll0_cpp_bias(&mut self) -> PLL0_CPP_BIAS_W<SYS_SYSCFG_6_SPEC> {
        PLL0_CPP_BIAS_W::new(self, 21)
    }
    #[doc = "Bit 24 - pll0_dacpd"]
    #[inline(always)]
    #[must_use]
    pub fn pll0_dacpd(&mut self) -> PLL0_DACPD_W<SYS_SYSCFG_6_SPEC> {
        PLL0_DACPD_W::new(self, 24)
    }
    #[doc = "Bit 25 - pll0_dsmpd"]
    #[inline(always)]
    #[must_use]
    pub fn pll0_dsmpd(&mut self) -> PLL0_DSMPD_W<SYS_SYSCFG_6_SPEC> {
        PLL0_DSMPD_W::new(self, 25)
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
#[doc = "SYS SYSCONSAIF SYSCFG 24\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`sys_syscfg_6::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`sys_syscfg_6::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct SYS_SYSCFG_6_SPEC;
impl crate::RegisterSpec for SYS_SYSCFG_6_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`sys_syscfg_6::R`](R) reader structure"]
impl crate::Readable for SYS_SYSCFG_6_SPEC {}
#[doc = "`write(|w| ..)` method takes [`sys_syscfg_6::W`](W) writer structure"]
impl crate::Writable for SYS_SYSCFG_6_SPEC {
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets sys_syscfg_6 to value 0"]
impl crate::Resettable for SYS_SYSCFG_6_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
