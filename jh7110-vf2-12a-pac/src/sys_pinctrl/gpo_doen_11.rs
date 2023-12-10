#[doc = "Register `gpo_doen_11` reader"]
pub type R = crate::R<GPO_DOEN_11_SPEC>;
#[doc = "Register `gpo_doen_11` writer"]
pub type W = crate::W<GPO_DOEN_11_SPEC>;
#[doc = "Field `doen_44` reader - The selected OEN signal for GPIO44. The register value indicates the selected GPIO (Output Enable) OEN index from GPIO OEN list 0-49. See Table 2-41: GPIO OEN List for SYS_IOMUX (on page 97) for more information."]
pub type DOEN_44_R = crate::FieldReader;
#[doc = "Field `doen_44` writer - The selected OEN signal for GPIO44. The register value indicates the selected GPIO (Output Enable) OEN index from GPIO OEN list 0-49. See Table 2-41: GPIO OEN List for SYS_IOMUX (on page 97) for more information."]
pub type DOEN_44_W<'a, REG> = crate::FieldWriter<'a, REG, 6>;
#[doc = "Field `doen_45` reader - The selected OEN signal for GPIO45. The register value indicates the selected GPIO (Output Enable) OEN index from GPIO OEN list 0-49. See Table 2-41: GPIO OEN List for SYS_IOMUX (on page 97) for more information."]
pub type DOEN_45_R = crate::FieldReader;
#[doc = "Field `doen_45` writer - The selected OEN signal for GPIO45. The register value indicates the selected GPIO (Output Enable) OEN index from GPIO OEN list 0-49. See Table 2-41: GPIO OEN List for SYS_IOMUX (on page 97) for more information."]
pub type DOEN_45_W<'a, REG> = crate::FieldWriter<'a, REG, 6>;
#[doc = "Field `doen_46` reader - The selected OEN signal for GPIO46. The register value indicates the selected GPIO (Output Enable) OEN index from GPIO OEN list 0-49. See Table 2-41: GPIO OEN List for SYS_IOMUX (on page 97) for more information."]
pub type DOEN_46_R = crate::FieldReader;
#[doc = "Field `doen_46` writer - The selected OEN signal for GPIO46. The register value indicates the selected GPIO (Output Enable) OEN index from GPIO OEN list 0-49. See Table 2-41: GPIO OEN List for SYS_IOMUX (on page 97) for more information."]
pub type DOEN_46_W<'a, REG> = crate::FieldWriter<'a, REG, 6>;
#[doc = "Field `doen_47` reader - The selected OEN signal for GPIO47. The register value indicates the selected GPIO (Output Enable) OEN index from GPIO OEN list 0-49. See Table 2-41: GPIO OEN List for SYS_IOMUX (on page 97) for more information."]
pub type DOEN_47_R = crate::FieldReader;
#[doc = "Field `doen_47` writer - The selected OEN signal for GPIO47. The register value indicates the selected GPIO (Output Enable) OEN index from GPIO OEN list 0-49. See Table 2-41: GPIO OEN List for SYS_IOMUX (on page 97) for more information."]
pub type DOEN_47_W<'a, REG> = crate::FieldWriter<'a, REG, 6>;
impl R {
    #[doc = "Bits 0:5 - The selected OEN signal for GPIO44. The register value indicates the selected GPIO (Output Enable) OEN index from GPIO OEN list 0-49. See Table 2-41: GPIO OEN List for SYS_IOMUX (on page 97) for more information."]
    #[inline(always)]
    pub fn doen_44(&self) -> DOEN_44_R {
        DOEN_44_R::new((self.bits & 0x3f) as u8)
    }
    #[doc = "Bits 8:13 - The selected OEN signal for GPIO45. The register value indicates the selected GPIO (Output Enable) OEN index from GPIO OEN list 0-49. See Table 2-41: GPIO OEN List for SYS_IOMUX (on page 97) for more information."]
    #[inline(always)]
    pub fn doen_45(&self) -> DOEN_45_R {
        DOEN_45_R::new(((self.bits >> 8) & 0x3f) as u8)
    }
    #[doc = "Bits 16:21 - The selected OEN signal for GPIO46. The register value indicates the selected GPIO (Output Enable) OEN index from GPIO OEN list 0-49. See Table 2-41: GPIO OEN List for SYS_IOMUX (on page 97) for more information."]
    #[inline(always)]
    pub fn doen_46(&self) -> DOEN_46_R {
        DOEN_46_R::new(((self.bits >> 16) & 0x3f) as u8)
    }
    #[doc = "Bits 24:29 - The selected OEN signal for GPIO47. The register value indicates the selected GPIO (Output Enable) OEN index from GPIO OEN list 0-49. See Table 2-41: GPIO OEN List for SYS_IOMUX (on page 97) for more information."]
    #[inline(always)]
    pub fn doen_47(&self) -> DOEN_47_R {
        DOEN_47_R::new(((self.bits >> 24) & 0x3f) as u8)
    }
}
impl W {
    #[doc = "Bits 0:5 - The selected OEN signal for GPIO44. The register value indicates the selected GPIO (Output Enable) OEN index from GPIO OEN list 0-49. See Table 2-41: GPIO OEN List for SYS_IOMUX (on page 97) for more information."]
    #[inline(always)]
    #[must_use]
    pub fn doen_44(&mut self) -> DOEN_44_W<GPO_DOEN_11_SPEC> {
        DOEN_44_W::new(self, 0)
    }
    #[doc = "Bits 8:13 - The selected OEN signal for GPIO45. The register value indicates the selected GPIO (Output Enable) OEN index from GPIO OEN list 0-49. See Table 2-41: GPIO OEN List for SYS_IOMUX (on page 97) for more information."]
    #[inline(always)]
    #[must_use]
    pub fn doen_45(&mut self) -> DOEN_45_W<GPO_DOEN_11_SPEC> {
        DOEN_45_W::new(self, 8)
    }
    #[doc = "Bits 16:21 - The selected OEN signal for GPIO46. The register value indicates the selected GPIO (Output Enable) OEN index from GPIO OEN list 0-49. See Table 2-41: GPIO OEN List for SYS_IOMUX (on page 97) for more information."]
    #[inline(always)]
    #[must_use]
    pub fn doen_46(&mut self) -> DOEN_46_W<GPO_DOEN_11_SPEC> {
        DOEN_46_W::new(self, 16)
    }
    #[doc = "Bits 24:29 - The selected OEN signal for GPIO47. The register value indicates the selected GPIO (Output Enable) OEN index from GPIO OEN list 0-49. See Table 2-41: GPIO OEN List for SYS_IOMUX (on page 97) for more information."]
    #[inline(always)]
    #[must_use]
    pub fn doen_47(&mut self) -> DOEN_47_W<GPO_DOEN_11_SPEC> {
        DOEN_47_W::new(self, 24)
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
#[doc = "SYS IOMUX CFG SAIF SYSCFG FMUX 11 DOEN\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`gpo_doen_11::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`gpo_doen_11::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct GPO_DOEN_11_SPEC;
impl crate::RegisterSpec for GPO_DOEN_11_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`gpo_doen_11::R`](R) reader structure"]
impl crate::Readable for GPO_DOEN_11_SPEC {}
#[doc = "`write(|w| ..)` method takes [`gpo_doen_11::W`](W) writer structure"]
impl crate::Writable for GPO_DOEN_11_SPEC {
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets gpo_doen_11 to value 0x0100_0001"]
impl crate::Resettable for GPO_DOEN_11_SPEC {
    const RESET_VALUE: Self::Ux = 0x0100_0001;
}
