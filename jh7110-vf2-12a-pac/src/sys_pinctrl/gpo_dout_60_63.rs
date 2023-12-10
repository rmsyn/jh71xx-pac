#[doc = "Register `gpo_dout_60_63` reader"]
pub type R = crate::R<GPO_DOUT_60_63_SPEC>;
#[doc = "Register `gpo_dout_60_63` writer"]
pub type W = crate::W<GPO_DOUT_60_63_SPEC>;
#[doc = "Field `dout_60` reader - The selected output signal for GPIO60. The register value indicates the selected GPIO output index signal index from GPIO output signal list 0-107. See Table 2-41: GPIO OEN List for SYS_IOMUX (on page 97) for more information."]
pub type DOUT_60_R = crate::FieldReader;
#[doc = "Field `dout_60` writer - The selected output signal for GPIO60. The register value indicates the selected GPIO output index signal index from GPIO output signal list 0-107. See Table 2-41: GPIO OEN List for SYS_IOMUX (on page 97) for more information."]
pub type DOUT_60_W<'a, REG> = crate::FieldWriter<'a, REG, 7>;
#[doc = "Field `dout_61` reader - The selected output signal for GPIO61. The register value indicates the selected GPIO output index signal index from GPIO output signal list 0-107. See Table 2-41: GPIO OEN List for SYS_IOMUX (on page 97) for more information."]
pub type DOUT_61_R = crate::FieldReader;
#[doc = "Field `dout_61` writer - The selected output signal for GPIO61. The register value indicates the selected GPIO output index signal index from GPIO output signal list 0-107. See Table 2-41: GPIO OEN List for SYS_IOMUX (on page 97) for more information."]
pub type DOUT_61_W<'a, REG> = crate::FieldWriter<'a, REG, 7>;
#[doc = "Field `dout_62` reader - The selected output signal for GPIO62. The register value indicates the selected GPIO output index signal index from GPIO output signal list 0-107. See Table 2-41: GPIO OEN List for SYS_IOMUX (on page 97) for more information."]
pub type DOUT_62_R = crate::FieldReader;
#[doc = "Field `dout_62` writer - The selected output signal for GPIO62. The register value indicates the selected GPIO output index signal index from GPIO output signal list 0-107. See Table 2-41: GPIO OEN List for SYS_IOMUX (on page 97) for more information."]
pub type DOUT_62_W<'a, REG> = crate::FieldWriter<'a, REG, 7>;
#[doc = "Field `dout_63` reader - The selected output signal for GPIO63. The register value indicates the selected GPIO output index signal index from GPIO output signal list 0-107. See Table 2-41: GPIO OEN List for SYS_IOMUX (on page 97) for more information."]
pub type DOUT_63_R = crate::FieldReader;
#[doc = "Field `dout_63` writer - The selected output signal for GPIO63. The register value indicates the selected GPIO output index signal index from GPIO output signal list 0-107. See Table 2-41: GPIO OEN List for SYS_IOMUX (on page 97) for more information."]
pub type DOUT_63_W<'a, REG> = crate::FieldWriter<'a, REG, 7>;
impl R {
    #[doc = "Bits 0:6 - The selected output signal for GPIO60. The register value indicates the selected GPIO output index signal index from GPIO output signal list 0-107. See Table 2-41: GPIO OEN List for SYS_IOMUX (on page 97) for more information."]
    #[inline(always)]
    pub fn dout_60(&self) -> DOUT_60_R {
        DOUT_60_R::new((self.bits & 0x7f) as u8)
    }
    #[doc = "Bits 8:14 - The selected output signal for GPIO61. The register value indicates the selected GPIO output index signal index from GPIO output signal list 0-107. See Table 2-41: GPIO OEN List for SYS_IOMUX (on page 97) for more information."]
    #[inline(always)]
    pub fn dout_61(&self) -> DOUT_61_R {
        DOUT_61_R::new(((self.bits >> 8) & 0x7f) as u8)
    }
    #[doc = "Bits 16:22 - The selected output signal for GPIO62. The register value indicates the selected GPIO output index signal index from GPIO output signal list 0-107. See Table 2-41: GPIO OEN List for SYS_IOMUX (on page 97) for more information."]
    #[inline(always)]
    pub fn dout_62(&self) -> DOUT_62_R {
        DOUT_62_R::new(((self.bits >> 16) & 0x7f) as u8)
    }
    #[doc = "Bits 24:30 - The selected output signal for GPIO63. The register value indicates the selected GPIO output index signal index from GPIO output signal list 0-107. See Table 2-41: GPIO OEN List for SYS_IOMUX (on page 97) for more information."]
    #[inline(always)]
    pub fn dout_63(&self) -> DOUT_63_R {
        DOUT_63_R::new(((self.bits >> 24) & 0x7f) as u8)
    }
}
impl W {
    #[doc = "Bits 0:6 - The selected output signal for GPIO60. The register value indicates the selected GPIO output index signal index from GPIO output signal list 0-107. See Table 2-41: GPIO OEN List for SYS_IOMUX (on page 97) for more information."]
    #[inline(always)]
    #[must_use]
    pub fn dout_60(&mut self) -> DOUT_60_W<GPO_DOUT_60_63_SPEC> {
        DOUT_60_W::new(self, 0)
    }
    #[doc = "Bits 8:14 - The selected output signal for GPIO61. The register value indicates the selected GPIO output index signal index from GPIO output signal list 0-107. See Table 2-41: GPIO OEN List for SYS_IOMUX (on page 97) for more information."]
    #[inline(always)]
    #[must_use]
    pub fn dout_61(&mut self) -> DOUT_61_W<GPO_DOUT_60_63_SPEC> {
        DOUT_61_W::new(self, 8)
    }
    #[doc = "Bits 16:22 - The selected output signal for GPIO62. The register value indicates the selected GPIO output index signal index from GPIO output signal list 0-107. See Table 2-41: GPIO OEN List for SYS_IOMUX (on page 97) for more information."]
    #[inline(always)]
    #[must_use]
    pub fn dout_62(&mut self) -> DOUT_62_W<GPO_DOUT_60_63_SPEC> {
        DOUT_62_W::new(self, 16)
    }
    #[doc = "Bits 24:30 - The selected output signal for GPIO63. The register value indicates the selected GPIO output index signal index from GPIO output signal list 0-107. See Table 2-41: GPIO OEN List for SYS_IOMUX (on page 97) for more information."]
    #[inline(always)]
    #[must_use]
    pub fn dout_63(&mut self) -> DOUT_63_W<GPO_DOUT_60_63_SPEC> {
        DOUT_63_W::new(self, 24)
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
#[doc = "SYS IOMUX CFG SAIF SYSCFG FMUX GPIO 60-63 DOUT\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`gpo_dout_60_63::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`gpo_dout_60_63::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct GPO_DOUT_60_63_SPEC;
impl crate::RegisterSpec for GPO_DOUT_60_63_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`gpo_dout_60_63::R`](R) reader structure"]
impl crate::Readable for GPO_DOUT_60_63_SPEC {}
#[doc = "`write(|w| ..)` method takes [`gpo_dout_60_63::W`](W) writer structure"]
impl crate::Writable for GPO_DOUT_60_63_SPEC {
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets gpo_dout_60_63 to value 0x5f00_5d5e"]
impl crate::Resettable for GPO_DOUT_60_63_SPEC {
    const RESET_VALUE: Self::Ux = 0x5f00_5d5e;
}
