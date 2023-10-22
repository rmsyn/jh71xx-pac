#[doc = "Register `gpo_dout4_7` reader"]
pub type R = crate::R<GPO_DOUT4_7_SPEC>;
#[doc = "Register `gpo_dout4_7` writer"]
pub type W = crate::W<GPO_DOUT4_7_SPEC>;
#[doc = "Field `gpo4_dout` reader - The selected output signal for GPIO4. The register value indicates the selected GPIO output index signal index from GPIO output signal list 0-107. See Table 2-41: GPIO OEN List for SYS_IOMUX (on page 97) for more information."]
pub type GPO4_DOUT_R = crate::FieldReader;
#[doc = "Field `gpo4_dout` writer - The selected output signal for GPIO4. The register value indicates the selected GPIO output index signal index from GPIO output signal list 0-107. See Table 2-41: GPIO OEN List for SYS_IOMUX (on page 97) for more information."]
pub type GPO4_DOUT_W<'a, REG, const O: u8> = crate::FieldWriter<'a, REG, 7, O>;
#[doc = "Field `gpo5_dout` reader - The selected output signal for GPIO5. The register value indicates the selected GPIO output index signal index from GPIO output signal list 0-107. See Table 2-41: GPIO OEN List for SYS_IOMUX (on page 97) for more information."]
pub type GPO5_DOUT_R = crate::FieldReader;
#[doc = "Field `gpo5_dout` writer - The selected output signal for GPIO5. The register value indicates the selected GPIO output index signal index from GPIO output signal list 0-107. See Table 2-41: GPIO OEN List for SYS_IOMUX (on page 97) for more information."]
pub type GPO5_DOUT_W<'a, REG, const O: u8> = crate::FieldWriter<'a, REG, 7, O>;
#[doc = "Field `gpo6_dout` reader - The selected output signal for GPIO6. The register value indicates the selected GPIO output index signal index from GPIO output signal list 0-107. See Table 2-41: GPIO OEN List for SYS_IOMUX (on page 97) for more information."]
pub type GPO6_DOUT_R = crate::FieldReader;
#[doc = "Field `gpo6_dout` writer - The selected output signal for GPIO6. The register value indicates the selected GPIO output index signal index from GPIO output signal list 0-107. See Table 2-41: GPIO OEN List for SYS_IOMUX (on page 97) for more information."]
pub type GPO6_DOUT_W<'a, REG, const O: u8> = crate::FieldWriter<'a, REG, 7, O>;
#[doc = "Field `gpo7_dout` reader - The selected output signal for GPIO7. The register value indicates the selected GPIO output index signal index from GPIO output signal list 0-107. See Table 2-41: GPIO OEN List for SYS_IOMUX (on page 97) for more information."]
pub type GPO7_DOUT_R = crate::FieldReader;
#[doc = "Field `gpo7_dout` writer - The selected output signal for GPIO7. The register value indicates the selected GPIO output index signal index from GPIO output signal list 0-107. See Table 2-41: GPIO OEN List for SYS_IOMUX (on page 97) for more information."]
pub type GPO7_DOUT_W<'a, REG, const O: u8> = crate::FieldWriter<'a, REG, 7, O>;
impl R {
    #[doc = "Bits 0:6 - The selected output signal for GPIO4. The register value indicates the selected GPIO output index signal index from GPIO output signal list 0-107. See Table 2-41: GPIO OEN List for SYS_IOMUX (on page 97) for more information."]
    #[inline(always)]
    pub fn gpo4_dout(&self) -> GPO4_DOUT_R {
        GPO4_DOUT_R::new((self.bits & 0x7f) as u8)
    }
    #[doc = "Bits 8:14 - The selected output signal for GPIO5. The register value indicates the selected GPIO output index signal index from GPIO output signal list 0-107. See Table 2-41: GPIO OEN List for SYS_IOMUX (on page 97) for more information."]
    #[inline(always)]
    pub fn gpo5_dout(&self) -> GPO5_DOUT_R {
        GPO5_DOUT_R::new(((self.bits >> 8) & 0x7f) as u8)
    }
    #[doc = "Bits 16:22 - The selected output signal for GPIO6. The register value indicates the selected GPIO output index signal index from GPIO output signal list 0-107. See Table 2-41: GPIO OEN List for SYS_IOMUX (on page 97) for more information."]
    #[inline(always)]
    pub fn gpo6_dout(&self) -> GPO6_DOUT_R {
        GPO6_DOUT_R::new(((self.bits >> 16) & 0x7f) as u8)
    }
    #[doc = "Bits 24:30 - The selected output signal for GPIO7. The register value indicates the selected GPIO output index signal index from GPIO output signal list 0-107. See Table 2-41: GPIO OEN List for SYS_IOMUX (on page 97) for more information."]
    #[inline(always)]
    pub fn gpo7_dout(&self) -> GPO7_DOUT_R {
        GPO7_DOUT_R::new(((self.bits >> 24) & 0x7f) as u8)
    }
}
impl W {
    #[doc = "Bits 0:6 - The selected output signal for GPIO4. The register value indicates the selected GPIO output index signal index from GPIO output signal list 0-107. See Table 2-41: GPIO OEN List for SYS_IOMUX (on page 97) for more information."]
    #[inline(always)]
    #[must_use]
    pub fn gpo4_dout(&mut self) -> GPO4_DOUT_W<GPO_DOUT4_7_SPEC, 0> {
        GPO4_DOUT_W::new(self)
    }
    #[doc = "Bits 8:14 - The selected output signal for GPIO5. The register value indicates the selected GPIO output index signal index from GPIO output signal list 0-107. See Table 2-41: GPIO OEN List for SYS_IOMUX (on page 97) for more information."]
    #[inline(always)]
    #[must_use]
    pub fn gpo5_dout(&mut self) -> GPO5_DOUT_W<GPO_DOUT4_7_SPEC, 8> {
        GPO5_DOUT_W::new(self)
    }
    #[doc = "Bits 16:22 - The selected output signal for GPIO6. The register value indicates the selected GPIO output index signal index from GPIO output signal list 0-107. See Table 2-41: GPIO OEN List for SYS_IOMUX (on page 97) for more information."]
    #[inline(always)]
    #[must_use]
    pub fn gpo6_dout(&mut self) -> GPO6_DOUT_W<GPO_DOUT4_7_SPEC, 16> {
        GPO6_DOUT_W::new(self)
    }
    #[doc = "Bits 24:30 - The selected output signal for GPIO7. The register value indicates the selected GPIO output index signal index from GPIO output signal list 0-107. See Table 2-41: GPIO OEN List for SYS_IOMUX (on page 97) for more information."]
    #[inline(always)]
    #[must_use]
    pub fn gpo7_dout(&mut self) -> GPO7_DOUT_W<GPO_DOUT4_7_SPEC, 24> {
        GPO7_DOUT_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.bits = bits;
        self
    }
}
#[doc = "SYS IOMUX CFG SAIF SYSCFG FMUX GPIO 4-7 DOUT\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`gpo_dout4_7::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`gpo_dout4_7::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct GPO_DOUT4_7_SPEC;
impl crate::RegisterSpec for GPO_DOUT4_7_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`gpo_dout4_7::R`](R) reader structure"]
impl crate::Readable for GPO_DOUT4_7_SPEC {}
#[doc = "`write(|w| ..)` method takes [`gpo_dout4_7::W`](W) writer structure"]
impl crate::Writable for GPO_DOUT4_7_SPEC {
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets gpo_dout4_7 to value 0x1400"]
impl crate::Resettable for GPO_DOUT4_7_SPEC {
    const RESET_VALUE: Self::Ux = 0x1400;
}
