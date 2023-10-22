#[doc = "Register `gpo_doen4` reader"]
pub type R = crate::R<GPO_DOEN4_SPEC>;
#[doc = "Register `gpo_doen4` writer"]
pub type W = crate::W<GPO_DOEN4_SPEC>;
#[doc = "Field `gpo16_doen` reader - The selected OEN signal for GPIO16. The register value indicates the selected GPIO (Output Enable) OEN index from GPIO OEN list 0-49. See Table 2-41: GPIO OEN List for SYS_IOMUX (on page 97) for more information."]
pub type GPO16_DOEN_R = crate::FieldReader;
#[doc = "Field `gpo16_doen` writer - The selected OEN signal for GPIO16. The register value indicates the selected GPIO (Output Enable) OEN index from GPIO OEN list 0-49. See Table 2-41: GPIO OEN List for SYS_IOMUX (on page 97) for more information."]
pub type GPO16_DOEN_W<'a, REG, const O: u8> = crate::FieldWriter<'a, REG, 6, O>;
#[doc = "Field `gpo17_doen` reader - The selected OEN signal for GPIO17. The register value indicates the selected GPIO (Output Enable) OEN index from GPIO OEN list 0-49. See Table 2-41: GPIO OEN List for SYS_IOMUX (on page 97) for more information."]
pub type GPO17_DOEN_R = crate::FieldReader;
#[doc = "Field `gpo17_doen` writer - The selected OEN signal for GPIO17. The register value indicates the selected GPIO (Output Enable) OEN index from GPIO OEN list 0-49. See Table 2-41: GPIO OEN List for SYS_IOMUX (on page 97) for more information."]
pub type GPO17_DOEN_W<'a, REG, const O: u8> = crate::FieldWriter<'a, REG, 6, O>;
#[doc = "Field `gpo18_doen` reader - The selected OEN signal for GPIO18. The register value indicates the selected GPIO (Output Enable) OEN index from GPIO OEN list 0-49. See Table 2-41: GPIO OEN List for SYS_IOMUX (on page 97) for more information."]
pub type GPO18_DOEN_R = crate::FieldReader;
#[doc = "Field `gpo18_doen` writer - The selected OEN signal for GPIO18. The register value indicates the selected GPIO (Output Enable) OEN index from GPIO OEN list 0-49. See Table 2-41: GPIO OEN List for SYS_IOMUX (on page 97) for more information."]
pub type GPO18_DOEN_W<'a, REG, const O: u8> = crate::FieldWriter<'a, REG, 6, O>;
#[doc = "Field `gpo19_doen` reader - The selected OEN signal for GPIO19. The register value indicates the selected GPIO (Output Enable) OEN index from GPIO OEN list 0-49. See Table 2-41: GPIO OEN List for SYS_IOMUX (on page 97) for more information."]
pub type GPO19_DOEN_R = crate::FieldReader;
#[doc = "Field `gpo19_doen` writer - The selected OEN signal for GPIO19. The register value indicates the selected GPIO (Output Enable) OEN index from GPIO OEN list 0-49. See Table 2-41: GPIO OEN List for SYS_IOMUX (on page 97) for more information."]
pub type GPO19_DOEN_W<'a, REG, const O: u8> = crate::FieldWriter<'a, REG, 6, O>;
impl R {
    #[doc = "Bits 0:5 - The selected OEN signal for GPIO16. The register value indicates the selected GPIO (Output Enable) OEN index from GPIO OEN list 0-49. See Table 2-41: GPIO OEN List for SYS_IOMUX (on page 97) for more information."]
    #[inline(always)]
    pub fn gpo16_doen(&self) -> GPO16_DOEN_R {
        GPO16_DOEN_R::new((self.bits & 0x3f) as u8)
    }
    #[doc = "Bits 8:13 - The selected OEN signal for GPIO17. The register value indicates the selected GPIO (Output Enable) OEN index from GPIO OEN list 0-49. See Table 2-41: GPIO OEN List for SYS_IOMUX (on page 97) for more information."]
    #[inline(always)]
    pub fn gpo17_doen(&self) -> GPO17_DOEN_R {
        GPO17_DOEN_R::new(((self.bits >> 8) & 0x3f) as u8)
    }
    #[doc = "Bits 16:21 - The selected OEN signal for GPIO18. The register value indicates the selected GPIO (Output Enable) OEN index from GPIO OEN list 0-49. See Table 2-41: GPIO OEN List for SYS_IOMUX (on page 97) for more information."]
    #[inline(always)]
    pub fn gpo18_doen(&self) -> GPO18_DOEN_R {
        GPO18_DOEN_R::new(((self.bits >> 16) & 0x3f) as u8)
    }
    #[doc = "Bits 24:29 - The selected OEN signal for GPIO19. The register value indicates the selected GPIO (Output Enable) OEN index from GPIO OEN list 0-49. See Table 2-41: GPIO OEN List for SYS_IOMUX (on page 97) for more information."]
    #[inline(always)]
    pub fn gpo19_doen(&self) -> GPO19_DOEN_R {
        GPO19_DOEN_R::new(((self.bits >> 24) & 0x3f) as u8)
    }
}
impl W {
    #[doc = "Bits 0:5 - The selected OEN signal for GPIO16. The register value indicates the selected GPIO (Output Enable) OEN index from GPIO OEN list 0-49. See Table 2-41: GPIO OEN List for SYS_IOMUX (on page 97) for more information."]
    #[inline(always)]
    #[must_use]
    pub fn gpo16_doen(&mut self) -> GPO16_DOEN_W<GPO_DOEN4_SPEC, 0> {
        GPO16_DOEN_W::new(self)
    }
    #[doc = "Bits 8:13 - The selected OEN signal for GPIO17. The register value indicates the selected GPIO (Output Enable) OEN index from GPIO OEN list 0-49. See Table 2-41: GPIO OEN List for SYS_IOMUX (on page 97) for more information."]
    #[inline(always)]
    #[must_use]
    pub fn gpo17_doen(&mut self) -> GPO17_DOEN_W<GPO_DOEN4_SPEC, 8> {
        GPO17_DOEN_W::new(self)
    }
    #[doc = "Bits 16:21 - The selected OEN signal for GPIO18. The register value indicates the selected GPIO (Output Enable) OEN index from GPIO OEN list 0-49. See Table 2-41: GPIO OEN List for SYS_IOMUX (on page 97) for more information."]
    #[inline(always)]
    #[must_use]
    pub fn gpo18_doen(&mut self) -> GPO18_DOEN_W<GPO_DOEN4_SPEC, 16> {
        GPO18_DOEN_W::new(self)
    }
    #[doc = "Bits 24:29 - The selected OEN signal for GPIO19. The register value indicates the selected GPIO (Output Enable) OEN index from GPIO OEN list 0-49. See Table 2-41: GPIO OEN List for SYS_IOMUX (on page 97) for more information."]
    #[inline(always)]
    #[must_use]
    pub fn gpo19_doen(&mut self) -> GPO19_DOEN_W<GPO_DOEN4_SPEC, 24> {
        GPO19_DOEN_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.bits = bits;
        self
    }
}
#[doc = "SYS IOMUX CFG SAIF SYSCFG FMUX 4 DOEN\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`gpo_doen4::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`gpo_doen4::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct GPO_DOEN4_SPEC;
impl crate::RegisterSpec for GPO_DOEN4_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`gpo_doen4::R`](R) reader structure"]
impl crate::Readable for GPO_DOEN4_SPEC {}
#[doc = "`write(|w| ..)` method takes [`gpo_doen4::W`](W) writer structure"]
impl crate::Writable for GPO_DOEN4_SPEC {
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets gpo_doen4 to value 0x0100_0000"]
impl crate::Resettable for GPO_DOEN4_SPEC {
    const RESET_VALUE: Self::Ux = 0x0100_0000;
}
