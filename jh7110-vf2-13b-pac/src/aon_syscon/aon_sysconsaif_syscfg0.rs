#[doc = "Register `aon_sysconsaif_syscfg0` reader"]
pub type R = crate::R<AON_SYSCONSAIF_SYSCFG0_SPEC>;
#[doc = "Register `aon_sysconsaif_syscfg0` writer"]
pub type W = crate::W<AON_SYSCONSAIF_SYSCFG0_SPEC>;
#[doc = "Field `aon_gp_reg` reader - aon_gp_reg"]
pub type AON_GP_REG_R = crate::FieldReader<u32>;
#[doc = "Field `aon_gp_reg` writer - aon_gp_reg"]
pub type AON_GP_REG_W<'a, REG, const O: u8> = crate::FieldWriter<'a, REG, 32, O, u32>;
impl R {
    #[doc = "Bits 0:31 - aon_gp_reg"]
    #[inline(always)]
    pub fn aon_gp_reg(&self) -> AON_GP_REG_R {
        AON_GP_REG_R::new(self.bits)
    }
}
impl W {
    #[doc = "Bits 0:31 - aon_gp_reg"]
    #[inline(always)]
    #[must_use]
    pub fn aon_gp_reg(&mut self) -> AON_GP_REG_W<AON_SYSCONSAIF_SYSCFG0_SPEC, 0> {
        AON_GP_REG_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.bits = bits;
        self
    }
}
#[doc = "AON SYSCONSAIF SYSCFG 0\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`aon_sysconsaif_syscfg0::R`](R).  You can [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`aon_sysconsaif_syscfg0::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct AON_SYSCONSAIF_SYSCFG0_SPEC;
impl crate::RegisterSpec for AON_SYSCONSAIF_SYSCFG0_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`aon_sysconsaif_syscfg0::R`](R) reader structure"]
impl crate::Readable for AON_SYSCONSAIF_SYSCFG0_SPEC {}
#[doc = "`write(|w| ..)` method takes [`aon_sysconsaif_syscfg0::W`](W) writer structure"]
impl crate::Writable for AON_SYSCONSAIF_SYSCFG0_SPEC {
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
