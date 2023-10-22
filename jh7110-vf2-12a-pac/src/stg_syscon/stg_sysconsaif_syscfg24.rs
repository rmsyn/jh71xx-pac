#[doc = "Register `stg_sysconsaif_syscfg24` reader"]
pub type R = crate::R<STG_SYSCONSAIF_SYSCFG24_SPEC>;
#[doc = "Register `stg_sysconsaif_syscfg24` writer"]
pub type W = crate::W<STG_SYSCONSAIF_SYSCFG24_SPEC>;
#[doc = "Field `u0_cdn_usb_xhci_debug_sel` reader - u0_cdn_usb_xhci_debug_sel"]
pub type U0_CDN_USB_XHCI_DEBUG_SEL_R = crate::FieldReader;
#[doc = "Field `u0_cdn_usb_xhci_debug_sel` writer - u0_cdn_usb_xhci_debug_sel"]
pub type U0_CDN_USB_XHCI_DEBUG_SEL_W<'a, REG, const O: u8> = crate::FieldWriter<'a, REG, 5, O>;
#[doc = "Field `u0_cdn_usb_xhci_main_power_off_ack` reader - u0_cdn_usb_xhci_main_power_off_ack"]
pub type U0_CDN_USB_XHCI_MAIN_POWER_OFF_ACK_R = crate::BitReader;
#[doc = "Field `u0_cdn_usb_xhci_main_power_off_req` reader - u0_cdn_usb_xhci_main_power_off_req"]
pub type U0_CDN_USB_XHCI_MAIN_POWER_OFF_REQ_R = crate::BitReader;
#[doc = "Field `u0_cdn_usb_xhci_main_power_on_ready` reader - u0_cdn_usb_xhci_main_power_on_ready"]
pub type U0_CDN_USB_XHCI_MAIN_POWER_ON_READY_R = crate::BitReader;
#[doc = "Field `u0_cdn_usb_xhci_main_power_on_ready` writer - u0_cdn_usb_xhci_main_power_on_ready"]
pub type U0_CDN_USB_XHCI_MAIN_POWER_ON_READY_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O>;
#[doc = "Field `u0_cdn_usb_xhci_main_power_on_req` reader - u0_cdn_usb_xhci_main_power_on_req"]
pub type U0_CDN_USB_XHCI_MAIN_POWER_ON_REQ_R = crate::BitReader;
#[doc = "Field `u0_cdn_usb_xhci_main_power_on_valid` reader - u0_cdn_usb_xhci_main_power_on_valid"]
pub type U0_CDN_USB_XHCI_MAIN_POWER_ON_VALID_R = crate::BitReader;
#[doc = "Field `u0_cdn_usb_xhci_main_power_on_valid` writer - u0_cdn_usb_xhci_main_power_on_valid"]
pub type U0_CDN_USB_XHCI_MAIN_POWER_ON_VALID_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O>;
#[doc = "Field `u0_cdn_usb_xhci_power_off_ack` reader - u0_cdn_usb_xhci_power_off_ack"]
pub type U0_CDN_USB_XHCI_POWER_OFF_ACK_R = crate::BitReader;
#[doc = "Field `u0_cdn_usb_xhci_power_off_ready` reader - u0_cdn_usb_xhci_power_off_ready"]
pub type U0_CDN_USB_XHCI_POWER_OFF_READY_R = crate::BitReader;
#[doc = "Field `u0_cdn_usb_xhci_power_off_req` reader - u0_cdn_usb_xhci_power_off_req"]
pub type U0_CDN_USB_XHCI_POWER_OFF_REQ_R = crate::BitReader;
#[doc = "Field `u0_cdn_usb_xhci_power_off_req` writer - u0_cdn_usb_xhci_power_off_req"]
pub type U0_CDN_USB_XHCI_POWER_OFF_REQ_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O>;
#[doc = "Field `u0_cdn_usb_xhci_power_on_ready` reader - u0_cdn_usb_xhci_power_on_ready"]
pub type U0_CDN_USB_XHCI_POWER_ON_READY_R = crate::BitReader;
#[doc = "Field `u0_cdn_usb_xhci_power_on_req` reader - u0_cdn_usb_xhci_power_on_req"]
pub type U0_CDN_USB_XHCI_POWER_ON_REQ_R = crate::BitReader;
#[doc = "Field `u0_cdn_usb_xhci_power_on_valid` reader - u0_cdn_usb_xhci_power_on_valid"]
pub type U0_CDN_USB_XHCI_POWER_ON_VALID_R = crate::BitReader;
#[doc = "Field `u0_cdn_usb_xhci_power_on_valid` writer - u0_cdn_usb_xhci_power_on_valid"]
pub type U0_CDN_USB_XHCI_POWER_ON_VALID_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O>;
#[doc = "Field `u0_e2_sft7110_cease_from_tile_0` reader - u0_e2_sft7110_cease_from_tile_0"]
pub type U0_E2_SFT7110_CEASE_FROM_TILE_0_R = crate::BitReader;
#[doc = "Field `u0_e2_sft7110_debug_from_tile_0` reader - u0_e2_sft7110_debug_from_tile_0"]
pub type U0_E2_SFT7110_DEBUG_FROM_TILE_0_R = crate::BitReader;
#[doc = "Field `u0_e2_sft7110_halt_from_tile_0` reader - u0_e2_sft7110_halt_from_tile_0"]
pub type U0_E2_SFT7110_HALT_FROM_TILE_0_R = crate::BitReader;
impl R {
    #[doc = "Bits 0:4 - u0_cdn_usb_xhci_debug_sel"]
    #[inline(always)]
    pub fn u0_cdn_usb_xhci_debug_sel(&self) -> U0_CDN_USB_XHCI_DEBUG_SEL_R {
        U0_CDN_USB_XHCI_DEBUG_SEL_R::new((self.bits & 0x1f) as u8)
    }
    #[doc = "Bit 5 - u0_cdn_usb_xhci_main_power_off_ack"]
    #[inline(always)]
    pub fn u0_cdn_usb_xhci_main_power_off_ack(&self) -> U0_CDN_USB_XHCI_MAIN_POWER_OFF_ACK_R {
        U0_CDN_USB_XHCI_MAIN_POWER_OFF_ACK_R::new(((self.bits >> 5) & 1) != 0)
    }
    #[doc = "Bit 6 - u0_cdn_usb_xhci_main_power_off_req"]
    #[inline(always)]
    pub fn u0_cdn_usb_xhci_main_power_off_req(&self) -> U0_CDN_USB_XHCI_MAIN_POWER_OFF_REQ_R {
        U0_CDN_USB_XHCI_MAIN_POWER_OFF_REQ_R::new(((self.bits >> 6) & 1) != 0)
    }
    #[doc = "Bit 7 - u0_cdn_usb_xhci_main_power_on_ready"]
    #[inline(always)]
    pub fn u0_cdn_usb_xhci_main_power_on_ready(&self) -> U0_CDN_USB_XHCI_MAIN_POWER_ON_READY_R {
        U0_CDN_USB_XHCI_MAIN_POWER_ON_READY_R::new(((self.bits >> 7) & 1) != 0)
    }
    #[doc = "Bit 8 - u0_cdn_usb_xhci_main_power_on_req"]
    #[inline(always)]
    pub fn u0_cdn_usb_xhci_main_power_on_req(&self) -> U0_CDN_USB_XHCI_MAIN_POWER_ON_REQ_R {
        U0_CDN_USB_XHCI_MAIN_POWER_ON_REQ_R::new(((self.bits >> 8) & 1) != 0)
    }
    #[doc = "Bit 9 - u0_cdn_usb_xhci_main_power_on_valid"]
    #[inline(always)]
    pub fn u0_cdn_usb_xhci_main_power_on_valid(&self) -> U0_CDN_USB_XHCI_MAIN_POWER_ON_VALID_R {
        U0_CDN_USB_XHCI_MAIN_POWER_ON_VALID_R::new(((self.bits >> 9) & 1) != 0)
    }
    #[doc = "Bit 10 - u0_cdn_usb_xhci_power_off_ack"]
    #[inline(always)]
    pub fn u0_cdn_usb_xhci_power_off_ack(&self) -> U0_CDN_USB_XHCI_POWER_OFF_ACK_R {
        U0_CDN_USB_XHCI_POWER_OFF_ACK_R::new(((self.bits >> 10) & 1) != 0)
    }
    #[doc = "Bit 11 - u0_cdn_usb_xhci_power_off_ready"]
    #[inline(always)]
    pub fn u0_cdn_usb_xhci_power_off_ready(&self) -> U0_CDN_USB_XHCI_POWER_OFF_READY_R {
        U0_CDN_USB_XHCI_POWER_OFF_READY_R::new(((self.bits >> 11) & 1) != 0)
    }
    #[doc = "Bit 12 - u0_cdn_usb_xhci_power_off_req"]
    #[inline(always)]
    pub fn u0_cdn_usb_xhci_power_off_req(&self) -> U0_CDN_USB_XHCI_POWER_OFF_REQ_R {
        U0_CDN_USB_XHCI_POWER_OFF_REQ_R::new(((self.bits >> 12) & 1) != 0)
    }
    #[doc = "Bit 13 - u0_cdn_usb_xhci_power_on_ready"]
    #[inline(always)]
    pub fn u0_cdn_usb_xhci_power_on_ready(&self) -> U0_CDN_USB_XHCI_POWER_ON_READY_R {
        U0_CDN_USB_XHCI_POWER_ON_READY_R::new(((self.bits >> 13) & 1) != 0)
    }
    #[doc = "Bit 14 - u0_cdn_usb_xhci_power_on_req"]
    #[inline(always)]
    pub fn u0_cdn_usb_xhci_power_on_req(&self) -> U0_CDN_USB_XHCI_POWER_ON_REQ_R {
        U0_CDN_USB_XHCI_POWER_ON_REQ_R::new(((self.bits >> 14) & 1) != 0)
    }
    #[doc = "Bit 15 - u0_cdn_usb_xhci_power_on_valid"]
    #[inline(always)]
    pub fn u0_cdn_usb_xhci_power_on_valid(&self) -> U0_CDN_USB_XHCI_POWER_ON_VALID_R {
        U0_CDN_USB_XHCI_POWER_ON_VALID_R::new(((self.bits >> 15) & 1) != 0)
    }
    #[doc = "Bit 16 - u0_e2_sft7110_cease_from_tile_0"]
    #[inline(always)]
    pub fn u0_e2_sft7110_cease_from_tile_0(&self) -> U0_E2_SFT7110_CEASE_FROM_TILE_0_R {
        U0_E2_SFT7110_CEASE_FROM_TILE_0_R::new(((self.bits >> 16) & 1) != 0)
    }
    #[doc = "Bit 17 - u0_e2_sft7110_debug_from_tile_0"]
    #[inline(always)]
    pub fn u0_e2_sft7110_debug_from_tile_0(&self) -> U0_E2_SFT7110_DEBUG_FROM_TILE_0_R {
        U0_E2_SFT7110_DEBUG_FROM_TILE_0_R::new(((self.bits >> 17) & 1) != 0)
    }
    #[doc = "Bit 18 - u0_e2_sft7110_halt_from_tile_0"]
    #[inline(always)]
    pub fn u0_e2_sft7110_halt_from_tile_0(&self) -> U0_E2_SFT7110_HALT_FROM_TILE_0_R {
        U0_E2_SFT7110_HALT_FROM_TILE_0_R::new(((self.bits >> 18) & 1) != 0)
    }
}
impl W {
    #[doc = "Bits 0:4 - u0_cdn_usb_xhci_debug_sel"]
    #[inline(always)]
    #[must_use]
    pub fn u0_cdn_usb_xhci_debug_sel(
        &mut self,
    ) -> U0_CDN_USB_XHCI_DEBUG_SEL_W<STG_SYSCONSAIF_SYSCFG24_SPEC, 0> {
        U0_CDN_USB_XHCI_DEBUG_SEL_W::new(self)
    }
    #[doc = "Bit 7 - u0_cdn_usb_xhci_main_power_on_ready"]
    #[inline(always)]
    #[must_use]
    pub fn u0_cdn_usb_xhci_main_power_on_ready(
        &mut self,
    ) -> U0_CDN_USB_XHCI_MAIN_POWER_ON_READY_W<STG_SYSCONSAIF_SYSCFG24_SPEC, 7> {
        U0_CDN_USB_XHCI_MAIN_POWER_ON_READY_W::new(self)
    }
    #[doc = "Bit 9 - u0_cdn_usb_xhci_main_power_on_valid"]
    #[inline(always)]
    #[must_use]
    pub fn u0_cdn_usb_xhci_main_power_on_valid(
        &mut self,
    ) -> U0_CDN_USB_XHCI_MAIN_POWER_ON_VALID_W<STG_SYSCONSAIF_SYSCFG24_SPEC, 9> {
        U0_CDN_USB_XHCI_MAIN_POWER_ON_VALID_W::new(self)
    }
    #[doc = "Bit 12 - u0_cdn_usb_xhci_power_off_req"]
    #[inline(always)]
    #[must_use]
    pub fn u0_cdn_usb_xhci_power_off_req(
        &mut self,
    ) -> U0_CDN_USB_XHCI_POWER_OFF_REQ_W<STG_SYSCONSAIF_SYSCFG24_SPEC, 12> {
        U0_CDN_USB_XHCI_POWER_OFF_REQ_W::new(self)
    }
    #[doc = "Bit 15 - u0_cdn_usb_xhci_power_on_valid"]
    #[inline(always)]
    #[must_use]
    pub fn u0_cdn_usb_xhci_power_on_valid(
        &mut self,
    ) -> U0_CDN_USB_XHCI_POWER_ON_VALID_W<STG_SYSCONSAIF_SYSCFG24_SPEC, 15> {
        U0_CDN_USB_XHCI_POWER_ON_VALID_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.bits = bits;
        self
    }
}
#[doc = "STG SYSCONSAIF SYSCFG 24\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`stg_sysconsaif_syscfg24::R`](R).  You can [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`stg_sysconsaif_syscfg24::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct STG_SYSCONSAIF_SYSCFG24_SPEC;
impl crate::RegisterSpec for STG_SYSCONSAIF_SYSCFG24_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`stg_sysconsaif_syscfg24::R`](R) reader structure"]
impl crate::Readable for STG_SYSCONSAIF_SYSCFG24_SPEC {}
#[doc = "`write(|w| ..)` method takes [`stg_sysconsaif_syscfg24::W`](W) writer structure"]
impl crate::Writable for STG_SYSCONSAIF_SYSCFG24_SPEC {
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
