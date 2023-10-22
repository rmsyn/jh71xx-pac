#[doc = "Register `gpo_dout48_51` reader"]
pub type R = crate::R<GPO_DOUT48_51_SPEC>;
#[doc = "Register `gpo_dout48_51` writer"]
pub type W = crate::W<GPO_DOUT48_51_SPEC>;
#[doc = "Field `gpo48_dout` reader - The selected output signal for GPIO48. The register value indicates the selected GPIO output index signal index from GPIO output signal list 0-107. See Table 2-41: GPIO OEN List for SYS_IOMUX (on page 97) for more information."]
pub type GPO48_DOUT_R = crate::FieldReader;
#[doc = "Field `gpo48_dout` writer - The selected output signal for GPIO48. The register value indicates the selected GPIO output index signal index from GPIO output signal list 0-107. See Table 2-41: GPIO OEN List for SYS_IOMUX (on page 97) for more information."]
pub type GPO48_DOUT_W<'a, REG, const O: u8> = crate::FieldWriter<'a, REG, 7, O>;
#[doc = "Field `gpo49_dout` reader - The selected output signal for GPIO49. The register value indicates the selected GPIO output index signal index from GPIO output signal list 0-107. See Table 2-41: GPIO OEN List for SYS_IOMUX (on page 97) for more information."]
pub type GPO49_DOUT_R = crate::FieldReader;
#[doc = "Field `gpo49_dout` writer - The selected output signal for GPIO49. The register value indicates the selected GPIO output index signal index from GPIO output signal list 0-107. See Table 2-41: GPIO OEN List for SYS_IOMUX (on page 97) for more information."]
pub type GPO49_DOUT_W<'a, REG, const O: u8> = crate::FieldWriter<'a, REG, 7, O>;
#[doc = "Field `gpo50_dout` reader - The selected output signal for GPIO50. The register value indicates the selected GPIO output index signal index from GPIO output signal list 0-107. See Table 2-41: GPIO OEN List for SYS_IOMUX (on page 97) for more information."]
pub type GPO50_DOUT_R = crate::FieldReader;
#[doc = "Field `gpo50_dout` writer - The selected output signal for GPIO50. The register value indicates the selected GPIO output index signal index from GPIO output signal list 0-107. See Table 2-41: GPIO OEN List for SYS_IOMUX (on page 97) for more information."]
pub type GPO50_DOUT_W<'a, REG, const O: u8> = crate::FieldWriter<'a, REG, 7, O>;
#[doc = "Field `gpo51_dout` reader - The selected output signal for GPIO51. The register value indicates the selected GPIO output index signal index from GPIO output signal list 0-107. See Table 2-41: GPIO OEN List for SYS_IOMUX (on page 97) for more information."]
pub type GPO51_DOUT_R = crate::FieldReader;
#[doc = "Field `gpo51_dout` writer - The selected output signal for GPIO51. The register value indicates the selected GPIO output index signal index from GPIO output signal list 0-107. See Table 2-41: GPIO OEN List for SYS_IOMUX (on page 97) for more information."]
pub type GPO51_DOUT_W<'a, REG, const O: u8> = crate::FieldWriter<'a, REG, 7, O>;
impl R {
    #[doc = "Bits 0:6 - The selected output signal for GPIO48. The register value indicates the selected GPIO output index signal index from GPIO output signal list 0-107. See Table 2-41: GPIO OEN List for SYS_IOMUX (on page 97) for more information."]
    #[inline(always)]
    pub fn gpo48_dout(&self) -> GPO48_DOUT_R {
        GPO48_DOUT_R::new((self.bits & 0x7f) as u8)
    }
    #[doc = "Bits 8:14 - The selected output signal for GPIO49. The register value indicates the selected GPIO output index signal index from GPIO output signal list 0-107. See Table 2-41: GPIO OEN List for SYS_IOMUX (on page 97) for more information."]
    #[inline(always)]
    pub fn gpo49_dout(&self) -> GPO49_DOUT_R {
        GPO49_DOUT_R::new(((self.bits >> 8) & 0x7f) as u8)
    }
    #[doc = "Bits 16:22 - The selected output signal for GPIO50. The register value indicates the selected GPIO output index signal index from GPIO output signal list 0-107. See Table 2-41: GPIO OEN List for SYS_IOMUX (on page 97) for more information."]
    #[inline(always)]
    pub fn gpo50_dout(&self) -> GPO50_DOUT_R {
        GPO50_DOUT_R::new(((self.bits >> 16) & 0x7f) as u8)
    }
    #[doc = "Bits 24:30 - The selected output signal for GPIO51. The register value indicates the selected GPIO output index signal index from GPIO output signal list 0-107. See Table 2-41: GPIO OEN List for SYS_IOMUX (on page 97) for more information."]
    #[inline(always)]
    pub fn gpo51_dout(&self) -> GPO51_DOUT_R {
        GPO51_DOUT_R::new(((self.bits >> 24) & 0x7f) as u8)
    }
}
impl W {
    #[doc = "Bits 0:6 - The selected output signal for GPIO48. The register value indicates the selected GPIO output index signal index from GPIO output signal list 0-107. See Table 2-41: GPIO OEN List for SYS_IOMUX (on page 97) for more information."]
    #[inline(always)]
    #[must_use]
    pub fn gpo48_dout(&mut self) -> GPO48_DOUT_W<GPO_DOUT48_51_SPEC, 0> {
        GPO48_DOUT_W::new(self)
    }
    #[doc = "Bits 8:14 - The selected output signal for GPIO49. The register value indicates the selected GPIO output index signal index from GPIO output signal list 0-107. See Table 2-41: GPIO OEN List for SYS_IOMUX (on page 97) for more information."]
    #[inline(always)]
    #[must_use]
    pub fn gpo49_dout(&mut self) -> GPO49_DOUT_W<GPO_DOUT48_51_SPEC, 8> {
        GPO49_DOUT_W::new(self)
    }
    #[doc = "Bits 16:22 - The selected output signal for GPIO50. The register value indicates the selected GPIO output index signal index from GPIO output signal list 0-107. See Table 2-41: GPIO OEN List for SYS_IOMUX (on page 97) for more information."]
    #[inline(always)]
    #[must_use]
    pub fn gpo50_dout(&mut self) -> GPO50_DOUT_W<GPO_DOUT48_51_SPEC, 16> {
        GPO50_DOUT_W::new(self)
    }
    #[doc = "Bits 24:30 - The selected output signal for GPIO51. The register value indicates the selected GPIO output index signal index from GPIO output signal list 0-107. See Table 2-41: GPIO OEN List for SYS_IOMUX (on page 97) for more information."]
    #[inline(always)]
    #[must_use]
    pub fn gpo51_dout(&mut self) -> GPO51_DOUT_W<GPO_DOUT48_51_SPEC, 24> {
        GPO51_DOUT_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.bits = bits;
        self
    }
}
#[doc = "SYS IOMUX CFG SAIF SYSCFG FMUX GPIO 48-51 DOUT\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`gpo_dout48_51::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`gpo_dout48_51::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct GPO_DOUT48_51_SPEC;
impl crate::RegisterSpec for GPO_DOUT48_51_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`gpo_dout48_51::R`](R) reader structure"]
impl crate::Readable for GPO_DOUT48_51_SPEC {}
#[doc = "`write(|w| ..)` method takes [`gpo_dout48_51::W`](W) writer structure"]
impl crate::Writable for GPO_DOUT48_51_SPEC {
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets gpo_dout48_51 to value 0x2000_1e1f"]
impl crate::Resettable for GPO_DOUT48_51_SPEC {
    const RESET_VALUE: Self::Ux = 0x2000_1e1f;
}
