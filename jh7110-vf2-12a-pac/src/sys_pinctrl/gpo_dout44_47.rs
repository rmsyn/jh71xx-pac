#[doc = "Register `gpo_dout44_47` reader"]
pub type R = crate::R<GPO_DOUT44_47_SPEC>;
#[doc = "Register `gpo_dout44_47` writer"]
pub type W = crate::W<GPO_DOUT44_47_SPEC>;
#[doc = "Field `gpo44_dout` reader - The selected output signal for GPIO44. The register value indicates the selected GPIO output index signal index from GPIO output signal list 0-107. See Table 2-41: GPIO OEN List for SYS_IOMUX (on page 97) for more information."]
pub type GPO44_DOUT_R = crate::FieldReader;
#[doc = "Field `gpo44_dout` writer - The selected output signal for GPIO44. The register value indicates the selected GPIO output index signal index from GPIO output signal list 0-107. See Table 2-41: GPIO OEN List for SYS_IOMUX (on page 97) for more information."]
pub type GPO44_DOUT_W<'a, REG, const O: u8> = crate::FieldWriter<'a, REG, 7, O>;
#[doc = "Field `gpo45_dout` reader - The selected output signal for GPIO45. The register value indicates the selected GPIO output index signal index from GPIO output signal list 0-107. See Table 2-41: GPIO OEN List for SYS_IOMUX (on page 97) for more information."]
pub type GPO45_DOUT_R = crate::FieldReader;
#[doc = "Field `gpo45_dout` writer - The selected output signal for GPIO45. The register value indicates the selected GPIO output index signal index from GPIO output signal list 0-107. See Table 2-41: GPIO OEN List for SYS_IOMUX (on page 97) for more information."]
pub type GPO45_DOUT_W<'a, REG, const O: u8> = crate::FieldWriter<'a, REG, 7, O>;
#[doc = "Field `gpo46_dout` reader - The selected output signal for GPIO46. The register value indicates the selected GPIO output index signal index from GPIO output signal list 0-107. See Table 2-41: GPIO OEN List for SYS_IOMUX (on page 97) for more information."]
pub type GPO46_DOUT_R = crate::FieldReader;
#[doc = "Field `gpo46_dout` writer - The selected output signal for GPIO46. The register value indicates the selected GPIO output index signal index from GPIO output signal list 0-107. See Table 2-41: GPIO OEN List for SYS_IOMUX (on page 97) for more information."]
pub type GPO46_DOUT_W<'a, REG, const O: u8> = crate::FieldWriter<'a, REG, 7, O>;
#[doc = "Field `gpo47_dout` reader - The selected output signal for GPIO47. The register value indicates the selected GPIO output index signal index from GPIO output signal list 0-107. See Table 2-41: GPIO OEN List for SYS_IOMUX (on page 97) for more information."]
pub type GPO47_DOUT_R = crate::FieldReader;
#[doc = "Field `gpo47_dout` writer - The selected output signal for GPIO47. The register value indicates the selected GPIO output index signal index from GPIO output signal list 0-107. See Table 2-41: GPIO OEN List for SYS_IOMUX (on page 97) for more information."]
pub type GPO47_DOUT_W<'a, REG, const O: u8> = crate::FieldWriter<'a, REG, 7, O>;
impl R {
    #[doc = "Bits 0:6 - The selected output signal for GPIO44. The register value indicates the selected GPIO output index signal index from GPIO output signal list 0-107. See Table 2-41: GPIO OEN List for SYS_IOMUX (on page 97) for more information."]
    #[inline(always)]
    pub fn gpo44_dout(&self) -> GPO44_DOUT_R {
        GPO44_DOUT_R::new((self.bits & 0x7f) as u8)
    }
    #[doc = "Bits 8:14 - The selected output signal for GPIO45. The register value indicates the selected GPIO output index signal index from GPIO output signal list 0-107. See Table 2-41: GPIO OEN List for SYS_IOMUX (on page 97) for more information."]
    #[inline(always)]
    pub fn gpo45_dout(&self) -> GPO45_DOUT_R {
        GPO45_DOUT_R::new(((self.bits >> 8) & 0x7f) as u8)
    }
    #[doc = "Bits 16:22 - The selected output signal for GPIO46. The register value indicates the selected GPIO output index signal index from GPIO output signal list 0-107. See Table 2-41: GPIO OEN List for SYS_IOMUX (on page 97) for more information."]
    #[inline(always)]
    pub fn gpo46_dout(&self) -> GPO46_DOUT_R {
        GPO46_DOUT_R::new(((self.bits >> 16) & 0x7f) as u8)
    }
    #[doc = "Bits 24:30 - The selected output signal for GPIO47. The register value indicates the selected GPIO output index signal index from GPIO output signal list 0-107. See Table 2-41: GPIO OEN List for SYS_IOMUX (on page 97) for more information."]
    #[inline(always)]
    pub fn gpo47_dout(&self) -> GPO47_DOUT_R {
        GPO47_DOUT_R::new(((self.bits >> 24) & 0x7f) as u8)
    }
}
impl W {
    #[doc = "Bits 0:6 - The selected output signal for GPIO44. The register value indicates the selected GPIO output index signal index from GPIO output signal list 0-107. See Table 2-41: GPIO OEN List for SYS_IOMUX (on page 97) for more information."]
    #[inline(always)]
    #[must_use]
    pub fn gpo44_dout(&mut self) -> GPO44_DOUT_W<GPO_DOUT44_47_SPEC, 0> {
        GPO44_DOUT_W::new(self)
    }
    #[doc = "Bits 8:14 - The selected output signal for GPIO45. The register value indicates the selected GPIO output index signal index from GPIO output signal list 0-107. See Table 2-41: GPIO OEN List for SYS_IOMUX (on page 97) for more information."]
    #[inline(always)]
    #[must_use]
    pub fn gpo45_dout(&mut self) -> GPO45_DOUT_W<GPO_DOUT44_47_SPEC, 8> {
        GPO45_DOUT_W::new(self)
    }
    #[doc = "Bits 16:22 - The selected output signal for GPIO46. The register value indicates the selected GPIO output index signal index from GPIO output signal list 0-107. See Table 2-41: GPIO OEN List for SYS_IOMUX (on page 97) for more information."]
    #[inline(always)]
    #[must_use]
    pub fn gpo46_dout(&mut self) -> GPO46_DOUT_W<GPO_DOUT44_47_SPEC, 16> {
        GPO46_DOUT_W::new(self)
    }
    #[doc = "Bits 24:30 - The selected output signal for GPIO47. The register value indicates the selected GPIO output index signal index from GPIO output signal list 0-107. See Table 2-41: GPIO OEN List for SYS_IOMUX (on page 97) for more information."]
    #[inline(always)]
    #[must_use]
    pub fn gpo47_dout(&mut self) -> GPO47_DOUT_W<GPO_DOUT44_47_SPEC, 24> {
        GPO47_DOUT_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.bits = bits;
        self
    }
}
#[doc = "SYS IOMUX CFG SAIF SYSCFG FMUX GPIO 44-47 DOUT\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`gpo_dout44_47::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`gpo_dout44_47::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct GPO_DOUT44_47_SPEC;
impl crate::RegisterSpec for GPO_DOUT44_47_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`gpo_dout44_47::R`](R) reader structure"]
impl crate::Readable for GPO_DOUT44_47_SPEC {}
#[doc = "`write(|w| ..)` method takes [`gpo_dout44_47::W`](W) writer structure"]
impl crate::Writable for GPO_DOUT44_47_SPEC {
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets gpo_dout44_47 to value 0x005b_5c00"]
impl crate::Resettable for GPO_DOUT44_47_SPEC {
    const RESET_VALUE: Self::Ux = 0x005b_5c00;
}
