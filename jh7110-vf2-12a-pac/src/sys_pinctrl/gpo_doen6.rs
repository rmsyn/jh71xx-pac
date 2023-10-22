#[doc = "Register `gpo_doen6` reader"]
pub type R = crate::R<GPO_DOEN6_SPEC>;
#[doc = "Register `gpo_doen6` writer"]
pub type W = crate::W<GPO_DOEN6_SPEC>;
#[doc = "Field `gpo24_doen` reader - The selected OEN signal for GPIO24. The register value indicates the selected GPIO (Output Enable) OEN index from GPIO OEN list 0-49. See Table 2-41: GPIO OEN List for SYS_IOMUX (on page 97) for more information."]
pub type GPO24_DOEN_R = crate::FieldReader;
#[doc = "Field `gpo24_doen` writer - The selected OEN signal for GPIO24. The register value indicates the selected GPIO (Output Enable) OEN index from GPIO OEN list 0-49. See Table 2-41: GPIO OEN List for SYS_IOMUX (on page 97) for more information."]
pub type GPO24_DOEN_W<'a, REG, const O: u8> = crate::FieldWriter<'a, REG, 6, O>;
#[doc = "Field `gpo25_doen` reader - The selected OEN signal for GPIO25. The register value indicates the selected GPIO (Output Enable) OEN index from GPIO OEN list 0-49. See Table 2-41: GPIO OEN List for SYS_IOMUX (on page 97) for more information."]
pub type GPO25_DOEN_R = crate::FieldReader;
#[doc = "Field `gpo25_doen` writer - The selected OEN signal for GPIO25. The register value indicates the selected GPIO (Output Enable) OEN index from GPIO OEN list 0-49. See Table 2-41: GPIO OEN List for SYS_IOMUX (on page 97) for more information."]
pub type GPO25_DOEN_W<'a, REG, const O: u8> = crate::FieldWriter<'a, REG, 6, O>;
#[doc = "Field `gpo26_doen` reader - The selected OEN signal for GPIO26. The register value indicates the selected GPIO (Output Enable) OEN index from GPIO OEN list 0-49. See Table 2-41: GPIO OEN List for SYS_IOMUX (on page 97) for more information."]
pub type GPO26_DOEN_R = crate::FieldReader;
#[doc = "Field `gpo26_doen` writer - The selected OEN signal for GPIO26. The register value indicates the selected GPIO (Output Enable) OEN index from GPIO OEN list 0-49. See Table 2-41: GPIO OEN List for SYS_IOMUX (on page 97) for more information."]
pub type GPO26_DOEN_W<'a, REG, const O: u8> = crate::FieldWriter<'a, REG, 6, O>;
#[doc = "Field `gpo27_doen` reader - The selected OEN signal for GPIO27. The register value indicates the selected GPIO (Output Enable) OEN index from GPIO OEN list 0-49. See Table 2-41: GPIO OEN List for SYS_IOMUX (on page 97) for more information."]
pub type GPO27_DOEN_R = crate::FieldReader;
#[doc = "Field `gpo27_doen` writer - The selected OEN signal for GPIO27. The register value indicates the selected GPIO (Output Enable) OEN index from GPIO OEN list 0-49. See Table 2-41: GPIO OEN List for SYS_IOMUX (on page 97) for more information."]
pub type GPO27_DOEN_W<'a, REG, const O: u8> = crate::FieldWriter<'a, REG, 6, O>;
impl R {
    #[doc = "Bits 0:5 - The selected OEN signal for GPIO24. The register value indicates the selected GPIO (Output Enable) OEN index from GPIO OEN list 0-49. See Table 2-41: GPIO OEN List for SYS_IOMUX (on page 97) for more information."]
    #[inline(always)]
    pub fn gpo24_doen(&self) -> GPO24_DOEN_R {
        GPO24_DOEN_R::new((self.bits & 0x3f) as u8)
    }
    #[doc = "Bits 8:13 - The selected OEN signal for GPIO25. The register value indicates the selected GPIO (Output Enable) OEN index from GPIO OEN list 0-49. See Table 2-41: GPIO OEN List for SYS_IOMUX (on page 97) for more information."]
    #[inline(always)]
    pub fn gpo25_doen(&self) -> GPO25_DOEN_R {
        GPO25_DOEN_R::new(((self.bits >> 8) & 0x3f) as u8)
    }
    #[doc = "Bits 16:21 - The selected OEN signal for GPIO26. The register value indicates the selected GPIO (Output Enable) OEN index from GPIO OEN list 0-49. See Table 2-41: GPIO OEN List for SYS_IOMUX (on page 97) for more information."]
    #[inline(always)]
    pub fn gpo26_doen(&self) -> GPO26_DOEN_R {
        GPO26_DOEN_R::new(((self.bits >> 16) & 0x3f) as u8)
    }
    #[doc = "Bits 24:29 - The selected OEN signal for GPIO27. The register value indicates the selected GPIO (Output Enable) OEN index from GPIO OEN list 0-49. See Table 2-41: GPIO OEN List for SYS_IOMUX (on page 97) for more information."]
    #[inline(always)]
    pub fn gpo27_doen(&self) -> GPO27_DOEN_R {
        GPO27_DOEN_R::new(((self.bits >> 24) & 0x3f) as u8)
    }
}
impl W {
    #[doc = "Bits 0:5 - The selected OEN signal for GPIO24. The register value indicates the selected GPIO (Output Enable) OEN index from GPIO OEN list 0-49. See Table 2-41: GPIO OEN List for SYS_IOMUX (on page 97) for more information."]
    #[inline(always)]
    #[must_use]
    pub fn gpo24_doen(&mut self) -> GPO24_DOEN_W<GPO_DOEN6_SPEC, 0> {
        GPO24_DOEN_W::new(self)
    }
    #[doc = "Bits 8:13 - The selected OEN signal for GPIO25. The register value indicates the selected GPIO (Output Enable) OEN index from GPIO OEN list 0-49. See Table 2-41: GPIO OEN List for SYS_IOMUX (on page 97) for more information."]
    #[inline(always)]
    #[must_use]
    pub fn gpo25_doen(&mut self) -> GPO25_DOEN_W<GPO_DOEN6_SPEC, 8> {
        GPO25_DOEN_W::new(self)
    }
    #[doc = "Bits 16:21 - The selected OEN signal for GPIO26. The register value indicates the selected GPIO (Output Enable) OEN index from GPIO OEN list 0-49. See Table 2-41: GPIO OEN List for SYS_IOMUX (on page 97) for more information."]
    #[inline(always)]
    #[must_use]
    pub fn gpo26_doen(&mut self) -> GPO26_DOEN_W<GPO_DOEN6_SPEC, 16> {
        GPO26_DOEN_W::new(self)
    }
    #[doc = "Bits 24:29 - The selected OEN signal for GPIO27. The register value indicates the selected GPIO (Output Enable) OEN index from GPIO OEN list 0-49. See Table 2-41: GPIO OEN List for SYS_IOMUX (on page 97) for more information."]
    #[inline(always)]
    #[must_use]
    pub fn gpo27_doen(&mut self) -> GPO27_DOEN_W<GPO_DOEN6_SPEC, 24> {
        GPO27_DOEN_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.bits = bits;
        self
    }
}
#[doc = "SYS IOMUX CFG SAIF SYSCFG FMUX 6 DOEN\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`gpo_doen6::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`gpo_doen6::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct GPO_DOEN6_SPEC;
impl crate::RegisterSpec for GPO_DOEN6_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`gpo_doen6::R`](R) reader structure"]
impl crate::Readable for GPO_DOEN6_SPEC {}
#[doc = "`write(|w| ..)` method takes [`gpo_doen6::W`](W) writer structure"]
impl crate::Writable for GPO_DOEN6_SPEC {
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets gpo_doen6 to value 0"]
impl crate::Resettable for GPO_DOEN6_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
