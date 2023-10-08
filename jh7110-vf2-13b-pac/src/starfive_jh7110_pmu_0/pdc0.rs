#[doc = "Register `pdc0` reader"]
pub type R = crate::R<PDC0_SPEC>;
#[doc = "Register `pdc0` writer"]
pub type W = crate::W<PDC0_SPEC>;
#[doc = "Field `pd0_off_cas` reader - Power domain 0 turn-off cascade. The register value indicates the power-off sequence of this domain. 0 means the highest priority. System only accepts value from 0 to 7, any other value is invalid."]
pub type PD0_OFF_CAS_R = crate::FieldReader;
#[doc = "Field `pd0_off_cas` writer - Power domain 0 turn-off cascade. The register value indicates the power-off sequence of this domain. 0 means the highest priority. System only accepts value from 0 to 7, any other value is invalid."]
pub type PD0_OFF_CAS_W<'a, REG, const O: u8> = crate::FieldWriter<'a, REG, 5, O>;
#[doc = "Field `pd0_on_cas` reader - Power domain 0 turn-on cascade. The register value indicates the power-on sequence of this domain. 0 means the highest priority. System only accepts value from 0 to 7, any other value is invalid."]
pub type PD0_ON_CAS_R = crate::FieldReader;
#[doc = "Field `pd0_on_cas` writer - Power domain 0 turn-on cascade. The register value indicates the power-on sequence of this domain. 0 means the highest priority. System only accepts value from 0 to 7, any other value is invalid."]
pub type PD0_ON_CAS_W<'a, REG, const O: u8> = crate::FieldWriter<'a, REG, 5, O>;
#[doc = "Field `pd1_off_cas` reader - Power domain 1 turn-off cascade. The register value indicates the power-off sequence of this domain. 0 means the highest priority. System only accepts value from 0 to 7, any other value is invalid."]
pub type PD1_OFF_CAS_R = crate::FieldReader;
#[doc = "Field `pd1_off_cas` writer - Power domain 1 turn-off cascade. The register value indicates the power-off sequence of this domain. 0 means the highest priority. System only accepts value from 0 to 7, any other value is invalid."]
pub type PD1_OFF_CAS_W<'a, REG, const O: u8> = crate::FieldWriter<'a, REG, 5, O>;
#[doc = "Field `pd1_on_cas` reader - Power domain 1 turn-on cascade. The register value indicates the power-on sequence of this domain. 0 means the highest priority. System only accepts value from 0 to 7, any other value is invalid."]
pub type PD1_ON_CAS_R = crate::FieldReader;
#[doc = "Field `pd1_on_cas` writer - Power domain 1 turn-on cascade. The register value indicates the power-on sequence of this domain. 0 means the highest priority. System only accepts value from 0 to 7, any other value is invalid."]
pub type PD1_ON_CAS_W<'a, REG, const O: u8> = crate::FieldWriter<'a, REG, 5, O>;
#[doc = "Field `pd2_off_cas` reader - Power domain 2 turn-off cascade. The register value indicates the power-off sequence of this domain. 0 means the highest priority. System only accepts value from 0 to 7, any other value is invalid."]
pub type PD2_OFF_CAS_R = crate::FieldReader;
#[doc = "Field `pd2_off_cas` writer - Power domain 2 turn-off cascade. The register value indicates the power-off sequence of this domain. 0 means the highest priority. System only accepts value from 0 to 7, any other value is invalid."]
pub type PD2_OFF_CAS_W<'a, REG, const O: u8> = crate::FieldWriter<'a, REG, 5, O>;
#[doc = "Field `pd2_on_cas` reader - Power domain 2 turn-on cascade. The register value indicates the power-on sequence of this domain. 0 means the highest priority. System only accepts value from 0 to 7, any other value is invalid."]
pub type PD2_ON_CAS_R = crate::FieldReader;
#[doc = "Field `pd2_on_cas` writer - Power domain 2 turn-on cascade. The register value indicates the power-on sequence of this domain. 0 means the highest priority. System only accepts value from 0 to 7, any other value is invalid."]
pub type PD2_ON_CAS_W<'a, REG, const O: u8> = crate::FieldWriter<'a, REG, 5, O>;
impl R {
    #[doc = "Bits 0:4 - Power domain 0 turn-off cascade. The register value indicates the power-off sequence of this domain. 0 means the highest priority. System only accepts value from 0 to 7, any other value is invalid."]
    #[inline(always)]
    pub fn pd0_off_cas(&self) -> PD0_OFF_CAS_R {
        PD0_OFF_CAS_R::new((self.bits & 0x1f) as u8)
    }
    #[doc = "Bits 5:9 - Power domain 0 turn-on cascade. The register value indicates the power-on sequence of this domain. 0 means the highest priority. System only accepts value from 0 to 7, any other value is invalid."]
    #[inline(always)]
    pub fn pd0_on_cas(&self) -> PD0_ON_CAS_R {
        PD0_ON_CAS_R::new(((self.bits >> 5) & 0x1f) as u8)
    }
    #[doc = "Bits 10:14 - Power domain 1 turn-off cascade. The register value indicates the power-off sequence of this domain. 0 means the highest priority. System only accepts value from 0 to 7, any other value is invalid."]
    #[inline(always)]
    pub fn pd1_off_cas(&self) -> PD1_OFF_CAS_R {
        PD1_OFF_CAS_R::new(((self.bits >> 10) & 0x1f) as u8)
    }
    #[doc = "Bits 15:19 - Power domain 1 turn-on cascade. The register value indicates the power-on sequence of this domain. 0 means the highest priority. System only accepts value from 0 to 7, any other value is invalid."]
    #[inline(always)]
    pub fn pd1_on_cas(&self) -> PD1_ON_CAS_R {
        PD1_ON_CAS_R::new(((self.bits >> 15) & 0x1f) as u8)
    }
    #[doc = "Bits 20:24 - Power domain 2 turn-off cascade. The register value indicates the power-off sequence of this domain. 0 means the highest priority. System only accepts value from 0 to 7, any other value is invalid."]
    #[inline(always)]
    pub fn pd2_off_cas(&self) -> PD2_OFF_CAS_R {
        PD2_OFF_CAS_R::new(((self.bits >> 20) & 0x1f) as u8)
    }
    #[doc = "Bits 25:29 - Power domain 2 turn-on cascade. The register value indicates the power-on sequence of this domain. 0 means the highest priority. System only accepts value from 0 to 7, any other value is invalid."]
    #[inline(always)]
    pub fn pd2_on_cas(&self) -> PD2_ON_CAS_R {
        PD2_ON_CAS_R::new(((self.bits >> 25) & 0x1f) as u8)
    }
}
impl W {
    #[doc = "Bits 0:4 - Power domain 0 turn-off cascade. The register value indicates the power-off sequence of this domain. 0 means the highest priority. System only accepts value from 0 to 7, any other value is invalid."]
    #[inline(always)]
    #[must_use]
    pub fn pd0_off_cas(&mut self) -> PD0_OFF_CAS_W<PDC0_SPEC, 0> {
        PD0_OFF_CAS_W::new(self)
    }
    #[doc = "Bits 5:9 - Power domain 0 turn-on cascade. The register value indicates the power-on sequence of this domain. 0 means the highest priority. System only accepts value from 0 to 7, any other value is invalid."]
    #[inline(always)]
    #[must_use]
    pub fn pd0_on_cas(&mut self) -> PD0_ON_CAS_W<PDC0_SPEC, 5> {
        PD0_ON_CAS_W::new(self)
    }
    #[doc = "Bits 10:14 - Power domain 1 turn-off cascade. The register value indicates the power-off sequence of this domain. 0 means the highest priority. System only accepts value from 0 to 7, any other value is invalid."]
    #[inline(always)]
    #[must_use]
    pub fn pd1_off_cas(&mut self) -> PD1_OFF_CAS_W<PDC0_SPEC, 10> {
        PD1_OFF_CAS_W::new(self)
    }
    #[doc = "Bits 15:19 - Power domain 1 turn-on cascade. The register value indicates the power-on sequence of this domain. 0 means the highest priority. System only accepts value from 0 to 7, any other value is invalid."]
    #[inline(always)]
    #[must_use]
    pub fn pd1_on_cas(&mut self) -> PD1_ON_CAS_W<PDC0_SPEC, 15> {
        PD1_ON_CAS_W::new(self)
    }
    #[doc = "Bits 20:24 - Power domain 2 turn-off cascade. The register value indicates the power-off sequence of this domain. 0 means the highest priority. System only accepts value from 0 to 7, any other value is invalid."]
    #[inline(always)]
    #[must_use]
    pub fn pd2_off_cas(&mut self) -> PD2_OFF_CAS_W<PDC0_SPEC, 20> {
        PD2_OFF_CAS_W::new(self)
    }
    #[doc = "Bits 25:29 - Power domain 2 turn-on cascade. The register value indicates the power-on sequence of this domain. 0 means the highest priority. System only accepts value from 0 to 7, any other value is invalid."]
    #[inline(always)]
    #[must_use]
    pub fn pd2_on_cas(&mut self) -> PD2_ON_CAS_W<PDC0_SPEC, 25> {
        PD2_ON_CAS_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.bits = bits;
        self
    }
}
#[doc = "Powerdomain Cascade 0\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`pdc0::R`](R).  You can [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`pdc0::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct PDC0_SPEC;
impl crate::RegisterSpec for PDC0_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`pdc0::R`](R) reader structure"]
impl crate::Readable for PDC0_SPEC {}
#[doc = "`write(|w| ..)` method takes [`pdc0::W`](W) writer structure"]
impl crate::Writable for PDC0_SPEC {
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
