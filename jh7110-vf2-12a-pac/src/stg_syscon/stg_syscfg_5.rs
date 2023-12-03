#[doc = "Register `stg_syscfg_5` reader"]
pub type R = crate::R<STG_SYSCFG_5_SPEC>;
#[doc = "Register `stg_syscfg_5` writer"]
pub type W = crate::W<STG_SYSCFG_5_SPEC>;
#[doc = "Field `u0_cdn_usb_xhci_debug_link_state` reader - u0_cdn_usb_xhci_debug_link_state"]
pub type U0_CDN_USB_XHCI_DEBUG_LINK_STATE_R = crate::FieldReader<u32>;
impl R {
    #[doc = "Bits 0:30 - u0_cdn_usb_xhci_debug_link_state"]
    #[inline(always)]
    pub fn u0_cdn_usb_xhci_debug_link_state(&self) -> U0_CDN_USB_XHCI_DEBUG_LINK_STATE_R {
        U0_CDN_USB_XHCI_DEBUG_LINK_STATE_R::new(self.bits & 0x7fff_ffff)
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
#[doc = "STG SYSCONSAIF SYSCFG 20\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`stg_syscfg_5::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`stg_syscfg_5::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct STG_SYSCFG_5_SPEC;
impl crate::RegisterSpec for STG_SYSCFG_5_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`stg_syscfg_5::R`](R) reader structure"]
impl crate::Readable for STG_SYSCFG_5_SPEC {}
#[doc = "`write(|w| ..)` method takes [`stg_syscfg_5::W`](W) writer structure"]
impl crate::Writable for STG_SYSCFG_5_SPEC {
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets stg_syscfg_5 to value 0"]
impl crate::Resettable for STG_SYSCFG_5_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
