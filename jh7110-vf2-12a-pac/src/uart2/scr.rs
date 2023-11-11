#[doc = "Register `scr` reader"]
pub type R = crate::R<SCR_SPEC>;
#[doc = "Register `scr` writer"]
pub type W = crate::W<SCR_SPEC>;
#[doc = "Field `scr` reader - This register is for programmers to use as a temporary storage space. It has no defined purpose in the DW_apb_uart."]
pub type SCR_R = crate::FieldReader;
#[doc = "Field `scr` writer - This register is for programmers to use as a temporary storage space. It has no defined purpose in the DW_apb_uart."]
pub type SCR_W<'a, REG, const O: u8> = crate::FieldWriter<'a, REG, 8, O>;
impl R {
    #[doc = "Bits 0:7 - This register is for programmers to use as a temporary storage space. It has no defined purpose in the DW_apb_uart."]
    #[inline(always)]
    pub fn scr(&self) -> SCR_R {
        SCR_R::new((self.bits & 0xff) as u8)
    }
}
impl W {
    #[doc = "Bits 0:7 - This register is for programmers to use as a temporary storage space. It has no defined purpose in the DW_apb_uart."]
    #[inline(always)]
    #[must_use]
    pub fn scr(&mut self) -> SCR_W<SCR_SPEC, 0> {
        SCR_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.bits = bits;
        self
    }
}
#[doc = "Scratch Pad Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`scr::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`scr::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct SCR_SPEC;
impl crate::RegisterSpec for SCR_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`scr::R`](R) reader structure"]
impl crate::Readable for SCR_SPEC {}
#[doc = "`write(|w| ..)` method takes [`scr::W`](W) writer structure"]
impl crate::Writable for SCR_SPEC {
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets scr to value 0"]
impl crate::Resettable for SCR_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
