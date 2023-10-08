#[doc = "Register `encourage_type_crd` reader"]
pub type R = crate::R<ENCOURAGE_TYPE_CRD_SPEC>;
#[doc = "Register `encourage_type_crd` writer"]
pub type W = crate::W<ENCOURAGE_TYPE_CRD_SPEC>;
#[doc = "Field `encourage_type_crd` reader - Hardware/Software encouragement type record. 0: Software, 1: Hardware."]
pub type ENCOURAGE_TYPE_CRD_R = crate::BitReader;
impl R {
    #[doc = "Bit 0 - Hardware/Software encouragement type record. 0: Software, 1: Hardware."]
    #[inline(always)]
    pub fn encourage_type_crd(&self) -> ENCOURAGE_TYPE_CRD_R {
        ENCOURAGE_TYPE_CRD_R::new((self.bits & 1) != 0)
    }
}
impl W {
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.bits = bits;
        self
    }
}
#[doc = "Hardware Event Type Record\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`encourage_type_crd::R`](R).  You can [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`encourage_type_crd::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct ENCOURAGE_TYPE_CRD_SPEC;
impl crate::RegisterSpec for ENCOURAGE_TYPE_CRD_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`encourage_type_crd::R`](R) reader structure"]
impl crate::Readable for ENCOURAGE_TYPE_CRD_SPEC {}
#[doc = "`write(|w| ..)` method takes [`encourage_type_crd::W`](W) writer structure"]
impl crate::Writable for ENCOURAGE_TYPE_CRD_SPEC {
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
