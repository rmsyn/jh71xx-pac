#[doc = "Register `gpo_dout_13` reader"]
pub type R = crate::R<GPO_DOUT_13_SPEC>;
#[doc = "Register `gpo_dout_13` writer"]
pub type W = crate::W<GPO_DOUT_13_SPEC>;
#[doc = "Field `dout_52` reader - The selected output signal for GPIO52. The register value indicates the selected GPIO output index signal index from GPIO output signal list 0-107. See Table 2-41: GPIO OEN List for SYS_IOMUX (on page 97) for more information."]
pub type DOUT_52_R = crate::FieldReader;
#[doc = "Field `dout_52` writer - The selected output signal for GPIO52. The register value indicates the selected GPIO output index signal index from GPIO output signal list 0-107. See Table 2-41: GPIO OEN List for SYS_IOMUX (on page 97) for more information."]
pub type DOUT_52_W<'a, REG> = crate::FieldWriter<'a, REG, 7>;
#[doc = "Field `dout_53` reader - The selected output signal for GPIO53. The register value indicates the selected GPIO output index signal index from GPIO output signal list 0-107. See Table 2-41: GPIO OEN List for SYS_IOMUX (on page 97) for more information."]
pub type DOUT_53_R = crate::FieldReader;
#[doc = "Field `dout_53` writer - The selected output signal for GPIO53. The register value indicates the selected GPIO output index signal index from GPIO output signal list 0-107. See Table 2-41: GPIO OEN List for SYS_IOMUX (on page 97) for more information."]
pub type DOUT_53_W<'a, REG> = crate::FieldWriter<'a, REG, 7>;
#[doc = "Field `dout_54` reader - The selected output signal for GPIO54. The register value indicates the selected GPIO output index signal index from GPIO output signal list 0-107. See Table 2-41: GPIO OEN List for SYS_IOMUX (on page 97) for more information."]
pub type DOUT_54_R = crate::FieldReader;
#[doc = "Field `dout_54` writer - The selected output signal for GPIO54. The register value indicates the selected GPIO output index signal index from GPIO output signal list 0-107. See Table 2-41: GPIO OEN List for SYS_IOMUX (on page 97) for more information."]
pub type DOUT_54_W<'a, REG> = crate::FieldWriter<'a, REG, 7>;
#[doc = "Field `dout_55` reader - The selected output signal for GPIO55. The register value indicates the selected GPIO output index signal index from GPIO output signal list 0-107. See Table 2-41: GPIO OEN List for SYS_IOMUX (on page 97) for more information."]
pub type DOUT_55_R = crate::FieldReader;
#[doc = "Field `dout_55` writer - The selected output signal for GPIO55. The register value indicates the selected GPIO output index signal index from GPIO output signal list 0-107. See Table 2-41: GPIO OEN List for SYS_IOMUX (on page 97) for more information."]
pub type DOUT_55_W<'a, REG> = crate::FieldWriter<'a, REG, 7>;
impl R {
    #[doc = "Bits 0:6 - The selected output signal for GPIO52. The register value indicates the selected GPIO output index signal index from GPIO output signal list 0-107. See Table 2-41: GPIO OEN List for SYS_IOMUX (on page 97) for more information."]
    #[inline(always)]
    pub fn dout_52(&self) -> DOUT_52_R {
        DOUT_52_R::new((self.bits & 0x7f) as u8)
    }
    #[doc = "Bits 8:14 - The selected output signal for GPIO53. The register value indicates the selected GPIO output index signal index from GPIO output signal list 0-107. See Table 2-41: GPIO OEN List for SYS_IOMUX (on page 97) for more information."]
    #[inline(always)]
    pub fn dout_53(&self) -> DOUT_53_R {
        DOUT_53_R::new(((self.bits >> 8) & 0x7f) as u8)
    }
    #[doc = "Bits 16:22 - The selected output signal for GPIO54. The register value indicates the selected GPIO output index signal index from GPIO output signal list 0-107. See Table 2-41: GPIO OEN List for SYS_IOMUX (on page 97) for more information."]
    #[inline(always)]
    pub fn dout_54(&self) -> DOUT_54_R {
        DOUT_54_R::new(((self.bits >> 16) & 0x7f) as u8)
    }
    #[doc = "Bits 24:30 - The selected output signal for GPIO55. The register value indicates the selected GPIO output index signal index from GPIO output signal list 0-107. See Table 2-41: GPIO OEN List for SYS_IOMUX (on page 97) for more information."]
    #[inline(always)]
    pub fn dout_55(&self) -> DOUT_55_R {
        DOUT_55_R::new(((self.bits >> 24) & 0x7f) as u8)
    }
}
impl W {
    #[doc = "Bits 0:6 - The selected output signal for GPIO52. The register value indicates the selected GPIO output index signal index from GPIO output signal list 0-107. See Table 2-41: GPIO OEN List for SYS_IOMUX (on page 97) for more information."]
    #[inline(always)]
    #[must_use]
    pub fn dout_52(&mut self) -> DOUT_52_W<GPO_DOUT_13_SPEC> {
        DOUT_52_W::new(self, 0)
    }
    #[doc = "Bits 8:14 - The selected output signal for GPIO53. The register value indicates the selected GPIO output index signal index from GPIO output signal list 0-107. See Table 2-41: GPIO OEN List for SYS_IOMUX (on page 97) for more information."]
    #[inline(always)]
    #[must_use]
    pub fn dout_53(&mut self) -> DOUT_53_W<GPO_DOUT_13_SPEC> {
        DOUT_53_W::new(self, 8)
    }
    #[doc = "Bits 16:22 - The selected output signal for GPIO54. The register value indicates the selected GPIO output index signal index from GPIO output signal list 0-107. See Table 2-41: GPIO OEN List for SYS_IOMUX (on page 97) for more information."]
    #[inline(always)]
    #[must_use]
    pub fn dout_54(&mut self) -> DOUT_54_W<GPO_DOUT_13_SPEC> {
        DOUT_54_W::new(self, 16)
    }
    #[doc = "Bits 24:30 - The selected output signal for GPIO55. The register value indicates the selected GPIO output index signal index from GPIO output signal list 0-107. See Table 2-41: GPIO OEN List for SYS_IOMUX (on page 97) for more information."]
    #[inline(always)]
    #[must_use]
    pub fn dout_55(&mut self) -> DOUT_55_W<GPO_DOUT_13_SPEC> {
        DOUT_55_W::new(self, 24)
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
#[doc = "SYS IOMUX CFG SAIF SYSCFG FMUX GPIO 52-55 DOUT\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`gpo_dout_13::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`gpo_dout_13::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct GPO_DOUT_13_SPEC;
impl crate::RegisterSpec for GPO_DOUT_13_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`gpo_dout_13::R`](R) reader structure"]
impl crate::Readable for GPO_DOUT_13_SPEC {}
#[doc = "`write(|w| ..)` method takes [`gpo_dout_13::W`](W) writer structure"]
impl crate::Writable for GPO_DOUT_13_SPEC {
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets gpo_dout_13 to value 0x4b00_494a"]
impl crate::Resettable for GPO_DOUT_13_SPEC {
    const RESET_VALUE: Self::Ux = 0x4b00_494a;
}
