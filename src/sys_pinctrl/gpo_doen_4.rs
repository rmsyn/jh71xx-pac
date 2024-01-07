#[doc = "Register `gpo_doen_4` reader"]
pub type R = crate::R<GPO_DOEN_4_SPEC>;
#[doc = "Register `gpo_doen_4` writer"]
pub type W = crate::W<GPO_DOEN_4_SPEC>;
#[doc = "Field `doen_16` reader - The selected OEN signal for GPIO16. The register value indicates the selected GPIO (Output Enable) OEN index from GPIO OEN list 0-49. See Table 2-41: GPIO OEN List for SYS_IOMUX (on page 97) for more information."]
pub type DOEN_16_R = crate::FieldReader;
#[doc = "Field `doen_16` writer - The selected OEN signal for GPIO16. The register value indicates the selected GPIO (Output Enable) OEN index from GPIO OEN list 0-49. See Table 2-41: GPIO OEN List for SYS_IOMUX (on page 97) for more information."]
pub type DOEN_16_W<'a, REG> = crate::FieldWriter<'a, REG, 6>;
#[doc = "Field `doen_17` reader - The selected OEN signal for GPIO17. The register value indicates the selected GPIO (Output Enable) OEN index from GPIO OEN list 0-49. See Table 2-41: GPIO OEN List for SYS_IOMUX (on page 97) for more information."]
pub type DOEN_17_R = crate::FieldReader;
#[doc = "Field `doen_17` writer - The selected OEN signal for GPIO17. The register value indicates the selected GPIO (Output Enable) OEN index from GPIO OEN list 0-49. See Table 2-41: GPIO OEN List for SYS_IOMUX (on page 97) for more information."]
pub type DOEN_17_W<'a, REG> = crate::FieldWriter<'a, REG, 6>;
#[doc = "Field `doen_18` reader - The selected OEN signal for GPIO18. The register value indicates the selected GPIO (Output Enable) OEN index from GPIO OEN list 0-49. See Table 2-41: GPIO OEN List for SYS_IOMUX (on page 97) for more information."]
pub type DOEN_18_R = crate::FieldReader;
#[doc = "Field `doen_18` writer - The selected OEN signal for GPIO18. The register value indicates the selected GPIO (Output Enable) OEN index from GPIO OEN list 0-49. See Table 2-41: GPIO OEN List for SYS_IOMUX (on page 97) for more information."]
pub type DOEN_18_W<'a, REG> = crate::FieldWriter<'a, REG, 6>;
#[doc = "Field `doen_19` reader - The selected OEN signal for GPIO19. The register value indicates the selected GPIO (Output Enable) OEN index from GPIO OEN list 0-49. See Table 2-41: GPIO OEN List for SYS_IOMUX (on page 97) for more information."]
pub type DOEN_19_R = crate::FieldReader;
#[doc = "Field `doen_19` writer - The selected OEN signal for GPIO19. The register value indicates the selected GPIO (Output Enable) OEN index from GPIO OEN list 0-49. See Table 2-41: GPIO OEN List for SYS_IOMUX (on page 97) for more information."]
pub type DOEN_19_W<'a, REG> = crate::FieldWriter<'a, REG, 6>;
impl R {
    #[doc = "Bits 0:5 - The selected OEN signal for GPIO16. The register value indicates the selected GPIO (Output Enable) OEN index from GPIO OEN list 0-49. See Table 2-41: GPIO OEN List for SYS_IOMUX (on page 97) for more information."]
    #[inline(always)]
    pub fn doen_16(&self) -> DOEN_16_R {
        DOEN_16_R::new((self.bits & 0x3f) as u8)
    }
    #[doc = "Bits 8:13 - The selected OEN signal for GPIO17. The register value indicates the selected GPIO (Output Enable) OEN index from GPIO OEN list 0-49. See Table 2-41: GPIO OEN List for SYS_IOMUX (on page 97) for more information."]
    #[inline(always)]
    pub fn doen_17(&self) -> DOEN_17_R {
        DOEN_17_R::new(((self.bits >> 8) & 0x3f) as u8)
    }
    #[doc = "Bits 16:21 - The selected OEN signal for GPIO18. The register value indicates the selected GPIO (Output Enable) OEN index from GPIO OEN list 0-49. See Table 2-41: GPIO OEN List for SYS_IOMUX (on page 97) for more information."]
    #[inline(always)]
    pub fn doen_18(&self) -> DOEN_18_R {
        DOEN_18_R::new(((self.bits >> 16) & 0x3f) as u8)
    }
    #[doc = "Bits 24:29 - The selected OEN signal for GPIO19. The register value indicates the selected GPIO (Output Enable) OEN index from GPIO OEN list 0-49. See Table 2-41: GPIO OEN List for SYS_IOMUX (on page 97) for more information."]
    #[inline(always)]
    pub fn doen_19(&self) -> DOEN_19_R {
        DOEN_19_R::new(((self.bits >> 24) & 0x3f) as u8)
    }
}
impl W {
    #[doc = "Bits 0:5 - The selected OEN signal for GPIO16. The register value indicates the selected GPIO (Output Enable) OEN index from GPIO OEN list 0-49. See Table 2-41: GPIO OEN List for SYS_IOMUX (on page 97) for more information."]
    #[inline(always)]
    #[must_use]
    pub fn doen_16(&mut self) -> DOEN_16_W<GPO_DOEN_4_SPEC> {
        DOEN_16_W::new(self, 0)
    }
    #[doc = "Bits 8:13 - The selected OEN signal for GPIO17. The register value indicates the selected GPIO (Output Enable) OEN index from GPIO OEN list 0-49. See Table 2-41: GPIO OEN List for SYS_IOMUX (on page 97) for more information."]
    #[inline(always)]
    #[must_use]
    pub fn doen_17(&mut self) -> DOEN_17_W<GPO_DOEN_4_SPEC> {
        DOEN_17_W::new(self, 8)
    }
    #[doc = "Bits 16:21 - The selected OEN signal for GPIO18. The register value indicates the selected GPIO (Output Enable) OEN index from GPIO OEN list 0-49. See Table 2-41: GPIO OEN List for SYS_IOMUX (on page 97) for more information."]
    #[inline(always)]
    #[must_use]
    pub fn doen_18(&mut self) -> DOEN_18_W<GPO_DOEN_4_SPEC> {
        DOEN_18_W::new(self, 16)
    }
    #[doc = "Bits 24:29 - The selected OEN signal for GPIO19. The register value indicates the selected GPIO (Output Enable) OEN index from GPIO OEN list 0-49. See Table 2-41: GPIO OEN List for SYS_IOMUX (on page 97) for more information."]
    #[inline(always)]
    #[must_use]
    pub fn doen_19(&mut self) -> DOEN_19_W<GPO_DOEN_4_SPEC> {
        DOEN_19_W::new(self, 24)
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
#[doc = "SYS IOMUX CFG SAIF SYSCFG FMUX 4 DOEN\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`gpo_doen_4::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`gpo_doen_4::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct GPO_DOEN_4_SPEC;
impl crate::RegisterSpec for GPO_DOEN_4_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`gpo_doen_4::R`](R) reader structure"]
impl crate::Readable for GPO_DOEN_4_SPEC {}
#[doc = "`write(|w| ..)` method takes [`gpo_doen_4::W`](W) writer structure"]
impl crate::Writable for GPO_DOEN_4_SPEC {
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets gpo_doen_4 to value 0x0100_0000"]
impl crate::Resettable for GPO_DOEN_4_SPEC {
    const RESET_VALUE: Self::Ux = 0x0100_0000;
}
