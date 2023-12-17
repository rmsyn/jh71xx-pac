#[doc = "Register `sys_syscfg_39` reader"]
pub type R = crate::R<SYS_SYSCFG_39_SPEC>;
#[doc = "Register `sys_syscfg_39` writer"]
pub type W = crate::W<SYS_SYSCFG_39_SPEC>;
#[doc = "Field `i2c_ic_en_1` reader - I2C interface enable."]
pub type I2C_IC_EN_1_R = crate::BitReader;
#[doc = "Field `sdio_data_strobe_phase_ctrl_1` reader - Data strobe delay chain select."]
pub type SDIO_DATA_STROBE_PHASE_CTRL_1_R = crate::FieldReader;
#[doc = "Field `sdio_data_strobe_phase_ctrl_1` writer - Data strobe delay chain select."]
pub type SDIO_DATA_STROBE_PHASE_CTRL_1_W<'a, REG> = crate::FieldWriter<'a, REG, 5>;
#[doc = "Field `sdio_hbig_endian_1` reader - AHB bus interface endianness: 1: Big-endian AHB bus interface, 0: Little-endian AHB bus interface"]
pub type SDIO_HBIG_ENDIAN_1_R = crate::BitReader;
#[doc = "Field `sdio_hbig_endian_1` writer - AHB bus interface endianness: 1: Big-endian AHB bus interface, 0: Little-endian AHB bus interface"]
pub type SDIO_HBIG_ENDIAN_1_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `sdio_m_hbig_endian_1` reader - AHB bus interface endianness: 1: Big-endian AHB bus interface, 0: Little-endian AHB bus interface"]
pub type SDIO_M_HBIG_ENDIAN_1_R = crate::BitReader;
#[doc = "Field `sdio_m_hbig_endian_1` writer - AHB bus interface endianness: 1: Big-endian AHB bus interface, 0: Little-endian AHB bus interface"]
pub type SDIO_M_HBIG_ENDIAN_1_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `reset_ctrl_clr_reset_status_1` reader - reset_ctrl_clr_reset_status_1"]
pub type RESET_CTRL_CLR_RESET_STATUS_1_R = crate::BitReader;
#[doc = "Field `reset_ctrl_clr_reset_status_1` writer - reset_ctrl_clr_reset_status_1"]
pub type RESET_CTRL_CLR_RESET_STATUS_1_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `reset_ctrl_pll_timecnt_finish_1` reader - reset_ctrl_pll_timecnt_finish_1"]
pub type RESET_CTRL_PLL_TIMECNT_FINISH_1_R = crate::BitReader;
#[doc = "Field `reset_ctrl_rstn_sw_1` reader - reset_ctrl_rstn_sw_1"]
pub type RESET_CTRL_RSTN_SW_1_R = crate::BitReader;
#[doc = "Field `reset_ctrl_rstn_sw_1` writer - reset_ctrl_rstn_sw_1"]
pub type RESET_CTRL_RSTN_SW_1_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `reset_ctrl_sys_reset_status_1` reader - reset_ctrl_sys_reset_status_1"]
pub type RESET_CTRL_SYS_RESET_STATUS_1_R = crate::FieldReader;
#[doc = "Field `i2c_ic_en_2` reader - I2C interface enable."]
pub type I2C_IC_EN_2_R = crate::BitReader;
#[doc = "Field `i2c_ic_en_3` reader - I2C interface enable."]
pub type I2C_IC_EN_3_R = crate::BitReader;
#[doc = "Field `i2c_ic_en_4` reader - I2C interface enable."]
pub type I2C_IC_EN_4_R = crate::BitReader;
#[doc = "Field `i2c_ic_en_5` reader - I2C interface enable."]
pub type I2C_IC_EN_5_R = crate::BitReader;
#[doc = "Field `i2c_ic_en_6` reader - I2C interface enable."]
pub type I2C_IC_EN_6_R = crate::BitReader;
impl R {
    #[doc = "Bit 0 - I2C interface enable."]
    #[inline(always)]
    pub fn i2c_ic_en_1(&self) -> I2C_IC_EN_1_R {
        I2C_IC_EN_1_R::new((self.bits & 1) != 0)
    }
    #[doc = "Bits 1:5 - Data strobe delay chain select."]
    #[inline(always)]
    pub fn sdio_data_strobe_phase_ctrl_1(&self) -> SDIO_DATA_STROBE_PHASE_CTRL_1_R {
        SDIO_DATA_STROBE_PHASE_CTRL_1_R::new(((self.bits >> 1) & 0x1f) as u8)
    }
    #[doc = "Bit 6 - AHB bus interface endianness: 1: Big-endian AHB bus interface, 0: Little-endian AHB bus interface"]
    #[inline(always)]
    pub fn sdio_hbig_endian_1(&self) -> SDIO_HBIG_ENDIAN_1_R {
        SDIO_HBIG_ENDIAN_1_R::new(((self.bits >> 6) & 1) != 0)
    }
    #[doc = "Bit 7 - AHB bus interface endianness: 1: Big-endian AHB bus interface, 0: Little-endian AHB bus interface"]
    #[inline(always)]
    pub fn sdio_m_hbig_endian_1(&self) -> SDIO_M_HBIG_ENDIAN_1_R {
        SDIO_M_HBIG_ENDIAN_1_R::new(((self.bits >> 7) & 1) != 0)
    }
    #[doc = "Bit 8 - reset_ctrl_clr_reset_status_1"]
    #[inline(always)]
    pub fn reset_ctrl_clr_reset_status_1(&self) -> RESET_CTRL_CLR_RESET_STATUS_1_R {
        RESET_CTRL_CLR_RESET_STATUS_1_R::new(((self.bits >> 8) & 1) != 0)
    }
    #[doc = "Bit 9 - reset_ctrl_pll_timecnt_finish_1"]
    #[inline(always)]
    pub fn reset_ctrl_pll_timecnt_finish_1(&self) -> RESET_CTRL_PLL_TIMECNT_FINISH_1_R {
        RESET_CTRL_PLL_TIMECNT_FINISH_1_R::new(((self.bits >> 9) & 1) != 0)
    }
    #[doc = "Bit 10 - reset_ctrl_rstn_sw_1"]
    #[inline(always)]
    pub fn reset_ctrl_rstn_sw_1(&self) -> RESET_CTRL_RSTN_SW_1_R {
        RESET_CTRL_RSTN_SW_1_R::new(((self.bits >> 10) & 1) != 0)
    }
    #[doc = "Bits 11:14 - reset_ctrl_sys_reset_status_1"]
    #[inline(always)]
    pub fn reset_ctrl_sys_reset_status_1(&self) -> RESET_CTRL_SYS_RESET_STATUS_1_R {
        RESET_CTRL_SYS_RESET_STATUS_1_R::new(((self.bits >> 11) & 0x0f) as u8)
    }
    #[doc = "Bit 15 - I2C interface enable."]
    #[inline(always)]
    pub fn i2c_ic_en_2(&self) -> I2C_IC_EN_2_R {
        I2C_IC_EN_2_R::new(((self.bits >> 15) & 1) != 0)
    }
    #[doc = "Bit 16 - I2C interface enable."]
    #[inline(always)]
    pub fn i2c_ic_en_3(&self) -> I2C_IC_EN_3_R {
        I2C_IC_EN_3_R::new(((self.bits >> 16) & 1) != 0)
    }
    #[doc = "Bit 17 - I2C interface enable."]
    #[inline(always)]
    pub fn i2c_ic_en_4(&self) -> I2C_IC_EN_4_R {
        I2C_IC_EN_4_R::new(((self.bits >> 17) & 1) != 0)
    }
    #[doc = "Bit 18 - I2C interface enable."]
    #[inline(always)]
    pub fn i2c_ic_en_5(&self) -> I2C_IC_EN_5_R {
        I2C_IC_EN_5_R::new(((self.bits >> 18) & 1) != 0)
    }
    #[doc = "Bit 19 - I2C interface enable."]
    #[inline(always)]
    pub fn i2c_ic_en_6(&self) -> I2C_IC_EN_6_R {
        I2C_IC_EN_6_R::new(((self.bits >> 19) & 1) != 0)
    }
}
impl W {
    #[doc = "Bits 1:5 - Data strobe delay chain select."]
    #[inline(always)]
    #[must_use]
    pub fn sdio_data_strobe_phase_ctrl_1(
        &mut self,
    ) -> SDIO_DATA_STROBE_PHASE_CTRL_1_W<SYS_SYSCFG_39_SPEC> {
        SDIO_DATA_STROBE_PHASE_CTRL_1_W::new(self, 1)
    }
    #[doc = "Bit 6 - AHB bus interface endianness: 1: Big-endian AHB bus interface, 0: Little-endian AHB bus interface"]
    #[inline(always)]
    #[must_use]
    pub fn sdio_hbig_endian_1(&mut self) -> SDIO_HBIG_ENDIAN_1_W<SYS_SYSCFG_39_SPEC> {
        SDIO_HBIG_ENDIAN_1_W::new(self, 6)
    }
    #[doc = "Bit 7 - AHB bus interface endianness: 1: Big-endian AHB bus interface, 0: Little-endian AHB bus interface"]
    #[inline(always)]
    #[must_use]
    pub fn sdio_m_hbig_endian_1(&mut self) -> SDIO_M_HBIG_ENDIAN_1_W<SYS_SYSCFG_39_SPEC> {
        SDIO_M_HBIG_ENDIAN_1_W::new(self, 7)
    }
    #[doc = "Bit 8 - reset_ctrl_clr_reset_status_1"]
    #[inline(always)]
    #[must_use]
    pub fn reset_ctrl_clr_reset_status_1(
        &mut self,
    ) -> RESET_CTRL_CLR_RESET_STATUS_1_W<SYS_SYSCFG_39_SPEC> {
        RESET_CTRL_CLR_RESET_STATUS_1_W::new(self, 8)
    }
    #[doc = "Bit 10 - reset_ctrl_rstn_sw_1"]
    #[inline(always)]
    #[must_use]
    pub fn reset_ctrl_rstn_sw_1(&mut self) -> RESET_CTRL_RSTN_SW_1_W<SYS_SYSCFG_39_SPEC> {
        RESET_CTRL_RSTN_SW_1_W::new(self, 10)
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
#[doc = "SYS SYSCONSAIF SYSCFG 156\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`sys_syscfg_39::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`sys_syscfg_39::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct SYS_SYSCFG_39_SPEC;
impl crate::RegisterSpec for SYS_SYSCFG_39_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`sys_syscfg_39::R`](R) reader structure"]
impl crate::Readable for SYS_SYSCFG_39_SPEC {}
#[doc = "`write(|w| ..)` method takes [`sys_syscfg_39::W`](W) writer structure"]
impl crate::Writable for SYS_SYSCFG_39_SPEC {
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets sys_syscfg_39 to value 0"]
impl crate::Resettable for SYS_SYSCFG_39_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
