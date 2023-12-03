#[doc = "Register `sys_sysconsaif_syscfg20` reader"]
pub type R = crate::R<SYS_SYSCONSAIF_SYSCFG20_SPEC>;
#[doc = "Register `sys_sysconsaif_syscfg20` writer"]
pub type W = crate::W<SYS_SYSCONSAIF_SYSCFG20_SPEC>;
#[doc = "Field `u0_cdns_qspi_scfg_sram_config_slp` reader - SRAM/ROM configuration. SLP: sleep enable, high active, default is low."]
pub type U0_CDNS_QSPI_SCFG_SRAM_CONFIG_SLP_R = crate::BitReader;
#[doc = "Field `u0_cdns_qspi_scfg_sram_config_slp` writer - SRAM/ROM configuration. SLP: sleep enable, high active, default is low."]
pub type U0_CDNS_QSPI_SCFG_SRAM_CONFIG_SLP_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `u0_cdns_qspi_scfg_sram_config_sd` reader - SRAM/ROM configuration. SD: shutdown enable, high active, default is low."]
pub type U0_CDNS_QSPI_SCFG_SRAM_CONFIG_SD_R = crate::BitReader;
#[doc = "Field `u0_cdns_qspi_scfg_sram_config_sd` writer - SRAM/ROM configuration. SD: shutdown enable, high active, default is low."]
pub type U0_CDNS_QSPI_SCFG_SRAM_CONFIG_SD_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `u0_cdns_qspi_scfg_sram_config_rtsel` reader - SRAM/ROM configuration. RTSEL: timing setting for debug purpose, default is 2'b01."]
pub type U0_CDNS_QSPI_SCFG_SRAM_CONFIG_RTSEL_R = crate::FieldReader;
#[doc = "Field `u0_cdns_qspi_scfg_sram_config_rtsel` writer - SRAM/ROM configuration. RTSEL: timing setting for debug purpose, default is 2'b01."]
pub type U0_CDNS_QSPI_SCFG_SRAM_CONFIG_RTSEL_W<'a, REG> = crate::FieldWriter<'a, REG, 2>;
#[doc = "Field `u0_cdns_qspi_scfg_sram_config_ptsel` reader - SRAM/ROM configuration. PTSEL: timing setting for debug purpose, default is 2'b01."]
pub type U0_CDNS_QSPI_SCFG_SRAM_CONFIG_PTSEL_R = crate::FieldReader;
#[doc = "Field `u0_cdns_qspi_scfg_sram_config_ptsel` writer - SRAM/ROM configuration. PTSEL: timing setting for debug purpose, default is 2'b01."]
pub type U0_CDNS_QSPI_SCFG_SRAM_CONFIG_PTSEL_W<'a, REG> = crate::FieldWriter<'a, REG, 2>;
#[doc = "Field `u0_cdns_qspi_scfg_sram_config_trb` reader - SRAM/ROM configuration. TRB: timing setting for debug purpose, default is 2'b01."]
pub type U0_CDNS_QSPI_SCFG_SRAM_CONFIG_TRB_R = crate::FieldReader;
#[doc = "Field `u0_cdns_qspi_scfg_sram_config_trb` writer - SRAM/ROM configuration. TRB: timing setting for debug purpose, default is 2'b01."]
pub type U0_CDNS_QSPI_SCFG_SRAM_CONFIG_TRB_W<'a, REG> = crate::FieldWriter<'a, REG, 2>;
#[doc = "Field `u0_cdns_qspi_scfg_sram_config_wtsel` reader - SRAM/ROM configuration. WTSEL: timing setting for debug purpose, default is 2'b01."]
pub type U0_CDNS_QSPI_SCFG_SRAM_CONFIG_WTSEL_R = crate::FieldReader;
#[doc = "Field `u0_cdns_qspi_scfg_sram_config_wtsel` writer - SRAM/ROM configuration. WTSEL: timing setting for debug purpose, default is 2'b01."]
pub type U0_CDNS_QSPI_SCFG_SRAM_CONFIG_WTSEL_W<'a, REG> = crate::FieldWriter<'a, REG, 2>;
#[doc = "Field `u0_cdns_qspi_scfg_sram_config_vs` reader - SRAM/ROM configuration. VS: timing setting for debug purpose, default is 1'b1."]
pub type U0_CDNS_QSPI_SCFG_SRAM_CONFIG_VS_R = crate::BitReader;
#[doc = "Field `u0_cdns_qspi_scfg_sram_config_vs` writer - SRAM/ROM configuration. VS: timing setting for debug purpose, default is 1'b1."]
pub type U0_CDNS_QSPI_SCFG_SRAM_CONFIG_VS_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `u0_cdns_qspi_scfg_sram_config_vg` reader - SRAM/ROM configuration. VG: timing setting for debug purpose, default is 1'b1."]
pub type U0_CDNS_QSPI_SCFG_SRAM_CONFIG_VG_R = crate::BitReader;
#[doc = "Field `u0_cdns_qspi_scfg_sram_config_vg` writer - SRAM/ROM configuration. VG: timing setting for debug purpose, default is 1'b1."]
pub type U0_CDNS_QSPI_SCFG_SRAM_CONFIG_VG_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `u0_cdns_spdif_scfg_sram_config_slp` reader - SRAM/ROM configuration. SLP: sleep enable, high active, default is low."]
pub type U0_CDNS_SPDIF_SCFG_SRAM_CONFIG_SLP_R = crate::BitReader;
#[doc = "Field `u0_cdns_spdif_scfg_sram_config_slp` writer - SRAM/ROM configuration. SLP: sleep enable, high active, default is low."]
pub type U0_CDNS_SPDIF_SCFG_SRAM_CONFIG_SLP_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `u0_cdns_spdif_scfg_sram_config_sd` reader - SRAM/ROM configuration. SD: shutdown enable, high active, default is low."]
pub type U0_CDNS_SPDIF_SCFG_SRAM_CONFIG_SD_R = crate::BitReader;
#[doc = "Field `u0_cdns_spdif_scfg_sram_config_sd` writer - SRAM/ROM configuration. SD: shutdown enable, high active, default is low."]
pub type U0_CDNS_SPDIF_SCFG_SRAM_CONFIG_SD_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `u0_cdns_spdif_scfg_sram_config_rtsel` reader - SRAM/ROM configuration. RTSEL: timing setting for debug purpose, default is 2'b01."]
pub type U0_CDNS_SPDIF_SCFG_SRAM_CONFIG_RTSEL_R = crate::FieldReader;
#[doc = "Field `u0_cdns_spdif_scfg_sram_config_rtsel` writer - SRAM/ROM configuration. RTSEL: timing setting for debug purpose, default is 2'b01."]
pub type U0_CDNS_SPDIF_SCFG_SRAM_CONFIG_RTSEL_W<'a, REG> = crate::FieldWriter<'a, REG, 2>;
#[doc = "Field `u0_cdns_spdif_scfg_sram_config_ptsel` reader - SRAM/ROM configuration. PTSEL: timing setting for debug purpose, default is 2'b01."]
pub type U0_CDNS_SPDIF_SCFG_SRAM_CONFIG_PTSEL_R = crate::FieldReader;
#[doc = "Field `u0_cdns_spdif_scfg_sram_config_ptsel` writer - SRAM/ROM configuration. PTSEL: timing setting for debug purpose, default is 2'b01."]
pub type U0_CDNS_SPDIF_SCFG_SRAM_CONFIG_PTSEL_W<'a, REG> = crate::FieldWriter<'a, REG, 2>;
#[doc = "Field `u0_cdns_spdif_scfg_sram_config_trb` reader - SRAM/ROM configuration. TRB: timing setting for debug purpose, default is 2'b01."]
pub type U0_CDNS_SPDIF_SCFG_SRAM_CONFIG_TRB_R = crate::FieldReader;
#[doc = "Field `u0_cdns_spdif_scfg_sram_config_trb` writer - SRAM/ROM configuration. TRB: timing setting for debug purpose, default is 2'b01."]
pub type U0_CDNS_SPDIF_SCFG_SRAM_CONFIG_TRB_W<'a, REG> = crate::FieldWriter<'a, REG, 2>;
#[doc = "Field `u0_cdns_spdif_scfg_sram_config_wtsel` reader - SRAM/ROM configuration. WTSEL: timing setting for debug purpose, default is 2'b01."]
pub type U0_CDNS_SPDIF_SCFG_SRAM_CONFIG_WTSEL_R = crate::FieldReader;
#[doc = "Field `u0_cdns_spdif_scfg_sram_config_wtsel` writer - SRAM/ROM configuration. WTSEL: timing setting for debug purpose, default is 2'b01."]
pub type U0_CDNS_SPDIF_SCFG_SRAM_CONFIG_WTSEL_W<'a, REG> = crate::FieldWriter<'a, REG, 2>;
#[doc = "Field `u0_cdns_spdif_scfg_sram_config_vs` reader - SRAM/ROM configuration. VS: timing setting for debug purpose, default is 1'b1."]
pub type U0_CDNS_SPDIF_SCFG_SRAM_CONFIG_VS_R = crate::BitReader;
#[doc = "Field `u0_cdns_spdif_scfg_sram_config_vs` writer - SRAM/ROM configuration. VS: timing setting for debug purpose, default is 1'b1."]
pub type U0_CDNS_SPDIF_SCFG_SRAM_CONFIG_VS_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `u0_cdns_spdif_scfg_sram_config_vg` reader - SRAM/ROM configuration. VG: timing setting for debug purpose, default is 1'b1."]
pub type U0_CDNS_SPDIF_SCFG_SRAM_CONFIG_VG_R = crate::BitReader;
#[doc = "Field `u0_cdns_spdif_scfg_sram_config_vg` writer - SRAM/ROM configuration. VG: timing setting for debug purpose, default is 1'b1."]
pub type U0_CDNS_SPDIF_SCFG_SRAM_CONFIG_VG_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `u0_cdns_spdif_trmodeo` reader - 1 for transmitter 0 for receiver"]
pub type U0_CDNS_SPDIF_TRMODEO_R = crate::BitReader;
#[doc = "Field `u0_i2c_ic_en` reader - I2C interface enable"]
pub type U0_I2C_IC_EN_R = crate::BitReader;
#[doc = "Field `u0_sdio_data_strobe_phase_ctrl` reader - Data strobe delay chain select"]
pub type U0_SDIO_DATA_STROBE_PHASE_CTRL_R = crate::FieldReader;
#[doc = "Field `u0_sdio_data_strobe_phase_ctrl` writer - Data strobe delay chain select"]
pub type U0_SDIO_DATA_STROBE_PHASE_CTRL_W<'a, REG> = crate::FieldWriter<'a, REG, 5>;
#[doc = "Field `u0_sdio_hbig_endian` reader - AHB bus interface endianness: 1: Big-endian AHB bus interface, 0: Little-endian AHB bus interface"]
pub type U0_SDIO_HBIG_ENDIAN_R = crate::BitReader;
#[doc = "Field `u0_sdio_hbig_endian` writer - AHB bus interface endianness: 1: Big-endian AHB bus interface, 0: Little-endian AHB bus interface"]
pub type U0_SDIO_HBIG_ENDIAN_W<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bit 0 - SRAM/ROM configuration. SLP: sleep enable, high active, default is low."]
    #[inline(always)]
    pub fn u0_cdns_qspi_scfg_sram_config_slp(&self) -> U0_CDNS_QSPI_SCFG_SRAM_CONFIG_SLP_R {
        U0_CDNS_QSPI_SCFG_SRAM_CONFIG_SLP_R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - SRAM/ROM configuration. SD: shutdown enable, high active, default is low."]
    #[inline(always)]
    pub fn u0_cdns_qspi_scfg_sram_config_sd(&self) -> U0_CDNS_QSPI_SCFG_SRAM_CONFIG_SD_R {
        U0_CDNS_QSPI_SCFG_SRAM_CONFIG_SD_R::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bits 2:3 - SRAM/ROM configuration. RTSEL: timing setting for debug purpose, default is 2'b01."]
    #[inline(always)]
    pub fn u0_cdns_qspi_scfg_sram_config_rtsel(&self) -> U0_CDNS_QSPI_SCFG_SRAM_CONFIG_RTSEL_R {
        U0_CDNS_QSPI_SCFG_SRAM_CONFIG_RTSEL_R::new(((self.bits >> 2) & 3) as u8)
    }
    #[doc = "Bits 4:5 - SRAM/ROM configuration. PTSEL: timing setting for debug purpose, default is 2'b01."]
    #[inline(always)]
    pub fn u0_cdns_qspi_scfg_sram_config_ptsel(&self) -> U0_CDNS_QSPI_SCFG_SRAM_CONFIG_PTSEL_R {
        U0_CDNS_QSPI_SCFG_SRAM_CONFIG_PTSEL_R::new(((self.bits >> 4) & 3) as u8)
    }
    #[doc = "Bits 6:7 - SRAM/ROM configuration. TRB: timing setting for debug purpose, default is 2'b01."]
    #[inline(always)]
    pub fn u0_cdns_qspi_scfg_sram_config_trb(&self) -> U0_CDNS_QSPI_SCFG_SRAM_CONFIG_TRB_R {
        U0_CDNS_QSPI_SCFG_SRAM_CONFIG_TRB_R::new(((self.bits >> 6) & 3) as u8)
    }
    #[doc = "Bits 8:9 - SRAM/ROM configuration. WTSEL: timing setting for debug purpose, default is 2'b01."]
    #[inline(always)]
    pub fn u0_cdns_qspi_scfg_sram_config_wtsel(&self) -> U0_CDNS_QSPI_SCFG_SRAM_CONFIG_WTSEL_R {
        U0_CDNS_QSPI_SCFG_SRAM_CONFIG_WTSEL_R::new(((self.bits >> 8) & 3) as u8)
    }
    #[doc = "Bit 10 - SRAM/ROM configuration. VS: timing setting for debug purpose, default is 1'b1."]
    #[inline(always)]
    pub fn u0_cdns_qspi_scfg_sram_config_vs(&self) -> U0_CDNS_QSPI_SCFG_SRAM_CONFIG_VS_R {
        U0_CDNS_QSPI_SCFG_SRAM_CONFIG_VS_R::new(((self.bits >> 10) & 1) != 0)
    }
    #[doc = "Bit 11 - SRAM/ROM configuration. VG: timing setting for debug purpose, default is 1'b1."]
    #[inline(always)]
    pub fn u0_cdns_qspi_scfg_sram_config_vg(&self) -> U0_CDNS_QSPI_SCFG_SRAM_CONFIG_VG_R {
        U0_CDNS_QSPI_SCFG_SRAM_CONFIG_VG_R::new(((self.bits >> 11) & 1) != 0)
    }
    #[doc = "Bit 12 - SRAM/ROM configuration. SLP: sleep enable, high active, default is low."]
    #[inline(always)]
    pub fn u0_cdns_spdif_scfg_sram_config_slp(&self) -> U0_CDNS_SPDIF_SCFG_SRAM_CONFIG_SLP_R {
        U0_CDNS_SPDIF_SCFG_SRAM_CONFIG_SLP_R::new(((self.bits >> 12) & 1) != 0)
    }
    #[doc = "Bit 13 - SRAM/ROM configuration. SD: shutdown enable, high active, default is low."]
    #[inline(always)]
    pub fn u0_cdns_spdif_scfg_sram_config_sd(&self) -> U0_CDNS_SPDIF_SCFG_SRAM_CONFIG_SD_R {
        U0_CDNS_SPDIF_SCFG_SRAM_CONFIG_SD_R::new(((self.bits >> 13) & 1) != 0)
    }
    #[doc = "Bits 14:15 - SRAM/ROM configuration. RTSEL: timing setting for debug purpose, default is 2'b01."]
    #[inline(always)]
    pub fn u0_cdns_spdif_scfg_sram_config_rtsel(&self) -> U0_CDNS_SPDIF_SCFG_SRAM_CONFIG_RTSEL_R {
        U0_CDNS_SPDIF_SCFG_SRAM_CONFIG_RTSEL_R::new(((self.bits >> 14) & 3) as u8)
    }
    #[doc = "Bits 16:17 - SRAM/ROM configuration. PTSEL: timing setting for debug purpose, default is 2'b01."]
    #[inline(always)]
    pub fn u0_cdns_spdif_scfg_sram_config_ptsel(&self) -> U0_CDNS_SPDIF_SCFG_SRAM_CONFIG_PTSEL_R {
        U0_CDNS_SPDIF_SCFG_SRAM_CONFIG_PTSEL_R::new(((self.bits >> 16) & 3) as u8)
    }
    #[doc = "Bits 18:19 - SRAM/ROM configuration. TRB: timing setting for debug purpose, default is 2'b01."]
    #[inline(always)]
    pub fn u0_cdns_spdif_scfg_sram_config_trb(&self) -> U0_CDNS_SPDIF_SCFG_SRAM_CONFIG_TRB_R {
        U0_CDNS_SPDIF_SCFG_SRAM_CONFIG_TRB_R::new(((self.bits >> 18) & 3) as u8)
    }
    #[doc = "Bits 20:21 - SRAM/ROM configuration. WTSEL: timing setting for debug purpose, default is 2'b01."]
    #[inline(always)]
    pub fn u0_cdns_spdif_scfg_sram_config_wtsel(&self) -> U0_CDNS_SPDIF_SCFG_SRAM_CONFIG_WTSEL_R {
        U0_CDNS_SPDIF_SCFG_SRAM_CONFIG_WTSEL_R::new(((self.bits >> 20) & 3) as u8)
    }
    #[doc = "Bit 22 - SRAM/ROM configuration. VS: timing setting for debug purpose, default is 1'b1."]
    #[inline(always)]
    pub fn u0_cdns_spdif_scfg_sram_config_vs(&self) -> U0_CDNS_SPDIF_SCFG_SRAM_CONFIG_VS_R {
        U0_CDNS_SPDIF_SCFG_SRAM_CONFIG_VS_R::new(((self.bits >> 22) & 1) != 0)
    }
    #[doc = "Bit 23 - SRAM/ROM configuration. VG: timing setting for debug purpose, default is 1'b1."]
    #[inline(always)]
    pub fn u0_cdns_spdif_scfg_sram_config_vg(&self) -> U0_CDNS_SPDIF_SCFG_SRAM_CONFIG_VG_R {
        U0_CDNS_SPDIF_SCFG_SRAM_CONFIG_VG_R::new(((self.bits >> 23) & 1) != 0)
    }
    #[doc = "Bit 24 - 1 for transmitter 0 for receiver"]
    #[inline(always)]
    pub fn u0_cdns_spdif_trmodeo(&self) -> U0_CDNS_SPDIF_TRMODEO_R {
        U0_CDNS_SPDIF_TRMODEO_R::new(((self.bits >> 24) & 1) != 0)
    }
    #[doc = "Bit 25 - I2C interface enable"]
    #[inline(always)]
    pub fn u0_i2c_ic_en(&self) -> U0_I2C_IC_EN_R {
        U0_I2C_IC_EN_R::new(((self.bits >> 25) & 1) != 0)
    }
    #[doc = "Bits 26:30 - Data strobe delay chain select"]
    #[inline(always)]
    pub fn u0_sdio_data_strobe_phase_ctrl(&self) -> U0_SDIO_DATA_STROBE_PHASE_CTRL_R {
        U0_SDIO_DATA_STROBE_PHASE_CTRL_R::new(((self.bits >> 26) & 0x1f) as u8)
    }
    #[doc = "Bit 31 - AHB bus interface endianness: 1: Big-endian AHB bus interface, 0: Little-endian AHB bus interface"]
    #[inline(always)]
    pub fn u0_sdio_hbig_endian(&self) -> U0_SDIO_HBIG_ENDIAN_R {
        U0_SDIO_HBIG_ENDIAN_R::new(((self.bits >> 31) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - SRAM/ROM configuration. SLP: sleep enable, high active, default is low."]
    #[inline(always)]
    #[must_use]
    pub fn u0_cdns_qspi_scfg_sram_config_slp(
        &mut self,
    ) -> U0_CDNS_QSPI_SCFG_SRAM_CONFIG_SLP_W<SYS_SYSCONSAIF_SYSCFG20_SPEC> {
        U0_CDNS_QSPI_SCFG_SRAM_CONFIG_SLP_W::new(self, 0)
    }
    #[doc = "Bit 1 - SRAM/ROM configuration. SD: shutdown enable, high active, default is low."]
    #[inline(always)]
    #[must_use]
    pub fn u0_cdns_qspi_scfg_sram_config_sd(
        &mut self,
    ) -> U0_CDNS_QSPI_SCFG_SRAM_CONFIG_SD_W<SYS_SYSCONSAIF_SYSCFG20_SPEC> {
        U0_CDNS_QSPI_SCFG_SRAM_CONFIG_SD_W::new(self, 1)
    }
    #[doc = "Bits 2:3 - SRAM/ROM configuration. RTSEL: timing setting for debug purpose, default is 2'b01."]
    #[inline(always)]
    #[must_use]
    pub fn u0_cdns_qspi_scfg_sram_config_rtsel(
        &mut self,
    ) -> U0_CDNS_QSPI_SCFG_SRAM_CONFIG_RTSEL_W<SYS_SYSCONSAIF_SYSCFG20_SPEC> {
        U0_CDNS_QSPI_SCFG_SRAM_CONFIG_RTSEL_W::new(self, 2)
    }
    #[doc = "Bits 4:5 - SRAM/ROM configuration. PTSEL: timing setting for debug purpose, default is 2'b01."]
    #[inline(always)]
    #[must_use]
    pub fn u0_cdns_qspi_scfg_sram_config_ptsel(
        &mut self,
    ) -> U0_CDNS_QSPI_SCFG_SRAM_CONFIG_PTSEL_W<SYS_SYSCONSAIF_SYSCFG20_SPEC> {
        U0_CDNS_QSPI_SCFG_SRAM_CONFIG_PTSEL_W::new(self, 4)
    }
    #[doc = "Bits 6:7 - SRAM/ROM configuration. TRB: timing setting for debug purpose, default is 2'b01."]
    #[inline(always)]
    #[must_use]
    pub fn u0_cdns_qspi_scfg_sram_config_trb(
        &mut self,
    ) -> U0_CDNS_QSPI_SCFG_SRAM_CONFIG_TRB_W<SYS_SYSCONSAIF_SYSCFG20_SPEC> {
        U0_CDNS_QSPI_SCFG_SRAM_CONFIG_TRB_W::new(self, 6)
    }
    #[doc = "Bits 8:9 - SRAM/ROM configuration. WTSEL: timing setting for debug purpose, default is 2'b01."]
    #[inline(always)]
    #[must_use]
    pub fn u0_cdns_qspi_scfg_sram_config_wtsel(
        &mut self,
    ) -> U0_CDNS_QSPI_SCFG_SRAM_CONFIG_WTSEL_W<SYS_SYSCONSAIF_SYSCFG20_SPEC> {
        U0_CDNS_QSPI_SCFG_SRAM_CONFIG_WTSEL_W::new(self, 8)
    }
    #[doc = "Bit 10 - SRAM/ROM configuration. VS: timing setting for debug purpose, default is 1'b1."]
    #[inline(always)]
    #[must_use]
    pub fn u0_cdns_qspi_scfg_sram_config_vs(
        &mut self,
    ) -> U0_CDNS_QSPI_SCFG_SRAM_CONFIG_VS_W<SYS_SYSCONSAIF_SYSCFG20_SPEC> {
        U0_CDNS_QSPI_SCFG_SRAM_CONFIG_VS_W::new(self, 10)
    }
    #[doc = "Bit 11 - SRAM/ROM configuration. VG: timing setting for debug purpose, default is 1'b1."]
    #[inline(always)]
    #[must_use]
    pub fn u0_cdns_qspi_scfg_sram_config_vg(
        &mut self,
    ) -> U0_CDNS_QSPI_SCFG_SRAM_CONFIG_VG_W<SYS_SYSCONSAIF_SYSCFG20_SPEC> {
        U0_CDNS_QSPI_SCFG_SRAM_CONFIG_VG_W::new(self, 11)
    }
    #[doc = "Bit 12 - SRAM/ROM configuration. SLP: sleep enable, high active, default is low."]
    #[inline(always)]
    #[must_use]
    pub fn u0_cdns_spdif_scfg_sram_config_slp(
        &mut self,
    ) -> U0_CDNS_SPDIF_SCFG_SRAM_CONFIG_SLP_W<SYS_SYSCONSAIF_SYSCFG20_SPEC> {
        U0_CDNS_SPDIF_SCFG_SRAM_CONFIG_SLP_W::new(self, 12)
    }
    #[doc = "Bit 13 - SRAM/ROM configuration. SD: shutdown enable, high active, default is low."]
    #[inline(always)]
    #[must_use]
    pub fn u0_cdns_spdif_scfg_sram_config_sd(
        &mut self,
    ) -> U0_CDNS_SPDIF_SCFG_SRAM_CONFIG_SD_W<SYS_SYSCONSAIF_SYSCFG20_SPEC> {
        U0_CDNS_SPDIF_SCFG_SRAM_CONFIG_SD_W::new(self, 13)
    }
    #[doc = "Bits 14:15 - SRAM/ROM configuration. RTSEL: timing setting for debug purpose, default is 2'b01."]
    #[inline(always)]
    #[must_use]
    pub fn u0_cdns_spdif_scfg_sram_config_rtsel(
        &mut self,
    ) -> U0_CDNS_SPDIF_SCFG_SRAM_CONFIG_RTSEL_W<SYS_SYSCONSAIF_SYSCFG20_SPEC> {
        U0_CDNS_SPDIF_SCFG_SRAM_CONFIG_RTSEL_W::new(self, 14)
    }
    #[doc = "Bits 16:17 - SRAM/ROM configuration. PTSEL: timing setting for debug purpose, default is 2'b01."]
    #[inline(always)]
    #[must_use]
    pub fn u0_cdns_spdif_scfg_sram_config_ptsel(
        &mut self,
    ) -> U0_CDNS_SPDIF_SCFG_SRAM_CONFIG_PTSEL_W<SYS_SYSCONSAIF_SYSCFG20_SPEC> {
        U0_CDNS_SPDIF_SCFG_SRAM_CONFIG_PTSEL_W::new(self, 16)
    }
    #[doc = "Bits 18:19 - SRAM/ROM configuration. TRB: timing setting for debug purpose, default is 2'b01."]
    #[inline(always)]
    #[must_use]
    pub fn u0_cdns_spdif_scfg_sram_config_trb(
        &mut self,
    ) -> U0_CDNS_SPDIF_SCFG_SRAM_CONFIG_TRB_W<SYS_SYSCONSAIF_SYSCFG20_SPEC> {
        U0_CDNS_SPDIF_SCFG_SRAM_CONFIG_TRB_W::new(self, 18)
    }
    #[doc = "Bits 20:21 - SRAM/ROM configuration. WTSEL: timing setting for debug purpose, default is 2'b01."]
    #[inline(always)]
    #[must_use]
    pub fn u0_cdns_spdif_scfg_sram_config_wtsel(
        &mut self,
    ) -> U0_CDNS_SPDIF_SCFG_SRAM_CONFIG_WTSEL_W<SYS_SYSCONSAIF_SYSCFG20_SPEC> {
        U0_CDNS_SPDIF_SCFG_SRAM_CONFIG_WTSEL_W::new(self, 20)
    }
    #[doc = "Bit 22 - SRAM/ROM configuration. VS: timing setting for debug purpose, default is 1'b1."]
    #[inline(always)]
    #[must_use]
    pub fn u0_cdns_spdif_scfg_sram_config_vs(
        &mut self,
    ) -> U0_CDNS_SPDIF_SCFG_SRAM_CONFIG_VS_W<SYS_SYSCONSAIF_SYSCFG20_SPEC> {
        U0_CDNS_SPDIF_SCFG_SRAM_CONFIG_VS_W::new(self, 22)
    }
    #[doc = "Bit 23 - SRAM/ROM configuration. VG: timing setting for debug purpose, default is 1'b1."]
    #[inline(always)]
    #[must_use]
    pub fn u0_cdns_spdif_scfg_sram_config_vg(
        &mut self,
    ) -> U0_CDNS_SPDIF_SCFG_SRAM_CONFIG_VG_W<SYS_SYSCONSAIF_SYSCFG20_SPEC> {
        U0_CDNS_SPDIF_SCFG_SRAM_CONFIG_VG_W::new(self, 23)
    }
    #[doc = "Bits 26:30 - Data strobe delay chain select"]
    #[inline(always)]
    #[must_use]
    pub fn u0_sdio_data_strobe_phase_ctrl(
        &mut self,
    ) -> U0_SDIO_DATA_STROBE_PHASE_CTRL_W<SYS_SYSCONSAIF_SYSCFG20_SPEC> {
        U0_SDIO_DATA_STROBE_PHASE_CTRL_W::new(self, 26)
    }
    #[doc = "Bit 31 - AHB bus interface endianness: 1: Big-endian AHB bus interface, 0: Little-endian AHB bus interface"]
    #[inline(always)]
    #[must_use]
    pub fn u0_sdio_hbig_endian(&mut self) -> U0_SDIO_HBIG_ENDIAN_W<SYS_SYSCONSAIF_SYSCFG20_SPEC> {
        U0_SDIO_HBIG_ENDIAN_W::new(self, 31)
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
#[doc = "SYS SYSCONSAIF SYSCFG 20\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`sys_sysconsaif_syscfg20::R`](R).  You can [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`sys_sysconsaif_syscfg20::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct SYS_SYSCONSAIF_SYSCFG20_SPEC;
impl crate::RegisterSpec for SYS_SYSCONSAIF_SYSCFG20_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`sys_sysconsaif_syscfg20::R`](R) reader structure"]
impl crate::Readable for SYS_SYSCONSAIF_SYSCFG20_SPEC {}
#[doc = "`write(|w| ..)` method takes [`sys_sysconsaif_syscfg20::W`](W) writer structure"]
impl crate::Writable for SYS_SYSCONSAIF_SYSCFG20_SPEC {
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
