#[doc = "Register `gpo_dout_32_35` reader"]
pub type R = crate::R<GPO_DOUT_32_35_SPEC>;
#[doc = "Register `gpo_dout_32_35` writer"]
pub type W = crate::W<GPO_DOUT_32_35_SPEC>;
#[doc = "Field `dout_32` reader - The selected output signal for GPIO32. The register value indicates the selected GPIO output index signal index from GPIO output signal list 0-107. See Table 2-41: GPIO OEN List for SYS_IOMUX (on page 97) for more information."]
pub type DOUT_32_R = crate::FieldReader;
#[doc = "Field `dout_32` writer - The selected output signal for GPIO32. The register value indicates the selected GPIO output index signal index from GPIO output signal list 0-107. See Table 2-41: GPIO OEN List for SYS_IOMUX (on page 97) for more information."]
pub type DOUT_32_W<'a, REG> = crate::FieldWriter<'a, REG, 7>;
#[doc = "Field `dout_33` reader - The selected output signal for GPIO33. The register value indicates the selected GPIO output index signal index from GPIO output signal list 0-107. See Table 2-41: GPIO OEN List for SYS_IOMUX (on page 97) for more information."]
pub type DOUT_33_R = crate::FieldReader;
#[doc = "Field `dout_33` writer - The selected output signal for GPIO33. The register value indicates the selected GPIO output index signal index from GPIO output signal list 0-107. See Table 2-41: GPIO OEN List for SYS_IOMUX (on page 97) for more information."]
pub type DOUT_33_W<'a, REG> = crate::FieldWriter<'a, REG, 7>;
#[doc = "Field `dout_34` reader - The selected output signal for GPIO34. The register value indicates the selected GPIO output index signal index from GPIO output signal list 0-107. See Table 2-41: GPIO OEN List for SYS_IOMUX (on page 97) for more information."]
pub type DOUT_34_R = crate::FieldReader;
#[doc = "Field `dout_34` writer - The selected output signal for GPIO34. The register value indicates the selected GPIO output index signal index from GPIO output signal list 0-107. See Table 2-41: GPIO OEN List for SYS_IOMUX (on page 97) for more information."]
pub type DOUT_34_W<'a, REG> = crate::FieldWriter<'a, REG, 7>;
#[doc = "Field `dout_35` reader - The selected output signal for GPIO35. The register value indicates the selected GPIO output index signal index from GPIO output signal list 0-107. See Table 2-41: GPIO OEN List for SYS_IOMUX (on page 97) for more information."]
pub type DOUT_35_R = crate::FieldReader;
#[doc = "Field `dout_35` writer - The selected output signal for GPIO35. The register value indicates the selected GPIO output index signal index from GPIO output signal list 0-107. See Table 2-41: GPIO OEN List for SYS_IOMUX (on page 97) for more information."]
pub type DOUT_35_W<'a, REG> = crate::FieldWriter<'a, REG, 7>;
impl R {
    #[doc = "Bits 0:6 - The selected output signal for GPIO32. The register value indicates the selected GPIO output index signal index from GPIO output signal list 0-107. See Table 2-41: GPIO OEN List for SYS_IOMUX (on page 97) for more information."]
    #[inline(always)]
    pub fn dout_32(&self) -> DOUT_32_R {
        DOUT_32_R::new((self.bits & 0x7f) as u8)
    }
    #[doc = "Bits 8:14 - The selected output signal for GPIO33. The register value indicates the selected GPIO output index signal index from GPIO output signal list 0-107. See Table 2-41: GPIO OEN List for SYS_IOMUX (on page 97) for more information."]
    #[inline(always)]
    pub fn dout_33(&self) -> DOUT_33_R {
        DOUT_33_R::new(((self.bits >> 8) & 0x7f) as u8)
    }
    #[doc = "Bits 16:22 - The selected output signal for GPIO34. The register value indicates the selected GPIO output index signal index from GPIO output signal list 0-107. See Table 2-41: GPIO OEN List for SYS_IOMUX (on page 97) for more information."]
    #[inline(always)]
    pub fn dout_34(&self) -> DOUT_34_R {
        DOUT_34_R::new(((self.bits >> 16) & 0x7f) as u8)
    }
    #[doc = "Bits 24:30 - The selected output signal for GPIO35. The register value indicates the selected GPIO output index signal index from GPIO output signal list 0-107. See Table 2-41: GPIO OEN List for SYS_IOMUX (on page 97) for more information."]
    #[inline(always)]
    pub fn dout_35(&self) -> DOUT_35_R {
        DOUT_35_R::new(((self.bits >> 24) & 0x7f) as u8)
    }
}
impl W {
    #[doc = "Bits 0:6 - The selected output signal for GPIO32. The register value indicates the selected GPIO output index signal index from GPIO output signal list 0-107. See Table 2-41: GPIO OEN List for SYS_IOMUX (on page 97) for more information."]
    #[inline(always)]
    #[must_use]
    pub fn dout_32(&mut self) -> DOUT_32_W<GPO_DOUT_32_35_SPEC> {
        DOUT_32_W::new(self, 0)
    }
    #[doc = "Bits 8:14 - The selected output signal for GPIO33. The register value indicates the selected GPIO output index signal index from GPIO output signal list 0-107. See Table 2-41: GPIO OEN List for SYS_IOMUX (on page 97) for more information."]
    #[inline(always)]
    #[must_use]
    pub fn dout_33(&mut self) -> DOUT_33_W<GPO_DOUT_32_35_SPEC> {
        DOUT_33_W::new(self, 8)
    }
    #[doc = "Bits 16:22 - The selected output signal for GPIO34. The register value indicates the selected GPIO output index signal index from GPIO output signal list 0-107. See Table 2-41: GPIO OEN List for SYS_IOMUX (on page 97) for more information."]
    #[inline(always)]
    #[must_use]
    pub fn dout_34(&mut self) -> DOUT_34_W<GPO_DOUT_32_35_SPEC> {
        DOUT_34_W::new(self, 16)
    }
    #[doc = "Bits 24:30 - The selected output signal for GPIO35. The register value indicates the selected GPIO output index signal index from GPIO output signal list 0-107. See Table 2-41: GPIO OEN List for SYS_IOMUX (on page 97) for more information."]
    #[inline(always)]
    #[must_use]
    pub fn dout_35(&mut self) -> DOUT_35_W<GPO_DOUT_32_35_SPEC> {
        DOUT_35_W::new(self, 24)
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
#[doc = "SYS IOMUX CFG SAIF SYSCFG FMUX GPIO 32-35 DOUT\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`gpo_dout_32_35::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`gpo_dout_32_35::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct GPO_DOUT_32_35_SPEC;
impl crate::RegisterSpec for GPO_DOUT_32_35_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`gpo_dout_32_35::R`](R) reader structure"]
impl crate::Readable for GPO_DOUT_32_35_SPEC {}
#[doc = "`write(|w| ..)` method takes [`gpo_dout_32_35::W`](W) writer structure"]
impl crate::Writable for GPO_DOUT_32_35_SPEC {
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets gpo_dout_32_35 to value 0x0d00_0000"]
impl crate::Resettable for GPO_DOUT_32_35_SPEC {
    const RESET_VALUE: Self::Ux = 0x0d00_0000;
}
