#[doc = "Register `aon_iomux_cfgsaif_syscfg_ioirq8` reader"]
pub type R = crate::R<AON_IOMUX_CFGSAIF_SYSCFG_IOIRQ8_SPEC>;
#[doc = "Register `aon_iomux_cfgsaif_syscfg_ioirq8` writer"]
pub type W = crate::W<AON_IOMUX_CFGSAIF_SYSCFG_IOIRQ8_SPEC>;
#[doc = "Field `aon_gpioie_0_reg` reader - 1: Unmask, 0: Mask"]
pub type AON_GPIOIE_0_REG_R = crate::FieldReader;
#[doc = "Field `aon_gpioie_0_reg` writer - 1: Unmask, 0: Mask"]
pub type AON_GPIOIE_0_REG_W<'a, REG> = crate::FieldWriter<'a, REG, 4>;
impl R {
    #[doc = "Bits 0:3 - 1: Unmask, 0: Mask"]
    #[inline(always)]
    pub fn aon_gpioie_0_reg(&self) -> AON_GPIOIE_0_REG_R {
        AON_GPIOIE_0_REG_R::new((self.bits & 0x0f) as u8)
    }
}
impl W {
    #[doc = "Bits 0:3 - 1: Unmask, 0: Mask"]
    #[inline(always)]
    #[must_use]
    pub fn aon_gpioie_0_reg(&mut self) -> AON_GPIOIE_0_REG_W<AON_IOMUX_CFGSAIF_SYSCFG_IOIRQ8_SPEC> {
        AON_GPIOIE_0_REG_W::new(self, 0)
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
#[doc = "AON IOMUX CFG SAIF SYSCFG IOIRQ 8\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`aon_iomux_cfgsaif_syscfg_ioirq8::R`](R).  You can [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`aon_iomux_cfgsaif_syscfg_ioirq8::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct AON_IOMUX_CFGSAIF_SYSCFG_IOIRQ8_SPEC;
impl crate::RegisterSpec for AON_IOMUX_CFGSAIF_SYSCFG_IOIRQ8_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`aon_iomux_cfgsaif_syscfg_ioirq8::R`](R) reader structure"]
impl crate::Readable for AON_IOMUX_CFGSAIF_SYSCFG_IOIRQ8_SPEC {}
#[doc = "`write(|w| ..)` method takes [`aon_iomux_cfgsaif_syscfg_ioirq8::W`](W) writer structure"]
impl crate::Writable for AON_IOMUX_CFGSAIF_SYSCFG_IOIRQ8_SPEC {
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
