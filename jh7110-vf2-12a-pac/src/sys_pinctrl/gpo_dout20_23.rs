#[doc = "Register `gpo_dout20_23` reader"]
pub type R = crate::R<GPO_DOUT20_23_SPEC>;
#[doc = "Register `gpo_dout20_23` writer"]
pub type W = crate::W<GPO_DOUT20_23_SPEC>;
#[doc = "Field `gpo20_dout` reader - The selected output signal for GPIO20. The register value indicates the selected GPIO output index signal index from GPIO output signal list 0-107. See Table 2-41: GPIO OEN List for SYS_IOMUX (on page 97) for more information."]
pub type GPO20_DOUT_R = crate::FieldReader;
#[doc = "Field `gpo20_dout` writer - The selected output signal for GPIO20. The register value indicates the selected GPIO output index signal index from GPIO output signal list 0-107. See Table 2-41: GPIO OEN List for SYS_IOMUX (on page 97) for more information."]
pub type GPO20_DOUT_W<'a, REG> = crate::FieldWriter<'a, REG, 7>;
#[doc = "Field `gpo21_dout` reader - The selected output signal for GPIO21. The register value indicates the selected GPIO output index signal index from GPIO output signal list 0-107. See Table 2-41: GPIO OEN List for SYS_IOMUX (on page 97) for more information."]
pub type GPO21_DOUT_R = crate::FieldReader;
#[doc = "Field `gpo21_dout` writer - The selected output signal for GPIO21. The register value indicates the selected GPIO output index signal index from GPIO output signal list 0-107. See Table 2-41: GPIO OEN List for SYS_IOMUX (on page 97) for more information."]
pub type GPO21_DOUT_W<'a, REG> = crate::FieldWriter<'a, REG, 7>;
#[doc = "Field `gpo22_dout` reader - The selected output signal for GPIO22. The register value indicates the selected GPIO output index signal index from GPIO output signal list 0-107. See Table 2-41: GPIO OEN List for SYS_IOMUX (on page 97) for more information."]
pub type GPO22_DOUT_R = crate::FieldReader;
#[doc = "Field `gpo22_dout` writer - The selected output signal for GPIO22. The register value indicates the selected GPIO output index signal index from GPIO output signal list 0-107. See Table 2-41: GPIO OEN List for SYS_IOMUX (on page 97) for more information."]
pub type GPO22_DOUT_W<'a, REG> = crate::FieldWriter<'a, REG, 7>;
#[doc = "Field `gpo23_dout` reader - The selected output signal for GPIO23. The register value indicates the selected GPIO output index signal index from GPIO output signal list 0-107. See Table 2-41: GPIO OEN List for SYS_IOMUX (on page 97) for more information."]
pub type GPO23_DOUT_R = crate::FieldReader;
#[doc = "Field `gpo23_dout` writer - The selected output signal for GPIO23. The register value indicates the selected GPIO output index signal index from GPIO output signal list 0-107. See Table 2-41: GPIO OEN List for SYS_IOMUX (on page 97) for more information."]
pub type GPO23_DOUT_W<'a, REG> = crate::FieldWriter<'a, REG, 7>;
impl R {
    #[doc = "Bits 0:6 - The selected output signal for GPIO20. The register value indicates the selected GPIO output index signal index from GPIO output signal list 0-107. See Table 2-41: GPIO OEN List for SYS_IOMUX (on page 97) for more information."]
    #[inline(always)]
    pub fn gpo20_dout(&self) -> GPO20_DOUT_R {
        GPO20_DOUT_R::new((self.bits & 0x7f) as u8)
    }
    #[doc = "Bits 8:14 - The selected output signal for GPIO21. The register value indicates the selected GPIO output index signal index from GPIO output signal list 0-107. See Table 2-41: GPIO OEN List for SYS_IOMUX (on page 97) for more information."]
    #[inline(always)]
    pub fn gpo21_dout(&self) -> GPO21_DOUT_R {
        GPO21_DOUT_R::new(((self.bits >> 8) & 0x7f) as u8)
    }
    #[doc = "Bits 16:22 - The selected output signal for GPIO22. The register value indicates the selected GPIO output index signal index from GPIO output signal list 0-107. See Table 2-41: GPIO OEN List for SYS_IOMUX (on page 97) for more information."]
    #[inline(always)]
    pub fn gpo22_dout(&self) -> GPO22_DOUT_R {
        GPO22_DOUT_R::new(((self.bits >> 16) & 0x7f) as u8)
    }
    #[doc = "Bits 24:30 - The selected output signal for GPIO23. The register value indicates the selected GPIO output index signal index from GPIO output signal list 0-107. See Table 2-41: GPIO OEN List for SYS_IOMUX (on page 97) for more information."]
    #[inline(always)]
    pub fn gpo23_dout(&self) -> GPO23_DOUT_R {
        GPO23_DOUT_R::new(((self.bits >> 24) & 0x7f) as u8)
    }
}
impl W {
    #[doc = "Bits 0:6 - The selected output signal for GPIO20. The register value indicates the selected GPIO output index signal index from GPIO output signal list 0-107. See Table 2-41: GPIO OEN List for SYS_IOMUX (on page 97) for more information."]
    #[inline(always)]
    #[must_use]
    pub fn gpo20_dout(&mut self) -> GPO20_DOUT_W<GPO_DOUT20_23_SPEC> {
        GPO20_DOUT_W::new(self, 0)
    }
    #[doc = "Bits 8:14 - The selected output signal for GPIO21. The register value indicates the selected GPIO output index signal index from GPIO output signal list 0-107. See Table 2-41: GPIO OEN List for SYS_IOMUX (on page 97) for more information."]
    #[inline(always)]
    #[must_use]
    pub fn gpo21_dout(&mut self) -> GPO21_DOUT_W<GPO_DOUT20_23_SPEC> {
        GPO21_DOUT_W::new(self, 8)
    }
    #[doc = "Bits 16:22 - The selected output signal for GPIO22. The register value indicates the selected GPIO output index signal index from GPIO output signal list 0-107. See Table 2-41: GPIO OEN List for SYS_IOMUX (on page 97) for more information."]
    #[inline(always)]
    #[must_use]
    pub fn gpo22_dout(&mut self) -> GPO22_DOUT_W<GPO_DOUT20_23_SPEC> {
        GPO22_DOUT_W::new(self, 16)
    }
    #[doc = "Bits 24:30 - The selected output signal for GPIO23. The register value indicates the selected GPIO output index signal index from GPIO output signal list 0-107. See Table 2-41: GPIO OEN List for SYS_IOMUX (on page 97) for more information."]
    #[inline(always)]
    #[must_use]
    pub fn gpo23_dout(&mut self) -> GPO23_DOUT_W<GPO_DOUT20_23_SPEC> {
        GPO23_DOUT_W::new(self, 24)
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
#[doc = "SYS IOMUX CFG SAIF SYSCFG FMUX GPIO 20-23 DOUT\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`gpo_dout20_23::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`gpo_dout20_23::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct GPO_DOUT20_23_SPEC;
impl crate::RegisterSpec for GPO_DOUT20_23_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`gpo_dout20_23::R`](R) reader structure"]
impl crate::Readable for GPO_DOUT20_23_SPEC {}
#[doc = "`write(|w| ..)` method takes [`gpo_dout20_23::W`](W) writer structure"]
impl crate::Writable for GPO_DOUT20_23_SPEC {
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets gpo_dout20_23 to value 0x0055_0000"]
impl crate::Resettable for GPO_DOUT20_23_SPEC {
    const RESET_VALUE: Self::Ux = 0x0055_0000;
}
