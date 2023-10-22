#[doc = "Register `gpo_dout8_11` reader"]
pub type R = crate::R<GPO_DOUT8_11_SPEC>;
#[doc = "Register `gpo_dout8_11` writer"]
pub type W = crate::W<GPO_DOUT8_11_SPEC>;
#[doc = "Field `gpo8_dout` reader - The selected output signal for GPIO8. The register value indicates the selected GPIO output index signal index from GPIO output signal list 0-107. See Table 2-41: GPIO OEN List for SYS_IOMUX (on page 97) for more information."]
pub type GPO8_DOUT_R = crate::FieldReader;
#[doc = "Field `gpo8_dout` writer - The selected output signal for GPIO8. The register value indicates the selected GPIO output index signal index from GPIO output signal list 0-107. See Table 2-41: GPIO OEN List for SYS_IOMUX (on page 97) for more information."]
pub type GPO8_DOUT_W<'a, REG, const O: u8> = crate::FieldWriter<'a, REG, 7, O>;
#[doc = "Field `gpo9_dout` reader - The selected output signal for GPIO9. The register value indicates the selected GPIO output index signal index from GPIO output signal list 0-107. See Table 2-41: GPIO OEN List for SYS_IOMUX (on page 97) for more information."]
pub type GPO9_DOUT_R = crate::FieldReader;
#[doc = "Field `gpo9_dout` writer - The selected output signal for GPIO9. The register value indicates the selected GPIO output index signal index from GPIO output signal list 0-107. See Table 2-41: GPIO OEN List for SYS_IOMUX (on page 97) for more information."]
pub type GPO9_DOUT_W<'a, REG, const O: u8> = crate::FieldWriter<'a, REG, 7, O>;
#[doc = "Field `gpo10_dout` reader - The selected output signal for GPIO10. The register value indicates the selected GPIO output index signal index from GPIO output signal list 0-107. See Table 2-41: GPIO OEN List for SYS_IOMUX (on page 97) for more information."]
pub type GPO10_DOUT_R = crate::FieldReader;
#[doc = "Field `gpo10_dout` writer - The selected output signal for GPIO10. The register value indicates the selected GPIO output index signal index from GPIO output signal list 0-107. See Table 2-41: GPIO OEN List for SYS_IOMUX (on page 97) for more information."]
pub type GPO10_DOUT_W<'a, REG, const O: u8> = crate::FieldWriter<'a, REG, 7, O>;
#[doc = "Field `gpo11_dout` reader - The selected output signal for GPIO11. The register value indicates the selected GPIO output index signal index from GPIO output signal list 0-107. See Table 2-41: GPIO OEN List for SYS_IOMUX (on page 97) for more information."]
pub type GPO11_DOUT_R = crate::FieldReader;
#[doc = "Field `gpo11_dout` writer - The selected output signal for GPIO11. The register value indicates the selected GPIO output index signal index from GPIO output signal list 0-107. See Table 2-41: GPIO OEN List for SYS_IOMUX (on page 97) for more information."]
pub type GPO11_DOUT_W<'a, REG, const O: u8> = crate::FieldWriter<'a, REG, 7, O>;
impl R {
    #[doc = "Bits 0:6 - The selected output signal for GPIO8. The register value indicates the selected GPIO output index signal index from GPIO output signal list 0-107. See Table 2-41: GPIO OEN List for SYS_IOMUX (on page 97) for more information."]
    #[inline(always)]
    pub fn gpo8_dout(&self) -> GPO8_DOUT_R {
        GPO8_DOUT_R::new((self.bits & 0x7f) as u8)
    }
    #[doc = "Bits 8:14 - The selected output signal for GPIO9. The register value indicates the selected GPIO output index signal index from GPIO output signal list 0-107. See Table 2-41: GPIO OEN List for SYS_IOMUX (on page 97) for more information."]
    #[inline(always)]
    pub fn gpo9_dout(&self) -> GPO9_DOUT_R {
        GPO9_DOUT_R::new(((self.bits >> 8) & 0x7f) as u8)
    }
    #[doc = "Bits 16:22 - The selected output signal for GPIO10. The register value indicates the selected GPIO output index signal index from GPIO output signal list 0-107. See Table 2-41: GPIO OEN List for SYS_IOMUX (on page 97) for more information."]
    #[inline(always)]
    pub fn gpo10_dout(&self) -> GPO10_DOUT_R {
        GPO10_DOUT_R::new(((self.bits >> 16) & 0x7f) as u8)
    }
    #[doc = "Bits 24:30 - The selected output signal for GPIO11. The register value indicates the selected GPIO output index signal index from GPIO output signal list 0-107. See Table 2-41: GPIO OEN List for SYS_IOMUX (on page 97) for more information."]
    #[inline(always)]
    pub fn gpo11_dout(&self) -> GPO11_DOUT_R {
        GPO11_DOUT_R::new(((self.bits >> 24) & 0x7f) as u8)
    }
}
impl W {
    #[doc = "Bits 0:6 - The selected output signal for GPIO8. The register value indicates the selected GPIO output index signal index from GPIO output signal list 0-107. See Table 2-41: GPIO OEN List for SYS_IOMUX (on page 97) for more information."]
    #[inline(always)]
    #[must_use]
    pub fn gpo8_dout(&mut self) -> GPO8_DOUT_W<GPO_DOUT8_11_SPEC, 0> {
        GPO8_DOUT_W::new(self)
    }
    #[doc = "Bits 8:14 - The selected output signal for GPIO9. The register value indicates the selected GPIO output index signal index from GPIO output signal list 0-107. See Table 2-41: GPIO OEN List for SYS_IOMUX (on page 97) for more information."]
    #[inline(always)]
    #[must_use]
    pub fn gpo9_dout(&mut self) -> GPO9_DOUT_W<GPO_DOUT8_11_SPEC, 8> {
        GPO9_DOUT_W::new(self)
    }
    #[doc = "Bits 16:22 - The selected output signal for GPIO10. The register value indicates the selected GPIO output index signal index from GPIO output signal list 0-107. See Table 2-41: GPIO OEN List for SYS_IOMUX (on page 97) for more information."]
    #[inline(always)]
    #[must_use]
    pub fn gpo10_dout(&mut self) -> GPO10_DOUT_W<GPO_DOUT8_11_SPEC, 16> {
        GPO10_DOUT_W::new(self)
    }
    #[doc = "Bits 24:30 - The selected output signal for GPIO11. The register value indicates the selected GPIO output index signal index from GPIO output signal list 0-107. See Table 2-41: GPIO OEN List for SYS_IOMUX (on page 97) for more information."]
    #[inline(always)]
    #[must_use]
    pub fn gpo11_dout(&mut self) -> GPO11_DOUT_W<GPO_DOUT8_11_SPEC, 24> {
        GPO11_DOUT_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.bits = bits;
        self
    }
}
#[doc = "SYS IOMUX CFG SAIF SYSCFG FMUX GPIO 8-11 DOUT\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`gpo_dout8_11::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`gpo_dout8_11::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct GPO_DOUT8_11_SPEC;
impl crate::RegisterSpec for GPO_DOUT8_11_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`gpo_dout8_11::R`](R) reader structure"]
impl crate::Readable for GPO_DOUT8_11_SPEC {}
#[doc = "`write(|w| ..)` method takes [`gpo_dout8_11::W`](W) writer structure"]
impl crate::Writable for GPO_DOUT8_11_SPEC {
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets gpo_dout8_11 to value 0x1500_0000"]
impl crate::Resettable for GPO_DOUT8_11_SPEC {
    const RESET_VALUE: Self::Ux = 0x1500_0000;
}
