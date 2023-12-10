#[doc = "Register `gpo_doen_1` reader"]
pub type R = crate::R<GPO_DOEN_1_SPEC>;
#[doc = "Register `gpo_doen_1` writer"]
pub type W = crate::W<GPO_DOEN_1_SPEC>;
#[doc = "Field `doen_4` reader - The selected OEN signal for GPIO4. The register value indicates the selected GPIO (Output Enable) OEN index from GPIO OEN list 0-49. See Table 2-41: GPIO OEN List for SYS_IOMUX (on page 97) for more information."]
pub type DOEN_4_R = crate::FieldReader;
#[doc = "Field `doen_4` writer - The selected OEN signal for GPIO4. The register value indicates the selected GPIO (Output Enable) OEN index from GPIO OEN list 0-49. See Table 2-41: GPIO OEN List for SYS_IOMUX (on page 97) for more information."]
pub type DOEN_4_W<'a, REG> = crate::FieldWriter<'a, REG, 6>;
#[doc = "Field `doen_5` reader - The selected OEN signal for GPIO5. The register value indicates the selected GPIO (Output Enable) OEN index from GPIO OEN list 0-49. See Table 2-41: GPIO OEN List for SYS_IOMUX (on page 97) for more information."]
pub type DOEN_5_R = crate::FieldReader;
#[doc = "Field `doen_5` writer - The selected OEN signal for GPIO5. The register value indicates the selected GPIO (Output Enable) OEN index from GPIO OEN list 0-49. See Table 2-41: GPIO OEN List for SYS_IOMUX (on page 97) for more information."]
pub type DOEN_5_W<'a, REG> = crate::FieldWriter<'a, REG, 6>;
#[doc = "Field `doen_6` reader - The selected OEN signal for GPIO6. The register value indicates the selected GPIO (Output Enable) OEN index from GPIO OEN list 0-49. See Table 2-41: GPIO OEN List for SYS_IOMUX (on page 97) for more information."]
pub type DOEN_6_R = crate::FieldReader;
#[doc = "Field `doen_6` writer - The selected OEN signal for GPIO6. The register value indicates the selected GPIO (Output Enable) OEN index from GPIO OEN list 0-49. See Table 2-41: GPIO OEN List for SYS_IOMUX (on page 97) for more information."]
pub type DOEN_6_W<'a, REG> = crate::FieldWriter<'a, REG, 6>;
#[doc = "Field `doen_7` reader - The selected OEN signal for GPIO7. The register value indicates the selected GPIO (Output Enable) OEN index from GPIO OEN list 0-49. See Table 2-41: GPIO OEN List for SYS_IOMUX (on page 97) for more information."]
pub type DOEN_7_R = crate::FieldReader;
#[doc = "Field `doen_7` writer - The selected OEN signal for GPIO7. The register value indicates the selected GPIO (Output Enable) OEN index from GPIO OEN list 0-49. See Table 2-41: GPIO OEN List for SYS_IOMUX (on page 97) for more information."]
pub type DOEN_7_W<'a, REG> = crate::FieldWriter<'a, REG, 6>;
impl R {
    #[doc = "Bits 0:5 - The selected OEN signal for GPIO4. The register value indicates the selected GPIO (Output Enable) OEN index from GPIO OEN list 0-49. See Table 2-41: GPIO OEN List for SYS_IOMUX (on page 97) for more information."]
    #[inline(always)]
    pub fn doen_4(&self) -> DOEN_4_R {
        DOEN_4_R::new((self.bits & 0x3f) as u8)
    }
    #[doc = "Bits 8:13 - The selected OEN signal for GPIO5. The register value indicates the selected GPIO (Output Enable) OEN index from GPIO OEN list 0-49. See Table 2-41: GPIO OEN List for SYS_IOMUX (on page 97) for more information."]
    #[inline(always)]
    pub fn doen_5(&self) -> DOEN_5_R {
        DOEN_5_R::new(((self.bits >> 8) & 0x3f) as u8)
    }
    #[doc = "Bits 16:21 - The selected OEN signal for GPIO6. The register value indicates the selected GPIO (Output Enable) OEN index from GPIO OEN list 0-49. See Table 2-41: GPIO OEN List for SYS_IOMUX (on page 97) for more information."]
    #[inline(always)]
    pub fn doen_6(&self) -> DOEN_6_R {
        DOEN_6_R::new(((self.bits >> 16) & 0x3f) as u8)
    }
    #[doc = "Bits 24:29 - The selected OEN signal for GPIO7. The register value indicates the selected GPIO (Output Enable) OEN index from GPIO OEN list 0-49. See Table 2-41: GPIO OEN List for SYS_IOMUX (on page 97) for more information."]
    #[inline(always)]
    pub fn doen_7(&self) -> DOEN_7_R {
        DOEN_7_R::new(((self.bits >> 24) & 0x3f) as u8)
    }
}
impl W {
    #[doc = "Bits 0:5 - The selected OEN signal for GPIO4. The register value indicates the selected GPIO (Output Enable) OEN index from GPIO OEN list 0-49. See Table 2-41: GPIO OEN List for SYS_IOMUX (on page 97) for more information."]
    #[inline(always)]
    #[must_use]
    pub fn doen_4(&mut self) -> DOEN_4_W<GPO_DOEN_1_SPEC> {
        DOEN_4_W::new(self, 0)
    }
    #[doc = "Bits 8:13 - The selected OEN signal for GPIO5. The register value indicates the selected GPIO (Output Enable) OEN index from GPIO OEN list 0-49. See Table 2-41: GPIO OEN List for SYS_IOMUX (on page 97) for more information."]
    #[inline(always)]
    #[must_use]
    pub fn doen_5(&mut self) -> DOEN_5_W<GPO_DOEN_1_SPEC> {
        DOEN_5_W::new(self, 8)
    }
    #[doc = "Bits 16:21 - The selected OEN signal for GPIO6. The register value indicates the selected GPIO (Output Enable) OEN index from GPIO OEN list 0-49. See Table 2-41: GPIO OEN List for SYS_IOMUX (on page 97) for more information."]
    #[inline(always)]
    #[must_use]
    pub fn doen_6(&mut self) -> DOEN_6_W<GPO_DOEN_1_SPEC> {
        DOEN_6_W::new(self, 16)
    }
    #[doc = "Bits 24:29 - The selected OEN signal for GPIO7. The register value indicates the selected GPIO (Output Enable) OEN index from GPIO OEN list 0-49. See Table 2-41: GPIO OEN List for SYS_IOMUX (on page 97) for more information."]
    #[inline(always)]
    #[must_use]
    pub fn doen_7(&mut self) -> DOEN_7_W<GPO_DOEN_1_SPEC> {
        DOEN_7_W::new(self, 24)
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
#[doc = "SYS IOMUX CFG SAIF SYSCFG FMUX 1 DOEN\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`gpo_doen_1::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`gpo_doen_1::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct GPO_DOEN_1_SPEC;
impl crate::RegisterSpec for GPO_DOEN_1_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`gpo_doen_1::R`](R) reader structure"]
impl crate::Readable for GPO_DOEN_1_SPEC {}
#[doc = "`write(|w| ..)` method takes [`gpo_doen_1::W`](W) writer structure"]
impl crate::Writable for GPO_DOEN_1_SPEC {
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets gpo_doen_1 to value 0x0001_0001"]
impl crate::Resettable for GPO_DOEN_1_SPEC {
    const RESET_VALUE: Self::Ux = 0x0001_0001;
}
