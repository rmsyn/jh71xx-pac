#[doc = "Register `sys_syscfg_11` reader"]
pub type R = crate::R<SYS_SYSCFG_11_SPEC>;
#[doc = "Register `sys_syscfg_11` writer"]
pub type W = crate::W<SYS_SYSCFG_11_SPEC>;
#[doc = "Field `pll1_prediv` reader - pll1_prediv"]
pub type PLL1_PREDIV_R = crate::FieldReader;
#[doc = "Field `pll1_prediv` writer - pll1_prediv"]
pub type PLL1_PREDIV_W<'a, REG> = crate::FieldWriter<'a, REG, 6>;
#[doc = "Field `pll1_testen` reader - pll1_testen"]
pub type PLL1_TESTEN_R = crate::BitReader;
#[doc = "Field `pll1_testen` writer - pll1_testen"]
pub type PLL1_TESTEN_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `pll1_testsel` reader - pll1_testsel"]
pub type PLL1_TESTSEL_R = crate::FieldReader;
#[doc = "Field `pll1_testsel` writer - pll1_testsel"]
pub type PLL1_TESTSEL_W<'a, REG> = crate::FieldWriter<'a, REG, 2>;
#[doc = "Field `pll2_cpi_bias` reader - pll2_cpi_bias"]
pub type PLL2_CPI_BIAS_R = crate::FieldReader;
#[doc = "Field `pll2_cpi_bias` writer - pll2_cpi_bias"]
pub type PLL2_CPI_BIAS_W<'a, REG> = crate::FieldWriter<'a, REG, 3>;
#[doc = "Field `pll2_cpp_bias` reader - pll2_cpp_bias"]
pub type PLL2_CPP_BIAS_R = crate::FieldReader;
#[doc = "Field `pll2_cpp_bias` writer - pll2_cpp_bias"]
pub type PLL2_CPP_BIAS_W<'a, REG> = crate::FieldWriter<'a, REG, 3>;
#[doc = "Field `pll2_dacpd` reader - pll2_dacpd"]
pub type PLL2_DACPD_R = crate::BitReader;
#[doc = "Field `pll2_dacpd` writer - pll2_dacpd"]
pub type PLL2_DACPD_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `pll2_dsmpd` reader - pll2_dsmpd"]
pub type PLL2_DSMPD_R = crate::BitReader;
#[doc = "Field `pll2_dsmpd` writer - pll2_dsmpd"]
pub type PLL2_DSMPD_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `pll2_fbdiv` reader - pll2_fbdiv"]
pub type PLL2_FBDIV_R = crate::FieldReader<u16>;
#[doc = "Field `pll2_fbdiv` writer - pll2_fbdiv"]
pub type PLL2_FBDIV_W<'a, REG> = crate::FieldWriter<'a, REG, 12, u16>;
impl R {
    #[doc = "Bits 0:5 - pll1_prediv"]
    #[inline(always)]
    pub fn pll1_prediv(&self) -> PLL1_PREDIV_R {
        PLL1_PREDIV_R::new((self.bits & 0x3f) as u8)
    }
    #[doc = "Bit 6 - pll1_testen"]
    #[inline(always)]
    pub fn pll1_testen(&self) -> PLL1_TESTEN_R {
        PLL1_TESTEN_R::new(((self.bits >> 6) & 1) != 0)
    }
    #[doc = "Bits 7:8 - pll1_testsel"]
    #[inline(always)]
    pub fn pll1_testsel(&self) -> PLL1_TESTSEL_R {
        PLL1_TESTSEL_R::new(((self.bits >> 7) & 3) as u8)
    }
    #[doc = "Bits 9:11 - pll2_cpi_bias"]
    #[inline(always)]
    pub fn pll2_cpi_bias(&self) -> PLL2_CPI_BIAS_R {
        PLL2_CPI_BIAS_R::new(((self.bits >> 9) & 7) as u8)
    }
    #[doc = "Bits 12:14 - pll2_cpp_bias"]
    #[inline(always)]
    pub fn pll2_cpp_bias(&self) -> PLL2_CPP_BIAS_R {
        PLL2_CPP_BIAS_R::new(((self.bits >> 12) & 7) as u8)
    }
    #[doc = "Bit 15 - pll2_dacpd"]
    #[inline(always)]
    pub fn pll2_dacpd(&self) -> PLL2_DACPD_R {
        PLL2_DACPD_R::new(((self.bits >> 15) & 1) != 0)
    }
    #[doc = "Bit 16 - pll2_dsmpd"]
    #[inline(always)]
    pub fn pll2_dsmpd(&self) -> PLL2_DSMPD_R {
        PLL2_DSMPD_R::new(((self.bits >> 16) & 1) != 0)
    }
    #[doc = "Bits 17:28 - pll2_fbdiv"]
    #[inline(always)]
    pub fn pll2_fbdiv(&self) -> PLL2_FBDIV_R {
        PLL2_FBDIV_R::new(((self.bits >> 17) & 0x0fff) as u16)
    }
}
impl W {
    #[doc = "Bits 0:5 - pll1_prediv"]
    #[inline(always)]
    #[must_use]
    pub fn pll1_prediv(&mut self) -> PLL1_PREDIV_W<SYS_SYSCFG_11_SPEC> {
        PLL1_PREDIV_W::new(self, 0)
    }
    #[doc = "Bit 6 - pll1_testen"]
    #[inline(always)]
    #[must_use]
    pub fn pll1_testen(&mut self) -> PLL1_TESTEN_W<SYS_SYSCFG_11_SPEC> {
        PLL1_TESTEN_W::new(self, 6)
    }
    #[doc = "Bits 7:8 - pll1_testsel"]
    #[inline(always)]
    #[must_use]
    pub fn pll1_testsel(&mut self) -> PLL1_TESTSEL_W<SYS_SYSCFG_11_SPEC> {
        PLL1_TESTSEL_W::new(self, 7)
    }
    #[doc = "Bits 9:11 - pll2_cpi_bias"]
    #[inline(always)]
    #[must_use]
    pub fn pll2_cpi_bias(&mut self) -> PLL2_CPI_BIAS_W<SYS_SYSCFG_11_SPEC> {
        PLL2_CPI_BIAS_W::new(self, 9)
    }
    #[doc = "Bits 12:14 - pll2_cpp_bias"]
    #[inline(always)]
    #[must_use]
    pub fn pll2_cpp_bias(&mut self) -> PLL2_CPP_BIAS_W<SYS_SYSCFG_11_SPEC> {
        PLL2_CPP_BIAS_W::new(self, 12)
    }
    #[doc = "Bit 15 - pll2_dacpd"]
    #[inline(always)]
    #[must_use]
    pub fn pll2_dacpd(&mut self) -> PLL2_DACPD_W<SYS_SYSCFG_11_SPEC> {
        PLL2_DACPD_W::new(self, 15)
    }
    #[doc = "Bit 16 - pll2_dsmpd"]
    #[inline(always)]
    #[must_use]
    pub fn pll2_dsmpd(&mut self) -> PLL2_DSMPD_W<SYS_SYSCFG_11_SPEC> {
        PLL2_DSMPD_W::new(self, 16)
    }
    #[doc = "Bits 17:28 - pll2_fbdiv"]
    #[inline(always)]
    #[must_use]
    pub fn pll2_fbdiv(&mut self) -> PLL2_FBDIV_W<SYS_SYSCFG_11_SPEC> {
        PLL2_FBDIV_W::new(self, 17)
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
#[doc = "SYS SYSCONSAIF SYSCFG 44\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`sys_syscfg_11::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`sys_syscfg_11::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct SYS_SYSCFG_11_SPEC;
impl crate::RegisterSpec for SYS_SYSCFG_11_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`sys_syscfg_11::R`](R) reader structure"]
impl crate::Readable for SYS_SYSCFG_11_SPEC {}
#[doc = "`write(|w| ..)` method takes [`sys_syscfg_11::W`](W) writer structure"]
impl crate::Writable for SYS_SYSCFG_11_SPEC {
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets sys_syscfg_11 to value 0"]
impl crate::Resettable for SYS_SYSCFG_11_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
