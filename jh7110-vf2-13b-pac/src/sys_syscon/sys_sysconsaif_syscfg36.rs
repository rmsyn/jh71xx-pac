#[doc = "Register `sys_sysconsaif_syscfg36` reader"]
pub type R = crate::R<SYS_SYSCONSAIF_SYSCFG36_SPEC>;
#[doc = "Register `sys_sysconsaif_syscfg36` writer"]
pub type W = crate::W<SYS_SYSCONSAIF_SYSCFG36_SPEC>;
#[doc = "Field `u0_pll_wrap_pll0_prediv` reader - u0_pll_wrap_pll0_prediv"]
pub type U0_PLL_WRAP_PLL0_PREDIV_R = crate::FieldReader;
#[doc = "Field `u0_pll_wrap_pll0_prediv` writer - u0_pll_wrap_pll0_prediv"]
pub type U0_PLL_WRAP_PLL0_PREDIV_W<'a, REG, const O: u8> = crate::FieldWriter<'a, REG, 6, O>;
#[doc = "Field `u0_pll_wrap_pll0_testen` reader - u0_pll_wrap_pll0_testen"]
pub type U0_PLL_WRAP_PLL0_TESTEN_R = crate::BitReader;
#[doc = "Field `u0_pll_wrap_pll0_testen` writer - u0_pll_wrap_pll0_testen"]
pub type U0_PLL_WRAP_PLL0_TESTEN_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O>;
#[doc = "Field `u0_pll_wrap_pll0_testsel` reader - u0_pll_wrap_pll0_testsel"]
pub type U0_PLL_WRAP_PLL0_TESTSEL_R = crate::FieldReader;
#[doc = "Field `u0_pll_wrap_pll0_testsel` writer - u0_pll_wrap_pll0_testsel"]
pub type U0_PLL_WRAP_PLL0_TESTSEL_W<'a, REG, const O: u8> = crate::FieldWriter<'a, REG, 2, O>;
#[doc = "Field `u0_pll_wrap_pll1_cpi_bias` reader - u0_pll_wrap_pll1_cpi_bias"]
pub type U0_PLL_WRAP_PLL1_CPI_BIAS_R = crate::FieldReader;
#[doc = "Field `u0_pll_wrap_pll1_cpi_bias` writer - u0_pll_wrap_pll1_cpi_bias"]
pub type U0_PLL_WRAP_PLL1_CPI_BIAS_W<'a, REG, const O: u8> = crate::FieldWriter<'a, REG, 3, O>;
#[doc = "Field `u0_pll_wrap_pll1_cpp_bias` reader - u0_pll_wrap_pll1_cpp_bias"]
pub type U0_PLL_WRAP_PLL1_CPP_BIAS_R = crate::FieldReader;
#[doc = "Field `u0_pll_wrap_pll1_cpp_bias` writer - u0_pll_wrap_pll1_cpp_bias"]
pub type U0_PLL_WRAP_PLL1_CPP_BIAS_W<'a, REG, const O: u8> = crate::FieldWriter<'a, REG, 3, O>;
#[doc = "Field `u0_pll_wrap_pll1_dacpd` reader - u0_pll_wrap_pll1_dacpd"]
pub type U0_PLL_WRAP_PLL1_DACPD_R = crate::BitReader;
#[doc = "Field `u0_pll_wrap_pll1_dacpd` writer - u0_pll_wrap_pll1_dacpd"]
pub type U0_PLL_WRAP_PLL1_DACPD_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O>;
#[doc = "Field `u0_pll_wrap_pll1_dsmpd` reader - u0_pll_wrap_pll1_dsmpd"]
pub type U0_PLL_WRAP_PLL1_DSMPD_R = crate::BitReader;
#[doc = "Field `u0_pll_wrap_pll1_dsmpd` writer - u0_pll_wrap_pll1_dsmpd"]
pub type U0_PLL_WRAP_PLL1_DSMPD_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O>;
#[doc = "Field `u0_pll_wrap_pll1_fbdiv` reader - u0_pll_wrap_pll1_fbdiv"]
pub type U0_PLL_WRAP_PLL1_FBDIV_R = crate::FieldReader<u16>;
#[doc = "Field `u0_pll_wrap_pll1_fbdiv` writer - u0_pll_wrap_pll1_fbdiv"]
pub type U0_PLL_WRAP_PLL1_FBDIV_W<'a, REG, const O: u8> = crate::FieldWriter<'a, REG, 12, O, u16>;
impl R {
    #[doc = "Bits 0:5 - u0_pll_wrap_pll0_prediv"]
    #[inline(always)]
    pub fn u0_pll_wrap_pll0_prediv(&self) -> U0_PLL_WRAP_PLL0_PREDIV_R {
        U0_PLL_WRAP_PLL0_PREDIV_R::new((self.bits & 0x3f) as u8)
    }
    #[doc = "Bit 6 - u0_pll_wrap_pll0_testen"]
    #[inline(always)]
    pub fn u0_pll_wrap_pll0_testen(&self) -> U0_PLL_WRAP_PLL0_TESTEN_R {
        U0_PLL_WRAP_PLL0_TESTEN_R::new(((self.bits >> 6) & 1) != 0)
    }
    #[doc = "Bits 7:8 - u0_pll_wrap_pll0_testsel"]
    #[inline(always)]
    pub fn u0_pll_wrap_pll0_testsel(&self) -> U0_PLL_WRAP_PLL0_TESTSEL_R {
        U0_PLL_WRAP_PLL0_TESTSEL_R::new(((self.bits >> 7) & 3) as u8)
    }
    #[doc = "Bits 9:11 - u0_pll_wrap_pll1_cpi_bias"]
    #[inline(always)]
    pub fn u0_pll_wrap_pll1_cpi_bias(&self) -> U0_PLL_WRAP_PLL1_CPI_BIAS_R {
        U0_PLL_WRAP_PLL1_CPI_BIAS_R::new(((self.bits >> 9) & 7) as u8)
    }
    #[doc = "Bits 12:14 - u0_pll_wrap_pll1_cpp_bias"]
    #[inline(always)]
    pub fn u0_pll_wrap_pll1_cpp_bias(&self) -> U0_PLL_WRAP_PLL1_CPP_BIAS_R {
        U0_PLL_WRAP_PLL1_CPP_BIAS_R::new(((self.bits >> 12) & 7) as u8)
    }
    #[doc = "Bit 15 - u0_pll_wrap_pll1_dacpd"]
    #[inline(always)]
    pub fn u0_pll_wrap_pll1_dacpd(&self) -> U0_PLL_WRAP_PLL1_DACPD_R {
        U0_PLL_WRAP_PLL1_DACPD_R::new(((self.bits >> 15) & 1) != 0)
    }
    #[doc = "Bit 16 - u0_pll_wrap_pll1_dsmpd"]
    #[inline(always)]
    pub fn u0_pll_wrap_pll1_dsmpd(&self) -> U0_PLL_WRAP_PLL1_DSMPD_R {
        U0_PLL_WRAP_PLL1_DSMPD_R::new(((self.bits >> 16) & 1) != 0)
    }
    #[doc = "Bits 17:28 - u0_pll_wrap_pll1_fbdiv"]
    #[inline(always)]
    pub fn u0_pll_wrap_pll1_fbdiv(&self) -> U0_PLL_WRAP_PLL1_FBDIV_R {
        U0_PLL_WRAP_PLL1_FBDIV_R::new(((self.bits >> 17) & 0x0fff) as u16)
    }
}
impl W {
    #[doc = "Bits 0:5 - u0_pll_wrap_pll0_prediv"]
    #[inline(always)]
    #[must_use]
    pub fn u0_pll_wrap_pll0_prediv(
        &mut self,
    ) -> U0_PLL_WRAP_PLL0_PREDIV_W<SYS_SYSCONSAIF_SYSCFG36_SPEC, 0> {
        U0_PLL_WRAP_PLL0_PREDIV_W::new(self)
    }
    #[doc = "Bit 6 - u0_pll_wrap_pll0_testen"]
    #[inline(always)]
    #[must_use]
    pub fn u0_pll_wrap_pll0_testen(
        &mut self,
    ) -> U0_PLL_WRAP_PLL0_TESTEN_W<SYS_SYSCONSAIF_SYSCFG36_SPEC, 6> {
        U0_PLL_WRAP_PLL0_TESTEN_W::new(self)
    }
    #[doc = "Bits 7:8 - u0_pll_wrap_pll0_testsel"]
    #[inline(always)]
    #[must_use]
    pub fn u0_pll_wrap_pll0_testsel(
        &mut self,
    ) -> U0_PLL_WRAP_PLL0_TESTSEL_W<SYS_SYSCONSAIF_SYSCFG36_SPEC, 7> {
        U0_PLL_WRAP_PLL0_TESTSEL_W::new(self)
    }
    #[doc = "Bits 9:11 - u0_pll_wrap_pll1_cpi_bias"]
    #[inline(always)]
    #[must_use]
    pub fn u0_pll_wrap_pll1_cpi_bias(
        &mut self,
    ) -> U0_PLL_WRAP_PLL1_CPI_BIAS_W<SYS_SYSCONSAIF_SYSCFG36_SPEC, 9> {
        U0_PLL_WRAP_PLL1_CPI_BIAS_W::new(self)
    }
    #[doc = "Bits 12:14 - u0_pll_wrap_pll1_cpp_bias"]
    #[inline(always)]
    #[must_use]
    pub fn u0_pll_wrap_pll1_cpp_bias(
        &mut self,
    ) -> U0_PLL_WRAP_PLL1_CPP_BIAS_W<SYS_SYSCONSAIF_SYSCFG36_SPEC, 12> {
        U0_PLL_WRAP_PLL1_CPP_BIAS_W::new(self)
    }
    #[doc = "Bit 15 - u0_pll_wrap_pll1_dacpd"]
    #[inline(always)]
    #[must_use]
    pub fn u0_pll_wrap_pll1_dacpd(
        &mut self,
    ) -> U0_PLL_WRAP_PLL1_DACPD_W<SYS_SYSCONSAIF_SYSCFG36_SPEC, 15> {
        U0_PLL_WRAP_PLL1_DACPD_W::new(self)
    }
    #[doc = "Bit 16 - u0_pll_wrap_pll1_dsmpd"]
    #[inline(always)]
    #[must_use]
    pub fn u0_pll_wrap_pll1_dsmpd(
        &mut self,
    ) -> U0_PLL_WRAP_PLL1_DSMPD_W<SYS_SYSCONSAIF_SYSCFG36_SPEC, 16> {
        U0_PLL_WRAP_PLL1_DSMPD_W::new(self)
    }
    #[doc = "Bits 17:28 - u0_pll_wrap_pll1_fbdiv"]
    #[inline(always)]
    #[must_use]
    pub fn u0_pll_wrap_pll1_fbdiv(
        &mut self,
    ) -> U0_PLL_WRAP_PLL1_FBDIV_W<SYS_SYSCONSAIF_SYSCFG36_SPEC, 17> {
        U0_PLL_WRAP_PLL1_FBDIV_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.bits = bits;
        self
    }
}
#[doc = "SYS SYSCONSAIF SYSCFG 36\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`sys_sysconsaif_syscfg36::R`](R).  You can [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`sys_sysconsaif_syscfg36::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct SYS_SYSCONSAIF_SYSCFG36_SPEC;
impl crate::RegisterSpec for SYS_SYSCONSAIF_SYSCFG36_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`sys_sysconsaif_syscfg36::R`](R) reader structure"]
impl crate::Readable for SYS_SYSCONSAIF_SYSCFG36_SPEC {}
#[doc = "`write(|w| ..)` method takes [`sys_sysconsaif_syscfg36::W`](W) writer structure"]
impl crate::Writable for SYS_SYSCONSAIF_SYSCFG36_SPEC {
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
