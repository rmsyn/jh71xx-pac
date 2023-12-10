#[doc = "Register `gpo_dout_40_43` reader"]
pub type R = crate::R<GPO_DOUT_40_43_SPEC>;
#[doc = "Register `gpo_dout_40_43` writer"]
pub type W = crate::W<GPO_DOUT_40_43_SPEC>;
#[doc = "Field `dout_40` reader - The selected output signal for GPIO40. The register value indicates the selected GPIO output index signal index from GPIO output signal list 0-107. See Table 2-41: GPIO OEN List for SYS_IOMUX (on page 97) for more information."]
pub type DOUT_40_R = crate::FieldReader;
#[doc = "Field `dout_40` writer - The selected output signal for GPIO40. The register value indicates the selected GPIO output index signal index from GPIO output signal list 0-107. See Table 2-41: GPIO OEN List for SYS_IOMUX (on page 97) for more information."]
pub type DOUT_40_W<'a, REG> = crate::FieldWriter<'a, REG, 7>;
#[doc = "Field `dout_41` reader - The selected output signal for GPIO41. The register value indicates the selected GPIO output index signal index from GPIO output signal list 0-107. See Table 2-41: GPIO OEN List for SYS_IOMUX (on page 97) for more information."]
pub type DOUT_41_R = crate::FieldReader;
#[doc = "Field `dout_41` writer - The selected output signal for GPIO41. The register value indicates the selected GPIO output index signal index from GPIO output signal list 0-107. See Table 2-41: GPIO OEN List for SYS_IOMUX (on page 97) for more information."]
pub type DOUT_41_W<'a, REG> = crate::FieldWriter<'a, REG, 7>;
#[doc = "Field `dout_42` reader - The selected output signal for GPIO42. The register value indicates the selected GPIO output index signal index from GPIO output signal list 0-107. See Table 2-41: GPIO OEN List for SYS_IOMUX (on page 97) for more information."]
pub type DOUT_42_R = crate::FieldReader;
#[doc = "Field `dout_42` writer - The selected output signal for GPIO42. The register value indicates the selected GPIO output index signal index from GPIO output signal list 0-107. See Table 2-41: GPIO OEN List for SYS_IOMUX (on page 97) for more information."]
pub type DOUT_42_W<'a, REG> = crate::FieldWriter<'a, REG, 7>;
#[doc = "Field `dout_43` reader - The selected output signal for GPIO43. The register value indicates the selected GPIO output index signal index from GPIO output signal list 0-107. See Table 2-41: GPIO OEN List for SYS_IOMUX (on page 97) for more information."]
pub type DOUT_43_R = crate::FieldReader;
#[doc = "Field `dout_43` writer - The selected output signal for GPIO43. The register value indicates the selected GPIO output index signal index from GPIO output signal list 0-107. See Table 2-41: GPIO OEN List for SYS_IOMUX (on page 97) for more information."]
pub type DOUT_43_W<'a, REG> = crate::FieldWriter<'a, REG, 7>;
impl R {
    #[doc = "Bits 0:6 - The selected output signal for GPIO40. The register value indicates the selected GPIO output index signal index from GPIO output signal list 0-107. See Table 2-41: GPIO OEN List for SYS_IOMUX (on page 97) for more information."]
    #[inline(always)]
    pub fn dout_40(&self) -> DOUT_40_R {
        DOUT_40_R::new((self.bits & 0x7f) as u8)
    }
    #[doc = "Bits 8:14 - The selected output signal for GPIO41. The register value indicates the selected GPIO output index signal index from GPIO output signal list 0-107. See Table 2-41: GPIO OEN List for SYS_IOMUX (on page 97) for more information."]
    #[inline(always)]
    pub fn dout_41(&self) -> DOUT_41_R {
        DOUT_41_R::new(((self.bits >> 8) & 0x7f) as u8)
    }
    #[doc = "Bits 16:22 - The selected output signal for GPIO42. The register value indicates the selected GPIO output index signal index from GPIO output signal list 0-107. See Table 2-41: GPIO OEN List for SYS_IOMUX (on page 97) for more information."]
    #[inline(always)]
    pub fn dout_42(&self) -> DOUT_42_R {
        DOUT_42_R::new(((self.bits >> 16) & 0x7f) as u8)
    }
    #[doc = "Bits 24:30 - The selected output signal for GPIO43. The register value indicates the selected GPIO output index signal index from GPIO output signal list 0-107. See Table 2-41: GPIO OEN List for SYS_IOMUX (on page 97) for more information."]
    #[inline(always)]
    pub fn dout_43(&self) -> DOUT_43_R {
        DOUT_43_R::new(((self.bits >> 24) & 0x7f) as u8)
    }
}
impl W {
    #[doc = "Bits 0:6 - The selected output signal for GPIO40. The register value indicates the selected GPIO output index signal index from GPIO output signal list 0-107. See Table 2-41: GPIO OEN List for SYS_IOMUX (on page 97) for more information."]
    #[inline(always)]
    #[must_use]
    pub fn dout_40(&mut self) -> DOUT_40_W<GPO_DOUT_40_43_SPEC> {
        DOUT_40_W::new(self, 0)
    }
    #[doc = "Bits 8:14 - The selected output signal for GPIO41. The register value indicates the selected GPIO output index signal index from GPIO output signal list 0-107. See Table 2-41: GPIO OEN List for SYS_IOMUX (on page 97) for more information."]
    #[inline(always)]
    #[must_use]
    pub fn dout_41(&mut self) -> DOUT_41_W<GPO_DOUT_40_43_SPEC> {
        DOUT_41_W::new(self, 8)
    }
    #[doc = "Bits 16:22 - The selected output signal for GPIO42. The register value indicates the selected GPIO output index signal index from GPIO output signal list 0-107. See Table 2-41: GPIO OEN List for SYS_IOMUX (on page 97) for more information."]
    #[inline(always)]
    #[must_use]
    pub fn dout_42(&mut self) -> DOUT_42_W<GPO_DOUT_40_43_SPEC> {
        DOUT_42_W::new(self, 16)
    }
    #[doc = "Bits 24:30 - The selected output signal for GPIO43. The register value indicates the selected GPIO output index signal index from GPIO output signal list 0-107. See Table 2-41: GPIO OEN List for SYS_IOMUX (on page 97) for more information."]
    #[inline(always)]
    #[must_use]
    pub fn dout_43(&mut self) -> DOUT_43_W<GPO_DOUT_40_43_SPEC> {
        DOUT_43_W::new(self, 24)
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
#[doc = "SYS IOMUX CFG SAIF SYSCFG FMUX GPIO 40-43 DOUT\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`gpo_dout_40_43::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`gpo_dout_40_43::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct GPO_DOUT_40_43_SPEC;
impl crate::RegisterSpec for GPO_DOUT_40_43_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`gpo_dout_40_43::R`](R) reader structure"]
impl crate::Readable for GPO_DOUT_40_43_SPEC {}
#[doc = "`write(|w| ..)` method takes [`gpo_dout_40_43::W`](W) writer structure"]
impl crate::Writable for GPO_DOUT_40_43_SPEC {
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets gpo_dout_40_43 to value 0x004e_4f00"]
impl crate::Resettable for GPO_DOUT_40_43_SPEC {
    const RESET_VALUE: Self::Ux = 0x004e_4f00;
}
