#[doc = "Register `gpo_dout_36_39` reader"]
pub type R = crate::R<GPO_DOUT_36_39_SPEC>;
#[doc = "Register `gpo_dout_36_39` writer"]
pub type W = crate::W<GPO_DOUT_36_39_SPEC>;
#[doc = "Field `dout_36` reader - The selected output signal for GPIO36. The register value indicates the selected GPIO output index signal index from GPIO output signal list 0-107. See Table 2-41: GPIO OEN List for SYS_IOMUX (on page 97) for more information."]
pub type DOUT_36_R = crate::FieldReader;
#[doc = "Field `dout_36` writer - The selected output signal for GPIO36. The register value indicates the selected GPIO output index signal index from GPIO output signal list 0-107. See Table 2-41: GPIO OEN List for SYS_IOMUX (on page 97) for more information."]
pub type DOUT_36_W<'a, REG> = crate::FieldWriter<'a, REG, 7>;
#[doc = "Field `dout_37` reader - The selected output signal for GPIO37. The register value indicates the selected GPIO output index signal index from GPIO output signal list 0-107. See Table 2-41: GPIO OEN List for SYS_IOMUX (on page 97) for more information."]
pub type DOUT_37_R = crate::FieldReader;
#[doc = "Field `dout_37` writer - The selected output signal for GPIO37. The register value indicates the selected GPIO output index signal index from GPIO output signal list 0-107. See Table 2-41: GPIO OEN List for SYS_IOMUX (on page 97) for more information."]
pub type DOUT_37_W<'a, REG> = crate::FieldWriter<'a, REG, 7>;
#[doc = "Field `dout_38` reader - The selected output signal for GPIO38. The register value indicates the selected GPIO output index signal index from GPIO output signal list 0-107. See Table 2-41: GPIO OEN List for SYS_IOMUX (on page 97) for more information."]
pub type DOUT_38_R = crate::FieldReader;
#[doc = "Field `dout_38` writer - The selected output signal for GPIO38. The register value indicates the selected GPIO output index signal index from GPIO output signal list 0-107. See Table 2-41: GPIO OEN List for SYS_IOMUX (on page 97) for more information."]
pub type DOUT_38_W<'a, REG> = crate::FieldWriter<'a, REG, 7>;
#[doc = "Field `dout_39` reader - The selected output signal for GPIO39. The register value indicates the selected GPIO output index signal index from GPIO output signal list 0-107. See Table 2-41: GPIO OEN List for SYS_IOMUX (on page 97) for more information."]
pub type DOUT_39_R = crate::FieldReader;
#[doc = "Field `dout_39` writer - The selected output signal for GPIO39. The register value indicates the selected GPIO output index signal index from GPIO output signal list 0-107. See Table 2-41: GPIO OEN List for SYS_IOMUX (on page 97) for more information."]
pub type DOUT_39_W<'a, REG> = crate::FieldWriter<'a, REG, 7>;
impl R {
    #[doc = "Bits 0:6 - The selected output signal for GPIO36. The register value indicates the selected GPIO output index signal index from GPIO output signal list 0-107. See Table 2-41: GPIO OEN List for SYS_IOMUX (on page 97) for more information."]
    #[inline(always)]
    pub fn dout_36(&self) -> DOUT_36_R {
        DOUT_36_R::new((self.bits & 0x7f) as u8)
    }
    #[doc = "Bits 8:14 - The selected output signal for GPIO37. The register value indicates the selected GPIO output index signal index from GPIO output signal list 0-107. See Table 2-41: GPIO OEN List for SYS_IOMUX (on page 97) for more information."]
    #[inline(always)]
    pub fn dout_37(&self) -> DOUT_37_R {
        DOUT_37_R::new(((self.bits >> 8) & 0x7f) as u8)
    }
    #[doc = "Bits 16:22 - The selected output signal for GPIO38. The register value indicates the selected GPIO output index signal index from GPIO output signal list 0-107. See Table 2-41: GPIO OEN List for SYS_IOMUX (on page 97) for more information."]
    #[inline(always)]
    pub fn dout_38(&self) -> DOUT_38_R {
        DOUT_38_R::new(((self.bits >> 16) & 0x7f) as u8)
    }
    #[doc = "Bits 24:30 - The selected output signal for GPIO39. The register value indicates the selected GPIO output index signal index from GPIO output signal list 0-107. See Table 2-41: GPIO OEN List for SYS_IOMUX (on page 97) for more information."]
    #[inline(always)]
    pub fn dout_39(&self) -> DOUT_39_R {
        DOUT_39_R::new(((self.bits >> 24) & 0x7f) as u8)
    }
}
impl W {
    #[doc = "Bits 0:6 - The selected output signal for GPIO36. The register value indicates the selected GPIO output index signal index from GPIO output signal list 0-107. See Table 2-41: GPIO OEN List for SYS_IOMUX (on page 97) for more information."]
    #[inline(always)]
    #[must_use]
    pub fn dout_36(&mut self) -> DOUT_36_W<GPO_DOUT_36_39_SPEC> {
        DOUT_36_W::new(self, 0)
    }
    #[doc = "Bits 8:14 - The selected output signal for GPIO37. The register value indicates the selected GPIO output index signal index from GPIO output signal list 0-107. See Table 2-41: GPIO OEN List for SYS_IOMUX (on page 97) for more information."]
    #[inline(always)]
    #[must_use]
    pub fn dout_37(&mut self) -> DOUT_37_W<GPO_DOUT_36_39_SPEC> {
        DOUT_37_W::new(self, 8)
    }
    #[doc = "Bits 16:22 - The selected output signal for GPIO38. The register value indicates the selected GPIO output index signal index from GPIO output signal list 0-107. See Table 2-41: GPIO OEN List for SYS_IOMUX (on page 97) for more information."]
    #[inline(always)]
    #[must_use]
    pub fn dout_38(&mut self) -> DOUT_38_W<GPO_DOUT_36_39_SPEC> {
        DOUT_38_W::new(self, 16)
    }
    #[doc = "Bits 24:30 - The selected output signal for GPIO39. The register value indicates the selected GPIO output index signal index from GPIO output signal list 0-107. See Table 2-41: GPIO OEN List for SYS_IOMUX (on page 97) for more information."]
    #[inline(always)]
    #[must_use]
    pub fn dout_39(&mut self) -> DOUT_39_W<GPO_DOUT_36_39_SPEC> {
        DOUT_39_W::new(self, 24)
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
#[doc = "SYS IOMUX CFG SAIF SYSCFG FMUX GPIO 36-39 DOUT\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`gpo_dout_36_39::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`gpo_dout_36_39::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct GPO_DOUT_36_39_SPEC;
impl crate::RegisterSpec for GPO_DOUT_36_39_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`gpo_dout_36_39::R`](R) reader structure"]
impl crate::Readable for GPO_DOUT_36_39_SPEC {}
#[doc = "`write(|w| ..)` method takes [`gpo_dout_36_39::W`](W) writer structure"]
impl crate::Writable for GPO_DOUT_36_39_SPEC {
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets gpo_dout_36_39 to value 0x5453_0f0e"]
impl crate::Resettable for GPO_DOUT_36_39_SPEC {
    const RESET_VALUE: Self::Ux = 0x5453_0f0e;
}
