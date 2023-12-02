#[doc = "Register `gpo_doen5` reader"]
pub type R = crate::R<GPO_DOEN5_SPEC>;
#[doc = "Register `gpo_doen5` writer"]
pub type W = crate::W<GPO_DOEN5_SPEC>;
#[doc = "Field `gpo20_doen` reader - The selected OEN signal for GPIO20. The register value indicates the selected GPIO (Output Enable) OEN index from GPIO OEN list 0-49. See Table 2-41: GPIO OEN List for SYS_IOMUX (on page 97) for more information."]
pub type GPO20_DOEN_R = crate::FieldReader;
#[doc = "Field `gpo20_doen` writer - The selected OEN signal for GPIO20. The register value indicates the selected GPIO (Output Enable) OEN index from GPIO OEN list 0-49. See Table 2-41: GPIO OEN List for SYS_IOMUX (on page 97) for more information."]
pub type GPO20_DOEN_W<'a, REG> = crate::FieldWriter<'a, REG, 6>;
#[doc = "Field `gpo21_doen` reader - The selected OEN signal for GPIO21. The register value indicates the selected GPIO (Output Enable) OEN index from GPIO OEN list 0-49. See Table 2-41: GPIO OEN List for SYS_IOMUX (on page 97) for more information."]
pub type GPO21_DOEN_R = crate::FieldReader;
#[doc = "Field `gpo21_doen` writer - The selected OEN signal for GPIO21. The register value indicates the selected GPIO (Output Enable) OEN index from GPIO OEN list 0-49. See Table 2-41: GPIO OEN List for SYS_IOMUX (on page 97) for more information."]
pub type GPO21_DOEN_W<'a, REG> = crate::FieldWriter<'a, REG, 6>;
#[doc = "Field `gpo22_doen` reader - The selected OEN signal for GPIO22. The register value indicates the selected GPIO (Output Enable) OEN index from GPIO OEN list 0-49. See Table 2-41: GPIO OEN List for SYS_IOMUX (on page 97) for more information."]
pub type GPO22_DOEN_R = crate::FieldReader;
#[doc = "Field `gpo22_doen` writer - The selected OEN signal for GPIO22. The register value indicates the selected GPIO (Output Enable) OEN index from GPIO OEN list 0-49. See Table 2-41: GPIO OEN List for SYS_IOMUX (on page 97) for more information."]
pub type GPO22_DOEN_W<'a, REG> = crate::FieldWriter<'a, REG, 6>;
#[doc = "Field `gpo23_doen` reader - The selected OEN signal for GPIO23. The register value indicates the selected GPIO (Output Enable) OEN index from GPIO OEN list 0-49. See Table 2-41: GPIO OEN List for SYS_IOMUX (on page 97) for more information."]
pub type GPO23_DOEN_R = crate::FieldReader;
#[doc = "Field `gpo23_doen` writer - The selected OEN signal for GPIO23. The register value indicates the selected GPIO (Output Enable) OEN index from GPIO OEN list 0-49. See Table 2-41: GPIO OEN List for SYS_IOMUX (on page 97) for more information."]
pub type GPO23_DOEN_W<'a, REG> = crate::FieldWriter<'a, REG, 6>;
impl R {
    #[doc = "Bits 0:5 - The selected OEN signal for GPIO20. The register value indicates the selected GPIO (Output Enable) OEN index from GPIO OEN list 0-49. See Table 2-41: GPIO OEN List for SYS_IOMUX (on page 97) for more information."]
    #[inline(always)]
    pub fn gpo20_doen(&self) -> GPO20_DOEN_R {
        GPO20_DOEN_R::new((self.bits & 0x3f) as u8)
    }
    #[doc = "Bits 8:13 - The selected OEN signal for GPIO21. The register value indicates the selected GPIO (Output Enable) OEN index from GPIO OEN list 0-49. See Table 2-41: GPIO OEN List for SYS_IOMUX (on page 97) for more information."]
    #[inline(always)]
    pub fn gpo21_doen(&self) -> GPO21_DOEN_R {
        GPO21_DOEN_R::new(((self.bits >> 8) & 0x3f) as u8)
    }
    #[doc = "Bits 16:21 - The selected OEN signal for GPIO22. The register value indicates the selected GPIO (Output Enable) OEN index from GPIO OEN list 0-49. See Table 2-41: GPIO OEN List for SYS_IOMUX (on page 97) for more information."]
    #[inline(always)]
    pub fn gpo22_doen(&self) -> GPO22_DOEN_R {
        GPO22_DOEN_R::new(((self.bits >> 16) & 0x3f) as u8)
    }
    #[doc = "Bits 24:29 - The selected OEN signal for GPIO23. The register value indicates the selected GPIO (Output Enable) OEN index from GPIO OEN list 0-49. See Table 2-41: GPIO OEN List for SYS_IOMUX (on page 97) for more information."]
    #[inline(always)]
    pub fn gpo23_doen(&self) -> GPO23_DOEN_R {
        GPO23_DOEN_R::new(((self.bits >> 24) & 0x3f) as u8)
    }
}
impl W {
    #[doc = "Bits 0:5 - The selected OEN signal for GPIO20. The register value indicates the selected GPIO (Output Enable) OEN index from GPIO OEN list 0-49. See Table 2-41: GPIO OEN List for SYS_IOMUX (on page 97) for more information."]
    #[inline(always)]
    #[must_use]
    pub fn gpo20_doen(&mut self) -> GPO20_DOEN_W<GPO_DOEN5_SPEC> {
        GPO20_DOEN_W::new(self, 0)
    }
    #[doc = "Bits 8:13 - The selected OEN signal for GPIO21. The register value indicates the selected GPIO (Output Enable) OEN index from GPIO OEN list 0-49. See Table 2-41: GPIO OEN List for SYS_IOMUX (on page 97) for more information."]
    #[inline(always)]
    #[must_use]
    pub fn gpo21_doen(&mut self) -> GPO21_DOEN_W<GPO_DOEN5_SPEC> {
        GPO21_DOEN_W::new(self, 8)
    }
    #[doc = "Bits 16:21 - The selected OEN signal for GPIO22. The register value indicates the selected GPIO (Output Enable) OEN index from GPIO OEN list 0-49. See Table 2-41: GPIO OEN List for SYS_IOMUX (on page 97) for more information."]
    #[inline(always)]
    #[must_use]
    pub fn gpo22_doen(&mut self) -> GPO22_DOEN_W<GPO_DOEN5_SPEC> {
        GPO22_DOEN_W::new(self, 16)
    }
    #[doc = "Bits 24:29 - The selected OEN signal for GPIO23. The register value indicates the selected GPIO (Output Enable) OEN index from GPIO OEN list 0-49. See Table 2-41: GPIO OEN List for SYS_IOMUX (on page 97) for more information."]
    #[inline(always)]
    #[must_use]
    pub fn gpo23_doen(&mut self) -> GPO23_DOEN_W<GPO_DOEN5_SPEC> {
        GPO23_DOEN_W::new(self, 24)
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
#[doc = "SYS IOMUX CFG SAIF SYSCFG FMUX 5 DOEN\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`gpo_doen5::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`gpo_doen5::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct GPO_DOEN5_SPEC;
impl crate::RegisterSpec for GPO_DOEN5_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`gpo_doen5::R`](R) reader structure"]
impl crate::Readable for GPO_DOEN5_SPEC {}
#[doc = "`write(|w| ..)` method takes [`gpo_doen5::W`](W) writer structure"]
impl crate::Writable for GPO_DOEN5_SPEC {
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets gpo_doen5 to value 0"]
impl crate::Resettable for GPO_DOEN5_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
