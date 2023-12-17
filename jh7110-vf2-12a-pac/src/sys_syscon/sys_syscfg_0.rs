#[doc = "Register `sys_syscfg_0` reader"]
pub type R = crate::R<SYS_SYSCFG_0_SPEC>;
#[doc = "Register `sys_syscfg_0` writer"]
pub type W = crate::W<SYS_SYSCFG_0_SPEC>;
#[doc = "Field `e24_remap_haddr` reader - e24_remap_haddr"]
pub type E24_REMAP_HADDR_R = crate::FieldReader;
#[doc = "Field `e24_remap_haddr` writer - e24_remap_haddr"]
pub type E24_REMAP_HADDR_W<'a, REG> = crate::FieldWriter<'a, REG, 4>;
#[doc = "Field `hifi4_idma_remap_araddr` reader - hifi4_idma_remap_araddr"]
pub type HIFI4_IDMA_REMAP_ARADDR_R = crate::FieldReader;
#[doc = "Field `hifi4_idma_remap_araddr` writer - hifi4_idma_remap_araddr"]
pub type HIFI4_IDMA_REMAP_ARADDR_W<'a, REG> = crate::FieldWriter<'a, REG, 4>;
#[doc = "Field `hifi4_idma_remap_awaddr` reader - hifi4_idma_remap_awaddr"]
pub type HIFI4_IDMA_REMAP_AWADDR_R = crate::FieldReader;
#[doc = "Field `hifi4_idma_remap_awaddr` writer - hifi4_idma_remap_awaddr"]
pub type HIFI4_IDMA_REMAP_AWADDR_W<'a, REG> = crate::FieldWriter<'a, REG, 4>;
#[doc = "Field `hifi4_sys_remap_araddr` reader - hifi4_sys_remap_araddr"]
pub type HIFI4_SYS_REMAP_ARADDR_R = crate::FieldReader;
#[doc = "Field `hifi4_sys_remap_araddr` writer - hifi4_sys_remap_araddr"]
pub type HIFI4_SYS_REMAP_ARADDR_W<'a, REG> = crate::FieldWriter<'a, REG, 4>;
#[doc = "Field `hifi4_sys_remap_awaddr` reader - hifi4_sys_remap_awaddr"]
pub type HIFI4_SYS_REMAP_AWADDR_R = crate::FieldReader;
#[doc = "Field `hifi4_sys_remap_awaddr` writer - hifi4_sys_remap_awaddr"]
pub type HIFI4_SYS_REMAP_AWADDR_W<'a, REG> = crate::FieldWriter<'a, REG, 4>;
#[doc = "Field `jpg_remap_araddr` reader - jpg_remap_araddr"]
pub type JPG_REMAP_ARADDR_R = crate::FieldReader;
#[doc = "Field `jpg_remap_araddr` writer - jpg_remap_araddr"]
pub type JPG_REMAP_ARADDR_W<'a, REG> = crate::FieldWriter<'a, REG, 4>;
#[doc = "Field `jpg_remap_awaddr` reader - jpg_remap_awaddr"]
pub type JPG_REMAP_AWADDR_R = crate::FieldReader;
#[doc = "Field `jpg_remap_awaddr` writer - jpg_remap_awaddr"]
pub type JPG_REMAP_AWADDR_W<'a, REG> = crate::FieldWriter<'a, REG, 4>;
#[doc = "Field `sd0_remap_araddr` reader - sd0_remap_araddr"]
pub type SD0_REMAP_ARADDR_R = crate::FieldReader;
#[doc = "Field `sd0_remap_araddr` writer - sd0_remap_araddr"]
pub type SD0_REMAP_ARADDR_W<'a, REG> = crate::FieldWriter<'a, REG, 4>;
impl R {
    #[doc = "Bits 0:3 - e24_remap_haddr"]
    #[inline(always)]
    pub fn e24_remap_haddr(&self) -> E24_REMAP_HADDR_R {
        E24_REMAP_HADDR_R::new((self.bits & 0x0f) as u8)
    }
    #[doc = "Bits 4:7 - hifi4_idma_remap_araddr"]
    #[inline(always)]
    pub fn hifi4_idma_remap_araddr(&self) -> HIFI4_IDMA_REMAP_ARADDR_R {
        HIFI4_IDMA_REMAP_ARADDR_R::new(((self.bits >> 4) & 0x0f) as u8)
    }
    #[doc = "Bits 8:11 - hifi4_idma_remap_awaddr"]
    #[inline(always)]
    pub fn hifi4_idma_remap_awaddr(&self) -> HIFI4_IDMA_REMAP_AWADDR_R {
        HIFI4_IDMA_REMAP_AWADDR_R::new(((self.bits >> 8) & 0x0f) as u8)
    }
    #[doc = "Bits 12:15 - hifi4_sys_remap_araddr"]
    #[inline(always)]
    pub fn hifi4_sys_remap_araddr(&self) -> HIFI4_SYS_REMAP_ARADDR_R {
        HIFI4_SYS_REMAP_ARADDR_R::new(((self.bits >> 12) & 0x0f) as u8)
    }
    #[doc = "Bits 16:19 - hifi4_sys_remap_awaddr"]
    #[inline(always)]
    pub fn hifi4_sys_remap_awaddr(&self) -> HIFI4_SYS_REMAP_AWADDR_R {
        HIFI4_SYS_REMAP_AWADDR_R::new(((self.bits >> 16) & 0x0f) as u8)
    }
    #[doc = "Bits 20:23 - jpg_remap_araddr"]
    #[inline(always)]
    pub fn jpg_remap_araddr(&self) -> JPG_REMAP_ARADDR_R {
        JPG_REMAP_ARADDR_R::new(((self.bits >> 20) & 0x0f) as u8)
    }
    #[doc = "Bits 24:27 - jpg_remap_awaddr"]
    #[inline(always)]
    pub fn jpg_remap_awaddr(&self) -> JPG_REMAP_AWADDR_R {
        JPG_REMAP_AWADDR_R::new(((self.bits >> 24) & 0x0f) as u8)
    }
    #[doc = "Bits 28:31 - sd0_remap_araddr"]
    #[inline(always)]
    pub fn sd0_remap_araddr(&self) -> SD0_REMAP_ARADDR_R {
        SD0_REMAP_ARADDR_R::new(((self.bits >> 28) & 0x0f) as u8)
    }
}
impl W {
    #[doc = "Bits 0:3 - e24_remap_haddr"]
    #[inline(always)]
    #[must_use]
    pub fn e24_remap_haddr(&mut self) -> E24_REMAP_HADDR_W<SYS_SYSCFG_0_SPEC> {
        E24_REMAP_HADDR_W::new(self, 0)
    }
    #[doc = "Bits 4:7 - hifi4_idma_remap_araddr"]
    #[inline(always)]
    #[must_use]
    pub fn hifi4_idma_remap_araddr(&mut self) -> HIFI4_IDMA_REMAP_ARADDR_W<SYS_SYSCFG_0_SPEC> {
        HIFI4_IDMA_REMAP_ARADDR_W::new(self, 4)
    }
    #[doc = "Bits 8:11 - hifi4_idma_remap_awaddr"]
    #[inline(always)]
    #[must_use]
    pub fn hifi4_idma_remap_awaddr(&mut self) -> HIFI4_IDMA_REMAP_AWADDR_W<SYS_SYSCFG_0_SPEC> {
        HIFI4_IDMA_REMAP_AWADDR_W::new(self, 8)
    }
    #[doc = "Bits 12:15 - hifi4_sys_remap_araddr"]
    #[inline(always)]
    #[must_use]
    pub fn hifi4_sys_remap_araddr(&mut self) -> HIFI4_SYS_REMAP_ARADDR_W<SYS_SYSCFG_0_SPEC> {
        HIFI4_SYS_REMAP_ARADDR_W::new(self, 12)
    }
    #[doc = "Bits 16:19 - hifi4_sys_remap_awaddr"]
    #[inline(always)]
    #[must_use]
    pub fn hifi4_sys_remap_awaddr(&mut self) -> HIFI4_SYS_REMAP_AWADDR_W<SYS_SYSCFG_0_SPEC> {
        HIFI4_SYS_REMAP_AWADDR_W::new(self, 16)
    }
    #[doc = "Bits 20:23 - jpg_remap_araddr"]
    #[inline(always)]
    #[must_use]
    pub fn jpg_remap_araddr(&mut self) -> JPG_REMAP_ARADDR_W<SYS_SYSCFG_0_SPEC> {
        JPG_REMAP_ARADDR_W::new(self, 20)
    }
    #[doc = "Bits 24:27 - jpg_remap_awaddr"]
    #[inline(always)]
    #[must_use]
    pub fn jpg_remap_awaddr(&mut self) -> JPG_REMAP_AWADDR_W<SYS_SYSCFG_0_SPEC> {
        JPG_REMAP_AWADDR_W::new(self, 24)
    }
    #[doc = "Bits 28:31 - sd0_remap_araddr"]
    #[inline(always)]
    #[must_use]
    pub fn sd0_remap_araddr(&mut self) -> SD0_REMAP_ARADDR_W<SYS_SYSCFG_0_SPEC> {
        SD0_REMAP_ARADDR_W::new(self, 28)
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
#[doc = "SYS SYSCONSAIF SYSCFG 0\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`sys_syscfg_0::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`sys_syscfg_0::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct SYS_SYSCFG_0_SPEC;
impl crate::RegisterSpec for SYS_SYSCFG_0_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`sys_syscfg_0::R`](R) reader structure"]
impl crate::Readable for SYS_SYSCFG_0_SPEC {}
#[doc = "`write(|w| ..)` method takes [`sys_syscfg_0::W`](W) writer structure"]
impl crate::Writable for SYS_SYSCFG_0_SPEC {
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets sys_syscfg_0 to value 0"]
impl crate::Resettable for SYS_SYSCFG_0_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
