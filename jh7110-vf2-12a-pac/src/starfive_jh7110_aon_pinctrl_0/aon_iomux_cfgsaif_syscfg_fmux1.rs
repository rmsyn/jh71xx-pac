#[doc = "Register `aon_iomux_cfgsaif_syscfg_fmux1` reader"]
pub type R = crate::R<AON_IOMUX_CFGSAIF_SYSCFG_FMUX1_SPEC>;
#[doc = "Register `aon_iomux_cfgsaif_syscfg_fmux1` writer"]
pub type W = crate::W<AON_IOMUX_CFGSAIF_SYSCFG_FMUX1_SPEC>;
#[doc = "Field `aon_iomux_gpo0_dout_cfg` reader - The selected OEN signal for GPIO0. The register value indicates the selected GPIO output signal list 0-9. See Table 2-42: GPIO OEN List for AON_IOMUX for more information."]
pub type AON_IOMUX_GPO0_DOUT_CFG_R = crate::FieldReader;
#[doc = "Field `aon_iomux_gpo0_dout_cfg` writer - The selected OEN signal for GPIO0. The register value indicates the selected GPIO output signal list 0-9. See Table 2-42: GPIO OEN List for AON_IOMUX for more information."]
pub type AON_IOMUX_GPO0_DOUT_CFG_W<'a, REG, const O: u8> = crate::FieldWriter<'a, REG, 4, O>;
#[doc = "Field `aon_iomux_gpo1_dout_cfg` reader - The selected OEN signal for GPIO1. The register value indicates the selected GPIO output signal list 0-9. See Table 2-42: GPIO OEN List for AON_IOMUX for more information."]
pub type AON_IOMUX_GPO1_DOUT_CFG_R = crate::FieldReader;
#[doc = "Field `aon_iomux_gpo1_dout_cfg` writer - The selected OEN signal for GPIO1. The register value indicates the selected GPIO output signal list 0-9. See Table 2-42: GPIO OEN List for AON_IOMUX for more information."]
pub type AON_IOMUX_GPO1_DOUT_CFG_W<'a, REG, const O: u8> = crate::FieldWriter<'a, REG, 4, O>;
#[doc = "Field `aon_iomux_gpo2_dout_cfg` reader - The selected OEN signal for GPIO2. The register value indicates the selected GPIO output signal list 0-9. See Table 2-42: GPIO OEN List for AON_IOMUX for more information."]
pub type AON_IOMUX_GPO2_DOUT_CFG_R = crate::FieldReader;
#[doc = "Field `aon_iomux_gpo2_dout_cfg` writer - The selected OEN signal for GPIO2. The register value indicates the selected GPIO output signal list 0-9. See Table 2-42: GPIO OEN List for AON_IOMUX for more information."]
pub type AON_IOMUX_GPO2_DOUT_CFG_W<'a, REG, const O: u8> = crate::FieldWriter<'a, REG, 4, O>;
#[doc = "Field `aon_iomux_gpo3_dout_cfg` reader - The selected OEN signal for GPIO3. The register value indicates the selected GPIO output signal list 0-9. See Table 2-42: GPIO OEN List for AON_IOMUX for more information."]
pub type AON_IOMUX_GPO3_DOUT_CFG_R = crate::FieldReader;
#[doc = "Field `aon_iomux_gpo3_dout_cfg` writer - The selected OEN signal for GPIO3. The register value indicates the selected GPIO output signal list 0-9. See Table 2-42: GPIO OEN List for AON_IOMUX for more information."]
pub type AON_IOMUX_GPO3_DOUT_CFG_W<'a, REG, const O: u8> = crate::FieldWriter<'a, REG, 4, O>;
impl R {
    #[doc = "Bits 0:3 - The selected OEN signal for GPIO0. The register value indicates the selected GPIO output signal list 0-9. See Table 2-42: GPIO OEN List for AON_IOMUX for more information."]
    #[inline(always)]
    pub fn aon_iomux_gpo0_dout_cfg(&self) -> AON_IOMUX_GPO0_DOUT_CFG_R {
        AON_IOMUX_GPO0_DOUT_CFG_R::new((self.bits & 0x0f) as u8)
    }
    #[doc = "Bits 8:11 - The selected OEN signal for GPIO1. The register value indicates the selected GPIO output signal list 0-9. See Table 2-42: GPIO OEN List for AON_IOMUX for more information."]
    #[inline(always)]
    pub fn aon_iomux_gpo1_dout_cfg(&self) -> AON_IOMUX_GPO1_DOUT_CFG_R {
        AON_IOMUX_GPO1_DOUT_CFG_R::new(((self.bits >> 8) & 0x0f) as u8)
    }
    #[doc = "Bits 16:19 - The selected OEN signal for GPIO2. The register value indicates the selected GPIO output signal list 0-9. See Table 2-42: GPIO OEN List for AON_IOMUX for more information."]
    #[inline(always)]
    pub fn aon_iomux_gpo2_dout_cfg(&self) -> AON_IOMUX_GPO2_DOUT_CFG_R {
        AON_IOMUX_GPO2_DOUT_CFG_R::new(((self.bits >> 16) & 0x0f) as u8)
    }
    #[doc = "Bits 24:27 - The selected OEN signal for GPIO3. The register value indicates the selected GPIO output signal list 0-9. See Table 2-42: GPIO OEN List for AON_IOMUX for more information."]
    #[inline(always)]
    pub fn aon_iomux_gpo3_dout_cfg(&self) -> AON_IOMUX_GPO3_DOUT_CFG_R {
        AON_IOMUX_GPO3_DOUT_CFG_R::new(((self.bits >> 24) & 0x0f) as u8)
    }
}
impl W {
    #[doc = "Bits 0:3 - The selected OEN signal for GPIO0. The register value indicates the selected GPIO output signal list 0-9. See Table 2-42: GPIO OEN List for AON_IOMUX for more information."]
    #[inline(always)]
    #[must_use]
    pub fn aon_iomux_gpo0_dout_cfg(
        &mut self,
    ) -> AON_IOMUX_GPO0_DOUT_CFG_W<AON_IOMUX_CFGSAIF_SYSCFG_FMUX1_SPEC, 0> {
        AON_IOMUX_GPO0_DOUT_CFG_W::new(self)
    }
    #[doc = "Bits 8:11 - The selected OEN signal for GPIO1. The register value indicates the selected GPIO output signal list 0-9. See Table 2-42: GPIO OEN List for AON_IOMUX for more information."]
    #[inline(always)]
    #[must_use]
    pub fn aon_iomux_gpo1_dout_cfg(
        &mut self,
    ) -> AON_IOMUX_GPO1_DOUT_CFG_W<AON_IOMUX_CFGSAIF_SYSCFG_FMUX1_SPEC, 8> {
        AON_IOMUX_GPO1_DOUT_CFG_W::new(self)
    }
    #[doc = "Bits 16:19 - The selected OEN signal for GPIO2. The register value indicates the selected GPIO output signal list 0-9. See Table 2-42: GPIO OEN List for AON_IOMUX for more information."]
    #[inline(always)]
    #[must_use]
    pub fn aon_iomux_gpo2_dout_cfg(
        &mut self,
    ) -> AON_IOMUX_GPO2_DOUT_CFG_W<AON_IOMUX_CFGSAIF_SYSCFG_FMUX1_SPEC, 16> {
        AON_IOMUX_GPO2_DOUT_CFG_W::new(self)
    }
    #[doc = "Bits 24:27 - The selected OEN signal for GPIO3. The register value indicates the selected GPIO output signal list 0-9. See Table 2-42: GPIO OEN List for AON_IOMUX for more information."]
    #[inline(always)]
    #[must_use]
    pub fn aon_iomux_gpo3_dout_cfg(
        &mut self,
    ) -> AON_IOMUX_GPO3_DOUT_CFG_W<AON_IOMUX_CFGSAIF_SYSCFG_FMUX1_SPEC, 24> {
        AON_IOMUX_GPO3_DOUT_CFG_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.bits = bits;
        self
    }
}
#[doc = "AON IOMUX CFG SAIF SYSCFG FMUX 1\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`aon_iomux_cfgsaif_syscfg_fmux1::R`](R).  You can [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`aon_iomux_cfgsaif_syscfg_fmux1::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct AON_IOMUX_CFGSAIF_SYSCFG_FMUX1_SPEC;
impl crate::RegisterSpec for AON_IOMUX_CFGSAIF_SYSCFG_FMUX1_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`aon_iomux_cfgsaif_syscfg_fmux1::R`](R) reader structure"]
impl crate::Readable for AON_IOMUX_CFGSAIF_SYSCFG_FMUX1_SPEC {}
#[doc = "`write(|w| ..)` method takes [`aon_iomux_cfgsaif_syscfg_fmux1::W`](W) writer structure"]
impl crate::Writable for AON_IOMUX_CFGSAIF_SYSCFG_FMUX1_SPEC {
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
