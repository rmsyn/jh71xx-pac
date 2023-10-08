#[doc = "Register `pdc2` reader"]
pub type R = crate::R<PDC2_SPEC>;
#[doc = "Register `pdc2` writer"]
pub type W = crate::W<PDC2_SPEC>;
#[doc = "Field `pd6_off_cas` reader - Power domain 6 turn-off cascade. The register value indicates the power-off sequence of this domain. 0 means the highest priority. System only accepts value from 0 to 7, any other value is invalid."]
pub type PD6_OFF_CAS_R = crate::FieldReader;
#[doc = "Field `pd6_off_cas` writer - Power domain 6 turn-off cascade. The register value indicates the power-off sequence of this domain. 0 means the highest priority. System only accepts value from 0 to 7, any other value is invalid."]
pub type PD6_OFF_CAS_W<'a, REG, const O: u8> = crate::FieldWriter<'a, REG, 5, O>;
#[doc = "Field `pd6_on_cas` reader - Power domain 6 turn-on cascade. The register value indicates the power-on sequence of this domain. 0 means the highest priority. System only accepts value from 0 to 7, any other value is invalid."]
pub type PD6_ON_CAS_R = crate::FieldReader;
#[doc = "Field `pd6_on_cas` writer - Power domain 6 turn-on cascade. The register value indicates the power-on sequence of this domain. 0 means the highest priority. System only accepts value from 0 to 7, any other value is invalid."]
pub type PD6_ON_CAS_W<'a, REG, const O: u8> = crate::FieldWriter<'a, REG, 5, O>;
#[doc = "Field `pd7_off_cas` reader - Power domain 7 turn-off cascade. The register value indicates the power-off sequence of this domain. 0 means the highest priority. System only accepts value from 0 to 7, any other value is invalid."]
pub type PD7_OFF_CAS_R = crate::FieldReader;
#[doc = "Field `pd7_off_cas` writer - Power domain 7 turn-off cascade. The register value indicates the power-off sequence of this domain. 0 means the highest priority. System only accepts value from 0 to 7, any other value is invalid."]
pub type PD7_OFF_CAS_W<'a, REG, const O: u8> = crate::FieldWriter<'a, REG, 5, O>;
#[doc = "Field `pd7_on_cas` reader - Power domain 7 turn-on cascade. The register value indicates the power-on sequence of this domain. 0 means the highest priority. System only accepts value from 0 to 7, any other value is invalid."]
pub type PD7_ON_CAS_R = crate::FieldReader;
#[doc = "Field `pd7_on_cas` writer - Power domain 7 turn-on cascade. The register value indicates the power-on sequence of this domain. 0 means the highest priority. System only accepts value from 0 to 7, any other value is invalid."]
pub type PD7_ON_CAS_W<'a, REG, const O: u8> = crate::FieldWriter<'a, REG, 5, O>;
#[doc = "Field `pd8_off_cas` reader - Power domain 8 turn-off cascade. The register value indicates the power-off sequence of this domain. 0 means the highest priority. System only accepts value from 0 to 7, any other value is invalid."]
pub type PD8_OFF_CAS_R = crate::FieldReader;
#[doc = "Field `pd8_off_cas` writer - Power domain 8 turn-off cascade. The register value indicates the power-off sequence of this domain. 0 means the highest priority. System only accepts value from 0 to 7, any other value is invalid."]
pub type PD8_OFF_CAS_W<'a, REG, const O: u8> = crate::FieldWriter<'a, REG, 5, O>;
#[doc = "Field `pd8_on_cas` reader - Power domain 8 turn-on cascade. The register value indicates the power-on sequence of this domain. 0 means the highest priority. System only accepts value from 0 to 7, any other value is invalid."]
pub type PD8_ON_CAS_R = crate::FieldReader;
#[doc = "Field `pd8_on_cas` writer - Power domain 8 turn-on cascade. The register value indicates the power-on sequence of this domain. 0 means the highest priority. System only accepts value from 0 to 7, any other value is invalid."]
pub type PD8_ON_CAS_W<'a, REG, const O: u8> = crate::FieldWriter<'a, REG, 5, O>;
impl R {
    #[doc = "Bits 0:4 - Power domain 6 turn-off cascade. The register value indicates the power-off sequence of this domain. 0 means the highest priority. System only accepts value from 0 to 7, any other value is invalid."]
    #[inline(always)]
    pub fn pd6_off_cas(&self) -> PD6_OFF_CAS_R {
        PD6_OFF_CAS_R::new((self.bits & 0x1f) as u8)
    }
    #[doc = "Bits 5:9 - Power domain 6 turn-on cascade. The register value indicates the power-on sequence of this domain. 0 means the highest priority. System only accepts value from 0 to 7, any other value is invalid."]
    #[inline(always)]
    pub fn pd6_on_cas(&self) -> PD6_ON_CAS_R {
        PD6_ON_CAS_R::new(((self.bits >> 5) & 0x1f) as u8)
    }
    #[doc = "Bits 10:14 - Power domain 7 turn-off cascade. The register value indicates the power-off sequence of this domain. 0 means the highest priority. System only accepts value from 0 to 7, any other value is invalid."]
    #[inline(always)]
    pub fn pd7_off_cas(&self) -> PD7_OFF_CAS_R {
        PD7_OFF_CAS_R::new(((self.bits >> 10) & 0x1f) as u8)
    }
    #[doc = "Bits 15:19 - Power domain 7 turn-on cascade. The register value indicates the power-on sequence of this domain. 0 means the highest priority. System only accepts value from 0 to 7, any other value is invalid."]
    #[inline(always)]
    pub fn pd7_on_cas(&self) -> PD7_ON_CAS_R {
        PD7_ON_CAS_R::new(((self.bits >> 15) & 0x1f) as u8)
    }
    #[doc = "Bits 20:24 - Power domain 8 turn-off cascade. The register value indicates the power-off sequence of this domain. 0 means the highest priority. System only accepts value from 0 to 7, any other value is invalid."]
    #[inline(always)]
    pub fn pd8_off_cas(&self) -> PD8_OFF_CAS_R {
        PD8_OFF_CAS_R::new(((self.bits >> 20) & 0x1f) as u8)
    }
    #[doc = "Bits 25:29 - Power domain 8 turn-on cascade. The register value indicates the power-on sequence of this domain. 0 means the highest priority. System only accepts value from 0 to 7, any other value is invalid."]
    #[inline(always)]
    pub fn pd8_on_cas(&self) -> PD8_ON_CAS_R {
        PD8_ON_CAS_R::new(((self.bits >> 25) & 0x1f) as u8)
    }
}
impl W {
    #[doc = "Bits 0:4 - Power domain 6 turn-off cascade. The register value indicates the power-off sequence of this domain. 0 means the highest priority. System only accepts value from 0 to 7, any other value is invalid."]
    #[inline(always)]
    #[must_use]
    pub fn pd6_off_cas(&mut self) -> PD6_OFF_CAS_W<PDC2_SPEC, 0> {
        PD6_OFF_CAS_W::new(self)
    }
    #[doc = "Bits 5:9 - Power domain 6 turn-on cascade. The register value indicates the power-on sequence of this domain. 0 means the highest priority. System only accepts value from 0 to 7, any other value is invalid."]
    #[inline(always)]
    #[must_use]
    pub fn pd6_on_cas(&mut self) -> PD6_ON_CAS_W<PDC2_SPEC, 5> {
        PD6_ON_CAS_W::new(self)
    }
    #[doc = "Bits 10:14 - Power domain 7 turn-off cascade. The register value indicates the power-off sequence of this domain. 0 means the highest priority. System only accepts value from 0 to 7, any other value is invalid."]
    #[inline(always)]
    #[must_use]
    pub fn pd7_off_cas(&mut self) -> PD7_OFF_CAS_W<PDC2_SPEC, 10> {
        PD7_OFF_CAS_W::new(self)
    }
    #[doc = "Bits 15:19 - Power domain 7 turn-on cascade. The register value indicates the power-on sequence of this domain. 0 means the highest priority. System only accepts value from 0 to 7, any other value is invalid."]
    #[inline(always)]
    #[must_use]
    pub fn pd7_on_cas(&mut self) -> PD7_ON_CAS_W<PDC2_SPEC, 15> {
        PD7_ON_CAS_W::new(self)
    }
    #[doc = "Bits 20:24 - Power domain 8 turn-off cascade. The register value indicates the power-off sequence of this domain. 0 means the highest priority. System only accepts value from 0 to 7, any other value is invalid."]
    #[inline(always)]
    #[must_use]
    pub fn pd8_off_cas(&mut self) -> PD8_OFF_CAS_W<PDC2_SPEC, 20> {
        PD8_OFF_CAS_W::new(self)
    }
    #[doc = "Bits 25:29 - Power domain 8 turn-on cascade. The register value indicates the power-on sequence of this domain. 0 means the highest priority. System only accepts value from 0 to 7, any other value is invalid."]
    #[inline(always)]
    #[must_use]
    pub fn pd8_on_cas(&mut self) -> PD8_ON_CAS_W<PDC2_SPEC, 25> {
        PD8_ON_CAS_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.bits = bits;
        self
    }
}
#[doc = "Powerdomain Cascade 2\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`pdc2::R`](R).  You can [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`pdc2::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct PDC2_SPEC;
impl crate::RegisterSpec for PDC2_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`pdc2::R`](R) reader structure"]
impl crate::Readable for PDC2_SPEC {}
#[doc = "`write(|w| ..)` method takes [`pdc2::W`](W) writer structure"]
impl crate::Writable for PDC2_SPEC {
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
