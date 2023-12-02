#[doc = "Register `sys_sysconsaif_syscfg52` reader"]
pub type R = crate::R<SYS_SYSCONSAIF_SYSCFG52_SPEC>;
#[doc = "Register `sys_sysconsaif_syscfg52` writer"]
pub type W = crate::W<SYS_SYSCONSAIF_SYSCFG52_SPEC>;
#[doc = "Field `u0_pll_wrap_pll2_prediv` reader - u0_pll_wrap_pll2_prediv"]
pub type U0_PLL_WRAP_PLL2_PREDIV_R = crate::FieldReader;
#[doc = "Field `u0_pll_wrap_pll2_prediv` writer - u0_pll_wrap_pll2_prediv"]
pub type U0_PLL_WRAP_PLL2_PREDIV_W<'a, REG> = crate::FieldWriter<'a, REG, 6>;
#[doc = "Field `u0_pll_wrap_pll2_testen` reader - u0_pll_wrap_pll2_testen"]
pub type U0_PLL_WRAP_PLL2_TESTEN_R = crate::BitReader;
#[doc = "Field `u0_pll_wrap_pll2_testen` writer - u0_pll_wrap_pll2_testen"]
pub type U0_PLL_WRAP_PLL2_TESTEN_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `u0_pll_wrap_pll2_testsel` reader - u0_pll_wrap_pll2_testsel"]
pub type U0_PLL_WRAP_PLL2_TESTSEL_R = crate::FieldReader;
#[doc = "Field `u0_pll_wrap_pll2_testsel` writer - u0_pll_wrap_pll2_testsel"]
pub type U0_PLL_WRAP_PLL2_TESTSEL_W<'a, REG> = crate::FieldWriter<'a, REG, 2>;
#[doc = "Field `u0_pll_wrap_syscfg_test_pll_mode` reader - PLL test mode, only used for PLL BIST through jtag2apb"]
pub type U0_PLL_WRAP_SYSCFG_TEST_PLL_MODE_R = crate::BitReader;
#[doc = "Field `u0_pll_wrap_syscfg_test_pll_mode` writer - PLL test mode, only used for PLL BIST through jtag2apb"]
pub type U0_PLL_WRAP_SYSCFG_TEST_PLL_MODE_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `u0_saif_audio_sdin_mux_scfg_i2sdin_sel` reader - u0_saif_audio_sdin_mux_scfg_i2sdin_sel"]
pub type U0_SAIF_AUDIO_SDIN_MUX_SCFG_I2SDIN_SEL_R = crate::FieldReader;
#[doc = "Field `u0_saif_audio_sdin_mux_scfg_i2sdin_sel` writer - u0_saif_audio_sdin_mux_scfg_i2sdin_sel"]
pub type U0_SAIF_AUDIO_SDIN_MUX_SCFG_I2SDIN_SEL_W<'a, REG> = crate::FieldWriter<'a, REG, 8>;
#[doc = "Field `u0_sft7110_noc_bus_clock_gating_off` reader - u0_sft7110_noc_bus_clock_gating_off"]
pub type U0_SFT7110_NOC_BUS_CLOCK_GATING_OFF_R = crate::BitReader;
#[doc = "Field `u0_sft7110_noc_bus_clock_gating_off` writer - u0_sft7110_noc_bus_clock_gating_off"]
pub type U0_SFT7110_NOC_BUS_CLOCK_GATING_OFF_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `u0_sft7110_noc_bus_oic_evemon_0_start` reader - u0_sft7110_noc_bus_oic_evemon_0_start"]
pub type U0_SFT7110_NOC_BUS_OIC_EVEMON_0_START_R = crate::BitReader;
#[doc = "Field `u0_sft7110_noc_bus_oic_evemon_0_start` writer - u0_sft7110_noc_bus_oic_evemon_0_start"]
pub type U0_SFT7110_NOC_BUS_OIC_EVEMON_0_START_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `u0_sft7110_noc_bus_oic_evemon_0_trigger` reader - u0_sft7110_noc_bus_oic_evemon_0_trigger"]
pub type U0_SFT7110_NOC_BUS_OIC_EVEMON_0_TRIGGER_R = crate::BitReader;
#[doc = "Field `u0_sft7110_noc_bus_oic_evemon_1_start` reader - u0_sft7110_noc_bus_oic_evemon_1_start"]
pub type U0_SFT7110_NOC_BUS_OIC_EVEMON_1_START_R = crate::BitReader;
#[doc = "Field `u0_sft7110_noc_bus_oic_evemon_1_start` writer - u0_sft7110_noc_bus_oic_evemon_1_start"]
pub type U0_SFT7110_NOC_BUS_OIC_EVEMON_1_START_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `u0_sft7110_noc_bus_oic_evemon_1_trigger` reader - u0_sft7110_noc_bus_oic_evemon_1_trigger"]
pub type U0_SFT7110_NOC_BUS_OIC_EVEMON_1_TRIGGER_R = crate::BitReader;
#[doc = "Field `u0_sft7110_noc_bus_oic_evemon_2_start` reader - u0_sft7110_noc_bus_oic_evemon_2_start"]
pub type U0_SFT7110_NOC_BUS_OIC_EVEMON_2_START_R = crate::BitReader;
#[doc = "Field `u0_sft7110_noc_bus_oic_evemon_2_start` writer - u0_sft7110_noc_bus_oic_evemon_2_start"]
pub type U0_SFT7110_NOC_BUS_OIC_EVEMON_2_START_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `u0_sft7110_noc_bus_oic_evemon_2_trigger` reader - u0_sft7110_noc_bus_oic_evemon_2_trigger"]
pub type U0_SFT7110_NOC_BUS_OIC_EVEMON_2_TRIGGER_R = crate::BitReader;
#[doc = "Field `u0_sft7110_noc_bus_oic_evemon_3_start` reader - u0_sft7110_noc_bus_oic_evemon_3_start"]
pub type U0_SFT7110_NOC_BUS_OIC_EVEMON_3_START_R = crate::BitReader;
#[doc = "Field `u0_sft7110_noc_bus_oic_evemon_3_start` writer - u0_sft7110_noc_bus_oic_evemon_3_start"]
pub type U0_SFT7110_NOC_BUS_OIC_EVEMON_3_START_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `u0_sft7110_noc_bus_oic_evemon_3_trigger` reader - u0_sft7110_noc_bus_oic_evemon_3_trigger"]
pub type U0_SFT7110_NOC_BUS_OIC_EVEMON_3_TRIGGER_R = crate::BitReader;
#[doc = "Field `u0_sft7110_noc_bus_oic_evemon_4_start` reader - u0_sft7110_noc_bus_oic_evemon_4_start"]
pub type U0_SFT7110_NOC_BUS_OIC_EVEMON_4_START_R = crate::BitReader;
#[doc = "Field `u0_sft7110_noc_bus_oic_evemon_4_start` writer - u0_sft7110_noc_bus_oic_evemon_4_start"]
pub type U0_SFT7110_NOC_BUS_OIC_EVEMON_4_START_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `u0_sft7110_noc_bus_oic_evemon_4_trigger` reader - u0_sft7110_noc_bus_oic_evemon_4_trigger"]
pub type U0_SFT7110_NOC_BUS_OIC_EVEMON_4_TRIGGER_R = crate::BitReader;
#[doc = "Field `u0_sft7110_noc_bus_oic_evemon_5_start` reader - u0_sft7110_noc_bus_oic_evemon_5_start"]
pub type U0_SFT7110_NOC_BUS_OIC_EVEMON_5_START_R = crate::BitReader;
#[doc = "Field `u0_sft7110_noc_bus_oic_evemon_5_start` writer - u0_sft7110_noc_bus_oic_evemon_5_start"]
pub type U0_SFT7110_NOC_BUS_OIC_EVEMON_5_START_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `u0_sft7110_noc_bus_oic_evemon_5_trigger` reader - u0_sft7110_noc_bus_oic_evemon_5_trigger"]
pub type U0_SFT7110_NOC_BUS_OIC_EVEMON_5_TRIGGER_R = crate::BitReader;
#[doc = "Field `u0_sft7110_noc_bus_oic_evemon_6_start` reader - u0_sft7110_noc_bus_oic_evemon_6_start"]
pub type U0_SFT7110_NOC_BUS_OIC_EVEMON_6_START_R = crate::BitReader;
#[doc = "Field `u0_sft7110_noc_bus_oic_evemon_6_start` writer - u0_sft7110_noc_bus_oic_evemon_6_start"]
pub type U0_SFT7110_NOC_BUS_OIC_EVEMON_6_START_W<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bits 0:5 - u0_pll_wrap_pll2_prediv"]
    #[inline(always)]
    pub fn u0_pll_wrap_pll2_prediv(&self) -> U0_PLL_WRAP_PLL2_PREDIV_R {
        U0_PLL_WRAP_PLL2_PREDIV_R::new((self.bits & 0x3f) as u8)
    }
    #[doc = "Bit 6 - u0_pll_wrap_pll2_testen"]
    #[inline(always)]
    pub fn u0_pll_wrap_pll2_testen(&self) -> U0_PLL_WRAP_PLL2_TESTEN_R {
        U0_PLL_WRAP_PLL2_TESTEN_R::new(((self.bits >> 6) & 1) != 0)
    }
    #[doc = "Bits 7:8 - u0_pll_wrap_pll2_testsel"]
    #[inline(always)]
    pub fn u0_pll_wrap_pll2_testsel(&self) -> U0_PLL_WRAP_PLL2_TESTSEL_R {
        U0_PLL_WRAP_PLL2_TESTSEL_R::new(((self.bits >> 7) & 3) as u8)
    }
    #[doc = "Bit 9 - PLL test mode, only used for PLL BIST through jtag2apb"]
    #[inline(always)]
    pub fn u0_pll_wrap_syscfg_test_pll_mode(&self) -> U0_PLL_WRAP_SYSCFG_TEST_PLL_MODE_R {
        U0_PLL_WRAP_SYSCFG_TEST_PLL_MODE_R::new(((self.bits >> 9) & 1) != 0)
    }
    #[doc = "Bits 10:17 - u0_saif_audio_sdin_mux_scfg_i2sdin_sel"]
    #[inline(always)]
    pub fn u0_saif_audio_sdin_mux_scfg_i2sdin_sel(
        &self,
    ) -> U0_SAIF_AUDIO_SDIN_MUX_SCFG_I2SDIN_SEL_R {
        U0_SAIF_AUDIO_SDIN_MUX_SCFG_I2SDIN_SEL_R::new(((self.bits >> 10) & 0xff) as u8)
    }
    #[doc = "Bit 18 - u0_sft7110_noc_bus_clock_gating_off"]
    #[inline(always)]
    pub fn u0_sft7110_noc_bus_clock_gating_off(&self) -> U0_SFT7110_NOC_BUS_CLOCK_GATING_OFF_R {
        U0_SFT7110_NOC_BUS_CLOCK_GATING_OFF_R::new(((self.bits >> 18) & 1) != 0)
    }
    #[doc = "Bit 19 - u0_sft7110_noc_bus_oic_evemon_0_start"]
    #[inline(always)]
    pub fn u0_sft7110_noc_bus_oic_evemon_0_start(&self) -> U0_SFT7110_NOC_BUS_OIC_EVEMON_0_START_R {
        U0_SFT7110_NOC_BUS_OIC_EVEMON_0_START_R::new(((self.bits >> 19) & 1) != 0)
    }
    #[doc = "Bit 20 - u0_sft7110_noc_bus_oic_evemon_0_trigger"]
    #[inline(always)]
    pub fn u0_sft7110_noc_bus_oic_evemon_0_trigger(
        &self,
    ) -> U0_SFT7110_NOC_BUS_OIC_EVEMON_0_TRIGGER_R {
        U0_SFT7110_NOC_BUS_OIC_EVEMON_0_TRIGGER_R::new(((self.bits >> 20) & 1) != 0)
    }
    #[doc = "Bit 21 - u0_sft7110_noc_bus_oic_evemon_1_start"]
    #[inline(always)]
    pub fn u0_sft7110_noc_bus_oic_evemon_1_start(&self) -> U0_SFT7110_NOC_BUS_OIC_EVEMON_1_START_R {
        U0_SFT7110_NOC_BUS_OIC_EVEMON_1_START_R::new(((self.bits >> 21) & 1) != 0)
    }
    #[doc = "Bit 22 - u0_sft7110_noc_bus_oic_evemon_1_trigger"]
    #[inline(always)]
    pub fn u0_sft7110_noc_bus_oic_evemon_1_trigger(
        &self,
    ) -> U0_SFT7110_NOC_BUS_OIC_EVEMON_1_TRIGGER_R {
        U0_SFT7110_NOC_BUS_OIC_EVEMON_1_TRIGGER_R::new(((self.bits >> 22) & 1) != 0)
    }
    #[doc = "Bit 23 - u0_sft7110_noc_bus_oic_evemon_2_start"]
    #[inline(always)]
    pub fn u0_sft7110_noc_bus_oic_evemon_2_start(&self) -> U0_SFT7110_NOC_BUS_OIC_EVEMON_2_START_R {
        U0_SFT7110_NOC_BUS_OIC_EVEMON_2_START_R::new(((self.bits >> 23) & 1) != 0)
    }
    #[doc = "Bit 24 - u0_sft7110_noc_bus_oic_evemon_2_trigger"]
    #[inline(always)]
    pub fn u0_sft7110_noc_bus_oic_evemon_2_trigger(
        &self,
    ) -> U0_SFT7110_NOC_BUS_OIC_EVEMON_2_TRIGGER_R {
        U0_SFT7110_NOC_BUS_OIC_EVEMON_2_TRIGGER_R::new(((self.bits >> 24) & 1) != 0)
    }
    #[doc = "Bit 25 - u0_sft7110_noc_bus_oic_evemon_3_start"]
    #[inline(always)]
    pub fn u0_sft7110_noc_bus_oic_evemon_3_start(&self) -> U0_SFT7110_NOC_BUS_OIC_EVEMON_3_START_R {
        U0_SFT7110_NOC_BUS_OIC_EVEMON_3_START_R::new(((self.bits >> 25) & 1) != 0)
    }
    #[doc = "Bit 26 - u0_sft7110_noc_bus_oic_evemon_3_trigger"]
    #[inline(always)]
    pub fn u0_sft7110_noc_bus_oic_evemon_3_trigger(
        &self,
    ) -> U0_SFT7110_NOC_BUS_OIC_EVEMON_3_TRIGGER_R {
        U0_SFT7110_NOC_BUS_OIC_EVEMON_3_TRIGGER_R::new(((self.bits >> 26) & 1) != 0)
    }
    #[doc = "Bit 27 - u0_sft7110_noc_bus_oic_evemon_4_start"]
    #[inline(always)]
    pub fn u0_sft7110_noc_bus_oic_evemon_4_start(&self) -> U0_SFT7110_NOC_BUS_OIC_EVEMON_4_START_R {
        U0_SFT7110_NOC_BUS_OIC_EVEMON_4_START_R::new(((self.bits >> 27) & 1) != 0)
    }
    #[doc = "Bit 28 - u0_sft7110_noc_bus_oic_evemon_4_trigger"]
    #[inline(always)]
    pub fn u0_sft7110_noc_bus_oic_evemon_4_trigger(
        &self,
    ) -> U0_SFT7110_NOC_BUS_OIC_EVEMON_4_TRIGGER_R {
        U0_SFT7110_NOC_BUS_OIC_EVEMON_4_TRIGGER_R::new(((self.bits >> 28) & 1) != 0)
    }
    #[doc = "Bit 29 - u0_sft7110_noc_bus_oic_evemon_5_start"]
    #[inline(always)]
    pub fn u0_sft7110_noc_bus_oic_evemon_5_start(&self) -> U0_SFT7110_NOC_BUS_OIC_EVEMON_5_START_R {
        U0_SFT7110_NOC_BUS_OIC_EVEMON_5_START_R::new(((self.bits >> 29) & 1) != 0)
    }
    #[doc = "Bit 30 - u0_sft7110_noc_bus_oic_evemon_5_trigger"]
    #[inline(always)]
    pub fn u0_sft7110_noc_bus_oic_evemon_5_trigger(
        &self,
    ) -> U0_SFT7110_NOC_BUS_OIC_EVEMON_5_TRIGGER_R {
        U0_SFT7110_NOC_BUS_OIC_EVEMON_5_TRIGGER_R::new(((self.bits >> 30) & 1) != 0)
    }
    #[doc = "Bit 31 - u0_sft7110_noc_bus_oic_evemon_6_start"]
    #[inline(always)]
    pub fn u0_sft7110_noc_bus_oic_evemon_6_start(&self) -> U0_SFT7110_NOC_BUS_OIC_EVEMON_6_START_R {
        U0_SFT7110_NOC_BUS_OIC_EVEMON_6_START_R::new(((self.bits >> 31) & 1) != 0)
    }
}
impl W {
    #[doc = "Bits 0:5 - u0_pll_wrap_pll2_prediv"]
    #[inline(always)]
    #[must_use]
    pub fn u0_pll_wrap_pll2_prediv(
        &mut self,
    ) -> U0_PLL_WRAP_PLL2_PREDIV_W<SYS_SYSCONSAIF_SYSCFG52_SPEC> {
        U0_PLL_WRAP_PLL2_PREDIV_W::new(self, 0)
    }
    #[doc = "Bit 6 - u0_pll_wrap_pll2_testen"]
    #[inline(always)]
    #[must_use]
    pub fn u0_pll_wrap_pll2_testen(
        &mut self,
    ) -> U0_PLL_WRAP_PLL2_TESTEN_W<SYS_SYSCONSAIF_SYSCFG52_SPEC> {
        U0_PLL_WRAP_PLL2_TESTEN_W::new(self, 6)
    }
    #[doc = "Bits 7:8 - u0_pll_wrap_pll2_testsel"]
    #[inline(always)]
    #[must_use]
    pub fn u0_pll_wrap_pll2_testsel(
        &mut self,
    ) -> U0_PLL_WRAP_PLL2_TESTSEL_W<SYS_SYSCONSAIF_SYSCFG52_SPEC> {
        U0_PLL_WRAP_PLL2_TESTSEL_W::new(self, 7)
    }
    #[doc = "Bit 9 - PLL test mode, only used for PLL BIST through jtag2apb"]
    #[inline(always)]
    #[must_use]
    pub fn u0_pll_wrap_syscfg_test_pll_mode(
        &mut self,
    ) -> U0_PLL_WRAP_SYSCFG_TEST_PLL_MODE_W<SYS_SYSCONSAIF_SYSCFG52_SPEC> {
        U0_PLL_WRAP_SYSCFG_TEST_PLL_MODE_W::new(self, 9)
    }
    #[doc = "Bits 10:17 - u0_saif_audio_sdin_mux_scfg_i2sdin_sel"]
    #[inline(always)]
    #[must_use]
    pub fn u0_saif_audio_sdin_mux_scfg_i2sdin_sel(
        &mut self,
    ) -> U0_SAIF_AUDIO_SDIN_MUX_SCFG_I2SDIN_SEL_W<SYS_SYSCONSAIF_SYSCFG52_SPEC> {
        U0_SAIF_AUDIO_SDIN_MUX_SCFG_I2SDIN_SEL_W::new(self, 10)
    }
    #[doc = "Bit 18 - u0_sft7110_noc_bus_clock_gating_off"]
    #[inline(always)]
    #[must_use]
    pub fn u0_sft7110_noc_bus_clock_gating_off(
        &mut self,
    ) -> U0_SFT7110_NOC_BUS_CLOCK_GATING_OFF_W<SYS_SYSCONSAIF_SYSCFG52_SPEC> {
        U0_SFT7110_NOC_BUS_CLOCK_GATING_OFF_W::new(self, 18)
    }
    #[doc = "Bit 19 - u0_sft7110_noc_bus_oic_evemon_0_start"]
    #[inline(always)]
    #[must_use]
    pub fn u0_sft7110_noc_bus_oic_evemon_0_start(
        &mut self,
    ) -> U0_SFT7110_NOC_BUS_OIC_EVEMON_0_START_W<SYS_SYSCONSAIF_SYSCFG52_SPEC> {
        U0_SFT7110_NOC_BUS_OIC_EVEMON_0_START_W::new(self, 19)
    }
    #[doc = "Bit 21 - u0_sft7110_noc_bus_oic_evemon_1_start"]
    #[inline(always)]
    #[must_use]
    pub fn u0_sft7110_noc_bus_oic_evemon_1_start(
        &mut self,
    ) -> U0_SFT7110_NOC_BUS_OIC_EVEMON_1_START_W<SYS_SYSCONSAIF_SYSCFG52_SPEC> {
        U0_SFT7110_NOC_BUS_OIC_EVEMON_1_START_W::new(self, 21)
    }
    #[doc = "Bit 23 - u0_sft7110_noc_bus_oic_evemon_2_start"]
    #[inline(always)]
    #[must_use]
    pub fn u0_sft7110_noc_bus_oic_evemon_2_start(
        &mut self,
    ) -> U0_SFT7110_NOC_BUS_OIC_EVEMON_2_START_W<SYS_SYSCONSAIF_SYSCFG52_SPEC> {
        U0_SFT7110_NOC_BUS_OIC_EVEMON_2_START_W::new(self, 23)
    }
    #[doc = "Bit 25 - u0_sft7110_noc_bus_oic_evemon_3_start"]
    #[inline(always)]
    #[must_use]
    pub fn u0_sft7110_noc_bus_oic_evemon_3_start(
        &mut self,
    ) -> U0_SFT7110_NOC_BUS_OIC_EVEMON_3_START_W<SYS_SYSCONSAIF_SYSCFG52_SPEC> {
        U0_SFT7110_NOC_BUS_OIC_EVEMON_3_START_W::new(self, 25)
    }
    #[doc = "Bit 27 - u0_sft7110_noc_bus_oic_evemon_4_start"]
    #[inline(always)]
    #[must_use]
    pub fn u0_sft7110_noc_bus_oic_evemon_4_start(
        &mut self,
    ) -> U0_SFT7110_NOC_BUS_OIC_EVEMON_4_START_W<SYS_SYSCONSAIF_SYSCFG52_SPEC> {
        U0_SFT7110_NOC_BUS_OIC_EVEMON_4_START_W::new(self, 27)
    }
    #[doc = "Bit 29 - u0_sft7110_noc_bus_oic_evemon_5_start"]
    #[inline(always)]
    #[must_use]
    pub fn u0_sft7110_noc_bus_oic_evemon_5_start(
        &mut self,
    ) -> U0_SFT7110_NOC_BUS_OIC_EVEMON_5_START_W<SYS_SYSCONSAIF_SYSCFG52_SPEC> {
        U0_SFT7110_NOC_BUS_OIC_EVEMON_5_START_W::new(self, 29)
    }
    #[doc = "Bit 31 - u0_sft7110_noc_bus_oic_evemon_6_start"]
    #[inline(always)]
    #[must_use]
    pub fn u0_sft7110_noc_bus_oic_evemon_6_start(
        &mut self,
    ) -> U0_SFT7110_NOC_BUS_OIC_EVEMON_6_START_W<SYS_SYSCONSAIF_SYSCFG52_SPEC> {
        U0_SFT7110_NOC_BUS_OIC_EVEMON_6_START_W::new(self, 31)
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
#[doc = "SYS SYSCONSAIF SYSCFG 52\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`sys_sysconsaif_syscfg52::R`](R).  You can [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`sys_sysconsaif_syscfg52::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct SYS_SYSCONSAIF_SYSCFG52_SPEC;
impl crate::RegisterSpec for SYS_SYSCONSAIF_SYSCFG52_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`sys_sysconsaif_syscfg52::R`](R) reader structure"]
impl crate::Readable for SYS_SYSCONSAIF_SYSCFG52_SPEC {}
#[doc = "`write(|w| ..)` method takes [`sys_sysconsaif_syscfg52::W`](W) writer structure"]
impl crate::Writable for SYS_SYSCONSAIF_SYSCFG52_SPEC {
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
