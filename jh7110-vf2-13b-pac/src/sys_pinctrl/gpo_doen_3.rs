#[doc = "Register `gpo_doen_3` reader"]
pub type R = crate::R<GPO_DOEN_3_SPEC>;
#[doc = "Register `gpo_doen_3` writer"]
pub type W = crate::W<GPO_DOEN_3_SPEC>;
#[doc = "Field `doen_12` reader - The selected OEN signal for GPIO12. The register value indicates the selected GPIO (Output Enable) OEN index from GPIO OEN list 0-49. See Table 2-41: GPIO OEN List for SYS_IOMUX (on page 97) for more information."]
pub type DOEN_12_R = crate::FieldReader;
#[doc = "Field `doen_12` writer - The selected OEN signal for GPIO12. The register value indicates the selected GPIO (Output Enable) OEN index from GPIO OEN list 0-49. See Table 2-41: GPIO OEN List for SYS_IOMUX (on page 97) for more information."]
pub type DOEN_12_W<'a, REG> = crate::FieldWriter<'a, REG, 6>;
#[doc = "Field `doen_13` reader - The selected OEN signal for GPIO13. The register value indicates the selected GPIO (Output Enable) OEN index from GPIO OEN list 0-49. See Table 2-41: GPIO OEN List for SYS_IOMUX (on page 97) for more information."]
pub type DOEN_13_R = crate::FieldReader;
#[doc = "Field `doen_13` writer - The selected OEN signal for GPIO13. The register value indicates the selected GPIO (Output Enable) OEN index from GPIO OEN list 0-49. See Table 2-41: GPIO OEN List for SYS_IOMUX (on page 97) for more information."]
pub type DOEN_13_W<'a, REG> = crate::FieldWriter<'a, REG, 6>;
#[doc = "Field `doen_14` reader - The selected OEN signal for GPIO14. The register value indicates the selected GPIO (Output Enable) OEN index from GPIO OEN list 0-49. See Table 2-41: GPIO OEN List for SYS_IOMUX (on page 97) for more information."]
pub type DOEN_14_R = crate::FieldReader;
#[doc = "Field `doen_14` writer - The selected OEN signal for GPIO14. The register value indicates the selected GPIO (Output Enable) OEN index from GPIO OEN list 0-49. See Table 2-41: GPIO OEN List for SYS_IOMUX (on page 97) for more information."]
pub type DOEN_14_W<'a, REG> = crate::FieldWriter<'a, REG, 6>;
#[doc = "Field `doen_15` reader - The selected OEN signal for GPIO15. The register value indicates the selected GPIO (Output Enable) OEN index from GPIO OEN list 0-49. See Table 2-41: GPIO OEN List for SYS_IOMUX (on page 97) for more information."]
pub type DOEN_15_R = crate::FieldReader;
#[doc = "Field `doen_15` writer - The selected OEN signal for GPIO15. The register value indicates the selected GPIO (Output Enable) OEN index from GPIO OEN list 0-49. See Table 2-41: GPIO OEN List for SYS_IOMUX (on page 97) for more information."]
pub type DOEN_15_W<'a, REG> = crate::FieldWriter<'a, REG, 6>;
impl R {
    #[doc = "Bits 0:5 - The selected OEN signal for GPIO12. The register value indicates the selected GPIO (Output Enable) OEN index from GPIO OEN list 0-49. See Table 2-41: GPIO OEN List for SYS_IOMUX (on page 97) for more information."]
    #[inline(always)]
    pub fn doen_12(&self) -> DOEN_12_R {
        DOEN_12_R::new((self.bits & 0x3f) as u8)
    }
    #[doc = "Bits 8:13 - The selected OEN signal for GPIO13. The register value indicates the selected GPIO (Output Enable) OEN index from GPIO OEN list 0-49. See Table 2-41: GPIO OEN List for SYS_IOMUX (on page 97) for more information."]
    #[inline(always)]
    pub fn doen_13(&self) -> DOEN_13_R {
        DOEN_13_R::new(((self.bits >> 8) & 0x3f) as u8)
    }
    #[doc = "Bits 16:21 - The selected OEN signal for GPIO14. The register value indicates the selected GPIO (Output Enable) OEN index from GPIO OEN list 0-49. See Table 2-41: GPIO OEN List for SYS_IOMUX (on page 97) for more information."]
    #[inline(always)]
    pub fn doen_14(&self) -> DOEN_14_R {
        DOEN_14_R::new(((self.bits >> 16) & 0x3f) as u8)
    }
    #[doc = "Bits 24:29 - The selected OEN signal for GPIO15. The register value indicates the selected GPIO (Output Enable) OEN index from GPIO OEN list 0-49. See Table 2-41: GPIO OEN List for SYS_IOMUX (on page 97) for more information."]
    #[inline(always)]
    pub fn doen_15(&self) -> DOEN_15_R {
        DOEN_15_R::new(((self.bits >> 24) & 0x3f) as u8)
    }
}
impl W {
    #[doc = "Bits 0:5 - The selected OEN signal for GPIO12. The register value indicates the selected GPIO (Output Enable) OEN index from GPIO OEN list 0-49. See Table 2-41: GPIO OEN List for SYS_IOMUX (on page 97) for more information."]
    #[inline(always)]
    #[must_use]
    pub fn doen_12(&mut self) -> DOEN_12_W<GPO_DOEN_3_SPEC> {
        DOEN_12_W::new(self, 0)
    }
    #[doc = "Bits 8:13 - The selected OEN signal for GPIO13. The register value indicates the selected GPIO (Output Enable) OEN index from GPIO OEN list 0-49. See Table 2-41: GPIO OEN List for SYS_IOMUX (on page 97) for more information."]
    #[inline(always)]
    #[must_use]
    pub fn doen_13(&mut self) -> DOEN_13_W<GPO_DOEN_3_SPEC> {
        DOEN_13_W::new(self, 8)
    }
    #[doc = "Bits 16:21 - The selected OEN signal for GPIO14. The register value indicates the selected GPIO (Output Enable) OEN index from GPIO OEN list 0-49. See Table 2-41: GPIO OEN List for SYS_IOMUX (on page 97) for more information."]
    #[inline(always)]
    #[must_use]
    pub fn doen_14(&mut self) -> DOEN_14_W<GPO_DOEN_3_SPEC> {
        DOEN_14_W::new(self, 16)
    }
    #[doc = "Bits 24:29 - The selected OEN signal for GPIO15. The register value indicates the selected GPIO (Output Enable) OEN index from GPIO OEN list 0-49. See Table 2-41: GPIO OEN List for SYS_IOMUX (on page 97) for more information."]
    #[inline(always)]
    #[must_use]
    pub fn doen_15(&mut self) -> DOEN_15_W<GPO_DOEN_3_SPEC> {
        DOEN_15_W::new(self, 24)
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
#[doc = "SYS IOMUX CFG SAIF SYSCFG FMUX 3 DOEN\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`gpo_doen_3::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`gpo_doen_3::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct GPO_DOEN_3_SPEC;
impl crate::RegisterSpec for GPO_DOEN_3_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`gpo_doen_3::R`](R) reader structure"]
impl crate::Readable for GPO_DOEN_3_SPEC {}
#[doc = "`write(|w| ..)` method takes [`gpo_doen_3::W`](W) writer structure"]
impl crate::Writable for GPO_DOEN_3_SPEC {
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets gpo_doen_3 to value 0x0101"]
impl crate::Resettable for GPO_DOEN_3_SPEC {
    const RESET_VALUE: Self::Ux = 0x0101;
}
