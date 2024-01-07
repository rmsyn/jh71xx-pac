#[doc = "Register `gpo_doen_0` reader"]
pub type R = crate::R<GPO_DOEN_0_SPEC>;
#[doc = "Register `gpo_doen_0` writer"]
pub type W = crate::W<GPO_DOEN_0_SPEC>;
#[doc = "Field `doen_0` reader - The selected OEN signal for GPIO0. The register value indicates the selected GPIO (Output Enable) OEN index from GPIO OEN list 0-49. See Table 2-41: GPIO OEN List for SYS_IOMUX (on page 97) for more information."]
pub type DOEN_0_R = crate::FieldReader;
#[doc = "Field `doen_0` writer - The selected OEN signal for GPIO0. The register value indicates the selected GPIO (Output Enable) OEN index from GPIO OEN list 0-49. See Table 2-41: GPIO OEN List for SYS_IOMUX (on page 97) for more information."]
pub type DOEN_0_W<'a, REG> = crate::FieldWriter<'a, REG, 6>;
#[doc = "Field `doen_1` reader - The selected OEN signal for GPIO1. The register value indicates the selected GPIO (Output Enable) OEN index from GPIO OEN list 0-49. See Table 2-41: GPIO OEN List for SYS_IOMUX (on page 97) for more information."]
pub type DOEN_1_R = crate::FieldReader;
#[doc = "Field `doen_1` writer - The selected OEN signal for GPIO1. The register value indicates the selected GPIO (Output Enable) OEN index from GPIO OEN list 0-49. See Table 2-41: GPIO OEN List for SYS_IOMUX (on page 97) for more information."]
pub type DOEN_1_W<'a, REG> = crate::FieldWriter<'a, REG, 6>;
#[doc = "Field `doen_2` reader - The selected OEN signal for GPIO2. The register value indicates the selected GPIO (Output Enable) OEN index from GPIO OEN list 0-49. See Table 2-41: GPIO OEN List for SYS_IOMUX (on page 97) for more information."]
pub type DOEN_2_R = crate::FieldReader;
#[doc = "Field `doen_2` writer - The selected OEN signal for GPIO2. The register value indicates the selected GPIO (Output Enable) OEN index from GPIO OEN list 0-49. See Table 2-41: GPIO OEN List for SYS_IOMUX (on page 97) for more information."]
pub type DOEN_2_W<'a, REG> = crate::FieldWriter<'a, REG, 6>;
#[doc = "Field `doen_3` reader - The selected OEN signal for GPIO3. The register value indicates the selected GPIO (Output Enable) OEN index from GPIO OEN list 0-49. See Table 2-41: GPIO OEN List for SYS_IOMUX (on page 97) for more information."]
pub type DOEN_3_R = crate::FieldReader;
#[doc = "Field `doen_3` writer - The selected OEN signal for GPIO3. The register value indicates the selected GPIO (Output Enable) OEN index from GPIO OEN list 0-49. See Table 2-41: GPIO OEN List for SYS_IOMUX (on page 97) for more information."]
pub type DOEN_3_W<'a, REG> = crate::FieldWriter<'a, REG, 6>;
impl R {
    #[doc = "Bits 0:5 - The selected OEN signal for GPIO0. The register value indicates the selected GPIO (Output Enable) OEN index from GPIO OEN list 0-49. See Table 2-41: GPIO OEN List for SYS_IOMUX (on page 97) for more information."]
    #[inline(always)]
    pub fn doen_0(&self) -> DOEN_0_R {
        DOEN_0_R::new((self.bits & 0x3f) as u8)
    }
    #[doc = "Bits 8:13 - The selected OEN signal for GPIO1. The register value indicates the selected GPIO (Output Enable) OEN index from GPIO OEN list 0-49. See Table 2-41: GPIO OEN List for SYS_IOMUX (on page 97) for more information."]
    #[inline(always)]
    pub fn doen_1(&self) -> DOEN_1_R {
        DOEN_1_R::new(((self.bits >> 8) & 0x3f) as u8)
    }
    #[doc = "Bits 16:21 - The selected OEN signal for GPIO2. The register value indicates the selected GPIO (Output Enable) OEN index from GPIO OEN list 0-49. See Table 2-41: GPIO OEN List for SYS_IOMUX (on page 97) for more information."]
    #[inline(always)]
    pub fn doen_2(&self) -> DOEN_2_R {
        DOEN_2_R::new(((self.bits >> 16) & 0x3f) as u8)
    }
    #[doc = "Bits 24:29 - The selected OEN signal for GPIO3. The register value indicates the selected GPIO (Output Enable) OEN index from GPIO OEN list 0-49. See Table 2-41: GPIO OEN List for SYS_IOMUX (on page 97) for more information."]
    #[inline(always)]
    pub fn doen_3(&self) -> DOEN_3_R {
        DOEN_3_R::new(((self.bits >> 24) & 0x3f) as u8)
    }
}
impl W {
    #[doc = "Bits 0:5 - The selected OEN signal for GPIO0. The register value indicates the selected GPIO (Output Enable) OEN index from GPIO OEN list 0-49. See Table 2-41: GPIO OEN List for SYS_IOMUX (on page 97) for more information."]
    #[inline(always)]
    #[must_use]
    pub fn doen_0(&mut self) -> DOEN_0_W<GPO_DOEN_0_SPEC> {
        DOEN_0_W::new(self, 0)
    }
    #[doc = "Bits 8:13 - The selected OEN signal for GPIO1. The register value indicates the selected GPIO (Output Enable) OEN index from GPIO OEN list 0-49. See Table 2-41: GPIO OEN List for SYS_IOMUX (on page 97) for more information."]
    #[inline(always)]
    #[must_use]
    pub fn doen_1(&mut self) -> DOEN_1_W<GPO_DOEN_0_SPEC> {
        DOEN_1_W::new(self, 8)
    }
    #[doc = "Bits 16:21 - The selected OEN signal for GPIO2. The register value indicates the selected GPIO (Output Enable) OEN index from GPIO OEN list 0-49. See Table 2-41: GPIO OEN List for SYS_IOMUX (on page 97) for more information."]
    #[inline(always)]
    #[must_use]
    pub fn doen_2(&mut self) -> DOEN_2_W<GPO_DOEN_0_SPEC> {
        DOEN_2_W::new(self, 16)
    }
    #[doc = "Bits 24:29 - The selected OEN signal for GPIO3. The register value indicates the selected GPIO (Output Enable) OEN index from GPIO OEN list 0-49. See Table 2-41: GPIO OEN List for SYS_IOMUX (on page 97) for more information."]
    #[inline(always)]
    #[must_use]
    pub fn doen_3(&mut self) -> DOEN_3_W<GPO_DOEN_0_SPEC> {
        DOEN_3_W::new(self, 24)
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
#[doc = "SYS IOMUX CFG SAIF SYSCFG FMUX 0 DOEN\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`gpo_doen_0::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`gpo_doen_0::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct GPO_DOEN_0_SPEC;
impl crate::RegisterSpec for GPO_DOEN_0_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`gpo_doen_0::R`](R) reader structure"]
impl crate::Readable for GPO_DOEN_0_SPEC {}
#[doc = "`write(|w| ..)` method takes [`gpo_doen_0::W`](W) writer structure"]
impl crate::Writable for GPO_DOEN_0_SPEC {
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets gpo_doen_0 to value 0x0801_0101"]
impl crate::Resettable for GPO_DOEN_0_SPEC {
    const RESET_VALUE: Self::Ux = 0x0801_0101;
}
