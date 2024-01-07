#[doc = "Register `gpo_doen_10` reader"]
pub type R = crate::R<GPO_DOEN_10_SPEC>;
#[doc = "Register `gpo_doen_10` writer"]
pub type W = crate::W<GPO_DOEN_10_SPEC>;
#[doc = "Field `doen_40` reader - The selected OEN signal for GPIO40. The register value indicates the selected GPIO (Output Enable) OEN index from GPIO OEN list 0-49. See Table 2-41: GPIO OEN List for SYS_IOMUX (on page 97) for more information."]
pub type DOEN_40_R = crate::FieldReader;
#[doc = "Field `doen_40` writer - The selected OEN signal for GPIO40. The register value indicates the selected GPIO (Output Enable) OEN index from GPIO OEN list 0-49. See Table 2-41: GPIO OEN List for SYS_IOMUX (on page 97) for more information."]
pub type DOEN_40_W<'a, REG> = crate::FieldWriter<'a, REG, 6>;
#[doc = "Field `doen_41` reader - The selected OEN signal for GPIO41. The register value indicates the selected GPIO (Output Enable) OEN index from GPIO OEN list 0-49. See Table 2-41: GPIO OEN List for SYS_IOMUX (on page 97) for more information."]
pub type DOEN_41_R = crate::FieldReader;
#[doc = "Field `doen_41` writer - The selected OEN signal for GPIO41. The register value indicates the selected GPIO (Output Enable) OEN index from GPIO OEN list 0-49. See Table 2-41: GPIO OEN List for SYS_IOMUX (on page 97) for more information."]
pub type DOEN_41_W<'a, REG> = crate::FieldWriter<'a, REG, 6>;
#[doc = "Field `doen_42` reader - The selected OEN signal for GPIO42. The register value indicates the selected GPIO (Output Enable) OEN index from GPIO OEN list 0-49. See Table 2-41: GPIO OEN List for SYS_IOMUX (on page 97) for more information."]
pub type DOEN_42_R = crate::FieldReader;
#[doc = "Field `doen_42` writer - The selected OEN signal for GPIO42. The register value indicates the selected GPIO (Output Enable) OEN index from GPIO OEN list 0-49. See Table 2-41: GPIO OEN List for SYS_IOMUX (on page 97) for more information."]
pub type DOEN_42_W<'a, REG> = crate::FieldWriter<'a, REG, 6>;
#[doc = "Field `doen_43` reader - The selected OEN signal for GPIO43. The register value indicates the selected GPIO (Output Enable) OEN index from GPIO OEN list 0-49. See Table 2-41: GPIO OEN List for SYS_IOMUX (on page 97) for more information."]
pub type DOEN_43_R = crate::FieldReader;
#[doc = "Field `doen_43` writer - The selected OEN signal for GPIO43. The register value indicates the selected GPIO (Output Enable) OEN index from GPIO OEN list 0-49. See Table 2-41: GPIO OEN List for SYS_IOMUX (on page 97) for more information."]
pub type DOEN_43_W<'a, REG> = crate::FieldWriter<'a, REG, 6>;
impl R {
    #[doc = "Bits 0:5 - The selected OEN signal for GPIO40. The register value indicates the selected GPIO (Output Enable) OEN index from GPIO OEN list 0-49. See Table 2-41: GPIO OEN List for SYS_IOMUX (on page 97) for more information."]
    #[inline(always)]
    pub fn doen_40(&self) -> DOEN_40_R {
        DOEN_40_R::new((self.bits & 0x3f) as u8)
    }
    #[doc = "Bits 8:13 - The selected OEN signal for GPIO41. The register value indicates the selected GPIO (Output Enable) OEN index from GPIO OEN list 0-49. See Table 2-41: GPIO OEN List for SYS_IOMUX (on page 97) for more information."]
    #[inline(always)]
    pub fn doen_41(&self) -> DOEN_41_R {
        DOEN_41_R::new(((self.bits >> 8) & 0x3f) as u8)
    }
    #[doc = "Bits 16:21 - The selected OEN signal for GPIO42. The register value indicates the selected GPIO (Output Enable) OEN index from GPIO OEN list 0-49. See Table 2-41: GPIO OEN List for SYS_IOMUX (on page 97) for more information."]
    #[inline(always)]
    pub fn doen_42(&self) -> DOEN_42_R {
        DOEN_42_R::new(((self.bits >> 16) & 0x3f) as u8)
    }
    #[doc = "Bits 24:29 - The selected OEN signal for GPIO43. The register value indicates the selected GPIO (Output Enable) OEN index from GPIO OEN list 0-49. See Table 2-41: GPIO OEN List for SYS_IOMUX (on page 97) for more information."]
    #[inline(always)]
    pub fn doen_43(&self) -> DOEN_43_R {
        DOEN_43_R::new(((self.bits >> 24) & 0x3f) as u8)
    }
}
impl W {
    #[doc = "Bits 0:5 - The selected OEN signal for GPIO40. The register value indicates the selected GPIO (Output Enable) OEN index from GPIO OEN list 0-49. See Table 2-41: GPIO OEN List for SYS_IOMUX (on page 97) for more information."]
    #[inline(always)]
    #[must_use]
    pub fn doen_40(&mut self) -> DOEN_40_W<GPO_DOEN_10_SPEC> {
        DOEN_40_W::new(self, 0)
    }
    #[doc = "Bits 8:13 - The selected OEN signal for GPIO41. The register value indicates the selected GPIO (Output Enable) OEN index from GPIO OEN list 0-49. See Table 2-41: GPIO OEN List for SYS_IOMUX (on page 97) for more information."]
    #[inline(always)]
    #[must_use]
    pub fn doen_41(&mut self) -> DOEN_41_W<GPO_DOEN_10_SPEC> {
        DOEN_41_W::new(self, 8)
    }
    #[doc = "Bits 16:21 - The selected OEN signal for GPIO42. The register value indicates the selected GPIO (Output Enable) OEN index from GPIO OEN list 0-49. See Table 2-41: GPIO OEN List for SYS_IOMUX (on page 97) for more information."]
    #[inline(always)]
    #[must_use]
    pub fn doen_42(&mut self) -> DOEN_42_W<GPO_DOEN_10_SPEC> {
        DOEN_42_W::new(self, 16)
    }
    #[doc = "Bits 24:29 - The selected OEN signal for GPIO43. The register value indicates the selected GPIO (Output Enable) OEN index from GPIO OEN list 0-49. See Table 2-41: GPIO OEN List for SYS_IOMUX (on page 97) for more information."]
    #[inline(always)]
    #[must_use]
    pub fn doen_43(&mut self) -> DOEN_43_W<GPO_DOEN_10_SPEC> {
        DOEN_43_W::new(self, 24)
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
#[doc = "SYS IOMUX CFG SAIF SYSCFG FMUX 10 DOEN\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`gpo_doen_10::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`gpo_doen_10::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct GPO_DOEN_10_SPEC;
impl crate::RegisterSpec for GPO_DOEN_10_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`gpo_doen_10::R`](R) reader structure"]
impl crate::Readable for GPO_DOEN_10_SPEC {}
#[doc = "`write(|w| ..)` method takes [`gpo_doen_10::W`](W) writer structure"]
impl crate::Writable for GPO_DOEN_10_SPEC {
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets gpo_doen_10 to value 0x0100_0001"]
impl crate::Resettable for GPO_DOEN_10_SPEC {
    const RESET_VALUE: Self::Ux = 0x0100_0001;
}
