#[doc = "Register `stg_syscfg_83` reader"]
pub type R = crate::R<STG_SYSCFG_83_SPEC>;
#[doc = "Register `stg_syscfg_83` writer"]
pub type W = crate::W<STG_SYSCFG_83_SPEC>;
#[doc = "Field `u0_pcie_pl_pclk_rate` reader - u0_pcie_pl_pclk_rate"]
pub type U0_PCIE_PL_PCLK_RATE_R = crate::FieldReader;
impl R {
    #[doc = "Bits 0:4 - u0_pcie_pl_pclk_rate"]
    #[inline(always)]
    pub fn u0_pcie_pl_pclk_rate(&self) -> U0_PCIE_PL_PCLK_RATE_R {
        U0_PCIE_PL_PCLK_RATE_R::new((self.bits & 0x1f) as u8)
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
#[doc = "STG SYSCONSAIF SYSCFG 332\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`stg_syscfg_83::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`stg_syscfg_83::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct STG_SYSCFG_83_SPEC;
impl crate::RegisterSpec for STG_SYSCFG_83_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`stg_syscfg_83::R`](R) reader structure"]
impl crate::Readable for STG_SYSCFG_83_SPEC {}
#[doc = "`write(|w| ..)` method takes [`stg_syscfg_83::W`](W) writer structure"]
impl crate::Writable for STG_SYSCFG_83_SPEC {
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets stg_syscfg_83 to value 0"]
impl crate::Resettable for STG_SYSCFG_83_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
