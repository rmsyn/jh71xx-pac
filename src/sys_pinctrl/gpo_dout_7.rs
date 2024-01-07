#[doc = "Register `gpo_dout_7` reader"]
pub type R = crate::R<GPO_DOUT_7_SPEC>;
#[doc = "Register `gpo_dout_7` writer"]
pub type W = crate::W<GPO_DOUT_7_SPEC>;
#[doc = "Field `dout_28` reader - The selected output signal for GPIO28. The register value indicates the selected GPIO output index signal index from GPIO output signal list 0-107. See Table 2-41: GPIO OEN List for SYS_IOMUX (on page 97) for more information."]
pub type DOUT_28_R = crate::FieldReader;
#[doc = "Field `dout_28` writer - The selected output signal for GPIO28. The register value indicates the selected GPIO output index signal index from GPIO output signal list 0-107. See Table 2-41: GPIO OEN List for SYS_IOMUX (on page 97) for more information."]
pub type DOUT_28_W<'a, REG> = crate::FieldWriter<'a, REG, 7>;
#[doc = "Field `dout_29` reader - The selected output signal for GPIO29. The register value indicates the selected GPIO output index signal index from GPIO output signal list 0-107. See Table 2-41: GPIO OEN List for SYS_IOMUX (on page 97) for more information."]
pub type DOUT_29_R = crate::FieldReader;
#[doc = "Field `dout_29` writer - The selected output signal for GPIO29. The register value indicates the selected GPIO output index signal index from GPIO output signal list 0-107. See Table 2-41: GPIO OEN List for SYS_IOMUX (on page 97) for more information."]
pub type DOUT_29_W<'a, REG> = crate::FieldWriter<'a, REG, 7>;
#[doc = "Field `dout_30` reader - The selected output signal for GPIO30. The register value indicates the selected GPIO output index signal index from GPIO output signal list 0-107. See Table 2-41: GPIO OEN List for SYS_IOMUX (on page 97) for more information."]
pub type DOUT_30_R = crate::FieldReader;
#[doc = "Field `dout_30` writer - The selected output signal for GPIO30. The register value indicates the selected GPIO output index signal index from GPIO output signal list 0-107. See Table 2-41: GPIO OEN List for SYS_IOMUX (on page 97) for more information."]
pub type DOUT_30_W<'a, REG> = crate::FieldWriter<'a, REG, 7>;
#[doc = "Field `dout_31` reader - The selected output signal for GPIO31. The register value indicates the selected GPIO output index signal index from GPIO output signal list 0-107. See Table 2-41: GPIO OEN List for SYS_IOMUX (on page 97) for more information."]
pub type DOUT_31_R = crate::FieldReader;
#[doc = "Field `dout_31` writer - The selected output signal for GPIO31. The register value indicates the selected GPIO output index signal index from GPIO output signal list 0-107. See Table 2-41: GPIO OEN List for SYS_IOMUX (on page 97) for more information."]
pub type DOUT_31_W<'a, REG> = crate::FieldWriter<'a, REG, 7>;
impl R {
    #[doc = "Bits 0:6 - The selected output signal for GPIO28. The register value indicates the selected GPIO output index signal index from GPIO output signal list 0-107. See Table 2-41: GPIO OEN List for SYS_IOMUX (on page 97) for more information."]
    #[inline(always)]
    pub fn dout_28(&self) -> DOUT_28_R {
        DOUT_28_R::new((self.bits & 0x7f) as u8)
    }
    #[doc = "Bits 8:14 - The selected output signal for GPIO29. The register value indicates the selected GPIO output index signal index from GPIO output signal list 0-107. See Table 2-41: GPIO OEN List for SYS_IOMUX (on page 97) for more information."]
    #[inline(always)]
    pub fn dout_29(&self) -> DOUT_29_R {
        DOUT_29_R::new(((self.bits >> 8) & 0x7f) as u8)
    }
    #[doc = "Bits 16:22 - The selected output signal for GPIO30. The register value indicates the selected GPIO output index signal index from GPIO output signal list 0-107. See Table 2-41: GPIO OEN List for SYS_IOMUX (on page 97) for more information."]
    #[inline(always)]
    pub fn dout_30(&self) -> DOUT_30_R {
        DOUT_30_R::new(((self.bits >> 16) & 0x7f) as u8)
    }
    #[doc = "Bits 24:30 - The selected output signal for GPIO31. The register value indicates the selected GPIO output index signal index from GPIO output signal list 0-107. See Table 2-41: GPIO OEN List for SYS_IOMUX (on page 97) for more information."]
    #[inline(always)]
    pub fn dout_31(&self) -> DOUT_31_R {
        DOUT_31_R::new(((self.bits >> 24) & 0x7f) as u8)
    }
}
impl W {
    #[doc = "Bits 0:6 - The selected output signal for GPIO28. The register value indicates the selected GPIO output index signal index from GPIO output signal list 0-107. See Table 2-41: GPIO OEN List for SYS_IOMUX (on page 97) for more information."]
    #[inline(always)]
    #[must_use]
    pub fn dout_28(&mut self) -> DOUT_28_W<GPO_DOUT_7_SPEC> {
        DOUT_28_W::new(self, 0)
    }
    #[doc = "Bits 8:14 - The selected output signal for GPIO29. The register value indicates the selected GPIO output index signal index from GPIO output signal list 0-107. See Table 2-41: GPIO OEN List for SYS_IOMUX (on page 97) for more information."]
    #[inline(always)]
    #[must_use]
    pub fn dout_29(&mut self) -> DOUT_29_W<GPO_DOUT_7_SPEC> {
        DOUT_29_W::new(self, 8)
    }
    #[doc = "Bits 16:22 - The selected output signal for GPIO30. The register value indicates the selected GPIO output index signal index from GPIO output signal list 0-107. See Table 2-41: GPIO OEN List for SYS_IOMUX (on page 97) for more information."]
    #[inline(always)]
    #[must_use]
    pub fn dout_30(&mut self) -> DOUT_30_W<GPO_DOUT_7_SPEC> {
        DOUT_30_W::new(self, 16)
    }
    #[doc = "Bits 24:30 - The selected output signal for GPIO31. The register value indicates the selected GPIO output index signal index from GPIO output signal list 0-107. See Table 2-41: GPIO OEN List for SYS_IOMUX (on page 97) for more information."]
    #[inline(always)]
    #[must_use]
    pub fn dout_31(&mut self) -> DOUT_31_W<GPO_DOUT_7_SPEC> {
        DOUT_31_W::new(self, 24)
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
#[doc = "SYS IOMUX CFG SAIF SYSCFG FMUX GPIO 28-31 DOUT\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`gpo_dout_7::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`gpo_dout_7::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct GPO_DOUT_7_SPEC;
impl crate::RegisterSpec for GPO_DOUT_7_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`gpo_dout_7::R`](R) reader structure"]
impl crate::Readable for GPO_DOUT_7_SPEC {}
#[doc = "`write(|w| ..)` method takes [`gpo_dout_7::W`](W) writer structure"]
impl crate::Writable for GPO_DOUT_7_SPEC {
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets gpo_dout_7 to value 0"]
impl crate::Resettable for GPO_DOUT_7_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
