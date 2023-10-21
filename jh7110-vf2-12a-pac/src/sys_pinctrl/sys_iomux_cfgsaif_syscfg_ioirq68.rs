#[doc = "Register `sys_iomux_cfgsaif_syscfg_ioirq68` reader"]
pub type R = crate::R<SYS_IOMUX_CFGSAIF_SYSCFG_IOIRQ68_SPEC>;
#[doc = "Register `sys_iomux_cfgsaif_syscfg_ioirq68` writer"]
pub type W = crate::W<SYS_IOMUX_CFGSAIF_SYSCFG_IOIRQ68_SPEC>;
#[doc = "Field `sys_gpiomis_0_reg` reader - The masked GPIO IRQ status"]
pub type SYS_GPIOMIS_0_REG_R = crate::FieldReader<u32>;
impl R {
    #[doc = "Bits 0:31 - The masked GPIO IRQ status"]
    #[inline(always)]
    pub fn sys_gpiomis_0_reg(&self) -> SYS_GPIOMIS_0_REG_R {
        SYS_GPIOMIS_0_REG_R::new(self.bits)
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
#[doc = "SYS IOMUX CFG SAIF SYSCFG IOIRQ 68\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`sys_iomux_cfgsaif_syscfg_ioirq68::R`](R).  You can [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`sys_iomux_cfgsaif_syscfg_ioirq68::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct SYS_IOMUX_CFGSAIF_SYSCFG_IOIRQ68_SPEC;
impl crate::RegisterSpec for SYS_IOMUX_CFGSAIF_SYSCFG_IOIRQ68_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`sys_iomux_cfgsaif_syscfg_ioirq68::R`](R) reader structure"]
impl crate::Readable for SYS_IOMUX_CFGSAIF_SYSCFG_IOIRQ68_SPEC {}
#[doc = "`write(|w| ..)` method takes [`sys_iomux_cfgsaif_syscfg_ioirq68::W`](W) writer structure"]
impl crate::Writable for SYS_IOMUX_CFGSAIF_SYSCFG_IOIRQ68_SPEC {
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
