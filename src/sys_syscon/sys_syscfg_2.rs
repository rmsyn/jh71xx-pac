#[doc = "Register `sys_syscfg_2` reader"]
pub type R = crate::R<SYS_SYSCFG_2_SPEC>;
#[doc = "Register `sys_syscfg_2` writer"]
pub type W = crate::W<SYS_SYSCFG_2_SPEC>;
#[doc = "Field `vout0_remap_awaddr` reader - vout0_remap_awaddr"]
pub type VOUT0_REMAP_AWADDR_R = crate::FieldReader;
#[doc = "Field `vout0_remap_awaddr` writer - vout0_remap_awaddr"]
pub type VOUT0_REMAP_AWADDR_W<'a, REG> = crate::FieldWriter<'a, REG, 4>;
#[doc = "Field `vout1_remap_araddr` reader - vout1_remap_araddr"]
pub type VOUT1_REMAP_ARADDR_R = crate::FieldReader;
#[doc = "Field `vout1_remap_araddr` writer - vout1_remap_araddr"]
pub type VOUT1_REMAP_ARADDR_W<'a, REG> = crate::FieldWriter<'a, REG, 4>;
#[doc = "Field `vout1_remap_awaddr` reader - vout1_remap_awaddr"]
pub type VOUT1_REMAP_AWADDR_R = crate::FieldReader;
#[doc = "Field `vout1_remap_awaddr` writer - vout1_remap_awaddr"]
pub type VOUT1_REMAP_AWADDR_W<'a, REG> = crate::FieldWriter<'a, REG, 4>;
impl R {
    #[doc = "Bits 0:3 - vout0_remap_awaddr"]
    #[inline(always)]
    pub fn vout0_remap_awaddr(&self) -> VOUT0_REMAP_AWADDR_R {
        VOUT0_REMAP_AWADDR_R::new((self.bits & 0x0f) as u8)
    }
    #[doc = "Bits 4:7 - vout1_remap_araddr"]
    #[inline(always)]
    pub fn vout1_remap_araddr(&self) -> VOUT1_REMAP_ARADDR_R {
        VOUT1_REMAP_ARADDR_R::new(((self.bits >> 4) & 0x0f) as u8)
    }
    #[doc = "Bits 8:11 - vout1_remap_awaddr"]
    #[inline(always)]
    pub fn vout1_remap_awaddr(&self) -> VOUT1_REMAP_AWADDR_R {
        VOUT1_REMAP_AWADDR_R::new(((self.bits >> 8) & 0x0f) as u8)
    }
}
impl W {
    #[doc = "Bits 0:3 - vout0_remap_awaddr"]
    #[inline(always)]
    #[must_use]
    pub fn vout0_remap_awaddr(&mut self) -> VOUT0_REMAP_AWADDR_W<SYS_SYSCFG_2_SPEC> {
        VOUT0_REMAP_AWADDR_W::new(self, 0)
    }
    #[doc = "Bits 4:7 - vout1_remap_araddr"]
    #[inline(always)]
    #[must_use]
    pub fn vout1_remap_araddr(&mut self) -> VOUT1_REMAP_ARADDR_W<SYS_SYSCFG_2_SPEC> {
        VOUT1_REMAP_ARADDR_W::new(self, 4)
    }
    #[doc = "Bits 8:11 - vout1_remap_awaddr"]
    #[inline(always)]
    #[must_use]
    pub fn vout1_remap_awaddr(&mut self) -> VOUT1_REMAP_AWADDR_W<SYS_SYSCFG_2_SPEC> {
        VOUT1_REMAP_AWADDR_W::new(self, 8)
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
#[doc = "SYS SYSCONSAIF SYSCFG 8\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`sys_syscfg_2::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`sys_syscfg_2::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct SYS_SYSCFG_2_SPEC;
impl crate::RegisterSpec for SYS_SYSCFG_2_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`sys_syscfg_2::R`](R) reader structure"]
impl crate::Readable for SYS_SYSCFG_2_SPEC {}
#[doc = "`write(|w| ..)` method takes [`sys_syscfg_2::W`](W) writer structure"]
impl crate::Writable for SYS_SYSCFG_2_SPEC {
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets sys_syscfg_2 to value 0"]
impl crate::Resettable for SYS_SYSCFG_2_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
