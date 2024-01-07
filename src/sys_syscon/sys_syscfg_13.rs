#[doc = "Register `sys_syscfg_13` reader"]
pub type R = crate::R<SYS_SYSCFG_13_SPEC>;
#[doc = "Register `sys_syscfg_13` writer"]
pub type W = crate::W<SYS_SYSCFG_13_SPEC>;
#[doc = "Field `pll2_prediv` reader - pll2_prediv"]
pub type PLL2_PREDIV_R = crate::FieldReader;
#[doc = "Field `pll2_prediv` writer - pll2_prediv"]
pub type PLL2_PREDIV_W<'a, REG> = crate::FieldWriter<'a, REG, 6>;
#[doc = "Field `pll2_testen` reader - pll2_testen"]
pub type PLL2_TESTEN_R = crate::BitReader;
#[doc = "Field `pll2_testen` writer - pll2_testen"]
pub type PLL2_TESTEN_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `pll2_testsel` reader - pll2_testsel"]
pub type PLL2_TESTSEL_R = crate::FieldReader;
#[doc = "Field `pll2_testsel` writer - pll2_testsel"]
pub type PLL2_TESTSEL_W<'a, REG> = crate::FieldWriter<'a, REG, 2>;
#[doc = "Field `pll_test_mode` reader - PLL test mode, only used for PLL BIST through jtag2apb"]
pub type PLL_TEST_MODE_R = crate::BitReader;
#[doc = "Field `pll_test_mode` writer - PLL test mode, only used for PLL BIST through jtag2apb"]
pub type PLL_TEST_MODE_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `audio_i2sdin_sel` reader - audio_i2sdin_sel"]
pub type AUDIO_I2SDIN_SEL_R = crate::FieldReader;
#[doc = "Field `audio_i2sdin_sel` writer - audio_i2sdin_sel"]
pub type AUDIO_I2SDIN_SEL_W<'a, REG> = crate::FieldWriter<'a, REG, 8>;
#[doc = "Field `noc_bus_clock_gating_off` reader - noc_bus_clock_gating_off"]
pub type NOC_BUS_CLOCK_GATING_OFF_R = crate::BitReader;
#[doc = "Field `noc_bus_clock_gating_off` writer - noc_bus_clock_gating_off"]
pub type NOC_BUS_CLOCK_GATING_OFF_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `noc_bus_oic_evemon_start_0` reader - noc_bus_oic_evemon_start_0"]
pub type NOC_BUS_OIC_EVEMON_START_0_R = crate::BitReader;
#[doc = "Field `noc_bus_oic_evemon_start_0` writer - noc_bus_oic_evemon_start_0"]
pub type NOC_BUS_OIC_EVEMON_START_0_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `noc_bus_oic_evemon_trigger_0` reader - noc_bus_oic_evemon_trigger_0"]
pub type NOC_BUS_OIC_EVEMON_TRIGGER_0_R = crate::BitReader;
#[doc = "Field `noc_bus_oic_evemon_start_1` reader - noc_bus_oic_evemon_start_1"]
pub type NOC_BUS_OIC_EVEMON_START_1_R = crate::BitReader;
#[doc = "Field `noc_bus_oic_evemon_start_1` writer - noc_bus_oic_evemon_start_1"]
pub type NOC_BUS_OIC_EVEMON_START_1_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `noc_bus_oic_evemon_trigger_1` reader - noc_bus_oic_evemon_trigger_1"]
pub type NOC_BUS_OIC_EVEMON_TRIGGER_1_R = crate::BitReader;
#[doc = "Field `noc_bus_oic_evemon_start_2` reader - noc_bus_oic_evemon_start_2"]
pub type NOC_BUS_OIC_EVEMON_START_2_R = crate::BitReader;
#[doc = "Field `noc_bus_oic_evemon_start_2` writer - noc_bus_oic_evemon_start_2"]
pub type NOC_BUS_OIC_EVEMON_START_2_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `noc_bus_oic_evemon_trigger_2` reader - noc_bus_oic_evemon_trigger_2"]
pub type NOC_BUS_OIC_EVEMON_TRIGGER_2_R = crate::BitReader;
#[doc = "Field `noc_bus_oic_evemon_start_3` reader - noc_bus_oic_evemon_start_3"]
pub type NOC_BUS_OIC_EVEMON_START_3_R = crate::BitReader;
#[doc = "Field `noc_bus_oic_evemon_start_3` writer - noc_bus_oic_evemon_start_3"]
pub type NOC_BUS_OIC_EVEMON_START_3_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `noc_bus_oic_evemon_trigger_3` reader - noc_bus_oic_evemon_trigger_3"]
pub type NOC_BUS_OIC_EVEMON_TRIGGER_3_R = crate::BitReader;
#[doc = "Field `noc_bus_oic_evemon_start_4` reader - noc_bus_oic_evemon_start_4"]
pub type NOC_BUS_OIC_EVEMON_START_4_R = crate::BitReader;
#[doc = "Field `noc_bus_oic_evemon_start_4` writer - noc_bus_oic_evemon_start_4"]
pub type NOC_BUS_OIC_EVEMON_START_4_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `noc_bus_oic_evemon_trigger_4` reader - noc_bus_oic_evemon_trigger_4"]
pub type NOC_BUS_OIC_EVEMON_TRIGGER_4_R = crate::BitReader;
#[doc = "Field `noc_bus_oic_evemon_start_5` reader - noc_bus_oic_evemon_start_5"]
pub type NOC_BUS_OIC_EVEMON_START_5_R = crate::BitReader;
#[doc = "Field `noc_bus_oic_evemon_start_5` writer - noc_bus_oic_evemon_start_5"]
pub type NOC_BUS_OIC_EVEMON_START_5_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `noc_bus_oic_evemon_trigger_5` reader - noc_bus_oic_evemon_trigger_5"]
pub type NOC_BUS_OIC_EVEMON_TRIGGER_5_R = crate::BitReader;
#[doc = "Field `noc_bus_oic_evemon_start_6` reader - noc_bus_oic_evemon_start_6"]
pub type NOC_BUS_OIC_EVEMON_START_6_R = crate::BitReader;
#[doc = "Field `noc_bus_oic_evemon_start_6` writer - noc_bus_oic_evemon_start_6"]
pub type NOC_BUS_OIC_EVEMON_START_6_W<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bits 0:5 - pll2_prediv"]
    #[inline(always)]
    pub fn pll2_prediv(&self) -> PLL2_PREDIV_R {
        PLL2_PREDIV_R::new((self.bits & 0x3f) as u8)
    }
    #[doc = "Bit 6 - pll2_testen"]
    #[inline(always)]
    pub fn pll2_testen(&self) -> PLL2_TESTEN_R {
        PLL2_TESTEN_R::new(((self.bits >> 6) & 1) != 0)
    }
    #[doc = "Bits 7:8 - pll2_testsel"]
    #[inline(always)]
    pub fn pll2_testsel(&self) -> PLL2_TESTSEL_R {
        PLL2_TESTSEL_R::new(((self.bits >> 7) & 3) as u8)
    }
    #[doc = "Bit 9 - PLL test mode, only used for PLL BIST through jtag2apb"]
    #[inline(always)]
    pub fn pll_test_mode(&self) -> PLL_TEST_MODE_R {
        PLL_TEST_MODE_R::new(((self.bits >> 9) & 1) != 0)
    }
    #[doc = "Bits 10:17 - audio_i2sdin_sel"]
    #[inline(always)]
    pub fn audio_i2sdin_sel(&self) -> AUDIO_I2SDIN_SEL_R {
        AUDIO_I2SDIN_SEL_R::new(((self.bits >> 10) & 0xff) as u8)
    }
    #[doc = "Bit 18 - noc_bus_clock_gating_off"]
    #[inline(always)]
    pub fn noc_bus_clock_gating_off(&self) -> NOC_BUS_CLOCK_GATING_OFF_R {
        NOC_BUS_CLOCK_GATING_OFF_R::new(((self.bits >> 18) & 1) != 0)
    }
    #[doc = "Bit 19 - noc_bus_oic_evemon_start_0"]
    #[inline(always)]
    pub fn noc_bus_oic_evemon_start_0(&self) -> NOC_BUS_OIC_EVEMON_START_0_R {
        NOC_BUS_OIC_EVEMON_START_0_R::new(((self.bits >> 19) & 1) != 0)
    }
    #[doc = "Bit 20 - noc_bus_oic_evemon_trigger_0"]
    #[inline(always)]
    pub fn noc_bus_oic_evemon_trigger_0(&self) -> NOC_BUS_OIC_EVEMON_TRIGGER_0_R {
        NOC_BUS_OIC_EVEMON_TRIGGER_0_R::new(((self.bits >> 20) & 1) != 0)
    }
    #[doc = "Bit 21 - noc_bus_oic_evemon_start_1"]
    #[inline(always)]
    pub fn noc_bus_oic_evemon_start_1(&self) -> NOC_BUS_OIC_EVEMON_START_1_R {
        NOC_BUS_OIC_EVEMON_START_1_R::new(((self.bits >> 21) & 1) != 0)
    }
    #[doc = "Bit 22 - noc_bus_oic_evemon_trigger_1"]
    #[inline(always)]
    pub fn noc_bus_oic_evemon_trigger_1(&self) -> NOC_BUS_OIC_EVEMON_TRIGGER_1_R {
        NOC_BUS_OIC_EVEMON_TRIGGER_1_R::new(((self.bits >> 22) & 1) != 0)
    }
    #[doc = "Bit 23 - noc_bus_oic_evemon_start_2"]
    #[inline(always)]
    pub fn noc_bus_oic_evemon_start_2(&self) -> NOC_BUS_OIC_EVEMON_START_2_R {
        NOC_BUS_OIC_EVEMON_START_2_R::new(((self.bits >> 23) & 1) != 0)
    }
    #[doc = "Bit 24 - noc_bus_oic_evemon_trigger_2"]
    #[inline(always)]
    pub fn noc_bus_oic_evemon_trigger_2(&self) -> NOC_BUS_OIC_EVEMON_TRIGGER_2_R {
        NOC_BUS_OIC_EVEMON_TRIGGER_2_R::new(((self.bits >> 24) & 1) != 0)
    }
    #[doc = "Bit 25 - noc_bus_oic_evemon_start_3"]
    #[inline(always)]
    pub fn noc_bus_oic_evemon_start_3(&self) -> NOC_BUS_OIC_EVEMON_START_3_R {
        NOC_BUS_OIC_EVEMON_START_3_R::new(((self.bits >> 25) & 1) != 0)
    }
    #[doc = "Bit 26 - noc_bus_oic_evemon_trigger_3"]
    #[inline(always)]
    pub fn noc_bus_oic_evemon_trigger_3(&self) -> NOC_BUS_OIC_EVEMON_TRIGGER_3_R {
        NOC_BUS_OIC_EVEMON_TRIGGER_3_R::new(((self.bits >> 26) & 1) != 0)
    }
    #[doc = "Bit 27 - noc_bus_oic_evemon_start_4"]
    #[inline(always)]
    pub fn noc_bus_oic_evemon_start_4(&self) -> NOC_BUS_OIC_EVEMON_START_4_R {
        NOC_BUS_OIC_EVEMON_START_4_R::new(((self.bits >> 27) & 1) != 0)
    }
    #[doc = "Bit 28 - noc_bus_oic_evemon_trigger_4"]
    #[inline(always)]
    pub fn noc_bus_oic_evemon_trigger_4(&self) -> NOC_BUS_OIC_EVEMON_TRIGGER_4_R {
        NOC_BUS_OIC_EVEMON_TRIGGER_4_R::new(((self.bits >> 28) & 1) != 0)
    }
    #[doc = "Bit 29 - noc_bus_oic_evemon_start_5"]
    #[inline(always)]
    pub fn noc_bus_oic_evemon_start_5(&self) -> NOC_BUS_OIC_EVEMON_START_5_R {
        NOC_BUS_OIC_EVEMON_START_5_R::new(((self.bits >> 29) & 1) != 0)
    }
    #[doc = "Bit 30 - noc_bus_oic_evemon_trigger_5"]
    #[inline(always)]
    pub fn noc_bus_oic_evemon_trigger_5(&self) -> NOC_BUS_OIC_EVEMON_TRIGGER_5_R {
        NOC_BUS_OIC_EVEMON_TRIGGER_5_R::new(((self.bits >> 30) & 1) != 0)
    }
    #[doc = "Bit 31 - noc_bus_oic_evemon_start_6"]
    #[inline(always)]
    pub fn noc_bus_oic_evemon_start_6(&self) -> NOC_BUS_OIC_EVEMON_START_6_R {
        NOC_BUS_OIC_EVEMON_START_6_R::new(((self.bits >> 31) & 1) != 0)
    }
}
impl W {
    #[doc = "Bits 0:5 - pll2_prediv"]
    #[inline(always)]
    #[must_use]
    pub fn pll2_prediv(&mut self) -> PLL2_PREDIV_W<SYS_SYSCFG_13_SPEC> {
        PLL2_PREDIV_W::new(self, 0)
    }
    #[doc = "Bit 6 - pll2_testen"]
    #[inline(always)]
    #[must_use]
    pub fn pll2_testen(&mut self) -> PLL2_TESTEN_W<SYS_SYSCFG_13_SPEC> {
        PLL2_TESTEN_W::new(self, 6)
    }
    #[doc = "Bits 7:8 - pll2_testsel"]
    #[inline(always)]
    #[must_use]
    pub fn pll2_testsel(&mut self) -> PLL2_TESTSEL_W<SYS_SYSCFG_13_SPEC> {
        PLL2_TESTSEL_W::new(self, 7)
    }
    #[doc = "Bit 9 - PLL test mode, only used for PLL BIST through jtag2apb"]
    #[inline(always)]
    #[must_use]
    pub fn pll_test_mode(&mut self) -> PLL_TEST_MODE_W<SYS_SYSCFG_13_SPEC> {
        PLL_TEST_MODE_W::new(self, 9)
    }
    #[doc = "Bits 10:17 - audio_i2sdin_sel"]
    #[inline(always)]
    #[must_use]
    pub fn audio_i2sdin_sel(&mut self) -> AUDIO_I2SDIN_SEL_W<SYS_SYSCFG_13_SPEC> {
        AUDIO_I2SDIN_SEL_W::new(self, 10)
    }
    #[doc = "Bit 18 - noc_bus_clock_gating_off"]
    #[inline(always)]
    #[must_use]
    pub fn noc_bus_clock_gating_off(&mut self) -> NOC_BUS_CLOCK_GATING_OFF_W<SYS_SYSCFG_13_SPEC> {
        NOC_BUS_CLOCK_GATING_OFF_W::new(self, 18)
    }
    #[doc = "Bit 19 - noc_bus_oic_evemon_start_0"]
    #[inline(always)]
    #[must_use]
    pub fn noc_bus_oic_evemon_start_0(
        &mut self,
    ) -> NOC_BUS_OIC_EVEMON_START_0_W<SYS_SYSCFG_13_SPEC> {
        NOC_BUS_OIC_EVEMON_START_0_W::new(self, 19)
    }
    #[doc = "Bit 21 - noc_bus_oic_evemon_start_1"]
    #[inline(always)]
    #[must_use]
    pub fn noc_bus_oic_evemon_start_1(
        &mut self,
    ) -> NOC_BUS_OIC_EVEMON_START_1_W<SYS_SYSCFG_13_SPEC> {
        NOC_BUS_OIC_EVEMON_START_1_W::new(self, 21)
    }
    #[doc = "Bit 23 - noc_bus_oic_evemon_start_2"]
    #[inline(always)]
    #[must_use]
    pub fn noc_bus_oic_evemon_start_2(
        &mut self,
    ) -> NOC_BUS_OIC_EVEMON_START_2_W<SYS_SYSCFG_13_SPEC> {
        NOC_BUS_OIC_EVEMON_START_2_W::new(self, 23)
    }
    #[doc = "Bit 25 - noc_bus_oic_evemon_start_3"]
    #[inline(always)]
    #[must_use]
    pub fn noc_bus_oic_evemon_start_3(
        &mut self,
    ) -> NOC_BUS_OIC_EVEMON_START_3_W<SYS_SYSCFG_13_SPEC> {
        NOC_BUS_OIC_EVEMON_START_3_W::new(self, 25)
    }
    #[doc = "Bit 27 - noc_bus_oic_evemon_start_4"]
    #[inline(always)]
    #[must_use]
    pub fn noc_bus_oic_evemon_start_4(
        &mut self,
    ) -> NOC_BUS_OIC_EVEMON_START_4_W<SYS_SYSCFG_13_SPEC> {
        NOC_BUS_OIC_EVEMON_START_4_W::new(self, 27)
    }
    #[doc = "Bit 29 - noc_bus_oic_evemon_start_5"]
    #[inline(always)]
    #[must_use]
    pub fn noc_bus_oic_evemon_start_5(
        &mut self,
    ) -> NOC_BUS_OIC_EVEMON_START_5_W<SYS_SYSCFG_13_SPEC> {
        NOC_BUS_OIC_EVEMON_START_5_W::new(self, 29)
    }
    #[doc = "Bit 31 - noc_bus_oic_evemon_start_6"]
    #[inline(always)]
    #[must_use]
    pub fn noc_bus_oic_evemon_start_6(
        &mut self,
    ) -> NOC_BUS_OIC_EVEMON_START_6_W<SYS_SYSCFG_13_SPEC> {
        NOC_BUS_OIC_EVEMON_START_6_W::new(self, 31)
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
#[doc = "SYS SYSCONSAIF SYSCFG 52\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`sys_syscfg_13::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`sys_syscfg_13::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct SYS_SYSCFG_13_SPEC;
impl crate::RegisterSpec for SYS_SYSCFG_13_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`sys_syscfg_13::R`](R) reader structure"]
impl crate::Readable for SYS_SYSCFG_13_SPEC {}
#[doc = "`write(|w| ..)` method takes [`sys_syscfg_13::W`](W) writer structure"]
impl crate::Writable for SYS_SYSCFG_13_SPEC {
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets sys_syscfg_13 to value 0"]
impl crate::Resettable for SYS_SYSCFG_13_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
