#[doc = "Register `sys_sysconsaif_syscfg56` reader"]
pub type R = crate::R<SYS_SYSCONSAIF_SYSCFG56_SPEC>;
#[doc = "Register `sys_sysconsaif_syscfg56` writer"]
pub type W = crate::W<SYS_SYSCONSAIF_SYSCFG56_SPEC>;
#[doc = "Field `u0_sft7110_noc_bus_oic_evemon_6_trigger` reader - u0_sft7110_noc_bus_oic_evemon_6_trigger"]
pub type U0_SFT7110_NOC_BUS_OIC_EVEMON_6_TRIGGER_R = crate::BitReader;
#[doc = "Field `u0_sft7110_noc_bus_oic_ignore_modifiable_0` reader - u0_sft7110_noc_bus_oic_ignore_modifiable_0"]
pub type U0_SFT7110_NOC_BUS_OIC_IGNORE_MODIFIABLE_0_R = crate::BitReader;
#[doc = "Field `u0_sft7110_noc_bus_oic_ignore_modifiable_0` writer - u0_sft7110_noc_bus_oic_ignore_modifiable_0"]
pub type U0_SFT7110_NOC_BUS_OIC_IGNORE_MODIFIABLE_0_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `u0_sft7110_noc_bus_oic_ignore_modifiable_1` reader - u0_sft7110_noc_bus_oic_ignore_modifiable_1"]
pub type U0_SFT7110_NOC_BUS_OIC_IGNORE_MODIFIABLE_1_R = crate::BitReader;
#[doc = "Field `u0_sft7110_noc_bus_oic_ignore_modifiable_1` writer - u0_sft7110_noc_bus_oic_ignore_modifiable_1"]
pub type U0_SFT7110_NOC_BUS_OIC_IGNORE_MODIFIABLE_1_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `u0_sft7110_noc_bus_oic_ignore_modifiable_2` reader - u0_sft7110_noc_bus_oic_ignore_modifiable_2"]
pub type U0_SFT7110_NOC_BUS_OIC_IGNORE_MODIFIABLE_2_R = crate::BitReader;
#[doc = "Field `u0_sft7110_noc_bus_oic_ignore_modifiable_2` writer - u0_sft7110_noc_bus_oic_ignore_modifiable_2"]
pub type U0_SFT7110_NOC_BUS_OIC_IGNORE_MODIFIABLE_2_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `u0_sft7110_noc_bus_oic_ignore_modifiable_3` reader - u0_sft7110_noc_bus_oic_ignore_modifiable_3"]
pub type U0_SFT7110_NOC_BUS_OIC_IGNORE_MODIFIABLE_3_R = crate::BitReader;
#[doc = "Field `u0_sft7110_noc_bus_oic_ignore_modifiable_3` writer - u0_sft7110_noc_bus_oic_ignore_modifiable_3"]
pub type U0_SFT7110_NOC_BUS_OIC_IGNORE_MODIFIABLE_3_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `u0_sft7110_noc_bus_oic_ignore_modifiable_4` reader - u0_sft7110_noc_bus_oic_ignore_modifiable_4"]
pub type U0_SFT7110_NOC_BUS_OIC_IGNORE_MODIFIABLE_4_R = crate::BitReader;
#[doc = "Field `u0_sft7110_noc_bus_oic_ignore_modifiable_4` writer - u0_sft7110_noc_bus_oic_ignore_modifiable_4"]
pub type U0_SFT7110_NOC_BUS_OIC_IGNORE_MODIFIABLE_4_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `u0_sft7110_noc_bus_oic_evemon_7_start` reader - u0_sft7110_noc_bus_oic_evemon_7_start"]
pub type U0_SFT7110_NOC_BUS_OIC_EVEMON_7_START_R = crate::BitReader;
#[doc = "Field `u0_sft7110_noc_bus_oic_evemon_7_start` writer - u0_sft7110_noc_bus_oic_evemon_7_start"]
pub type U0_SFT7110_NOC_BUS_OIC_EVEMON_7_START_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `u0_sft7110_noc_bus_oic_evemon_7_trigger` reader - u0_sft7110_noc_bus_oic_evemon_7_trigger"]
pub type U0_SFT7110_NOC_BUS_OIC_EVEMON_7_TRIGGER_R = crate::BitReader;
#[doc = "Field `u0_sft7110_noc_bus_oic_evemon_8_start` reader - u0_sft7110_noc_bus_oic_evemon_8_start"]
pub type U0_SFT7110_NOC_BUS_OIC_EVEMON_8_START_R = crate::BitReader;
#[doc = "Field `u0_sft7110_noc_bus_oic_evemon_8_start` writer - u0_sft7110_noc_bus_oic_evemon_8_start"]
pub type U0_SFT7110_NOC_BUS_OIC_EVEMON_8_START_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `u0_sft7110_noc_bus_oic_evemon_8_trigger` reader - u0_sft7110_noc_bus_oic_evemon_8_trigger"]
pub type U0_SFT7110_NOC_BUS_OIC_EVEMON_8_TRIGGER_R = crate::BitReader;
impl R {
    #[doc = "Bit 0 - u0_sft7110_noc_bus_oic_evemon_6_trigger"]
    #[inline(always)]
    pub fn u0_sft7110_noc_bus_oic_evemon_6_trigger(
        &self,
    ) -> U0_SFT7110_NOC_BUS_OIC_EVEMON_6_TRIGGER_R {
        U0_SFT7110_NOC_BUS_OIC_EVEMON_6_TRIGGER_R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 5 - u0_sft7110_noc_bus_oic_ignore_modifiable_0"]
    #[inline(always)]
    pub fn u0_sft7110_noc_bus_oic_ignore_modifiable_0(
        &self,
    ) -> U0_SFT7110_NOC_BUS_OIC_IGNORE_MODIFIABLE_0_R {
        U0_SFT7110_NOC_BUS_OIC_IGNORE_MODIFIABLE_0_R::new(((self.bits >> 5) & 1) != 0)
    }
    #[doc = "Bit 6 - u0_sft7110_noc_bus_oic_ignore_modifiable_1"]
    #[inline(always)]
    pub fn u0_sft7110_noc_bus_oic_ignore_modifiable_1(
        &self,
    ) -> U0_SFT7110_NOC_BUS_OIC_IGNORE_MODIFIABLE_1_R {
        U0_SFT7110_NOC_BUS_OIC_IGNORE_MODIFIABLE_1_R::new(((self.bits >> 6) & 1) != 0)
    }
    #[doc = "Bit 7 - u0_sft7110_noc_bus_oic_ignore_modifiable_2"]
    #[inline(always)]
    pub fn u0_sft7110_noc_bus_oic_ignore_modifiable_2(
        &self,
    ) -> U0_SFT7110_NOC_BUS_OIC_IGNORE_MODIFIABLE_2_R {
        U0_SFT7110_NOC_BUS_OIC_IGNORE_MODIFIABLE_2_R::new(((self.bits >> 7) & 1) != 0)
    }
    #[doc = "Bit 8 - u0_sft7110_noc_bus_oic_ignore_modifiable_3"]
    #[inline(always)]
    pub fn u0_sft7110_noc_bus_oic_ignore_modifiable_3(
        &self,
    ) -> U0_SFT7110_NOC_BUS_OIC_IGNORE_MODIFIABLE_3_R {
        U0_SFT7110_NOC_BUS_OIC_IGNORE_MODIFIABLE_3_R::new(((self.bits >> 8) & 1) != 0)
    }
    #[doc = "Bit 9 - u0_sft7110_noc_bus_oic_ignore_modifiable_4"]
    #[inline(always)]
    pub fn u0_sft7110_noc_bus_oic_ignore_modifiable_4(
        &self,
    ) -> U0_SFT7110_NOC_BUS_OIC_IGNORE_MODIFIABLE_4_R {
        U0_SFT7110_NOC_BUS_OIC_IGNORE_MODIFIABLE_4_R::new(((self.bits >> 9) & 1) != 0)
    }
    #[doc = "Bit 15 - u0_sft7110_noc_bus_oic_evemon_7_start"]
    #[inline(always)]
    pub fn u0_sft7110_noc_bus_oic_evemon_7_start(&self) -> U0_SFT7110_NOC_BUS_OIC_EVEMON_7_START_R {
        U0_SFT7110_NOC_BUS_OIC_EVEMON_7_START_R::new(((self.bits >> 15) & 1) != 0)
    }
    #[doc = "Bit 16 - u0_sft7110_noc_bus_oic_evemon_7_trigger"]
    #[inline(always)]
    pub fn u0_sft7110_noc_bus_oic_evemon_7_trigger(
        &self,
    ) -> U0_SFT7110_NOC_BUS_OIC_EVEMON_7_TRIGGER_R {
        U0_SFT7110_NOC_BUS_OIC_EVEMON_7_TRIGGER_R::new(((self.bits >> 16) & 1) != 0)
    }
    #[doc = "Bit 17 - u0_sft7110_noc_bus_oic_evemon_8_start"]
    #[inline(always)]
    pub fn u0_sft7110_noc_bus_oic_evemon_8_start(&self) -> U0_SFT7110_NOC_BUS_OIC_EVEMON_8_START_R {
        U0_SFT7110_NOC_BUS_OIC_EVEMON_8_START_R::new(((self.bits >> 17) & 1) != 0)
    }
    #[doc = "Bit 18 - u0_sft7110_noc_bus_oic_evemon_8_trigger"]
    #[inline(always)]
    pub fn u0_sft7110_noc_bus_oic_evemon_8_trigger(
        &self,
    ) -> U0_SFT7110_NOC_BUS_OIC_EVEMON_8_TRIGGER_R {
        U0_SFT7110_NOC_BUS_OIC_EVEMON_8_TRIGGER_R::new(((self.bits >> 18) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 5 - u0_sft7110_noc_bus_oic_ignore_modifiable_0"]
    #[inline(always)]
    #[must_use]
    pub fn u0_sft7110_noc_bus_oic_ignore_modifiable_0(
        &mut self,
    ) -> U0_SFT7110_NOC_BUS_OIC_IGNORE_MODIFIABLE_0_W<SYS_SYSCONSAIF_SYSCFG56_SPEC> {
        U0_SFT7110_NOC_BUS_OIC_IGNORE_MODIFIABLE_0_W::new(self, 5)
    }
    #[doc = "Bit 6 - u0_sft7110_noc_bus_oic_ignore_modifiable_1"]
    #[inline(always)]
    #[must_use]
    pub fn u0_sft7110_noc_bus_oic_ignore_modifiable_1(
        &mut self,
    ) -> U0_SFT7110_NOC_BUS_OIC_IGNORE_MODIFIABLE_1_W<SYS_SYSCONSAIF_SYSCFG56_SPEC> {
        U0_SFT7110_NOC_BUS_OIC_IGNORE_MODIFIABLE_1_W::new(self, 6)
    }
    #[doc = "Bit 7 - u0_sft7110_noc_bus_oic_ignore_modifiable_2"]
    #[inline(always)]
    #[must_use]
    pub fn u0_sft7110_noc_bus_oic_ignore_modifiable_2(
        &mut self,
    ) -> U0_SFT7110_NOC_BUS_OIC_IGNORE_MODIFIABLE_2_W<SYS_SYSCONSAIF_SYSCFG56_SPEC> {
        U0_SFT7110_NOC_BUS_OIC_IGNORE_MODIFIABLE_2_W::new(self, 7)
    }
    #[doc = "Bit 8 - u0_sft7110_noc_bus_oic_ignore_modifiable_3"]
    #[inline(always)]
    #[must_use]
    pub fn u0_sft7110_noc_bus_oic_ignore_modifiable_3(
        &mut self,
    ) -> U0_SFT7110_NOC_BUS_OIC_IGNORE_MODIFIABLE_3_W<SYS_SYSCONSAIF_SYSCFG56_SPEC> {
        U0_SFT7110_NOC_BUS_OIC_IGNORE_MODIFIABLE_3_W::new(self, 8)
    }
    #[doc = "Bit 9 - u0_sft7110_noc_bus_oic_ignore_modifiable_4"]
    #[inline(always)]
    #[must_use]
    pub fn u0_sft7110_noc_bus_oic_ignore_modifiable_4(
        &mut self,
    ) -> U0_SFT7110_NOC_BUS_OIC_IGNORE_MODIFIABLE_4_W<SYS_SYSCONSAIF_SYSCFG56_SPEC> {
        U0_SFT7110_NOC_BUS_OIC_IGNORE_MODIFIABLE_4_W::new(self, 9)
    }
    #[doc = "Bit 15 - u0_sft7110_noc_bus_oic_evemon_7_start"]
    #[inline(always)]
    #[must_use]
    pub fn u0_sft7110_noc_bus_oic_evemon_7_start(
        &mut self,
    ) -> U0_SFT7110_NOC_BUS_OIC_EVEMON_7_START_W<SYS_SYSCONSAIF_SYSCFG56_SPEC> {
        U0_SFT7110_NOC_BUS_OIC_EVEMON_7_START_W::new(self, 15)
    }
    #[doc = "Bit 17 - u0_sft7110_noc_bus_oic_evemon_8_start"]
    #[inline(always)]
    #[must_use]
    pub fn u0_sft7110_noc_bus_oic_evemon_8_start(
        &mut self,
    ) -> U0_SFT7110_NOC_BUS_OIC_EVEMON_8_START_W<SYS_SYSCONSAIF_SYSCFG56_SPEC> {
        U0_SFT7110_NOC_BUS_OIC_EVEMON_8_START_W::new(self, 17)
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
#[doc = "SYS SYSCONSAIF SYSCFG 56\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`sys_sysconsaif_syscfg56::R`](R).  You can [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`sys_sysconsaif_syscfg56::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct SYS_SYSCONSAIF_SYSCFG56_SPEC;
impl crate::RegisterSpec for SYS_SYSCONSAIF_SYSCFG56_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`sys_sysconsaif_syscfg56::R`](R) reader structure"]
impl crate::Readable for SYS_SYSCONSAIF_SYSCFG56_SPEC {}
#[doc = "`write(|w| ..)` method takes [`sys_sysconsaif_syscfg56::W`](W) writer structure"]
impl crate::Writable for SYS_SYSCONSAIF_SYSCFG56_SPEC {
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
