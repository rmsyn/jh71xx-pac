#[doc = "Register `sys_iomux_cfgsaif_syscfg692` reader"]
pub type R = crate::R<SYS_IOMUX_CFGSAIF_SYSCFG692_SPEC>;
#[doc = "Register `sys_iomux_cfgsaif_syscfg692` writer"]
pub type W = crate::W<SYS_IOMUX_CFGSAIF_SYSCFG692_SPEC>;
#[doc = "Field `u0_dom_isp_top_u0_vin_dvp_data_c5_func_sel` reader - Function Selector of DVP_DATA\\[idx\\], see Function 2 for more information"]
pub type U0_DOM_ISP_TOP_U0_VIN_DVP_DATA_C5_FUNC_SEL_R = crate::FieldReader;
#[doc = "Field `u0_dom_isp_top_u0_vin_dvp_data_c5_func_sel` writer - Function Selector of DVP_DATA\\[idx\\], see Function 2 for more information"]
pub type U0_DOM_ISP_TOP_U0_VIN_DVP_DATA_C5_FUNC_SEL_W<'a, REG, const O: u8> =
    crate::FieldWriter<'a, REG, 3, O>;
#[doc = "Field `u0_dom_isp_top_u0_vin_dvp_data_c6_func_sel` reader - Function Selector of DVP_DATA\\[idx\\], see Function 2 for more information"]
pub type U0_DOM_ISP_TOP_U0_VIN_DVP_DATA_C6_FUNC_SEL_R = crate::FieldReader;
#[doc = "Field `u0_dom_isp_top_u0_vin_dvp_data_c6_func_sel` writer - Function Selector of DVP_DATA\\[idx\\], see Function 2 for more information"]
pub type U0_DOM_ISP_TOP_U0_VIN_DVP_DATA_C6_FUNC_SEL_W<'a, REG, const O: u8> =
    crate::FieldWriter<'a, REG, 3, O>;
#[doc = "Field `u0_dom_isp_top_u0_vin_dvp_data_c7_func_sel` reader - Function Selector of DVP_DATA\\[idx\\], see Function 2 for more information"]
pub type U0_DOM_ISP_TOP_U0_VIN_DVP_DATA_C7_FUNC_SEL_R = crate::FieldReader;
#[doc = "Field `u0_dom_isp_top_u0_vin_dvp_data_c7_func_sel` writer - Function Selector of DVP_DATA\\[idx\\], see Function 2 for more information"]
pub type U0_DOM_ISP_TOP_U0_VIN_DVP_DATA_C7_FUNC_SEL_W<'a, REG, const O: u8> =
    crate::FieldWriter<'a, REG, 3, O>;
#[doc = "Field `u0_dom_isp_top_u0_vin_dvp_data_c8_func_sel` reader - Function Selector of DVP_DATA\\[idx\\], see Function 2 for more information"]
pub type U0_DOM_ISP_TOP_U0_VIN_DVP_DATA_C8_FUNC_SEL_R = crate::FieldReader;
#[doc = "Field `u0_dom_isp_top_u0_vin_dvp_data_c8_func_sel` writer - Function Selector of DVP_DATA\\[idx\\], see Function 2 for more information"]
pub type U0_DOM_ISP_TOP_U0_VIN_DVP_DATA_C8_FUNC_SEL_W<'a, REG, const O: u8> =
    crate::FieldWriter<'a, REG, 3, O>;
#[doc = "Field `u0_dom_isp_top_u0_vin_dvp_data_c9_func_sel` reader - Function Selector of DVP_DATA\\[idx\\], see Function 2 for more information"]
pub type U0_DOM_ISP_TOP_U0_VIN_DVP_DATA_C9_FUNC_SEL_R = crate::FieldReader;
#[doc = "Field `u0_dom_isp_top_u0_vin_dvp_data_c9_func_sel` writer - Function Selector of DVP_DATA\\[idx\\], see Function 2 for more information"]
pub type U0_DOM_ISP_TOP_U0_VIN_DVP_DATA_C9_FUNC_SEL_W<'a, REG, const O: u8> =
    crate::FieldWriter<'a, REG, 3, O>;
#[doc = "Field `u0_dom_isp_top_u0_vin_dvp_hvalid_c_func_sel` reader - Function Selector of DVP_HSYNC, see Function 2 for more information"]
pub type U0_DOM_ISP_TOP_U0_VIN_DVP_HVALID_C_FUNC_SEL_R = crate::FieldReader;
#[doc = "Field `u0_dom_isp_top_u0_vin_dvp_hvalid_c_func_sel` writer - Function Selector of DVP_HSYNC, see Function 2 for more information"]
pub type U0_DOM_ISP_TOP_U0_VIN_DVP_HVALID_C_FUNC_SEL_W<'a, REG, const O: u8> =
    crate::FieldWriter<'a, REG, 3, O>;
#[doc = "Field `u0_dom_isp_top_u0_vin_dvp_vvalid_c_func_sel` reader - Function Selector of DVP_VSYNC, see Function 2 for more information"]
pub type U0_DOM_ISP_TOP_U0_VIN_DVP_VVALID_C_FUNC_SEL_R = crate::FieldReader;
#[doc = "Field `u0_dom_isp_top_u0_vin_dvp_vvalid_c_func_sel` writer - Function Selector of DVP_VSYNC, see Function 2 for more information"]
pub type U0_DOM_ISP_TOP_U0_VIN_DVP_VVALID_C_FUNC_SEL_W<'a, REG, const O: u8> =
    crate::FieldWriter<'a, REG, 3, O>;
#[doc = "Field `u0_sys_crg_dvp_clk_func_sel` reader - Function Selector of DVP_CLK, see Function 2 for more information"]
pub type U0_SYS_CRG_DVP_CLK_FUNC_SEL_R = crate::FieldReader;
#[doc = "Field `u0_sys_crg_dvp_clk_func_sel` writer - Function Selector of DVP_CLK, see Function 2 for more information"]
pub type U0_SYS_CRG_DVP_CLK_FUNC_SEL_W<'a, REG, const O: u8> = crate::FieldWriter<'a, REG, 3, O>;
impl R {
    #[doc = "Bits 0:2 - Function Selector of DVP_DATA\\[idx\\], see Function 2 for more information"]
    #[inline(always)]
    pub fn u0_dom_isp_top_u0_vin_dvp_data_c5_func_sel(
        &self,
    ) -> U0_DOM_ISP_TOP_U0_VIN_DVP_DATA_C5_FUNC_SEL_R {
        U0_DOM_ISP_TOP_U0_VIN_DVP_DATA_C5_FUNC_SEL_R::new((self.bits & 7) as u8)
    }
    #[doc = "Bits 3:5 - Function Selector of DVP_DATA\\[idx\\], see Function 2 for more information"]
    #[inline(always)]
    pub fn u0_dom_isp_top_u0_vin_dvp_data_c6_func_sel(
        &self,
    ) -> U0_DOM_ISP_TOP_U0_VIN_DVP_DATA_C6_FUNC_SEL_R {
        U0_DOM_ISP_TOP_U0_VIN_DVP_DATA_C6_FUNC_SEL_R::new(((self.bits >> 3) & 7) as u8)
    }
    #[doc = "Bits 6:8 - Function Selector of DVP_DATA\\[idx\\], see Function 2 for more information"]
    #[inline(always)]
    pub fn u0_dom_isp_top_u0_vin_dvp_data_c7_func_sel(
        &self,
    ) -> U0_DOM_ISP_TOP_U0_VIN_DVP_DATA_C7_FUNC_SEL_R {
        U0_DOM_ISP_TOP_U0_VIN_DVP_DATA_C7_FUNC_SEL_R::new(((self.bits >> 6) & 7) as u8)
    }
    #[doc = "Bits 9:11 - Function Selector of DVP_DATA\\[idx\\], see Function 2 for more information"]
    #[inline(always)]
    pub fn u0_dom_isp_top_u0_vin_dvp_data_c8_func_sel(
        &self,
    ) -> U0_DOM_ISP_TOP_U0_VIN_DVP_DATA_C8_FUNC_SEL_R {
        U0_DOM_ISP_TOP_U0_VIN_DVP_DATA_C8_FUNC_SEL_R::new(((self.bits >> 9) & 7) as u8)
    }
    #[doc = "Bits 12:14 - Function Selector of DVP_DATA\\[idx\\], see Function 2 for more information"]
    #[inline(always)]
    pub fn u0_dom_isp_top_u0_vin_dvp_data_c9_func_sel(
        &self,
    ) -> U0_DOM_ISP_TOP_U0_VIN_DVP_DATA_C9_FUNC_SEL_R {
        U0_DOM_ISP_TOP_U0_VIN_DVP_DATA_C9_FUNC_SEL_R::new(((self.bits >> 12) & 7) as u8)
    }
    #[doc = "Bits 15:17 - Function Selector of DVP_HSYNC, see Function 2 for more information"]
    #[inline(always)]
    pub fn u0_dom_isp_top_u0_vin_dvp_hvalid_c_func_sel(
        &self,
    ) -> U0_DOM_ISP_TOP_U0_VIN_DVP_HVALID_C_FUNC_SEL_R {
        U0_DOM_ISP_TOP_U0_VIN_DVP_HVALID_C_FUNC_SEL_R::new(((self.bits >> 15) & 7) as u8)
    }
    #[doc = "Bits 18:20 - Function Selector of DVP_VSYNC, see Function 2 for more information"]
    #[inline(always)]
    pub fn u0_dom_isp_top_u0_vin_dvp_vvalid_c_func_sel(
        &self,
    ) -> U0_DOM_ISP_TOP_U0_VIN_DVP_VVALID_C_FUNC_SEL_R {
        U0_DOM_ISP_TOP_U0_VIN_DVP_VVALID_C_FUNC_SEL_R::new(((self.bits >> 18) & 7) as u8)
    }
    #[doc = "Bits 21:23 - Function Selector of DVP_CLK, see Function 2 for more information"]
    #[inline(always)]
    pub fn u0_sys_crg_dvp_clk_func_sel(&self) -> U0_SYS_CRG_DVP_CLK_FUNC_SEL_R {
        U0_SYS_CRG_DVP_CLK_FUNC_SEL_R::new(((self.bits >> 21) & 7) as u8)
    }
}
impl W {
    #[doc = "Bits 0:2 - Function Selector of DVP_DATA\\[idx\\], see Function 2 for more information"]
    #[inline(always)]
    #[must_use]
    pub fn u0_dom_isp_top_u0_vin_dvp_data_c5_func_sel(
        &mut self,
    ) -> U0_DOM_ISP_TOP_U0_VIN_DVP_DATA_C5_FUNC_SEL_W<SYS_IOMUX_CFGSAIF_SYSCFG692_SPEC, 0> {
        U0_DOM_ISP_TOP_U0_VIN_DVP_DATA_C5_FUNC_SEL_W::new(self)
    }
    #[doc = "Bits 3:5 - Function Selector of DVP_DATA\\[idx\\], see Function 2 for more information"]
    #[inline(always)]
    #[must_use]
    pub fn u0_dom_isp_top_u0_vin_dvp_data_c6_func_sel(
        &mut self,
    ) -> U0_DOM_ISP_TOP_U0_VIN_DVP_DATA_C6_FUNC_SEL_W<SYS_IOMUX_CFGSAIF_SYSCFG692_SPEC, 3> {
        U0_DOM_ISP_TOP_U0_VIN_DVP_DATA_C6_FUNC_SEL_W::new(self)
    }
    #[doc = "Bits 6:8 - Function Selector of DVP_DATA\\[idx\\], see Function 2 for more information"]
    #[inline(always)]
    #[must_use]
    pub fn u0_dom_isp_top_u0_vin_dvp_data_c7_func_sel(
        &mut self,
    ) -> U0_DOM_ISP_TOP_U0_VIN_DVP_DATA_C7_FUNC_SEL_W<SYS_IOMUX_CFGSAIF_SYSCFG692_SPEC, 6> {
        U0_DOM_ISP_TOP_U0_VIN_DVP_DATA_C7_FUNC_SEL_W::new(self)
    }
    #[doc = "Bits 9:11 - Function Selector of DVP_DATA\\[idx\\], see Function 2 for more information"]
    #[inline(always)]
    #[must_use]
    pub fn u0_dom_isp_top_u0_vin_dvp_data_c8_func_sel(
        &mut self,
    ) -> U0_DOM_ISP_TOP_U0_VIN_DVP_DATA_C8_FUNC_SEL_W<SYS_IOMUX_CFGSAIF_SYSCFG692_SPEC, 9> {
        U0_DOM_ISP_TOP_U0_VIN_DVP_DATA_C8_FUNC_SEL_W::new(self)
    }
    #[doc = "Bits 12:14 - Function Selector of DVP_DATA\\[idx\\], see Function 2 for more information"]
    #[inline(always)]
    #[must_use]
    pub fn u0_dom_isp_top_u0_vin_dvp_data_c9_func_sel(
        &mut self,
    ) -> U0_DOM_ISP_TOP_U0_VIN_DVP_DATA_C9_FUNC_SEL_W<SYS_IOMUX_CFGSAIF_SYSCFG692_SPEC, 12> {
        U0_DOM_ISP_TOP_U0_VIN_DVP_DATA_C9_FUNC_SEL_W::new(self)
    }
    #[doc = "Bits 15:17 - Function Selector of DVP_HSYNC, see Function 2 for more information"]
    #[inline(always)]
    #[must_use]
    pub fn u0_dom_isp_top_u0_vin_dvp_hvalid_c_func_sel(
        &mut self,
    ) -> U0_DOM_ISP_TOP_U0_VIN_DVP_HVALID_C_FUNC_SEL_W<SYS_IOMUX_CFGSAIF_SYSCFG692_SPEC, 15> {
        U0_DOM_ISP_TOP_U0_VIN_DVP_HVALID_C_FUNC_SEL_W::new(self)
    }
    #[doc = "Bits 18:20 - Function Selector of DVP_VSYNC, see Function 2 for more information"]
    #[inline(always)]
    #[must_use]
    pub fn u0_dom_isp_top_u0_vin_dvp_vvalid_c_func_sel(
        &mut self,
    ) -> U0_DOM_ISP_TOP_U0_VIN_DVP_VVALID_C_FUNC_SEL_W<SYS_IOMUX_CFGSAIF_SYSCFG692_SPEC, 18> {
        U0_DOM_ISP_TOP_U0_VIN_DVP_VVALID_C_FUNC_SEL_W::new(self)
    }
    #[doc = "Bits 21:23 - Function Selector of DVP_CLK, see Function 2 for more information"]
    #[inline(always)]
    #[must_use]
    pub fn u0_sys_crg_dvp_clk_func_sel(
        &mut self,
    ) -> U0_SYS_CRG_DVP_CLK_FUNC_SEL_W<SYS_IOMUX_CFGSAIF_SYSCFG692_SPEC, 21> {
        U0_SYS_CRG_DVP_CLK_FUNC_SEL_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.bits = bits;
        self
    }
}
#[doc = "SYS IOMUX CFG SAIF SYSCFG 692\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`sys_iomux_cfgsaif_syscfg692::R`](R).  You can [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`sys_iomux_cfgsaif_syscfg692::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct SYS_IOMUX_CFGSAIF_SYSCFG692_SPEC;
impl crate::RegisterSpec for SYS_IOMUX_CFGSAIF_SYSCFG692_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`sys_iomux_cfgsaif_syscfg692::R`](R) reader structure"]
impl crate::Readable for SYS_IOMUX_CFGSAIF_SYSCFG692_SPEC {}
#[doc = "`write(|w| ..)` method takes [`sys_iomux_cfgsaif_syscfg692::W`](W) writer structure"]
impl crate::Writable for SYS_IOMUX_CFGSAIF_SYSCFG692_SPEC {
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
