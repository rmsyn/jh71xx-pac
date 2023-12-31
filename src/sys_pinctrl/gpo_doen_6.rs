#[doc = "Register `gpo_doen_6` reader"]
pub type R = crate::R<GPO_DOEN_6_SPEC>;
#[doc = "Register `gpo_doen_6` writer"]
pub type W = crate::W<GPO_DOEN_6_SPEC>;
#[doc = "Field `doen_24` reader - The selected OEN signal for GPIO24. The register value indicates the selected GPIO (Output Enable) OEN index from GPIO OEN list 0-49. See Table 2-41: GPIO OEN List for SYS_IOMUX (on page 97) for more information."]
pub type DOEN_24_R = crate::FieldReader;
#[doc = "Field `doen_24` writer - The selected OEN signal for GPIO24. The register value indicates the selected GPIO (Output Enable) OEN index from GPIO OEN list 0-49. See Table 2-41: GPIO OEN List for SYS_IOMUX (on page 97) for more information."]
pub type DOEN_24_W<'a, REG> = crate::FieldWriter<'a, REG, 6>;
#[doc = "Field `doen_25` reader - The selected OEN signal for GPIO25. The register value indicates the selected GPIO (Output Enable) OEN index from GPIO OEN list 0-49. See Table 2-41: GPIO OEN List for SYS_IOMUX (on page 97) for more information."]
pub type DOEN_25_R = crate::FieldReader;
#[doc = "Field `doen_25` writer - The selected OEN signal for GPIO25. The register value indicates the selected GPIO (Output Enable) OEN index from GPIO OEN list 0-49. See Table 2-41: GPIO OEN List for SYS_IOMUX (on page 97) for more information."]
pub type DOEN_25_W<'a, REG> = crate::FieldWriter<'a, REG, 6>;
#[doc = "Field `doen_26` reader - The selected OEN signal for GPIO26. The register value indicates the selected GPIO (Output Enable) OEN index from GPIO OEN list 0-49. See Table 2-41: GPIO OEN List for SYS_IOMUX (on page 97) for more information."]
pub type DOEN_26_R = crate::FieldReader;
#[doc = "Field `doen_26` writer - The selected OEN signal for GPIO26. The register value indicates the selected GPIO (Output Enable) OEN index from GPIO OEN list 0-49. See Table 2-41: GPIO OEN List for SYS_IOMUX (on page 97) for more information."]
pub type DOEN_26_W<'a, REG> = crate::FieldWriter<'a, REG, 6>;
#[doc = "Field `doen_27` reader - The selected OEN signal for GPIO27. The register value indicates the selected GPIO (Output Enable) OEN index from GPIO OEN list 0-49. See Table 2-41: GPIO OEN List for SYS_IOMUX (on page 97) for more information."]
pub type DOEN_27_R = crate::FieldReader;
#[doc = "Field `doen_27` writer - The selected OEN signal for GPIO27. The register value indicates the selected GPIO (Output Enable) OEN index from GPIO OEN list 0-49. See Table 2-41: GPIO OEN List for SYS_IOMUX (on page 97) for more information."]
pub type DOEN_27_W<'a, REG> = crate::FieldWriter<'a, REG, 6>;
impl R {
    #[doc = "Bits 0:5 - The selected OEN signal for GPIO24. The register value indicates the selected GPIO (Output Enable) OEN index from GPIO OEN list 0-49. See Table 2-41: GPIO OEN List for SYS_IOMUX (on page 97) for more information."]
    #[inline(always)]
    pub fn doen_24(&self) -> DOEN_24_R {
        DOEN_24_R::new((self.bits & 0x3f) as u8)
    }
    #[doc = "Bits 8:13 - The selected OEN signal for GPIO25. The register value indicates the selected GPIO (Output Enable) OEN index from GPIO OEN list 0-49. See Table 2-41: GPIO OEN List for SYS_IOMUX (on page 97) for more information."]
    #[inline(always)]
    pub fn doen_25(&self) -> DOEN_25_R {
        DOEN_25_R::new(((self.bits >> 8) & 0x3f) as u8)
    }
    #[doc = "Bits 16:21 - The selected OEN signal for GPIO26. The register value indicates the selected GPIO (Output Enable) OEN index from GPIO OEN list 0-49. See Table 2-41: GPIO OEN List for SYS_IOMUX (on page 97) for more information."]
    #[inline(always)]
    pub fn doen_26(&self) -> DOEN_26_R {
        DOEN_26_R::new(((self.bits >> 16) & 0x3f) as u8)
    }
    #[doc = "Bits 24:29 - The selected OEN signal for GPIO27. The register value indicates the selected GPIO (Output Enable) OEN index from GPIO OEN list 0-49. See Table 2-41: GPIO OEN List for SYS_IOMUX (on page 97) for more information."]
    #[inline(always)]
    pub fn doen_27(&self) -> DOEN_27_R {
        DOEN_27_R::new(((self.bits >> 24) & 0x3f) as u8)
    }
}
impl W {
    #[doc = "Bits 0:5 - The selected OEN signal for GPIO24. The register value indicates the selected GPIO (Output Enable) OEN index from GPIO OEN list 0-49. See Table 2-41: GPIO OEN List for SYS_IOMUX (on page 97) for more information."]
    #[inline(always)]
    #[must_use]
    pub fn doen_24(&mut self) -> DOEN_24_W<GPO_DOEN_6_SPEC> {
        DOEN_24_W::new(self, 0)
    }
    #[doc = "Bits 8:13 - The selected OEN signal for GPIO25. The register value indicates the selected GPIO (Output Enable) OEN index from GPIO OEN list 0-49. See Table 2-41: GPIO OEN List for SYS_IOMUX (on page 97) for more information."]
    #[inline(always)]
    #[must_use]
    pub fn doen_25(&mut self) -> DOEN_25_W<GPO_DOEN_6_SPEC> {
        DOEN_25_W::new(self, 8)
    }
    #[doc = "Bits 16:21 - The selected OEN signal for GPIO26. The register value indicates the selected GPIO (Output Enable) OEN index from GPIO OEN list 0-49. See Table 2-41: GPIO OEN List for SYS_IOMUX (on page 97) for more information."]
    #[inline(always)]
    #[must_use]
    pub fn doen_26(&mut self) -> DOEN_26_W<GPO_DOEN_6_SPEC> {
        DOEN_26_W::new(self, 16)
    }
    #[doc = "Bits 24:29 - The selected OEN signal for GPIO27. The register value indicates the selected GPIO (Output Enable) OEN index from GPIO OEN list 0-49. See Table 2-41: GPIO OEN List for SYS_IOMUX (on page 97) for more information."]
    #[inline(always)]
    #[must_use]
    pub fn doen_27(&mut self) -> DOEN_27_W<GPO_DOEN_6_SPEC> {
        DOEN_27_W::new(self, 24)
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
#[doc = "SYS IOMUX CFG SAIF SYSCFG FMUX 6 DOEN\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`gpo_doen_6::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`gpo_doen_6::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct GPO_DOEN_6_SPEC;
impl crate::RegisterSpec for GPO_DOEN_6_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`gpo_doen_6::R`](R) reader structure"]
impl crate::Readable for GPO_DOEN_6_SPEC {}
#[doc = "`write(|w| ..)` method takes [`gpo_doen_6::W`](W) writer structure"]
impl crate::Writable for GPO_DOEN_6_SPEC {
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets gpo_doen_6 to value 0"]
impl crate::Resettable for GPO_DOEN_6_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
