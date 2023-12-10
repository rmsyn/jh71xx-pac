#[doc = "Register `gpo_doen_12` reader"]
pub type R = crate::R<GPO_DOEN_12_SPEC>;
#[doc = "Register `gpo_doen_12` writer"]
pub type W = crate::W<GPO_DOEN_12_SPEC>;
#[doc = "Field `doen_48` reader - The selected OEN signal for GPIO48. The register value indicates the selected GPIO (Output Enable) OEN index from GPIO OEN list 0-49. See Table 2-41: GPIO OEN List for SYS_IOMUX (on page 97) for more information."]
pub type DOEN_48_R = crate::FieldReader;
#[doc = "Field `doen_48` writer - The selected OEN signal for GPIO48. The register value indicates the selected GPIO (Output Enable) OEN index from GPIO OEN list 0-49. See Table 2-41: GPIO OEN List for SYS_IOMUX (on page 97) for more information."]
pub type DOEN_48_W<'a, REG> = crate::FieldWriter<'a, REG, 6>;
#[doc = "Field `doen_49` reader - The selected OEN signal for GPIO49. The register value indicates the selected GPIO (Output Enable) OEN index from GPIO OEN list 0-49. See Table 2-41: GPIO OEN List for SYS_IOMUX (on page 97) for more information."]
pub type DOEN_49_R = crate::FieldReader;
#[doc = "Field `doen_49` writer - The selected OEN signal for GPIO49. The register value indicates the selected GPIO (Output Enable) OEN index from GPIO OEN list 0-49. See Table 2-41: GPIO OEN List for SYS_IOMUX (on page 97) for more information."]
pub type DOEN_49_W<'a, REG> = crate::FieldWriter<'a, REG, 6>;
#[doc = "Field `doen_50` reader - The selected OEN signal for GPIO50. The register value indicates the selected GPIO (Output Enable) OEN index from GPIO OEN list 0-49. See Table 2-41: GPIO OEN List for SYS_IOMUX (on page 97) for more information."]
pub type DOEN_50_R = crate::FieldReader;
#[doc = "Field `doen_50` writer - The selected OEN signal for GPIO50. The register value indicates the selected GPIO (Output Enable) OEN index from GPIO OEN list 0-49. See Table 2-41: GPIO OEN List for SYS_IOMUX (on page 97) for more information."]
pub type DOEN_50_W<'a, REG> = crate::FieldWriter<'a, REG, 6>;
#[doc = "Field `doen_51` reader - The selected OEN signal for GPIO51. The register value indicates the selected GPIO (Output Enable) OEN index from GPIO OEN list 0-49. See Table 2-41: GPIO OEN List for SYS_IOMUX (on page 97) for more information."]
pub type DOEN_51_R = crate::FieldReader;
#[doc = "Field `doen_51` writer - The selected OEN signal for GPIO51. The register value indicates the selected GPIO (Output Enable) OEN index from GPIO OEN list 0-49. See Table 2-41: GPIO OEN List for SYS_IOMUX (on page 97) for more information."]
pub type DOEN_51_W<'a, REG> = crate::FieldWriter<'a, REG, 6>;
impl R {
    #[doc = "Bits 0:5 - The selected OEN signal for GPIO48. The register value indicates the selected GPIO (Output Enable) OEN index from GPIO OEN list 0-49. See Table 2-41: GPIO OEN List for SYS_IOMUX (on page 97) for more information."]
    #[inline(always)]
    pub fn doen_48(&self) -> DOEN_48_R {
        DOEN_48_R::new((self.bits & 0x3f) as u8)
    }
    #[doc = "Bits 8:13 - The selected OEN signal for GPIO49. The register value indicates the selected GPIO (Output Enable) OEN index from GPIO OEN list 0-49. See Table 2-41: GPIO OEN List for SYS_IOMUX (on page 97) for more information."]
    #[inline(always)]
    pub fn doen_49(&self) -> DOEN_49_R {
        DOEN_49_R::new(((self.bits >> 8) & 0x3f) as u8)
    }
    #[doc = "Bits 16:21 - The selected OEN signal for GPIO50. The register value indicates the selected GPIO (Output Enable) OEN index from GPIO OEN list 0-49. See Table 2-41: GPIO OEN List for SYS_IOMUX (on page 97) for more information."]
    #[inline(always)]
    pub fn doen_50(&self) -> DOEN_50_R {
        DOEN_50_R::new(((self.bits >> 16) & 0x3f) as u8)
    }
    #[doc = "Bits 24:29 - The selected OEN signal for GPIO51. The register value indicates the selected GPIO (Output Enable) OEN index from GPIO OEN list 0-49. See Table 2-41: GPIO OEN List for SYS_IOMUX (on page 97) for more information."]
    #[inline(always)]
    pub fn doen_51(&self) -> DOEN_51_R {
        DOEN_51_R::new(((self.bits >> 24) & 0x3f) as u8)
    }
}
impl W {
    #[doc = "Bits 0:5 - The selected OEN signal for GPIO48. The register value indicates the selected GPIO (Output Enable) OEN index from GPIO OEN list 0-49. See Table 2-41: GPIO OEN List for SYS_IOMUX (on page 97) for more information."]
    #[inline(always)]
    #[must_use]
    pub fn doen_48(&mut self) -> DOEN_48_W<GPO_DOEN_12_SPEC> {
        DOEN_48_W::new(self, 0)
    }
    #[doc = "Bits 8:13 - The selected OEN signal for GPIO49. The register value indicates the selected GPIO (Output Enable) OEN index from GPIO OEN list 0-49. See Table 2-41: GPIO OEN List for SYS_IOMUX (on page 97) for more information."]
    #[inline(always)]
    #[must_use]
    pub fn doen_49(&mut self) -> DOEN_49_W<GPO_DOEN_12_SPEC> {
        DOEN_49_W::new(self, 8)
    }
    #[doc = "Bits 16:21 - The selected OEN signal for GPIO50. The register value indicates the selected GPIO (Output Enable) OEN index from GPIO OEN list 0-49. See Table 2-41: GPIO OEN List for SYS_IOMUX (on page 97) for more information."]
    #[inline(always)]
    #[must_use]
    pub fn doen_50(&mut self) -> DOEN_50_W<GPO_DOEN_12_SPEC> {
        DOEN_50_W::new(self, 16)
    }
    #[doc = "Bits 24:29 - The selected OEN signal for GPIO51. The register value indicates the selected GPIO (Output Enable) OEN index from GPIO OEN list 0-49. See Table 2-41: GPIO OEN List for SYS_IOMUX (on page 97) for more information."]
    #[inline(always)]
    #[must_use]
    pub fn doen_51(&mut self) -> DOEN_51_W<GPO_DOEN_12_SPEC> {
        DOEN_51_W::new(self, 24)
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
#[doc = "SYS IOMUX CFG SAIF SYSCFG FMUX 12 DOEN\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`gpo_doen_12::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`gpo_doen_12::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct GPO_DOEN_12_SPEC;
impl crate::RegisterSpec for GPO_DOEN_12_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`gpo_doen_12::R`](R) reader structure"]
impl crate::Readable for GPO_DOEN_12_SPEC {}
#[doc = "`write(|w| ..)` method takes [`gpo_doen_12::W`](W) writer structure"]
impl crate::Writable for GPO_DOEN_12_SPEC {
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets gpo_doen_12 to value 0x0e01_0d0d"]
impl crate::Resettable for GPO_DOEN_12_SPEC {
    const RESET_VALUE: Self::Ux = 0x0e01_0d0d;
}
