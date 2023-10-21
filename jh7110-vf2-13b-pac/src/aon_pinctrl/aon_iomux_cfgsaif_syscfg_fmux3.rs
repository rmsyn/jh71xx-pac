#[doc = "Register `aon_iomux_cfgsaif_syscfg_fmux3` reader"]
pub type R = crate::R<AON_IOMUX_CFGSAIF_SYSCFG_FMUX3_SPEC>;
#[doc = "Register `aon_iomux_cfgsaif_syscfg_fmux3` writer"]
pub type W = crate::W<AON_IOMUX_CFGSAIF_SYSCFG_FMUX3_SPEC>;
#[doc = "Field `aon_gpioen_0_reg` reader - Enable GPIO IRQ function."]
pub type AON_GPIOEN_0_REG_R = crate::BitReader;
#[doc = "Field `aon_gpioen_0_reg` writer - Enable GPIO IRQ function."]
pub type AON_GPIOEN_0_REG_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O>;
impl R {
    #[doc = "Bit 0 - Enable GPIO IRQ function."]
    #[inline(always)]
    pub fn aon_gpioen_0_reg(&self) -> AON_GPIOEN_0_REG_R {
        AON_GPIOEN_0_REG_R::new((self.bits & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - Enable GPIO IRQ function."]
    #[inline(always)]
    #[must_use]
    pub fn aon_gpioen_0_reg(
        &mut self,
    ) -> AON_GPIOEN_0_REG_W<AON_IOMUX_CFGSAIF_SYSCFG_FMUX3_SPEC, 0> {
        AON_GPIOEN_0_REG_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.bits = bits;
        self
    }
}
#[doc = "AON IOMUX CFG SAIF SYSCFG FMUX 3\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`aon_iomux_cfgsaif_syscfg_fmux3::R`](R).  You can [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`aon_iomux_cfgsaif_syscfg_fmux3::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct AON_IOMUX_CFGSAIF_SYSCFG_FMUX3_SPEC;
impl crate::RegisterSpec for AON_IOMUX_CFGSAIF_SYSCFG_FMUX3_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`aon_iomux_cfgsaif_syscfg_fmux3::R`](R) reader structure"]
impl crate::Readable for AON_IOMUX_CFGSAIF_SYSCFG_FMUX3_SPEC {}
#[doc = "`write(|w| ..)` method takes [`aon_iomux_cfgsaif_syscfg_fmux3::W`](W) writer structure"]
impl crate::Writable for AON_IOMUX_CFGSAIF_SYSCFG_FMUX3_SPEC {
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
