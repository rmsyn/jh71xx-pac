#[doc = "Register `sys_sysconsaif_syscfg44` reader"]
pub type R = crate::R<SYS_SYSCONSAIF_SYSCFG44_SPEC>;
#[doc = "Register `sys_sysconsaif_syscfg44` writer"]
pub type W = crate::W<SYS_SYSCONSAIF_SYSCFG44_SPEC>;
#[doc = "Field `u0_pll_wrap_pll1_prediv` reader - u0_pll_wrap_pll1_prediv"]
pub type U0_PLL_WRAP_PLL1_PREDIV_R = crate::FieldReader;
#[doc = "Field `u0_pll_wrap_pll1_prediv` writer - u0_pll_wrap_pll1_prediv"]
pub type U0_PLL_WRAP_PLL1_PREDIV_W<'a, REG> = crate::FieldWriter<'a, REG, 6>;
#[doc = "Field `u0_pll_wrap_pll1_testen` reader - u0_pll_wrap_pll1_testen"]
pub type U0_PLL_WRAP_PLL1_TESTEN_R = crate::BitReader;
#[doc = "Field `u0_pll_wrap_pll1_testen` writer - u0_pll_wrap_pll1_testen"]
pub type U0_PLL_WRAP_PLL1_TESTEN_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `u0_pll_wrap_pll1_testsel` reader - u0_pll_wrap_pll1_testsel"]
pub type U0_PLL_WRAP_PLL1_TESTSEL_R = crate::FieldReader;
#[doc = "Field `u0_pll_wrap_pll1_testsel` writer - u0_pll_wrap_pll1_testsel"]
pub type U0_PLL_WRAP_PLL1_TESTSEL_W<'a, REG> = crate::FieldWriter<'a, REG, 2>;
#[doc = "Field `u0_pll_wrap_pll2_cpi_bias` reader - u0_pll_wrap_pll2_cpi_bias"]
pub type U0_PLL_WRAP_PLL2_CPI_BIAS_R = crate::FieldReader;
#[doc = "Field `u0_pll_wrap_pll2_cpi_bias` writer - u0_pll_wrap_pll2_cpi_bias"]
pub type U0_PLL_WRAP_PLL2_CPI_BIAS_W<'a, REG> = crate::FieldWriter<'a, REG, 3>;
#[doc = "Field `u0_pll_wrap_pll2_cpp_bias` reader - u0_pll_wrap_pll2_cpp_bias"]
pub type U0_PLL_WRAP_PLL2_CPP_BIAS_R = crate::FieldReader;
#[doc = "Field `u0_pll_wrap_pll2_cpp_bias` writer - u0_pll_wrap_pll2_cpp_bias"]
pub type U0_PLL_WRAP_PLL2_CPP_BIAS_W<'a, REG> = crate::FieldWriter<'a, REG, 3>;
#[doc = "Field `u0_pll_wrap_pll2_dacpd` reader - u0_pll_wrap_pll2_dacpd"]
pub type U0_PLL_WRAP_PLL2_DACPD_R = crate::BitReader;
#[doc = "Field `u0_pll_wrap_pll2_dacpd` writer - u0_pll_wrap_pll2_dacpd"]
pub type U0_PLL_WRAP_PLL2_DACPD_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `u0_pll_wrap_pll2_dsmpd` reader - u0_pll_wrap_pll2_dsmpd"]
pub type U0_PLL_WRAP_PLL2_DSMPD_R = crate::BitReader;
#[doc = "Field `u0_pll_wrap_pll2_dsmpd` writer - u0_pll_wrap_pll2_dsmpd"]
pub type U0_PLL_WRAP_PLL2_DSMPD_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `u0_pll_wrap_pll2_fbdiv` reader - u0_pll_wrap_pll2_fbdiv"]
pub type U0_PLL_WRAP_PLL2_FBDIV_R = crate::FieldReader<u16>;
#[doc = "Field `u0_pll_wrap_pll2_fbdiv` writer - u0_pll_wrap_pll2_fbdiv"]
pub type U0_PLL_WRAP_PLL2_FBDIV_W<'a, REG> = crate::FieldWriter<'a, REG, 12, u16>;
impl R {
    #[doc = "Bits 0:5 - u0_pll_wrap_pll1_prediv"]
    #[inline(always)]
    pub fn u0_pll_wrap_pll1_prediv(&self) -> U0_PLL_WRAP_PLL1_PREDIV_R {
        U0_PLL_WRAP_PLL1_PREDIV_R::new((self.bits & 0x3f) as u8)
    }
    #[doc = "Bit 6 - u0_pll_wrap_pll1_testen"]
    #[inline(always)]
    pub fn u0_pll_wrap_pll1_testen(&self) -> U0_PLL_WRAP_PLL1_TESTEN_R {
        U0_PLL_WRAP_PLL1_TESTEN_R::new(((self.bits >> 6) & 1) != 0)
    }
    #[doc = "Bits 7:8 - u0_pll_wrap_pll1_testsel"]
    #[inline(always)]
    pub fn u0_pll_wrap_pll1_testsel(&self) -> U0_PLL_WRAP_PLL1_TESTSEL_R {
        U0_PLL_WRAP_PLL1_TESTSEL_R::new(((self.bits >> 7) & 3) as u8)
    }
    #[doc = "Bits 9:11 - u0_pll_wrap_pll2_cpi_bias"]
    #[inline(always)]
    pub fn u0_pll_wrap_pll2_cpi_bias(&self) -> U0_PLL_WRAP_PLL2_CPI_BIAS_R {
        U0_PLL_WRAP_PLL2_CPI_BIAS_R::new(((self.bits >> 9) & 7) as u8)
    }
    #[doc = "Bits 12:14 - u0_pll_wrap_pll2_cpp_bias"]
    #[inline(always)]
    pub fn u0_pll_wrap_pll2_cpp_bias(&self) -> U0_PLL_WRAP_PLL2_CPP_BIAS_R {
        U0_PLL_WRAP_PLL2_CPP_BIAS_R::new(((self.bits >> 12) & 7) as u8)
    }
    #[doc = "Bit 15 - u0_pll_wrap_pll2_dacpd"]
    #[inline(always)]
    pub fn u0_pll_wrap_pll2_dacpd(&self) -> U0_PLL_WRAP_PLL2_DACPD_R {
        U0_PLL_WRAP_PLL2_DACPD_R::new(((self.bits >> 15) & 1) != 0)
    }
    #[doc = "Bit 16 - u0_pll_wrap_pll2_dsmpd"]
    #[inline(always)]
    pub fn u0_pll_wrap_pll2_dsmpd(&self) -> U0_PLL_WRAP_PLL2_DSMPD_R {
        U0_PLL_WRAP_PLL2_DSMPD_R::new(((self.bits >> 16) & 1) != 0)
    }
    #[doc = "Bits 17:28 - u0_pll_wrap_pll2_fbdiv"]
    #[inline(always)]
    pub fn u0_pll_wrap_pll2_fbdiv(&self) -> U0_PLL_WRAP_PLL2_FBDIV_R {
        U0_PLL_WRAP_PLL2_FBDIV_R::new(((self.bits >> 17) & 0x0fff) as u16)
    }
}
impl W {
    #[doc = "Bits 0:5 - u0_pll_wrap_pll1_prediv"]
    #[inline(always)]
    #[must_use]
    pub fn u0_pll_wrap_pll1_prediv(
        &mut self,
    ) -> U0_PLL_WRAP_PLL1_PREDIV_W<SYS_SYSCONSAIF_SYSCFG44_SPEC> {
        U0_PLL_WRAP_PLL1_PREDIV_W::new(self, 0)
    }
    #[doc = "Bit 6 - u0_pll_wrap_pll1_testen"]
    #[inline(always)]
    #[must_use]
    pub fn u0_pll_wrap_pll1_testen(
        &mut self,
    ) -> U0_PLL_WRAP_PLL1_TESTEN_W<SYS_SYSCONSAIF_SYSCFG44_SPEC> {
        U0_PLL_WRAP_PLL1_TESTEN_W::new(self, 6)
    }
    #[doc = "Bits 7:8 - u0_pll_wrap_pll1_testsel"]
    #[inline(always)]
    #[must_use]
    pub fn u0_pll_wrap_pll1_testsel(
        &mut self,
    ) -> U0_PLL_WRAP_PLL1_TESTSEL_W<SYS_SYSCONSAIF_SYSCFG44_SPEC> {
        U0_PLL_WRAP_PLL1_TESTSEL_W::new(self, 7)
    }
    #[doc = "Bits 9:11 - u0_pll_wrap_pll2_cpi_bias"]
    #[inline(always)]
    #[must_use]
    pub fn u0_pll_wrap_pll2_cpi_bias(
        &mut self,
    ) -> U0_PLL_WRAP_PLL2_CPI_BIAS_W<SYS_SYSCONSAIF_SYSCFG44_SPEC> {
        U0_PLL_WRAP_PLL2_CPI_BIAS_W::new(self, 9)
    }
    #[doc = "Bits 12:14 - u0_pll_wrap_pll2_cpp_bias"]
    #[inline(always)]
    #[must_use]
    pub fn u0_pll_wrap_pll2_cpp_bias(
        &mut self,
    ) -> U0_PLL_WRAP_PLL2_CPP_BIAS_W<SYS_SYSCONSAIF_SYSCFG44_SPEC> {
        U0_PLL_WRAP_PLL2_CPP_BIAS_W::new(self, 12)
    }
    #[doc = "Bit 15 - u0_pll_wrap_pll2_dacpd"]
    #[inline(always)]
    #[must_use]
    pub fn u0_pll_wrap_pll2_dacpd(
        &mut self,
    ) -> U0_PLL_WRAP_PLL2_DACPD_W<SYS_SYSCONSAIF_SYSCFG44_SPEC> {
        U0_PLL_WRAP_PLL2_DACPD_W::new(self, 15)
    }
    #[doc = "Bit 16 - u0_pll_wrap_pll2_dsmpd"]
    #[inline(always)]
    #[must_use]
    pub fn u0_pll_wrap_pll2_dsmpd(
        &mut self,
    ) -> U0_PLL_WRAP_PLL2_DSMPD_W<SYS_SYSCONSAIF_SYSCFG44_SPEC> {
        U0_PLL_WRAP_PLL2_DSMPD_W::new(self, 16)
    }
    #[doc = "Bits 17:28 - u0_pll_wrap_pll2_fbdiv"]
    #[inline(always)]
    #[must_use]
    pub fn u0_pll_wrap_pll2_fbdiv(
        &mut self,
    ) -> U0_PLL_WRAP_PLL2_FBDIV_W<SYS_SYSCONSAIF_SYSCFG44_SPEC> {
        U0_PLL_WRAP_PLL2_FBDIV_W::new(self, 17)
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
#[doc = "SYS SYSCONSAIF SYSCFG 44\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`sys_sysconsaif_syscfg44::R`](R).  You can [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`sys_sysconsaif_syscfg44::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct SYS_SYSCONSAIF_SYSCFG44_SPEC;
impl crate::RegisterSpec for SYS_SYSCONSAIF_SYSCFG44_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`sys_sysconsaif_syscfg44::R`](R) reader structure"]
impl crate::Readable for SYS_SYSCONSAIF_SYSCFG44_SPEC {}
#[doc = "`write(|w| ..)` method takes [`sys_sysconsaif_syscfg44::W`](W) writer structure"]
impl crate::Writable for SYS_SYSCONSAIF_SYSCFG44_SPEC {
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
