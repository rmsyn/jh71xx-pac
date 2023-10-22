#[doc = "Register `aon_iomux_cfgsaif_syscfg_ioirq10` reader"]
pub type R = crate::R<AON_IOMUX_CFGSAIF_SYSCFG_IOIRQ10_SPEC>;
#[doc = "Register `aon_iomux_cfgsaif_syscfg_ioirq10` writer"]
pub type W = crate::W<AON_IOMUX_CFGSAIF_SYSCFG_IOIRQ10_SPEC>;
#[doc = "Field `aon_gpiomis_0_reg` reader - The masked GPIO IRQ status."]
pub type AON_GPIOMIS_0_REG_R = crate::FieldReader;
impl R {
    #[doc = "Bits 0:3 - The masked GPIO IRQ status."]
    #[inline(always)]
    pub fn aon_gpiomis_0_reg(&self) -> AON_GPIOMIS_0_REG_R {
        AON_GPIOMIS_0_REG_R::new((self.bits & 0x0f) as u8)
    }
}
impl W {
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.bits = bits;
        self
    }
}
#[doc = "AON IOMUX CFG SAIF SYSCFG IOIRQ 10\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`aon_iomux_cfgsaif_syscfg_ioirq10::R`](R).  You can [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`aon_iomux_cfgsaif_syscfg_ioirq10::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct AON_IOMUX_CFGSAIF_SYSCFG_IOIRQ10_SPEC;
impl crate::RegisterSpec for AON_IOMUX_CFGSAIF_SYSCFG_IOIRQ10_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`aon_iomux_cfgsaif_syscfg_ioirq10::R`](R) reader structure"]
impl crate::Readable for AON_IOMUX_CFGSAIF_SYSCFG_IOIRQ10_SPEC {}
#[doc = "`write(|w| ..)` method takes [`aon_iomux_cfgsaif_syscfg_ioirq10::W`](W) writer structure"]
impl crate::Writable for AON_IOMUX_CFGSAIF_SYSCFG_IOIRQ10_SPEC {
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
