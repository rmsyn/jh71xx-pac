#[doc = "Register `comp_param_1` reader"]
pub type R = crate::R<COMP_PARAM_1_SPEC>;
#[doc = "Register `comp_param_1` writer"]
pub type W = crate::W<COMP_PARAM_1_SPEC>;
#[doc = "Field `speed` reader - Speed mask - 01: Standard, 10: Full, 11: High"]
pub type SPEED_R = crate::FieldReader;
impl R {
    #[doc = "Bits 2:3 - Speed mask - 01: Standard, 10: Full, 11: High"]
    #[inline(always)]
    pub fn speed(&self) -> SPEED_R {
        SPEED_R::new(((self.bits >> 2) & 3) as u8)
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
#[doc = "DesignWare I2C Compatibility Parameter 1\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`comp_param_1::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`comp_param_1::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct COMP_PARAM_1_SPEC;
impl crate::RegisterSpec for COMP_PARAM_1_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`comp_param_1::R`](R) reader structure"]
impl crate::Readable for COMP_PARAM_1_SPEC {}
#[doc = "`write(|w| ..)` method takes [`comp_param_1::W`](W) writer structure"]
impl crate::Writable for COMP_PARAM_1_SPEC {
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets comp_param_1 to value 0"]
impl crate::Resettable for COMP_PARAM_1_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
