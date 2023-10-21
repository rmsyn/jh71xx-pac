#[doc = "Register `sys_iomux_cfgsaif_syscfg_ioirq57` reader"]
pub type R = crate::R<SYS_IOMUX_CFGSAIF_SYSCFG_IOIRQ57_SPEC>;
#[doc = "Register `sys_iomux_cfgsaif_syscfg_ioirq57` writer"]
pub type W = crate::W<SYS_IOMUX_CFGSAIF_SYSCFG_IOIRQ57_SPEC>;
#[doc = "Field `sys_gpiois_1_reg` reader - 1: Edge trigger, 0: Level trigger"]
pub type SYS_GPIOIS_1_REG_R = crate::FieldReader<u32>;
#[doc = "Field `sys_gpiois_1_reg` writer - 1: Edge trigger, 0: Level trigger"]
pub type SYS_GPIOIS_1_REG_W<'a, REG, const O: u8> = crate::FieldWriter<'a, REG, 32, O, u32>;
impl R {
    #[doc = "Bits 0:31 - 1: Edge trigger, 0: Level trigger"]
    #[inline(always)]
    pub fn sys_gpiois_1_reg(&self) -> SYS_GPIOIS_1_REG_R {
        SYS_GPIOIS_1_REG_R::new(self.bits)
    }
}
impl W {
    #[doc = "Bits 0:31 - 1: Edge trigger, 0: Level trigger"]
    #[inline(always)]
    #[must_use]
    pub fn sys_gpiois_1_reg(
        &mut self,
    ) -> SYS_GPIOIS_1_REG_W<SYS_IOMUX_CFGSAIF_SYSCFG_IOIRQ57_SPEC, 0> {
        SYS_GPIOIS_1_REG_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.bits = bits;
        self
    }
}
#[doc = "SYS IOMUX CFG SAIF SYSCFG IOIRQ 57\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`sys_iomux_cfgsaif_syscfg_ioirq57::R`](R).  You can [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`sys_iomux_cfgsaif_syscfg_ioirq57::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct SYS_IOMUX_CFGSAIF_SYSCFG_IOIRQ57_SPEC;
impl crate::RegisterSpec for SYS_IOMUX_CFGSAIF_SYSCFG_IOIRQ57_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`sys_iomux_cfgsaif_syscfg_ioirq57::R`](R) reader structure"]
impl crate::Readable for SYS_IOMUX_CFGSAIF_SYSCFG_IOIRQ57_SPEC {}
#[doc = "`write(|w| ..)` method takes [`sys_iomux_cfgsaif_syscfg_ioirq57::W`](W) writer structure"]
impl crate::Writable for SYS_IOMUX_CFGSAIF_SYSCFG_IOIRQ57_SPEC {
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
