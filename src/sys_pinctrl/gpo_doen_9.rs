#[doc = "Register `gpo_doen_9` reader"]
pub type R = crate::R<GPO_DOEN_9_SPEC>;
#[doc = "Register `gpo_doen_9` writer"]
pub type W = crate::W<GPO_DOEN_9_SPEC>;
#[doc = "Field `doen_36` reader - The selected OEN signal for GPIO36. The register value indicates the selected GPIO (Output Enable) OEN index from GPIO OEN list 0-49. See Table 2-41: GPIO OEN List for SYS_IOMUX (on page 97) for more information."]
pub type DOEN_36_R = crate::FieldReader;
#[doc = "Field `doen_36` writer - The selected OEN signal for GPIO36. The register value indicates the selected GPIO (Output Enable) OEN index from GPIO OEN list 0-49. See Table 2-41: GPIO OEN List for SYS_IOMUX (on page 97) for more information."]
pub type DOEN_36_W<'a, REG> = crate::FieldWriter<'a, REG, 6>;
#[doc = "Field `doen_37` reader - The selected OEN signal for GPIO37. The register value indicates the selected GPIO (Output Enable) OEN index from GPIO OEN list 0-49. See Table 2-41: GPIO OEN List for SYS_IOMUX (on page 97) for more information."]
pub type DOEN_37_R = crate::FieldReader;
#[doc = "Field `doen_37` writer - The selected OEN signal for GPIO37. The register value indicates the selected GPIO (Output Enable) OEN index from GPIO OEN list 0-49. See Table 2-41: GPIO OEN List for SYS_IOMUX (on page 97) for more information."]
pub type DOEN_37_W<'a, REG> = crate::FieldWriter<'a, REG, 6>;
#[doc = "Field `doen_38` reader - The selected OEN signal for GPIO38. The register value indicates the selected GPIO (Output Enable) OEN index from GPIO OEN list 0-49. See Table 2-41: GPIO OEN List for SYS_IOMUX (on page 97) for more information."]
pub type DOEN_38_R = crate::FieldReader;
#[doc = "Field `doen_38` writer - The selected OEN signal for GPIO38. The register value indicates the selected GPIO (Output Enable) OEN index from GPIO OEN list 0-49. See Table 2-41: GPIO OEN List for SYS_IOMUX (on page 97) for more information."]
pub type DOEN_38_W<'a, REG> = crate::FieldWriter<'a, REG, 6>;
#[doc = "Field `doen_39` reader - The selected OEN signal for GPIO39. The register value indicates the selected GPIO (Output Enable) OEN index from GPIO OEN list 0-49. See Table 2-41: GPIO OEN List for SYS_IOMUX (on page 97) for more information."]
pub type DOEN_39_R = crate::FieldReader;
#[doc = "Field `doen_39` writer - The selected OEN signal for GPIO39. The register value indicates the selected GPIO (Output Enable) OEN index from GPIO OEN list 0-49. See Table 2-41: GPIO OEN List for SYS_IOMUX (on page 97) for more information."]
pub type DOEN_39_W<'a, REG> = crate::FieldWriter<'a, REG, 6>;
impl R {
    #[doc = "Bits 0:5 - The selected OEN signal for GPIO36. The register value indicates the selected GPIO (Output Enable) OEN index from GPIO OEN list 0-49. See Table 2-41: GPIO OEN List for SYS_IOMUX (on page 97) for more information."]
    #[inline(always)]
    pub fn doen_36(&self) -> DOEN_36_R {
        DOEN_36_R::new((self.bits & 0x3f) as u8)
    }
    #[doc = "Bits 8:13 - The selected OEN signal for GPIO37. The register value indicates the selected GPIO (Output Enable) OEN index from GPIO OEN list 0-49. See Table 2-41: GPIO OEN List for SYS_IOMUX (on page 97) for more information."]
    #[inline(always)]
    pub fn doen_37(&self) -> DOEN_37_R {
        DOEN_37_R::new(((self.bits >> 8) & 0x3f) as u8)
    }
    #[doc = "Bits 16:21 - The selected OEN signal for GPIO38. The register value indicates the selected GPIO (Output Enable) OEN index from GPIO OEN list 0-49. See Table 2-41: GPIO OEN List for SYS_IOMUX (on page 97) for more information."]
    #[inline(always)]
    pub fn doen_38(&self) -> DOEN_38_R {
        DOEN_38_R::new(((self.bits >> 16) & 0x3f) as u8)
    }
    #[doc = "Bits 24:29 - The selected OEN signal for GPIO39. The register value indicates the selected GPIO (Output Enable) OEN index from GPIO OEN list 0-49. See Table 2-41: GPIO OEN List for SYS_IOMUX (on page 97) for more information."]
    #[inline(always)]
    pub fn doen_39(&self) -> DOEN_39_R {
        DOEN_39_R::new(((self.bits >> 24) & 0x3f) as u8)
    }
}
impl W {
    #[doc = "Bits 0:5 - The selected OEN signal for GPIO36. The register value indicates the selected GPIO (Output Enable) OEN index from GPIO OEN list 0-49. See Table 2-41: GPIO OEN List for SYS_IOMUX (on page 97) for more information."]
    #[inline(always)]
    #[must_use]
    pub fn doen_36(&mut self) -> DOEN_36_W<GPO_DOEN_9_SPEC> {
        DOEN_36_W::new(self, 0)
    }
    #[doc = "Bits 8:13 - The selected OEN signal for GPIO37. The register value indicates the selected GPIO (Output Enable) OEN index from GPIO OEN list 0-49. See Table 2-41: GPIO OEN List for SYS_IOMUX (on page 97) for more information."]
    #[inline(always)]
    #[must_use]
    pub fn doen_37(&mut self) -> DOEN_37_W<GPO_DOEN_9_SPEC> {
        DOEN_37_W::new(self, 8)
    }
    #[doc = "Bits 16:21 - The selected OEN signal for GPIO38. The register value indicates the selected GPIO (Output Enable) OEN index from GPIO OEN list 0-49. See Table 2-41: GPIO OEN List for SYS_IOMUX (on page 97) for more information."]
    #[inline(always)]
    #[must_use]
    pub fn doen_38(&mut self) -> DOEN_38_W<GPO_DOEN_9_SPEC> {
        DOEN_38_W::new(self, 16)
    }
    #[doc = "Bits 24:29 - The selected OEN signal for GPIO39. The register value indicates the selected GPIO (Output Enable) OEN index from GPIO OEN list 0-49. See Table 2-41: GPIO OEN List for SYS_IOMUX (on page 97) for more information."]
    #[inline(always)]
    #[must_use]
    pub fn doen_39(&mut self) -> DOEN_39_W<GPO_DOEN_9_SPEC> {
        DOEN_39_W::new(self, 24)
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
#[doc = "SYS IOMUX CFG SAIF SYSCFG FMUX 9 DOEN\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`gpo_doen_9::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`gpo_doen_9::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct GPO_DOEN_9_SPEC;
impl crate::RegisterSpec for GPO_DOEN_9_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`gpo_doen_9::R`](R) reader structure"]
impl crate::Readable for GPO_DOEN_9_SPEC {}
#[doc = "`write(|w| ..)` method takes [`gpo_doen_9::W`](W) writer structure"]
impl crate::Writable for GPO_DOEN_9_SPEC {
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets gpo_doen_9 to value 0x2322_0605"]
impl crate::Resettable for GPO_DOEN_9_SPEC {
    const RESET_VALUE: Self::Ux = 0x2322_0605;
}
