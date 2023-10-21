#[doc = "Register `sys_iomux_cfgsaif_syscfg_fmux26` reader"]
pub type R = crate::R<SYS_IOMUX_CFGSAIF_SYSCFG_FMUX26_SPEC>;
#[doc = "Register `sys_iomux_cfgsaif_syscfg_fmux26` writer"]
pub type W = crate::W<SYS_IOMUX_CFGSAIF_SYSCFG_FMUX26_SPEC>;
#[doc = "Field `sys_iomux_gpo104_doen_cfg` reader - The selected OEN signal for GPIO104. The register value indicates the selected GPIO (Output Enable) OEN index from GPIO OEN list 0-49. See Table 2-41: GPIO OEN List for SYS_IOMUX (on page 97) for more information."]
pub type SYS_IOMUX_GPO104_DOEN_CFG_R = crate::FieldReader;
#[doc = "Field `sys_iomux_gpo104_doen_cfg` writer - The selected OEN signal for GPIO104. The register value indicates the selected GPIO (Output Enable) OEN index from GPIO OEN list 0-49. See Table 2-41: GPIO OEN List for SYS_IOMUX (on page 97) for more information."]
pub type SYS_IOMUX_GPO104_DOEN_CFG_W<'a, REG, const O: u8> = crate::FieldWriter<'a, REG, 6, O>;
#[doc = "Field `sys_iomux_gpo105_doen_cfg` reader - The selected OEN signal for GPIO105. The register value indicates the selected GPIO (Output Enable) OEN index from GPIO OEN list 0-49. See Table 2-41: GPIO OEN List for SYS_IOMUX (on page 97) for more information."]
pub type SYS_IOMUX_GPO105_DOEN_CFG_R = crate::FieldReader;
#[doc = "Field `sys_iomux_gpo105_doen_cfg` writer - The selected OEN signal for GPIO105. The register value indicates the selected GPIO (Output Enable) OEN index from GPIO OEN list 0-49. See Table 2-41: GPIO OEN List for SYS_IOMUX (on page 97) for more information."]
pub type SYS_IOMUX_GPO105_DOEN_CFG_W<'a, REG, const O: u8> = crate::FieldWriter<'a, REG, 6, O>;
#[doc = "Field `sys_iomux_gpo106_doen_cfg` reader - The selected OEN signal for GPIO106. The register value indicates the selected GPIO (Output Enable) OEN index from GPIO OEN list 0-49. See Table 2-41: GPIO OEN List for SYS_IOMUX (on page 97) for more information."]
pub type SYS_IOMUX_GPO106_DOEN_CFG_R = crate::FieldReader;
#[doc = "Field `sys_iomux_gpo106_doen_cfg` writer - The selected OEN signal for GPIO106. The register value indicates the selected GPIO (Output Enable) OEN index from GPIO OEN list 0-49. See Table 2-41: GPIO OEN List for SYS_IOMUX (on page 97) for more information."]
pub type SYS_IOMUX_GPO106_DOEN_CFG_W<'a, REG, const O: u8> = crate::FieldWriter<'a, REG, 6, O>;
#[doc = "Field `sys_iomux_gpo107_doen_cfg` reader - The selected OEN signal for GPIO107. The register value indicates the selected GPIO (Output Enable) OEN index from GPIO OEN list 0-49. See Table 2-41: GPIO OEN List for SYS_IOMUX (on page 97) for more information."]
pub type SYS_IOMUX_GPO107_DOEN_CFG_R = crate::FieldReader;
#[doc = "Field `sys_iomux_gpo107_doen_cfg` writer - The selected OEN signal for GPIO107. The register value indicates the selected GPIO (Output Enable) OEN index from GPIO OEN list 0-49. See Table 2-41: GPIO OEN List for SYS_IOMUX (on page 97) for more information."]
pub type SYS_IOMUX_GPO107_DOEN_CFG_W<'a, REG, const O: u8> = crate::FieldWriter<'a, REG, 6, O>;
impl R {
    #[doc = "Bits 0:5 - The selected OEN signal for GPIO104. The register value indicates the selected GPIO (Output Enable) OEN index from GPIO OEN list 0-49. See Table 2-41: GPIO OEN List for SYS_IOMUX (on page 97) for more information."]
    #[inline(always)]
    pub fn sys_iomux_gpo104_doen_cfg(&self) -> SYS_IOMUX_GPO104_DOEN_CFG_R {
        SYS_IOMUX_GPO104_DOEN_CFG_R::new((self.bits & 0x3f) as u8)
    }
    #[doc = "Bits 8:13 - The selected OEN signal for GPIO105. The register value indicates the selected GPIO (Output Enable) OEN index from GPIO OEN list 0-49. See Table 2-41: GPIO OEN List for SYS_IOMUX (on page 97) for more information."]
    #[inline(always)]
    pub fn sys_iomux_gpo105_doen_cfg(&self) -> SYS_IOMUX_GPO105_DOEN_CFG_R {
        SYS_IOMUX_GPO105_DOEN_CFG_R::new(((self.bits >> 8) & 0x3f) as u8)
    }
    #[doc = "Bits 16:21 - The selected OEN signal for GPIO106. The register value indicates the selected GPIO (Output Enable) OEN index from GPIO OEN list 0-49. See Table 2-41: GPIO OEN List for SYS_IOMUX (on page 97) for more information."]
    #[inline(always)]
    pub fn sys_iomux_gpo106_doen_cfg(&self) -> SYS_IOMUX_GPO106_DOEN_CFG_R {
        SYS_IOMUX_GPO106_DOEN_CFG_R::new(((self.bits >> 16) & 0x3f) as u8)
    }
    #[doc = "Bits 24:29 - The selected OEN signal for GPIO107. The register value indicates the selected GPIO (Output Enable) OEN index from GPIO OEN list 0-49. See Table 2-41: GPIO OEN List for SYS_IOMUX (on page 97) for more information."]
    #[inline(always)]
    pub fn sys_iomux_gpo107_doen_cfg(&self) -> SYS_IOMUX_GPO107_DOEN_CFG_R {
        SYS_IOMUX_GPO107_DOEN_CFG_R::new(((self.bits >> 24) & 0x3f) as u8)
    }
}
impl W {
    #[doc = "Bits 0:5 - The selected OEN signal for GPIO104. The register value indicates the selected GPIO (Output Enable) OEN index from GPIO OEN list 0-49. See Table 2-41: GPIO OEN List for SYS_IOMUX (on page 97) for more information."]
    #[inline(always)]
    #[must_use]
    pub fn sys_iomux_gpo104_doen_cfg(
        &mut self,
    ) -> SYS_IOMUX_GPO104_DOEN_CFG_W<SYS_IOMUX_CFGSAIF_SYSCFG_FMUX26_SPEC, 0> {
        SYS_IOMUX_GPO104_DOEN_CFG_W::new(self)
    }
    #[doc = "Bits 8:13 - The selected OEN signal for GPIO105. The register value indicates the selected GPIO (Output Enable) OEN index from GPIO OEN list 0-49. See Table 2-41: GPIO OEN List for SYS_IOMUX (on page 97) for more information."]
    #[inline(always)]
    #[must_use]
    pub fn sys_iomux_gpo105_doen_cfg(
        &mut self,
    ) -> SYS_IOMUX_GPO105_DOEN_CFG_W<SYS_IOMUX_CFGSAIF_SYSCFG_FMUX26_SPEC, 8> {
        SYS_IOMUX_GPO105_DOEN_CFG_W::new(self)
    }
    #[doc = "Bits 16:21 - The selected OEN signal for GPIO106. The register value indicates the selected GPIO (Output Enable) OEN index from GPIO OEN list 0-49. See Table 2-41: GPIO OEN List for SYS_IOMUX (on page 97) for more information."]
    #[inline(always)]
    #[must_use]
    pub fn sys_iomux_gpo106_doen_cfg(
        &mut self,
    ) -> SYS_IOMUX_GPO106_DOEN_CFG_W<SYS_IOMUX_CFGSAIF_SYSCFG_FMUX26_SPEC, 16> {
        SYS_IOMUX_GPO106_DOEN_CFG_W::new(self)
    }
    #[doc = "Bits 24:29 - The selected OEN signal for GPIO107. The register value indicates the selected GPIO (Output Enable) OEN index from GPIO OEN list 0-49. See Table 2-41: GPIO OEN List for SYS_IOMUX (on page 97) for more information."]
    #[inline(always)]
    #[must_use]
    pub fn sys_iomux_gpo107_doen_cfg(
        &mut self,
    ) -> SYS_IOMUX_GPO107_DOEN_CFG_W<SYS_IOMUX_CFGSAIF_SYSCFG_FMUX26_SPEC, 24> {
        SYS_IOMUX_GPO107_DOEN_CFG_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.bits = bits;
        self
    }
}
#[doc = "SYS IOMUX CFG SAIF SYSCFG FMUX 26\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`sys_iomux_cfgsaif_syscfg_fmux26::R`](R).  You can [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`sys_iomux_cfgsaif_syscfg_fmux26::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct SYS_IOMUX_CFGSAIF_SYSCFG_FMUX26_SPEC;
impl crate::RegisterSpec for SYS_IOMUX_CFGSAIF_SYSCFG_FMUX26_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`sys_iomux_cfgsaif_syscfg_fmux26::R`](R) reader structure"]
impl crate::Readable for SYS_IOMUX_CFGSAIF_SYSCFG_FMUX26_SPEC {}
#[doc = "`write(|w| ..)` method takes [`sys_iomux_cfgsaif_syscfg_fmux26::W`](W) writer structure"]
impl crate::Writable for SYS_IOMUX_CFGSAIF_SYSCFG_FMUX26_SPEC {
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
