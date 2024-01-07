#[doc = "Register `gpo_doen_15` reader"]
pub type R = crate::R<GPO_DOEN_15_SPEC>;
#[doc = "Register `gpo_doen_15` writer"]
pub type W = crate::W<GPO_DOEN_15_SPEC>;
#[doc = "Field `doen_60` reader - The selected OEN signal for GPIO60. The register value indicates the selected GPIO (Output Enable) OEN index from GPIO OEN list 0-49. See Table 2-41: GPIO OEN List for SYS_IOMUX (on page 97) for more information."]
pub type DOEN_60_R = crate::FieldReader;
#[doc = "Field `doen_60` writer - The selected OEN signal for GPIO60. The register value indicates the selected GPIO (Output Enable) OEN index from GPIO OEN list 0-49. See Table 2-41: GPIO OEN List for SYS_IOMUX (on page 97) for more information."]
pub type DOEN_60_W<'a, REG> = crate::FieldWriter<'a, REG, 6>;
#[doc = "Field `doen_61` reader - The selected OEN signal for GPIO61. The register value indicates the selected GPIO (Output Enable) OEN index from GPIO OEN list 0-49. See Table 2-41: GPIO OEN List for SYS_IOMUX (on page 97) for more information."]
pub type DOEN_61_R = crate::FieldReader;
#[doc = "Field `doen_61` writer - The selected OEN signal for GPIO61. The register value indicates the selected GPIO (Output Enable) OEN index from GPIO OEN list 0-49. See Table 2-41: GPIO OEN List for SYS_IOMUX (on page 97) for more information."]
pub type DOEN_61_W<'a, REG> = crate::FieldWriter<'a, REG, 6>;
#[doc = "Field `doen_62` reader - The selected OEN signal for GPIO62. The register value indicates the selected GPIO (Output Enable) OEN index from GPIO OEN list 0-49. See Table 2-41: GPIO OEN List for SYS_IOMUX (on page 97) for more information."]
pub type DOEN_62_R = crate::FieldReader;
#[doc = "Field `doen_62` writer - The selected OEN signal for GPIO62. The register value indicates the selected GPIO (Output Enable) OEN index from GPIO OEN list 0-49. See Table 2-41: GPIO OEN List for SYS_IOMUX (on page 97) for more information."]
pub type DOEN_62_W<'a, REG> = crate::FieldWriter<'a, REG, 6>;
#[doc = "Field `doen_63` reader - The selected OEN signal for GPIO63. The register value indicates the selected GPIO (Output Enable) OEN index from GPIO OEN list 0-49. See Table 2-41: GPIO OEN List for SYS_IOMUX (on page 97) for more information."]
pub type DOEN_63_R = crate::FieldReader;
#[doc = "Field `doen_63` writer - The selected OEN signal for GPIO63. The register value indicates the selected GPIO (Output Enable) OEN index from GPIO OEN list 0-49. See Table 2-41: GPIO OEN List for SYS_IOMUX (on page 97) for more information."]
pub type DOEN_63_W<'a, REG> = crate::FieldWriter<'a, REG, 6>;
impl R {
    #[doc = "Bits 0:5 - The selected OEN signal for GPIO60. The register value indicates the selected GPIO (Output Enable) OEN index from GPIO OEN list 0-49. See Table 2-41: GPIO OEN List for SYS_IOMUX (on page 97) for more information."]
    #[inline(always)]
    pub fn doen_60(&self) -> DOEN_60_R {
        DOEN_60_R::new((self.bits & 0x3f) as u8)
    }
    #[doc = "Bits 8:13 - The selected OEN signal for GPIO61. The register value indicates the selected GPIO (Output Enable) OEN index from GPIO OEN list 0-49. See Table 2-41: GPIO OEN List for SYS_IOMUX (on page 97) for more information."]
    #[inline(always)]
    pub fn doen_61(&self) -> DOEN_61_R {
        DOEN_61_R::new(((self.bits >> 8) & 0x3f) as u8)
    }
    #[doc = "Bits 16:21 - The selected OEN signal for GPIO62. The register value indicates the selected GPIO (Output Enable) OEN index from GPIO OEN list 0-49. See Table 2-41: GPIO OEN List for SYS_IOMUX (on page 97) for more information."]
    #[inline(always)]
    pub fn doen_62(&self) -> DOEN_62_R {
        DOEN_62_R::new(((self.bits >> 16) & 0x3f) as u8)
    }
    #[doc = "Bits 24:29 - The selected OEN signal for GPIO63. The register value indicates the selected GPIO (Output Enable) OEN index from GPIO OEN list 0-49. See Table 2-41: GPIO OEN List for SYS_IOMUX (on page 97) for more information."]
    #[inline(always)]
    pub fn doen_63(&self) -> DOEN_63_R {
        DOEN_63_R::new(((self.bits >> 24) & 0x3f) as u8)
    }
}
impl W {
    #[doc = "Bits 0:5 - The selected OEN signal for GPIO60. The register value indicates the selected GPIO (Output Enable) OEN index from GPIO OEN list 0-49. See Table 2-41: GPIO OEN List for SYS_IOMUX (on page 97) for more information."]
    #[inline(always)]
    #[must_use]
    pub fn doen_60(&mut self) -> DOEN_60_W<GPO_DOEN_15_SPEC> {
        DOEN_60_W::new(self, 0)
    }
    #[doc = "Bits 8:13 - The selected OEN signal for GPIO61. The register value indicates the selected GPIO (Output Enable) OEN index from GPIO OEN list 0-49. See Table 2-41: GPIO OEN List for SYS_IOMUX (on page 97) for more information."]
    #[inline(always)]
    #[must_use]
    pub fn doen_61(&mut self) -> DOEN_61_W<GPO_DOEN_15_SPEC> {
        DOEN_61_W::new(self, 8)
    }
    #[doc = "Bits 16:21 - The selected OEN signal for GPIO62. The register value indicates the selected GPIO (Output Enable) OEN index from GPIO OEN list 0-49. See Table 2-41: GPIO OEN List for SYS_IOMUX (on page 97) for more information."]
    #[inline(always)]
    #[must_use]
    pub fn doen_62(&mut self) -> DOEN_62_W<GPO_DOEN_15_SPEC> {
        DOEN_62_W::new(self, 16)
    }
    #[doc = "Bits 24:29 - The selected OEN signal for GPIO63. The register value indicates the selected GPIO (Output Enable) OEN index from GPIO OEN list 0-49. See Table 2-41: GPIO OEN List for SYS_IOMUX (on page 97) for more information."]
    #[inline(always)]
    #[must_use]
    pub fn doen_63(&mut self) -> DOEN_63_W<GPO_DOEN_15_SPEC> {
        DOEN_63_W::new(self, 24)
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
#[doc = "SYS IOMUX CFG SAIF SYSCFG FMUX 15 DOEN\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`gpo_doen_15::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`gpo_doen_15::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct GPO_DOEN_15_SPEC;
impl crate::RegisterSpec for GPO_DOEN_15_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`gpo_doen_15::R`](R) reader structure"]
impl crate::Readable for GPO_DOEN_15_SPEC {}
#[doc = "`write(|w| ..)` method takes [`gpo_doen_15::W`](W) writer structure"]
impl crate::Writable for GPO_DOEN_15_SPEC {
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets gpo_doen_15 to value 0x2901_2828"]
impl crate::Resettable for GPO_DOEN_15_SPEC {
    const RESET_VALUE: Self::Ux = 0x2901_2828;
}
