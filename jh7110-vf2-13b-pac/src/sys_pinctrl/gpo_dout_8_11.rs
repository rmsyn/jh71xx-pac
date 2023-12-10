#[doc = "Register `gpo_dout_8_11` reader"]
pub type R = crate::R<GPO_DOUT_8_11_SPEC>;
#[doc = "Register `gpo_dout_8_11` writer"]
pub type W = crate::W<GPO_DOUT_8_11_SPEC>;
#[doc = "Field `dout_8` reader - The selected output signal for GPIO8. The register value indicates the selected GPIO output index signal index from GPIO output signal list 0-107. See Table 2-41: GPIO OEN List for SYS_IOMUX (on page 97) for more information."]
pub type DOUT_8_R = crate::FieldReader;
#[doc = "Field `dout_8` writer - The selected output signal for GPIO8. The register value indicates the selected GPIO output index signal index from GPIO output signal list 0-107. See Table 2-41: GPIO OEN List for SYS_IOMUX (on page 97) for more information."]
pub type DOUT_8_W<'a, REG> = crate::FieldWriter<'a, REG, 7>;
#[doc = "Field `dout_9` reader - The selected output signal for GPIO9. The register value indicates the selected GPIO output index signal index from GPIO output signal list 0-107. See Table 2-41: GPIO OEN List for SYS_IOMUX (on page 97) for more information."]
pub type DOUT_9_R = crate::FieldReader;
#[doc = "Field `dout_9` writer - The selected output signal for GPIO9. The register value indicates the selected GPIO output index signal index from GPIO output signal list 0-107. See Table 2-41: GPIO OEN List for SYS_IOMUX (on page 97) for more information."]
pub type DOUT_9_W<'a, REG> = crate::FieldWriter<'a, REG, 7>;
#[doc = "Field `dout_10` reader - The selected output signal for GPIO10. The register value indicates the selected GPIO output index signal index from GPIO output signal list 0-107. See Table 2-41: GPIO OEN List for SYS_IOMUX (on page 97) for more information."]
pub type DOUT_10_R = crate::FieldReader;
#[doc = "Field `dout_10` writer - The selected output signal for GPIO10. The register value indicates the selected GPIO output index signal index from GPIO output signal list 0-107. See Table 2-41: GPIO OEN List for SYS_IOMUX (on page 97) for more information."]
pub type DOUT_10_W<'a, REG> = crate::FieldWriter<'a, REG, 7>;
#[doc = "Field `dout_11` reader - The selected output signal for GPIO11. The register value indicates the selected GPIO output index signal index from GPIO output signal list 0-107. See Table 2-41: GPIO OEN List for SYS_IOMUX (on page 97) for more information."]
pub type DOUT_11_R = crate::FieldReader;
#[doc = "Field `dout_11` writer - The selected output signal for GPIO11. The register value indicates the selected GPIO output index signal index from GPIO output signal list 0-107. See Table 2-41: GPIO OEN List for SYS_IOMUX (on page 97) for more information."]
pub type DOUT_11_W<'a, REG> = crate::FieldWriter<'a, REG, 7>;
impl R {
    #[doc = "Bits 0:6 - The selected output signal for GPIO8. The register value indicates the selected GPIO output index signal index from GPIO output signal list 0-107. See Table 2-41: GPIO OEN List for SYS_IOMUX (on page 97) for more information."]
    #[inline(always)]
    pub fn dout_8(&self) -> DOUT_8_R {
        DOUT_8_R::new((self.bits & 0x7f) as u8)
    }
    #[doc = "Bits 8:14 - The selected output signal for GPIO9. The register value indicates the selected GPIO output index signal index from GPIO output signal list 0-107. See Table 2-41: GPIO OEN List for SYS_IOMUX (on page 97) for more information."]
    #[inline(always)]
    pub fn dout_9(&self) -> DOUT_9_R {
        DOUT_9_R::new(((self.bits >> 8) & 0x7f) as u8)
    }
    #[doc = "Bits 16:22 - The selected output signal for GPIO10. The register value indicates the selected GPIO output index signal index from GPIO output signal list 0-107. See Table 2-41: GPIO OEN List for SYS_IOMUX (on page 97) for more information."]
    #[inline(always)]
    pub fn dout_10(&self) -> DOUT_10_R {
        DOUT_10_R::new(((self.bits >> 16) & 0x7f) as u8)
    }
    #[doc = "Bits 24:30 - The selected output signal for GPIO11. The register value indicates the selected GPIO output index signal index from GPIO output signal list 0-107. See Table 2-41: GPIO OEN List for SYS_IOMUX (on page 97) for more information."]
    #[inline(always)]
    pub fn dout_11(&self) -> DOUT_11_R {
        DOUT_11_R::new(((self.bits >> 24) & 0x7f) as u8)
    }
}
impl W {
    #[doc = "Bits 0:6 - The selected output signal for GPIO8. The register value indicates the selected GPIO output index signal index from GPIO output signal list 0-107. See Table 2-41: GPIO OEN List for SYS_IOMUX (on page 97) for more information."]
    #[inline(always)]
    #[must_use]
    pub fn dout_8(&mut self) -> DOUT_8_W<GPO_DOUT_8_11_SPEC> {
        DOUT_8_W::new(self, 0)
    }
    #[doc = "Bits 8:14 - The selected output signal for GPIO9. The register value indicates the selected GPIO output index signal index from GPIO output signal list 0-107. See Table 2-41: GPIO OEN List for SYS_IOMUX (on page 97) for more information."]
    #[inline(always)]
    #[must_use]
    pub fn dout_9(&mut self) -> DOUT_9_W<GPO_DOUT_8_11_SPEC> {
        DOUT_9_W::new(self, 8)
    }
    #[doc = "Bits 16:22 - The selected output signal for GPIO10. The register value indicates the selected GPIO output index signal index from GPIO output signal list 0-107. See Table 2-41: GPIO OEN List for SYS_IOMUX (on page 97) for more information."]
    #[inline(always)]
    #[must_use]
    pub fn dout_10(&mut self) -> DOUT_10_W<GPO_DOUT_8_11_SPEC> {
        DOUT_10_W::new(self, 16)
    }
    #[doc = "Bits 24:30 - The selected output signal for GPIO11. The register value indicates the selected GPIO output index signal index from GPIO output signal list 0-107. See Table 2-41: GPIO OEN List for SYS_IOMUX (on page 97) for more information."]
    #[inline(always)]
    #[must_use]
    pub fn dout_11(&mut self) -> DOUT_11_W<GPO_DOUT_8_11_SPEC> {
        DOUT_11_W::new(self, 24)
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
#[doc = "SYS IOMUX CFG SAIF SYSCFG FMUX GPIO 8-11 DOUT\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`gpo_dout_8_11::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`gpo_dout_8_11::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct GPO_DOUT_8_11_SPEC;
impl crate::RegisterSpec for GPO_DOUT_8_11_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`gpo_dout_8_11::R`](R) reader structure"]
impl crate::Readable for GPO_DOUT_8_11_SPEC {}
#[doc = "`write(|w| ..)` method takes [`gpo_dout_8_11::W`](W) writer structure"]
impl crate::Writable for GPO_DOUT_8_11_SPEC {
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets gpo_dout_8_11 to value 0x1500_0000"]
impl crate::Resettable for GPO_DOUT_8_11_SPEC {
    const RESET_VALUE: Self::Ux = 0x1500_0000;
}
