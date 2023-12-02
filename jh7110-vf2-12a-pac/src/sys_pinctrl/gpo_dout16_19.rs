#[doc = "Register `gpo_dout16_19` reader"]
pub type R = crate::R<GPO_DOUT16_19_SPEC>;
#[doc = "Register `gpo_dout16_19` writer"]
pub type W = crate::W<GPO_DOUT16_19_SPEC>;
#[doc = "Field `gpo16_dout` reader - The selected output signal for GPIO16. The register value indicates the selected GPIO output index signal index from GPIO output signal list 0-107. See Table 2-41: GPIO OEN List for SYS_IOMUX (on page 97) for more information."]
pub type GPO16_DOUT_R = crate::FieldReader;
#[doc = "Field `gpo16_dout` writer - The selected output signal for GPIO16. The register value indicates the selected GPIO output index signal index from GPIO output signal list 0-107. See Table 2-41: GPIO OEN List for SYS_IOMUX (on page 97) for more information."]
pub type GPO16_DOUT_W<'a, REG> = crate::FieldWriter<'a, REG, 7>;
#[doc = "Field `gpo17_dout` reader - The selected output signal for GPIO17. The register value indicates the selected GPIO output index signal index from GPIO output signal list 0-107. See Table 2-41: GPIO OEN List for SYS_IOMUX (on page 97) for more information."]
pub type GPO17_DOUT_R = crate::FieldReader;
#[doc = "Field `gpo17_dout` writer - The selected output signal for GPIO17. The register value indicates the selected GPIO output index signal index from GPIO output signal list 0-107. See Table 2-41: GPIO OEN List for SYS_IOMUX (on page 97) for more information."]
pub type GPO17_DOUT_W<'a, REG> = crate::FieldWriter<'a, REG, 7>;
#[doc = "Field `gpo18_dout` reader - The selected output signal for GPIO18. The register value indicates the selected GPIO output index signal index from GPIO output signal list 0-107. See Table 2-41: GPIO OEN List for SYS_IOMUX (on page 97) for more information."]
pub type GPO18_DOUT_R = crate::FieldReader;
#[doc = "Field `gpo18_dout` writer - The selected output signal for GPIO18. The register value indicates the selected GPIO output index signal index from GPIO output signal list 0-107. See Table 2-41: GPIO OEN List for SYS_IOMUX (on page 97) for more information."]
pub type GPO18_DOUT_W<'a, REG> = crate::FieldWriter<'a, REG, 7>;
#[doc = "Field `gpo19_dout` reader - The selected output signal for GPIO19. The register value indicates the selected GPIO output index signal index from GPIO output signal list 0-107. See Table 2-41: GPIO OEN List for SYS_IOMUX (on page 97) for more information."]
pub type GPO19_DOUT_R = crate::FieldReader;
#[doc = "Field `gpo19_dout` writer - The selected output signal for GPIO19. The register value indicates the selected GPIO output index signal index from GPIO output signal list 0-107. See Table 2-41: GPIO OEN List for SYS_IOMUX (on page 97) for more information."]
pub type GPO19_DOUT_W<'a, REG> = crate::FieldWriter<'a, REG, 7>;
impl R {
    #[doc = "Bits 0:6 - The selected output signal for GPIO16. The register value indicates the selected GPIO output index signal index from GPIO output signal list 0-107. See Table 2-41: GPIO OEN List for SYS_IOMUX (on page 97) for more information."]
    #[inline(always)]
    pub fn gpo16_dout(&self) -> GPO16_DOUT_R {
        GPO16_DOUT_R::new((self.bits & 0x7f) as u8)
    }
    #[doc = "Bits 8:14 - The selected output signal for GPIO17. The register value indicates the selected GPIO output index signal index from GPIO output signal list 0-107. See Table 2-41: GPIO OEN List for SYS_IOMUX (on page 97) for more information."]
    #[inline(always)]
    pub fn gpo17_dout(&self) -> GPO17_DOUT_R {
        GPO17_DOUT_R::new(((self.bits >> 8) & 0x7f) as u8)
    }
    #[doc = "Bits 16:22 - The selected output signal for GPIO18. The register value indicates the selected GPIO output index signal index from GPIO output signal list 0-107. See Table 2-41: GPIO OEN List for SYS_IOMUX (on page 97) for more information."]
    #[inline(always)]
    pub fn gpo18_dout(&self) -> GPO18_DOUT_R {
        GPO18_DOUT_R::new(((self.bits >> 16) & 0x7f) as u8)
    }
    #[doc = "Bits 24:30 - The selected output signal for GPIO19. The register value indicates the selected GPIO output index signal index from GPIO output signal list 0-107. See Table 2-41: GPIO OEN List for SYS_IOMUX (on page 97) for more information."]
    #[inline(always)]
    pub fn gpo19_dout(&self) -> GPO19_DOUT_R {
        GPO19_DOUT_R::new(((self.bits >> 24) & 0x7f) as u8)
    }
}
impl W {
    #[doc = "Bits 0:6 - The selected output signal for GPIO16. The register value indicates the selected GPIO output index signal index from GPIO output signal list 0-107. See Table 2-41: GPIO OEN List for SYS_IOMUX (on page 97) for more information."]
    #[inline(always)]
    #[must_use]
    pub fn gpo16_dout(&mut self) -> GPO16_DOUT_W<GPO_DOUT16_19_SPEC> {
        GPO16_DOUT_W::new(self, 0)
    }
    #[doc = "Bits 8:14 - The selected output signal for GPIO17. The register value indicates the selected GPIO output index signal index from GPIO output signal list 0-107. See Table 2-41: GPIO OEN List for SYS_IOMUX (on page 97) for more information."]
    #[inline(always)]
    #[must_use]
    pub fn gpo17_dout(&mut self) -> GPO17_DOUT_W<GPO_DOUT16_19_SPEC> {
        GPO17_DOUT_W::new(self, 8)
    }
    #[doc = "Bits 16:22 - The selected output signal for GPIO18. The register value indicates the selected GPIO output index signal index from GPIO output signal list 0-107. See Table 2-41: GPIO OEN List for SYS_IOMUX (on page 97) for more information."]
    #[inline(always)]
    #[must_use]
    pub fn gpo18_dout(&mut self) -> GPO18_DOUT_W<GPO_DOUT16_19_SPEC> {
        GPO18_DOUT_W::new(self, 16)
    }
    #[doc = "Bits 24:30 - The selected output signal for GPIO19. The register value indicates the selected GPIO output index signal index from GPIO output signal list 0-107. See Table 2-41: GPIO OEN List for SYS_IOMUX (on page 97) for more information."]
    #[inline(always)]
    #[must_use]
    pub fn gpo19_dout(&mut self) -> GPO19_DOUT_W<GPO_DOUT16_19_SPEC> {
        GPO19_DOUT_W::new(self, 24)
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
#[doc = "SYS IOMUX CFG SAIF SYSCFG FMUX GPIO 16-19 DOUT\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`gpo_dout16_19::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`gpo_dout16_19::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct GPO_DOUT16_19_SPEC;
impl crate::RegisterSpec for GPO_DOUT16_19_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`gpo_dout16_19::R`](R) reader structure"]
impl crate::Readable for GPO_DOUT16_19_SPEC {}
#[doc = "`write(|w| ..)` method takes [`gpo_dout16_19::W`](W) writer structure"]
impl crate::Writable for GPO_DOUT16_19_SPEC {
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets gpo_dout16_19 to value 0x2000_0000"]
impl crate::Resettable for GPO_DOUT16_19_SPEC {
    const RESET_VALUE: Self::Ux = 0x2000_0000;
}
