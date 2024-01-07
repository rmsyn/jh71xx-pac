#[doc = "Register `gpo_doen_14` reader"]
pub type R = crate::R<GPO_DOEN_14_SPEC>;
#[doc = "Register `gpo_doen_14` writer"]
pub type W = crate::W<GPO_DOEN_14_SPEC>;
#[doc = "Field `doen_56` reader - The selected OEN signal for GPIO56. The register value indicates the selected GPIO (Output Enable) OEN index from GPIO OEN list 0-49. See Table 2-41: GPIO OEN List for SYS_IOMUX (on page 97) for more information."]
pub type DOEN_56_R = crate::FieldReader;
#[doc = "Field `doen_56` writer - The selected OEN signal for GPIO56. The register value indicates the selected GPIO (Output Enable) OEN index from GPIO OEN list 0-49. See Table 2-41: GPIO OEN List for SYS_IOMUX (on page 97) for more information."]
pub type DOEN_56_W<'a, REG> = crate::FieldWriter<'a, REG, 6>;
#[doc = "Field `doen_57` reader - The selected OEN signal for GPIO57. The register value indicates the selected GPIO (Output Enable) OEN index from GPIO OEN list 0-49. See Table 2-41: GPIO OEN List for SYS_IOMUX (on page 97) for more information."]
pub type DOEN_57_R = crate::FieldReader;
#[doc = "Field `doen_57` writer - The selected OEN signal for GPIO57. The register value indicates the selected GPIO (Output Enable) OEN index from GPIO OEN list 0-49. See Table 2-41: GPIO OEN List for SYS_IOMUX (on page 97) for more information."]
pub type DOEN_57_W<'a, REG> = crate::FieldWriter<'a, REG, 6>;
#[doc = "Field `doen_58` reader - The selected OEN signal for GPIO58. The register value indicates the selected GPIO (Output Enable) OEN index from GPIO OEN list 0-49. See Table 2-41: GPIO OEN List for SYS_IOMUX (on page 97) for more information."]
pub type DOEN_58_R = crate::FieldReader;
#[doc = "Field `doen_58` writer - The selected OEN signal for GPIO58. The register value indicates the selected GPIO (Output Enable) OEN index from GPIO OEN list 0-49. See Table 2-41: GPIO OEN List for SYS_IOMUX (on page 97) for more information."]
pub type DOEN_58_W<'a, REG> = crate::FieldWriter<'a, REG, 6>;
#[doc = "Field `doen_59` reader - The selected OEN signal for GPIO59. The register value indicates the selected GPIO (Output Enable) OEN index from GPIO OEN list 0-49. See Table 2-41: GPIO OEN List for SYS_IOMUX (on page 97) for more information."]
pub type DOEN_59_R = crate::FieldReader;
#[doc = "Field `doen_59` writer - The selected OEN signal for GPIO59. The register value indicates the selected GPIO (Output Enable) OEN index from GPIO OEN list 0-49. See Table 2-41: GPIO OEN List for SYS_IOMUX (on page 97) for more information."]
pub type DOEN_59_W<'a, REG> = crate::FieldWriter<'a, REG, 6>;
impl R {
    #[doc = "Bits 0:5 - The selected OEN signal for GPIO56. The register value indicates the selected GPIO (Output Enable) OEN index from GPIO OEN list 0-49. See Table 2-41: GPIO OEN List for SYS_IOMUX (on page 97) for more information."]
    #[inline(always)]
    pub fn doen_56(&self) -> DOEN_56_R {
        DOEN_56_R::new((self.bits & 0x3f) as u8)
    }
    #[doc = "Bits 8:13 - The selected OEN signal for GPIO57. The register value indicates the selected GPIO (Output Enable) OEN index from GPIO OEN list 0-49. See Table 2-41: GPIO OEN List for SYS_IOMUX (on page 97) for more information."]
    #[inline(always)]
    pub fn doen_57(&self) -> DOEN_57_R {
        DOEN_57_R::new(((self.bits >> 8) & 0x3f) as u8)
    }
    #[doc = "Bits 16:21 - The selected OEN signal for GPIO58. The register value indicates the selected GPIO (Output Enable) OEN index from GPIO OEN list 0-49. See Table 2-41: GPIO OEN List for SYS_IOMUX (on page 97) for more information."]
    #[inline(always)]
    pub fn doen_58(&self) -> DOEN_58_R {
        DOEN_58_R::new(((self.bits >> 16) & 0x3f) as u8)
    }
    #[doc = "Bits 24:29 - The selected OEN signal for GPIO59. The register value indicates the selected GPIO (Output Enable) OEN index from GPIO OEN list 0-49. See Table 2-41: GPIO OEN List for SYS_IOMUX (on page 97) for more information."]
    #[inline(always)]
    pub fn doen_59(&self) -> DOEN_59_R {
        DOEN_59_R::new(((self.bits >> 24) & 0x3f) as u8)
    }
}
impl W {
    #[doc = "Bits 0:5 - The selected OEN signal for GPIO56. The register value indicates the selected GPIO (Output Enable) OEN index from GPIO OEN list 0-49. See Table 2-41: GPIO OEN List for SYS_IOMUX (on page 97) for more information."]
    #[inline(always)]
    #[must_use]
    pub fn doen_56(&mut self) -> DOEN_56_W<GPO_DOEN_14_SPEC> {
        DOEN_56_W::new(self, 0)
    }
    #[doc = "Bits 8:13 - The selected OEN signal for GPIO57. The register value indicates the selected GPIO (Output Enable) OEN index from GPIO OEN list 0-49. See Table 2-41: GPIO OEN List for SYS_IOMUX (on page 97) for more information."]
    #[inline(always)]
    #[must_use]
    pub fn doen_57(&mut self) -> DOEN_57_W<GPO_DOEN_14_SPEC> {
        DOEN_57_W::new(self, 8)
    }
    #[doc = "Bits 16:21 - The selected OEN signal for GPIO58. The register value indicates the selected GPIO (Output Enable) OEN index from GPIO OEN list 0-49. See Table 2-41: GPIO OEN List for SYS_IOMUX (on page 97) for more information."]
    #[inline(always)]
    #[must_use]
    pub fn doen_58(&mut self) -> DOEN_58_W<GPO_DOEN_14_SPEC> {
        DOEN_58_W::new(self, 16)
    }
    #[doc = "Bits 24:29 - The selected OEN signal for GPIO59. The register value indicates the selected GPIO (Output Enable) OEN index from GPIO OEN list 0-49. See Table 2-41: GPIO OEN List for SYS_IOMUX (on page 97) for more information."]
    #[inline(always)]
    #[must_use]
    pub fn doen_59(&mut self) -> DOEN_59_W<GPO_DOEN_14_SPEC> {
        DOEN_59_W::new(self, 24)
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
#[doc = "SYS IOMUX CFG SAIF SYSCFG FMUX 14 DOEN\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`gpo_doen_14::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`gpo_doen_14::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct GPO_DOEN_14_SPEC;
impl crate::RegisterSpec for GPO_DOEN_14_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`gpo_doen_14::R`](R) reader structure"]
impl crate::Readable for GPO_DOEN_14_SPEC {}
#[doc = "`write(|w| ..)` method takes [`gpo_doen_14::W`](W) writer structure"]
impl crate::Writable for GPO_DOEN_14_SPEC {
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets gpo_doen_14 to value 0x2501_2424"]
impl crate::Resettable for GPO_DOEN_14_SPEC {
    const RESET_VALUE: Self::Ux = 0x2501_2424;
}
