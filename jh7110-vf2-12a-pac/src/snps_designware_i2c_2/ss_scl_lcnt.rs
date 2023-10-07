#[doc = "Register `ss_scl_lcnt` reader"]
pub type R = crate::R<SS_SCL_LCNT_SPEC>;
#[doc = "Register `ss_scl_lcnt` writer"]
pub type W = crate::W<SS_SCL_LCNT_SPEC>;
#[doc = "Field `ss_scl_lcnt` reader - ss_scl_lcnt"]
pub type SS_SCL_LCNT_R = crate::FieldReader<u32>;
#[doc = "Field `ss_scl_lcnt` writer - ss_scl_lcnt"]
pub type SS_SCL_LCNT_W<'a, REG, const O: u8> = crate::FieldWriter<'a, REG, 32, O, u32>;
impl R {
    #[doc = "Bits 0:31 - ss_scl_lcnt"]
    #[inline(always)]
    pub fn ss_scl_lcnt(&self) -> SS_SCL_LCNT_R {
        SS_SCL_LCNT_R::new(self.bits)
    }
}
impl W {
    #[doc = "Bits 0:31 - ss_scl_lcnt"]
    #[inline(always)]
    #[must_use]
    pub fn ss_scl_lcnt(&mut self) -> SS_SCL_LCNT_W<SS_SCL_LCNT_SPEC, 0> {
        SS_SCL_LCNT_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.bits = bits;
        self
    }
}
#[doc = "DesignWare I2C SS SCL LCNT\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`ss_scl_lcnt::R`](R).  You can [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`ss_scl_lcnt::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct SS_SCL_LCNT_SPEC;
impl crate::RegisterSpec for SS_SCL_LCNT_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`ss_scl_lcnt::R`](R) reader structure"]
impl crate::Readable for SS_SCL_LCNT_SPEC {}
#[doc = "`write(|w| ..)` method takes [`ss_scl_lcnt::W`](W) writer structure"]
impl crate::Writable for SS_SCL_LCNT_SPEC {
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
