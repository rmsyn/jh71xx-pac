#[doc = "Register `sys_syscfg_14` reader"]
pub type R = crate::R<SYS_SYSCFG_14_SPEC>;
#[doc = "Register `sys_syscfg_14` writer"]
pub type W = crate::W<SYS_SYSCFG_14_SPEC>;
#[doc = "Field `noc_bus_oic_evemon_trigger_6` reader - noc_bus_oic_evemon_trigger_6"]
pub type NOC_BUS_OIC_EVEMON_TRIGGER_6_R = crate::BitReader;
#[doc = "Field `noc_bus_oic_ignore_modifiable_0` reader - noc_bus_oic_ignore_modifiable_0"]
pub type NOC_BUS_OIC_IGNORE_MODIFIABLE_0_R = crate::BitReader;
#[doc = "Field `noc_bus_oic_ignore_modifiable_0` writer - noc_bus_oic_ignore_modifiable_0"]
pub type NOC_BUS_OIC_IGNORE_MODIFIABLE_0_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `noc_bus_oic_ignore_modifiable_1` reader - noc_bus_oic_ignore_modifiable_1"]
pub type NOC_BUS_OIC_IGNORE_MODIFIABLE_1_R = crate::BitReader;
#[doc = "Field `noc_bus_oic_ignore_modifiable_1` writer - noc_bus_oic_ignore_modifiable_1"]
pub type NOC_BUS_OIC_IGNORE_MODIFIABLE_1_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `noc_bus_oic_ignore_modifiable_2` reader - noc_bus_oic_ignore_modifiable_2"]
pub type NOC_BUS_OIC_IGNORE_MODIFIABLE_2_R = crate::BitReader;
#[doc = "Field `noc_bus_oic_ignore_modifiable_2` writer - noc_bus_oic_ignore_modifiable_2"]
pub type NOC_BUS_OIC_IGNORE_MODIFIABLE_2_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `noc_bus_oic_ignore_modifiable_3` reader - noc_bus_oic_ignore_modifiable_3"]
pub type NOC_BUS_OIC_IGNORE_MODIFIABLE_3_R = crate::BitReader;
#[doc = "Field `noc_bus_oic_ignore_modifiable_3` writer - noc_bus_oic_ignore_modifiable_3"]
pub type NOC_BUS_OIC_IGNORE_MODIFIABLE_3_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `noc_bus_oic_ignore_modifiable_4` reader - noc_bus_oic_ignore_modifiable_4"]
pub type NOC_BUS_OIC_IGNORE_MODIFIABLE_4_R = crate::BitReader;
#[doc = "Field `noc_bus_oic_ignore_modifiable_4` writer - noc_bus_oic_ignore_modifiable_4"]
pub type NOC_BUS_OIC_IGNORE_MODIFIABLE_4_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `noc_bus_oic_evemon_start_7` reader - noc_bus_oic_evemon_start_7"]
pub type NOC_BUS_OIC_EVEMON_START_7_R = crate::BitReader;
#[doc = "Field `noc_bus_oic_evemon_start_7` writer - noc_bus_oic_evemon_start_7"]
pub type NOC_BUS_OIC_EVEMON_START_7_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `noc_bus_oic_evemon_trigger_7` reader - noc_bus_oic_evemon_trigger_7"]
pub type NOC_BUS_OIC_EVEMON_TRIGGER_7_R = crate::BitReader;
#[doc = "Field `noc_bus_oic_evemon_start_8` reader - noc_bus_oic_evemon_start_8"]
pub type NOC_BUS_OIC_EVEMON_START_8_R = crate::BitReader;
#[doc = "Field `noc_bus_oic_evemon_start_8` writer - noc_bus_oic_evemon_start_8"]
pub type NOC_BUS_OIC_EVEMON_START_8_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `noc_bus_oic_evemon_trigger_8` reader - noc_bus_oic_evemon_trigger_8"]
pub type NOC_BUS_OIC_EVEMON_TRIGGER_8_R = crate::BitReader;
impl R {
    #[doc = "Bit 0 - noc_bus_oic_evemon_trigger_6"]
    #[inline(always)]
    pub fn noc_bus_oic_evemon_trigger_6(&self) -> NOC_BUS_OIC_EVEMON_TRIGGER_6_R {
        NOC_BUS_OIC_EVEMON_TRIGGER_6_R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 5 - noc_bus_oic_ignore_modifiable_0"]
    #[inline(always)]
    pub fn noc_bus_oic_ignore_modifiable_0(&self) -> NOC_BUS_OIC_IGNORE_MODIFIABLE_0_R {
        NOC_BUS_OIC_IGNORE_MODIFIABLE_0_R::new(((self.bits >> 5) & 1) != 0)
    }
    #[doc = "Bit 6 - noc_bus_oic_ignore_modifiable_1"]
    #[inline(always)]
    pub fn noc_bus_oic_ignore_modifiable_1(&self) -> NOC_BUS_OIC_IGNORE_MODIFIABLE_1_R {
        NOC_BUS_OIC_IGNORE_MODIFIABLE_1_R::new(((self.bits >> 6) & 1) != 0)
    }
    #[doc = "Bit 7 - noc_bus_oic_ignore_modifiable_2"]
    #[inline(always)]
    pub fn noc_bus_oic_ignore_modifiable_2(&self) -> NOC_BUS_OIC_IGNORE_MODIFIABLE_2_R {
        NOC_BUS_OIC_IGNORE_MODIFIABLE_2_R::new(((self.bits >> 7) & 1) != 0)
    }
    #[doc = "Bit 8 - noc_bus_oic_ignore_modifiable_3"]
    #[inline(always)]
    pub fn noc_bus_oic_ignore_modifiable_3(&self) -> NOC_BUS_OIC_IGNORE_MODIFIABLE_3_R {
        NOC_BUS_OIC_IGNORE_MODIFIABLE_3_R::new(((self.bits >> 8) & 1) != 0)
    }
    #[doc = "Bit 9 - noc_bus_oic_ignore_modifiable_4"]
    #[inline(always)]
    pub fn noc_bus_oic_ignore_modifiable_4(&self) -> NOC_BUS_OIC_IGNORE_MODIFIABLE_4_R {
        NOC_BUS_OIC_IGNORE_MODIFIABLE_4_R::new(((self.bits >> 9) & 1) != 0)
    }
    #[doc = "Bit 15 - noc_bus_oic_evemon_start_7"]
    #[inline(always)]
    pub fn noc_bus_oic_evemon_start_7(&self) -> NOC_BUS_OIC_EVEMON_START_7_R {
        NOC_BUS_OIC_EVEMON_START_7_R::new(((self.bits >> 15) & 1) != 0)
    }
    #[doc = "Bit 16 - noc_bus_oic_evemon_trigger_7"]
    #[inline(always)]
    pub fn noc_bus_oic_evemon_trigger_7(&self) -> NOC_BUS_OIC_EVEMON_TRIGGER_7_R {
        NOC_BUS_OIC_EVEMON_TRIGGER_7_R::new(((self.bits >> 16) & 1) != 0)
    }
    #[doc = "Bit 17 - noc_bus_oic_evemon_start_8"]
    #[inline(always)]
    pub fn noc_bus_oic_evemon_start_8(&self) -> NOC_BUS_OIC_EVEMON_START_8_R {
        NOC_BUS_OIC_EVEMON_START_8_R::new(((self.bits >> 17) & 1) != 0)
    }
    #[doc = "Bit 18 - noc_bus_oic_evemon_trigger_8"]
    #[inline(always)]
    pub fn noc_bus_oic_evemon_trigger_8(&self) -> NOC_BUS_OIC_EVEMON_TRIGGER_8_R {
        NOC_BUS_OIC_EVEMON_TRIGGER_8_R::new(((self.bits >> 18) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 5 - noc_bus_oic_ignore_modifiable_0"]
    #[inline(always)]
    #[must_use]
    pub fn noc_bus_oic_ignore_modifiable_0(
        &mut self,
    ) -> NOC_BUS_OIC_IGNORE_MODIFIABLE_0_W<SYS_SYSCFG_14_SPEC> {
        NOC_BUS_OIC_IGNORE_MODIFIABLE_0_W::new(self, 5)
    }
    #[doc = "Bit 6 - noc_bus_oic_ignore_modifiable_1"]
    #[inline(always)]
    #[must_use]
    pub fn noc_bus_oic_ignore_modifiable_1(
        &mut self,
    ) -> NOC_BUS_OIC_IGNORE_MODIFIABLE_1_W<SYS_SYSCFG_14_SPEC> {
        NOC_BUS_OIC_IGNORE_MODIFIABLE_1_W::new(self, 6)
    }
    #[doc = "Bit 7 - noc_bus_oic_ignore_modifiable_2"]
    #[inline(always)]
    #[must_use]
    pub fn noc_bus_oic_ignore_modifiable_2(
        &mut self,
    ) -> NOC_BUS_OIC_IGNORE_MODIFIABLE_2_W<SYS_SYSCFG_14_SPEC> {
        NOC_BUS_OIC_IGNORE_MODIFIABLE_2_W::new(self, 7)
    }
    #[doc = "Bit 8 - noc_bus_oic_ignore_modifiable_3"]
    #[inline(always)]
    #[must_use]
    pub fn noc_bus_oic_ignore_modifiable_3(
        &mut self,
    ) -> NOC_BUS_OIC_IGNORE_MODIFIABLE_3_W<SYS_SYSCFG_14_SPEC> {
        NOC_BUS_OIC_IGNORE_MODIFIABLE_3_W::new(self, 8)
    }
    #[doc = "Bit 9 - noc_bus_oic_ignore_modifiable_4"]
    #[inline(always)]
    #[must_use]
    pub fn noc_bus_oic_ignore_modifiable_4(
        &mut self,
    ) -> NOC_BUS_OIC_IGNORE_MODIFIABLE_4_W<SYS_SYSCFG_14_SPEC> {
        NOC_BUS_OIC_IGNORE_MODIFIABLE_4_W::new(self, 9)
    }
    #[doc = "Bit 15 - noc_bus_oic_evemon_start_7"]
    #[inline(always)]
    #[must_use]
    pub fn noc_bus_oic_evemon_start_7(
        &mut self,
    ) -> NOC_BUS_OIC_EVEMON_START_7_W<SYS_SYSCFG_14_SPEC> {
        NOC_BUS_OIC_EVEMON_START_7_W::new(self, 15)
    }
    #[doc = "Bit 17 - noc_bus_oic_evemon_start_8"]
    #[inline(always)]
    #[must_use]
    pub fn noc_bus_oic_evemon_start_8(
        &mut self,
    ) -> NOC_BUS_OIC_EVEMON_START_8_W<SYS_SYSCFG_14_SPEC> {
        NOC_BUS_OIC_EVEMON_START_8_W::new(self, 17)
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
#[doc = "SYS SYSCONSAIF SYSCFG 56\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`sys_syscfg_14::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`sys_syscfg_14::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct SYS_SYSCFG_14_SPEC;
impl crate::RegisterSpec for SYS_SYSCFG_14_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`sys_syscfg_14::R`](R) reader structure"]
impl crate::Readable for SYS_SYSCFG_14_SPEC {}
#[doc = "`write(|w| ..)` method takes [`sys_syscfg_14::W`](W) writer structure"]
impl crate::Writable for SYS_SYSCFG_14_SPEC {
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets sys_syscfg_14 to value 0"]
impl crate::Resettable for SYS_SYSCFG_14_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
