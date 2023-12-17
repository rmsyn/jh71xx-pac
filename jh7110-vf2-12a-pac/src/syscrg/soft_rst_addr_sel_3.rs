#[doc = "Register `soft_rst_addr_sel_3` reader"]
pub type R = crate::R<SOFT_RST_ADDR_SEL_3_SPEC>;
#[doc = "Register `soft_rst_addr_sel_3` writer"]
pub type W = crate::W<SOFT_RST_ADDR_SEL_3_SPEC>;
#[doc = "Field `rstn_u0_pwmdac_rstn_apb` reader - 1: Assert reset, 0: De-assert reset"]
pub type RSTN_U0_PWMDAC_RSTN_APB_R = crate::BitReader;
#[doc = "Field `rstn_u0_pwmdac_rstn_apb` writer - 1: Assert reset, 0: De-assert reset"]
pub type RSTN_U0_PWMDAC_RSTN_APB_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `rstn_u0_pdm_4mic_rstn_dmic` reader - 1: Assert reset, 0: De-assert reset"]
pub type RSTN_U0_PDM_4MIC_RSTN_DMIC_R = crate::BitReader;
#[doc = "Field `rstn_u0_pdm_4mic_rstn_dmic` writer - 1: Assert reset, 0: De-assert reset"]
pub type RSTN_U0_PDM_4MIC_RSTN_DMIC_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `rstn_u0_pdm_4mic_rstn_apb` reader - 1: Assert reset, 0: De-assert reset"]
pub type RSTN_U0_PDM_4MIC_RSTN_APB_R = crate::BitReader;
#[doc = "Field `rstn_u0_pdm_4mic_rstn_apb` writer - 1: Assert reset, 0: De-assert reset"]
pub type RSTN_U0_PDM_4MIC_RSTN_APB_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `rstn_u0_i2srx_3ch_rstn_apb` reader - 1: Assert reset, 0: De-assert reset"]
pub type RSTN_U0_I2SRX_3CH_RSTN_APB_R = crate::BitReader;
#[doc = "Field `rstn_u0_i2srx_3ch_rstn_apb` writer - 1: Assert reset, 0: De-assert reset"]
pub type RSTN_U0_I2SRX_3CH_RSTN_APB_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `rstn_u0_i2srx_3ch_rstn_bclk` reader - 1: Assert reset, 0: De-assert reset"]
pub type RSTN_U0_I2SRX_3CH_RSTN_BCLK_R = crate::BitReader;
#[doc = "Field `rstn_u0_i2srx_3ch_rstn_bclk` writer - 1: Assert reset, 0: De-assert reset"]
pub type RSTN_U0_I2SRX_3CH_RSTN_BCLK_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `rstn_u0_i2stx_4ch_rstn_apb` reader - 1: Assert reset, 0: De-assert reset"]
pub type RSTN_U0_I2STX_4CH_RSTN_APB_R = crate::BitReader;
#[doc = "Field `rstn_u0_i2stx_4ch_rstn_apb` writer - 1: Assert reset, 0: De-assert reset"]
pub type RSTN_U0_I2STX_4CH_RSTN_APB_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `rstn_u0_i2stx_4ch_rstn_bclk` reader - 1: Assert reset, 0: De-assert reset"]
pub type RSTN_U0_I2STX_4CH_RSTN_BCLK_R = crate::BitReader;
#[doc = "Field `rstn_u0_i2stx_4ch_rstn_bclk` writer - 1: Assert reset, 0: De-assert reset"]
pub type RSTN_U0_I2STX_4CH_RSTN_BCLK_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `rstn_u1_i2stx_4ch_rstn_apb` reader - 1: Assert reset, 0: De-assert reset"]
pub type RSTN_U1_I2STX_4CH_RSTN_APB_R = crate::BitReader;
#[doc = "Field `rstn_u1_i2stx_4ch_rstn_apb` writer - 1: Assert reset, 0: De-assert reset"]
pub type RSTN_U1_I2STX_4CH_RSTN_APB_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `rstn_u1_i2stx_4ch_rstn_bclk` reader - 1: Assert reset, 0: De-assert reset"]
pub type RSTN_U1_I2STX_4CH_RSTN_BCLK_R = crate::BitReader;
#[doc = "Field `rstn_u1_i2stx_4ch_rstn_bclk` writer - 1: Assert reset, 0: De-assert reset"]
pub type RSTN_U1_I2STX_4CH_RSTN_BCLK_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `rstn_u0_tdm16slot_rstn_ahb` reader - 1: Assert reset, 0: De-assert reset"]
pub type RSTN_U0_TDM16SLOT_RSTN_AHB_R = crate::BitReader;
#[doc = "Field `rstn_u0_tdm16slot_rstn_ahb` writer - 1: Assert reset, 0: De-assert reset"]
pub type RSTN_U0_TDM16SLOT_RSTN_AHB_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `rstn_u0_tdm16slot_rstn_tdm` reader - 1: Assert reset, 0: De-assert reset"]
pub type RSTN_U0_TDM16SLOT_RSTN_TDM_R = crate::BitReader;
#[doc = "Field `rstn_u0_tdm16slot_rstn_tdm` writer - 1: Assert reset, 0: De-assert reset"]
pub type RSTN_U0_TDM16SLOT_RSTN_TDM_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `rstn_u0_tdm16slot_rstn_apb` reader - 1: Assert reset, 0: De-assert reset"]
pub type RSTN_U0_TDM16SLOT_RSTN_APB_R = crate::BitReader;
#[doc = "Field `rstn_u0_tdm16slot_rstn_apb` writer - 1: Assert reset, 0: De-assert reset"]
pub type RSTN_U0_TDM16SLOT_RSTN_APB_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `rstn_u0_pwm_8ch_rstn_apb` reader - 1: Assert reset, 0: De-assert reset"]
pub type RSTN_U0_PWM_8CH_RSTN_APB_R = crate::BitReader;
#[doc = "Field `rstn_u0_pwm_8ch_rstn_apb` writer - 1: Assert reset, 0: De-assert reset"]
pub type RSTN_U0_PWM_8CH_RSTN_APB_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `rstn_u0_dskit_wdt_rstn_apb` reader - 1: Assert reset, 0: De-assert reset"]
pub type RSTN_U0_DSKIT_WDT_RSTN_APB_R = crate::BitReader;
#[doc = "Field `rstn_u0_dskit_wdt_rstn_apb` writer - 1: Assert reset, 0: De-assert reset"]
pub type RSTN_U0_DSKIT_WDT_RSTN_APB_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `rstn_u0_dskit_wdt_rstn_wdt` reader - 1: Assert reset, 0: De-assert reset"]
pub type RSTN_U0_DSKIT_WDT_RSTN_WDT_R = crate::BitReader;
#[doc = "Field `rstn_u0_dskit_wdt_rstn_wdt` writer - 1: Assert reset, 0: De-assert reset"]
pub type RSTN_U0_DSKIT_WDT_RSTN_WDT_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `rstn_u0_can_ctrl_rstn_apb` reader - 1: Assert reset, 0: De-assert reset"]
pub type RSTN_U0_CAN_CTRL_RSTN_APB_R = crate::BitReader;
#[doc = "Field `rstn_u0_can_ctrl_rstn_apb` writer - 1: Assert reset, 0: De-assert reset"]
pub type RSTN_U0_CAN_CTRL_RSTN_APB_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `rstn_u0_can_ctrl_rstn_can` reader - 1: Assert reset, 0: De-assert reset"]
pub type RSTN_U0_CAN_CTRL_RSTN_CAN_R = crate::BitReader;
#[doc = "Field `rstn_u0_can_ctrl_rstn_can` writer - 1: Assert reset, 0: De-assert reset"]
pub type RSTN_U0_CAN_CTRL_RSTN_CAN_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `rstn_u0_can_ctrl_rstn_timer` reader - 1: Assert reset, 0: De-assert reset"]
pub type RSTN_U0_CAN_CTRL_RSTN_TIMER_R = crate::BitReader;
#[doc = "Field `rstn_u0_can_ctrl_rstn_timer` writer - 1: Assert reset, 0: De-assert reset"]
pub type RSTN_U0_CAN_CTRL_RSTN_TIMER_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `rstn_u1_can_ctrl_rstn_apb` reader - 1: Assert reset, 0: De-assert reset"]
pub type RSTN_U1_CAN_CTRL_RSTN_APB_R = crate::BitReader;
#[doc = "Field `rstn_u1_can_ctrl_rstn_apb` writer - 1: Assert reset, 0: De-assert reset"]
pub type RSTN_U1_CAN_CTRL_RSTN_APB_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `rstn_u1_can_ctrl_rstn_can` reader - 1: Assert reset, 0: De-assert reset"]
pub type RSTN_U1_CAN_CTRL_RSTN_CAN_R = crate::BitReader;
#[doc = "Field `rstn_u1_can_ctrl_rstn_can` writer - 1: Assert reset, 0: De-assert reset"]
pub type RSTN_U1_CAN_CTRL_RSTN_CAN_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `rstn_u1_can_ctrl_rstn_timer` reader - 1: Assert reset, 0: De-assert reset"]
pub type RSTN_U1_CAN_CTRL_RSTN_TIMER_R = crate::BitReader;
#[doc = "Field `rstn_u1_can_ctrl_rstn_timer` writer - 1: Assert reset, 0: De-assert reset"]
pub type RSTN_U1_CAN_CTRL_RSTN_TIMER_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `rstn_u0_si5_timer_rstn_apb` reader - 1: Assert reset, 0: De-assert reset"]
pub type RSTN_U0_SI5_TIMER_RSTN_APB_R = crate::BitReader;
#[doc = "Field `rstn_u0_si5_timer_rstn_apb` writer - 1: Assert reset, 0: De-assert reset"]
pub type RSTN_U0_SI5_TIMER_RSTN_APB_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `rstn_u0_si5_timer_rstn_timer0` reader - 1: Assert reset, 0: De-assert reset"]
pub type RSTN_U0_SI5_TIMER_RSTN_TIMER0_R = crate::BitReader;
#[doc = "Field `rstn_u0_si5_timer_rstn_timer0` writer - 1: Assert reset, 0: De-assert reset"]
pub type RSTN_U0_SI5_TIMER_RSTN_TIMER0_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `rstn_u0_si5_timer_rstn_time10` reader - 1: Assert reset, 0: De-assert reset"]
pub type RSTN_U0_SI5_TIMER_RSTN_TIME10_R = crate::BitReader;
#[doc = "Field `rstn_u0_si5_timer_rstn_time10` writer - 1: Assert reset, 0: De-assert reset"]
pub type RSTN_U0_SI5_TIMER_RSTN_TIME10_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `rstn_u0_si5_timer_rstn_timer2` reader - 1: Assert reset, 0: De-assert reset"]
pub type RSTN_U0_SI5_TIMER_RSTN_TIMER2_R = crate::BitReader;
#[doc = "Field `rstn_u0_si5_timer_rstn_timer2` writer - 1: Assert reset, 0: De-assert reset"]
pub type RSTN_U0_SI5_TIMER_RSTN_TIMER2_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `rstn_u0_si5_timer_rstn_timer3` reader - 1: Assert reset, 0: De-assert reset"]
pub type RSTN_U0_SI5_TIMER_RSTN_TIMER3_R = crate::BitReader;
#[doc = "Field `rstn_u0_si5_timer_rstn_timer3` writer - 1: Assert reset, 0: De-assert reset"]
pub type RSTN_U0_SI5_TIMER_RSTN_TIMER3_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `rstn_u0_int_ctrl_rstn_apb` reader - 1: Assert reset, 0: De-assert reset"]
pub type RSTN_U0_INT_CTRL_RSTN_APB_R = crate::BitReader;
#[doc = "Field `rstn_u0_int_ctrl_rstn_apb` writer - 1: Assert reset, 0: De-assert reset"]
pub type RSTN_U0_INT_CTRL_RSTN_APB_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `rstn_u0_temp_sensor_rstn_apb` reader - 1: Assert reset, 0: De-assert reset"]
pub type RSTN_U0_TEMP_SENSOR_RSTN_APB_R = crate::BitReader;
#[doc = "Field `rstn_u0_temp_sensor_rstn_apb` writer - 1: Assert reset, 0: De-assert reset"]
pub type RSTN_U0_TEMP_SENSOR_RSTN_APB_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `rstn_u0_temp_sensor_rstn_temp` reader - 1: Assert reset, 0: De-assert reset"]
pub type RSTN_U0_TEMP_SENSOR_RSTN_TEMP_R = crate::BitReader;
#[doc = "Field `rstn_u0_temp_sensor_rstn_temp` writer - 1: Assert reset, 0: De-assert reset"]
pub type RSTN_U0_TEMP_SENSOR_RSTN_TEMP_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `rstn_u0_jtag_certification_rst_n` reader - 1: Assert reset, 0: De-assert reset"]
pub type RSTN_U0_JTAG_CERTIFICATION_RST_N_R = crate::BitReader;
#[doc = "Field `rstn_u0_jtag_certification_rst_n` writer - 1: Assert reset, 0: De-assert reset"]
pub type RSTN_U0_JTAG_CERTIFICATION_RST_N_W<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bit 0 - 1: Assert reset, 0: De-assert reset"]
    #[inline(always)]
    pub fn rstn_u0_pwmdac_rstn_apb(&self) -> RSTN_U0_PWMDAC_RSTN_APB_R {
        RSTN_U0_PWMDAC_RSTN_APB_R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - 1: Assert reset, 0: De-assert reset"]
    #[inline(always)]
    pub fn rstn_u0_pdm_4mic_rstn_dmic(&self) -> RSTN_U0_PDM_4MIC_RSTN_DMIC_R {
        RSTN_U0_PDM_4MIC_RSTN_DMIC_R::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - 1: Assert reset, 0: De-assert reset"]
    #[inline(always)]
    pub fn rstn_u0_pdm_4mic_rstn_apb(&self) -> RSTN_U0_PDM_4MIC_RSTN_APB_R {
        RSTN_U0_PDM_4MIC_RSTN_APB_R::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3 - 1: Assert reset, 0: De-assert reset"]
    #[inline(always)]
    pub fn rstn_u0_i2srx_3ch_rstn_apb(&self) -> RSTN_U0_I2SRX_3CH_RSTN_APB_R {
        RSTN_U0_I2SRX_3CH_RSTN_APB_R::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bit 4 - 1: Assert reset, 0: De-assert reset"]
    #[inline(always)]
    pub fn rstn_u0_i2srx_3ch_rstn_bclk(&self) -> RSTN_U0_I2SRX_3CH_RSTN_BCLK_R {
        RSTN_U0_I2SRX_3CH_RSTN_BCLK_R::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bit 5 - 1: Assert reset, 0: De-assert reset"]
    #[inline(always)]
    pub fn rstn_u0_i2stx_4ch_rstn_apb(&self) -> RSTN_U0_I2STX_4CH_RSTN_APB_R {
        RSTN_U0_I2STX_4CH_RSTN_APB_R::new(((self.bits >> 5) & 1) != 0)
    }
    #[doc = "Bit 6 - 1: Assert reset, 0: De-assert reset"]
    #[inline(always)]
    pub fn rstn_u0_i2stx_4ch_rstn_bclk(&self) -> RSTN_U0_I2STX_4CH_RSTN_BCLK_R {
        RSTN_U0_I2STX_4CH_RSTN_BCLK_R::new(((self.bits >> 6) & 1) != 0)
    }
    #[doc = "Bit 7 - 1: Assert reset, 0: De-assert reset"]
    #[inline(always)]
    pub fn rstn_u1_i2stx_4ch_rstn_apb(&self) -> RSTN_U1_I2STX_4CH_RSTN_APB_R {
        RSTN_U1_I2STX_4CH_RSTN_APB_R::new(((self.bits >> 7) & 1) != 0)
    }
    #[doc = "Bit 8 - 1: Assert reset, 0: De-assert reset"]
    #[inline(always)]
    pub fn rstn_u1_i2stx_4ch_rstn_bclk(&self) -> RSTN_U1_I2STX_4CH_RSTN_BCLK_R {
        RSTN_U1_I2STX_4CH_RSTN_BCLK_R::new(((self.bits >> 8) & 1) != 0)
    }
    #[doc = "Bit 9 - 1: Assert reset, 0: De-assert reset"]
    #[inline(always)]
    pub fn rstn_u0_tdm16slot_rstn_ahb(&self) -> RSTN_U0_TDM16SLOT_RSTN_AHB_R {
        RSTN_U0_TDM16SLOT_RSTN_AHB_R::new(((self.bits >> 9) & 1) != 0)
    }
    #[doc = "Bit 10 - 1: Assert reset, 0: De-assert reset"]
    #[inline(always)]
    pub fn rstn_u0_tdm16slot_rstn_tdm(&self) -> RSTN_U0_TDM16SLOT_RSTN_TDM_R {
        RSTN_U0_TDM16SLOT_RSTN_TDM_R::new(((self.bits >> 10) & 1) != 0)
    }
    #[doc = "Bit 11 - 1: Assert reset, 0: De-assert reset"]
    #[inline(always)]
    pub fn rstn_u0_tdm16slot_rstn_apb(&self) -> RSTN_U0_TDM16SLOT_RSTN_APB_R {
        RSTN_U0_TDM16SLOT_RSTN_APB_R::new(((self.bits >> 11) & 1) != 0)
    }
    #[doc = "Bit 12 - 1: Assert reset, 0: De-assert reset"]
    #[inline(always)]
    pub fn rstn_u0_pwm_8ch_rstn_apb(&self) -> RSTN_U0_PWM_8CH_RSTN_APB_R {
        RSTN_U0_PWM_8CH_RSTN_APB_R::new(((self.bits >> 12) & 1) != 0)
    }
    #[doc = "Bit 13 - 1: Assert reset, 0: De-assert reset"]
    #[inline(always)]
    pub fn rstn_u0_dskit_wdt_rstn_apb(&self) -> RSTN_U0_DSKIT_WDT_RSTN_APB_R {
        RSTN_U0_DSKIT_WDT_RSTN_APB_R::new(((self.bits >> 13) & 1) != 0)
    }
    #[doc = "Bit 14 - 1: Assert reset, 0: De-assert reset"]
    #[inline(always)]
    pub fn rstn_u0_dskit_wdt_rstn_wdt(&self) -> RSTN_U0_DSKIT_WDT_RSTN_WDT_R {
        RSTN_U0_DSKIT_WDT_RSTN_WDT_R::new(((self.bits >> 14) & 1) != 0)
    }
    #[doc = "Bit 15 - 1: Assert reset, 0: De-assert reset"]
    #[inline(always)]
    pub fn rstn_u0_can_ctrl_rstn_apb(&self) -> RSTN_U0_CAN_CTRL_RSTN_APB_R {
        RSTN_U0_CAN_CTRL_RSTN_APB_R::new(((self.bits >> 15) & 1) != 0)
    }
    #[doc = "Bit 16 - 1: Assert reset, 0: De-assert reset"]
    #[inline(always)]
    pub fn rstn_u0_can_ctrl_rstn_can(&self) -> RSTN_U0_CAN_CTRL_RSTN_CAN_R {
        RSTN_U0_CAN_CTRL_RSTN_CAN_R::new(((self.bits >> 16) & 1) != 0)
    }
    #[doc = "Bit 17 - 1: Assert reset, 0: De-assert reset"]
    #[inline(always)]
    pub fn rstn_u0_can_ctrl_rstn_timer(&self) -> RSTN_U0_CAN_CTRL_RSTN_TIMER_R {
        RSTN_U0_CAN_CTRL_RSTN_TIMER_R::new(((self.bits >> 17) & 1) != 0)
    }
    #[doc = "Bit 18 - 1: Assert reset, 0: De-assert reset"]
    #[inline(always)]
    pub fn rstn_u1_can_ctrl_rstn_apb(&self) -> RSTN_U1_CAN_CTRL_RSTN_APB_R {
        RSTN_U1_CAN_CTRL_RSTN_APB_R::new(((self.bits >> 18) & 1) != 0)
    }
    #[doc = "Bit 19 - 1: Assert reset, 0: De-assert reset"]
    #[inline(always)]
    pub fn rstn_u1_can_ctrl_rstn_can(&self) -> RSTN_U1_CAN_CTRL_RSTN_CAN_R {
        RSTN_U1_CAN_CTRL_RSTN_CAN_R::new(((self.bits >> 19) & 1) != 0)
    }
    #[doc = "Bit 20 - 1: Assert reset, 0: De-assert reset"]
    #[inline(always)]
    pub fn rstn_u1_can_ctrl_rstn_timer(&self) -> RSTN_U1_CAN_CTRL_RSTN_TIMER_R {
        RSTN_U1_CAN_CTRL_RSTN_TIMER_R::new(((self.bits >> 20) & 1) != 0)
    }
    #[doc = "Bit 21 - 1: Assert reset, 0: De-assert reset"]
    #[inline(always)]
    pub fn rstn_u0_si5_timer_rstn_apb(&self) -> RSTN_U0_SI5_TIMER_RSTN_APB_R {
        RSTN_U0_SI5_TIMER_RSTN_APB_R::new(((self.bits >> 21) & 1) != 0)
    }
    #[doc = "Bit 22 - 1: Assert reset, 0: De-assert reset"]
    #[inline(always)]
    pub fn rstn_u0_si5_timer_rstn_timer0(&self) -> RSTN_U0_SI5_TIMER_RSTN_TIMER0_R {
        RSTN_U0_SI5_TIMER_RSTN_TIMER0_R::new(((self.bits >> 22) & 1) != 0)
    }
    #[doc = "Bit 23 - 1: Assert reset, 0: De-assert reset"]
    #[inline(always)]
    pub fn rstn_u0_si5_timer_rstn_time10(&self) -> RSTN_U0_SI5_TIMER_RSTN_TIME10_R {
        RSTN_U0_SI5_TIMER_RSTN_TIME10_R::new(((self.bits >> 23) & 1) != 0)
    }
    #[doc = "Bit 24 - 1: Assert reset, 0: De-assert reset"]
    #[inline(always)]
    pub fn rstn_u0_si5_timer_rstn_timer2(&self) -> RSTN_U0_SI5_TIMER_RSTN_TIMER2_R {
        RSTN_U0_SI5_TIMER_RSTN_TIMER2_R::new(((self.bits >> 24) & 1) != 0)
    }
    #[doc = "Bit 25 - 1: Assert reset, 0: De-assert reset"]
    #[inline(always)]
    pub fn rstn_u0_si5_timer_rstn_timer3(&self) -> RSTN_U0_SI5_TIMER_RSTN_TIMER3_R {
        RSTN_U0_SI5_TIMER_RSTN_TIMER3_R::new(((self.bits >> 25) & 1) != 0)
    }
    #[doc = "Bit 26 - 1: Assert reset, 0: De-assert reset"]
    #[inline(always)]
    pub fn rstn_u0_int_ctrl_rstn_apb(&self) -> RSTN_U0_INT_CTRL_RSTN_APB_R {
        RSTN_U0_INT_CTRL_RSTN_APB_R::new(((self.bits >> 26) & 1) != 0)
    }
    #[doc = "Bit 27 - 1: Assert reset, 0: De-assert reset"]
    #[inline(always)]
    pub fn rstn_u0_temp_sensor_rstn_apb(&self) -> RSTN_U0_TEMP_SENSOR_RSTN_APB_R {
        RSTN_U0_TEMP_SENSOR_RSTN_APB_R::new(((self.bits >> 27) & 1) != 0)
    }
    #[doc = "Bit 28 - 1: Assert reset, 0: De-assert reset"]
    #[inline(always)]
    pub fn rstn_u0_temp_sensor_rstn_temp(&self) -> RSTN_U0_TEMP_SENSOR_RSTN_TEMP_R {
        RSTN_U0_TEMP_SENSOR_RSTN_TEMP_R::new(((self.bits >> 28) & 1) != 0)
    }
    #[doc = "Bit 29 - 1: Assert reset, 0: De-assert reset"]
    #[inline(always)]
    pub fn rstn_u0_jtag_certification_rst_n(&self) -> RSTN_U0_JTAG_CERTIFICATION_RST_N_R {
        RSTN_U0_JTAG_CERTIFICATION_RST_N_R::new(((self.bits >> 29) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - 1: Assert reset, 0: De-assert reset"]
    #[inline(always)]
    #[must_use]
    pub fn rstn_u0_pwmdac_rstn_apb(
        &mut self,
    ) -> RSTN_U0_PWMDAC_RSTN_APB_W<SOFT_RST_ADDR_SEL_3_SPEC> {
        RSTN_U0_PWMDAC_RSTN_APB_W::new(self, 0)
    }
    #[doc = "Bit 1 - 1: Assert reset, 0: De-assert reset"]
    #[inline(always)]
    #[must_use]
    pub fn rstn_u0_pdm_4mic_rstn_dmic(
        &mut self,
    ) -> RSTN_U0_PDM_4MIC_RSTN_DMIC_W<SOFT_RST_ADDR_SEL_3_SPEC> {
        RSTN_U0_PDM_4MIC_RSTN_DMIC_W::new(self, 1)
    }
    #[doc = "Bit 2 - 1: Assert reset, 0: De-assert reset"]
    #[inline(always)]
    #[must_use]
    pub fn rstn_u0_pdm_4mic_rstn_apb(
        &mut self,
    ) -> RSTN_U0_PDM_4MIC_RSTN_APB_W<SOFT_RST_ADDR_SEL_3_SPEC> {
        RSTN_U0_PDM_4MIC_RSTN_APB_W::new(self, 2)
    }
    #[doc = "Bit 3 - 1: Assert reset, 0: De-assert reset"]
    #[inline(always)]
    #[must_use]
    pub fn rstn_u0_i2srx_3ch_rstn_apb(
        &mut self,
    ) -> RSTN_U0_I2SRX_3CH_RSTN_APB_W<SOFT_RST_ADDR_SEL_3_SPEC> {
        RSTN_U0_I2SRX_3CH_RSTN_APB_W::new(self, 3)
    }
    #[doc = "Bit 4 - 1: Assert reset, 0: De-assert reset"]
    #[inline(always)]
    #[must_use]
    pub fn rstn_u0_i2srx_3ch_rstn_bclk(
        &mut self,
    ) -> RSTN_U0_I2SRX_3CH_RSTN_BCLK_W<SOFT_RST_ADDR_SEL_3_SPEC> {
        RSTN_U0_I2SRX_3CH_RSTN_BCLK_W::new(self, 4)
    }
    #[doc = "Bit 5 - 1: Assert reset, 0: De-assert reset"]
    #[inline(always)]
    #[must_use]
    pub fn rstn_u0_i2stx_4ch_rstn_apb(
        &mut self,
    ) -> RSTN_U0_I2STX_4CH_RSTN_APB_W<SOFT_RST_ADDR_SEL_3_SPEC> {
        RSTN_U0_I2STX_4CH_RSTN_APB_W::new(self, 5)
    }
    #[doc = "Bit 6 - 1: Assert reset, 0: De-assert reset"]
    #[inline(always)]
    #[must_use]
    pub fn rstn_u0_i2stx_4ch_rstn_bclk(
        &mut self,
    ) -> RSTN_U0_I2STX_4CH_RSTN_BCLK_W<SOFT_RST_ADDR_SEL_3_SPEC> {
        RSTN_U0_I2STX_4CH_RSTN_BCLK_W::new(self, 6)
    }
    #[doc = "Bit 7 - 1: Assert reset, 0: De-assert reset"]
    #[inline(always)]
    #[must_use]
    pub fn rstn_u1_i2stx_4ch_rstn_apb(
        &mut self,
    ) -> RSTN_U1_I2STX_4CH_RSTN_APB_W<SOFT_RST_ADDR_SEL_3_SPEC> {
        RSTN_U1_I2STX_4CH_RSTN_APB_W::new(self, 7)
    }
    #[doc = "Bit 8 - 1: Assert reset, 0: De-assert reset"]
    #[inline(always)]
    #[must_use]
    pub fn rstn_u1_i2stx_4ch_rstn_bclk(
        &mut self,
    ) -> RSTN_U1_I2STX_4CH_RSTN_BCLK_W<SOFT_RST_ADDR_SEL_3_SPEC> {
        RSTN_U1_I2STX_4CH_RSTN_BCLK_W::new(self, 8)
    }
    #[doc = "Bit 9 - 1: Assert reset, 0: De-assert reset"]
    #[inline(always)]
    #[must_use]
    pub fn rstn_u0_tdm16slot_rstn_ahb(
        &mut self,
    ) -> RSTN_U0_TDM16SLOT_RSTN_AHB_W<SOFT_RST_ADDR_SEL_3_SPEC> {
        RSTN_U0_TDM16SLOT_RSTN_AHB_W::new(self, 9)
    }
    #[doc = "Bit 10 - 1: Assert reset, 0: De-assert reset"]
    #[inline(always)]
    #[must_use]
    pub fn rstn_u0_tdm16slot_rstn_tdm(
        &mut self,
    ) -> RSTN_U0_TDM16SLOT_RSTN_TDM_W<SOFT_RST_ADDR_SEL_3_SPEC> {
        RSTN_U0_TDM16SLOT_RSTN_TDM_W::new(self, 10)
    }
    #[doc = "Bit 11 - 1: Assert reset, 0: De-assert reset"]
    #[inline(always)]
    #[must_use]
    pub fn rstn_u0_tdm16slot_rstn_apb(
        &mut self,
    ) -> RSTN_U0_TDM16SLOT_RSTN_APB_W<SOFT_RST_ADDR_SEL_3_SPEC> {
        RSTN_U0_TDM16SLOT_RSTN_APB_W::new(self, 11)
    }
    #[doc = "Bit 12 - 1: Assert reset, 0: De-assert reset"]
    #[inline(always)]
    #[must_use]
    pub fn rstn_u0_pwm_8ch_rstn_apb(
        &mut self,
    ) -> RSTN_U0_PWM_8CH_RSTN_APB_W<SOFT_RST_ADDR_SEL_3_SPEC> {
        RSTN_U0_PWM_8CH_RSTN_APB_W::new(self, 12)
    }
    #[doc = "Bit 13 - 1: Assert reset, 0: De-assert reset"]
    #[inline(always)]
    #[must_use]
    pub fn rstn_u0_dskit_wdt_rstn_apb(
        &mut self,
    ) -> RSTN_U0_DSKIT_WDT_RSTN_APB_W<SOFT_RST_ADDR_SEL_3_SPEC> {
        RSTN_U0_DSKIT_WDT_RSTN_APB_W::new(self, 13)
    }
    #[doc = "Bit 14 - 1: Assert reset, 0: De-assert reset"]
    #[inline(always)]
    #[must_use]
    pub fn rstn_u0_dskit_wdt_rstn_wdt(
        &mut self,
    ) -> RSTN_U0_DSKIT_WDT_RSTN_WDT_W<SOFT_RST_ADDR_SEL_3_SPEC> {
        RSTN_U0_DSKIT_WDT_RSTN_WDT_W::new(self, 14)
    }
    #[doc = "Bit 15 - 1: Assert reset, 0: De-assert reset"]
    #[inline(always)]
    #[must_use]
    pub fn rstn_u0_can_ctrl_rstn_apb(
        &mut self,
    ) -> RSTN_U0_CAN_CTRL_RSTN_APB_W<SOFT_RST_ADDR_SEL_3_SPEC> {
        RSTN_U0_CAN_CTRL_RSTN_APB_W::new(self, 15)
    }
    #[doc = "Bit 16 - 1: Assert reset, 0: De-assert reset"]
    #[inline(always)]
    #[must_use]
    pub fn rstn_u0_can_ctrl_rstn_can(
        &mut self,
    ) -> RSTN_U0_CAN_CTRL_RSTN_CAN_W<SOFT_RST_ADDR_SEL_3_SPEC> {
        RSTN_U0_CAN_CTRL_RSTN_CAN_W::new(self, 16)
    }
    #[doc = "Bit 17 - 1: Assert reset, 0: De-assert reset"]
    #[inline(always)]
    #[must_use]
    pub fn rstn_u0_can_ctrl_rstn_timer(
        &mut self,
    ) -> RSTN_U0_CAN_CTRL_RSTN_TIMER_W<SOFT_RST_ADDR_SEL_3_SPEC> {
        RSTN_U0_CAN_CTRL_RSTN_TIMER_W::new(self, 17)
    }
    #[doc = "Bit 18 - 1: Assert reset, 0: De-assert reset"]
    #[inline(always)]
    #[must_use]
    pub fn rstn_u1_can_ctrl_rstn_apb(
        &mut self,
    ) -> RSTN_U1_CAN_CTRL_RSTN_APB_W<SOFT_RST_ADDR_SEL_3_SPEC> {
        RSTN_U1_CAN_CTRL_RSTN_APB_W::new(self, 18)
    }
    #[doc = "Bit 19 - 1: Assert reset, 0: De-assert reset"]
    #[inline(always)]
    #[must_use]
    pub fn rstn_u1_can_ctrl_rstn_can(
        &mut self,
    ) -> RSTN_U1_CAN_CTRL_RSTN_CAN_W<SOFT_RST_ADDR_SEL_3_SPEC> {
        RSTN_U1_CAN_CTRL_RSTN_CAN_W::new(self, 19)
    }
    #[doc = "Bit 20 - 1: Assert reset, 0: De-assert reset"]
    #[inline(always)]
    #[must_use]
    pub fn rstn_u1_can_ctrl_rstn_timer(
        &mut self,
    ) -> RSTN_U1_CAN_CTRL_RSTN_TIMER_W<SOFT_RST_ADDR_SEL_3_SPEC> {
        RSTN_U1_CAN_CTRL_RSTN_TIMER_W::new(self, 20)
    }
    #[doc = "Bit 21 - 1: Assert reset, 0: De-assert reset"]
    #[inline(always)]
    #[must_use]
    pub fn rstn_u0_si5_timer_rstn_apb(
        &mut self,
    ) -> RSTN_U0_SI5_TIMER_RSTN_APB_W<SOFT_RST_ADDR_SEL_3_SPEC> {
        RSTN_U0_SI5_TIMER_RSTN_APB_W::new(self, 21)
    }
    #[doc = "Bit 22 - 1: Assert reset, 0: De-assert reset"]
    #[inline(always)]
    #[must_use]
    pub fn rstn_u0_si5_timer_rstn_timer0(
        &mut self,
    ) -> RSTN_U0_SI5_TIMER_RSTN_TIMER0_W<SOFT_RST_ADDR_SEL_3_SPEC> {
        RSTN_U0_SI5_TIMER_RSTN_TIMER0_W::new(self, 22)
    }
    #[doc = "Bit 23 - 1: Assert reset, 0: De-assert reset"]
    #[inline(always)]
    #[must_use]
    pub fn rstn_u0_si5_timer_rstn_time10(
        &mut self,
    ) -> RSTN_U0_SI5_TIMER_RSTN_TIME10_W<SOFT_RST_ADDR_SEL_3_SPEC> {
        RSTN_U0_SI5_TIMER_RSTN_TIME10_W::new(self, 23)
    }
    #[doc = "Bit 24 - 1: Assert reset, 0: De-assert reset"]
    #[inline(always)]
    #[must_use]
    pub fn rstn_u0_si5_timer_rstn_timer2(
        &mut self,
    ) -> RSTN_U0_SI5_TIMER_RSTN_TIMER2_W<SOFT_RST_ADDR_SEL_3_SPEC> {
        RSTN_U0_SI5_TIMER_RSTN_TIMER2_W::new(self, 24)
    }
    #[doc = "Bit 25 - 1: Assert reset, 0: De-assert reset"]
    #[inline(always)]
    #[must_use]
    pub fn rstn_u0_si5_timer_rstn_timer3(
        &mut self,
    ) -> RSTN_U0_SI5_TIMER_RSTN_TIMER3_W<SOFT_RST_ADDR_SEL_3_SPEC> {
        RSTN_U0_SI5_TIMER_RSTN_TIMER3_W::new(self, 25)
    }
    #[doc = "Bit 26 - 1: Assert reset, 0: De-assert reset"]
    #[inline(always)]
    #[must_use]
    pub fn rstn_u0_int_ctrl_rstn_apb(
        &mut self,
    ) -> RSTN_U0_INT_CTRL_RSTN_APB_W<SOFT_RST_ADDR_SEL_3_SPEC> {
        RSTN_U0_INT_CTRL_RSTN_APB_W::new(self, 26)
    }
    #[doc = "Bit 27 - 1: Assert reset, 0: De-assert reset"]
    #[inline(always)]
    #[must_use]
    pub fn rstn_u0_temp_sensor_rstn_apb(
        &mut self,
    ) -> RSTN_U0_TEMP_SENSOR_RSTN_APB_W<SOFT_RST_ADDR_SEL_3_SPEC> {
        RSTN_U0_TEMP_SENSOR_RSTN_APB_W::new(self, 27)
    }
    #[doc = "Bit 28 - 1: Assert reset, 0: De-assert reset"]
    #[inline(always)]
    #[must_use]
    pub fn rstn_u0_temp_sensor_rstn_temp(
        &mut self,
    ) -> RSTN_U0_TEMP_SENSOR_RSTN_TEMP_W<SOFT_RST_ADDR_SEL_3_SPEC> {
        RSTN_U0_TEMP_SENSOR_RSTN_TEMP_W::new(self, 28)
    }
    #[doc = "Bit 29 - 1: Assert reset, 0: De-assert reset"]
    #[inline(always)]
    #[must_use]
    pub fn rstn_u0_jtag_certification_rst_n(
        &mut self,
    ) -> RSTN_U0_JTAG_CERTIFICATION_RST_N_W<SOFT_RST_ADDR_SEL_3_SPEC> {
        RSTN_U0_JTAG_CERTIFICATION_RST_N_W::new(self, 29)
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
