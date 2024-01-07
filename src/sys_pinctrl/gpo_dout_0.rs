#[doc = "Register `gpo_dout_0` reader"]
pub type R = crate::R<GPO_DOUT_0_SPEC>;
#[doc = "Register `gpo_dout_0` writer"]
pub type W = crate::W<GPO_DOUT_0_SPEC>;
#[doc = "Field `dout_0` reader - The selected output signal for GPIO0. The register value indicates the selected GPIO output index signal index from GPIO output signal list 0-107. See Table 2-41: GPIO OEN List for SYS_IOMUX (on page 97) for more information."]
pub type DOUT_0_R = crate::FieldReader;
#[doc = "Field `dout_0` writer - The selected output signal for GPIO0. The register value indicates the selected GPIO output index signal index from GPIO output signal list 0-107. See Table 2-41: GPIO OEN List for SYS_IOMUX (on page 97) for more information."]
pub type DOUT_0_W<'a, REG> = crate::FieldWriter<'a, REG, 7>;
#[doc = "Field `dout_1` reader - The selected output signal for GPIO1. The register value indicates the selected GPIO output index signal index from GPIO output signal list 0-107. See Table 2-41: GPIO OEN List for SYS_IOMUX (on page 97) for more information."]
pub type DOUT_1_R = crate::FieldReader;
#[doc = "Field `dout_1` writer - The selected output signal for GPIO1. The register value indicates the selected GPIO output index signal index from GPIO output signal list 0-107. See Table 2-41: GPIO OEN List for SYS_IOMUX (on page 97) for more information."]
pub type DOUT_1_W<'a, REG> = crate::FieldWriter<'a, REG, 7>;
#[doc = "Field `dout_2` reader - The selected output signal for GPIO2. The register value indicates the selected GPIO output index signal index from GPIO output signal list 0-107. See Table 2-41: GPIO OEN List for SYS_IOMUX (on page 97) for more information."]
pub type DOUT_2_R = crate::FieldReader;
#[doc = "Field `dout_2` writer - The selected output signal for GPIO2. The register value indicates the selected GPIO output index signal index from GPIO output signal list 0-107. See Table 2-41: GPIO OEN List for SYS_IOMUX (on page 97) for more information."]
pub type DOUT_2_W<'a, REG> = crate::FieldWriter<'a, REG, 7>;
#[doc = "Field `dout_3` reader - The selected output signal for GPIO3. The register value indicates the selected GPIO output index signal index from GPIO output signal list 0-107. See Table 2-41: GPIO OEN List for SYS_IOMUX (on page 97) for more information."]
pub type DOUT_3_R = crate::FieldReader;
#[doc = "Field `dout_3` writer - The selected output signal for GPIO3. The register value indicates the selected GPIO output index signal index from GPIO output signal list 0-107. See Table 2-41: GPIO OEN List for SYS_IOMUX (on page 97) for more information."]
pub type DOUT_3_W<'a, REG> = crate::FieldWriter<'a, REG, 7>;
impl R {
    #[doc = "Bits 0:6 - The selected output signal for GPIO0. The register value indicates the selected GPIO output index signal index from GPIO output signal list 0-107. See Table 2-41: GPIO OEN List for SYS_IOMUX (on page 97) for more information."]
    #[inline(always)]
    pub fn dout_0(&self) -> DOUT_0_R {
        DOUT_0_R::new((self.bits & 0x7f) as u8)
    }
    #[doc = "Bits 8:14 - The selected output signal for GPIO1. The register value indicates the selected GPIO output index signal index from GPIO output signal list 0-107. See Table 2-41: GPIO OEN List for SYS_IOMUX (on page 97) for more information."]
    #[inline(always)]
    pub fn dout_1(&self) -> DOUT_1_R {
        DOUT_1_R::new(((self.bits >> 8) & 0x7f) as u8)
    }
    #[doc = "Bits 16:22 - The selected output signal for GPIO2. The register value indicates the selected GPIO output index signal index from GPIO output signal list 0-107. See Table 2-41: GPIO OEN List for SYS_IOMUX (on page 97) for more information."]
    #[inline(always)]
    pub fn dout_2(&self) -> DOUT_2_R {
        DOUT_2_R::new(((self.bits >> 16) & 0x7f) as u8)
    }
    #[doc = "Bits 24:30 - The selected output signal for GPIO3. The register value indicates the selected GPIO output index signal index from GPIO output signal list 0-107. See Table 2-41: GPIO OEN List for SYS_IOMUX (on page 97) for more information."]
    #[inline(always)]
    pub fn dout_3(&self) -> DOUT_3_R {
        DOUT_3_R::new(((self.bits >> 24) & 0x7f) as u8)
    }
}
impl W {
    #[doc = "Bits 0:6 - The selected output signal for GPIO0. The register value indicates the selected GPIO output index signal index from GPIO output signal list 0-107. See Table 2-41: GPIO OEN List for SYS_IOMUX (on page 97) for more information."]
    #[inline(always)]
    #[must_use]
    pub fn dout_0(&mut self) -> DOUT_0_W<GPO_DOUT_0_SPEC> {
        DOUT_0_W::new(self, 0)
    }
    #[doc = "Bits 8:14 - The selected output signal for GPIO1. The register value indicates the selected GPIO output index signal index from GPIO output signal list 0-107. See Table 2-41: GPIO OEN List for SYS_IOMUX (on page 97) for more information."]
    #[inline(always)]
    #[must_use]
    pub fn dout_1(&mut self) -> DOUT_1_W<GPO_DOUT_0_SPEC> {
        DOUT_1_W::new(self, 8)
    }
    #[doc = "Bits 16:22 - The selected output signal for GPIO2. The register value indicates the selected GPIO output index signal index from GPIO output signal list 0-107. See Table 2-41: GPIO OEN List for SYS_IOMUX (on page 97) for more information."]
    #[inline(always)]
    #[must_use]
    pub fn dout_2(&mut self) -> DOUT_2_W<GPO_DOUT_0_SPEC> {
        DOUT_2_W::new(self, 16)
    }
    #[doc = "Bits 24:30 - The selected output signal for GPIO3. The register value indicates the selected GPIO output index signal index from GPIO output signal list 0-107. See Table 2-41: GPIO OEN List for SYS_IOMUX (on page 97) for more information."]
    #[inline(always)]
    #[must_use]
    pub fn dout_3(&mut self) -> DOUT_3_W<GPO_DOUT_0_SPEC> {
        DOUT_3_W::new(self, 24)
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
#[doc = "SYS IOMUX CFG SAIF SYSCFG FMUX GPIO 0-3 DOUT\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`gpo_dout_0::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`gpo_dout_0::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct GPO_DOUT_0_SPEC;
impl crate::RegisterSpec for GPO_DOUT_0_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`gpo_dout_0::R`](R) reader structure"]
impl crate::Readable for GPO_DOUT_0_SPEC {}
#[doc = "`write(|w| ..)` method takes [`gpo_dout_0::W`](W) writer structure"]
impl crate::Writable for GPO_DOUT_0_SPEC {
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets gpo_dout_0 to value 0x1600_0000"]
impl crate::Resettable for GPO_DOUT_0_SPEC {
    const RESET_VALUE: Self::Ux = 0x1600_0000;
}
