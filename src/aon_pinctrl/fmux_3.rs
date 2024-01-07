#[doc = "Register `fmux_3` reader"]
pub type R = crate::R<FMUX_3_SPEC>;
#[doc = "Register `fmux_3` writer"]
pub type W = crate::W<FMUX_3_SPEC>;
#[doc = "Field `gpen_0` reader - Enable GPIO IRQ function."]
pub type GPEN_0_R = crate::BitReader;
#[doc = "Field `gpen_0` writer - Enable GPIO IRQ function."]
pub type GPEN_0_W<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bit 0 - Enable GPIO IRQ function."]
    #[inline(always)]
    pub fn gpen_0(&self) -> GPEN_0_R {
        GPEN_0_R::new((self.bits & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - Enable GPIO IRQ function."]
    #[inline(always)]
    #[must_use]
    pub fn gpen_0(&mut self) -> GPEN_0_W<FMUX_3_SPEC> {
        GPEN_0_W::new(self, 0)
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
#[doc = "AON IOMUX CFG SAIF SYSCFG FMUX 12\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`fmux_3::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`fmux_3::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct FMUX_3_SPEC;
impl crate::RegisterSpec for FMUX_3_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`fmux_3::R`](R) reader structure"]
impl crate::Readable for FMUX_3_SPEC {}
#[doc = "`write(|w| ..)` method takes [`fmux_3::W`](W) writer structure"]
impl crate::Writable for FMUX_3_SPEC {
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets fmux_3 to value 0"]
impl crate::Resettable for FMUX_3_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
