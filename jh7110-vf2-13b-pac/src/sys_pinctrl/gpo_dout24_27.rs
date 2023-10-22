#[doc = "Register `gpo_dout24_27` reader"]
pub type R = crate::R<GPO_DOUT24_27_SPEC>;
#[doc = "Register `gpo_dout24_27` writer"]
pub type W = crate::W<GPO_DOUT24_27_SPEC>;
#[doc = "Field `gpo24_dout` reader - The selected output signal for GPIO24. The register value indicates the selected GPIO output index signal index from GPIO output signal list 0-107. See Table 2-41: GPIO OEN List for SYS_IOMUX (on page 97) for more information."]
pub type GPO24_DOUT_R = crate::FieldReader;
#[doc = "Field `gpo24_dout` writer - The selected output signal for GPIO24. The register value indicates the selected GPIO output index signal index from GPIO output signal list 0-107. See Table 2-41: GPIO OEN List for SYS_IOMUX (on page 97) for more information."]
pub type GPO24_DOUT_W<'a, REG, const O: u8> = crate::FieldWriter<'a, REG, 7, O>;
#[doc = "Field `gpo25_dout` reader - The selected output signal for GPIO25. The register value indicates the selected GPIO output index signal index from GPIO output signal list 0-107. See Table 2-41: GPIO OEN List for SYS_IOMUX (on page 97) for more information."]
pub type GPO25_DOUT_R = crate::FieldReader;
#[doc = "Field `gpo25_dout` writer - The selected output signal for GPIO25. The register value indicates the selected GPIO output index signal index from GPIO output signal list 0-107. See Table 2-41: GPIO OEN List for SYS_IOMUX (on page 97) for more information."]
pub type GPO25_DOUT_W<'a, REG, const O: u8> = crate::FieldWriter<'a, REG, 7, O>;
#[doc = "Field `gpo26_dout` reader - The selected output signal for GPIO26. The register value indicates the selected GPIO output index signal index from GPIO output signal list 0-107. See Table 2-41: GPIO OEN List for SYS_IOMUX (on page 97) for more information."]
pub type GPO26_DOUT_R = crate::FieldReader;
#[doc = "Field `gpo26_dout` writer - The selected output signal for GPIO26. The register value indicates the selected GPIO output index signal index from GPIO output signal list 0-107. See Table 2-41: GPIO OEN List for SYS_IOMUX (on page 97) for more information."]
pub type GPO26_DOUT_W<'a, REG, const O: u8> = crate::FieldWriter<'a, REG, 7, O>;
#[doc = "Field `gpo27_dout` reader - The selected output signal for GPIO27. The register value indicates the selected GPIO output index signal index from GPIO output signal list 0-107. See Table 2-41: GPIO OEN List for SYS_IOMUX (on page 97) for more information."]
pub type GPO27_DOUT_R = crate::FieldReader;
#[doc = "Field `gpo27_dout` writer - The selected output signal for GPIO27. The register value indicates the selected GPIO output index signal index from GPIO output signal list 0-107. See Table 2-41: GPIO OEN List for SYS_IOMUX (on page 97) for more information."]
pub type GPO27_DOUT_W<'a, REG, const O: u8> = crate::FieldWriter<'a, REG, 7, O>;
impl R {
    #[doc = "Bits 0:6 - The selected output signal for GPIO24. The register value indicates the selected GPIO output index signal index from GPIO output signal list 0-107. See Table 2-41: GPIO OEN List for SYS_IOMUX (on page 97) for more information."]
    #[inline(always)]
    pub fn gpo24_dout(&self) -> GPO24_DOUT_R {
        GPO24_DOUT_R::new((self.bits & 0x7f) as u8)
    }
    #[doc = "Bits 8:14 - The selected output signal for GPIO25. The register value indicates the selected GPIO output index signal index from GPIO output signal list 0-107. See Table 2-41: GPIO OEN List for SYS_IOMUX (on page 97) for more information."]
    #[inline(always)]
    pub fn gpo25_dout(&self) -> GPO25_DOUT_R {
        GPO25_DOUT_R::new(((self.bits >> 8) & 0x7f) as u8)
    }
    #[doc = "Bits 16:22 - The selected output signal for GPIO26. The register value indicates the selected GPIO output index signal index from GPIO output signal list 0-107. See Table 2-41: GPIO OEN List for SYS_IOMUX (on page 97) for more information."]
    #[inline(always)]
    pub fn gpo26_dout(&self) -> GPO26_DOUT_R {
        GPO26_DOUT_R::new(((self.bits >> 16) & 0x7f) as u8)
    }
    #[doc = "Bits 24:30 - The selected output signal for GPIO27. The register value indicates the selected GPIO output index signal index from GPIO output signal list 0-107. See Table 2-41: GPIO OEN List for SYS_IOMUX (on page 97) for more information."]
    #[inline(always)]
    pub fn gpo27_dout(&self) -> GPO27_DOUT_R {
        GPO27_DOUT_R::new(((self.bits >> 24) & 0x7f) as u8)
    }
}
impl W {
    #[doc = "Bits 0:6 - The selected output signal for GPIO24. The register value indicates the selected GPIO output index signal index from GPIO output signal list 0-107. See Table 2-41: GPIO OEN List for SYS_IOMUX (on page 97) for more information."]
    #[inline(always)]
    #[must_use]
    pub fn gpo24_dout(&mut self) -> GPO24_DOUT_W<GPO_DOUT24_27_SPEC, 0> {
        GPO24_DOUT_W::new(self)
    }
    #[doc = "Bits 8:14 - The selected output signal for GPIO25. The register value indicates the selected GPIO output index signal index from GPIO output signal list 0-107. See Table 2-41: GPIO OEN List for SYS_IOMUX (on page 97) for more information."]
    #[inline(always)]
    #[must_use]
    pub fn gpo25_dout(&mut self) -> GPO25_DOUT_W<GPO_DOUT24_27_SPEC, 8> {
        GPO25_DOUT_W::new(self)
    }
    #[doc = "Bits 16:22 - The selected output signal for GPIO26. The register value indicates the selected GPIO output index signal index from GPIO output signal list 0-107. See Table 2-41: GPIO OEN List for SYS_IOMUX (on page 97) for more information."]
    #[inline(always)]
    #[must_use]
    pub fn gpo26_dout(&mut self) -> GPO26_DOUT_W<GPO_DOUT24_27_SPEC, 16> {
        GPO26_DOUT_W::new(self)
    }
    #[doc = "Bits 24:30 - The selected output signal for GPIO27. The register value indicates the selected GPIO output index signal index from GPIO output signal list 0-107. See Table 2-41: GPIO OEN List for SYS_IOMUX (on page 97) for more information."]
    #[inline(always)]
    #[must_use]
    pub fn gpo27_dout(&mut self) -> GPO27_DOUT_W<GPO_DOUT24_27_SPEC, 24> {
        GPO27_DOUT_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.bits = bits;
        self
    }
}
#[doc = "SYS IOMUX CFG SAIF SYSCFG FMUX GPIO 24-27 DOUT\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`gpo_dout24_27::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`gpo_dout24_27::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct GPO_DOUT24_27_SPEC;
impl crate::RegisterSpec for GPO_DOUT24_27_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`gpo_dout24_27::R`](R) reader structure"]
impl crate::Readable for GPO_DOUT24_27_SPEC {}
#[doc = "`write(|w| ..)` method takes [`gpo_dout24_27::W`](W) writer structure"]
impl crate::Writable for GPO_DOUT24_27_SPEC {
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets gpo_dout24_27 to value 0"]
impl crate::Resettable for GPO_DOUT24_27_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
