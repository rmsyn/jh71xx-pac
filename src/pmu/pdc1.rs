#[doc = "Register `pdc1` reader"]
pub type R = crate::R<PDC1_SPEC>;
#[doc = "Register `pdc1` writer"]
pub type W = crate::W<PDC1_SPEC>;
#[doc = "Field `pd3_off_cas` reader - Power domain 3 turn-off cascade. The register value indicates the power-off sequence of this domain. 0 means the highest priority. System only accepts value from 0 to 7, any other value is invalid."]
pub type PD3_OFF_CAS_R = crate::FieldReader;
#[doc = "Field `pd3_off_cas` writer - Power domain 3 turn-off cascade. The register value indicates the power-off sequence of this domain. 0 means the highest priority. System only accepts value from 0 to 7, any other value is invalid."]
pub type PD3_OFF_CAS_W<'a, REG> = crate::FieldWriter<'a, REG, 5>;
#[doc = "Field `pd3_on_cas` reader - Power domain 3 turn-on cascade. The register value indicates the power-on sequence of this domain. 0 means the highest priority. System only accepts value from 0 to 7, any other value is invalid."]
pub type PD3_ON_CAS_R = crate::FieldReader;
#[doc = "Field `pd3_on_cas` writer - Power domain 3 turn-on cascade. The register value indicates the power-on sequence of this domain. 0 means the highest priority. System only accepts value from 0 to 7, any other value is invalid."]
pub type PD3_ON_CAS_W<'a, REG> = crate::FieldWriter<'a, REG, 5>;
#[doc = "Field `pd4_off_cas` reader - Power domain 4 turn-off cascade. The register value indicates the power-off sequence of this domain. 0 means the highest priority. System only accepts value from 0 to 7, any other value is invalid."]
pub type PD4_OFF_CAS_R = crate::FieldReader;
#[doc = "Field `pd4_off_cas` writer - Power domain 4 turn-off cascade. The register value indicates the power-off sequence of this domain. 0 means the highest priority. System only accepts value from 0 to 7, any other value is invalid."]
pub type PD4_OFF_CAS_W<'a, REG> = crate::FieldWriter<'a, REG, 5>;
#[doc = "Field `pd4_on_cas` reader - Power domain 4 turn-on cascade. The register value indicates the power-on sequence of this domain. 0 means the highest priority. System only accepts value from 0 to 7, any other value is invalid."]
pub type PD4_ON_CAS_R = crate::FieldReader;
#[doc = "Field `pd4_on_cas` writer - Power domain 4 turn-on cascade. The register value indicates the power-on sequence of this domain. 0 means the highest priority. System only accepts value from 0 to 7, any other value is invalid."]
pub type PD4_ON_CAS_W<'a, REG> = crate::FieldWriter<'a, REG, 5>;
#[doc = "Field `pd5_off_cas` reader - Power domain 5 turn-off cascade. The register value indicates the power-off sequence of this domain. 0 means the highest priority. System only accepts value from 0 to 7, any other value is invalid."]
pub type PD5_OFF_CAS_R = crate::FieldReader;
#[doc = "Field `pd5_off_cas` writer - Power domain 5 turn-off cascade. The register value indicates the power-off sequence of this domain. 0 means the highest priority. System only accepts value from 0 to 7, any other value is invalid."]
pub type PD5_OFF_CAS_W<'a, REG> = crate::FieldWriter<'a, REG, 5>;
#[doc = "Field `pd5_on_cas` reader - Power domain 5 turn-on cascade. The register value indicates the power-on sequence of this domain. 0 means the highest priority. System only accepts value from 0 to 7, any other value is invalid."]
pub type PD5_ON_CAS_R = crate::FieldReader;
#[doc = "Field `pd5_on_cas` writer - Power domain 5 turn-on cascade. The register value indicates the power-on sequence of this domain. 0 means the highest priority. System only accepts value from 0 to 7, any other value is invalid."]
pub type PD5_ON_CAS_W<'a, REG> = crate::FieldWriter<'a, REG, 5>;
impl R {
    #[doc = "Bits 0:4 - Power domain 3 turn-off cascade. The register value indicates the power-off sequence of this domain. 0 means the highest priority. System only accepts value from 0 to 7, any other value is invalid."]
    #[inline(always)]
    pub fn pd3_off_cas(&self) -> PD3_OFF_CAS_R {
        PD3_OFF_CAS_R::new((self.bits & 0x1f) as u8)
    }
    #[doc = "Bits 5:9 - Power domain 3 turn-on cascade. The register value indicates the power-on sequence of this domain. 0 means the highest priority. System only accepts value from 0 to 7, any other value is invalid."]
    #[inline(always)]
    pub fn pd3_on_cas(&self) -> PD3_ON_CAS_R {
        PD3_ON_CAS_R::new(((self.bits >> 5) & 0x1f) as u8)
    }
    #[doc = "Bits 10:14 - Power domain 4 turn-off cascade. The register value indicates the power-off sequence of this domain. 0 means the highest priority. System only accepts value from 0 to 7, any other value is invalid."]
    #[inline(always)]
    pub fn pd4_off_cas(&self) -> PD4_OFF_CAS_R {
        PD4_OFF_CAS_R::new(((self.bits >> 10) & 0x1f) as u8)
    }
    #[doc = "Bits 15:19 - Power domain 4 turn-on cascade. The register value indicates the power-on sequence of this domain. 0 means the highest priority. System only accepts value from 0 to 7, any other value is invalid."]
    #[inline(always)]
    pub fn pd4_on_cas(&self) -> PD4_ON_CAS_R {
        PD4_ON_CAS_R::new(((self.bits >> 15) & 0x1f) as u8)
    }
    #[doc = "Bits 20:24 - Power domain 5 turn-off cascade. The register value indicates the power-off sequence of this domain. 0 means the highest priority. System only accepts value from 0 to 7, any other value is invalid."]
    #[inline(always)]
    pub fn pd5_off_cas(&self) -> PD5_OFF_CAS_R {
        PD5_OFF_CAS_R::new(((self.bits >> 20) & 0x1f) as u8)
    }
    #[doc = "Bits 25:29 - Power domain 5 turn-on cascade. The register value indicates the power-on sequence of this domain. 0 means the highest priority. System only accepts value from 0 to 7, any other value is invalid."]
    #[inline(always)]
    pub fn pd5_on_cas(&self) -> PD5_ON_CAS_R {
        PD5_ON_CAS_R::new(((self.bits >> 25) & 0x1f) as u8)
    }
}
impl W {
    #[doc = "Bits 0:4 - Power domain 3 turn-off cascade. The register value indicates the power-off sequence of this domain. 0 means the highest priority. System only accepts value from 0 to 7, any other value is invalid."]
    #[inline(always)]
    #[must_use]
    pub fn pd3_off_cas(&mut self) -> PD3_OFF_CAS_W<PDC1_SPEC> {
        PD3_OFF_CAS_W::new(self, 0)
    }
    #[doc = "Bits 5:9 - Power domain 3 turn-on cascade. The register value indicates the power-on sequence of this domain. 0 means the highest priority. System only accepts value from 0 to 7, any other value is invalid."]
    #[inline(always)]
    #[must_use]
    pub fn pd3_on_cas(&mut self) -> PD3_ON_CAS_W<PDC1_SPEC> {
        PD3_ON_CAS_W::new(self, 5)
    }
    #[doc = "Bits 10:14 - Power domain 4 turn-off cascade. The register value indicates the power-off sequence of this domain. 0 means the highest priority. System only accepts value from 0 to 7, any other value is invalid."]
    #[inline(always)]
    #[must_use]
    pub fn pd4_off_cas(&mut self) -> PD4_OFF_CAS_W<PDC1_SPEC> {
        PD4_OFF_CAS_W::new(self, 10)
    }
    #[doc = "Bits 15:19 - Power domain 4 turn-on cascade. The register value indicates the power-on sequence of this domain. 0 means the highest priority. System only accepts value from 0 to 7, any other value is invalid."]
    #[inline(always)]
    #[must_use]
    pub fn pd4_on_cas(&mut self) -> PD4_ON_CAS_W<PDC1_SPEC> {
        PD4_ON_CAS_W::new(self, 15)
    }
    #[doc = "Bits 20:24 - Power domain 5 turn-off cascade. The register value indicates the power-off sequence of this domain. 0 means the highest priority. System only accepts value from 0 to 7, any other value is invalid."]
    #[inline(always)]
    #[must_use]
    pub fn pd5_off_cas(&mut self) -> PD5_OFF_CAS_W<PDC1_SPEC> {
        PD5_OFF_CAS_W::new(self, 20)
    }
    #[doc = "Bits 25:29 - Power domain 5 turn-on cascade. The register value indicates the power-on sequence of this domain. 0 means the highest priority. System only accepts value from 0 to 7, any other value is invalid."]
    #[inline(always)]
    #[must_use]
    pub fn pd5_on_cas(&mut self) -> PD5_ON_CAS_W<PDC1_SPEC> {
        PD5_ON_CAS_W::new(self, 25)
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
#[doc = "Powerdomain Cascade 1\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`pdc1::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`pdc1::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct PDC1_SPEC;
impl crate::RegisterSpec for PDC1_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`pdc1::R`](R) reader structure"]
impl crate::Readable for PDC1_SPEC {}
#[doc = "`write(|w| ..)` method takes [`pdc1::W`](W) writer structure"]
impl crate::Writable for PDC1_SPEC {
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets pdc1 to value 0"]
impl crate::Resettable for PDC1_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
