#[doc = "Register `gpo_dout12_15` reader"]
pub type R = crate::R<GPO_DOUT12_15_SPEC>;
#[doc = "Register `gpo_dout12_15` writer"]
pub type W = crate::W<GPO_DOUT12_15_SPEC>;
#[doc = "Field `gpo12_dout` reader - The selected output signal for GPIO12. The register value indicates the selected GPIO output index signal index from GPIO output signal list 0-107. See Table 2-41: GPIO OEN List for SYS_IOMUX (on page 97) for more information."]
pub type GPO12_DOUT_R = crate::FieldReader;
#[doc = "Field `gpo12_dout` writer - The selected output signal for GPIO12. The register value indicates the selected GPIO output index signal index from GPIO output signal list 0-107. See Table 2-41: GPIO OEN List for SYS_IOMUX (on page 97) for more information."]
pub type GPO12_DOUT_W<'a, REG, const O: u8> = crate::FieldWriter<'a, REG, 7, O>;
#[doc = "Field `gpo13_dout` reader - The selected output signal for GPIO13. The register value indicates the selected GPIO output index signal index from GPIO output signal list 0-107. See Table 2-41: GPIO OEN List for SYS_IOMUX (on page 97) for more information."]
pub type GPO13_DOUT_R = crate::FieldReader;
#[doc = "Field `gpo13_dout` writer - The selected output signal for GPIO13. The register value indicates the selected GPIO output index signal index from GPIO output signal list 0-107. See Table 2-41: GPIO OEN List for SYS_IOMUX (on page 97) for more information."]
pub type GPO13_DOUT_W<'a, REG, const O: u8> = crate::FieldWriter<'a, REG, 7, O>;
#[doc = "Field `gpo14_dout` reader - The selected output signal for GPIO14. The register value indicates the selected GPIO output index signal index from GPIO output signal list 0-107. See Table 2-41: GPIO OEN List for SYS_IOMUX (on page 97) for more information."]
pub type GPO14_DOUT_R = crate::FieldReader;
#[doc = "Field `gpo14_dout` writer - The selected output signal for GPIO14. The register value indicates the selected GPIO output index signal index from GPIO output signal list 0-107. See Table 2-41: GPIO OEN List for SYS_IOMUX (on page 97) for more information."]
pub type GPO14_DOUT_W<'a, REG, const O: u8> = crate::FieldWriter<'a, REG, 7, O>;
#[doc = "Field `gpo15_dout` reader - The selected output signal for GPIO15. The register value indicates the selected GPIO output index signal index from GPIO output signal list 0-107. See Table 2-41: GPIO OEN List for SYS_IOMUX (on page 97) for more information."]
pub type GPO15_DOUT_R = crate::FieldReader;
#[doc = "Field `gpo15_dout` writer - The selected output signal for GPIO15. The register value indicates the selected GPIO output index signal index from GPIO output signal list 0-107. See Table 2-41: GPIO OEN List for SYS_IOMUX (on page 97) for more information."]
pub type GPO15_DOUT_W<'a, REG, const O: u8> = crate::FieldWriter<'a, REG, 7, O>;
impl R {
    #[doc = "Bits 0:6 - The selected output signal for GPIO12. The register value indicates the selected GPIO output index signal index from GPIO output signal list 0-107. See Table 2-41: GPIO OEN List for SYS_IOMUX (on page 97) for more information."]
    #[inline(always)]
    pub fn gpo12_dout(&self) -> GPO12_DOUT_R {
        GPO12_DOUT_R::new((self.bits & 0x7f) as u8)
    }
    #[doc = "Bits 8:14 - The selected output signal for GPIO13. The register value indicates the selected GPIO output index signal index from GPIO output signal list 0-107. See Table 2-41: GPIO OEN List for SYS_IOMUX (on page 97) for more information."]
    #[inline(always)]
    pub fn gpo13_dout(&self) -> GPO13_DOUT_R {
        GPO13_DOUT_R::new(((self.bits >> 8) & 0x7f) as u8)
    }
    #[doc = "Bits 16:22 - The selected output signal for GPIO14. The register value indicates the selected GPIO output index signal index from GPIO output signal list 0-107. See Table 2-41: GPIO OEN List for SYS_IOMUX (on page 97) for more information."]
    #[inline(always)]
    pub fn gpo14_dout(&self) -> GPO14_DOUT_R {
        GPO14_DOUT_R::new(((self.bits >> 16) & 0x7f) as u8)
    }
    #[doc = "Bits 24:30 - The selected output signal for GPIO15. The register value indicates the selected GPIO output index signal index from GPIO output signal list 0-107. See Table 2-41: GPIO OEN List for SYS_IOMUX (on page 97) for more information."]
    #[inline(always)]
    pub fn gpo15_dout(&self) -> GPO15_DOUT_R {
        GPO15_DOUT_R::new(((self.bits >> 24) & 0x7f) as u8)
    }
}
impl W {
    #[doc = "Bits 0:6 - The selected output signal for GPIO12. The register value indicates the selected GPIO output index signal index from GPIO output signal list 0-107. See Table 2-41: GPIO OEN List for SYS_IOMUX (on page 97) for more information."]
    #[inline(always)]
    #[must_use]
    pub fn gpo12_dout(&mut self) -> GPO12_DOUT_W<GPO_DOUT12_15_SPEC, 0> {
        GPO12_DOUT_W::new(self)
    }
    #[doc = "Bits 8:14 - The selected output signal for GPIO13. The register value indicates the selected GPIO output index signal index from GPIO output signal list 0-107. See Table 2-41: GPIO OEN List for SYS_IOMUX (on page 97) for more information."]
    #[inline(always)]
    #[must_use]
    pub fn gpo13_dout(&mut self) -> GPO13_DOUT_W<GPO_DOUT12_15_SPEC, 8> {
        GPO13_DOUT_W::new(self)
    }
    #[doc = "Bits 16:22 - The selected output signal for GPIO14. The register value indicates the selected GPIO output index signal index from GPIO output signal list 0-107. See Table 2-41: GPIO OEN List for SYS_IOMUX (on page 97) for more information."]
    #[inline(always)]
    #[must_use]
    pub fn gpo14_dout(&mut self) -> GPO14_DOUT_W<GPO_DOUT12_15_SPEC, 16> {
        GPO14_DOUT_W::new(self)
    }
    #[doc = "Bits 24:30 - The selected output signal for GPIO15. The register value indicates the selected GPIO output index signal index from GPIO output signal list 0-107. See Table 2-41: GPIO OEN List for SYS_IOMUX (on page 97) for more information."]
    #[inline(always)]
    #[must_use]
    pub fn gpo15_dout(&mut self) -> GPO15_DOUT_W<GPO_DOUT12_15_SPEC, 24> {
        GPO15_DOUT_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.bits = bits;
        self
    }
}
#[doc = "SYS IOMUX CFG SAIF SYSCFG FMUX GPIO 12-15 DOUT\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`gpo_dout12_15::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`gpo_dout12_15::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct GPO_DOUT12_15_SPEC;
impl crate::RegisterSpec for GPO_DOUT12_15_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`gpo_dout12_15::R`](R) reader structure"]
impl crate::Readable for GPO_DOUT12_15_SPEC {}
#[doc = "`write(|w| ..)` method takes [`gpo_dout12_15::W`](W) writer structure"]
impl crate::Writable for GPO_DOUT12_15_SPEC {
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets gpo_dout12_15 to value 0"]
impl crate::Resettable for GPO_DOUT12_15_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
