#[doc = "Register `gpo_dout_48_51` reader"]
pub type R = crate::R<GPO_DOUT_48_51_SPEC>;
#[doc = "Register `gpo_dout_48_51` writer"]
pub type W = crate::W<GPO_DOUT_48_51_SPEC>;
#[doc = "Field `dout_48` reader - The selected output signal for GPIO48. The register value indicates the selected GPIO output index signal index from GPIO output signal list 0-107. See Table 2-41: GPIO OEN List for SYS_IOMUX (on page 97) for more information."]
pub type DOUT_48_R = crate::FieldReader;
#[doc = "Field `dout_48` writer - The selected output signal for GPIO48. The register value indicates the selected GPIO output index signal index from GPIO output signal list 0-107. See Table 2-41: GPIO OEN List for SYS_IOMUX (on page 97) for more information."]
pub type DOUT_48_W<'a, REG> = crate::FieldWriter<'a, REG, 7>;
#[doc = "Field `dout_49` reader - The selected output signal for GPIO49. The register value indicates the selected GPIO output index signal index from GPIO output signal list 0-107. See Table 2-41: GPIO OEN List for SYS_IOMUX (on page 97) for more information."]
pub type DOUT_49_R = crate::FieldReader;
#[doc = "Field `dout_49` writer - The selected output signal for GPIO49. The register value indicates the selected GPIO output index signal index from GPIO output signal list 0-107. See Table 2-41: GPIO OEN List for SYS_IOMUX (on page 97) for more information."]
pub type DOUT_49_W<'a, REG> = crate::FieldWriter<'a, REG, 7>;
#[doc = "Field `dout_50` reader - The selected output signal for GPIO50. The register value indicates the selected GPIO output index signal index from GPIO output signal list 0-107. See Table 2-41: GPIO OEN List for SYS_IOMUX (on page 97) for more information."]
pub type DOUT_50_R = crate::FieldReader;
#[doc = "Field `dout_50` writer - The selected output signal for GPIO50. The register value indicates the selected GPIO output index signal index from GPIO output signal list 0-107. See Table 2-41: GPIO OEN List for SYS_IOMUX (on page 97) for more information."]
pub type DOUT_50_W<'a, REG> = crate::FieldWriter<'a, REG, 7>;
#[doc = "Field `dout_51` reader - The selected output signal for GPIO51. The register value indicates the selected GPIO output index signal index from GPIO output signal list 0-107. See Table 2-41: GPIO OEN List for SYS_IOMUX (on page 97) for more information."]
pub type DOUT_51_R = crate::FieldReader;
#[doc = "Field `dout_51` writer - The selected output signal for GPIO51. The register value indicates the selected GPIO output index signal index from GPIO output signal list 0-107. See Table 2-41: GPIO OEN List for SYS_IOMUX (on page 97) for more information."]
pub type DOUT_51_W<'a, REG> = crate::FieldWriter<'a, REG, 7>;
impl R {
    #[doc = "Bits 0:6 - The selected output signal for GPIO48. The register value indicates the selected GPIO output index signal index from GPIO output signal list 0-107. See Table 2-41: GPIO OEN List for SYS_IOMUX (on page 97) for more information."]
    #[inline(always)]
    pub fn dout_48(&self) -> DOUT_48_R {
        DOUT_48_R::new((self.bits & 0x7f) as u8)
    }
    #[doc = "Bits 8:14 - The selected output signal for GPIO49. The register value indicates the selected GPIO output index signal index from GPIO output signal list 0-107. See Table 2-41: GPIO OEN List for SYS_IOMUX (on page 97) for more information."]
    #[inline(always)]
    pub fn dout_49(&self) -> DOUT_49_R {
        DOUT_49_R::new(((self.bits >> 8) & 0x7f) as u8)
    }
    #[doc = "Bits 16:22 - The selected output signal for GPIO50. The register value indicates the selected GPIO output index signal index from GPIO output signal list 0-107. See Table 2-41: GPIO OEN List for SYS_IOMUX (on page 97) for more information."]
    #[inline(always)]
    pub fn dout_50(&self) -> DOUT_50_R {
        DOUT_50_R::new(((self.bits >> 16) & 0x7f) as u8)
    }
    #[doc = "Bits 24:30 - The selected output signal for GPIO51. The register value indicates the selected GPIO output index signal index from GPIO output signal list 0-107. See Table 2-41: GPIO OEN List for SYS_IOMUX (on page 97) for more information."]
    #[inline(always)]
    pub fn dout_51(&self) -> DOUT_51_R {
        DOUT_51_R::new(((self.bits >> 24) & 0x7f) as u8)
    }
}
impl W {
    #[doc = "Bits 0:6 - The selected output signal for GPIO48. The register value indicates the selected GPIO output index signal index from GPIO output signal list 0-107. See Table 2-41: GPIO OEN List for SYS_IOMUX (on page 97) for more information."]
    #[inline(always)]
    #[must_use]
    pub fn dout_48(&mut self) -> DOUT_48_W<GPO_DOUT_48_51_SPEC> {
        DOUT_48_W::new(self, 0)
    }
    #[doc = "Bits 8:14 - The selected output signal for GPIO49. The register value indicates the selected GPIO output index signal index from GPIO output signal list 0-107. See Table 2-41: GPIO OEN List for SYS_IOMUX (on page 97) for more information."]
    #[inline(always)]
    #[must_use]
    pub fn dout_49(&mut self) -> DOUT_49_W<GPO_DOUT_48_51_SPEC> {
        DOUT_49_W::new(self, 8)
    }
    #[doc = "Bits 16:22 - The selected output signal for GPIO50. The register value indicates the selected GPIO output index signal index from GPIO output signal list 0-107. See Table 2-41: GPIO OEN List for SYS_IOMUX (on page 97) for more information."]
    #[inline(always)]
    #[must_use]
    pub fn dout_50(&mut self) -> DOUT_50_W<GPO_DOUT_48_51_SPEC> {
        DOUT_50_W::new(self, 16)
    }
    #[doc = "Bits 24:30 - The selected output signal for GPIO51. The register value indicates the selected GPIO output index signal index from GPIO output signal list 0-107. See Table 2-41: GPIO OEN List for SYS_IOMUX (on page 97) for more information."]
    #[inline(always)]
    #[must_use]
    pub fn dout_51(&mut self) -> DOUT_51_W<GPO_DOUT_48_51_SPEC> {
        DOUT_51_W::new(self, 24)
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
#[doc = "SYS IOMUX CFG SAIF SYSCFG FMUX GPIO 48-51 DOUT\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`gpo_dout_48_51::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`gpo_dout_48_51::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct GPO_DOUT_48_51_SPEC;
impl crate::RegisterSpec for GPO_DOUT_48_51_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`gpo_dout_48_51::R`](R) reader structure"]
impl crate::Readable for GPO_DOUT_48_51_SPEC {}
#[doc = "`write(|w| ..)` method takes [`gpo_dout_48_51::W`](W) writer structure"]
impl crate::Writable for GPO_DOUT_48_51_SPEC {
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets gpo_dout_48_51 to value 0x2000_1e1f"]
impl crate::Resettable for GPO_DOUT_48_51_SPEC {
    const RESET_VALUE: Self::Ux = 0x2000_1e1f;
}
