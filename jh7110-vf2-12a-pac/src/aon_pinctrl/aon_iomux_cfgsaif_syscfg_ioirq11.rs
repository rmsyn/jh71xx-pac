#[doc = "Register `aon_iomux_cfgsaif_syscfg_ioirq11` reader"]
pub type R = crate::R<AON_IOMUX_CFGSAIF_SYSCFG_IOIRQ11_SPEC>;
#[doc = "Register `aon_iomux_cfgsaif_syscfg_ioirq11` writer"]
pub type W = crate::W<AON_IOMUX_CFGSAIF_SYSCFG_IOIRQ11_SPEC>;
#[doc = "Field `aon_gpio_in_sync2_0_reg` reader - Status of gpio_in after synchronization."]
pub type AON_GPIO_IN_SYNC2_0_REG_R = crate::FieldReader;
impl R {
    #[doc = "Bits 0:3 - Status of gpio_in after synchronization."]
    #[inline(always)]
    pub fn aon_gpio_in_sync2_0_reg(&self) -> AON_GPIO_IN_SYNC2_0_REG_R {
        AON_GPIO_IN_SYNC2_0_REG_R::new((self.bits & 0x0f) as u8)
    }
}
impl W {
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
#[doc = "AON IOMUX CFG SAIF SYSCFG IOIRQ 11\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`aon_iomux_cfgsaif_syscfg_ioirq11::R`](R).  You can [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`aon_iomux_cfgsaif_syscfg_ioirq11::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct AON_IOMUX_CFGSAIF_SYSCFG_IOIRQ11_SPEC;
impl crate::RegisterSpec for AON_IOMUX_CFGSAIF_SYSCFG_IOIRQ11_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`aon_iomux_cfgsaif_syscfg_ioirq11::R`](R) reader structure"]
impl crate::Readable for AON_IOMUX_CFGSAIF_SYSCFG_IOIRQ11_SPEC {}
#[doc = "`write(|w| ..)` method takes [`aon_iomux_cfgsaif_syscfg_ioirq11::W`](W) writer structure"]
impl crate::Writable for AON_IOMUX_CFGSAIF_SYSCFG_IOIRQ11_SPEC {
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
