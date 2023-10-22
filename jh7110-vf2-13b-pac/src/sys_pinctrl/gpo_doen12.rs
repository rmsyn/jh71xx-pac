#[doc = "Register `gpo_doen12` reader"]
pub type R = crate::R<GPO_DOEN12_SPEC>;
#[doc = "Register `gpo_doen12` writer"]
pub type W = crate::W<GPO_DOEN12_SPEC>;
#[doc = "Field `gpo48_doen` reader - The selected OEN signal for GPIO48. The register value indicates the selected GPIO (Output Enable) OEN index from GPIO OEN list 0-49. See Table 2-41: GPIO OEN List for SYS_IOMUX (on page 97) for more information."]
pub type GPO48_DOEN_R = crate::FieldReader;
#[doc = "Field `gpo48_doen` writer - The selected OEN signal for GPIO48. The register value indicates the selected GPIO (Output Enable) OEN index from GPIO OEN list 0-49. See Table 2-41: GPIO OEN List for SYS_IOMUX (on page 97) for more information."]
pub type GPO48_DOEN_W<'a, REG, const O: u8> = crate::FieldWriter<'a, REG, 6, O>;
#[doc = "Field `gpo49_doen` reader - The selected OEN signal for GPIO49. The register value indicates the selected GPIO (Output Enable) OEN index from GPIO OEN list 0-49. See Table 2-41: GPIO OEN List for SYS_IOMUX (on page 97) for more information."]
pub type GPO49_DOEN_R = crate::FieldReader;
#[doc = "Field `gpo49_doen` writer - The selected OEN signal for GPIO49. The register value indicates the selected GPIO (Output Enable) OEN index from GPIO OEN list 0-49. See Table 2-41: GPIO OEN List for SYS_IOMUX (on page 97) for more information."]
pub type GPO49_DOEN_W<'a, REG, const O: u8> = crate::FieldWriter<'a, REG, 6, O>;
#[doc = "Field `gpo50_doen` reader - The selected OEN signal for GPIO50. The register value indicates the selected GPIO (Output Enable) OEN index from GPIO OEN list 0-49. See Table 2-41: GPIO OEN List for SYS_IOMUX (on page 97) for more information."]
pub type GPO50_DOEN_R = crate::FieldReader;
#[doc = "Field `gpo50_doen` writer - The selected OEN signal for GPIO50. The register value indicates the selected GPIO (Output Enable) OEN index from GPIO OEN list 0-49. See Table 2-41: GPIO OEN List for SYS_IOMUX (on page 97) for more information."]
pub type GPO50_DOEN_W<'a, REG, const O: u8> = crate::FieldWriter<'a, REG, 6, O>;
#[doc = "Field `gpo51_doen` reader - The selected OEN signal for GPIO51. The register value indicates the selected GPIO (Output Enable) OEN index from GPIO OEN list 0-49. See Table 2-41: GPIO OEN List for SYS_IOMUX (on page 97) for more information."]
pub type GPO51_DOEN_R = crate::FieldReader;
#[doc = "Field `gpo51_doen` writer - The selected OEN signal for GPIO51. The register value indicates the selected GPIO (Output Enable) OEN index from GPIO OEN list 0-49. See Table 2-41: GPIO OEN List for SYS_IOMUX (on page 97) for more information."]
pub type GPO51_DOEN_W<'a, REG, const O: u8> = crate::FieldWriter<'a, REG, 6, O>;
impl R {
    #[doc = "Bits 0:5 - The selected OEN signal for GPIO48. The register value indicates the selected GPIO (Output Enable) OEN index from GPIO OEN list 0-49. See Table 2-41: GPIO OEN List for SYS_IOMUX (on page 97) for more information."]
    #[inline(always)]
    pub fn gpo48_doen(&self) -> GPO48_DOEN_R {
        GPO48_DOEN_R::new((self.bits & 0x3f) as u8)
    }
    #[doc = "Bits 8:13 - The selected OEN signal for GPIO49. The register value indicates the selected GPIO (Output Enable) OEN index from GPIO OEN list 0-49. See Table 2-41: GPIO OEN List for SYS_IOMUX (on page 97) for more information."]
    #[inline(always)]
    pub fn gpo49_doen(&self) -> GPO49_DOEN_R {
        GPO49_DOEN_R::new(((self.bits >> 8) & 0x3f) as u8)
    }
    #[doc = "Bits 16:21 - The selected OEN signal for GPIO50. The register value indicates the selected GPIO (Output Enable) OEN index from GPIO OEN list 0-49. See Table 2-41: GPIO OEN List for SYS_IOMUX (on page 97) for more information."]
    #[inline(always)]
    pub fn gpo50_doen(&self) -> GPO50_DOEN_R {
        GPO50_DOEN_R::new(((self.bits >> 16) & 0x3f) as u8)
    }
    #[doc = "Bits 24:29 - The selected OEN signal for GPIO51. The register value indicates the selected GPIO (Output Enable) OEN index from GPIO OEN list 0-49. See Table 2-41: GPIO OEN List for SYS_IOMUX (on page 97) for more information."]
    #[inline(always)]
    pub fn gpo51_doen(&self) -> GPO51_DOEN_R {
        GPO51_DOEN_R::new(((self.bits >> 24) & 0x3f) as u8)
    }
}
impl W {
    #[doc = "Bits 0:5 - The selected OEN signal for GPIO48. The register value indicates the selected GPIO (Output Enable) OEN index from GPIO OEN list 0-49. See Table 2-41: GPIO OEN List for SYS_IOMUX (on page 97) for more information."]
    #[inline(always)]
    #[must_use]
    pub fn gpo48_doen(&mut self) -> GPO48_DOEN_W<GPO_DOEN12_SPEC, 0> {
        GPO48_DOEN_W::new(self)
    }
    #[doc = "Bits 8:13 - The selected OEN signal for GPIO49. The register value indicates the selected GPIO (Output Enable) OEN index from GPIO OEN list 0-49. See Table 2-41: GPIO OEN List for SYS_IOMUX (on page 97) for more information."]
    #[inline(always)]
    #[must_use]
    pub fn gpo49_doen(&mut self) -> GPO49_DOEN_W<GPO_DOEN12_SPEC, 8> {
        GPO49_DOEN_W::new(self)
    }
    #[doc = "Bits 16:21 - The selected OEN signal for GPIO50. The register value indicates the selected GPIO (Output Enable) OEN index from GPIO OEN list 0-49. See Table 2-41: GPIO OEN List for SYS_IOMUX (on page 97) for more information."]
    #[inline(always)]
    #[must_use]
    pub fn gpo50_doen(&mut self) -> GPO50_DOEN_W<GPO_DOEN12_SPEC, 16> {
        GPO50_DOEN_W::new(self)
    }
    #[doc = "Bits 24:29 - The selected OEN signal for GPIO51. The register value indicates the selected GPIO (Output Enable) OEN index from GPIO OEN list 0-49. See Table 2-41: GPIO OEN List for SYS_IOMUX (on page 97) for more information."]
    #[inline(always)]
    #[must_use]
    pub fn gpo51_doen(&mut self) -> GPO51_DOEN_W<GPO_DOEN12_SPEC, 24> {
        GPO51_DOEN_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.bits = bits;
        self
    }
}
#[doc = "SYS IOMUX CFG SAIF SYSCFG FMUX 12 DOEN\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`gpo_doen12::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`gpo_doen12::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct GPO_DOEN12_SPEC;
impl crate::RegisterSpec for GPO_DOEN12_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`gpo_doen12::R`](R) reader structure"]
impl crate::Readable for GPO_DOEN12_SPEC {}
#[doc = "`write(|w| ..)` method takes [`gpo_doen12::W`](W) writer structure"]
impl crate::Writable for GPO_DOEN12_SPEC {
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets gpo_doen12 to value 0x0e01_0d0d"]
impl crate::Resettable for GPO_DOEN12_SPEC {
    const RESET_VALUE: Self::Ux = 0x0e01_0d0d;
}
