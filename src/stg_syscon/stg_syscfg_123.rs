#[doc = "Register `stg_syscfg_123` reader"]
pub type R = crate::R<STG_SYSCFG_123_SPEC>;
#[doc = "Register `stg_syscfg_123` writer"]
pub type W = crate::W<STG_SYSCFG_123_SPEC>;
#[doc = "Field `u0_pcie_test_sel` reader - u0_pcie_test_sel"]
pub type U0_PCIE_TEST_SEL_R = crate::FieldReader;
#[doc = "Field `u0_pcie_test_sel` writer - u0_pcie_test_sel"]
pub type U0_PCIE_TEST_SEL_W<'a, REG> = crate::FieldWriter<'a, REG, 4>;
#[doc = "Field `u0_pcie_tl_clock_freq` reader - u0_pcie_tl_clock_freq"]
pub type U0_PCIE_TL_CLOCK_FREQ_R = crate::FieldReader<u32>;
#[doc = "Field `u0_pcie_tl_clock_freq` writer - u0_pcie_tl_clock_freq"]
pub type U0_PCIE_TL_CLOCK_FREQ_W<'a, REG> = crate::FieldWriter<'a, REG, 22, u32>;
impl R {
    #[doc = "Bits 0:3 - u0_pcie_test_sel"]
    #[inline(always)]
    pub fn u0_pcie_test_sel(&self) -> U0_PCIE_TEST_SEL_R {
        U0_PCIE_TEST_SEL_R::new((self.bits & 0x0f) as u8)
    }
    #[doc = "Bits 4:25 - u0_pcie_tl_clock_freq"]
    #[inline(always)]
    pub fn u0_pcie_tl_clock_freq(&self) -> U0_PCIE_TL_CLOCK_FREQ_R {
        U0_PCIE_TL_CLOCK_FREQ_R::new((self.bits >> 4) & 0x003f_ffff)
    }
}
impl W {
    #[doc = "Bits 0:3 - u0_pcie_test_sel"]
    #[inline(always)]
    #[must_use]
    pub fn u0_pcie_test_sel(&mut self) -> U0_PCIE_TEST_SEL_W<STG_SYSCFG_123_SPEC> {
        U0_PCIE_TEST_SEL_W::new(self, 0)
    }
    #[doc = "Bits 4:25 - u0_pcie_tl_clock_freq"]
    #[inline(always)]
    #[must_use]
    pub fn u0_pcie_tl_clock_freq(&mut self) -> U0_PCIE_TL_CLOCK_FREQ_W<STG_SYSCFG_123_SPEC> {
        U0_PCIE_TL_CLOCK_FREQ_W::new(self, 4)
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
#[doc = "STG SYSCONSAIF SYSCFG 492\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`stg_syscfg_123::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`stg_syscfg_123::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct STG_SYSCFG_123_SPEC;
impl crate::RegisterSpec for STG_SYSCFG_123_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`stg_syscfg_123::R`](R) reader structure"]
impl crate::Readable for STG_SYSCFG_123_SPEC {}
#[doc = "`write(|w| ..)` method takes [`stg_syscfg_123::W`](W) writer structure"]
impl crate::Writable for STG_SYSCFG_123_SPEC {
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets stg_syscfg_123 to value 0"]
impl crate::Resettable for STG_SYSCFG_123_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
