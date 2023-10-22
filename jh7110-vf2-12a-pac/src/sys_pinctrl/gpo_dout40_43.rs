#[doc = "Register `gpo_dout40_43` reader"]
pub type R = crate::R<GPO_DOUT40_43_SPEC>;
#[doc = "Register `gpo_dout40_43` writer"]
pub type W = crate::W<GPO_DOUT40_43_SPEC>;
#[doc = "Field `gpo40_dout` reader - The selected output signal for GPIO40. The register value indicates the selected GPIO output index signal index from GPIO output signal list 0-107. See Table 2-41: GPIO OEN List for SYS_IOMUX (on page 97) for more information."]
pub type GPO40_DOUT_R = crate::FieldReader;
#[doc = "Field `gpo40_dout` writer - The selected output signal for GPIO40. The register value indicates the selected GPIO output index signal index from GPIO output signal list 0-107. See Table 2-41: GPIO OEN List for SYS_IOMUX (on page 97) for more information."]
pub type GPO40_DOUT_W<'a, REG, const O: u8> = crate::FieldWriter<'a, REG, 7, O>;
#[doc = "Field `gpo41_dout` reader - The selected output signal for GPIO41. The register value indicates the selected GPIO output index signal index from GPIO output signal list 0-107. See Table 2-41: GPIO OEN List for SYS_IOMUX (on page 97) for more information."]
pub type GPO41_DOUT_R = crate::FieldReader;
#[doc = "Field `gpo41_dout` writer - The selected output signal for GPIO41. The register value indicates the selected GPIO output index signal index from GPIO output signal list 0-107. See Table 2-41: GPIO OEN List for SYS_IOMUX (on page 97) for more information."]
pub type GPO41_DOUT_W<'a, REG, const O: u8> = crate::FieldWriter<'a, REG, 7, O>;
#[doc = "Field `gpo42_dout` reader - The selected output signal for GPIO42. The register value indicates the selected GPIO output index signal index from GPIO output signal list 0-107. See Table 2-41: GPIO OEN List for SYS_IOMUX (on page 97) for more information."]
pub type GPO42_DOUT_R = crate::FieldReader;
#[doc = "Field `gpo42_dout` writer - The selected output signal for GPIO42. The register value indicates the selected GPIO output index signal index from GPIO output signal list 0-107. See Table 2-41: GPIO OEN List for SYS_IOMUX (on page 97) for more information."]
pub type GPO42_DOUT_W<'a, REG, const O: u8> = crate::FieldWriter<'a, REG, 7, O>;
#[doc = "Field `gpo43_dout` reader - The selected output signal for GPIO43. The register value indicates the selected GPIO output index signal index from GPIO output signal list 0-107. See Table 2-41: GPIO OEN List for SYS_IOMUX (on page 97) for more information."]
pub type GPO43_DOUT_R = crate::FieldReader;
#[doc = "Field `gpo43_dout` writer - The selected output signal for GPIO43. The register value indicates the selected GPIO output index signal index from GPIO output signal list 0-107. See Table 2-41: GPIO OEN List for SYS_IOMUX (on page 97) for more information."]
pub type GPO43_DOUT_W<'a, REG, const O: u8> = crate::FieldWriter<'a, REG, 7, O>;
impl R {
    #[doc = "Bits 0:6 - The selected output signal for GPIO40. The register value indicates the selected GPIO output index signal index from GPIO output signal list 0-107. See Table 2-41: GPIO OEN List for SYS_IOMUX (on page 97) for more information."]
    #[inline(always)]
    pub fn gpo40_dout(&self) -> GPO40_DOUT_R {
        GPO40_DOUT_R::new((self.bits & 0x7f) as u8)
    }
    #[doc = "Bits 8:14 - The selected output signal for GPIO41. The register value indicates the selected GPIO output index signal index from GPIO output signal list 0-107. See Table 2-41: GPIO OEN List for SYS_IOMUX (on page 97) for more information."]
    #[inline(always)]
    pub fn gpo41_dout(&self) -> GPO41_DOUT_R {
        GPO41_DOUT_R::new(((self.bits >> 8) & 0x7f) as u8)
    }
    #[doc = "Bits 16:22 - The selected output signal for GPIO42. The register value indicates the selected GPIO output index signal index from GPIO output signal list 0-107. See Table 2-41: GPIO OEN List for SYS_IOMUX (on page 97) for more information."]
    #[inline(always)]
    pub fn gpo42_dout(&self) -> GPO42_DOUT_R {
        GPO42_DOUT_R::new(((self.bits >> 16) & 0x7f) as u8)
    }
    #[doc = "Bits 24:30 - The selected output signal for GPIO43. The register value indicates the selected GPIO output index signal index from GPIO output signal list 0-107. See Table 2-41: GPIO OEN List for SYS_IOMUX (on page 97) for more information."]
    #[inline(always)]
    pub fn gpo43_dout(&self) -> GPO43_DOUT_R {
        GPO43_DOUT_R::new(((self.bits >> 24) & 0x7f) as u8)
    }
}
impl W {
    #[doc = "Bits 0:6 - The selected output signal for GPIO40. The register value indicates the selected GPIO output index signal index from GPIO output signal list 0-107. See Table 2-41: GPIO OEN List for SYS_IOMUX (on page 97) for more information."]
    #[inline(always)]
    #[must_use]
    pub fn gpo40_dout(&mut self) -> GPO40_DOUT_W<GPO_DOUT40_43_SPEC, 0> {
        GPO40_DOUT_W::new(self)
    }
    #[doc = "Bits 8:14 - The selected output signal for GPIO41. The register value indicates the selected GPIO output index signal index from GPIO output signal list 0-107. See Table 2-41: GPIO OEN List for SYS_IOMUX (on page 97) for more information."]
    #[inline(always)]
    #[must_use]
    pub fn gpo41_dout(&mut self) -> GPO41_DOUT_W<GPO_DOUT40_43_SPEC, 8> {
        GPO41_DOUT_W::new(self)
    }
    #[doc = "Bits 16:22 - The selected output signal for GPIO42. The register value indicates the selected GPIO output index signal index from GPIO output signal list 0-107. See Table 2-41: GPIO OEN List for SYS_IOMUX (on page 97) for more information."]
    #[inline(always)]
    #[must_use]
    pub fn gpo42_dout(&mut self) -> GPO42_DOUT_W<GPO_DOUT40_43_SPEC, 16> {
        GPO42_DOUT_W::new(self)
    }
    #[doc = "Bits 24:30 - The selected output signal for GPIO43. The register value indicates the selected GPIO output index signal index from GPIO output signal list 0-107. See Table 2-41: GPIO OEN List for SYS_IOMUX (on page 97) for more information."]
    #[inline(always)]
    #[must_use]
    pub fn gpo43_dout(&mut self) -> GPO43_DOUT_W<GPO_DOUT40_43_SPEC, 24> {
        GPO43_DOUT_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.bits = bits;
        self
    }
}
#[doc = "SYS IOMUX CFG SAIF SYSCFG FMUX GPIO 40-43 DOUT\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`gpo_dout40_43::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`gpo_dout40_43::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct GPO_DOUT40_43_SPEC;
impl crate::RegisterSpec for GPO_DOUT40_43_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`gpo_dout40_43::R`](R) reader structure"]
impl crate::Readable for GPO_DOUT40_43_SPEC {}
#[doc = "`write(|w| ..)` method takes [`gpo_dout40_43::W`](W) writer structure"]
impl crate::Writable for GPO_DOUT40_43_SPEC {
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets gpo_dout40_43 to value 0x004e_4f00"]
impl crate::Resettable for GPO_DOUT40_43_SPEC {
    const RESET_VALUE: Self::Ux = 0x004e_4f00;
}
