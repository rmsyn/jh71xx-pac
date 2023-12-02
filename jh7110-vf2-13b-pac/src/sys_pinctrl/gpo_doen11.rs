#[doc = "Register `gpo_doen11` reader"]
pub type R = crate::R<GPO_DOEN11_SPEC>;
#[doc = "Register `gpo_doen11` writer"]
pub type W = crate::W<GPO_DOEN11_SPEC>;
#[doc = "Field `gpo44_doen` reader - The selected OEN signal for GPIO44. The register value indicates the selected GPIO (Output Enable) OEN index from GPIO OEN list 0-49. See Table 2-41: GPIO OEN List for SYS_IOMUX (on page 97) for more information."]
pub type GPO44_DOEN_R = crate::FieldReader;
#[doc = "Field `gpo44_doen` writer - The selected OEN signal for GPIO44. The register value indicates the selected GPIO (Output Enable) OEN index from GPIO OEN list 0-49. See Table 2-41: GPIO OEN List for SYS_IOMUX (on page 97) for more information."]
pub type GPO44_DOEN_W<'a, REG> = crate::FieldWriter<'a, REG, 6>;
#[doc = "Field `gpo45_doen` reader - The selected OEN signal for GPIO45. The register value indicates the selected GPIO (Output Enable) OEN index from GPIO OEN list 0-49. See Table 2-41: GPIO OEN List for SYS_IOMUX (on page 97) for more information."]
pub type GPO45_DOEN_R = crate::FieldReader;
#[doc = "Field `gpo45_doen` writer - The selected OEN signal for GPIO45. The register value indicates the selected GPIO (Output Enable) OEN index from GPIO OEN list 0-49. See Table 2-41: GPIO OEN List for SYS_IOMUX (on page 97) for more information."]
pub type GPO45_DOEN_W<'a, REG> = crate::FieldWriter<'a, REG, 6>;
#[doc = "Field `gpo46_doen` reader - The selected OEN signal for GPIO46. The register value indicates the selected GPIO (Output Enable) OEN index from GPIO OEN list 0-49. See Table 2-41: GPIO OEN List for SYS_IOMUX (on page 97) for more information."]
pub type GPO46_DOEN_R = crate::FieldReader;
#[doc = "Field `gpo46_doen` writer - The selected OEN signal for GPIO46. The register value indicates the selected GPIO (Output Enable) OEN index from GPIO OEN list 0-49. See Table 2-41: GPIO OEN List for SYS_IOMUX (on page 97) for more information."]
pub type GPO46_DOEN_W<'a, REG> = crate::FieldWriter<'a, REG, 6>;
#[doc = "Field `gpo47_doen` reader - The selected OEN signal for GPIO47. The register value indicates the selected GPIO (Output Enable) OEN index from GPIO OEN list 0-49. See Table 2-41: GPIO OEN List for SYS_IOMUX (on page 97) for more information."]
pub type GPO47_DOEN_R = crate::FieldReader;
#[doc = "Field `gpo47_doen` writer - The selected OEN signal for GPIO47. The register value indicates the selected GPIO (Output Enable) OEN index from GPIO OEN list 0-49. See Table 2-41: GPIO OEN List for SYS_IOMUX (on page 97) for more information."]
pub type GPO47_DOEN_W<'a, REG> = crate::FieldWriter<'a, REG, 6>;
impl R {
    #[doc = "Bits 0:5 - The selected OEN signal for GPIO44. The register value indicates the selected GPIO (Output Enable) OEN index from GPIO OEN list 0-49. See Table 2-41: GPIO OEN List for SYS_IOMUX (on page 97) for more information."]
    #[inline(always)]
    pub fn gpo44_doen(&self) -> GPO44_DOEN_R {
        GPO44_DOEN_R::new((self.bits & 0x3f) as u8)
    }
    #[doc = "Bits 8:13 - The selected OEN signal for GPIO45. The register value indicates the selected GPIO (Output Enable) OEN index from GPIO OEN list 0-49. See Table 2-41: GPIO OEN List for SYS_IOMUX (on page 97) for more information."]
    #[inline(always)]
    pub fn gpo45_doen(&self) -> GPO45_DOEN_R {
        GPO45_DOEN_R::new(((self.bits >> 8) & 0x3f) as u8)
    }
    #[doc = "Bits 16:21 - The selected OEN signal for GPIO46. The register value indicates the selected GPIO (Output Enable) OEN index from GPIO OEN list 0-49. See Table 2-41: GPIO OEN List for SYS_IOMUX (on page 97) for more information."]
    #[inline(always)]
    pub fn gpo46_doen(&self) -> GPO46_DOEN_R {
        GPO46_DOEN_R::new(((self.bits >> 16) & 0x3f) as u8)
    }
    #[doc = "Bits 24:29 - The selected OEN signal for GPIO47. The register value indicates the selected GPIO (Output Enable) OEN index from GPIO OEN list 0-49. See Table 2-41: GPIO OEN List for SYS_IOMUX (on page 97) for more information."]
    #[inline(always)]
    pub fn gpo47_doen(&self) -> GPO47_DOEN_R {
        GPO47_DOEN_R::new(((self.bits >> 24) & 0x3f) as u8)
    }
}
impl W {
    #[doc = "Bits 0:5 - The selected OEN signal for GPIO44. The register value indicates the selected GPIO (Output Enable) OEN index from GPIO OEN list 0-49. See Table 2-41: GPIO OEN List for SYS_IOMUX (on page 97) for more information."]
    #[inline(always)]
    #[must_use]
    pub fn gpo44_doen(&mut self) -> GPO44_DOEN_W<GPO_DOEN11_SPEC> {
        GPO44_DOEN_W::new(self, 0)
    }
    #[doc = "Bits 8:13 - The selected OEN signal for GPIO45. The register value indicates the selected GPIO (Output Enable) OEN index from GPIO OEN list 0-49. See Table 2-41: GPIO OEN List for SYS_IOMUX (on page 97) for more information."]
    #[inline(always)]
    #[must_use]
    pub fn gpo45_doen(&mut self) -> GPO45_DOEN_W<GPO_DOEN11_SPEC> {
        GPO45_DOEN_W::new(self, 8)
    }
    #[doc = "Bits 16:21 - The selected OEN signal for GPIO46. The register value indicates the selected GPIO (Output Enable) OEN index from GPIO OEN list 0-49. See Table 2-41: GPIO OEN List for SYS_IOMUX (on page 97) for more information."]
    #[inline(always)]
    #[must_use]
    pub fn gpo46_doen(&mut self) -> GPO46_DOEN_W<GPO_DOEN11_SPEC> {
        GPO46_DOEN_W::new(self, 16)
    }
    #[doc = "Bits 24:29 - The selected OEN signal for GPIO47. The register value indicates the selected GPIO (Output Enable) OEN index from GPIO OEN list 0-49. See Table 2-41: GPIO OEN List for SYS_IOMUX (on page 97) for more information."]
    #[inline(always)]
    #[must_use]
    pub fn gpo47_doen(&mut self) -> GPO47_DOEN_W<GPO_DOEN11_SPEC> {
        GPO47_DOEN_W::new(self, 24)
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
#[doc = "SYS IOMUX CFG SAIF SYSCFG FMUX 11 DOEN\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`gpo_doen11::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`gpo_doen11::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct GPO_DOEN11_SPEC;
impl crate::RegisterSpec for GPO_DOEN11_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`gpo_doen11::R`](R) reader structure"]
impl crate::Readable for GPO_DOEN11_SPEC {}
#[doc = "`write(|w| ..)` method takes [`gpo_doen11::W`](W) writer structure"]
impl crate::Writable for GPO_DOEN11_SPEC {
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets gpo_doen11 to value 0x0100_0001"]
impl crate::Resettable for GPO_DOEN11_SPEC {
    const RESET_VALUE: Self::Ux = 0x0100_0001;
}
