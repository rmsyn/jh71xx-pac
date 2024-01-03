#[doc = "Register `gpo_dout_11` reader"]
pub type R = crate::R<GPO_DOUT_11_SPEC>;
#[doc = "Register `gpo_dout_11` writer"]
pub type W = crate::W<GPO_DOUT_11_SPEC>;
#[doc = "Field `dout_44` reader - The selected output signal for GPIO44. The register value indicates the selected GPIO output index signal index from GPIO output signal list 0-107. See Table 2-41: GPIO OEN List for SYS_IOMUX (on page 97) for more information."]
pub type DOUT_44_R = crate::FieldReader;
#[doc = "Field `dout_44` writer - The selected output signal for GPIO44. The register value indicates the selected GPIO output index signal index from GPIO output signal list 0-107. See Table 2-41: GPIO OEN List for SYS_IOMUX (on page 97) for more information."]
pub type DOUT_44_W<'a, REG> = crate::FieldWriter<'a, REG, 7>;
#[doc = "Field `dout_45` reader - The selected output signal for GPIO45. The register value indicates the selected GPIO output index signal index from GPIO output signal list 0-107. See Table 2-41: GPIO OEN List for SYS_IOMUX (on page 97) for more information."]
pub type DOUT_45_R = crate::FieldReader;
#[doc = "Field `dout_45` writer - The selected output signal for GPIO45. The register value indicates the selected GPIO output index signal index from GPIO output signal list 0-107. See Table 2-41: GPIO OEN List for SYS_IOMUX (on page 97) for more information."]
pub type DOUT_45_W<'a, REG> = crate::FieldWriter<'a, REG, 7>;
#[doc = "Field `dout_46` reader - The selected output signal for GPIO46. The register value indicates the selected GPIO output index signal index from GPIO output signal list 0-107. See Table 2-41: GPIO OEN List for SYS_IOMUX (on page 97) for more information."]
pub type DOUT_46_R = crate::FieldReader;
#[doc = "Field `dout_46` writer - The selected output signal for GPIO46. The register value indicates the selected GPIO output index signal index from GPIO output signal list 0-107. See Table 2-41: GPIO OEN List for SYS_IOMUX (on page 97) for more information."]
pub type DOUT_46_W<'a, REG> = crate::FieldWriter<'a, REG, 7>;
#[doc = "Field `dout_47` reader - The selected output signal for GPIO47. The register value indicates the selected GPIO output index signal index from GPIO output signal list 0-107. See Table 2-41: GPIO OEN List for SYS_IOMUX (on page 97) for more information."]
pub type DOUT_47_R = crate::FieldReader;
#[doc = "Field `dout_47` writer - The selected output signal for GPIO47. The register value indicates the selected GPIO output index signal index from GPIO output signal list 0-107. See Table 2-41: GPIO OEN List for SYS_IOMUX (on page 97) for more information."]
pub type DOUT_47_W<'a, REG> = crate::FieldWriter<'a, REG, 7>;
impl R {
    #[doc = "Bits 0:6 - The selected output signal for GPIO44. The register value indicates the selected GPIO output index signal index from GPIO output signal list 0-107. See Table 2-41: GPIO OEN List for SYS_IOMUX (on page 97) for more information."]
    #[inline(always)]
    pub fn dout_44(&self) -> DOUT_44_R {
        DOUT_44_R::new((self.bits & 0x7f) as u8)
    }
    #[doc = "Bits 8:14 - The selected output signal for GPIO45. The register value indicates the selected GPIO output index signal index from GPIO output signal list 0-107. See Table 2-41: GPIO OEN List for SYS_IOMUX (on page 97) for more information."]
    #[inline(always)]
    pub fn dout_45(&self) -> DOUT_45_R {
        DOUT_45_R::new(((self.bits >> 8) & 0x7f) as u8)
    }
    #[doc = "Bits 16:22 - The selected output signal for GPIO46. The register value indicates the selected GPIO output index signal index from GPIO output signal list 0-107. See Table 2-41: GPIO OEN List for SYS_IOMUX (on page 97) for more information."]
    #[inline(always)]
    pub fn dout_46(&self) -> DOUT_46_R {
        DOUT_46_R::new(((self.bits >> 16) & 0x7f) as u8)
    }
    #[doc = "Bits 24:30 - The selected output signal for GPIO47. The register value indicates the selected GPIO output index signal index from GPIO output signal list 0-107. See Table 2-41: GPIO OEN List for SYS_IOMUX (on page 97) for more information."]
    #[inline(always)]
    pub fn dout_47(&self) -> DOUT_47_R {
        DOUT_47_R::new(((self.bits >> 24) & 0x7f) as u8)
    }
}
impl W {
    #[doc = "Bits 0:6 - The selected output signal for GPIO44. The register value indicates the selected GPIO output index signal index from GPIO output signal list 0-107. See Table 2-41: GPIO OEN List for SYS_IOMUX (on page 97) for more information."]
    #[inline(always)]
    #[must_use]
    pub fn dout_44(&mut self) -> DOUT_44_W<GPO_DOUT_11_SPEC> {
        DOUT_44_W::new(self, 0)
    }
    #[doc = "Bits 8:14 - The selected output signal for GPIO45. The register value indicates the selected GPIO output index signal index from GPIO output signal list 0-107. See Table 2-41: GPIO OEN List for SYS_IOMUX (on page 97) for more information."]
    #[inline(always)]
    #[must_use]
    pub fn dout_45(&mut self) -> DOUT_45_W<GPO_DOUT_11_SPEC> {
        DOUT_45_W::new(self, 8)
    }
    #[doc = "Bits 16:22 - The selected output signal for GPIO46. The register value indicates the selected GPIO output index signal index from GPIO output signal list 0-107. See Table 2-41: GPIO OEN List for SYS_IOMUX (on page 97) for more information."]
    #[inline(always)]
    #[must_use]
    pub fn dout_46(&mut self) -> DOUT_46_W<GPO_DOUT_11_SPEC> {
        DOUT_46_W::new(self, 16)
    }
    #[doc = "Bits 24:30 - The selected output signal for GPIO47. The register value indicates the selected GPIO output index signal index from GPIO output signal list 0-107. See Table 2-41: GPIO OEN List for SYS_IOMUX (on page 97) for more information."]
    #[inline(always)]
    #[must_use]
    pub fn dout_47(&mut self) -> DOUT_47_W<GPO_DOUT_11_SPEC> {
        DOUT_47_W::new(self, 24)
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
#[doc = "SYS IOMUX CFG SAIF SYSCFG FMUX GPIO 44-47 DOUT\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`gpo_dout_11::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`gpo_dout_11::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct GPO_DOUT_11_SPEC;
impl crate::RegisterSpec for GPO_DOUT_11_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`gpo_dout_11::R`](R) reader structure"]
impl crate::Readable for GPO_DOUT_11_SPEC {}
#[doc = "`write(|w| ..)` method takes [`gpo_dout_11::W`](W) writer structure"]
impl crate::Writable for GPO_DOUT_11_SPEC {
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets gpo_dout_11 to value 0x005b_5c00"]
impl crate::Resettable for GPO_DOUT_11_SPEC {
    const RESET_VALUE: Self::Ux = 0x005b_5c00;
}
