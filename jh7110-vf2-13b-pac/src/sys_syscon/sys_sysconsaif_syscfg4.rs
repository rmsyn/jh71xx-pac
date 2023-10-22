#[doc = "Register `sys_sysconsaif_syscfg4` reader"]
pub type R = crate::R<SYS_SYSCONSAIF_SYSCFG4_SPEC>;
#[doc = "Register `sys_sysconsaif_syscfg4` writer"]
pub type W = crate::W<SYS_SYSCONSAIF_SYSCFG4_SPEC>;
#[doc = "Field `scfg_sd1_remap_awaddr` reader - scfg_sd1_remap_awaddr"]
pub type SCFG_SD1_REMAP_AWADDR_R = crate::FieldReader;
#[doc = "Field `scfg_sd1_remap_awaddr` writer - scfg_sd1_remap_awaddr"]
pub type SCFG_SD1_REMAP_AWADDR_W<'a, REG, const O: u8> = crate::FieldWriter<'a, REG, 4, O>;
#[doc = "Field `scfg_sec_haddr_remap` reader - scfg_sec_haddr_remap"]
pub type SCFG_SEC_HADDR_REMAP_R = crate::FieldReader;
#[doc = "Field `scfg_sec_haddr_remap` writer - scfg_sec_haddr_remap"]
pub type SCFG_SEC_HADDR_REMAP_W<'a, REG, const O: u8> = crate::FieldWriter<'a, REG, 4, O>;
#[doc = "Field `scfg_usb_araddr_remap` reader - scfg_usb_araddr_remap"]
pub type SCFG_USB_ARADDR_REMAP_R = crate::FieldReader;
#[doc = "Field `scfg_usb_araddr_remap` writer - scfg_usb_araddr_remap"]
pub type SCFG_USB_ARADDR_REMAP_W<'a, REG, const O: u8> = crate::FieldWriter<'a, REG, 4, O>;
#[doc = "Field `scfg_usb_awaddr_remap` reader - scfg_usb_awaddr_remap"]
pub type SCFG_USB_AWADDR_REMAP_R = crate::FieldReader;
#[doc = "Field `scfg_usb_awaddr_remap` writer - scfg_usb_awaddr_remap"]
pub type SCFG_USB_AWADDR_REMAP_W<'a, REG, const O: u8> = crate::FieldWriter<'a, REG, 4, O>;
#[doc = "Field `scfg_vdec_remap_awaddr` reader - scfg_vdec_remap_awaddr"]
pub type SCFG_VDEC_REMAP_AWADDR_R = crate::FieldReader;
#[doc = "Field `scfg_vdec_remap_awaddr` writer - scfg_vdec_remap_awaddr"]
pub type SCFG_VDEC_REMAP_AWADDR_W<'a, REG, const O: u8> = crate::FieldWriter<'a, REG, 4, O>;
#[doc = "Field `scfg_venc_remap_araddr` reader - scfg_venc_remap_araddr"]
pub type SCFG_VENC_REMAP_ARADDR_R = crate::FieldReader;
#[doc = "Field `scfg_venc_remap_araddr` writer - scfg_venc_remap_araddr"]
pub type SCFG_VENC_REMAP_ARADDR_W<'a, REG, const O: u8> = crate::FieldWriter<'a, REG, 4, O>;
#[doc = "Field `scfg_venc_remap_awaddr` reader - scfg_venc_remap_awaddr"]
pub type SCFG_VENC_REMAP_AWADDR_R = crate::FieldReader;
#[doc = "Field `scfg_venc_remap_awaddr` writer - scfg_venc_remap_awaddr"]
pub type SCFG_VENC_REMAP_AWADDR_W<'a, REG, const O: u8> = crate::FieldWriter<'a, REG, 4, O>;
#[doc = "Field `scfg_vout0_remap_araddr` reader - scfg_vout0_remap_araddr"]
pub type SCFG_VOUT0_REMAP_ARADDR_R = crate::FieldReader;
#[doc = "Field `scfg_vout0_remap_araddr` writer - scfg_vout0_remap_araddr"]
pub type SCFG_VOUT0_REMAP_ARADDR_W<'a, REG, const O: u8> = crate::FieldWriter<'a, REG, 4, O>;
impl R {
    #[doc = "Bits 0:3 - scfg_sd1_remap_awaddr"]
    #[inline(always)]
    pub fn scfg_sd1_remap_awaddr(&self) -> SCFG_SD1_REMAP_AWADDR_R {
        SCFG_SD1_REMAP_AWADDR_R::new((self.bits & 0x0f) as u8)
    }
    #[doc = "Bits 4:7 - scfg_sec_haddr_remap"]
    #[inline(always)]
    pub fn scfg_sec_haddr_remap(&self) -> SCFG_SEC_HADDR_REMAP_R {
        SCFG_SEC_HADDR_REMAP_R::new(((self.bits >> 4) & 0x0f) as u8)
    }
    #[doc = "Bits 8:11 - scfg_usb_araddr_remap"]
    #[inline(always)]
    pub fn scfg_usb_araddr_remap(&self) -> SCFG_USB_ARADDR_REMAP_R {
        SCFG_USB_ARADDR_REMAP_R::new(((self.bits >> 8) & 0x0f) as u8)
    }
    #[doc = "Bits 12:15 - scfg_usb_awaddr_remap"]
    #[inline(always)]
    pub fn scfg_usb_awaddr_remap(&self) -> SCFG_USB_AWADDR_REMAP_R {
        SCFG_USB_AWADDR_REMAP_R::new(((self.bits >> 12) & 0x0f) as u8)
    }
    #[doc = "Bits 16:19 - scfg_vdec_remap_awaddr"]
    #[inline(always)]
    pub fn scfg_vdec_remap_awaddr(&self) -> SCFG_VDEC_REMAP_AWADDR_R {
        SCFG_VDEC_REMAP_AWADDR_R::new(((self.bits >> 16) & 0x0f) as u8)
    }
    #[doc = "Bits 20:23 - scfg_venc_remap_araddr"]
    #[inline(always)]
    pub fn scfg_venc_remap_araddr(&self) -> SCFG_VENC_REMAP_ARADDR_R {
        SCFG_VENC_REMAP_ARADDR_R::new(((self.bits >> 20) & 0x0f) as u8)
    }
    #[doc = "Bits 24:27 - scfg_venc_remap_awaddr"]
    #[inline(always)]
    pub fn scfg_venc_remap_awaddr(&self) -> SCFG_VENC_REMAP_AWADDR_R {
        SCFG_VENC_REMAP_AWADDR_R::new(((self.bits >> 24) & 0x0f) as u8)
    }
    #[doc = "Bits 28:31 - scfg_vout0_remap_araddr"]
    #[inline(always)]
    pub fn scfg_vout0_remap_araddr(&self) -> SCFG_VOUT0_REMAP_ARADDR_R {
        SCFG_VOUT0_REMAP_ARADDR_R::new(((self.bits >> 28) & 0x0f) as u8)
    }
}
impl W {
    #[doc = "Bits 0:3 - scfg_sd1_remap_awaddr"]
    #[inline(always)]
    #[must_use]
    pub fn scfg_sd1_remap_awaddr(
        &mut self,
    ) -> SCFG_SD1_REMAP_AWADDR_W<SYS_SYSCONSAIF_SYSCFG4_SPEC, 0> {
        SCFG_SD1_REMAP_AWADDR_W::new(self)
    }
    #[doc = "Bits 4:7 - scfg_sec_haddr_remap"]
    #[inline(always)]
    #[must_use]
    pub fn scfg_sec_haddr_remap(
        &mut self,
    ) -> SCFG_SEC_HADDR_REMAP_W<SYS_SYSCONSAIF_SYSCFG4_SPEC, 4> {
        SCFG_SEC_HADDR_REMAP_W::new(self)
    }
    #[doc = "Bits 8:11 - scfg_usb_araddr_remap"]
    #[inline(always)]
    #[must_use]
    pub fn scfg_usb_araddr_remap(
        &mut self,
    ) -> SCFG_USB_ARADDR_REMAP_W<SYS_SYSCONSAIF_SYSCFG4_SPEC, 8> {
        SCFG_USB_ARADDR_REMAP_W::new(self)
    }
    #[doc = "Bits 12:15 - scfg_usb_awaddr_remap"]
    #[inline(always)]
    #[must_use]
    pub fn scfg_usb_awaddr_remap(
        &mut self,
    ) -> SCFG_USB_AWADDR_REMAP_W<SYS_SYSCONSAIF_SYSCFG4_SPEC, 12> {
        SCFG_USB_AWADDR_REMAP_W::new(self)
    }
    #[doc = "Bits 16:19 - scfg_vdec_remap_awaddr"]
    #[inline(always)]
    #[must_use]
    pub fn scfg_vdec_remap_awaddr(
        &mut self,
    ) -> SCFG_VDEC_REMAP_AWADDR_W<SYS_SYSCONSAIF_SYSCFG4_SPEC, 16> {
        SCFG_VDEC_REMAP_AWADDR_W::new(self)
    }
    #[doc = "Bits 20:23 - scfg_venc_remap_araddr"]
    #[inline(always)]
    #[must_use]
    pub fn scfg_venc_remap_araddr(
        &mut self,
    ) -> SCFG_VENC_REMAP_ARADDR_W<SYS_SYSCONSAIF_SYSCFG4_SPEC, 20> {
        SCFG_VENC_REMAP_ARADDR_W::new(self)
    }
    #[doc = "Bits 24:27 - scfg_venc_remap_awaddr"]
    #[inline(always)]
    #[must_use]
    pub fn scfg_venc_remap_awaddr(
        &mut self,
    ) -> SCFG_VENC_REMAP_AWADDR_W<SYS_SYSCONSAIF_SYSCFG4_SPEC, 24> {
        SCFG_VENC_REMAP_AWADDR_W::new(self)
    }
    #[doc = "Bits 28:31 - scfg_vout0_remap_araddr"]
    #[inline(always)]
    #[must_use]
    pub fn scfg_vout0_remap_araddr(
        &mut self,
    ) -> SCFG_VOUT0_REMAP_ARADDR_W<SYS_SYSCONSAIF_SYSCFG4_SPEC, 28> {
        SCFG_VOUT0_REMAP_ARADDR_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.bits = bits;
        self
    }
}
#[doc = "SYS SYSCONSAIF SYSCFG 4\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`sys_sysconsaif_syscfg4::R`](R).  You can [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`sys_sysconsaif_syscfg4::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct SYS_SYSCONSAIF_SYSCFG4_SPEC;
impl crate::RegisterSpec for SYS_SYSCONSAIF_SYSCFG4_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`sys_sysconsaif_syscfg4::R`](R) reader structure"]
impl crate::Readable for SYS_SYSCONSAIF_SYSCFG4_SPEC {}
#[doc = "`write(|w| ..)` method takes [`sys_sysconsaif_syscfg4::W`](W) writer structure"]
impl crate::Writable for SYS_SYSCONSAIF_SYSCFG4_SPEC {
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}