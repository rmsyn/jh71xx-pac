#[doc = "Register `aon_syscfg_8` reader"]
pub type R = crate::R<AON_SYSCFG_8_SPEC>;
#[doc = "Register `aon_syscfg_8` writer"]
pub type W = crate::W<AON_SYSCFG_8_SPEC>;
#[doc = "Field `u0_otpc_fl_pll0_lock` reader - u0_otpc_fl_pll0_lock"]
pub type U0_OTPC_FL_PLL0_LOCK_R = crate::FieldReader<u32>;
impl R {
    #[doc = "Bits 0:31 - u0_otpc_fl_pll0_lock"]
    #[inline(always)]
    pub fn u0_otpc_fl_pll0_lock(&self) -> U0_OTPC_FL_PLL0_LOCK_R {
        U0_OTPC_FL_PLL0_LOCK_R::new(self.bits)
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
#[doc = "AON SYSCONSAIF SYSCFG 32\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`aon_syscfg_8::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`aon_syscfg_8::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct AON_SYSCFG_8_SPEC;
impl crate::RegisterSpec for AON_SYSCFG_8_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`aon_syscfg_8::R`](R) reader structure"]
impl crate::Readable for AON_SYSCFG_8_SPEC {}
#[doc = "`write(|w| ..)` method takes [`aon_syscfg_8::W`](W) writer structure"]
impl crate::Writable for AON_SYSCFG_8_SPEC {
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets aon_syscfg_8 to value 0"]
impl crate::Resettable for AON_SYSCFG_8_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
