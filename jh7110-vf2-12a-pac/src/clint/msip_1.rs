#[doc = "Register `msip_1` reader"]
pub type R = crate::R<MSIP_1_SPEC>;
#[doc = "Register `msip_1` writer"]
pub type W = crate::W<MSIP_1_SPEC>;
#[doc = "Field `control` reader - "]
pub type CONTROL_R = crate::BitReader;
#[doc = "Field `control` writer - "]
pub type CONTROL_W<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bit 0"]
    #[inline(always)]
    pub fn control(&self) -> CONTROL_R {
        CONTROL_R::new((self.bits & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0"]
    #[inline(always)]
    #[must_use]
    pub fn control(&mut self) -> CONTROL_W<MSIP_1_SPEC> {
        CONTROL_W::new(self, 0)
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
#[doc = "MSIP Register for hart 1\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`msip_1::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`msip_1::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct MSIP_1_SPEC;
impl crate::RegisterSpec for MSIP_1_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`msip_1::R`](R) reader structure"]
impl crate::Readable for MSIP_1_SPEC {}
#[doc = "`write(|w| ..)` method takes [`msip_1::W`](W) writer structure"]
impl crate::Writable for MSIP_1_SPEC {
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets msip_1 to value 0"]
impl crate::Resettable for MSIP_1_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
