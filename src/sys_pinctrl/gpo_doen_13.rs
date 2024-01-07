#[doc = "Register `gpo_doen_13` reader"]
pub type R = crate::R<GPO_DOEN_13_SPEC>;
#[doc = "Register `gpo_doen_13` writer"]
pub type W = crate::W<GPO_DOEN_13_SPEC>;
#[doc = "Field `doen_52` reader - The selected OEN signal for GPIO52. The register value indicates the selected GPIO (Output Enable) OEN index from GPIO OEN list 0-49. See Table 2-41: GPIO OEN List for SYS_IOMUX (on page 97) for more information."]
pub type DOEN_52_R = crate::FieldReader;
#[doc = "Field `doen_52` writer - The selected OEN signal for GPIO52. The register value indicates the selected GPIO (Output Enable) OEN index from GPIO OEN list 0-49. See Table 2-41: GPIO OEN List for SYS_IOMUX (on page 97) for more information."]
pub type DOEN_52_W<'a, REG> = crate::FieldWriter<'a, REG, 6>;
#[doc = "Field `doen_53` reader - The selected OEN signal for GPIO53. The register value indicates the selected GPIO (Output Enable) OEN index from GPIO OEN list 0-49. See Table 2-41: GPIO OEN List for SYS_IOMUX (on page 97) for more information."]
pub type DOEN_53_R = crate::FieldReader;
#[doc = "Field `doen_53` writer - The selected OEN signal for GPIO53. The register value indicates the selected GPIO (Output Enable) OEN index from GPIO OEN list 0-49. See Table 2-41: GPIO OEN List for SYS_IOMUX (on page 97) for more information."]
pub type DOEN_53_W<'a, REG> = crate::FieldWriter<'a, REG, 6>;
#[doc = "Field `doen_54` reader - The selected OEN signal for GPIO54. The register value indicates the selected GPIO (Output Enable) OEN index from GPIO OEN list 0-49. See Table 2-41: GPIO OEN List for SYS_IOMUX (on page 97) for more information."]
pub type DOEN_54_R = crate::FieldReader;
#[doc = "Field `doen_54` writer - The selected OEN signal for GPIO54. The register value indicates the selected GPIO (Output Enable) OEN index from GPIO OEN list 0-49. See Table 2-41: GPIO OEN List for SYS_IOMUX (on page 97) for more information."]
pub type DOEN_54_W<'a, REG> = crate::FieldWriter<'a, REG, 6>;
#[doc = "Field `doen_55` reader - The selected OEN signal for GPIO55. The register value indicates the selected GPIO (Output Enable) OEN index from GPIO OEN list 0-49. See Table 2-41: GPIO OEN List for SYS_IOMUX (on page 97) for more information."]
pub type DOEN_55_R = crate::FieldReader;
#[doc = "Field `doen_55` writer - The selected OEN signal for GPIO55. The register value indicates the selected GPIO (Output Enable) OEN index from GPIO OEN list 0-49. See Table 2-41: GPIO OEN List for SYS_IOMUX (on page 97) for more information."]
pub type DOEN_55_W<'a, REG> = crate::FieldWriter<'a, REG, 6>;
impl R {
    #[doc = "Bits 0:5 - The selected OEN signal for GPIO52. The register value indicates the selected GPIO (Output Enable) OEN index from GPIO OEN list 0-49. See Table 2-41: GPIO OEN List for SYS_IOMUX (on page 97) for more information."]
    #[inline(always)]
    pub fn doen_52(&self) -> DOEN_52_R {
        DOEN_52_R::new((self.bits & 0x3f) as u8)
    }
    #[doc = "Bits 8:13 - The selected OEN signal for GPIO53. The register value indicates the selected GPIO (Output Enable) OEN index from GPIO OEN list 0-49. See Table 2-41: GPIO OEN List for SYS_IOMUX (on page 97) for more information."]
    #[inline(always)]
    pub fn doen_53(&self) -> DOEN_53_R {
        DOEN_53_R::new(((self.bits >> 8) & 0x3f) as u8)
    }
    #[doc = "Bits 16:21 - The selected OEN signal for GPIO54. The register value indicates the selected GPIO (Output Enable) OEN index from GPIO OEN list 0-49. See Table 2-41: GPIO OEN List for SYS_IOMUX (on page 97) for more information."]
    #[inline(always)]
    pub fn doen_54(&self) -> DOEN_54_R {
        DOEN_54_R::new(((self.bits >> 16) & 0x3f) as u8)
    }
    #[doc = "Bits 24:29 - The selected OEN signal for GPIO55. The register value indicates the selected GPIO (Output Enable) OEN index from GPIO OEN list 0-49. See Table 2-41: GPIO OEN List for SYS_IOMUX (on page 97) for more information."]
    #[inline(always)]
    pub fn doen_55(&self) -> DOEN_55_R {
        DOEN_55_R::new(((self.bits >> 24) & 0x3f) as u8)
    }
}
impl W {
    #[doc = "Bits 0:5 - The selected OEN signal for GPIO52. The register value indicates the selected GPIO (Output Enable) OEN index from GPIO OEN list 0-49. See Table 2-41: GPIO OEN List for SYS_IOMUX (on page 97) for more information."]
    #[inline(always)]
    #[must_use]
    pub fn doen_52(&mut self) -> DOEN_52_W<GPO_DOEN_13_SPEC> {
        DOEN_52_W::new(self, 0)
    }
    #[doc = "Bits 8:13 - The selected OEN signal for GPIO53. The register value indicates the selected GPIO (Output Enable) OEN index from GPIO OEN list 0-49. See Table 2-41: GPIO OEN List for SYS_IOMUX (on page 97) for more information."]
    #[inline(always)]
    #[must_use]
    pub fn doen_53(&mut self) -> DOEN_53_W<GPO_DOEN_13_SPEC> {
        DOEN_53_W::new(self, 8)
    }
    #[doc = "Bits 16:21 - The selected OEN signal for GPIO54. The register value indicates the selected GPIO (Output Enable) OEN index from GPIO OEN list 0-49. See Table 2-41: GPIO OEN List for SYS_IOMUX (on page 97) for more information."]
    #[inline(always)]
    #[must_use]
    pub fn doen_54(&mut self) -> DOEN_54_W<GPO_DOEN_13_SPEC> {
        DOEN_54_W::new(self, 16)
    }
    #[doc = "Bits 24:29 - The selected OEN signal for GPIO55. The register value indicates the selected GPIO (Output Enable) OEN index from GPIO OEN list 0-49. See Table 2-41: GPIO OEN List for SYS_IOMUX (on page 97) for more information."]
    #[inline(always)]
    #[must_use]
    pub fn doen_55(&mut self) -> DOEN_55_W<GPO_DOEN_13_SPEC> {
        DOEN_55_W::new(self, 24)
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
#[doc = "SYS IOMUX CFG SAIF SYSCFG FMUX 13 DOEN\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`gpo_doen_13::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`gpo_doen_13::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct GPO_DOEN_13_SPEC;
impl crate::RegisterSpec for GPO_DOEN_13_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`gpo_doen_13::R`](R) reader structure"]
impl crate::Readable for GPO_DOEN_13_SPEC {}
#[doc = "`write(|w| ..)` method takes [`gpo_doen_13::W`](W) writer structure"]
impl crate::Writable for GPO_DOEN_13_SPEC {
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets gpo_doen_13 to value 0x1d01_1c1c"]
impl crate::Resettable for GPO_DOEN_13_SPEC {
    const RESET_VALUE: Self::Ux = 0x1d01_1c1c;
}
