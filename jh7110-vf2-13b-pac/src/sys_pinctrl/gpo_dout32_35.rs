#[doc = "Register `gpo_dout32_35` reader"]
pub type R = crate::R<GPO_DOUT32_35_SPEC>;
#[doc = "Register `gpo_dout32_35` writer"]
pub type W = crate::W<GPO_DOUT32_35_SPEC>;
#[doc = "Field `gpo32_dout` reader - The selected output signal for GPIO32. The register value indicates the selected GPIO output index signal index from GPIO output signal list 0-107. See Table 2-41: GPIO OEN List for SYS_IOMUX (on page 97) for more information."]
pub type GPO32_DOUT_R = crate::FieldReader;
#[doc = "Field `gpo32_dout` writer - The selected output signal for GPIO32. The register value indicates the selected GPIO output index signal index from GPIO output signal list 0-107. See Table 2-41: GPIO OEN List for SYS_IOMUX (on page 97) for more information."]
pub type GPO32_DOUT_W<'a, REG, const O: u8> = crate::FieldWriter<'a, REG, 7, O>;
#[doc = "Field `gpo33_dout` reader - The selected output signal for GPIO33. The register value indicates the selected GPIO output index signal index from GPIO output signal list 0-107. See Table 2-41: GPIO OEN List for SYS_IOMUX (on page 97) for more information."]
pub type GPO33_DOUT_R = crate::FieldReader;
#[doc = "Field `gpo33_dout` writer - The selected output signal for GPIO33. The register value indicates the selected GPIO output index signal index from GPIO output signal list 0-107. See Table 2-41: GPIO OEN List for SYS_IOMUX (on page 97) for more information."]
pub type GPO33_DOUT_W<'a, REG, const O: u8> = crate::FieldWriter<'a, REG, 7, O>;
#[doc = "Field `gpo34_dout` reader - The selected output signal for GPIO34. The register value indicates the selected GPIO output index signal index from GPIO output signal list 0-107. See Table 2-41: GPIO OEN List for SYS_IOMUX (on page 97) for more information."]
pub type GPO34_DOUT_R = crate::FieldReader;
#[doc = "Field `gpo34_dout` writer - The selected output signal for GPIO34. The register value indicates the selected GPIO output index signal index from GPIO output signal list 0-107. See Table 2-41: GPIO OEN List for SYS_IOMUX (on page 97) for more information."]
pub type GPO34_DOUT_W<'a, REG, const O: u8> = crate::FieldWriter<'a, REG, 7, O>;
#[doc = "Field `gpo35_dout` reader - The selected output signal for GPIO35. The register value indicates the selected GPIO output index signal index from GPIO output signal list 0-107. See Table 2-41: GPIO OEN List for SYS_IOMUX (on page 97) for more information."]
pub type GPO35_DOUT_R = crate::FieldReader;
#[doc = "Field `gpo35_dout` writer - The selected output signal for GPIO35. The register value indicates the selected GPIO output index signal index from GPIO output signal list 0-107. See Table 2-41: GPIO OEN List for SYS_IOMUX (on page 97) for more information."]
pub type GPO35_DOUT_W<'a, REG, const O: u8> = crate::FieldWriter<'a, REG, 7, O>;
impl R {
    #[doc = "Bits 0:6 - The selected output signal for GPIO32. The register value indicates the selected GPIO output index signal index from GPIO output signal list 0-107. See Table 2-41: GPIO OEN List for SYS_IOMUX (on page 97) for more information."]
    #[inline(always)]
    pub fn gpo32_dout(&self) -> GPO32_DOUT_R {
        GPO32_DOUT_R::new((self.bits & 0x7f) as u8)
    }
    #[doc = "Bits 8:14 - The selected output signal for GPIO33. The register value indicates the selected GPIO output index signal index from GPIO output signal list 0-107. See Table 2-41: GPIO OEN List for SYS_IOMUX (on page 97) for more information."]
    #[inline(always)]
    pub fn gpo33_dout(&self) -> GPO33_DOUT_R {
        GPO33_DOUT_R::new(((self.bits >> 8) & 0x7f) as u8)
    }
    #[doc = "Bits 16:22 - The selected output signal for GPIO34. The register value indicates the selected GPIO output index signal index from GPIO output signal list 0-107. See Table 2-41: GPIO OEN List for SYS_IOMUX (on page 97) for more information."]
    #[inline(always)]
    pub fn gpo34_dout(&self) -> GPO34_DOUT_R {
        GPO34_DOUT_R::new(((self.bits >> 16) & 0x7f) as u8)
    }
    #[doc = "Bits 24:30 - The selected output signal for GPIO35. The register value indicates the selected GPIO output index signal index from GPIO output signal list 0-107. See Table 2-41: GPIO OEN List for SYS_IOMUX (on page 97) for more information."]
    #[inline(always)]
    pub fn gpo35_dout(&self) -> GPO35_DOUT_R {
        GPO35_DOUT_R::new(((self.bits >> 24) & 0x7f) as u8)
    }
}
impl W {
    #[doc = "Bits 0:6 - The selected output signal for GPIO32. The register value indicates the selected GPIO output index signal index from GPIO output signal list 0-107. See Table 2-41: GPIO OEN List for SYS_IOMUX (on page 97) for more information."]
    #[inline(always)]
    #[must_use]
    pub fn gpo32_dout(&mut self) -> GPO32_DOUT_W<GPO_DOUT32_35_SPEC, 0> {
        GPO32_DOUT_W::new(self)
    }
    #[doc = "Bits 8:14 - The selected output signal for GPIO33. The register value indicates the selected GPIO output index signal index from GPIO output signal list 0-107. See Table 2-41: GPIO OEN List for SYS_IOMUX (on page 97) for more information."]
    #[inline(always)]
    #[must_use]
    pub fn gpo33_dout(&mut self) -> GPO33_DOUT_W<GPO_DOUT32_35_SPEC, 8> {
        GPO33_DOUT_W::new(self)
    }
    #[doc = "Bits 16:22 - The selected output signal for GPIO34. The register value indicates the selected GPIO output index signal index from GPIO output signal list 0-107. See Table 2-41: GPIO OEN List for SYS_IOMUX (on page 97) for more information."]
    #[inline(always)]
    #[must_use]
    pub fn gpo34_dout(&mut self) -> GPO34_DOUT_W<GPO_DOUT32_35_SPEC, 16> {
        GPO34_DOUT_W::new(self)
    }
    #[doc = "Bits 24:30 - The selected output signal for GPIO35. The register value indicates the selected GPIO output index signal index from GPIO output signal list 0-107. See Table 2-41: GPIO OEN List for SYS_IOMUX (on page 97) for more information."]
    #[inline(always)]
    #[must_use]
    pub fn gpo35_dout(&mut self) -> GPO35_DOUT_W<GPO_DOUT32_35_SPEC, 24> {
        GPO35_DOUT_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.bits = bits;
        self
    }
}
#[doc = "SYS IOMUX CFG SAIF SYSCFG FMUX GPIO 32-35 DOUT\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`gpo_dout32_35::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`gpo_dout32_35::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct GPO_DOUT32_35_SPEC;
impl crate::RegisterSpec for GPO_DOUT32_35_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`gpo_dout32_35::R`](R) reader structure"]
impl crate::Readable for GPO_DOUT32_35_SPEC {}
#[doc = "`write(|w| ..)` method takes [`gpo_dout32_35::W`](W) writer structure"]
impl crate::Writable for GPO_DOUT32_35_SPEC {
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets gpo_dout32_35 to value 0x0d00_0000"]
impl crate::Resettable for GPO_DOUT32_35_SPEC {
    const RESET_VALUE: Self::Ux = 0x0d00_0000;
}
