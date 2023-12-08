#[doc = "Register `sys_syscfg_7` reader"]
pub type R = crate::R<SYS_SYSCFG_7_SPEC>;
#[doc = "Register `sys_syscfg_7` writer"]
pub type W = crate::W<SYS_SYSCFG_7_SPEC>;
#[doc = "Field `u0_pll_wrap_pll0_fbdiv` reader - u0_pll_wrap_pll0_fbdiv"]
pub type U0_PLL_WRAP_PLL0_FBDIV_R = crate::FieldReader<u16>;
#[doc = "Field `u0_pll_wrap_pll0_fbdiv` writer - u0_pll_wrap_pll0_fbdiv"]
pub type U0_PLL_WRAP_PLL0_FBDIV_W<'a, REG> = crate::FieldWriter<'a, REG, 12, u16>;
impl R {
    #[doc = "Bits 0:11 - u0_pll_wrap_pll0_fbdiv"]
    #[inline(always)]
    pub fn u0_pll_wrap_pll0_fbdiv(&self) -> U0_PLL_WRAP_PLL0_FBDIV_R {
        U0_PLL_WRAP_PLL0_FBDIV_R::new((self.bits & 0x0fff) as u16)
    }
}
impl W {
    #[doc = "Bits 0:11 - u0_pll_wrap_pll0_fbdiv"]
    #[inline(always)]
    #[must_use]
    pub fn u0_pll_wrap_pll0_fbdiv(&mut self) -> U0_PLL_WRAP_PLL0_FBDIV_W<SYS_SYSCFG_7_SPEC> {
        U0_PLL_WRAP_PLL0_FBDIV_W::new(self, 0)
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
#[doc = "SYS SYSCONSAIF SYSCFG 28\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`sys_syscfg_7::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`sys_syscfg_7::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct SYS_SYSCFG_7_SPEC;
impl crate::RegisterSpec for SYS_SYSCFG_7_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`sys_syscfg_7::R`](R) reader structure"]
impl crate::Readable for SYS_SYSCFG_7_SPEC {}
#[doc = "`write(|w| ..)` method takes [`sys_syscfg_7::W`](W) writer structure"]
impl crate::Writable for SYS_SYSCFG_7_SPEC {
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets sys_syscfg_7 to value 0"]
impl crate::Resettable for SYS_SYSCFG_7_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}