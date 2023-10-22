#[doc = "Register `gpo_dout56_59` reader"]
pub type R = crate::R<GPO_DOUT56_59_SPEC>;
#[doc = "Register `gpo_dout56_59` writer"]
pub type W = crate::W<GPO_DOUT56_59_SPEC>;
#[doc = "Field `gpo56_dout` reader - The selected output signal for GPIO56. The register value indicates the selected GPIO output index signal index from GPIO output signal list 0-107. See Table 2-41: GPIO OEN List for SYS_IOMUX (on page 97) for more information."]
pub type GPO56_DOUT_R = crate::FieldReader;
#[doc = "Field `gpo56_dout` writer - The selected output signal for GPIO56. The register value indicates the selected GPIO output index signal index from GPIO output signal list 0-107. See Table 2-41: GPIO OEN List for SYS_IOMUX (on page 97) for more information."]
pub type GPO56_DOUT_W<'a, REG, const O: u8> = crate::FieldWriter<'a, REG, 7, O>;
#[doc = "Field `gpo57_dout` reader - The selected output signal for GPIO57. The register value indicates the selected GPIO output index signal index from GPIO output signal list 0-107. See Table 2-41: GPIO OEN List for SYS_IOMUX (on page 97) for more information."]
pub type GPO57_DOUT_R = crate::FieldReader;
#[doc = "Field `gpo57_dout` writer - The selected output signal for GPIO57. The register value indicates the selected GPIO output index signal index from GPIO output signal list 0-107. See Table 2-41: GPIO OEN List for SYS_IOMUX (on page 97) for more information."]
pub type GPO57_DOUT_W<'a, REG, const O: u8> = crate::FieldWriter<'a, REG, 7, O>;
#[doc = "Field `gpo58_dout` reader - The selected output signal for GPIO58. The register value indicates the selected GPIO output index signal index from GPIO output signal list 0-107. See Table 2-41: GPIO OEN List for SYS_IOMUX (on page 97) for more information."]
pub type GPO58_DOUT_R = crate::FieldReader;
#[doc = "Field `gpo58_dout` writer - The selected output signal for GPIO58. The register value indicates the selected GPIO output index signal index from GPIO output signal list 0-107. See Table 2-41: GPIO OEN List for SYS_IOMUX (on page 97) for more information."]
pub type GPO58_DOUT_W<'a, REG, const O: u8> = crate::FieldWriter<'a, REG, 7, O>;
#[doc = "Field `gpo59_dout` reader - The selected output signal for GPIO59. The register value indicates the selected GPIO output index signal index from GPIO output signal list 0-107. See Table 2-41: GPIO OEN List for SYS_IOMUX (on page 97) for more information."]
pub type GPO59_DOUT_R = crate::FieldReader;
#[doc = "Field `gpo59_dout` writer - The selected output signal for GPIO59. The register value indicates the selected GPIO output index signal index from GPIO output signal list 0-107. See Table 2-41: GPIO OEN List for SYS_IOMUX (on page 97) for more information."]
pub type GPO59_DOUT_W<'a, REG, const O: u8> = crate::FieldWriter<'a, REG, 7, O>;
impl R {
    #[doc = "Bits 0:6 - The selected output signal for GPIO56. The register value indicates the selected GPIO output index signal index from GPIO output signal list 0-107. See Table 2-41: GPIO OEN List for SYS_IOMUX (on page 97) for more information."]
    #[inline(always)]
    pub fn gpo56_dout(&self) -> GPO56_DOUT_R {
        GPO56_DOUT_R::new((self.bits & 0x7f) as u8)
    }
    #[doc = "Bits 8:14 - The selected output signal for GPIO57. The register value indicates the selected GPIO output index signal index from GPIO output signal list 0-107. See Table 2-41: GPIO OEN List for SYS_IOMUX (on page 97) for more information."]
    #[inline(always)]
    pub fn gpo57_dout(&self) -> GPO57_DOUT_R {
        GPO57_DOUT_R::new(((self.bits >> 8) & 0x7f) as u8)
    }
    #[doc = "Bits 16:22 - The selected output signal for GPIO58. The register value indicates the selected GPIO output index signal index from GPIO output signal list 0-107. See Table 2-41: GPIO OEN List for SYS_IOMUX (on page 97) for more information."]
    #[inline(always)]
    pub fn gpo58_dout(&self) -> GPO58_DOUT_R {
        GPO58_DOUT_R::new(((self.bits >> 16) & 0x7f) as u8)
    }
    #[doc = "Bits 24:30 - The selected output signal for GPIO59. The register value indicates the selected GPIO output index signal index from GPIO output signal list 0-107. See Table 2-41: GPIO OEN List for SYS_IOMUX (on page 97) for more information."]
    #[inline(always)]
    pub fn gpo59_dout(&self) -> GPO59_DOUT_R {
        GPO59_DOUT_R::new(((self.bits >> 24) & 0x7f) as u8)
    }
}
impl W {
    #[doc = "Bits 0:6 - The selected output signal for GPIO56. The register value indicates the selected GPIO output index signal index from GPIO output signal list 0-107. See Table 2-41: GPIO OEN List for SYS_IOMUX (on page 97) for more information."]
    #[inline(always)]
    #[must_use]
    pub fn gpo56_dout(&mut self) -> GPO56_DOUT_W<GPO_DOUT56_59_SPEC, 0> {
        GPO56_DOUT_W::new(self)
    }
    #[doc = "Bits 8:14 - The selected output signal for GPIO57. The register value indicates the selected GPIO output index signal index from GPIO output signal list 0-107. See Table 2-41: GPIO OEN List for SYS_IOMUX (on page 97) for more information."]
    #[inline(always)]
    #[must_use]
    pub fn gpo57_dout(&mut self) -> GPO57_DOUT_W<GPO_DOUT56_59_SPEC, 8> {
        GPO57_DOUT_W::new(self)
    }
    #[doc = "Bits 16:22 - The selected output signal for GPIO58. The register value indicates the selected GPIO output index signal index from GPIO output signal list 0-107. See Table 2-41: GPIO OEN List for SYS_IOMUX (on page 97) for more information."]
    #[inline(always)]
    #[must_use]
    pub fn gpo58_dout(&mut self) -> GPO58_DOUT_W<GPO_DOUT56_59_SPEC, 16> {
        GPO58_DOUT_W::new(self)
    }
    #[doc = "Bits 24:30 - The selected output signal for GPIO59. The register value indicates the selected GPIO output index signal index from GPIO output signal list 0-107. See Table 2-41: GPIO OEN List for SYS_IOMUX (on page 97) for more information."]
    #[inline(always)]
    #[must_use]
    pub fn gpo59_dout(&mut self) -> GPO59_DOUT_W<GPO_DOUT56_59_SPEC, 24> {
        GPO59_DOUT_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.bits = bits;
        self
    }
}
#[doc = "SYS IOMUX CFG SAIF SYSCFG FMUX GPIO 56-59 DOUT\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`gpo_dout56_59::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`gpo_dout56_59::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct GPO_DOUT56_59_SPEC;
impl crate::RegisterSpec for GPO_DOUT56_59_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`gpo_dout56_59::R`](R) reader structure"]
impl crate::Readable for GPO_DOUT56_59_SPEC {}
#[doc = "`write(|w| ..)` method takes [`gpo_dout56_59::W`](W) writer structure"]
impl crate::Writable for GPO_DOUT56_59_SPEC {
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets gpo_dout56_59 to value 0x5800_5657"]
impl crate::Resettable for GPO_DOUT56_59_SPEC {
    const RESET_VALUE: Self::Ux = 0x5800_5657;
}
