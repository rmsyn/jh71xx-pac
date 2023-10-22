#[doc = "Register `sys_sysconsaif_syscfg0` reader"]
pub type R = crate::R<SYS_SYSCONSAIF_SYSCFG0_SPEC>;
#[doc = "Register `sys_sysconsaif_syscfg0` writer"]
pub type W = crate::W<SYS_SYSCONSAIF_SYSCFG0_SPEC>;
#[doc = "Field `scfg_e24_remap_haddr` reader - scfg_e24_remap_haddr"]
pub type SCFG_E24_REMAP_HADDR_R = crate::FieldReader;
#[doc = "Field `scfg_e24_remap_haddr` writer - scfg_e24_remap_haddr"]
pub type SCFG_E24_REMAP_HADDR_W<'a, REG, const O: u8> = crate::FieldWriter<'a, REG, 4, O>;
#[doc = "Field `scfg_hifi4_idma_remap_araddr` reader - scfg_hifi4_idma_remap_araddr"]
pub type SCFG_HIFI4_IDMA_REMAP_ARADDR_R = crate::FieldReader;
#[doc = "Field `scfg_hifi4_idma_remap_araddr` writer - scfg_hifi4_idma_remap_araddr"]
pub type SCFG_HIFI4_IDMA_REMAP_ARADDR_W<'a, REG, const O: u8> = crate::FieldWriter<'a, REG, 4, O>;
#[doc = "Field `scfg_hifi4_idma_remap_awaddr` reader - scfg_hifi4_idma_remap_awaddr"]
pub type SCFG_HIFI4_IDMA_REMAP_AWADDR_R = crate::FieldReader;
#[doc = "Field `scfg_hifi4_idma_remap_awaddr` writer - scfg_hifi4_idma_remap_awaddr"]
pub type SCFG_HIFI4_IDMA_REMAP_AWADDR_W<'a, REG, const O: u8> = crate::FieldWriter<'a, REG, 4, O>;
#[doc = "Field `scfg_hifi4_sys_remap_araddr` reader - scfg_hifi4_sys_remap_araddr"]
pub type SCFG_HIFI4_SYS_REMAP_ARADDR_R = crate::FieldReader;
#[doc = "Field `scfg_hifi4_sys_remap_araddr` writer - scfg_hifi4_sys_remap_araddr"]
pub type SCFG_HIFI4_SYS_REMAP_ARADDR_W<'a, REG, const O: u8> = crate::FieldWriter<'a, REG, 4, O>;
#[doc = "Field `scfg_hifi4_sys_remap_awaddr` reader - scfg_hifi4_sys_remap_awaddr"]
pub type SCFG_HIFI4_SYS_REMAP_AWADDR_R = crate::FieldReader;
#[doc = "Field `scfg_hifi4_sys_remap_awaddr` writer - scfg_hifi4_sys_remap_awaddr"]
pub type SCFG_HIFI4_SYS_REMAP_AWADDR_W<'a, REG, const O: u8> = crate::FieldWriter<'a, REG, 4, O>;
#[doc = "Field `scfg_jpg_remap_araddr` reader - scfg_jpg_remap_araddr"]
pub type SCFG_JPG_REMAP_ARADDR_R = crate::FieldReader;
#[doc = "Field `scfg_jpg_remap_araddr` writer - scfg_jpg_remap_araddr"]
pub type SCFG_JPG_REMAP_ARADDR_W<'a, REG, const O: u8> = crate::FieldWriter<'a, REG, 4, O>;
#[doc = "Field `scfg_jpg_remap_awaddr` reader - scfg_jpg_remap_awaddr"]
pub type SCFG_JPG_REMAP_AWADDR_R = crate::FieldReader;
#[doc = "Field `scfg_jpg_remap_awaddr` writer - scfg_jpg_remap_awaddr"]
pub type SCFG_JPG_REMAP_AWADDR_W<'a, REG, const O: u8> = crate::FieldWriter<'a, REG, 4, O>;
#[doc = "Field `scfg_sd0_remap_araddr` reader - scfg_sd0_remap_araddr"]
pub type SCFG_SD0_REMAP_ARADDR_R = crate::FieldReader;
#[doc = "Field `scfg_sd0_remap_araddr` writer - scfg_sd0_remap_araddr"]
pub type SCFG_SD0_REMAP_ARADDR_W<'a, REG, const O: u8> = crate::FieldWriter<'a, REG, 4, O>;
impl R {
    #[doc = "Bits 0:3 - scfg_e24_remap_haddr"]
    #[inline(always)]
    pub fn scfg_e24_remap_haddr(&self) -> SCFG_E24_REMAP_HADDR_R {
        SCFG_E24_REMAP_HADDR_R::new((self.bits & 0x0f) as u8)
    }
    #[doc = "Bits 4:7 - scfg_hifi4_idma_remap_araddr"]
    #[inline(always)]
    pub fn scfg_hifi4_idma_remap_araddr(&self) -> SCFG_HIFI4_IDMA_REMAP_ARADDR_R {
        SCFG_HIFI4_IDMA_REMAP_ARADDR_R::new(((self.bits >> 4) & 0x0f) as u8)
    }
    #[doc = "Bits 8:11 - scfg_hifi4_idma_remap_awaddr"]
    #[inline(always)]
    pub fn scfg_hifi4_idma_remap_awaddr(&self) -> SCFG_HIFI4_IDMA_REMAP_AWADDR_R {
        SCFG_HIFI4_IDMA_REMAP_AWADDR_R::new(((self.bits >> 8) & 0x0f) as u8)
    }
    #[doc = "Bits 12:15 - scfg_hifi4_sys_remap_araddr"]
    #[inline(always)]
    pub fn scfg_hifi4_sys_remap_araddr(&self) -> SCFG_HIFI4_SYS_REMAP_ARADDR_R {
        SCFG_HIFI4_SYS_REMAP_ARADDR_R::new(((self.bits >> 12) & 0x0f) as u8)
    }
    #[doc = "Bits 16:19 - scfg_hifi4_sys_remap_awaddr"]
    #[inline(always)]
    pub fn scfg_hifi4_sys_remap_awaddr(&self) -> SCFG_HIFI4_SYS_REMAP_AWADDR_R {
        SCFG_HIFI4_SYS_REMAP_AWADDR_R::new(((self.bits >> 16) & 0x0f) as u8)
    }
    #[doc = "Bits 20:23 - scfg_jpg_remap_araddr"]
    #[inline(always)]
    pub fn scfg_jpg_remap_araddr(&self) -> SCFG_JPG_REMAP_ARADDR_R {
        SCFG_JPG_REMAP_ARADDR_R::new(((self.bits >> 20) & 0x0f) as u8)
    }
    #[doc = "Bits 24:27 - scfg_jpg_remap_awaddr"]
    #[inline(always)]
    pub fn scfg_jpg_remap_awaddr(&self) -> SCFG_JPG_REMAP_AWADDR_R {
        SCFG_JPG_REMAP_AWADDR_R::new(((self.bits >> 24) & 0x0f) as u8)
    }
    #[doc = "Bits 28:31 - scfg_sd0_remap_araddr"]
    #[inline(always)]
    pub fn scfg_sd0_remap_araddr(&self) -> SCFG_SD0_REMAP_ARADDR_R {
        SCFG_SD0_REMAP_ARADDR_R::new(((self.bits >> 28) & 0x0f) as u8)
    }
}
impl W {
    #[doc = "Bits 0:3 - scfg_e24_remap_haddr"]
    #[inline(always)]
    #[must_use]
    pub fn scfg_e24_remap_haddr(
        &mut self,
    ) -> SCFG_E24_REMAP_HADDR_W<SYS_SYSCONSAIF_SYSCFG0_SPEC, 0> {
        SCFG_E24_REMAP_HADDR_W::new(self)
    }
    #[doc = "Bits 4:7 - scfg_hifi4_idma_remap_araddr"]
    #[inline(always)]
    #[must_use]
    pub fn scfg_hifi4_idma_remap_araddr(
        &mut self,
    ) -> SCFG_HIFI4_IDMA_REMAP_ARADDR_W<SYS_SYSCONSAIF_SYSCFG0_SPEC, 4> {
        SCFG_HIFI4_IDMA_REMAP_ARADDR_W::new(self)
    }
    #[doc = "Bits 8:11 - scfg_hifi4_idma_remap_awaddr"]
    #[inline(always)]
    #[must_use]
    pub fn scfg_hifi4_idma_remap_awaddr(
        &mut self,
    ) -> SCFG_HIFI4_IDMA_REMAP_AWADDR_W<SYS_SYSCONSAIF_SYSCFG0_SPEC, 8> {
        SCFG_HIFI4_IDMA_REMAP_AWADDR_W::new(self)
    }
    #[doc = "Bits 12:15 - scfg_hifi4_sys_remap_araddr"]
    #[inline(always)]
    #[must_use]
    pub fn scfg_hifi4_sys_remap_araddr(
        &mut self,
    ) -> SCFG_HIFI4_SYS_REMAP_ARADDR_W<SYS_SYSCONSAIF_SYSCFG0_SPEC, 12> {
        SCFG_HIFI4_SYS_REMAP_ARADDR_W::new(self)
    }
    #[doc = "Bits 16:19 - scfg_hifi4_sys_remap_awaddr"]
    #[inline(always)]
    #[must_use]
    pub fn scfg_hifi4_sys_remap_awaddr(
        &mut self,
    ) -> SCFG_HIFI4_SYS_REMAP_AWADDR_W<SYS_SYSCONSAIF_SYSCFG0_SPEC, 16> {
        SCFG_HIFI4_SYS_REMAP_AWADDR_W::new(self)
    }
    #[doc = "Bits 20:23 - scfg_jpg_remap_araddr"]
    #[inline(always)]
    #[must_use]
    pub fn scfg_jpg_remap_araddr(
        &mut self,
    ) -> SCFG_JPG_REMAP_ARADDR_W<SYS_SYSCONSAIF_SYSCFG0_SPEC, 20> {
        SCFG_JPG_REMAP_ARADDR_W::new(self)
    }
    #[doc = "Bits 24:27 - scfg_jpg_remap_awaddr"]
    #[inline(always)]
    #[must_use]
    pub fn scfg_jpg_remap_awaddr(
        &mut self,
    ) -> SCFG_JPG_REMAP_AWADDR_W<SYS_SYSCONSAIF_SYSCFG0_SPEC, 24> {
        SCFG_JPG_REMAP_AWADDR_W::new(self)
    }
    #[doc = "Bits 28:31 - scfg_sd0_remap_araddr"]
    #[inline(always)]
    #[must_use]
    pub fn scfg_sd0_remap_araddr(
        &mut self,
    ) -> SCFG_SD0_REMAP_ARADDR_W<SYS_SYSCONSAIF_SYSCFG0_SPEC, 28> {
        SCFG_SD0_REMAP_ARADDR_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.bits = bits;
        self
    }
}
#[doc = "SYS SYSCONSAIF SYSCFG 0\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`sys_sysconsaif_syscfg0::R`](R).  You can [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`sys_sysconsaif_syscfg0::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct SYS_SYSCONSAIF_SYSCFG0_SPEC;
impl crate::RegisterSpec for SYS_SYSCONSAIF_SYSCFG0_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`sys_sysconsaif_syscfg0::R`](R) reader structure"]
impl crate::Readable for SYS_SYSCONSAIF_SYSCFG0_SPEC {}
#[doc = "`write(|w| ..)` method takes [`sys_sysconsaif_syscfg0::W`](W) writer structure"]
impl crate::Writable for SYS_SYSCONSAIF_SYSCFG0_SPEC {
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
