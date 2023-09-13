#[doc = "Register `aon_iomux_cfgsaif_syscfg_ioirq7` reader"]
pub type R = crate::R<AON_IOMUX_CFGSAIF_SYSCFG_IOIRQ7_SPEC>;
#[doc = "Register `aon_iomux_cfgsaif_syscfg_ioirq7` writer"]
pub type W = crate::W<AON_IOMUX_CFGSAIF_SYSCFG_IOIRQ7_SPEC>;
#[doc = "Field `aon_gpioiev_0_reg` reader - 1: Positive/Low, 0: Negative/High"]
pub type AON_GPIOIEV_0_REG_R = crate::FieldReader;
#[doc = "Field `aon_gpioiev_0_reg` writer - 1: Positive/Low, 0: Negative/High"]
pub type AON_GPIOIEV_0_REG_W<'a, REG, const O: u8> = crate::FieldWriter<'a, REG, 4, O>;
impl R {
    #[doc = "Bits 0:3 - 1: Positive/Low, 0: Negative/High"]
    #[inline(always)]
    pub fn aon_gpioiev_0_reg(&self) -> AON_GPIOIEV_0_REG_R {
        AON_GPIOIEV_0_REG_R::new((self.bits & 0x0f) as u8)
    }
}
impl W {
    #[doc = "Bits 0:3 - 1: Positive/Low, 0: Negative/High"]
    #[inline(always)]
    #[must_use]
    pub fn aon_gpioiev_0_reg(
        &mut self,
    ) -> AON_GPIOIEV_0_REG_W<AON_IOMUX_CFGSAIF_SYSCFG_IOIRQ7_SPEC, 0> {
        AON_GPIOIEV_0_REG_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.bits = bits;
        self
    }
}
#[doc = "AON IOMUX CFG SAIF SYSCFG IOIRQ 7\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`aon_iomux_cfgsaif_syscfg_ioirq7::R`](R).  You can [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`aon_iomux_cfgsaif_syscfg_ioirq7::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct AON_IOMUX_CFGSAIF_SYSCFG_IOIRQ7_SPEC;
impl crate::RegisterSpec for AON_IOMUX_CFGSAIF_SYSCFG_IOIRQ7_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`aon_iomux_cfgsaif_syscfg_ioirq7::R`](R) reader structure"]
impl crate::Readable for AON_IOMUX_CFGSAIF_SYSCFG_IOIRQ7_SPEC {}
#[doc = "`write(|w| ..)` method takes [`aon_iomux_cfgsaif_syscfg_ioirq7::W`](W) writer structure"]
impl crate::Writable for AON_IOMUX_CFGSAIF_SYSCFG_IOIRQ7_SPEC {
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
