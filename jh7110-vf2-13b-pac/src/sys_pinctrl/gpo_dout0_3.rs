#[doc = "Register `gpo_dout0_3` reader"]
pub type R = crate::R<GPO_DOUT0_3_SPEC>;
#[doc = "Register `gpo_dout0_3` writer"]
pub type W = crate::W<GPO_DOUT0_3_SPEC>;
#[doc = "Field `gpo0_dout` reader - The selected output signal for GPIO0. The register value indicates the selected GPIO output index signal index from GPIO output signal list 0-107. See Table 2-41: GPIO OEN List for SYS_IOMUX (on page 97) for more information."]
pub type GPO0_DOUT_R = crate::FieldReader;
#[doc = "Field `gpo0_dout` writer - The selected output signal for GPIO0. The register value indicates the selected GPIO output index signal index from GPIO output signal list 0-107. See Table 2-41: GPIO OEN List for SYS_IOMUX (on page 97) for more information."]
pub type GPO0_DOUT_W<'a, REG> = crate::FieldWriter<'a, REG, 7>;
#[doc = "Field `gpo1_dout` reader - The selected output signal for GPIO1. The register value indicates the selected GPIO output index signal index from GPIO output signal list 0-107. See Table 2-41: GPIO OEN List for SYS_IOMUX (on page 97) for more information."]
pub type GPO1_DOUT_R = crate::FieldReader;
#[doc = "Field `gpo1_dout` writer - The selected output signal for GPIO1. The register value indicates the selected GPIO output index signal index from GPIO output signal list 0-107. See Table 2-41: GPIO OEN List for SYS_IOMUX (on page 97) for more information."]
pub type GPO1_DOUT_W<'a, REG> = crate::FieldWriter<'a, REG, 7>;
#[doc = "Field `gpo2_dout` reader - The selected output signal for GPIO2. The register value indicates the selected GPIO output index signal index from GPIO output signal list 0-107. See Table 2-41: GPIO OEN List for SYS_IOMUX (on page 97) for more information."]
pub type GPO2_DOUT_R = crate::FieldReader;
#[doc = "Field `gpo2_dout` writer - The selected output signal for GPIO2. The register value indicates the selected GPIO output index signal index from GPIO output signal list 0-107. See Table 2-41: GPIO OEN List for SYS_IOMUX (on page 97) for more information."]
pub type GPO2_DOUT_W<'a, REG> = crate::FieldWriter<'a, REG, 7>;
#[doc = "Field `gpo3_dout` reader - The selected output signal for GPIO3. The register value indicates the selected GPIO output index signal index from GPIO output signal list 0-107. See Table 2-41: GPIO OEN List for SYS_IOMUX (on page 97) for more information."]
pub type GPO3_DOUT_R = crate::FieldReader;
#[doc = "Field `gpo3_dout` writer - The selected output signal for GPIO3. The register value indicates the selected GPIO output index signal index from GPIO output signal list 0-107. See Table 2-41: GPIO OEN List for SYS_IOMUX (on page 97) for more information."]
pub type GPO3_DOUT_W<'a, REG> = crate::FieldWriter<'a, REG, 7>;
impl R {
    #[doc = "Bits 0:6 - The selected output signal for GPIO0. The register value indicates the selected GPIO output index signal index from GPIO output signal list 0-107. See Table 2-41: GPIO OEN List for SYS_IOMUX (on page 97) for more information."]
    #[inline(always)]
    pub fn gpo0_dout(&self) -> GPO0_DOUT_R {
        GPO0_DOUT_R::new((self.bits & 0x7f) as u8)
    }
    #[doc = "Bits 8:14 - The selected output signal for GPIO1. The register value indicates the selected GPIO output index signal index from GPIO output signal list 0-107. See Table 2-41: GPIO OEN List for SYS_IOMUX (on page 97) for more information."]
    #[inline(always)]
    pub fn gpo1_dout(&self) -> GPO1_DOUT_R {
        GPO1_DOUT_R::new(((self.bits >> 8) & 0x7f) as u8)
    }
    #[doc = "Bits 16:22 - The selected output signal for GPIO2. The register value indicates the selected GPIO output index signal index from GPIO output signal list 0-107. See Table 2-41: GPIO OEN List for SYS_IOMUX (on page 97) for more information."]
    #[inline(always)]
    pub fn gpo2_dout(&self) -> GPO2_DOUT_R {
        GPO2_DOUT_R::new(((self.bits >> 16) & 0x7f) as u8)
    }
    #[doc = "Bits 24:30 - The selected output signal for GPIO3. The register value indicates the selected GPIO output index signal index from GPIO output signal list 0-107. See Table 2-41: GPIO OEN List for SYS_IOMUX (on page 97) for more information."]
    #[inline(always)]
    pub fn gpo3_dout(&self) -> GPO3_DOUT_R {
        GPO3_DOUT_R::new(((self.bits >> 24) & 0x7f) as u8)
    }
}
impl W {
    #[doc = "Bits 0:6 - The selected output signal for GPIO0. The register value indicates the selected GPIO output index signal index from GPIO output signal list 0-107. See Table 2-41: GPIO OEN List for SYS_IOMUX (on page 97) for more information."]
    #[inline(always)]
    #[must_use]
    pub fn gpo0_dout(&mut self) -> GPO0_DOUT_W<GPO_DOUT0_3_SPEC> {
        GPO0_DOUT_W::new(self, 0)
    }
    #[doc = "Bits 8:14 - The selected output signal for GPIO1. The register value indicates the selected GPIO output index signal index from GPIO output signal list 0-107. See Table 2-41: GPIO OEN List for SYS_IOMUX (on page 97) for more information."]
    #[inline(always)]
    #[must_use]
    pub fn gpo1_dout(&mut self) -> GPO1_DOUT_W<GPO_DOUT0_3_SPEC> {
        GPO1_DOUT_W::new(self, 8)
    }
    #[doc = "Bits 16:22 - The selected output signal for GPIO2. The register value indicates the selected GPIO output index signal index from GPIO output signal list 0-107. See Table 2-41: GPIO OEN List for SYS_IOMUX (on page 97) for more information."]
    #[inline(always)]
    #[must_use]
    pub fn gpo2_dout(&mut self) -> GPO2_DOUT_W<GPO_DOUT0_3_SPEC> {
        GPO2_DOUT_W::new(self, 16)
    }
    #[doc = "Bits 24:30 - The selected output signal for GPIO3. The register value indicates the selected GPIO output index signal index from GPIO output signal list 0-107. See Table 2-41: GPIO OEN List for SYS_IOMUX (on page 97) for more information."]
    #[inline(always)]
    #[must_use]
    pub fn gpo3_dout(&mut self) -> GPO3_DOUT_W<GPO_DOUT0_3_SPEC> {
        GPO3_DOUT_W::new(self, 24)
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
#[doc = "SYS IOMUX CFG SAIF SYSCFG FMUX GPIO 0-3 DOUT\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`gpo_dout0_3::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`gpo_dout0_3::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct GPO_DOUT0_3_SPEC;
impl crate::RegisterSpec for GPO_DOUT0_3_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`gpo_dout0_3::R`](R) reader structure"]
impl crate::Readable for GPO_DOUT0_3_SPEC {}
#[doc = "`write(|w| ..)` method takes [`gpo_dout0_3::W`](W) writer structure"]
impl crate::Writable for GPO_DOUT0_3_SPEC {
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets gpo_dout0_3 to value 0x1600_0000"]
impl crate::Resettable for GPO_DOUT0_3_SPEC {
    const RESET_VALUE: Self::Ux = 0x1600_0000;
}
