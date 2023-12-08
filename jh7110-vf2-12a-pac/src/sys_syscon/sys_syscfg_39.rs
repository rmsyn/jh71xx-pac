#[doc = "Register `sys_syscfg_39` reader"]
pub type R = crate::R<SYS_SYSCFG_39_SPEC>;
#[doc = "Register `sys_syscfg_39` writer"]
pub type W = crate::W<SYS_SYSCFG_39_SPEC>;
#[doc = "Field `u1_i2c_ic_en` reader - I2C interface enable."]
pub type U1_I2C_IC_EN_R = crate::BitReader;
#[doc = "Field `u1_sdio_data_strobe_phase_ctrl` reader - Data strobe delay chain select."]
pub type U1_SDIO_DATA_STROBE_PHASE_CTRL_R = crate::FieldReader;
#[doc = "Field `u1_sdio_data_strobe_phase_ctrl` writer - Data strobe delay chain select."]
pub type U1_SDIO_DATA_STROBE_PHASE_CTRL_W<'a, REG> = crate::FieldWriter<'a, REG, 5>;
#[doc = "Field `u1_sdio_hbig_endian` reader - AHB bus interface endianness: 1: Big-endian AHB bus interface, 0: Little-endian AHB bus interface"]
pub type U1_SDIO_HBIG_ENDIAN_R = crate::BitReader;
#[doc = "Field `u1_sdio_hbig_endian` writer - AHB bus interface endianness: 1: Big-endian AHB bus interface, 0: Little-endian AHB bus interface"]
pub type U1_SDIO_HBIG_ENDIAN_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `u1_sdio_m_hbig_endian` reader - AHB bus interface endianness: 1: Big-endian AHB bus interface, 0: Little-endian AHB bus interface"]
pub type U1_SDIO_M_HBIG_ENDIAN_R = crate::BitReader;
#[doc = "Field `u1_sdio_m_hbig_endian` writer - AHB bus interface endianness: 1: Big-endian AHB bus interface, 0: Little-endian AHB bus interface"]
pub type U1_SDIO_M_HBIG_ENDIAN_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `u1_reset_ctrl_clr_reset_status` reader - u1_reset_ctrl_clr_reset_status"]
pub type U1_RESET_CTRL_CLR_RESET_STATUS_R = crate::BitReader;
#[doc = "Field `u1_reset_ctrl_clr_reset_status` writer - u1_reset_ctrl_clr_reset_status"]
pub type U1_RESET_CTRL_CLR_RESET_STATUS_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `u1_reset_ctrl_pll_timecnt_finish` reader - u1_reset_ctrl_pll_timecnt_finish"]
pub type U1_RESET_CTRL_PLL_TIMECNT_FINISH_R = crate::BitReader;
#[doc = "Field `u1_reset_ctrl_rstn_sw` reader - u1_reset_ctrl_rstn_sw"]
pub type U1_RESET_CTRL_RSTN_SW_R = crate::BitReader;
#[doc = "Field `u1_reset_ctrl_rstn_sw` writer - u1_reset_ctrl_rstn_sw"]
pub type U1_RESET_CTRL_RSTN_SW_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `u1_reset_ctrl_sys_reset_status` reader - u1_reset_ctrl_sys_reset_status"]
pub type U1_RESET_CTRL_SYS_RESET_STATUS_R = crate::FieldReader;
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
    pub fn u1_i2c_ic_en(&self) -> U1_I2C_IC_EN_R {
        U1_I2C_IC_EN_R::new((self.bits & 1) != 0)
    }
    #[doc = "Bits 1:5 - Data strobe delay chain select."]
    #[inline(always)]
    pub fn u1_sdio_data_strobe_phase_ctrl(&self) -> U1_SDIO_DATA_STROBE_PHASE_CTRL_R {
        U1_SDIO_DATA_STROBE_PHASE_CTRL_R::new(((self.bits >> 1) & 0x1f) as u8)
    }
    #[doc = "Bit 6 - AHB bus interface endianness: 1: Big-endian AHB bus interface, 0: Little-endian AHB bus interface"]
    #[inline(always)]
    pub fn u1_sdio_hbig_endian(&self) -> U1_SDIO_HBIG_ENDIAN_R {
        U1_SDIO_HBIG_ENDIAN_R::new(((self.bits >> 6) & 1) != 0)
    }
    #[doc = "Bit 7 - AHB bus interface endianness: 1: Big-endian AHB bus interface, 0: Little-endian AHB bus interface"]
    #[inline(always)]
    pub fn u1_sdio_m_hbig_endian(&self) -> U1_SDIO_M_HBIG_ENDIAN_R {
        U1_SDIO_M_HBIG_ENDIAN_R::new(((self.bits >> 7) & 1) != 0)
    }
    #[doc = "Bit 8 - u1_reset_ctrl_clr_reset_status"]
    #[inline(always)]
    pub fn u1_reset_ctrl_clr_reset_status(&self) -> U1_RESET_CTRL_CLR_RESET_STATUS_R {
        U1_RESET_CTRL_CLR_RESET_STATUS_R::new(((self.bits >> 8) & 1) != 0)
    }
    #[doc = "Bit 9 - u1_reset_ctrl_pll_timecnt_finish"]
    #[inline(always)]
    pub fn u1_reset_ctrl_pll_timecnt_finish(&self) -> U1_RESET_CTRL_PLL_TIMECNT_FINISH_R {
        U1_RESET_CTRL_PLL_TIMECNT_FINISH_R::new(((self.bits >> 9) & 1) != 0)
    }
    #[doc = "Bit 10 - u1_reset_ctrl_rstn_sw"]
    #[inline(always)]
    pub fn u1_reset_ctrl_rstn_sw(&self) -> U1_RESET_CTRL_RSTN_SW_R {
        U1_RESET_CTRL_RSTN_SW_R::new(((self.bits >> 10) & 1) != 0)
    }
    #[doc = "Bits 11:14 - u1_reset_ctrl_sys_reset_status"]
    #[inline(always)]
    pub fn u1_reset_ctrl_sys_reset_status(&self) -> U1_RESET_CTRL_SYS_RESET_STATUS_R {
        U1_RESET_CTRL_SYS_RESET_STATUS_R::new(((self.bits >> 11) & 0x0f) as u8)
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
    pub fn u1_sdio_data_strobe_phase_ctrl(
        &mut self,
    ) -> U1_SDIO_DATA_STROBE_PHASE_CTRL_W<SYS_SYSCFG_39_SPEC> {
        U1_SDIO_DATA_STROBE_PHASE_CTRL_W::new(self, 1)
    }
    #[doc = "Bit 6 - AHB bus interface endianness: 1: Big-endian AHB bus interface, 0: Little-endian AHB bus interface"]
    #[inline(always)]
    #[must_use]
    pub fn u1_sdio_hbig_endian(&mut self) -> U1_SDIO_HBIG_ENDIAN_W<SYS_SYSCFG_39_SPEC> {
        U1_SDIO_HBIG_ENDIAN_W::new(self, 6)
    }
    #[doc = "Bit 7 - AHB bus interface endianness: 1: Big-endian AHB bus interface, 0: Little-endian AHB bus interface"]
    #[inline(always)]
    #[must_use]
    pub fn u1_sdio_m_hbig_endian(&mut self) -> U1_SDIO_M_HBIG_ENDIAN_W<SYS_SYSCFG_39_SPEC> {
        U1_SDIO_M_HBIG_ENDIAN_W::new(self, 7)
    }
    #[doc = "Bit 8 - u1_reset_ctrl_clr_reset_status"]
    #[inline(always)]
    #[must_use]
    pub fn u1_reset_ctrl_clr_reset_status(
        &mut self,
    ) -> U1_RESET_CTRL_CLR_RESET_STATUS_W<SYS_SYSCFG_39_SPEC> {
        U1_RESET_CTRL_CLR_RESET_STATUS_W::new(self, 8)
    }
    #[doc = "Bit 10 - u1_reset_ctrl_rstn_sw"]
    #[inline(always)]
    #[must_use]
    pub fn u1_reset_ctrl_rstn_sw(&mut self) -> U1_RESET_CTRL_RSTN_SW_W<SYS_SYSCFG_39_SPEC> {
        U1_RESET_CTRL_RSTN_SW_W::new(self, 10)
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
