#[doc = "Register `sys_iomux_cfgsaif_syscfg_ioirq55` reader"]
pub type R = crate::R<SYS_IOMUX_CFGSAIF_SYSCFG_IOIRQ55_SPEC>;
#[doc = "Register `sys_iomux_cfgsaif_syscfg_ioirq55` writer"]
pub type W = crate::W<SYS_IOMUX_CFGSAIF_SYSCFG_IOIRQ55_SPEC>;
#[doc = "Field `sys_gpioen_0_reg` reader - Enable GPIO IRQ function"]
pub type SYS_GPIOEN_0_REG_R = crate::BitReader;
#[doc = "Field `sys_gpioen_0_reg` writer - Enable GPIO IRQ function"]
pub type SYS_GPIOEN_0_REG_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O>;
impl R {
    #[doc = "Bit 0 - Enable GPIO IRQ function"]
    #[inline(always)]
    pub fn sys_gpioen_0_reg(&self) -> SYS_GPIOEN_0_REG_R {
        SYS_GPIOEN_0_REG_R::new((self.bits & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - Enable GPIO IRQ function"]
    #[inline(always)]
    #[must_use]
    pub fn sys_gpioen_0_reg(
        &mut self,
    ) -> SYS_GPIOEN_0_REG_W<SYS_IOMUX_CFGSAIF_SYSCFG_IOIRQ55_SPEC, 0> {
        SYS_GPIOEN_0_REG_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.bits = bits;
        self
    }
}
#[doc = "SYS IOMUX CFG SAIF SYSCFG IOIRQ 55\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`sys_iomux_cfgsaif_syscfg_ioirq55::R`](R).  You can [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`sys_iomux_cfgsaif_syscfg_ioirq55::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct SYS_IOMUX_CFGSAIF_SYSCFG_IOIRQ55_SPEC;
impl crate::RegisterSpec for SYS_IOMUX_CFGSAIF_SYSCFG_IOIRQ55_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`sys_iomux_cfgsaif_syscfg_ioirq55::R`](R) reader structure"]
impl crate::Readable for SYS_IOMUX_CFGSAIF_SYSCFG_IOIRQ55_SPEC {}
#[doc = "`write(|w| ..)` method takes [`sys_iomux_cfgsaif_syscfg_ioirq55::W`](W) writer structure"]
impl crate::Writable for SYS_IOMUX_CFGSAIF_SYSCFG_IOIRQ55_SPEC {
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
