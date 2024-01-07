#[doc = "Register `gpo_doen_7` reader"]
pub type R = crate::R<GPO_DOEN_7_SPEC>;
#[doc = "Register `gpo_doen_7` writer"]
pub type W = crate::W<GPO_DOEN_7_SPEC>;
#[doc = "Field `doen_28` reader - The selected OEN signal for GPIO28. The register value indicates the selected GPIO (Output Enable) OEN index from GPIO OEN list 0-49. See Table 2-41: GPIO OEN List for SYS_IOMUX (on page 97) for more information."]
pub type DOEN_28_R = crate::FieldReader;
#[doc = "Field `doen_28` writer - The selected OEN signal for GPIO28. The register value indicates the selected GPIO (Output Enable) OEN index from GPIO OEN list 0-49. See Table 2-41: GPIO OEN List for SYS_IOMUX (on page 97) for more information."]
pub type DOEN_28_W<'a, REG> = crate::FieldWriter<'a, REG, 6>;
#[doc = "Field `doen_29` reader - The selected OEN signal for GPIO29. The register value indicates the selected GPIO (Output Enable) OEN index from GPIO OEN list 0-49. See Table 2-41: GPIO OEN List for SYS_IOMUX (on page 97) for more information."]
pub type DOEN_29_R = crate::FieldReader;
#[doc = "Field `doen_29` writer - The selected OEN signal for GPIO29. The register value indicates the selected GPIO (Output Enable) OEN index from GPIO OEN list 0-49. See Table 2-41: GPIO OEN List for SYS_IOMUX (on page 97) for more information."]
pub type DOEN_29_W<'a, REG> = crate::FieldWriter<'a, REG, 6>;
#[doc = "Field `doen_30` reader - The selected OEN signal for GPIO30. The register value indicates the selected GPIO (Output Enable) OEN index from GPIO OEN list 0-49. See Table 2-41: GPIO OEN List for SYS_IOMUX (on page 97) for more information."]
pub type DOEN_30_R = crate::FieldReader;
#[doc = "Field `doen_30` writer - The selected OEN signal for GPIO30. The register value indicates the selected GPIO (Output Enable) OEN index from GPIO OEN list 0-49. See Table 2-41: GPIO OEN List for SYS_IOMUX (on page 97) for more information."]
pub type DOEN_30_W<'a, REG> = crate::FieldWriter<'a, REG, 6>;
#[doc = "Field `doen_31` reader - The selected OEN signal for GPIO31. The register value indicates the selected GPIO (Output Enable) OEN index from GPIO OEN list 0-49. See Table 2-41: GPIO OEN List for SYS_IOMUX (on page 97) for more information."]
pub type DOEN_31_R = crate::FieldReader;
#[doc = "Field `doen_31` writer - The selected OEN signal for GPIO31. The register value indicates the selected GPIO (Output Enable) OEN index from GPIO OEN list 0-49. See Table 2-41: GPIO OEN List for SYS_IOMUX (on page 97) for more information."]
pub type DOEN_31_W<'a, REG> = crate::FieldWriter<'a, REG, 6>;
impl R {
    #[doc = "Bits 0:5 - The selected OEN signal for GPIO28. The register value indicates the selected GPIO (Output Enable) OEN index from GPIO OEN list 0-49. See Table 2-41: GPIO OEN List for SYS_IOMUX (on page 97) for more information."]
    #[inline(always)]
    pub fn doen_28(&self) -> DOEN_28_R {
        DOEN_28_R::new((self.bits & 0x3f) as u8)
    }
    #[doc = "Bits 8:13 - The selected OEN signal for GPIO29. The register value indicates the selected GPIO (Output Enable) OEN index from GPIO OEN list 0-49. See Table 2-41: GPIO OEN List for SYS_IOMUX (on page 97) for more information."]
    #[inline(always)]
    pub fn doen_29(&self) -> DOEN_29_R {
        DOEN_29_R::new(((self.bits >> 8) & 0x3f) as u8)
    }
    #[doc = "Bits 16:21 - The selected OEN signal for GPIO30. The register value indicates the selected GPIO (Output Enable) OEN index from GPIO OEN list 0-49. See Table 2-41: GPIO OEN List for SYS_IOMUX (on page 97) for more information."]
    #[inline(always)]
    pub fn doen_30(&self) -> DOEN_30_R {
        DOEN_30_R::new(((self.bits >> 16) & 0x3f) as u8)
    }
    #[doc = "Bits 24:29 - The selected OEN signal for GPIO31. The register value indicates the selected GPIO (Output Enable) OEN index from GPIO OEN list 0-49. See Table 2-41: GPIO OEN List for SYS_IOMUX (on page 97) for more information."]
    #[inline(always)]
    pub fn doen_31(&self) -> DOEN_31_R {
        DOEN_31_R::new(((self.bits >> 24) & 0x3f) as u8)
    }
}
impl W {
    #[doc = "Bits 0:5 - The selected OEN signal for GPIO28. The register value indicates the selected GPIO (Output Enable) OEN index from GPIO OEN list 0-49. See Table 2-41: GPIO OEN List for SYS_IOMUX (on page 97) for more information."]
    #[inline(always)]
    #[must_use]
    pub fn doen_28(&mut self) -> DOEN_28_W<GPO_DOEN_7_SPEC> {
        DOEN_28_W::new(self, 0)
    }
    #[doc = "Bits 8:13 - The selected OEN signal for GPIO29. The register value indicates the selected GPIO (Output Enable) OEN index from GPIO OEN list 0-49. See Table 2-41: GPIO OEN List for SYS_IOMUX (on page 97) for more information."]
    #[inline(always)]
    #[must_use]
    pub fn doen_29(&mut self) -> DOEN_29_W<GPO_DOEN_7_SPEC> {
        DOEN_29_W::new(self, 8)
    }
    #[doc = "Bits 16:21 - The selected OEN signal for GPIO30. The register value indicates the selected GPIO (Output Enable) OEN index from GPIO OEN list 0-49. See Table 2-41: GPIO OEN List for SYS_IOMUX (on page 97) for more information."]
    #[inline(always)]
    #[must_use]
    pub fn doen_30(&mut self) -> DOEN_30_W<GPO_DOEN_7_SPEC> {
        DOEN_30_W::new(self, 16)
    }
    #[doc = "Bits 24:29 - The selected OEN signal for GPIO31. The register value indicates the selected GPIO (Output Enable) OEN index from GPIO OEN list 0-49. See Table 2-41: GPIO OEN List for SYS_IOMUX (on page 97) for more information."]
    #[inline(always)]
    #[must_use]
    pub fn doen_31(&mut self) -> DOEN_31_W<GPO_DOEN_7_SPEC> {
        DOEN_31_W::new(self, 24)
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
#[doc = "SYS IOMUX CFG SAIF SYSCFG FMUX 7 DOEN\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`gpo_doen_7::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`gpo_doen_7::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct GPO_DOEN_7_SPEC;
impl crate::RegisterSpec for GPO_DOEN_7_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`gpo_doen_7::R`](R) reader structure"]
impl crate::Readable for GPO_DOEN_7_SPEC {}
#[doc = "`write(|w| ..)` method takes [`gpo_doen_7::W`](W) writer structure"]
impl crate::Writable for GPO_DOEN_7_SPEC {
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets gpo_doen_7 to value 0"]
impl crate::Resettable for GPO_DOEN_7_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
