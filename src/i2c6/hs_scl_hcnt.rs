#[doc = "Register `hs_scl_hcnt` reader"]
pub type R = crate::R<HS_SCL_HCNT_SPEC>;
#[doc = "Register `hs_scl_hcnt` writer"]
pub type W = crate::W<HS_SCL_HCNT_SPEC>;
#[doc = "Field `hs_scl_hcnt` reader - hs_scl_hcnt"]
pub type HS_SCL_HCNT_R = crate::FieldReader<u32>;
#[doc = "Field `hs_scl_hcnt` writer - hs_scl_hcnt"]
pub type HS_SCL_HCNT_W<'a, REG> = crate::FieldWriter<'a, REG, 32, u32>;
impl R {
    #[doc = "Bits 0:31 - hs_scl_hcnt"]
    #[inline(always)]
    pub fn hs_scl_hcnt(&self) -> HS_SCL_HCNT_R {
        HS_SCL_HCNT_R::new(self.bits)
    }
}
impl W {
    #[doc = "Bits 0:31 - hs_scl_hcnt"]
    #[inline(always)]
    #[must_use]
    pub fn hs_scl_hcnt(&mut self) -> HS_SCL_HCNT_W<HS_SCL_HCNT_SPEC> {
        HS_SCL_HCNT_W::new(self, 0)
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
#[doc = "DesignWare I2C HS SCL HCNT\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`hs_scl_hcnt::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`hs_scl_hcnt::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct HS_SCL_HCNT_SPEC;
impl crate::RegisterSpec for HS_SCL_HCNT_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`hs_scl_hcnt::R`](R) reader structure"]
impl crate::Readable for HS_SCL_HCNT_SPEC {}
#[doc = "`write(|w| ..)` method takes [`hs_scl_hcnt::W`](W) writer structure"]
impl crate::Writable for HS_SCL_HCNT_SPEC {
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets hs_scl_hcnt to value 0"]
impl crate::Resettable for HS_SCL_HCNT_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
