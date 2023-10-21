#[doc = "Register `ss_scl_hcnt` reader"]
pub type R = crate::R<SS_SCL_HCNT_SPEC>;
#[doc = "Register `ss_scl_hcnt` writer"]
pub type W = crate::W<SS_SCL_HCNT_SPEC>;
#[doc = "Field `ss_scl_hcnt` reader - ss_scl_hcnt"]
pub type SS_SCL_HCNT_R = crate::FieldReader<u32>;
#[doc = "Field `ss_scl_hcnt` writer - ss_scl_hcnt"]
pub type SS_SCL_HCNT_W<'a, REG, const O: u8> = crate::FieldWriter<'a, REG, 32, O, u32>;
impl R {
    #[doc = "Bits 0:31 - ss_scl_hcnt"]
    #[inline(always)]
    pub fn ss_scl_hcnt(&self) -> SS_SCL_HCNT_R {
        SS_SCL_HCNT_R::new(self.bits)
    }
}
impl W {
    #[doc = "Bits 0:31 - ss_scl_hcnt"]
    #[inline(always)]
    #[must_use]
    pub fn ss_scl_hcnt(&mut self) -> SS_SCL_HCNT_W<SS_SCL_HCNT_SPEC, 0> {
        SS_SCL_HCNT_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.bits = bits;
        self
    }
}
#[doc = "DesignWare I2C SS SCL HCNT\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`ss_scl_hcnt::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`ss_scl_hcnt::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct SS_SCL_HCNT_SPEC;
impl crate::RegisterSpec for SS_SCL_HCNT_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`ss_scl_hcnt::R`](R) reader structure"]
impl crate::Readable for SS_SCL_HCNT_SPEC {}
#[doc = "`write(|w| ..)` method takes [`ss_scl_hcnt::W`](W) writer structure"]
impl crate::Writable for SS_SCL_HCNT_SPEC {
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets ss_scl_hcnt to value 0"]
impl crate::Resettable for SS_SCL_HCNT_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
