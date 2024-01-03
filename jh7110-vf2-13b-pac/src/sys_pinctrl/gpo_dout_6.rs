#[doc = "Register `gpo_dout_6` reader"]
pub type R = crate::R<GPO_DOUT_6_SPEC>;
#[doc = "Register `gpo_dout_6` writer"]
pub type W = crate::W<GPO_DOUT_6_SPEC>;
#[doc = "Field `dout_24` reader - The selected output signal for GPIO24. The register value indicates the selected GPIO output index signal index from GPIO output signal list 0-107. See Table 2-41: GPIO OEN List for SYS_IOMUX (on page 97) for more information."]
pub type DOUT_24_R = crate::FieldReader;
#[doc = "Field `dout_24` writer - The selected output signal for GPIO24. The register value indicates the selected GPIO output index signal index from GPIO output signal list 0-107. See Table 2-41: GPIO OEN List for SYS_IOMUX (on page 97) for more information."]
pub type DOUT_24_W<'a, REG> = crate::FieldWriter<'a, REG, 7>;
#[doc = "Field `dout_25` reader - The selected output signal for GPIO25. The register value indicates the selected GPIO output index signal index from GPIO output signal list 0-107. See Table 2-41: GPIO OEN List for SYS_IOMUX (on page 97) for more information."]
pub type DOUT_25_R = crate::FieldReader;
#[doc = "Field `dout_25` writer - The selected output signal for GPIO25. The register value indicates the selected GPIO output index signal index from GPIO output signal list 0-107. See Table 2-41: GPIO OEN List for SYS_IOMUX (on page 97) for more information."]
pub type DOUT_25_W<'a, REG> = crate::FieldWriter<'a, REG, 7>;
#[doc = "Field `dout_26` reader - The selected output signal for GPIO26. The register value indicates the selected GPIO output index signal index from GPIO output signal list 0-107. See Table 2-41: GPIO OEN List for SYS_IOMUX (on page 97) for more information."]
pub type DOUT_26_R = crate::FieldReader;
#[doc = "Field `dout_26` writer - The selected output signal for GPIO26. The register value indicates the selected GPIO output index signal index from GPIO output signal list 0-107. See Table 2-41: GPIO OEN List for SYS_IOMUX (on page 97) for more information."]
pub type DOUT_26_W<'a, REG> = crate::FieldWriter<'a, REG, 7>;
#[doc = "Field `dout_27` reader - The selected output signal for GPIO27. The register value indicates the selected GPIO output index signal index from GPIO output signal list 0-107. See Table 2-41: GPIO OEN List for SYS_IOMUX (on page 97) for more information."]
pub type DOUT_27_R = crate::FieldReader;
#[doc = "Field `dout_27` writer - The selected output signal for GPIO27. The register value indicates the selected GPIO output index signal index from GPIO output signal list 0-107. See Table 2-41: GPIO OEN List for SYS_IOMUX (on page 97) for more information."]
pub type DOUT_27_W<'a, REG> = crate::FieldWriter<'a, REG, 7>;
impl R {
    #[doc = "Bits 0:6 - The selected output signal for GPIO24. The register value indicates the selected GPIO output index signal index from GPIO output signal list 0-107. See Table 2-41: GPIO OEN List for SYS_IOMUX (on page 97) for more information."]
    #[inline(always)]
    pub fn dout_24(&self) -> DOUT_24_R {
        DOUT_24_R::new((self.bits & 0x7f) as u8)
    }
    #[doc = "Bits 8:14 - The selected output signal for GPIO25. The register value indicates the selected GPIO output index signal index from GPIO output signal list 0-107. See Table 2-41: GPIO OEN List for SYS_IOMUX (on page 97) for more information."]
    #[inline(always)]
    pub fn dout_25(&self) -> DOUT_25_R {
        DOUT_25_R::new(((self.bits >> 8) & 0x7f) as u8)
    }
    #[doc = "Bits 16:22 - The selected output signal for GPIO26. The register value indicates the selected GPIO output index signal index from GPIO output signal list 0-107. See Table 2-41: GPIO OEN List for SYS_IOMUX (on page 97) for more information."]
    #[inline(always)]
    pub fn dout_26(&self) -> DOUT_26_R {
        DOUT_26_R::new(((self.bits >> 16) & 0x7f) as u8)
    }
    #[doc = "Bits 24:30 - The selected output signal for GPIO27. The register value indicates the selected GPIO output index signal index from GPIO output signal list 0-107. See Table 2-41: GPIO OEN List for SYS_IOMUX (on page 97) for more information."]
    #[inline(always)]
    pub fn dout_27(&self) -> DOUT_27_R {
        DOUT_27_R::new(((self.bits >> 24) & 0x7f) as u8)
    }
}
impl W {
    #[doc = "Bits 0:6 - The selected output signal for GPIO24. The register value indicates the selected GPIO output index signal index from GPIO output signal list 0-107. See Table 2-41: GPIO OEN List for SYS_IOMUX (on page 97) for more information."]
    #[inline(always)]
    #[must_use]
    pub fn dout_24(&mut self) -> DOUT_24_W<GPO_DOUT_6_SPEC> {
        DOUT_24_W::new(self, 0)
    }
    #[doc = "Bits 8:14 - The selected output signal for GPIO25. The register value indicates the selected GPIO output index signal index from GPIO output signal list 0-107. See Table 2-41: GPIO OEN List for SYS_IOMUX (on page 97) for more information."]
    #[inline(always)]
    #[must_use]
    pub fn dout_25(&mut self) -> DOUT_25_W<GPO_DOUT_6_SPEC> {
        DOUT_25_W::new(self, 8)
    }
    #[doc = "Bits 16:22 - The selected output signal for GPIO26. The register value indicates the selected GPIO output index signal index from GPIO output signal list 0-107. See Table 2-41: GPIO OEN List for SYS_IOMUX (on page 97) for more information."]
    #[inline(always)]
    #[must_use]
    pub fn dout_26(&mut self) -> DOUT_26_W<GPO_DOUT_6_SPEC> {
        DOUT_26_W::new(self, 16)
    }
    #[doc = "Bits 24:30 - The selected output signal for GPIO27. The register value indicates the selected GPIO output index signal index from GPIO output signal list 0-107. See Table 2-41: GPIO OEN List for SYS_IOMUX (on page 97) for more information."]
    #[inline(always)]
    #[must_use]
    pub fn dout_27(&mut self) -> DOUT_27_W<GPO_DOUT_6_SPEC> {
        DOUT_27_W::new(self, 24)
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
#[doc = "SYS IOMUX CFG SAIF SYSCFG FMUX GPIO 24-27 DOUT\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`gpo_dout_6::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`gpo_dout_6::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct GPO_DOUT_6_SPEC;
impl crate::RegisterSpec for GPO_DOUT_6_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`gpo_dout_6::R`](R) reader structure"]
impl crate::Readable for GPO_DOUT_6_SPEC {}
#[doc = "`write(|w| ..)` method takes [`gpo_dout_6::W`](W) writer structure"]
impl crate::Writable for GPO_DOUT_6_SPEC {
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets gpo_dout_6 to value 0"]
impl crate::Resettable for GPO_DOUT_6_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
