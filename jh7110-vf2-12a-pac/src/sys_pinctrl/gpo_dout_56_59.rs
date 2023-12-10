#[doc = "Register `gpo_dout_56_59` reader"]
pub type R = crate::R<GPO_DOUT_56_59_SPEC>;
#[doc = "Register `gpo_dout_56_59` writer"]
pub type W = crate::W<GPO_DOUT_56_59_SPEC>;
#[doc = "Field `dout_56` reader - The selected output signal for GPIO56. The register value indicates the selected GPIO output index signal index from GPIO output signal list 0-107. See Table 2-41: GPIO OEN List for SYS_IOMUX (on page 97) for more information."]
pub type DOUT_56_R = crate::FieldReader;
#[doc = "Field `dout_56` writer - The selected output signal for GPIO56. The register value indicates the selected GPIO output index signal index from GPIO output signal list 0-107. See Table 2-41: GPIO OEN List for SYS_IOMUX (on page 97) for more information."]
pub type DOUT_56_W<'a, REG> = crate::FieldWriter<'a, REG, 7>;
#[doc = "Field `dout_57` reader - The selected output signal for GPIO57. The register value indicates the selected GPIO output index signal index from GPIO output signal list 0-107. See Table 2-41: GPIO OEN List for SYS_IOMUX (on page 97) for more information."]
pub type DOUT_57_R = crate::FieldReader;
#[doc = "Field `dout_57` writer - The selected output signal for GPIO57. The register value indicates the selected GPIO output index signal index from GPIO output signal list 0-107. See Table 2-41: GPIO OEN List for SYS_IOMUX (on page 97) for more information."]
pub type DOUT_57_W<'a, REG> = crate::FieldWriter<'a, REG, 7>;
#[doc = "Field `dout_58` reader - The selected output signal for GPIO58. The register value indicates the selected GPIO output index signal index from GPIO output signal list 0-107. See Table 2-41: GPIO OEN List for SYS_IOMUX (on page 97) for more information."]
pub type DOUT_58_R = crate::FieldReader;
#[doc = "Field `dout_58` writer - The selected output signal for GPIO58. The register value indicates the selected GPIO output index signal index from GPIO output signal list 0-107. See Table 2-41: GPIO OEN List for SYS_IOMUX (on page 97) for more information."]
pub type DOUT_58_W<'a, REG> = crate::FieldWriter<'a, REG, 7>;
#[doc = "Field `dout_59` reader - The selected output signal for GPIO59. The register value indicates the selected GPIO output index signal index from GPIO output signal list 0-107. See Table 2-41: GPIO OEN List for SYS_IOMUX (on page 97) for more information."]
pub type DOUT_59_R = crate::FieldReader;
#[doc = "Field `dout_59` writer - The selected output signal for GPIO59. The register value indicates the selected GPIO output index signal index from GPIO output signal list 0-107. See Table 2-41: GPIO OEN List for SYS_IOMUX (on page 97) for more information."]
pub type DOUT_59_W<'a, REG> = crate::FieldWriter<'a, REG, 7>;
impl R {
    #[doc = "Bits 0:6 - The selected output signal for GPIO56. The register value indicates the selected GPIO output index signal index from GPIO output signal list 0-107. See Table 2-41: GPIO OEN List for SYS_IOMUX (on page 97) for more information."]
    #[inline(always)]
    pub fn dout_56(&self) -> DOUT_56_R {
        DOUT_56_R::new((self.bits & 0x7f) as u8)
    }
    #[doc = "Bits 8:14 - The selected output signal for GPIO57. The register value indicates the selected GPIO output index signal index from GPIO output signal list 0-107. See Table 2-41: GPIO OEN List for SYS_IOMUX (on page 97) for more information."]
    #[inline(always)]
    pub fn dout_57(&self) -> DOUT_57_R {
        DOUT_57_R::new(((self.bits >> 8) & 0x7f) as u8)
    }
    #[doc = "Bits 16:22 - The selected output signal for GPIO58. The register value indicates the selected GPIO output index signal index from GPIO output signal list 0-107. See Table 2-41: GPIO OEN List for SYS_IOMUX (on page 97) for more information."]
    #[inline(always)]
    pub fn dout_58(&self) -> DOUT_58_R {
        DOUT_58_R::new(((self.bits >> 16) & 0x7f) as u8)
    }
    #[doc = "Bits 24:30 - The selected output signal for GPIO59. The register value indicates the selected GPIO output index signal index from GPIO output signal list 0-107. See Table 2-41: GPIO OEN List for SYS_IOMUX (on page 97) for more information."]
    #[inline(always)]
    pub fn dout_59(&self) -> DOUT_59_R {
        DOUT_59_R::new(((self.bits >> 24) & 0x7f) as u8)
    }
}
impl W {
    #[doc = "Bits 0:6 - The selected output signal for GPIO56. The register value indicates the selected GPIO output index signal index from GPIO output signal list 0-107. See Table 2-41: GPIO OEN List for SYS_IOMUX (on page 97) for more information."]
    #[inline(always)]
    #[must_use]
    pub fn dout_56(&mut self) -> DOUT_56_W<GPO_DOUT_56_59_SPEC> {
        DOUT_56_W::new(self, 0)
    }
    #[doc = "Bits 8:14 - The selected output signal for GPIO57. The register value indicates the selected GPIO output index signal index from GPIO output signal list 0-107. See Table 2-41: GPIO OEN List for SYS_IOMUX (on page 97) for more information."]
    #[inline(always)]
    #[must_use]
    pub fn dout_57(&mut self) -> DOUT_57_W<GPO_DOUT_56_59_SPEC> {
        DOUT_57_W::new(self, 8)
    }
    #[doc = "Bits 16:22 - The selected output signal for GPIO58. The register value indicates the selected GPIO output index signal index from GPIO output signal list 0-107. See Table 2-41: GPIO OEN List for SYS_IOMUX (on page 97) for more information."]
    #[inline(always)]
    #[must_use]
    pub fn dout_58(&mut self) -> DOUT_58_W<GPO_DOUT_56_59_SPEC> {
        DOUT_58_W::new(self, 16)
    }
    #[doc = "Bits 24:30 - The selected output signal for GPIO59. The register value indicates the selected GPIO output index signal index from GPIO output signal list 0-107. See Table 2-41: GPIO OEN List for SYS_IOMUX (on page 97) for more information."]
    #[inline(always)]
    #[must_use]
    pub fn dout_59(&mut self) -> DOUT_59_W<GPO_DOUT_56_59_SPEC> {
        DOUT_59_W::new(self, 24)
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
#[doc = "SYS IOMUX CFG SAIF SYSCFG FMUX GPIO 56-59 DOUT\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`gpo_dout_56_59::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`gpo_dout_56_59::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct GPO_DOUT_56_59_SPEC;
impl crate::RegisterSpec for GPO_DOUT_56_59_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`gpo_dout_56_59::R`](R) reader structure"]
impl crate::Readable for GPO_DOUT_56_59_SPEC {}
#[doc = "`write(|w| ..)` method takes [`gpo_dout_56_59::W`](W) writer structure"]
impl crate::Writable for GPO_DOUT_56_59_SPEC {
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets gpo_dout_56_59 to value 0x5800_5657"]
impl crate::Resettable for GPO_DOUT_56_59_SPEC {
    const RESET_VALUE: Self::Ux = 0x5800_5657;
}
