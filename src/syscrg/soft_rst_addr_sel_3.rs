#[doc = "Register `soft_rst_addr_sel_3` reader"]
pub type R = crate::R<SOFT_RST_ADDR_SEL_3_SPEC>;
#[doc = "Register `soft_rst_addr_sel_3` writer"]
pub type W = crate::W<SOFT_RST_ADDR_SEL_3_SPEC>;
#[doc = "Field `u0_pwmdac_apb` reader - 1: Assert reset, 0: De-assert reset"]
pub type U0_PWMDAC_APB_R = crate::BitReader;
#[doc = "Field `u0_pwmdac_apb` writer - 1: Assert reset, 0: De-assert reset"]
pub type U0_PWMDAC_APB_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `u0_pdm_4mic_dmic` reader - 1: Assert reset, 0: De-assert reset"]
pub type U0_PDM_4MIC_DMIC_R = crate::BitReader;
#[doc = "Field `u0_pdm_4mic_dmic` writer - 1: Assert reset, 0: De-assert reset"]
pub type U0_PDM_4MIC_DMIC_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `u0_pdm_4mic_apb` reader - 1: Assert reset, 0: De-assert reset"]
pub type U0_PDM_4MIC_APB_R = crate::BitReader;
#[doc = "Field `u0_pdm_4mic_apb` writer - 1: Assert reset, 0: De-assert reset"]
pub type U0_PDM_4MIC_APB_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `u0_i2srx_apb` reader - 1: Assert reset, 0: De-assert reset"]
pub type U0_I2SRX_APB_R = crate::BitReader;
#[doc = "Field `u0_i2srx_apb` writer - 1: Assert reset, 0: De-assert reset"]
pub type U0_I2SRX_APB_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `u0_i2srx_bclk` reader - 1: Assert reset, 0: De-assert reset"]
pub type U0_I2SRX_BCLK_R = crate::BitReader;
#[doc = "Field `u0_i2srx_bclk` writer - 1: Assert reset, 0: De-assert reset"]
pub type U0_I2SRX_BCLK_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `u0_i2stx_apb` reader - 1: Assert reset, 0: De-assert reset"]
pub type U0_I2STX_APB_R = crate::BitReader;
#[doc = "Field `u0_i2stx_apb` writer - 1: Assert reset, 0: De-assert reset"]
pub type U0_I2STX_APB_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `u0_i2stx_bclk` reader - 1: Assert reset, 0: De-assert reset"]
pub type U0_I2STX_BCLK_R = crate::BitReader;
#[doc = "Field `u0_i2stx_bclk` writer - 1: Assert reset, 0: De-assert reset"]
pub type U0_I2STX_BCLK_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `u1_i2stx_apb` reader - 1: Assert reset, 0: De-assert reset"]
pub type U1_I2STX_APB_R = crate::BitReader;
#[doc = "Field `u1_i2stx_apb` writer - 1: Assert reset, 0: De-assert reset"]
pub type U1_I2STX_APB_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `u1_i2stx_bclk` reader - 1: Assert reset, 0: De-assert reset"]
pub type U1_I2STX_BCLK_R = crate::BitReader;
#[doc = "Field `u1_i2stx_bclk` writer - 1: Assert reset, 0: De-assert reset"]
pub type U1_I2STX_BCLK_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `u0_tdm16slot_ahb` reader - 1: Assert reset, 0: De-assert reset"]
pub type U0_TDM16SLOT_AHB_R = crate::BitReader;
#[doc = "Field `u0_tdm16slot_ahb` writer - 1: Assert reset, 0: De-assert reset"]
pub type U0_TDM16SLOT_AHB_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `u0_tdm16slot_tdm` reader - 1: Assert reset, 0: De-assert reset"]
pub type U0_TDM16SLOT_TDM_R = crate::BitReader;
#[doc = "Field `u0_tdm16slot_tdm` writer - 1: Assert reset, 0: De-assert reset"]
pub type U0_TDM16SLOT_TDM_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `u0_tdm16slot_apb` reader - 1: Assert reset, 0: De-assert reset"]
pub type U0_TDM16SLOT_APB_R = crate::BitReader;
#[doc = "Field `u0_tdm16slot_apb` writer - 1: Assert reset, 0: De-assert reset"]
pub type U0_TDM16SLOT_APB_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `u0_pwm_apb` reader - 1: Assert reset, 0: De-assert reset"]
pub type U0_PWM_APB_R = crate::BitReader;
#[doc = "Field `u0_pwm_apb` writer - 1: Assert reset, 0: De-assert reset"]
pub type U0_PWM_APB_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `u0_dskit_wdt_rstn_apb` reader - 1: Assert reset, 0: De-assert reset"]
pub type U0_DSKIT_WDT_RSTN_APB_R = crate::BitReader;
#[doc = "Field `u0_dskit_wdt_rstn_apb` writer - 1: Assert reset, 0: De-assert reset"]
pub type U0_DSKIT_WDT_RSTN_APB_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `u0_dskit_wdt` reader - 1: Assert reset, 0: De-assert reset"]
pub type U0_DSKIT_WDT_R = crate::BitReader;
#[doc = "Field `u0_dskit_wdt` writer - 1: Assert reset, 0: De-assert reset"]
pub type U0_DSKIT_WDT_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `u0_can_ctrl_apb` reader - 1: Assert reset, 0: De-assert reset"]
pub type U0_CAN_CTRL_APB_R = crate::BitReader;
#[doc = "Field `u0_can_ctrl_apb` writer - 1: Assert reset, 0: De-assert reset"]
pub type U0_CAN_CTRL_APB_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `u0_can_ctrl` reader - 1: Assert reset, 0: De-assert reset"]
pub type U0_CAN_CTRL_R = crate::BitReader;
#[doc = "Field `u0_can_ctrl` writer - 1: Assert reset, 0: De-assert reset"]
pub type U0_CAN_CTRL_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `u0_can_ctrl_timer` reader - 1: Assert reset, 0: De-assert reset"]
pub type U0_CAN_CTRL_TIMER_R = crate::BitReader;
#[doc = "Field `u0_can_ctrl_timer` writer - 1: Assert reset, 0: De-assert reset"]
pub type U0_CAN_CTRL_TIMER_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `u1_can_ctrl_apb` reader - 1: Assert reset, 0: De-assert reset"]
pub type U1_CAN_CTRL_APB_R = crate::BitReader;
#[doc = "Field `u1_can_ctrl_apb` writer - 1: Assert reset, 0: De-assert reset"]
pub type U1_CAN_CTRL_APB_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `u1_can_ctrl_can` reader - 1: Assert reset, 0: De-assert reset"]
pub type U1_CAN_CTRL_CAN_R = crate::BitReader;
#[doc = "Field `u1_can_ctrl_can` writer - 1: Assert reset, 0: De-assert reset"]
pub type U1_CAN_CTRL_CAN_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `u1_can_ctrl_timer` reader - 1: Assert reset, 0: De-assert reset"]
pub type U1_CAN_CTRL_TIMER_R = crate::BitReader;
#[doc = "Field `u1_can_ctrl_timer` writer - 1: Assert reset, 0: De-assert reset"]
pub type U1_CAN_CTRL_TIMER_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `u0_si5_timer_apb` reader - 1: Assert reset, 0: De-assert reset"]
pub type U0_SI5_TIMER_APB_R = crate::BitReader;
#[doc = "Field `u0_si5_timer_apb` writer - 1: Assert reset, 0: De-assert reset"]
pub type U0_SI5_TIMER_APB_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `u0_si5_timer_0` reader - 1: Assert reset, 0: De-assert reset"]
pub type U0_SI5_TIMER_0_R = crate::BitReader;
#[doc = "Field `u0_si5_timer_0` writer - 1: Assert reset, 0: De-assert reset"]
pub type U0_SI5_TIMER_0_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `u0_si5_timer_1` reader - 1: Assert reset, 0: De-assert reset"]
pub type U0_SI5_TIMER_1_R = crate::BitReader;
#[doc = "Field `u0_si5_timer_1` writer - 1: Assert reset, 0: De-assert reset"]
pub type U0_SI5_TIMER_1_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `u0_si5_timer_2` reader - 1: Assert reset, 0: De-assert reset"]
pub type U0_SI5_TIMER_2_R = crate::BitReader;
#[doc = "Field `u0_si5_timer_2` writer - 1: Assert reset, 0: De-assert reset"]
pub type U0_SI5_TIMER_2_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `u0_si5_timer_3` reader - 1: Assert reset, 0: De-assert reset"]
pub type U0_SI5_TIMER_3_R = crate::BitReader;
#[doc = "Field `u0_si5_timer_3` writer - 1: Assert reset, 0: De-assert reset"]
pub type U0_SI5_TIMER_3_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `u0_int_ctrl_apb` reader - 1: Assert reset, 0: De-assert reset"]
pub type U0_INT_CTRL_APB_R = crate::BitReader;
#[doc = "Field `u0_int_ctrl_apb` writer - 1: Assert reset, 0: De-assert reset"]
pub type U0_INT_CTRL_APB_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `u0_temp_sensor_apb` reader - 1: Assert reset, 0: De-assert reset"]
pub type U0_TEMP_SENSOR_APB_R = crate::BitReader;
#[doc = "Field `u0_temp_sensor_apb` writer - 1: Assert reset, 0: De-assert reset"]
pub type U0_TEMP_SENSOR_APB_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `u0_temp_sensor` reader - 1: Assert reset, 0: De-assert reset"]
pub type U0_TEMP_SENSOR_R = crate::BitReader;
#[doc = "Field `u0_temp_sensor` writer - 1: Assert reset, 0: De-assert reset"]
pub type U0_TEMP_SENSOR_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `u0_jtag_rst` reader - 1: Assert reset, 0: De-assert reset"]
pub type U0_JTAG_RST_R = crate::BitReader;
#[doc = "Field `u0_jtag_rst` writer - 1: Assert reset, 0: De-assert reset"]
pub type U0_JTAG_RST_W<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bit 0 - 1: Assert reset, 0: De-assert reset"]
    #[inline(always)]
    pub fn u0_pwmdac_apb(&self) -> U0_PWMDAC_APB_R {
        U0_PWMDAC_APB_R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - 1: Assert reset, 0: De-assert reset"]
    #[inline(always)]
    pub fn u0_pdm_4mic_dmic(&self) -> U0_PDM_4MIC_DMIC_R {
        U0_PDM_4MIC_DMIC_R::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - 1: Assert reset, 0: De-assert reset"]
    #[inline(always)]
    pub fn u0_pdm_4mic_apb(&self) -> U0_PDM_4MIC_APB_R {
        U0_PDM_4MIC_APB_R::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3 - 1: Assert reset, 0: De-assert reset"]
    #[inline(always)]
    pub fn u0_i2srx_apb(&self) -> U0_I2SRX_APB_R {
        U0_I2SRX_APB_R::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bit 4 - 1: Assert reset, 0: De-assert reset"]
    #[inline(always)]
    pub fn u0_i2srx_bclk(&self) -> U0_I2SRX_BCLK_R {
        U0_I2SRX_BCLK_R::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bit 5 - 1: Assert reset, 0: De-assert reset"]
    #[inline(always)]
    pub fn u0_i2stx_apb(&self) -> U0_I2STX_APB_R {
        U0_I2STX_APB_R::new(((self.bits >> 5) & 1) != 0)
    }
    #[doc = "Bit 6 - 1: Assert reset, 0: De-assert reset"]
    #[inline(always)]
    pub fn u0_i2stx_bclk(&self) -> U0_I2STX_BCLK_R {
        U0_I2STX_BCLK_R::new(((self.bits >> 6) & 1) != 0)
    }
    #[doc = "Bit 7 - 1: Assert reset, 0: De-assert reset"]
    #[inline(always)]
    pub fn u1_i2stx_apb(&self) -> U1_I2STX_APB_R {
        U1_I2STX_APB_R::new(((self.bits >> 7) & 1) != 0)
    }
    #[doc = "Bit 8 - 1: Assert reset, 0: De-assert reset"]
    #[inline(always)]
    pub fn u1_i2stx_bclk(&self) -> U1_I2STX_BCLK_R {
        U1_I2STX_BCLK_R::new(((self.bits >> 8) & 1) != 0)
    }
    #[doc = "Bit 9 - 1: Assert reset, 0: De-assert reset"]
    #[inline(always)]
    pub fn u0_tdm16slot_ahb(&self) -> U0_TDM16SLOT_AHB_R {
        U0_TDM16SLOT_AHB_R::new(((self.bits >> 9) & 1) != 0)
    }
    #[doc = "Bit 10 - 1: Assert reset, 0: De-assert reset"]
    #[inline(always)]
    pub fn u0_tdm16slot_tdm(&self) -> U0_TDM16SLOT_TDM_R {
        U0_TDM16SLOT_TDM_R::new(((self.bits >> 10) & 1) != 0)
    }
    #[doc = "Bit 11 - 1: Assert reset, 0: De-assert reset"]
    #[inline(always)]
    pub fn u0_tdm16slot_apb(&self) -> U0_TDM16SLOT_APB_R {
        U0_TDM16SLOT_APB_R::new(((self.bits >> 11) & 1) != 0)
    }
    #[doc = "Bit 12 - 1: Assert reset, 0: De-assert reset"]
    #[inline(always)]
    pub fn u0_pwm_apb(&self) -> U0_PWM_APB_R {
        U0_PWM_APB_R::new(((self.bits >> 12) & 1) != 0)
    }
    #[doc = "Bit 13 - 1: Assert reset, 0: De-assert reset"]
    #[inline(always)]
    pub fn u0_dskit_wdt_rstn_apb(&self) -> U0_DSKIT_WDT_RSTN_APB_R {
        U0_DSKIT_WDT_RSTN_APB_R::new(((self.bits >> 13) & 1) != 0)
    }
    #[doc = "Bit 14 - 1: Assert reset, 0: De-assert reset"]
    #[inline(always)]
    pub fn u0_dskit_wdt(&self) -> U0_DSKIT_WDT_R {
        U0_DSKIT_WDT_R::new(((self.bits >> 14) & 1) != 0)
    }
    #[doc = "Bit 15 - 1: Assert reset, 0: De-assert reset"]
    #[inline(always)]
    pub fn u0_can_ctrl_apb(&self) -> U0_CAN_CTRL_APB_R {
        U0_CAN_CTRL_APB_R::new(((self.bits >> 15) & 1) != 0)
    }
    #[doc = "Bit 16 - 1: Assert reset, 0: De-assert reset"]
    #[inline(always)]
    pub fn u0_can_ctrl(&self) -> U0_CAN_CTRL_R {
        U0_CAN_CTRL_R::new(((self.bits >> 16) & 1) != 0)
    }
    #[doc = "Bit 17 - 1: Assert reset, 0: De-assert reset"]
    #[inline(always)]
    pub fn u0_can_ctrl_timer(&self) -> U0_CAN_CTRL_TIMER_R {
        U0_CAN_CTRL_TIMER_R::new(((self.bits >> 17) & 1) != 0)
    }
    #[doc = "Bit 18 - 1: Assert reset, 0: De-assert reset"]
    #[inline(always)]
    pub fn u1_can_ctrl_apb(&self) -> U1_CAN_CTRL_APB_R {
        U1_CAN_CTRL_APB_R::new(((self.bits >> 18) & 1) != 0)
    }
    #[doc = "Bit 19 - 1: Assert reset, 0: De-assert reset"]
    #[inline(always)]
    pub fn u1_can_ctrl_can(&self) -> U1_CAN_CTRL_CAN_R {
        U1_CAN_CTRL_CAN_R::new(((self.bits >> 19) & 1) != 0)
    }
    #[doc = "Bit 20 - 1: Assert reset, 0: De-assert reset"]
    #[inline(always)]
    pub fn u1_can_ctrl_timer(&self) -> U1_CAN_CTRL_TIMER_R {
        U1_CAN_CTRL_TIMER_R::new(((self.bits >> 20) & 1) != 0)
    }
    #[doc = "Bit 21 - 1: Assert reset, 0: De-assert reset"]
    #[inline(always)]
    pub fn u0_si5_timer_apb(&self) -> U0_SI5_TIMER_APB_R {
        U0_SI5_TIMER_APB_R::new(((self.bits >> 21) & 1) != 0)
    }
    #[doc = "Bit 22 - 1: Assert reset, 0: De-assert reset"]
    #[inline(always)]
    pub fn u0_si5_timer_0(&self) -> U0_SI5_TIMER_0_R {
        U0_SI5_TIMER_0_R::new(((self.bits >> 22) & 1) != 0)
    }
    #[doc = "Bit 23 - 1: Assert reset, 0: De-assert reset"]
    #[inline(always)]
    pub fn u0_si5_timer_1(&self) -> U0_SI5_TIMER_1_R {
        U0_SI5_TIMER_1_R::new(((self.bits >> 23) & 1) != 0)
    }
    #[doc = "Bit 24 - 1: Assert reset, 0: De-assert reset"]
    #[inline(always)]
    pub fn u0_si5_timer_2(&self) -> U0_SI5_TIMER_2_R {
        U0_SI5_TIMER_2_R::new(((self.bits >> 24) & 1) != 0)
    }
    #[doc = "Bit 25 - 1: Assert reset, 0: De-assert reset"]
    #[inline(always)]
    pub fn u0_si5_timer_3(&self) -> U0_SI5_TIMER_3_R {
        U0_SI5_TIMER_3_R::new(((self.bits >> 25) & 1) != 0)
    }
    #[doc = "Bit 26 - 1: Assert reset, 0: De-assert reset"]
    #[inline(always)]
    pub fn u0_int_ctrl_apb(&self) -> U0_INT_CTRL_APB_R {
        U0_INT_CTRL_APB_R::new(((self.bits >> 26) & 1) != 0)
    }
    #[doc = "Bit 27 - 1: Assert reset, 0: De-assert reset"]
    #[inline(always)]
    pub fn u0_temp_sensor_apb(&self) -> U0_TEMP_SENSOR_APB_R {
        U0_TEMP_SENSOR_APB_R::new(((self.bits >> 27) & 1) != 0)
    }
    #[doc = "Bit 28 - 1: Assert reset, 0: De-assert reset"]
    #[inline(always)]
    pub fn u0_temp_sensor(&self) -> U0_TEMP_SENSOR_R {
        U0_TEMP_SENSOR_R::new(((self.bits >> 28) & 1) != 0)
    }
    #[doc = "Bit 29 - 1: Assert reset, 0: De-assert reset"]
    #[inline(always)]
    pub fn u0_jtag_rst(&self) -> U0_JTAG_RST_R {
        U0_JTAG_RST_R::new(((self.bits >> 29) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - 1: Assert reset, 0: De-assert reset"]
    #[inline(always)]
    #[must_use]
    pub fn u0_pwmdac_apb(&mut self) -> U0_PWMDAC_APB_W<SOFT_RST_ADDR_SEL_3_SPEC> {
        U0_PWMDAC_APB_W::new(self, 0)
    }
    #[doc = "Bit 1 - 1: Assert reset, 0: De-assert reset"]
    #[inline(always)]
    #[must_use]
    pub fn u0_pdm_4mic_dmic(&mut self) -> U0_PDM_4MIC_DMIC_W<SOFT_RST_ADDR_SEL_3_SPEC> {
        U0_PDM_4MIC_DMIC_W::new(self, 1)
    }
    #[doc = "Bit 2 - 1: Assert reset, 0: De-assert reset"]
    #[inline(always)]
    #[must_use]
    pub fn u0_pdm_4mic_apb(&mut self) -> U0_PDM_4MIC_APB_W<SOFT_RST_ADDR_SEL_3_SPEC> {
        U0_PDM_4MIC_APB_W::new(self, 2)
    }
    #[doc = "Bit 3 - 1: Assert reset, 0: De-assert reset"]
    #[inline(always)]
    #[must_use]
    pub fn u0_i2srx_apb(&mut self) -> U0_I2SRX_APB_W<SOFT_RST_ADDR_SEL_3_SPEC> {
        U0_I2SRX_APB_W::new(self, 3)
    }
    #[doc = "Bit 4 - 1: Assert reset, 0: De-assert reset"]
    #[inline(always)]
    #[must_use]
    pub fn u0_i2srx_bclk(&mut self) -> U0_I2SRX_BCLK_W<SOFT_RST_ADDR_SEL_3_SPEC> {
        U0_I2SRX_BCLK_W::new(self, 4)
    }
    #[doc = "Bit 5 - 1: Assert reset, 0: De-assert reset"]
    #[inline(always)]
    #[must_use]
    pub fn u0_i2stx_apb(&mut self) -> U0_I2STX_APB_W<SOFT_RST_ADDR_SEL_3_SPEC> {
        U0_I2STX_APB_W::new(self, 5)
    }
    #[doc = "Bit 6 - 1: Assert reset, 0: De-assert reset"]
    #[inline(always)]
    #[must_use]
    pub fn u0_i2stx_bclk(&mut self) -> U0_I2STX_BCLK_W<SOFT_RST_ADDR_SEL_3_SPEC> {
        U0_I2STX_BCLK_W::new(self, 6)
    }
    #[doc = "Bit 7 - 1: Assert reset, 0: De-assert reset"]
    #[inline(always)]
    #[must_use]
    pub fn u1_i2stx_apb(&mut self) -> U1_I2STX_APB_W<SOFT_RST_ADDR_SEL_3_SPEC> {
        U1_I2STX_APB_W::new(self, 7)
    }
    #[doc = "Bit 8 - 1: Assert reset, 0: De-assert reset"]
    #[inline(always)]
    #[must_use]
    pub fn u1_i2stx_bclk(&mut self) -> U1_I2STX_BCLK_W<SOFT_RST_ADDR_SEL_3_SPEC> {
        U1_I2STX_BCLK_W::new(self, 8)
    }
    #[doc = "Bit 9 - 1: Assert reset, 0: De-assert reset"]
    #[inline(always)]
    #[must_use]
    pub fn u0_tdm16slot_ahb(&mut self) -> U0_TDM16SLOT_AHB_W<SOFT_RST_ADDR_SEL_3_SPEC> {
        U0_TDM16SLOT_AHB_W::new(self, 9)
    }
    #[doc = "Bit 10 - 1: Assert reset, 0: De-assert reset"]
    #[inline(always)]
    #[must_use]
    pub fn u0_tdm16slot_tdm(&mut self) -> U0_TDM16SLOT_TDM_W<SOFT_RST_ADDR_SEL_3_SPEC> {
        U0_TDM16SLOT_TDM_W::new(self, 10)
    }
    #[doc = "Bit 11 - 1: Assert reset, 0: De-assert reset"]
    #[inline(always)]
    #[must_use]
    pub fn u0_tdm16slot_apb(&mut self) -> U0_TDM16SLOT_APB_W<SOFT_RST_ADDR_SEL_3_SPEC> {
        U0_TDM16SLOT_APB_W::new(self, 11)
    }
    #[doc = "Bit 12 - 1: Assert reset, 0: De-assert reset"]
    #[inline(always)]
    #[must_use]
    pub fn u0_pwm_apb(&mut self) -> U0_PWM_APB_W<SOFT_RST_ADDR_SEL_3_SPEC> {
        U0_PWM_APB_W::new(self, 12)
    }
    #[doc = "Bit 13 - 1: Assert reset, 0: De-assert reset"]
    #[inline(always)]
    #[must_use]
    pub fn u0_dskit_wdt_rstn_apb(&mut self) -> U0_DSKIT_WDT_RSTN_APB_W<SOFT_RST_ADDR_SEL_3_SPEC> {
        U0_DSKIT_WDT_RSTN_APB_W::new(self, 13)
    }
    #[doc = "Bit 14 - 1: Assert reset, 0: De-assert reset"]
    #[inline(always)]
    #[must_use]
    pub fn u0_dskit_wdt(&mut self) -> U0_DSKIT_WDT_W<SOFT_RST_ADDR_SEL_3_SPEC> {
        U0_DSKIT_WDT_W::new(self, 14)
    }
    #[doc = "Bit 15 - 1: Assert reset, 0: De-assert reset"]
    #[inline(always)]
    #[must_use]
    pub fn u0_can_ctrl_apb(&mut self) -> U0_CAN_CTRL_APB_W<SOFT_RST_ADDR_SEL_3_SPEC> {
        U0_CAN_CTRL_APB_W::new(self, 15)
    }
    #[doc = "Bit 16 - 1: Assert reset, 0: De-assert reset"]
    #[inline(always)]
    #[must_use]
    pub fn u0_can_ctrl(&mut self) -> U0_CAN_CTRL_W<SOFT_RST_ADDR_SEL_3_SPEC> {
        U0_CAN_CTRL_W::new(self, 16)
    }
    #[doc = "Bit 17 - 1: Assert reset, 0: De-assert reset"]
    #[inline(always)]
    #[must_use]
    pub fn u0_can_ctrl_timer(&mut self) -> U0_CAN_CTRL_TIMER_W<SOFT_RST_ADDR_SEL_3_SPEC> {
        U0_CAN_CTRL_TIMER_W::new(self, 17)
    }
    #[doc = "Bit 18 - 1: Assert reset, 0: De-assert reset"]
    #[inline(always)]
    #[must_use]
    pub fn u1_can_ctrl_apb(&mut self) -> U1_CAN_CTRL_APB_W<SOFT_RST_ADDR_SEL_3_SPEC> {
        U1_CAN_CTRL_APB_W::new(self, 18)
    }
    #[doc = "Bit 19 - 1: Assert reset, 0: De-assert reset"]
    #[inline(always)]
    #[must_use]
    pub fn u1_can_ctrl_can(&mut self) -> U1_CAN_CTRL_CAN_W<SOFT_RST_ADDR_SEL_3_SPEC> {
        U1_CAN_CTRL_CAN_W::new(self, 19)
    }
    #[doc = "Bit 20 - 1: Assert reset, 0: De-assert reset"]
    #[inline(always)]
    #[must_use]
    pub fn u1_can_ctrl_timer(&mut self) -> U1_CAN_CTRL_TIMER_W<SOFT_RST_ADDR_SEL_3_SPEC> {
        U1_CAN_CTRL_TIMER_W::new(self, 20)
    }
    #[doc = "Bit 21 - 1: Assert reset, 0: De-assert reset"]
    #[inline(always)]
    #[must_use]
    pub fn u0_si5_timer_apb(&mut self) -> U0_SI5_TIMER_APB_W<SOFT_RST_ADDR_SEL_3_SPEC> {
        U0_SI5_TIMER_APB_W::new(self, 21)
    }
    #[doc = "Bit 22 - 1: Assert reset, 0: De-assert reset"]
    #[inline(always)]
    #[must_use]
    pub fn u0_si5_timer_0(&mut self) -> U0_SI5_TIMER_0_W<SOFT_RST_ADDR_SEL_3_SPEC> {
        U0_SI5_TIMER_0_W::new(self, 22)
    }
    #[doc = "Bit 23 - 1: Assert reset, 0: De-assert reset"]
    #[inline(always)]
    #[must_use]
    pub fn u0_si5_timer_1(&mut self) -> U0_SI5_TIMER_1_W<SOFT_RST_ADDR_SEL_3_SPEC> {
        U0_SI5_TIMER_1_W::new(self, 23)
    }
    #[doc = "Bit 24 - 1: Assert reset, 0: De-assert reset"]
    #[inline(always)]
    #[must_use]
    pub fn u0_si5_timer_2(&mut self) -> U0_SI5_TIMER_2_W<SOFT_RST_ADDR_SEL_3_SPEC> {
        U0_SI5_TIMER_2_W::new(self, 24)
    }
    #[doc = "Bit 25 - 1: Assert reset, 0: De-assert reset"]
    #[inline(always)]
    #[must_use]
    pub fn u0_si5_timer_3(&mut self) -> U0_SI5_TIMER_3_W<SOFT_RST_ADDR_SEL_3_SPEC> {
        U0_SI5_TIMER_3_W::new(self, 25)
    }
    #[doc = "Bit 26 - 1: Assert reset, 0: De-assert reset"]
    #[inline(always)]
    #[must_use]
    pub fn u0_int_ctrl_apb(&mut self) -> U0_INT_CTRL_APB_W<SOFT_RST_ADDR_SEL_3_SPEC> {
        U0_INT_CTRL_APB_W::new(self, 26)
    }
    #[doc = "Bit 27 - 1: Assert reset, 0: De-assert reset"]
    #[inline(always)]
    #[must_use]
    pub fn u0_temp_sensor_apb(&mut self) -> U0_TEMP_SENSOR_APB_W<SOFT_RST_ADDR_SEL_3_SPEC> {
        U0_TEMP_SENSOR_APB_W::new(self, 27)
    }
    #[doc = "Bit 28 - 1: Assert reset, 0: De-assert reset"]
    #[inline(always)]
    #[must_use]
    pub fn u0_temp_sensor(&mut self) -> U0_TEMP_SENSOR_W<SOFT_RST_ADDR_SEL_3_SPEC> {
        U0_TEMP_SENSOR_W::new(self, 28)
    }
    #[doc = "Bit 29 - 1: Assert reset, 0: De-assert reset"]
    #[inline(always)]
    #[must_use]
    pub fn u0_jtag_rst(&mut self) -> U0_JTAG_RST_W<SOFT_RST_ADDR_SEL_3_SPEC> {
        U0_JTAG_RST_W::new(self, 29)
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
#[doc = "Software RESET 3 Address Selector\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`soft_rst_addr_sel_3::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`soft_rst_addr_sel_3::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct SOFT_RST_ADDR_SEL_3_SPEC;
impl crate::RegisterSpec for SOFT_RST_ADDR_SEL_3_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`soft_rst_addr_sel_3::R`](R) reader structure"]
impl crate::Readable for SOFT_RST_ADDR_SEL_3_SPEC {}
#[doc = "`write(|w| ..)` method takes [`soft_rst_addr_sel_3::W`](W) writer structure"]
impl crate::Writable for SOFT_RST_ADDR_SEL_3_SPEC {
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets soft_rst_addr_sel_3 to value 0"]
impl crate::Resettable for SOFT_RST_ADDR_SEL_3_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
