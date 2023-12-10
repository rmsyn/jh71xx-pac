#[doc = "Register `gpo_dout_12_15` reader"]
pub type R = crate::R<GPO_DOUT_12_15_SPEC>;
#[doc = "Register `gpo_dout_12_15` writer"]
pub type W = crate::W<GPO_DOUT_12_15_SPEC>;
#[doc = "Field `dout_12` reader - The selected output signal for GPIO12. The register value indicates the selected GPIO output index signal index from GPIO output signal list 0-107. See Table 2-41: GPIO OEN List for SYS_IOMUX (on page 97) for more information."]
pub type DOUT_12_R = crate::FieldReader;
#[doc = "Field `dout_12` writer - The selected output signal for GPIO12. The register value indicates the selected GPIO output index signal index from GPIO output signal list 0-107. See Table 2-41: GPIO OEN List for SYS_IOMUX (on page 97) for more information."]
pub type DOUT_12_W<'a, REG> = crate::FieldWriter<'a, REG, 7>;
#[doc = "Field `dout_13` reader - The selected output signal for GPIO13. The register value indicates the selected GPIO output index signal index from GPIO output signal list 0-107. See Table 2-41: GPIO OEN List for SYS_IOMUX (on page 97) for more information."]
pub type DOUT_13_R = crate::FieldReader;
#[doc = "Field `dout_13` writer - The selected output signal for GPIO13. The register value indicates the selected GPIO output index signal index from GPIO output signal list 0-107. See Table 2-41: GPIO OEN List for SYS_IOMUX (on page 97) for more information."]
pub type DOUT_13_W<'a, REG> = crate::FieldWriter<'a, REG, 7>;
#[doc = "Field `dout_14` reader - The selected output signal for GPIO14. The register value indicates the selected GPIO output index signal index from GPIO output signal list 0-107. See Table 2-41: GPIO OEN List for SYS_IOMUX (on page 97) for more information."]
pub type DOUT_14_R = crate::FieldReader;
#[doc = "Field `dout_14` writer - The selected output signal for GPIO14. The register value indicates the selected GPIO output index signal index from GPIO output signal list 0-107. See Table 2-41: GPIO OEN List for SYS_IOMUX (on page 97) for more information."]
pub type DOUT_14_W<'a, REG> = crate::FieldWriter<'a, REG, 7>;
#[doc = "Field `dout_15` reader - The selected output signal for GPIO15. The register value indicates the selected GPIO output index signal index from GPIO output signal list 0-107. See Table 2-41: GPIO OEN List for SYS_IOMUX (on page 97) for more information."]
pub type DOUT_15_R = crate::FieldReader;
#[doc = "Field `dout_15` writer - The selected output signal for GPIO15. The register value indicates the selected GPIO output index signal index from GPIO output signal list 0-107. See Table 2-41: GPIO OEN List for SYS_IOMUX (on page 97) for more information."]
pub type DOUT_15_W<'a, REG> = crate::FieldWriter<'a, REG, 7>;
impl R {
    #[doc = "Bits 0:6 - The selected output signal for GPIO12. The register value indicates the selected GPIO output index signal index from GPIO output signal list 0-107. See Table 2-41: GPIO OEN List for SYS_IOMUX (on page 97) for more information."]
    #[inline(always)]
    pub fn dout_12(&self) -> DOUT_12_R {
        DOUT_12_R::new((self.bits & 0x7f) as u8)
    }
    #[doc = "Bits 8:14 - The selected output signal for GPIO13. The register value indicates the selected GPIO output index signal index from GPIO output signal list 0-107. See Table 2-41: GPIO OEN List for SYS_IOMUX (on page 97) for more information."]
    #[inline(always)]
    pub fn dout_13(&self) -> DOUT_13_R {
        DOUT_13_R::new(((self.bits >> 8) & 0x7f) as u8)
    }
    #[doc = "Bits 16:22 - The selected output signal for GPIO14. The register value indicates the selected GPIO output index signal index from GPIO output signal list 0-107. See Table 2-41: GPIO OEN List for SYS_IOMUX (on page 97) for more information."]
    #[inline(always)]
    pub fn dout_14(&self) -> DOUT_14_R {
        DOUT_14_R::new(((self.bits >> 16) & 0x7f) as u8)
    }
    #[doc = "Bits 24:30 - The selected output signal for GPIO15. The register value indicates the selected GPIO output index signal index from GPIO output signal list 0-107. See Table 2-41: GPIO OEN List for SYS_IOMUX (on page 97) for more information."]
    #[inline(always)]
    pub fn dout_15(&self) -> DOUT_15_R {
        DOUT_15_R::new(((self.bits >> 24) & 0x7f) as u8)
    }
}
impl W {
    #[doc = "Bits 0:6 - The selected output signal for GPIO12. The register value indicates the selected GPIO output index signal index from GPIO output signal list 0-107. See Table 2-41: GPIO OEN List for SYS_IOMUX (on page 97) for more information."]
    #[inline(always)]
    #[must_use]
    pub fn dout_12(&mut self) -> DOUT_12_W<GPO_DOUT_12_15_SPEC> {
        DOUT_12_W::new(self, 0)
    }
    #[doc = "Bits 8:14 - The selected output signal for GPIO13. The register value indicates the selected GPIO output index signal index from GPIO output signal list 0-107. See Table 2-41: GPIO OEN List for SYS_IOMUX (on page 97) for more information."]
    #[inline(always)]
    #[must_use]
    pub fn dout_13(&mut self) -> DOUT_13_W<GPO_DOUT_12_15_SPEC> {
        DOUT_13_W::new(self, 8)
    }
    #[doc = "Bits 16:22 - The selected output signal for GPIO14. The register value indicates the selected GPIO output index signal index from GPIO output signal list 0-107. See Table 2-41: GPIO OEN List for SYS_IOMUX (on page 97) for more information."]
    #[inline(always)]
    #[must_use]
    pub fn dout_14(&mut self) -> DOUT_14_W<GPO_DOUT_12_15_SPEC> {
        DOUT_14_W::new(self, 16)
    }
    #[doc = "Bits 24:30 - The selected output signal for GPIO15. The register value indicates the selected GPIO output index signal index from GPIO output signal list 0-107. See Table 2-41: GPIO OEN List for SYS_IOMUX (on page 97) for more information."]
    #[inline(always)]
    #[must_use]
    pub fn dout_15(&mut self) -> DOUT_15_W<GPO_DOUT_12_15_SPEC> {
        DOUT_15_W::new(self, 24)
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
#[doc = "SYS IOMUX CFG SAIF SYSCFG FMUX GPIO 12-15 DOUT\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`gpo_dout_12_15::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`gpo_dout_12_15::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct GPO_DOUT_12_15_SPEC;
impl crate::RegisterSpec for GPO_DOUT_12_15_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`gpo_dout_12_15::R`](R) reader structure"]
impl crate::Readable for GPO_DOUT_12_15_SPEC {}
#[doc = "`write(|w| ..)` method takes [`gpo_dout_12_15::W`](W) writer structure"]
impl crate::Writable for GPO_DOUT_12_15_SPEC {
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets gpo_dout_12_15 to value 0"]
impl crate::Resettable for GPO_DOUT_12_15_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
