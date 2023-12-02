#[doc = "Register `aon_iomux_cfgsaif_syscfg_ioirq5` reader"]
pub type R = crate::R<AON_IOMUX_CFGSAIF_SYSCFG_IOIRQ5_SPEC>;
#[doc = "Register `aon_iomux_cfgsaif_syscfg_ioirq5` writer"]
pub type W = crate::W<AON_IOMUX_CFGSAIF_SYSCFG_IOIRQ5_SPEC>;
#[doc = "Field `aon_gpioic_0_reg` reader - 1: Do not clear the register, 0: Clear the register"]
pub type AON_GPIOIC_0_REG_R = crate::FieldReader;
#[doc = "Field `aon_gpioic_0_reg` writer - 1: Do not clear the register, 0: Clear the register"]
pub type AON_GPIOIC_0_REG_W<'a, REG> = crate::FieldWriter<'a, REG, 4>;
impl R {
    #[doc = "Bits 0:3 - 1: Do not clear the register, 0: Clear the register"]
    #[inline(always)]
    pub fn aon_gpioic_0_reg(&self) -> AON_GPIOIC_0_REG_R {
        AON_GPIOIC_0_REG_R::new((self.bits & 0x0f) as u8)
    }
}
impl W {
    #[doc = "Bits 0:3 - 1: Do not clear the register, 0: Clear the register"]
    #[inline(always)]
    #[must_use]
    pub fn aon_gpioic_0_reg(&mut self) -> AON_GPIOIC_0_REG_W<AON_IOMUX_CFGSAIF_SYSCFG_IOIRQ5_SPEC> {
        AON_GPIOIC_0_REG_W::new(self, 0)
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
#[doc = "AON IOMUX CFG SAIF SYSCFG IOIRQ 5\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`aon_iomux_cfgsaif_syscfg_ioirq5::R`](R).  You can [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`aon_iomux_cfgsaif_syscfg_ioirq5::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct AON_IOMUX_CFGSAIF_SYSCFG_IOIRQ5_SPEC;
impl crate::RegisterSpec for AON_IOMUX_CFGSAIF_SYSCFG_IOIRQ5_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`aon_iomux_cfgsaif_syscfg_ioirq5::R`](R) reader structure"]
impl crate::Readable for AON_IOMUX_CFGSAIF_SYSCFG_IOIRQ5_SPEC {}
#[doc = "`write(|w| ..)` method takes [`aon_iomux_cfgsaif_syscfg_ioirq5::W`](W) writer structure"]
impl crate::Writable for AON_IOMUX_CFGSAIF_SYSCFG_IOIRQ5_SPEC {
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
