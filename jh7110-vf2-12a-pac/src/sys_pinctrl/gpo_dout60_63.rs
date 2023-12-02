#[doc = "Register `gpo_dout60_63` reader"]
pub type R = crate::R<GPO_DOUT60_63_SPEC>;
#[doc = "Register `gpo_dout60_63` writer"]
pub type W = crate::W<GPO_DOUT60_63_SPEC>;
#[doc = "Field `gpo60_dout` reader - The selected output signal for GPIO60. The register value indicates the selected GPIO output index signal index from GPIO output signal list 0-107. See Table 2-41: GPIO OEN List for SYS_IOMUX (on page 97) for more information."]
pub type GPO60_DOUT_R = crate::FieldReader;
#[doc = "Field `gpo60_dout` writer - The selected output signal for GPIO60. The register value indicates the selected GPIO output index signal index from GPIO output signal list 0-107. See Table 2-41: GPIO OEN List for SYS_IOMUX (on page 97) for more information."]
pub type GPO60_DOUT_W<'a, REG> = crate::FieldWriter<'a, REG, 7>;
#[doc = "Field `gpo61_dout` reader - The selected output signal for GPIO61. The register value indicates the selected GPIO output index signal index from GPIO output signal list 0-107. See Table 2-41: GPIO OEN List for SYS_IOMUX (on page 97) for more information."]
pub type GPO61_DOUT_R = crate::FieldReader;
#[doc = "Field `gpo61_dout` writer - The selected output signal for GPIO61. The register value indicates the selected GPIO output index signal index from GPIO output signal list 0-107. See Table 2-41: GPIO OEN List for SYS_IOMUX (on page 97) for more information."]
pub type GPO61_DOUT_W<'a, REG> = crate::FieldWriter<'a, REG, 7>;
#[doc = "Field `gpo62_dout` reader - The selected output signal for GPIO62. The register value indicates the selected GPIO output index signal index from GPIO output signal list 0-107. See Table 2-41: GPIO OEN List for SYS_IOMUX (on page 97) for more information."]
pub type GPO62_DOUT_R = crate::FieldReader;
#[doc = "Field `gpo62_dout` writer - The selected output signal for GPIO62. The register value indicates the selected GPIO output index signal index from GPIO output signal list 0-107. See Table 2-41: GPIO OEN List for SYS_IOMUX (on page 97) for more information."]
pub type GPO62_DOUT_W<'a, REG> = crate::FieldWriter<'a, REG, 7>;
#[doc = "Field `gpo63_dout` reader - The selected output signal for GPIO63. The register value indicates the selected GPIO output index signal index from GPIO output signal list 0-107. See Table 2-41: GPIO OEN List for SYS_IOMUX (on page 97) for more information."]
pub type GPO63_DOUT_R = crate::FieldReader;
#[doc = "Field `gpo63_dout` writer - The selected output signal for GPIO63. The register value indicates the selected GPIO output index signal index from GPIO output signal list 0-107. See Table 2-41: GPIO OEN List for SYS_IOMUX (on page 97) for more information."]
pub type GPO63_DOUT_W<'a, REG> = crate::FieldWriter<'a, REG, 7>;
impl R {
    #[doc = "Bits 0:6 - The selected output signal for GPIO60. The register value indicates the selected GPIO output index signal index from GPIO output signal list 0-107. See Table 2-41: GPIO OEN List for SYS_IOMUX (on page 97) for more information."]
    #[inline(always)]
    pub fn gpo60_dout(&self) -> GPO60_DOUT_R {
        GPO60_DOUT_R::new((self.bits & 0x7f) as u8)
    }
    #[doc = "Bits 8:14 - The selected output signal for GPIO61. The register value indicates the selected GPIO output index signal index from GPIO output signal list 0-107. See Table 2-41: GPIO OEN List for SYS_IOMUX (on page 97) for more information."]
    #[inline(always)]
    pub fn gpo61_dout(&self) -> GPO61_DOUT_R {
        GPO61_DOUT_R::new(((self.bits >> 8) & 0x7f) as u8)
    }
    #[doc = "Bits 16:22 - The selected output signal for GPIO62. The register value indicates the selected GPIO output index signal index from GPIO output signal list 0-107. See Table 2-41: GPIO OEN List for SYS_IOMUX (on page 97) for more information."]
    #[inline(always)]
    pub fn gpo62_dout(&self) -> GPO62_DOUT_R {
        GPO62_DOUT_R::new(((self.bits >> 16) & 0x7f) as u8)
    }
    #[doc = "Bits 24:30 - The selected output signal for GPIO63. The register value indicates the selected GPIO output index signal index from GPIO output signal list 0-107. See Table 2-41: GPIO OEN List for SYS_IOMUX (on page 97) for more information."]
    #[inline(always)]
    pub fn gpo63_dout(&self) -> GPO63_DOUT_R {
        GPO63_DOUT_R::new(((self.bits >> 24) & 0x7f) as u8)
    }
}
impl W {
    #[doc = "Bits 0:6 - The selected output signal for GPIO60. The register value indicates the selected GPIO output index signal index from GPIO output signal list 0-107. See Table 2-41: GPIO OEN List for SYS_IOMUX (on page 97) for more information."]
    #[inline(always)]
    #[must_use]
    pub fn gpo60_dout(&mut self) -> GPO60_DOUT_W<GPO_DOUT60_63_SPEC> {
        GPO60_DOUT_W::new(self, 0)
    }
    #[doc = "Bits 8:14 - The selected output signal for GPIO61. The register value indicates the selected GPIO output index signal index from GPIO output signal list 0-107. See Table 2-41: GPIO OEN List for SYS_IOMUX (on page 97) for more information."]
    #[inline(always)]
    #[must_use]
    pub fn gpo61_dout(&mut self) -> GPO61_DOUT_W<GPO_DOUT60_63_SPEC> {
        GPO61_DOUT_W::new(self, 8)
    }
    #[doc = "Bits 16:22 - The selected output signal for GPIO62. The register value indicates the selected GPIO output index signal index from GPIO output signal list 0-107. See Table 2-41: GPIO OEN List for SYS_IOMUX (on page 97) for more information."]
    #[inline(always)]
    #[must_use]
    pub fn gpo62_dout(&mut self) -> GPO62_DOUT_W<GPO_DOUT60_63_SPEC> {
        GPO62_DOUT_W::new(self, 16)
    }
    #[doc = "Bits 24:30 - The selected output signal for GPIO63. The register value indicates the selected GPIO output index signal index from GPIO output signal list 0-107. See Table 2-41: GPIO OEN List for SYS_IOMUX (on page 97) for more information."]
    #[inline(always)]
    #[must_use]
    pub fn gpo63_dout(&mut self) -> GPO63_DOUT_W<GPO_DOUT60_63_SPEC> {
        GPO63_DOUT_W::new(self, 24)
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
#[doc = "SYS IOMUX CFG SAIF SYSCFG FMUX GPIO 60-63 DOUT\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`gpo_dout60_63::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`gpo_dout60_63::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct GPO_DOUT60_63_SPEC;
impl crate::RegisterSpec for GPO_DOUT60_63_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`gpo_dout60_63::R`](R) reader structure"]
impl crate::Readable for GPO_DOUT60_63_SPEC {}
#[doc = "`write(|w| ..)` method takes [`gpo_dout60_63::W`](W) writer structure"]
impl crate::Writable for GPO_DOUT60_63_SPEC {
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets gpo_dout60_63 to value 0x5f00_5d5e"]
impl crate::Resettable for GPO_DOUT60_63_SPEC {
    const RESET_VALUE: Self::Ux = 0x5f00_5d5e;
}
