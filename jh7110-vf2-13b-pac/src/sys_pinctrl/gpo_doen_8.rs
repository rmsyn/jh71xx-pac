#[doc = "Register `gpo_doen_8` reader"]
pub type R = crate::R<GPO_DOEN_8_SPEC>;
#[doc = "Register `gpo_doen_8` writer"]
pub type W = crate::W<GPO_DOEN_8_SPEC>;
#[doc = "Field `doen_32` reader - The selected OEN signal for GPIO32. The register value indicates the selected GPIO (Output Enable) OEN index from GPIO OEN list 0-49. See Table 2-41: GPIO OEN List for SYS_IOMUX (on page 97) for more information."]
pub type DOEN_32_R = crate::FieldReader;
#[doc = "Field `doen_32` writer - The selected OEN signal for GPIO32. The register value indicates the selected GPIO (Output Enable) OEN index from GPIO OEN list 0-49. See Table 2-41: GPIO OEN List for SYS_IOMUX (on page 97) for more information."]
pub type DOEN_32_W<'a, REG> = crate::FieldWriter<'a, REG, 6>;
#[doc = "Field `doen_33` reader - The selected OEN signal for GPIO33. The register value indicates the selected GPIO (Output Enable) OEN index from GPIO OEN list 0-49. See Table 2-41: GPIO OEN List for SYS_IOMUX (on page 97) for more information."]
pub type DOEN_33_R = crate::FieldReader;
#[doc = "Field `doen_33` writer - The selected OEN signal for GPIO33. The register value indicates the selected GPIO (Output Enable) OEN index from GPIO OEN list 0-49. See Table 2-41: GPIO OEN List for SYS_IOMUX (on page 97) for more information."]
pub type DOEN_33_W<'a, REG> = crate::FieldWriter<'a, REG, 6>;
#[doc = "Field `doen_34` reader - The selected OEN signal for GPIO34. The register value indicates the selected GPIO (Output Enable) OEN index from GPIO OEN list 0-49. See Table 2-41: GPIO OEN List for SYS_IOMUX (on page 97) for more information."]
pub type DOEN_34_R = crate::FieldReader;
#[doc = "Field `doen_34` writer - The selected OEN signal for GPIO34. The register value indicates the selected GPIO (Output Enable) OEN index from GPIO OEN list 0-49. See Table 2-41: GPIO OEN List for SYS_IOMUX (on page 97) for more information."]
pub type DOEN_34_W<'a, REG> = crate::FieldWriter<'a, REG, 6>;
#[doc = "Field `doen_35` reader - The selected OEN signal for GPIO35. The register value indicates the selected GPIO (Output Enable) OEN index from GPIO OEN list 0-49. See Table 2-41: GPIO OEN List for SYS_IOMUX (on page 97) for more information."]
pub type DOEN_35_R = crate::FieldReader;
#[doc = "Field `doen_35` writer - The selected OEN signal for GPIO35. The register value indicates the selected GPIO (Output Enable) OEN index from GPIO OEN list 0-49. See Table 2-41: GPIO OEN List for SYS_IOMUX (on page 97) for more information."]
pub type DOEN_35_W<'a, REG> = crate::FieldWriter<'a, REG, 6>;
impl R {
    #[doc = "Bits 0:5 - The selected OEN signal for GPIO32. The register value indicates the selected GPIO (Output Enable) OEN index from GPIO OEN list 0-49. See Table 2-41: GPIO OEN List for SYS_IOMUX (on page 97) for more information."]
    #[inline(always)]
    pub fn doen_32(&self) -> DOEN_32_R {
        DOEN_32_R::new((self.bits & 0x3f) as u8)
    }
    #[doc = "Bits 8:13 - The selected OEN signal for GPIO33. The register value indicates the selected GPIO (Output Enable) OEN index from GPIO OEN list 0-49. See Table 2-41: GPIO OEN List for SYS_IOMUX (on page 97) for more information."]
    #[inline(always)]
    pub fn doen_33(&self) -> DOEN_33_R {
        DOEN_33_R::new(((self.bits >> 8) & 0x3f) as u8)
    }
    #[doc = "Bits 16:21 - The selected OEN signal for GPIO34. The register value indicates the selected GPIO (Output Enable) OEN index from GPIO OEN list 0-49. See Table 2-41: GPIO OEN List for SYS_IOMUX (on page 97) for more information."]
    #[inline(always)]
    pub fn doen_34(&self) -> DOEN_34_R {
        DOEN_34_R::new(((self.bits >> 16) & 0x3f) as u8)
    }
    #[doc = "Bits 24:29 - The selected OEN signal for GPIO35. The register value indicates the selected GPIO (Output Enable) OEN index from GPIO OEN list 0-49. See Table 2-41: GPIO OEN List for SYS_IOMUX (on page 97) for more information."]
    #[inline(always)]
    pub fn doen_35(&self) -> DOEN_35_R {
        DOEN_35_R::new(((self.bits >> 24) & 0x3f) as u8)
    }
}
impl W {
    #[doc = "Bits 0:5 - The selected OEN signal for GPIO32. The register value indicates the selected GPIO (Output Enable) OEN index from GPIO OEN list 0-49. See Table 2-41: GPIO OEN List for SYS_IOMUX (on page 97) for more information."]
    #[inline(always)]
    #[must_use]
    pub fn doen_32(&mut self) -> DOEN_32_W<GPO_DOEN_8_SPEC> {
        DOEN_32_W::new(self, 0)
    }
    #[doc = "Bits 8:13 - The selected OEN signal for GPIO33. The register value indicates the selected GPIO (Output Enable) OEN index from GPIO OEN list 0-49. See Table 2-41: GPIO OEN List for SYS_IOMUX (on page 97) for more information."]
    #[inline(always)]
    #[must_use]
    pub fn doen_33(&mut self) -> DOEN_33_W<GPO_DOEN_8_SPEC> {
        DOEN_33_W::new(self, 8)
    }
    #[doc = "Bits 16:21 - The selected OEN signal for GPIO34. The register value indicates the selected GPIO (Output Enable) OEN index from GPIO OEN list 0-49. See Table 2-41: GPIO OEN List for SYS_IOMUX (on page 97) for more information."]
    #[inline(always)]
    #[must_use]
    pub fn doen_34(&mut self) -> DOEN_34_W<GPO_DOEN_8_SPEC> {
        DOEN_34_W::new(self, 16)
    }
    #[doc = "Bits 24:29 - The selected OEN signal for GPIO35. The register value indicates the selected GPIO (Output Enable) OEN index from GPIO OEN list 0-49. See Table 2-41: GPIO OEN List for SYS_IOMUX (on page 97) for more information."]
    #[inline(always)]
    #[must_use]
    pub fn doen_35(&mut self) -> DOEN_35_W<GPO_DOEN_8_SPEC> {
        DOEN_35_W::new(self, 24)
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
#[doc = "SYS IOMUX CFG SAIF SYSCFG FMUX 8 DOEN\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`gpo_doen_8::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`gpo_doen_8::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct GPO_DOEN_8_SPEC;
impl crate::RegisterSpec for GPO_DOEN_8_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`gpo_doen_8::R`](R) reader structure"]
impl crate::Readable for GPO_DOEN_8_SPEC {}
#[doc = "`write(|w| ..)` method takes [`gpo_doen_8::W`](W) writer structure"]
impl crate::Writable for GPO_DOEN_8_SPEC {
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets gpo_doen_8 to value 0"]
impl crate::Resettable for GPO_DOEN_8_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
