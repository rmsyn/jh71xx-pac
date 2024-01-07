#[doc = "Register `gpo_doen_2` reader"]
pub type R = crate::R<GPO_DOEN_2_SPEC>;
#[doc = "Register `gpo_doen_2` writer"]
pub type W = crate::W<GPO_DOEN_2_SPEC>;
#[doc = "Field `doen_8` reader - The selected OEN signal for GPIO8. The register value indicates the selected GPIO (Output Enable) OEN index from GPIO OEN list 0-49. See Table 2-41: GPIO OEN List for SYS_IOMUX (on page 97) for more information."]
pub type DOEN_8_R = crate::FieldReader;
#[doc = "Field `doen_8` writer - The selected OEN signal for GPIO8. The register value indicates the selected GPIO (Output Enable) OEN index from GPIO OEN list 0-49. See Table 2-41: GPIO OEN List for SYS_IOMUX (on page 97) for more information."]
pub type DOEN_8_W<'a, REG> = crate::FieldWriter<'a, REG, 6>;
#[doc = "Field `doen_9` reader - The selected OEN signal for GPIO9. The register value indicates the selected GPIO (Output Enable) OEN index from GPIO OEN list 0-49. See Table 2-41: GPIO OEN List for SYS_IOMUX (on page 97) for more information."]
pub type DOEN_9_R = crate::FieldReader;
#[doc = "Field `doen_9` writer - The selected OEN signal for GPIO9. The register value indicates the selected GPIO (Output Enable) OEN index from GPIO OEN list 0-49. See Table 2-41: GPIO OEN List for SYS_IOMUX (on page 97) for more information."]
pub type DOEN_9_W<'a, REG> = crate::FieldWriter<'a, REG, 6>;
#[doc = "Field `doen_10` reader - The selected OEN signal for GPIO10. The register value indicates the selected GPIO (Output Enable) OEN index from GPIO OEN list 0-49. See Table 2-41: GPIO OEN List for SYS_IOMUX (on page 97) for more information."]
pub type DOEN_10_R = crate::FieldReader;
#[doc = "Field `doen_10` writer - The selected OEN signal for GPIO10. The register value indicates the selected GPIO (Output Enable) OEN index from GPIO OEN list 0-49. See Table 2-41: GPIO OEN List for SYS_IOMUX (on page 97) for more information."]
pub type DOEN_10_W<'a, REG> = crate::FieldWriter<'a, REG, 6>;
#[doc = "Field `doen_11` reader - The selected OEN signal for GPIO11. The register value indicates the selected GPIO (Output Enable) OEN index from GPIO OEN list 0-49. See Table 2-41: GPIO OEN List for SYS_IOMUX (on page 97) for more information."]
pub type DOEN_11_R = crate::FieldReader;
#[doc = "Field `doen_11` writer - The selected OEN signal for GPIO11. The register value indicates the selected GPIO (Output Enable) OEN index from GPIO OEN list 0-49. See Table 2-41: GPIO OEN List for SYS_IOMUX (on page 97) for more information."]
pub type DOEN_11_W<'a, REG> = crate::FieldWriter<'a, REG, 6>;
impl R {
    #[doc = "Bits 0:5 - The selected OEN signal for GPIO8. The register value indicates the selected GPIO (Output Enable) OEN index from GPIO OEN list 0-49. See Table 2-41: GPIO OEN List for SYS_IOMUX (on page 97) for more information."]
    #[inline(always)]
    pub fn doen_8(&self) -> DOEN_8_R {
        DOEN_8_R::new((self.bits & 0x3f) as u8)
    }
    #[doc = "Bits 8:13 - The selected OEN signal for GPIO9. The register value indicates the selected GPIO (Output Enable) OEN index from GPIO OEN list 0-49. See Table 2-41: GPIO OEN List for SYS_IOMUX (on page 97) for more information."]
    #[inline(always)]
    pub fn doen_9(&self) -> DOEN_9_R {
        DOEN_9_R::new(((self.bits >> 8) & 0x3f) as u8)
    }
    #[doc = "Bits 16:21 - The selected OEN signal for GPIO10. The register value indicates the selected GPIO (Output Enable) OEN index from GPIO OEN list 0-49. See Table 2-41: GPIO OEN List for SYS_IOMUX (on page 97) for more information."]
    #[inline(always)]
    pub fn doen_10(&self) -> DOEN_10_R {
        DOEN_10_R::new(((self.bits >> 16) & 0x3f) as u8)
    }
    #[doc = "Bits 24:29 - The selected OEN signal for GPIO11. The register value indicates the selected GPIO (Output Enable) OEN index from GPIO OEN list 0-49. See Table 2-41: GPIO OEN List for SYS_IOMUX (on page 97) for more information."]
    #[inline(always)]
    pub fn doen_11(&self) -> DOEN_11_R {
        DOEN_11_R::new(((self.bits >> 24) & 0x3f) as u8)
    }
}
impl W {
    #[doc = "Bits 0:5 - The selected OEN signal for GPIO8. The register value indicates the selected GPIO (Output Enable) OEN index from GPIO OEN list 0-49. See Table 2-41: GPIO OEN List for SYS_IOMUX (on page 97) for more information."]
    #[inline(always)]
    #[must_use]
    pub fn doen_8(&mut self) -> DOEN_8_W<GPO_DOEN_2_SPEC> {
        DOEN_8_W::new(self, 0)
    }
    #[doc = "Bits 8:13 - The selected OEN signal for GPIO9. The register value indicates the selected GPIO (Output Enable) OEN index from GPIO OEN list 0-49. See Table 2-41: GPIO OEN List for SYS_IOMUX (on page 97) for more information."]
    #[inline(always)]
    #[must_use]
    pub fn doen_9(&mut self) -> DOEN_9_W<GPO_DOEN_2_SPEC> {
        DOEN_9_W::new(self, 8)
    }
    #[doc = "Bits 16:21 - The selected OEN signal for GPIO10. The register value indicates the selected GPIO (Output Enable) OEN index from GPIO OEN list 0-49. See Table 2-41: GPIO OEN List for SYS_IOMUX (on page 97) for more information."]
    #[inline(always)]
    #[must_use]
    pub fn doen_10(&mut self) -> DOEN_10_W<GPO_DOEN_2_SPEC> {
        DOEN_10_W::new(self, 16)
    }
    #[doc = "Bits 24:29 - The selected OEN signal for GPIO11. The register value indicates the selected GPIO (Output Enable) OEN index from GPIO OEN list 0-49. See Table 2-41: GPIO OEN List for SYS_IOMUX (on page 97) for more information."]
    #[inline(always)]
    #[must_use]
    pub fn doen_11(&mut self) -> DOEN_11_W<GPO_DOEN_2_SPEC> {
        DOEN_11_W::new(self, 24)
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
#[doc = "SYS IOMUX CFG SAIF SYSCFG FMUX 2 DOEN\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`gpo_doen_2::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`gpo_doen_2::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct GPO_DOEN_2_SPEC;
impl crate::RegisterSpec for GPO_DOEN_2_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`gpo_doen_2::R`](R) reader structure"]
impl crate::Readable for GPO_DOEN_2_SPEC {}
#[doc = "`write(|w| ..)` method takes [`gpo_doen_2::W`](W) writer structure"]
impl crate::Writable for GPO_DOEN_2_SPEC {
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets gpo_doen_2 to value 0x0701_0100"]
impl crate::Resettable for GPO_DOEN_2_SPEC {
    const RESET_VALUE: Self::Ux = 0x0701_0100;
}
