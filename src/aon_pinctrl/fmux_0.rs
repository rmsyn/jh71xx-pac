#[doc = "Register `fmux_0` reader"]
pub type R = crate::R<FMUX_0_SPEC>;
#[doc = "Register `fmux_0` writer"]
pub type W = crate::W<FMUX_0_SPEC>;
#[doc = "Field `gpo_doen_0` reader - The selected OEN signal for GPIO0. The register value indicates the selected GPIO (Output Enable) OEN index from GPIO OEN list 0-5. See Table 2-42: GPIO OEN List for AON_IOMUX for more information."]
pub type GPO_DOEN_0_R = crate::FieldReader;
#[doc = "Field `gpo_doen_0` writer - The selected OEN signal for GPIO0. The register value indicates the selected GPIO (Output Enable) OEN index from GPIO OEN list 0-5. See Table 2-42: GPIO OEN List for AON_IOMUX for more information."]
pub type GPO_DOEN_0_W<'a, REG> = crate::FieldWriter<'a, REG, 3>;
#[doc = "Field `gpo_doen_1` reader - The selected OEN signal for GPIO1. The register value indicates the selected GPIO (Output Enable) OEN index from GPIO OEN list 0-5. See Table 2-42: GPIO OEN List for AON_IOMUX for more information."]
pub type GPO_DOEN_1_R = crate::FieldReader;
#[doc = "Field `gpo_doen_1` writer - The selected OEN signal for GPIO1. The register value indicates the selected GPIO (Output Enable) OEN index from GPIO OEN list 0-5. See Table 2-42: GPIO OEN List for AON_IOMUX for more information."]
pub type GPO_DOEN_1_W<'a, REG> = crate::FieldWriter<'a, REG, 3>;
#[doc = "Field `gpo_doen_2` reader - The selected OEN signal for GPIO2. The register value indicates the selected GPIO (Output Enable) OEN index from GPIO OEN list 0-5. See Table 2-42: GPIO OEN List for AON_IOMUX for more information."]
pub type GPO_DOEN_2_R = crate::FieldReader;
#[doc = "Field `gpo_doen_2` writer - The selected OEN signal for GPIO2. The register value indicates the selected GPIO (Output Enable) OEN index from GPIO OEN list 0-5. See Table 2-42: GPIO OEN List for AON_IOMUX for more information."]
pub type GPO_DOEN_2_W<'a, REG> = crate::FieldWriter<'a, REG, 3>;
#[doc = "Field `gpo_doen_3` reader - The selected OEN signal for GPIO3. The register value indicates the selected GPIO (Output Enable) OEN index from GPIO OEN list 0-5. See Table 2-42: GPIO OEN List for AON_IOMUX for more information."]
pub type GPO_DOEN_3_R = crate::FieldReader;
#[doc = "Field `gpo_doen_3` writer - The selected OEN signal for GPIO3. The register value indicates the selected GPIO (Output Enable) OEN index from GPIO OEN list 0-5. See Table 2-42: GPIO OEN List for AON_IOMUX for more information."]
pub type GPO_DOEN_3_W<'a, REG> = crate::FieldWriter<'a, REG, 3>;
impl R {
    #[doc = "Bits 0:2 - The selected OEN signal for GPIO0. The register value indicates the selected GPIO (Output Enable) OEN index from GPIO OEN list 0-5. See Table 2-42: GPIO OEN List for AON_IOMUX for more information."]
    #[inline(always)]
    pub fn gpo_doen_0(&self) -> GPO_DOEN_0_R {
        GPO_DOEN_0_R::new((self.bits & 7) as u8)
    }
    #[doc = "Bits 8:10 - The selected OEN signal for GPIO1. The register value indicates the selected GPIO (Output Enable) OEN index from GPIO OEN list 0-5. See Table 2-42: GPIO OEN List for AON_IOMUX for more information."]
    #[inline(always)]
    pub fn gpo_doen_1(&self) -> GPO_DOEN_1_R {
        GPO_DOEN_1_R::new(((self.bits >> 8) & 7) as u8)
    }
    #[doc = "Bits 16:18 - The selected OEN signal for GPIO2. The register value indicates the selected GPIO (Output Enable) OEN index from GPIO OEN list 0-5. See Table 2-42: GPIO OEN List for AON_IOMUX for more information."]
    #[inline(always)]
    pub fn gpo_doen_2(&self) -> GPO_DOEN_2_R {
        GPO_DOEN_2_R::new(((self.bits >> 16) & 7) as u8)
    }
    #[doc = "Bits 24:26 - The selected OEN signal for GPIO3. The register value indicates the selected GPIO (Output Enable) OEN index from GPIO OEN list 0-5. See Table 2-42: GPIO OEN List for AON_IOMUX for more information."]
    #[inline(always)]
    pub fn gpo_doen_3(&self) -> GPO_DOEN_3_R {
        GPO_DOEN_3_R::new(((self.bits >> 24) & 7) as u8)
    }
}
impl W {
    #[doc = "Bits 0:2 - The selected OEN signal for GPIO0. The register value indicates the selected GPIO (Output Enable) OEN index from GPIO OEN list 0-5. See Table 2-42: GPIO OEN List for AON_IOMUX for more information."]
    #[inline(always)]
    #[must_use]
    pub fn gpo_doen_0(&mut self) -> GPO_DOEN_0_W<FMUX_0_SPEC> {
        GPO_DOEN_0_W::new(self, 0)
    }
    #[doc = "Bits 8:10 - The selected OEN signal for GPIO1. The register value indicates the selected GPIO (Output Enable) OEN index from GPIO OEN list 0-5. See Table 2-42: GPIO OEN List for AON_IOMUX for more information."]
    #[inline(always)]
    #[must_use]
    pub fn gpo_doen_1(&mut self) -> GPO_DOEN_1_W<FMUX_0_SPEC> {
        GPO_DOEN_1_W::new(self, 8)
    }
    #[doc = "Bits 16:18 - The selected OEN signal for GPIO2. The register value indicates the selected GPIO (Output Enable) OEN index from GPIO OEN list 0-5. See Table 2-42: GPIO OEN List for AON_IOMUX for more information."]
    #[inline(always)]
    #[must_use]
    pub fn gpo_doen_2(&mut self) -> GPO_DOEN_2_W<FMUX_0_SPEC> {
        GPO_DOEN_2_W::new(self, 16)
    }
    #[doc = "Bits 24:26 - The selected OEN signal for GPIO3. The register value indicates the selected GPIO (Output Enable) OEN index from GPIO OEN list 0-5. See Table 2-42: GPIO OEN List for AON_IOMUX for more information."]
    #[inline(always)]
    #[must_use]
    pub fn gpo_doen_3(&mut self) -> GPO_DOEN_3_W<FMUX_0_SPEC> {
        GPO_DOEN_3_W::new(self, 24)
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
#[doc = "AON IOMUX CFG SAIF SYSCFG FMUX 0\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`fmux_0::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`fmux_0::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct FMUX_0_SPEC;
impl crate::RegisterSpec for FMUX_0_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`fmux_0::R`](R) reader structure"]
impl crate::Readable for FMUX_0_SPEC {}
#[doc = "`write(|w| ..)` method takes [`fmux_0::W`](W) writer structure"]
impl crate::Writable for FMUX_0_SPEC {
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets fmux_0 to value 0"]
impl crate::Resettable for FMUX_0_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
