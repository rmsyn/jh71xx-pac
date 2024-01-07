#[doc = "Register `ucv` reader"]
pub type R = crate::R<UCV_SPEC>;
#[doc = "Register `ucv` writer"]
pub type W = crate::W<UCV_SPEC>;
#[doc = "Field `ucv` reader - ASCII value for each number in the version, followed by *. For example 32_30_31_2A represents the version 2.01*"]
pub type UCV_R = crate::FieldReader<u32>;
impl R {
    #[doc = "Bits 0:31 - ASCII value for each number in the version, followed by *. For example 32_30_31_2A represents the version 2.01*"]
    #[inline(always)]
    pub fn ucv(&self) -> UCV_R {
        UCV_R::new(self.bits)
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
#[doc = "UART Component Version: This register is only valid when the DW_apb_uart is configured to have additional features implemented (ADDITIONAL_FEATURES == YES). If additional features are not implemented, this register does not exist and reading from this register address returns zero.\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`ucv::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`ucv::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct UCV_SPEC;
impl crate::RegisterSpec for UCV_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`ucv::R`](R) reader structure"]
impl crate::Readable for UCV_SPEC {}
#[doc = "`write(|w| ..)` method takes [`ucv::W`](W) writer structure"]
impl crate::Writable for UCV_SPEC {
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets ucv to value 0"]
impl crate::Resettable for UCV_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
