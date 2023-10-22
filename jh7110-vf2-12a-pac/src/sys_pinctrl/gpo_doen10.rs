#[doc = "Register `gpo_doen10` reader"]
pub type R = crate::R<GPO_DOEN10_SPEC>;
#[doc = "Register `gpo_doen10` writer"]
pub type W = crate::W<GPO_DOEN10_SPEC>;
#[doc = "Field `gpo40_doen` reader - The selected OEN signal for GPIO40. The register value indicates the selected GPIO (Output Enable) OEN index from GPIO OEN list 0-49. See Table 2-41: GPIO OEN List for SYS_IOMUX (on page 97) for more information."]
pub type GPO40_DOEN_R = crate::FieldReader;
#[doc = "Field `gpo40_doen` writer - The selected OEN signal for GPIO40. The register value indicates the selected GPIO (Output Enable) OEN index from GPIO OEN list 0-49. See Table 2-41: GPIO OEN List for SYS_IOMUX (on page 97) for more information."]
pub type GPO40_DOEN_W<'a, REG, const O: u8> = crate::FieldWriter<'a, REG, 6, O>;
#[doc = "Field `gpo41_doen` reader - The selected OEN signal for GPIO41. The register value indicates the selected GPIO (Output Enable) OEN index from GPIO OEN list 0-49. See Table 2-41: GPIO OEN List for SYS_IOMUX (on page 97) for more information."]
pub type GPO41_DOEN_R = crate::FieldReader;
#[doc = "Field `gpo41_doen` writer - The selected OEN signal for GPIO41. The register value indicates the selected GPIO (Output Enable) OEN index from GPIO OEN list 0-49. See Table 2-41: GPIO OEN List for SYS_IOMUX (on page 97) for more information."]
pub type GPO41_DOEN_W<'a, REG, const O: u8> = crate::FieldWriter<'a, REG, 6, O>;
#[doc = "Field `gpo42_doen` reader - The selected OEN signal for GPIO42. The register value indicates the selected GPIO (Output Enable) OEN index from GPIO OEN list 0-49. See Table 2-41: GPIO OEN List for SYS_IOMUX (on page 97) for more information."]
pub type GPO42_DOEN_R = crate::FieldReader;
#[doc = "Field `gpo42_doen` writer - The selected OEN signal for GPIO42. The register value indicates the selected GPIO (Output Enable) OEN index from GPIO OEN list 0-49. See Table 2-41: GPIO OEN List for SYS_IOMUX (on page 97) for more information."]
pub type GPO42_DOEN_W<'a, REG, const O: u8> = crate::FieldWriter<'a, REG, 6, O>;
#[doc = "Field `gpo43_doen` reader - The selected OEN signal for GPIO43. The register value indicates the selected GPIO (Output Enable) OEN index from GPIO OEN list 0-49. See Table 2-41: GPIO OEN List for SYS_IOMUX (on page 97) for more information."]
pub type GPO43_DOEN_R = crate::FieldReader;
#[doc = "Field `gpo43_doen` writer - The selected OEN signal for GPIO43. The register value indicates the selected GPIO (Output Enable) OEN index from GPIO OEN list 0-49. See Table 2-41: GPIO OEN List for SYS_IOMUX (on page 97) for more information."]
pub type GPO43_DOEN_W<'a, REG, const O: u8> = crate::FieldWriter<'a, REG, 6, O>;
impl R {
    #[doc = "Bits 0:5 - The selected OEN signal for GPIO40. The register value indicates the selected GPIO (Output Enable) OEN index from GPIO OEN list 0-49. See Table 2-41: GPIO OEN List for SYS_IOMUX (on page 97) for more information."]
    #[inline(always)]
    pub fn gpo40_doen(&self) -> GPO40_DOEN_R {
        GPO40_DOEN_R::new((self.bits & 0x3f) as u8)
    }
    #[doc = "Bits 8:13 - The selected OEN signal for GPIO41. The register value indicates the selected GPIO (Output Enable) OEN index from GPIO OEN list 0-49. See Table 2-41: GPIO OEN List for SYS_IOMUX (on page 97) for more information."]
    #[inline(always)]
    pub fn gpo41_doen(&self) -> GPO41_DOEN_R {
        GPO41_DOEN_R::new(((self.bits >> 8) & 0x3f) as u8)
    }
    #[doc = "Bits 16:21 - The selected OEN signal for GPIO42. The register value indicates the selected GPIO (Output Enable) OEN index from GPIO OEN list 0-49. See Table 2-41: GPIO OEN List for SYS_IOMUX (on page 97) for more information."]
    #[inline(always)]
    pub fn gpo42_doen(&self) -> GPO42_DOEN_R {
        GPO42_DOEN_R::new(((self.bits >> 16) & 0x3f) as u8)
    }
    #[doc = "Bits 24:29 - The selected OEN signal for GPIO43. The register value indicates the selected GPIO (Output Enable) OEN index from GPIO OEN list 0-49. See Table 2-41: GPIO OEN List for SYS_IOMUX (on page 97) for more information."]
    #[inline(always)]
    pub fn gpo43_doen(&self) -> GPO43_DOEN_R {
        GPO43_DOEN_R::new(((self.bits >> 24) & 0x3f) as u8)
    }
}
impl W {
    #[doc = "Bits 0:5 - The selected OEN signal for GPIO40. The register value indicates the selected GPIO (Output Enable) OEN index from GPIO OEN list 0-49. See Table 2-41: GPIO OEN List for SYS_IOMUX (on page 97) for more information."]
    #[inline(always)]
    #[must_use]
    pub fn gpo40_doen(&mut self) -> GPO40_DOEN_W<GPO_DOEN10_SPEC, 0> {
        GPO40_DOEN_W::new(self)
    }
    #[doc = "Bits 8:13 - The selected OEN signal for GPIO41. The register value indicates the selected GPIO (Output Enable) OEN index from GPIO OEN list 0-49. See Table 2-41: GPIO OEN List for SYS_IOMUX (on page 97) for more information."]
    #[inline(always)]
    #[must_use]
    pub fn gpo41_doen(&mut self) -> GPO41_DOEN_W<GPO_DOEN10_SPEC, 8> {
        GPO41_DOEN_W::new(self)
    }
    #[doc = "Bits 16:21 - The selected OEN signal for GPIO42. The register value indicates the selected GPIO (Output Enable) OEN index from GPIO OEN list 0-49. See Table 2-41: GPIO OEN List for SYS_IOMUX (on page 97) for more information."]
    #[inline(always)]
    #[must_use]
    pub fn gpo42_doen(&mut self) -> GPO42_DOEN_W<GPO_DOEN10_SPEC, 16> {
        GPO42_DOEN_W::new(self)
    }
    #[doc = "Bits 24:29 - The selected OEN signal for GPIO43. The register value indicates the selected GPIO (Output Enable) OEN index from GPIO OEN list 0-49. See Table 2-41: GPIO OEN List for SYS_IOMUX (on page 97) for more information."]
    #[inline(always)]
    #[must_use]
    pub fn gpo43_doen(&mut self) -> GPO43_DOEN_W<GPO_DOEN10_SPEC, 24> {
        GPO43_DOEN_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.bits = bits;
        self
    }
}
#[doc = "SYS IOMUX CFG SAIF SYSCFG FMUX 10 DOEN\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`gpo_doen10::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`gpo_doen10::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct GPO_DOEN10_SPEC;
impl crate::RegisterSpec for GPO_DOEN10_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`gpo_doen10::R`](R) reader structure"]
impl crate::Readable for GPO_DOEN10_SPEC {}
#[doc = "`write(|w| ..)` method takes [`gpo_doen10::W`](W) writer structure"]
impl crate::Writable for GPO_DOEN10_SPEC {
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets gpo_doen10 to value 0x0100_0001"]
impl crate::Resettable for GPO_DOEN10_SPEC {
    const RESET_VALUE: Self::Ux = 0x0100_0001;
}
