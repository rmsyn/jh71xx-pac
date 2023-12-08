#[doc = "Register `sys_syscfg_9` reader"]
pub type R = crate::R<SYS_SYSCFG_9_SPEC>;
#[doc = "Register `sys_syscfg_9` writer"]
pub type W = crate::W<SYS_SYSCFG_9_SPEC>;
#[doc = "Field `pll0_prediv` reader - pll0_prediv"]
pub type PLL0_PREDIV_R = crate::FieldReader;
#[doc = "Field `pll0_prediv` writer - pll0_prediv"]
pub type PLL0_PREDIV_W<'a, REG> = crate::FieldWriter<'a, REG, 6>;
#[doc = "Field `pll0_testen` reader - pll0_testen"]
pub type PLL0_TESTEN_R = crate::BitReader;
#[doc = "Field `pll0_testen` writer - pll0_testen"]
pub type PLL0_TESTEN_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `pll0_testsel` reader - pll0_testsel"]
pub type PLL0_TESTSEL_R = crate::FieldReader;
#[doc = "Field `pll0_testsel` writer - pll0_testsel"]
pub type PLL0_TESTSEL_W<'a, REG> = crate::FieldWriter<'a, REG, 2>;
#[doc = "Field `pll1_cpi_bias` reader - pll1_cpi_bias"]
pub type PLL1_CPI_BIAS_R = crate::FieldReader;
#[doc = "Field `pll1_cpi_bias` writer - pll1_cpi_bias"]
pub type PLL1_CPI_BIAS_W<'a, REG> = crate::FieldWriter<'a, REG, 3>;
#[doc = "Field `pll1_cpp_bias` reader - pll1_cpp_bias"]
pub type PLL1_CPP_BIAS_R = crate::FieldReader;
#[doc = "Field `pll1_cpp_bias` writer - pll1_cpp_bias"]
pub type PLL1_CPP_BIAS_W<'a, REG> = crate::FieldWriter<'a, REG, 3>;
#[doc = "Field `pll1_dacpd` reader - pll1_dacpd"]
pub type PLL1_DACPD_R = crate::BitReader;
#[doc = "Field `pll1_dacpd` writer - pll1_dacpd"]
pub type PLL1_DACPD_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `pll1_dsmpd` reader - pll1_dsmpd"]
pub type PLL1_DSMPD_R = crate::BitReader;
#[doc = "Field `pll1_dsmpd` writer - pll1_dsmpd"]
pub type PLL1_DSMPD_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `pll1_fbdiv` reader - pll1_fbdiv"]
pub type PLL1_FBDIV_R = crate::FieldReader<u16>;
#[doc = "Field `pll1_fbdiv` writer - pll1_fbdiv"]
pub type PLL1_FBDIV_W<'a, REG> = crate::FieldWriter<'a, REG, 12, u16>;
impl R {
    #[doc = "Bits 0:5 - pll0_prediv"]
    #[inline(always)]
    pub fn pll0_prediv(&self) -> PLL0_PREDIV_R {
        PLL0_PREDIV_R::new((self.bits & 0x3f) as u8)
    }
    #[doc = "Bit 6 - pll0_testen"]
    #[inline(always)]
    pub fn pll0_testen(&self) -> PLL0_TESTEN_R {
        PLL0_TESTEN_R::new(((self.bits >> 6) & 1) != 0)
    }
    #[doc = "Bits 7:8 - pll0_testsel"]
    #[inline(always)]
    pub fn pll0_testsel(&self) -> PLL0_TESTSEL_R {
        PLL0_TESTSEL_R::new(((self.bits >> 7) & 3) as u8)
    }
    #[doc = "Bits 9:11 - pll1_cpi_bias"]
    #[inline(always)]
    pub fn pll1_cpi_bias(&self) -> PLL1_CPI_BIAS_R {
        PLL1_CPI_BIAS_R::new(((self.bits >> 9) & 7) as u8)
    }
    #[doc = "Bits 12:14 - pll1_cpp_bias"]
    #[inline(always)]
    pub fn pll1_cpp_bias(&self) -> PLL1_CPP_BIAS_R {
        PLL1_CPP_BIAS_R::new(((self.bits >> 12) & 7) as u8)
    }
    #[doc = "Bit 15 - pll1_dacpd"]
    #[inline(always)]
    pub fn pll1_dacpd(&self) -> PLL1_DACPD_R {
        PLL1_DACPD_R::new(((self.bits >> 15) & 1) != 0)
    }
    #[doc = "Bit 16 - pll1_dsmpd"]
    #[inline(always)]
    pub fn pll1_dsmpd(&self) -> PLL1_DSMPD_R {
        PLL1_DSMPD_R::new(((self.bits >> 16) & 1) != 0)
    }
    #[doc = "Bits 17:28 - pll1_fbdiv"]
    #[inline(always)]
    pub fn pll1_fbdiv(&self) -> PLL1_FBDIV_R {
        PLL1_FBDIV_R::new(((self.bits >> 17) & 0x0fff) as u16)
    }
}
impl W {
    #[doc = "Bits 0:5 - pll0_prediv"]
    #[inline(always)]
    #[must_use]
    pub fn pll0_prediv(&mut self) -> PLL0_PREDIV_W<SYS_SYSCFG_9_SPEC> {
        PLL0_PREDIV_W::new(self, 0)
    }
    #[doc = "Bit 6 - pll0_testen"]
    #[inline(always)]
    #[must_use]
    pub fn pll0_testen(&mut self) -> PLL0_TESTEN_W<SYS_SYSCFG_9_SPEC> {
        PLL0_TESTEN_W::new(self, 6)
    }
    #[doc = "Bits 7:8 - pll0_testsel"]
    #[inline(always)]
    #[must_use]
    pub fn pll0_testsel(&mut self) -> PLL0_TESTSEL_W<SYS_SYSCFG_9_SPEC> {
        PLL0_TESTSEL_W::new(self, 7)
    }
    #[doc = "Bits 9:11 - pll1_cpi_bias"]
    #[inline(always)]
    #[must_use]
    pub fn pll1_cpi_bias(&mut self) -> PLL1_CPI_BIAS_W<SYS_SYSCFG_9_SPEC> {
        PLL1_CPI_BIAS_W::new(self, 9)
    }
    #[doc = "Bits 12:14 - pll1_cpp_bias"]
    #[inline(always)]
    #[must_use]
    pub fn pll1_cpp_bias(&mut self) -> PLL1_CPP_BIAS_W<SYS_SYSCFG_9_SPEC> {
        PLL1_CPP_BIAS_W::new(self, 12)
    }
    #[doc = "Bit 15 - pll1_dacpd"]
    #[inline(always)]
    #[must_use]
    pub fn pll1_dacpd(&mut self) -> PLL1_DACPD_W<SYS_SYSCFG_9_SPEC> {
        PLL1_DACPD_W::new(self, 15)
    }
    #[doc = "Bit 16 - pll1_dsmpd"]
    #[inline(always)]
    #[must_use]
    pub fn pll1_dsmpd(&mut self) -> PLL1_DSMPD_W<SYS_SYSCFG_9_SPEC> {
        PLL1_DSMPD_W::new(self, 16)
    }
    #[doc = "Bits 17:28 - pll1_fbdiv"]
    #[inline(always)]
    #[must_use]
    pub fn pll1_fbdiv(&mut self) -> PLL1_FBDIV_W<SYS_SYSCFG_9_SPEC> {
        PLL1_FBDIV_W::new(self, 17)
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
#[doc = "SYS SYSCONSAIF SYSCFG 36\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`sys_syscfg_9::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`sys_syscfg_9::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct SYS_SYSCFG_9_SPEC;
impl crate::RegisterSpec for SYS_SYSCFG_9_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`sys_syscfg_9::R`](R) reader structure"]
impl crate::Readable for SYS_SYSCFG_9_SPEC {}
#[doc = "`write(|w| ..)` method takes [`sys_syscfg_9::W`](W) writer structure"]
impl crate::Writable for SYS_SYSCFG_9_SPEC {
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets sys_syscfg_9 to value 0"]
impl crate::Resettable for SYS_SYSCFG_9_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
