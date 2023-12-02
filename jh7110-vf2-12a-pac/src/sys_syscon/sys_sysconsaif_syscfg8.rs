#[doc = "Register `sys_sysconsaif_syscfg8` reader"]
pub type R = crate::R<SYS_SYSCONSAIF_SYSCFG8_SPEC>;
#[doc = "Register `sys_sysconsaif_syscfg8` writer"]
pub type W = crate::W<SYS_SYSCONSAIF_SYSCFG8_SPEC>;
#[doc = "Field `scfg_vout0_remap_awaddr` reader - scfg_vout0_remap_awaddr"]
pub type SCFG_VOUT0_REMAP_AWADDR_R = crate::FieldReader;
#[doc = "Field `scfg_vout0_remap_awaddr` writer - scfg_vout0_remap_awaddr"]
pub type SCFG_VOUT0_REMAP_AWADDR_W<'a, REG> = crate::FieldWriter<'a, REG, 4>;
#[doc = "Field `scfg_vout1_remap_araddr` reader - scfg_vout1_remap_araddr"]
pub type SCFG_VOUT1_REMAP_ARADDR_R = crate::FieldReader;
#[doc = "Field `scfg_vout1_remap_araddr` writer - scfg_vout1_remap_araddr"]
pub type SCFG_VOUT1_REMAP_ARADDR_W<'a, REG> = crate::FieldWriter<'a, REG, 4>;
#[doc = "Field `scfg_vout1_remap_awaddr` reader - scfg_vout1_remap_awaddr"]
pub type SCFG_VOUT1_REMAP_AWADDR_R = crate::FieldReader;
#[doc = "Field `scfg_vout1_remap_awaddr` writer - scfg_vout1_remap_awaddr"]
pub type SCFG_VOUT1_REMAP_AWADDR_W<'a, REG> = crate::FieldWriter<'a, REG, 4>;
impl R {
    #[doc = "Bits 0:3 - scfg_vout0_remap_awaddr"]
    #[inline(always)]
    pub fn scfg_vout0_remap_awaddr(&self) -> SCFG_VOUT0_REMAP_AWADDR_R {
        SCFG_VOUT0_REMAP_AWADDR_R::new((self.bits & 0x0f) as u8)
    }
    #[doc = "Bits 4:7 - scfg_vout1_remap_araddr"]
    #[inline(always)]
    pub fn scfg_vout1_remap_araddr(&self) -> SCFG_VOUT1_REMAP_ARADDR_R {
        SCFG_VOUT1_REMAP_ARADDR_R::new(((self.bits >> 4) & 0x0f) as u8)
    }
    #[doc = "Bits 8:11 - scfg_vout1_remap_awaddr"]
    #[inline(always)]
    pub fn scfg_vout1_remap_awaddr(&self) -> SCFG_VOUT1_REMAP_AWADDR_R {
        SCFG_VOUT1_REMAP_AWADDR_R::new(((self.bits >> 8) & 0x0f) as u8)
    }
}
impl W {
    #[doc = "Bits 0:3 - scfg_vout0_remap_awaddr"]
    #[inline(always)]
    #[must_use]
    pub fn scfg_vout0_remap_awaddr(
        &mut self,
    ) -> SCFG_VOUT0_REMAP_AWADDR_W<SYS_SYSCONSAIF_SYSCFG8_SPEC> {
        SCFG_VOUT0_REMAP_AWADDR_W::new(self, 0)
    }
    #[doc = "Bits 4:7 - scfg_vout1_remap_araddr"]
    #[inline(always)]
    #[must_use]
    pub fn scfg_vout1_remap_araddr(
        &mut self,
    ) -> SCFG_VOUT1_REMAP_ARADDR_W<SYS_SYSCONSAIF_SYSCFG8_SPEC> {
        SCFG_VOUT1_REMAP_ARADDR_W::new(self, 4)
    }
    #[doc = "Bits 8:11 - scfg_vout1_remap_awaddr"]
    #[inline(always)]
    #[must_use]
    pub fn scfg_vout1_remap_awaddr(
        &mut self,
    ) -> SCFG_VOUT1_REMAP_AWADDR_W<SYS_SYSCONSAIF_SYSCFG8_SPEC> {
        SCFG_VOUT1_REMAP_AWADDR_W::new(self, 8)
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
#[doc = "SYS SYSCONSAIF SYSCFG 8\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`sys_sysconsaif_syscfg8::R`](R).  You can [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`sys_sysconsaif_syscfg8::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct SYS_SYSCONSAIF_SYSCFG8_SPEC;
impl crate::RegisterSpec for SYS_SYSCONSAIF_SYSCFG8_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`sys_sysconsaif_syscfg8::R`](R) reader structure"]
impl crate::Readable for SYS_SYSCONSAIF_SYSCFG8_SPEC {}
#[doc = "`write(|w| ..)` method takes [`sys_sysconsaif_syscfg8::W`](W) writer structure"]
impl crate::Writable for SYS_SYSCONSAIF_SYSCFG8_SPEC {
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
