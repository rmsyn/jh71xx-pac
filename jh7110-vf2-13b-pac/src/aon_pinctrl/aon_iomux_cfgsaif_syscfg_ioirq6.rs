#[doc = "Register `aon_iomux_cfgsaif_syscfg_ioirq6` reader"]
pub type R = crate::R<AON_IOMUX_CFGSAIF_SYSCFG_IOIRQ6_SPEC>;
#[doc = "Register `aon_iomux_cfgsaif_syscfg_ioirq6` writer"]
pub type W = crate::W<AON_IOMUX_CFGSAIF_SYSCFG_IOIRQ6_SPEC>;
#[doc = "Field `aon_gpioibe_0_reg` reader - 1: Trigger on both edges, 0: Trigger on a single edge"]
pub type AON_GPIOIBE_0_REG_R = crate::FieldReader;
#[doc = "Field `aon_gpioibe_0_reg` writer - 1: Trigger on both edges, 0: Trigger on a single edge"]
pub type AON_GPIOIBE_0_REG_W<'a, REG, const O: u8> = crate::FieldWriter<'a, REG, 4, O>;
impl R {
    #[doc = "Bits 0:3 - 1: Trigger on both edges, 0: Trigger on a single edge"]
    #[inline(always)]
    pub fn aon_gpioibe_0_reg(&self) -> AON_GPIOIBE_0_REG_R {
        AON_GPIOIBE_0_REG_R::new((self.bits & 0x0f) as u8)
    }
}
impl W {
    #[doc = "Bits 0:3 - 1: Trigger on both edges, 0: Trigger on a single edge"]
    #[inline(always)]
    #[must_use]
    pub fn aon_gpioibe_0_reg(
        &mut self,
    ) -> AON_GPIOIBE_0_REG_W<AON_IOMUX_CFGSAIF_SYSCFG_IOIRQ6_SPEC, 0> {
        AON_GPIOIBE_0_REG_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.bits = bits;
        self
    }
}
#[doc = "AON IOMUX CFG SAIF SYSCFG IOIRQ 6\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`aon_iomux_cfgsaif_syscfg_ioirq6::R`](R).  You can [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`aon_iomux_cfgsaif_syscfg_ioirq6::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct AON_IOMUX_CFGSAIF_SYSCFG_IOIRQ6_SPEC;
impl crate::RegisterSpec for AON_IOMUX_CFGSAIF_SYSCFG_IOIRQ6_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`aon_iomux_cfgsaif_syscfg_ioirq6::R`](R) reader structure"]
impl crate::Readable for AON_IOMUX_CFGSAIF_SYSCFG_IOIRQ6_SPEC {}
#[doc = "`write(|w| ..)` method takes [`aon_iomux_cfgsaif_syscfg_ioirq6::W`](W) writer structure"]
impl crate::Writable for AON_IOMUX_CFGSAIF_SYSCFG_IOIRQ6_SPEC {
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
