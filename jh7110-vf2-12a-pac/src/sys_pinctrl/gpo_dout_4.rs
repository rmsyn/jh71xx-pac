#[doc = "Register `gpo_dout_4` reader"]
pub type R = crate::R<GPO_DOUT_4_SPEC>;
#[doc = "Register `gpo_dout_4` writer"]
pub type W = crate::W<GPO_DOUT_4_SPEC>;
#[doc = "Field `dout_16` reader - The selected output signal for GPIO16. The register value indicates the selected GPIO output index signal index from GPIO output signal list 0-107. See Table 2-41: GPIO OEN List for SYS_IOMUX (on page 97) for more information."]
pub type DOUT_16_R = crate::FieldReader;
#[doc = "Field `dout_16` writer - The selected output signal for GPIO16. The register value indicates the selected GPIO output index signal index from GPIO output signal list 0-107. See Table 2-41: GPIO OEN List for SYS_IOMUX (on page 97) for more information."]
pub type DOUT_16_W<'a, REG> = crate::FieldWriter<'a, REG, 7>;
#[doc = "Field `dout_17` reader - The selected output signal for GPIO17. The register value indicates the selected GPIO output index signal index from GPIO output signal list 0-107. See Table 2-41: GPIO OEN List for SYS_IOMUX (on page 97) for more information."]
pub type DOUT_17_R = crate::FieldReader;
#[doc = "Field `dout_17` writer - The selected output signal for GPIO17. The register value indicates the selected GPIO output index signal index from GPIO output signal list 0-107. See Table 2-41: GPIO OEN List for SYS_IOMUX (on page 97) for more information."]
pub type DOUT_17_W<'a, REG> = crate::FieldWriter<'a, REG, 7>;
#[doc = "Field `dout_18` reader - The selected output signal for GPIO18. The register value indicates the selected GPIO output index signal index from GPIO output signal list 0-107. See Table 2-41: GPIO OEN List for SYS_IOMUX (on page 97) for more information."]
pub type DOUT_18_R = crate::FieldReader;
#[doc = "Field `dout_18` writer - The selected output signal for GPIO18. The register value indicates the selected GPIO output index signal index from GPIO output signal list 0-107. See Table 2-41: GPIO OEN List for SYS_IOMUX (on page 97) for more information."]
pub type DOUT_18_W<'a, REG> = crate::FieldWriter<'a, REG, 7>;
#[doc = "Field `dout_19` reader - The selected output signal for GPIO19. The register value indicates the selected GPIO output index signal index from GPIO output signal list 0-107. See Table 2-41: GPIO OEN List for SYS_IOMUX (on page 97) for more information."]
pub type DOUT_19_R = crate::FieldReader;
#[doc = "Field `dout_19` writer - The selected output signal for GPIO19. The register value indicates the selected GPIO output index signal index from GPIO output signal list 0-107. See Table 2-41: GPIO OEN List for SYS_IOMUX (on page 97) for more information."]
pub type DOUT_19_W<'a, REG> = crate::FieldWriter<'a, REG, 7>;
impl R {
    #[doc = "Bits 0:6 - The selected output signal for GPIO16. The register value indicates the selected GPIO output index signal index from GPIO output signal list 0-107. See Table 2-41: GPIO OEN List for SYS_IOMUX (on page 97) for more information."]
    #[inline(always)]
    pub fn dout_16(&self) -> DOUT_16_R {
        DOUT_16_R::new((self.bits & 0x7f) as u8)
    }
    #[doc = "Bits 8:14 - The selected output signal for GPIO17. The register value indicates the selected GPIO output index signal index from GPIO output signal list 0-107. See Table 2-41: GPIO OEN List for SYS_IOMUX (on page 97) for more information."]
    #[inline(always)]
    pub fn dout_17(&self) -> DOUT_17_R {
        DOUT_17_R::new(((self.bits >> 8) & 0x7f) as u8)
    }
    #[doc = "Bits 16:22 - The selected output signal for GPIO18. The register value indicates the selected GPIO output index signal index from GPIO output signal list 0-107. See Table 2-41: GPIO OEN List for SYS_IOMUX (on page 97) for more information."]
    #[inline(always)]
    pub fn dout_18(&self) -> DOUT_18_R {
        DOUT_18_R::new(((self.bits >> 16) & 0x7f) as u8)
    }
    #[doc = "Bits 24:30 - The selected output signal for GPIO19. The register value indicates the selected GPIO output index signal index from GPIO output signal list 0-107. See Table 2-41: GPIO OEN List for SYS_IOMUX (on page 97) for more information."]
    #[inline(always)]
    pub fn dout_19(&self) -> DOUT_19_R {
        DOUT_19_R::new(((self.bits >> 24) & 0x7f) as u8)
    }
}
impl W {
    #[doc = "Bits 0:6 - The selected output signal for GPIO16. The register value indicates the selected GPIO output index signal index from GPIO output signal list 0-107. See Table 2-41: GPIO OEN List for SYS_IOMUX (on page 97) for more information."]
    #[inline(always)]
    #[must_use]
    pub fn dout_16(&mut self) -> DOUT_16_W<GPO_DOUT_4_SPEC> {
        DOUT_16_W::new(self, 0)
    }
    #[doc = "Bits 8:14 - The selected output signal for GPIO17. The register value indicates the selected GPIO output index signal index from GPIO output signal list 0-107. See Table 2-41: GPIO OEN List for SYS_IOMUX (on page 97) for more information."]
    #[inline(always)]
    #[must_use]
    pub fn dout_17(&mut self) -> DOUT_17_W<GPO_DOUT_4_SPEC> {
        DOUT_17_W::new(self, 8)
    }
    #[doc = "Bits 16:22 - The selected output signal for GPIO18. The register value indicates the selected GPIO output index signal index from GPIO output signal list 0-107. See Table 2-41: GPIO OEN List for SYS_IOMUX (on page 97) for more information."]
    #[inline(always)]
    #[must_use]
    pub fn dout_18(&mut self) -> DOUT_18_W<GPO_DOUT_4_SPEC> {
        DOUT_18_W::new(self, 16)
    }
    #[doc = "Bits 24:30 - The selected output signal for GPIO19. The register value indicates the selected GPIO output index signal index from GPIO output signal list 0-107. See Table 2-41: GPIO OEN List for SYS_IOMUX (on page 97) for more information."]
    #[inline(always)]
    #[must_use]
    pub fn dout_19(&mut self) -> DOUT_19_W<GPO_DOUT_4_SPEC> {
        DOUT_19_W::new(self, 24)
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
#[doc = "SYS IOMUX CFG SAIF SYSCFG FMUX GPIO 16-19 DOUT\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`gpo_dout_4::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`gpo_dout_4::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct GPO_DOUT_4_SPEC;
impl crate::RegisterSpec for GPO_DOUT_4_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`gpo_dout_4::R`](R) reader structure"]
impl crate::Readable for GPO_DOUT_4_SPEC {}
#[doc = "`write(|w| ..)` method takes [`gpo_dout_4::W`](W) writer structure"]
impl crate::Writable for GPO_DOUT_4_SPEC {
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets gpo_dout_4 to value 0x2000_0000"]
impl crate::Resettable for GPO_DOUT_4_SPEC {
    const RESET_VALUE: Self::Ux = 0x2000_0000;
}
