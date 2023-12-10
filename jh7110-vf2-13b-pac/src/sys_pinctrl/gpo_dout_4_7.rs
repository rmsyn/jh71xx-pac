#[doc = "Register `gpo_dout_4_7` reader"]
pub type R = crate::R<GPO_DOUT_4_7_SPEC>;
#[doc = "Register `gpo_dout_4_7` writer"]
pub type W = crate::W<GPO_DOUT_4_7_SPEC>;
#[doc = "Field `dout_4` reader - The selected output signal for GPIO4. The register value indicates the selected GPIO output index signal index from GPIO output signal list 0-107. See Table 2-41: GPIO OEN List for SYS_IOMUX (on page 97) for more information."]
pub type DOUT_4_R = crate::FieldReader;
#[doc = "Field `dout_4` writer - The selected output signal for GPIO4. The register value indicates the selected GPIO output index signal index from GPIO output signal list 0-107. See Table 2-41: GPIO OEN List for SYS_IOMUX (on page 97) for more information."]
pub type DOUT_4_W<'a, REG> = crate::FieldWriter<'a, REG, 7>;
#[doc = "Field `dout_5` reader - The selected output signal for GPIO5. The register value indicates the selected GPIO output index signal index from GPIO output signal list 0-107. See Table 2-41: GPIO OEN List for SYS_IOMUX (on page 97) for more information."]
pub type DOUT_5_R = crate::FieldReader;
#[doc = "Field `dout_5` writer - The selected output signal for GPIO5. The register value indicates the selected GPIO output index signal index from GPIO output signal list 0-107. See Table 2-41: GPIO OEN List for SYS_IOMUX (on page 97) for more information."]
pub type DOUT_5_W<'a, REG> = crate::FieldWriter<'a, REG, 7>;
#[doc = "Field `dout_6` reader - The selected output signal for GPIO6. The register value indicates the selected GPIO output index signal index from GPIO output signal list 0-107. See Table 2-41: GPIO OEN List for SYS_IOMUX (on page 97) for more information."]
pub type DOUT_6_R = crate::FieldReader;
#[doc = "Field `dout_6` writer - The selected output signal for GPIO6. The register value indicates the selected GPIO output index signal index from GPIO output signal list 0-107. See Table 2-41: GPIO OEN List for SYS_IOMUX (on page 97) for more information."]
pub type DOUT_6_W<'a, REG> = crate::FieldWriter<'a, REG, 7>;
#[doc = "Field `dout_7` reader - The selected output signal for GPIO7. The register value indicates the selected GPIO output index signal index from GPIO output signal list 0-107. See Table 2-41: GPIO OEN List for SYS_IOMUX (on page 97) for more information."]
pub type DOUT_7_R = crate::FieldReader;
#[doc = "Field `dout_7` writer - The selected output signal for GPIO7. The register value indicates the selected GPIO output index signal index from GPIO output signal list 0-107. See Table 2-41: GPIO OEN List for SYS_IOMUX (on page 97) for more information."]
pub type DOUT_7_W<'a, REG> = crate::FieldWriter<'a, REG, 7>;
impl R {
    #[doc = "Bits 0:6 - The selected output signal for GPIO4. The register value indicates the selected GPIO output index signal index from GPIO output signal list 0-107. See Table 2-41: GPIO OEN List for SYS_IOMUX (on page 97) for more information."]
    #[inline(always)]
    pub fn dout_4(&self) -> DOUT_4_R {
        DOUT_4_R::new((self.bits & 0x7f) as u8)
    }
    #[doc = "Bits 8:14 - The selected output signal for GPIO5. The register value indicates the selected GPIO output index signal index from GPIO output signal list 0-107. See Table 2-41: GPIO OEN List for SYS_IOMUX (on page 97) for more information."]
    #[inline(always)]
    pub fn dout_5(&self) -> DOUT_5_R {
        DOUT_5_R::new(((self.bits >> 8) & 0x7f) as u8)
    }
    #[doc = "Bits 16:22 - The selected output signal for GPIO6. The register value indicates the selected GPIO output index signal index from GPIO output signal list 0-107. See Table 2-41: GPIO OEN List for SYS_IOMUX (on page 97) for more information."]
    #[inline(always)]
    pub fn dout_6(&self) -> DOUT_6_R {
        DOUT_6_R::new(((self.bits >> 16) & 0x7f) as u8)
    }
    #[doc = "Bits 24:30 - The selected output signal for GPIO7. The register value indicates the selected GPIO output index signal index from GPIO output signal list 0-107. See Table 2-41: GPIO OEN List for SYS_IOMUX (on page 97) for more information."]
    #[inline(always)]
    pub fn dout_7(&self) -> DOUT_7_R {
        DOUT_7_R::new(((self.bits >> 24) & 0x7f) as u8)
    }
}
impl W {
    #[doc = "Bits 0:6 - The selected output signal for GPIO4. The register value indicates the selected GPIO output index signal index from GPIO output signal list 0-107. See Table 2-41: GPIO OEN List for SYS_IOMUX (on page 97) for more information."]
    #[inline(always)]
    #[must_use]
    pub fn dout_4(&mut self) -> DOUT_4_W<GPO_DOUT_4_7_SPEC> {
        DOUT_4_W::new(self, 0)
    }
    #[doc = "Bits 8:14 - The selected output signal for GPIO5. The register value indicates the selected GPIO output index signal index from GPIO output signal list 0-107. See Table 2-41: GPIO OEN List for SYS_IOMUX (on page 97) for more information."]
    #[inline(always)]
    #[must_use]
    pub fn dout_5(&mut self) -> DOUT_5_W<GPO_DOUT_4_7_SPEC> {
        DOUT_5_W::new(self, 8)
    }
    #[doc = "Bits 16:22 - The selected output signal for GPIO6. The register value indicates the selected GPIO output index signal index from GPIO output signal list 0-107. See Table 2-41: GPIO OEN List for SYS_IOMUX (on page 97) for more information."]
    #[inline(always)]
    #[must_use]
    pub fn dout_6(&mut self) -> DOUT_6_W<GPO_DOUT_4_7_SPEC> {
        DOUT_6_W::new(self, 16)
    }
    #[doc = "Bits 24:30 - The selected output signal for GPIO7. The register value indicates the selected GPIO output index signal index from GPIO output signal list 0-107. See Table 2-41: GPIO OEN List for SYS_IOMUX (on page 97) for more information."]
    #[inline(always)]
    #[must_use]
    pub fn dout_7(&mut self) -> DOUT_7_W<GPO_DOUT_4_7_SPEC> {
        DOUT_7_W::new(self, 24)
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
#[doc = "SYS IOMUX CFG SAIF SYSCFG FMUX GPIO 4-7 DOUT\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`gpo_dout_4_7::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`gpo_dout_4_7::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct GPO_DOUT_4_7_SPEC;
impl crate::RegisterSpec for GPO_DOUT_4_7_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`gpo_dout_4_7::R`](R) reader structure"]
impl crate::Readable for GPO_DOUT_4_7_SPEC {}
#[doc = "`write(|w| ..)` method takes [`gpo_dout_4_7::W`](W) writer structure"]
impl crate::Writable for GPO_DOUT_4_7_SPEC {
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets gpo_dout_4_7 to value 0x1400"]
impl crate::Resettable for GPO_DOUT_4_7_SPEC {
    const RESET_VALUE: Self::Ux = 0x1400;
}
