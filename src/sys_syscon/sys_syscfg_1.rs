#[doc = "Register `sys_syscfg_1` reader"]
pub type R = crate::R<SYS_SYSCFG_1_SPEC>;
#[doc = "Register `sys_syscfg_1` writer"]
pub type W = crate::W<SYS_SYSCFG_1_SPEC>;
#[doc = "Field `sd1_remap_awaddr` reader - sd1_remap_awaddr"]
pub type SD1_REMAP_AWADDR_R = crate::FieldReader;
#[doc = "Field `sd1_remap_awaddr` writer - sd1_remap_awaddr"]
pub type SD1_REMAP_AWADDR_W<'a, REG> = crate::FieldWriter<'a, REG, 4>;
#[doc = "Field `sec_haddr_remap` reader - sec_haddr_remap"]
pub type SEC_HADDR_REMAP_R = crate::FieldReader;
#[doc = "Field `sec_haddr_remap` writer - sec_haddr_remap"]
pub type SEC_HADDR_REMAP_W<'a, REG> = crate::FieldWriter<'a, REG, 4>;
#[doc = "Field `usb_araddr_remap` reader - usb_araddr_remap"]
pub type USB_ARADDR_REMAP_R = crate::FieldReader;
#[doc = "Field `usb_araddr_remap` writer - usb_araddr_remap"]
pub type USB_ARADDR_REMAP_W<'a, REG> = crate::FieldWriter<'a, REG, 4>;
#[doc = "Field `usb_awaddr_remap` reader - usb_awaddr_remap"]
pub type USB_AWADDR_REMAP_R = crate::FieldReader;
#[doc = "Field `usb_awaddr_remap` writer - usb_awaddr_remap"]
pub type USB_AWADDR_REMAP_W<'a, REG> = crate::FieldWriter<'a, REG, 4>;
#[doc = "Field `vdec_remap_awaddr` reader - vdec_remap_awaddr"]
pub type VDEC_REMAP_AWADDR_R = crate::FieldReader;
#[doc = "Field `vdec_remap_awaddr` writer - vdec_remap_awaddr"]
pub type VDEC_REMAP_AWADDR_W<'a, REG> = crate::FieldWriter<'a, REG, 4>;
#[doc = "Field `venc_remap_araddr` reader - venc_remap_araddr"]
pub type VENC_REMAP_ARADDR_R = crate::FieldReader;
#[doc = "Field `venc_remap_araddr` writer - venc_remap_araddr"]
pub type VENC_REMAP_ARADDR_W<'a, REG> = crate::FieldWriter<'a, REG, 4>;
#[doc = "Field `venc_remap_awaddr` reader - venc_remap_awaddr"]
pub type VENC_REMAP_AWADDR_R = crate::FieldReader;
#[doc = "Field `venc_remap_awaddr` writer - venc_remap_awaddr"]
pub type VENC_REMAP_AWADDR_W<'a, REG> = crate::FieldWriter<'a, REG, 4>;
#[doc = "Field `vout0_remap_araddr` reader - vout0_remap_araddr"]
pub type VOUT0_REMAP_ARADDR_R = crate::FieldReader;
#[doc = "Field `vout0_remap_araddr` writer - vout0_remap_araddr"]
pub type VOUT0_REMAP_ARADDR_W<'a, REG> = crate::FieldWriter<'a, REG, 4>;
impl R {
    #[doc = "Bits 0:3 - sd1_remap_awaddr"]
    #[inline(always)]
    pub fn sd1_remap_awaddr(&self) -> SD1_REMAP_AWADDR_R {
        SD1_REMAP_AWADDR_R::new((self.bits & 0x0f) as u8)
    }
    #[doc = "Bits 4:7 - sec_haddr_remap"]
    #[inline(always)]
    pub fn sec_haddr_remap(&self) -> SEC_HADDR_REMAP_R {
        SEC_HADDR_REMAP_R::new(((self.bits >> 4) & 0x0f) as u8)
    }
    #[doc = "Bits 8:11 - usb_araddr_remap"]
    #[inline(always)]
    pub fn usb_araddr_remap(&self) -> USB_ARADDR_REMAP_R {
        USB_ARADDR_REMAP_R::new(((self.bits >> 8) & 0x0f) as u8)
    }
    #[doc = "Bits 12:15 - usb_awaddr_remap"]
    #[inline(always)]
    pub fn usb_awaddr_remap(&self) -> USB_AWADDR_REMAP_R {
        USB_AWADDR_REMAP_R::new(((self.bits >> 12) & 0x0f) as u8)
    }
    #[doc = "Bits 16:19 - vdec_remap_awaddr"]
    #[inline(always)]
    pub fn vdec_remap_awaddr(&self) -> VDEC_REMAP_AWADDR_R {
        VDEC_REMAP_AWADDR_R::new(((self.bits >> 16) & 0x0f) as u8)
    }
    #[doc = "Bits 20:23 - venc_remap_araddr"]
    #[inline(always)]
    pub fn venc_remap_araddr(&self) -> VENC_REMAP_ARADDR_R {
        VENC_REMAP_ARADDR_R::new(((self.bits >> 20) & 0x0f) as u8)
    }
    #[doc = "Bits 24:27 - venc_remap_awaddr"]
    #[inline(always)]
    pub fn venc_remap_awaddr(&self) -> VENC_REMAP_AWADDR_R {
        VENC_REMAP_AWADDR_R::new(((self.bits >> 24) & 0x0f) as u8)
    }
    #[doc = "Bits 28:31 - vout0_remap_araddr"]
    #[inline(always)]
    pub fn vout0_remap_araddr(&self) -> VOUT0_REMAP_ARADDR_R {
        VOUT0_REMAP_ARADDR_R::new(((self.bits >> 28) & 0x0f) as u8)
    }
}
impl W {
    #[doc = "Bits 0:3 - sd1_remap_awaddr"]
    #[inline(always)]
    #[must_use]
    pub fn sd1_remap_awaddr(&mut self) -> SD1_REMAP_AWADDR_W<SYS_SYSCFG_1_SPEC> {
        SD1_REMAP_AWADDR_W::new(self, 0)
    }
    #[doc = "Bits 4:7 - sec_haddr_remap"]
    #[inline(always)]
    #[must_use]
    pub fn sec_haddr_remap(&mut self) -> SEC_HADDR_REMAP_W<SYS_SYSCFG_1_SPEC> {
        SEC_HADDR_REMAP_W::new(self, 4)
    }
    #[doc = "Bits 8:11 - usb_araddr_remap"]
    #[inline(always)]
    #[must_use]
    pub fn usb_araddr_remap(&mut self) -> USB_ARADDR_REMAP_W<SYS_SYSCFG_1_SPEC> {
        USB_ARADDR_REMAP_W::new(self, 8)
    }
    #[doc = "Bits 12:15 - usb_awaddr_remap"]
    #[inline(always)]
    #[must_use]
    pub fn usb_awaddr_remap(&mut self) -> USB_AWADDR_REMAP_W<SYS_SYSCFG_1_SPEC> {
        USB_AWADDR_REMAP_W::new(self, 12)
    }
    #[doc = "Bits 16:19 - vdec_remap_awaddr"]
    #[inline(always)]
    #[must_use]
    pub fn vdec_remap_awaddr(&mut self) -> VDEC_REMAP_AWADDR_W<SYS_SYSCFG_1_SPEC> {
        VDEC_REMAP_AWADDR_W::new(self, 16)
    }
    #[doc = "Bits 20:23 - venc_remap_araddr"]
    #[inline(always)]
    #[must_use]
    pub fn venc_remap_araddr(&mut self) -> VENC_REMAP_ARADDR_W<SYS_SYSCFG_1_SPEC> {
        VENC_REMAP_ARADDR_W::new(self, 20)
    }
    #[doc = "Bits 24:27 - venc_remap_awaddr"]
    #[inline(always)]
    #[must_use]
    pub fn venc_remap_awaddr(&mut self) -> VENC_REMAP_AWADDR_W<SYS_SYSCFG_1_SPEC> {
        VENC_REMAP_AWADDR_W::new(self, 24)
    }
    #[doc = "Bits 28:31 - vout0_remap_araddr"]
    #[inline(always)]
    #[must_use]
    pub fn vout0_remap_araddr(&mut self) -> VOUT0_REMAP_ARADDR_W<SYS_SYSCFG_1_SPEC> {
        VOUT0_REMAP_ARADDR_W::new(self, 28)
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
#[doc = "SYS SYSCONSAIF SYSCFG 4\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`sys_syscfg_1::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`sys_syscfg_1::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct SYS_SYSCFG_1_SPEC;
impl crate::RegisterSpec for SYS_SYSCFG_1_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`sys_syscfg_1::R`](R) reader structure"]
impl crate::Readable for SYS_SYSCFG_1_SPEC {}
#[doc = "`write(|w| ..)` method takes [`sys_syscfg_1::W`](W) writer structure"]
impl crate::Writable for SYS_SYSCFG_1_SPEC {
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets sys_syscfg_1 to value 0"]
impl crate::Resettable for SYS_SYSCFG_1_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
