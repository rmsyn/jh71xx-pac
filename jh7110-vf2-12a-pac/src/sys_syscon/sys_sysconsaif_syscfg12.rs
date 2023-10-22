#[doc = "Register `sys_sysconsaif_syscfg12` reader"]
pub type R = crate::R<SYS_SYSCONSAIF_SYSCFG12_SPEC>;
#[doc = "Register `sys_sysconsaif_syscfg12` writer"]
pub type W = crate::W<SYS_SYSCONSAIF_SYSCFG12_SPEC>;
#[doc = "Field `scfg_vout0_remap_awaddr_gpio0` reader - 0: GPIO Group 0 (GPIO21-35) voltage select 3.3V, 1: GPIO Group 0 (GPIO21-35) voltage select 1.8V"]
pub type SCFG_VOUT0_REMAP_AWADDR_GPIO0_R = crate::BitReader;
#[doc = "Field `scfg_vout0_remap_awaddr_gpio0` writer - 0: GPIO Group 0 (GPIO21-35) voltage select 3.3V, 1: GPIO Group 0 (GPIO21-35) voltage select 1.8V"]
pub type SCFG_VOUT0_REMAP_AWADDR_GPIO0_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O>;
#[doc = "Field `scfg_vout0_remap_awaddr_gpio1` reader - 0: GPIO Group 1 (GPIO36-63) voltage select 3.3V, 1: GPIO Group 1 (GPIO36-63) voltage select 1.8V"]
pub type SCFG_VOUT0_REMAP_AWADDR_GPIO1_R = crate::BitReader;
#[doc = "Field `scfg_vout0_remap_awaddr_gpio1` writer - 0: GPIO Group 1 (GPIO36-63) voltage select 3.3V, 1: GPIO Group 1 (GPIO36-63) voltage select 1.8V"]
pub type SCFG_VOUT0_REMAP_AWADDR_GPIO1_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O>;
#[doc = "Field `scfg_vout0_remap_awaddr_gpio2` reader - 0: GPIO Group 2 (GPIO0-6) voltage select 3.3V, 1: GPIO Group 2 (GPIO0-6) voltage select 1.8V"]
pub type SCFG_VOUT0_REMAP_AWADDR_GPIO2_R = crate::BitReader;
#[doc = "Field `scfg_vout0_remap_awaddr_gpio2` writer - 0: GPIO Group 2 (GPIO0-6) voltage select 3.3V, 1: GPIO Group 2 (GPIO0-6) voltage select 1.8V"]
pub type SCFG_VOUT0_REMAP_AWADDR_GPIO2_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O>;
#[doc = "Field `scfg_vout0_remap_awaddr_gpio3` reader - 0: GPIO Group 3 (GPIO7-20) voltage select 3.3V, 1: GPIO Group 3 (GPIO7-20) voltage select 1.8V"]
pub type SCFG_VOUT0_REMAP_AWADDR_GPIO3_R = crate::BitReader;
#[doc = "Field `scfg_vout0_remap_awaddr_gpio3` writer - 0: GPIO Group 3 (GPIO7-20) voltage select 3.3V, 1: GPIO Group 3 (GPIO7-20) voltage select 1.8V"]
pub type SCFG_VOUT0_REMAP_AWADDR_GPIO3_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O>;
impl R {
    #[doc = "Bit 0 - 0: GPIO Group 0 (GPIO21-35) voltage select 3.3V, 1: GPIO Group 0 (GPIO21-35) voltage select 1.8V"]
    #[inline(always)]
    pub fn scfg_vout0_remap_awaddr_gpio0(&self) -> SCFG_VOUT0_REMAP_AWADDR_GPIO0_R {
        SCFG_VOUT0_REMAP_AWADDR_GPIO0_R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - 0: GPIO Group 1 (GPIO36-63) voltage select 3.3V, 1: GPIO Group 1 (GPIO36-63) voltage select 1.8V"]
    #[inline(always)]
    pub fn scfg_vout0_remap_awaddr_gpio1(&self) -> SCFG_VOUT0_REMAP_AWADDR_GPIO1_R {
        SCFG_VOUT0_REMAP_AWADDR_GPIO1_R::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - 0: GPIO Group 2 (GPIO0-6) voltage select 3.3V, 1: GPIO Group 2 (GPIO0-6) voltage select 1.8V"]
    #[inline(always)]
    pub fn scfg_vout0_remap_awaddr_gpio2(&self) -> SCFG_VOUT0_REMAP_AWADDR_GPIO2_R {
        SCFG_VOUT0_REMAP_AWADDR_GPIO2_R::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3 - 0: GPIO Group 3 (GPIO7-20) voltage select 3.3V, 1: GPIO Group 3 (GPIO7-20) voltage select 1.8V"]
    #[inline(always)]
    pub fn scfg_vout0_remap_awaddr_gpio3(&self) -> SCFG_VOUT0_REMAP_AWADDR_GPIO3_R {
        SCFG_VOUT0_REMAP_AWADDR_GPIO3_R::new(((self.bits >> 3) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - 0: GPIO Group 0 (GPIO21-35) voltage select 3.3V, 1: GPIO Group 0 (GPIO21-35) voltage select 1.8V"]
    #[inline(always)]
    #[must_use]
    pub fn scfg_vout0_remap_awaddr_gpio0(
        &mut self,
    ) -> SCFG_VOUT0_REMAP_AWADDR_GPIO0_W<SYS_SYSCONSAIF_SYSCFG12_SPEC, 0> {
        SCFG_VOUT0_REMAP_AWADDR_GPIO0_W::new(self)
    }
    #[doc = "Bit 1 - 0: GPIO Group 1 (GPIO36-63) voltage select 3.3V, 1: GPIO Group 1 (GPIO36-63) voltage select 1.8V"]
    #[inline(always)]
    #[must_use]
    pub fn scfg_vout0_remap_awaddr_gpio1(
        &mut self,
    ) -> SCFG_VOUT0_REMAP_AWADDR_GPIO1_W<SYS_SYSCONSAIF_SYSCFG12_SPEC, 1> {
        SCFG_VOUT0_REMAP_AWADDR_GPIO1_W::new(self)
    }
    #[doc = "Bit 2 - 0: GPIO Group 2 (GPIO0-6) voltage select 3.3V, 1: GPIO Group 2 (GPIO0-6) voltage select 1.8V"]
    #[inline(always)]
    #[must_use]
    pub fn scfg_vout0_remap_awaddr_gpio2(
        &mut self,
    ) -> SCFG_VOUT0_REMAP_AWADDR_GPIO2_W<SYS_SYSCONSAIF_SYSCFG12_SPEC, 2> {
        SCFG_VOUT0_REMAP_AWADDR_GPIO2_W::new(self)
    }
    #[doc = "Bit 3 - 0: GPIO Group 3 (GPIO7-20) voltage select 3.3V, 1: GPIO Group 3 (GPIO7-20) voltage select 1.8V"]
    #[inline(always)]
    #[must_use]
    pub fn scfg_vout0_remap_awaddr_gpio3(
        &mut self,
    ) -> SCFG_VOUT0_REMAP_AWADDR_GPIO3_W<SYS_SYSCONSAIF_SYSCFG12_SPEC, 3> {
        SCFG_VOUT0_REMAP_AWADDR_GPIO3_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.bits = bits;
        self
    }
}
#[doc = "SYS SYSCONSAIF SYSCFG 12: Set the GPIO voltage of all the 4 GPIO groups in this register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`sys_sysconsaif_syscfg12::R`](R).  You can [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`sys_sysconsaif_syscfg12::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct SYS_SYSCONSAIF_SYSCFG12_SPEC;
impl crate::RegisterSpec for SYS_SYSCONSAIF_SYSCFG12_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`sys_sysconsaif_syscfg12::R`](R) reader structure"]
impl crate::Readable for SYS_SYSCONSAIF_SYSCFG12_SPEC {}
#[doc = "`write(|w| ..)` method takes [`sys_sysconsaif_syscfg12::W`](W) writer structure"]
impl crate::Writable for SYS_SYSCONSAIF_SYSCFG12_SPEC {
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
