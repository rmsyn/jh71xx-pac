#[doc = "Register `gpo_dout36_39` reader"]
pub type R = crate::R<GPO_DOUT36_39_SPEC>;
#[doc = "Register `gpo_dout36_39` writer"]
pub type W = crate::W<GPO_DOUT36_39_SPEC>;
#[doc = "Field `gpo36_dout` reader - The selected output signal for GPIO36. The register value indicates the selected GPIO output index signal index from GPIO output signal list 0-107. See Table 2-41: GPIO OEN List for SYS_IOMUX (on page 97) for more information."]
pub type GPO36_DOUT_R = crate::FieldReader;
#[doc = "Field `gpo36_dout` writer - The selected output signal for GPIO36. The register value indicates the selected GPIO output index signal index from GPIO output signal list 0-107. See Table 2-41: GPIO OEN List for SYS_IOMUX (on page 97) for more information."]
pub type GPO36_DOUT_W<'a, REG> = crate::FieldWriter<'a, REG, 7>;
#[doc = "Field `gpo37_dout` reader - The selected output signal for GPIO37. The register value indicates the selected GPIO output index signal index from GPIO output signal list 0-107. See Table 2-41: GPIO OEN List for SYS_IOMUX (on page 97) for more information."]
pub type GPO37_DOUT_R = crate::FieldReader;
#[doc = "Field `gpo37_dout` writer - The selected output signal for GPIO37. The register value indicates the selected GPIO output index signal index from GPIO output signal list 0-107. See Table 2-41: GPIO OEN List for SYS_IOMUX (on page 97) for more information."]
pub type GPO37_DOUT_W<'a, REG> = crate::FieldWriter<'a, REG, 7>;
#[doc = "Field `gpo38_dout` reader - The selected output signal for GPIO38. The register value indicates the selected GPIO output index signal index from GPIO output signal list 0-107. See Table 2-41: GPIO OEN List for SYS_IOMUX (on page 97) for more information."]
pub type GPO38_DOUT_R = crate::FieldReader;
#[doc = "Field `gpo38_dout` writer - The selected output signal for GPIO38. The register value indicates the selected GPIO output index signal index from GPIO output signal list 0-107. See Table 2-41: GPIO OEN List for SYS_IOMUX (on page 97) for more information."]
pub type GPO38_DOUT_W<'a, REG> = crate::FieldWriter<'a, REG, 7>;
#[doc = "Field `gpo39_dout` reader - The selected output signal for GPIO39. The register value indicates the selected GPIO output index signal index from GPIO output signal list 0-107. See Table 2-41: GPIO OEN List for SYS_IOMUX (on page 97) for more information."]
pub type GPO39_DOUT_R = crate::FieldReader;
#[doc = "Field `gpo39_dout` writer - The selected output signal for GPIO39. The register value indicates the selected GPIO output index signal index from GPIO output signal list 0-107. See Table 2-41: GPIO OEN List for SYS_IOMUX (on page 97) for more information."]
pub type GPO39_DOUT_W<'a, REG> = crate::FieldWriter<'a, REG, 7>;
impl R {
    #[doc = "Bits 0:6 - The selected output signal for GPIO36. The register value indicates the selected GPIO output index signal index from GPIO output signal list 0-107. See Table 2-41: GPIO OEN List for SYS_IOMUX (on page 97) for more information."]
    #[inline(always)]
    pub fn gpo36_dout(&self) -> GPO36_DOUT_R {
        GPO36_DOUT_R::new((self.bits & 0x7f) as u8)
    }
    #[doc = "Bits 8:14 - The selected output signal for GPIO37. The register value indicates the selected GPIO output index signal index from GPIO output signal list 0-107. See Table 2-41: GPIO OEN List for SYS_IOMUX (on page 97) for more information."]
    #[inline(always)]
    pub fn gpo37_dout(&self) -> GPO37_DOUT_R {
        GPO37_DOUT_R::new(((self.bits >> 8) & 0x7f) as u8)
    }
    #[doc = "Bits 16:22 - The selected output signal for GPIO38. The register value indicates the selected GPIO output index signal index from GPIO output signal list 0-107. See Table 2-41: GPIO OEN List for SYS_IOMUX (on page 97) for more information."]
    #[inline(always)]
    pub fn gpo38_dout(&self) -> GPO38_DOUT_R {
        GPO38_DOUT_R::new(((self.bits >> 16) & 0x7f) as u8)
    }
    #[doc = "Bits 24:30 - The selected output signal for GPIO39. The register value indicates the selected GPIO output index signal index from GPIO output signal list 0-107. See Table 2-41: GPIO OEN List for SYS_IOMUX (on page 97) for more information."]
    #[inline(always)]
    pub fn gpo39_dout(&self) -> GPO39_DOUT_R {
        GPO39_DOUT_R::new(((self.bits >> 24) & 0x7f) as u8)
    }
}
impl W {
    #[doc = "Bits 0:6 - The selected output signal for GPIO36. The register value indicates the selected GPIO output index signal index from GPIO output signal list 0-107. See Table 2-41: GPIO OEN List for SYS_IOMUX (on page 97) for more information."]
    #[inline(always)]
    #[must_use]
    pub fn gpo36_dout(&mut self) -> GPO36_DOUT_W<GPO_DOUT36_39_SPEC> {
        GPO36_DOUT_W::new(self, 0)
    }
    #[doc = "Bits 8:14 - The selected output signal for GPIO37. The register value indicates the selected GPIO output index signal index from GPIO output signal list 0-107. See Table 2-41: GPIO OEN List for SYS_IOMUX (on page 97) for more information."]
    #[inline(always)]
    #[must_use]
    pub fn gpo37_dout(&mut self) -> GPO37_DOUT_W<GPO_DOUT36_39_SPEC> {
        GPO37_DOUT_W::new(self, 8)
    }
    #[doc = "Bits 16:22 - The selected output signal for GPIO38. The register value indicates the selected GPIO output index signal index from GPIO output signal list 0-107. See Table 2-41: GPIO OEN List for SYS_IOMUX (on page 97) for more information."]
    #[inline(always)]
    #[must_use]
    pub fn gpo38_dout(&mut self) -> GPO38_DOUT_W<GPO_DOUT36_39_SPEC> {
        GPO38_DOUT_W::new(self, 16)
    }
    #[doc = "Bits 24:30 - The selected output signal for GPIO39. The register value indicates the selected GPIO output index signal index from GPIO output signal list 0-107. See Table 2-41: GPIO OEN List for SYS_IOMUX (on page 97) for more information."]
    #[inline(always)]
    #[must_use]
    pub fn gpo39_dout(&mut self) -> GPO39_DOUT_W<GPO_DOUT36_39_SPEC> {
        GPO39_DOUT_W::new(self, 24)
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
#[doc = "SYS IOMUX CFG SAIF SYSCFG FMUX GPIO 36-39 DOUT\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`gpo_dout36_39::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`gpo_dout36_39::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct GPO_DOUT36_39_SPEC;
impl crate::RegisterSpec for GPO_DOUT36_39_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`gpo_dout36_39::R`](R) reader structure"]
impl crate::Readable for GPO_DOUT36_39_SPEC {}
#[doc = "`write(|w| ..)` method takes [`gpo_dout36_39::W`](W) writer structure"]
impl crate::Writable for GPO_DOUT36_39_SPEC {
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets gpo_dout36_39 to value 0x5453_0f0e"]
impl crate::Resettable for GPO_DOUT36_39_SPEC {
    const RESET_VALUE: Self::Ux = 0x5453_0f0e;
}
