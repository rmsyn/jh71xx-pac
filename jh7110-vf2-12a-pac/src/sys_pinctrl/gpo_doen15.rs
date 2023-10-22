#[doc = "Register `gpo_doen15` reader"]
pub type R = crate::R<GPO_DOEN15_SPEC>;
#[doc = "Register `gpo_doen15` writer"]
pub type W = crate::W<GPO_DOEN15_SPEC>;
#[doc = "Field `gpo60_doen` reader - The selected OEN signal for GPIO60. The register value indicates the selected GPIO (Output Enable) OEN index from GPIO OEN list 0-49. See Table 2-41: GPIO OEN List for SYS_IOMUX (on page 97) for more information."]
pub type GPO60_DOEN_R = crate::FieldReader;
#[doc = "Field `gpo60_doen` writer - The selected OEN signal for GPIO60. The register value indicates the selected GPIO (Output Enable) OEN index from GPIO OEN list 0-49. See Table 2-41: GPIO OEN List for SYS_IOMUX (on page 97) for more information."]
pub type GPO60_DOEN_W<'a, REG, const O: u8> = crate::FieldWriter<'a, REG, 6, O>;
#[doc = "Field `gpo61_doen` reader - The selected OEN signal for GPIO61. The register value indicates the selected GPIO (Output Enable) OEN index from GPIO OEN list 0-49. See Table 2-41: GPIO OEN List for SYS_IOMUX (on page 97) for more information."]
pub type GPO61_DOEN_R = crate::FieldReader;
#[doc = "Field `gpo61_doen` writer - The selected OEN signal for GPIO61. The register value indicates the selected GPIO (Output Enable) OEN index from GPIO OEN list 0-49. See Table 2-41: GPIO OEN List for SYS_IOMUX (on page 97) for more information."]
pub type GPO61_DOEN_W<'a, REG, const O: u8> = crate::FieldWriter<'a, REG, 6, O>;
#[doc = "Field `gpo62_doen` reader - The selected OEN signal for GPIO62. The register value indicates the selected GPIO (Output Enable) OEN index from GPIO OEN list 0-49. See Table 2-41: GPIO OEN List for SYS_IOMUX (on page 97) for more information."]
pub type GPO62_DOEN_R = crate::FieldReader;
#[doc = "Field `gpo62_doen` writer - The selected OEN signal for GPIO62. The register value indicates the selected GPIO (Output Enable) OEN index from GPIO OEN list 0-49. See Table 2-41: GPIO OEN List for SYS_IOMUX (on page 97) for more information."]
pub type GPO62_DOEN_W<'a, REG, const O: u8> = crate::FieldWriter<'a, REG, 6, O>;
#[doc = "Field `gpo63_doen` reader - The selected OEN signal for GPIO63. The register value indicates the selected GPIO (Output Enable) OEN index from GPIO OEN list 0-49. See Table 2-41: GPIO OEN List for SYS_IOMUX (on page 97) for more information."]
pub type GPO63_DOEN_R = crate::FieldReader;
#[doc = "Field `gpo63_doen` writer - The selected OEN signal for GPIO63. The register value indicates the selected GPIO (Output Enable) OEN index from GPIO OEN list 0-49. See Table 2-41: GPIO OEN List for SYS_IOMUX (on page 97) for more information."]
pub type GPO63_DOEN_W<'a, REG, const O: u8> = crate::FieldWriter<'a, REG, 6, O>;
impl R {
    #[doc = "Bits 0:5 - The selected OEN signal for GPIO60. The register value indicates the selected GPIO (Output Enable) OEN index from GPIO OEN list 0-49. See Table 2-41: GPIO OEN List for SYS_IOMUX (on page 97) for more information."]
    #[inline(always)]
    pub fn gpo60_doen(&self) -> GPO60_DOEN_R {
        GPO60_DOEN_R::new((self.bits & 0x3f) as u8)
    }
    #[doc = "Bits 8:13 - The selected OEN signal for GPIO61. The register value indicates the selected GPIO (Output Enable) OEN index from GPIO OEN list 0-49. See Table 2-41: GPIO OEN List for SYS_IOMUX (on page 97) for more information."]
    #[inline(always)]
    pub fn gpo61_doen(&self) -> GPO61_DOEN_R {
        GPO61_DOEN_R::new(((self.bits >> 8) & 0x3f) as u8)
    }
    #[doc = "Bits 16:21 - The selected OEN signal for GPIO62. The register value indicates the selected GPIO (Output Enable) OEN index from GPIO OEN list 0-49. See Table 2-41: GPIO OEN List for SYS_IOMUX (on page 97) for more information."]
    #[inline(always)]
    pub fn gpo62_doen(&self) -> GPO62_DOEN_R {
        GPO62_DOEN_R::new(((self.bits >> 16) & 0x3f) as u8)
    }
    #[doc = "Bits 24:29 - The selected OEN signal for GPIO63. The register value indicates the selected GPIO (Output Enable) OEN index from GPIO OEN list 0-49. See Table 2-41: GPIO OEN List for SYS_IOMUX (on page 97) for more information."]
    #[inline(always)]
    pub fn gpo63_doen(&self) -> GPO63_DOEN_R {
        GPO63_DOEN_R::new(((self.bits >> 24) & 0x3f) as u8)
    }
}
impl W {
    #[doc = "Bits 0:5 - The selected OEN signal for GPIO60. The register value indicates the selected GPIO (Output Enable) OEN index from GPIO OEN list 0-49. See Table 2-41: GPIO OEN List for SYS_IOMUX (on page 97) for more information."]
    #[inline(always)]
    #[must_use]
    pub fn gpo60_doen(&mut self) -> GPO60_DOEN_W<GPO_DOEN15_SPEC, 0> {
        GPO60_DOEN_W::new(self)
    }
    #[doc = "Bits 8:13 - The selected OEN signal for GPIO61. The register value indicates the selected GPIO (Output Enable) OEN index from GPIO OEN list 0-49. See Table 2-41: GPIO OEN List for SYS_IOMUX (on page 97) for more information."]
    #[inline(always)]
    #[must_use]
    pub fn gpo61_doen(&mut self) -> GPO61_DOEN_W<GPO_DOEN15_SPEC, 8> {
        GPO61_DOEN_W::new(self)
    }
    #[doc = "Bits 16:21 - The selected OEN signal for GPIO62. The register value indicates the selected GPIO (Output Enable) OEN index from GPIO OEN list 0-49. See Table 2-41: GPIO OEN List for SYS_IOMUX (on page 97) for more information."]
    #[inline(always)]
    #[must_use]
    pub fn gpo62_doen(&mut self) -> GPO62_DOEN_W<GPO_DOEN15_SPEC, 16> {
        GPO62_DOEN_W::new(self)
    }
    #[doc = "Bits 24:29 - The selected OEN signal for GPIO63. The register value indicates the selected GPIO (Output Enable) OEN index from GPIO OEN list 0-49. See Table 2-41: GPIO OEN List for SYS_IOMUX (on page 97) for more information."]
    #[inline(always)]
    #[must_use]
    pub fn gpo63_doen(&mut self) -> GPO63_DOEN_W<GPO_DOEN15_SPEC, 24> {
        GPO63_DOEN_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.bits = bits;
        self
    }
}
#[doc = "SYS IOMUX CFG SAIF SYSCFG FMUX 15 DOEN\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`gpo_doen15::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`gpo_doen15::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct GPO_DOEN15_SPEC;
impl crate::RegisterSpec for GPO_DOEN15_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`gpo_doen15::R`](R) reader structure"]
impl crate::Readable for GPO_DOEN15_SPEC {}
#[doc = "`write(|w| ..)` method takes [`gpo_doen15::W`](W) writer structure"]
impl crate::Writable for GPO_DOEN15_SPEC {
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets gpo_doen15 to value 0x2901_2828"]
impl crate::Resettable for GPO_DOEN15_SPEC {
    const RESET_VALUE: Self::Ux = 0x2901_2828;
}
