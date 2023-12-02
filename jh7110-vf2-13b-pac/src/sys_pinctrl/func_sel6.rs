#[doc = "Register `func_sel6` reader"]
pub type R = crate::R<FUNC_SEL6_SPEC>;
#[doc = "Register `func_sel6` writer"]
pub type W = crate::W<FUNC_SEL6_SPEC>;
#[doc = "Field `u0_dom_isp_top_u0_vin_dvp_data_c5_func_sel` reader - Function Selector of DVP_DATA\\[idx\\], see Function 2 for more information"]
pub type U0_DOM_ISP_TOP_U0_VIN_DVP_DATA_C5_FUNC_SEL_R = crate::FieldReader;
#[doc = "Field `u0_dom_isp_top_u0_vin_dvp_data_c5_func_sel` writer - Function Selector of DVP_DATA\\[idx\\], see Function 2 for more information"]
pub type U0_DOM_ISP_TOP_U0_VIN_DVP_DATA_C5_FUNC_SEL_W<'a, REG> = crate::FieldWriter<'a, REG, 3>;
#[doc = "Field `u0_dom_isp_top_u0_vin_dvp_data_c6_func_sel` reader - Function Selector of DVP_DATA\\[idx\\], see Function 2 for more information"]
pub type U0_DOM_ISP_TOP_U0_VIN_DVP_DATA_C6_FUNC_SEL_R = crate::FieldReader;
#[doc = "Field `u0_dom_isp_top_u0_vin_dvp_data_c6_func_sel` writer - Function Selector of DVP_DATA\\[idx\\], see Function 2 for more information"]
pub type U0_DOM_ISP_TOP_U0_VIN_DVP_DATA_C6_FUNC_SEL_W<'a, REG> = crate::FieldWriter<'a, REG, 3>;
#[doc = "Field `u0_dom_isp_top_u0_vin_dvp_data_c7_func_sel` reader - Function Selector of DVP_DATA\\[idx\\], see Function 2 for more information"]
pub type U0_DOM_ISP_TOP_U0_VIN_DVP_DATA_C7_FUNC_SEL_R = crate::FieldReader;
#[doc = "Field `u0_dom_isp_top_u0_vin_dvp_data_c7_func_sel` writer - Function Selector of DVP_DATA\\[idx\\], see Function 2 for more information"]
pub type U0_DOM_ISP_TOP_U0_VIN_DVP_DATA_C7_FUNC_SEL_W<'a, REG> = crate::FieldWriter<'a, REG, 3>;
#[doc = "Field `u0_dom_isp_top_u0_vin_dvp_data_c8_func_sel` reader - Function Selector of DVP_DATA\\[idx\\], see Function 2 for more information"]
pub type U0_DOM_ISP_TOP_U0_VIN_DVP_DATA_C8_FUNC_SEL_R = crate::FieldReader;
#[doc = "Field `u0_dom_isp_top_u0_vin_dvp_data_c8_func_sel` writer - Function Selector of DVP_DATA\\[idx\\], see Function 2 for more information"]
pub type U0_DOM_ISP_TOP_U0_VIN_DVP_DATA_C8_FUNC_SEL_W<'a, REG> = crate::FieldWriter<'a, REG, 3>;
#[doc = "Field `u0_dom_isp_top_u0_vin_dvp_data_c9_func_sel` reader - Function Selector of DVP_DATA\\[idx\\], see Function 2 for more information"]
pub type U0_DOM_ISP_TOP_U0_VIN_DVP_DATA_C9_FUNC_SEL_R = crate::FieldReader;
#[doc = "Field `u0_dom_isp_top_u0_vin_dvp_data_c9_func_sel` writer - Function Selector of DVP_DATA\\[idx\\], see Function 2 for more information"]
pub type U0_DOM_ISP_TOP_U0_VIN_DVP_DATA_C9_FUNC_SEL_W<'a, REG> = crate::FieldWriter<'a, REG, 3>;
#[doc = "Field `u0_dom_isp_top_u0_vin_dvp_hvalid_c_func_sel` reader - Function Selector of DVP_HSYNC, see Function 2 for more information"]
pub type U0_DOM_ISP_TOP_U0_VIN_DVP_HVALID_C_FUNC_SEL_R = crate::FieldReader;
#[doc = "Field `u0_dom_isp_top_u0_vin_dvp_hvalid_c_func_sel` writer - Function Selector of DVP_HSYNC, see Function 2 for more information"]
pub type U0_DOM_ISP_TOP_U0_VIN_DVP_HVALID_C_FUNC_SEL_W<'a, REG> = crate::FieldWriter<'a, REG, 3>;
#[doc = "Field `u0_dom_isp_top_u0_vin_dvp_vvalid_c_func_sel` reader - Function Selector of DVP_VSYNC, see Function 2 for more information"]
pub type U0_DOM_ISP_TOP_U0_VIN_DVP_VVALID_C_FUNC_SEL_R = crate::FieldReader;
#[doc = "Field `u0_dom_isp_top_u0_vin_dvp_vvalid_c_func_sel` writer - Function Selector of DVP_VSYNC, see Function 2 for more information"]
pub type U0_DOM_ISP_TOP_U0_VIN_DVP_VVALID_C_FUNC_SEL_W<'a, REG> = crate::FieldWriter<'a, REG, 3>;
#[doc = "Field `u0_sys_crg_dvp_clk_func_sel` reader - Function Selector of DVP_CLK, see Function 2 for more information"]
pub type U0_SYS_CRG_DVP_CLK_FUNC_SEL_R = crate::FieldReader;
#[doc = "Field `u0_sys_crg_dvp_clk_func_sel` writer - Function Selector of DVP_CLK, see Function 2 for more information"]
pub type U0_SYS_CRG_DVP_CLK_FUNC_SEL_W<'a, REG> = crate::FieldWriter<'a, REG, 3>;
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
    ) -> U0_DOM_ISP_TOP_U0_VIN_DVP_DATA_C5_FUNC_SEL_W<FUNC_SEL6_SPEC> {
        U0_DOM_ISP_TOP_U0_VIN_DVP_DATA_C5_FUNC_SEL_W::new(self, 0)
    }
    #[doc = "Bits 3:5 - Function Selector of DVP_DATA\\[idx\\], see Function 2 for more information"]
    #[inline(always)]
    #[must_use]
    pub fn u0_dom_isp_top_u0_vin_dvp_data_c6_func_sel(
        &mut self,
    ) -> U0_DOM_ISP_TOP_U0_VIN_DVP_DATA_C6_FUNC_SEL_W<FUNC_SEL6_SPEC> {
        U0_DOM_ISP_TOP_U0_VIN_DVP_DATA_C6_FUNC_SEL_W::new(self, 3)
    }
    #[doc = "Bits 6:8 - Function Selector of DVP_DATA\\[idx\\], see Function 2 for more information"]
    #[inline(always)]
    #[must_use]
    pub fn u0_dom_isp_top_u0_vin_dvp_data_c7_func_sel(
        &mut self,
    ) -> U0_DOM_ISP_TOP_U0_VIN_DVP_DATA_C7_FUNC_SEL_W<FUNC_SEL6_SPEC> {
        U0_DOM_ISP_TOP_U0_VIN_DVP_DATA_C7_FUNC_SEL_W::new(self, 6)
    }
    #[doc = "Bits 9:11 - Function Selector of DVP_DATA\\[idx\\], see Function 2 for more information"]
    #[inline(always)]
    #[must_use]
    pub fn u0_dom_isp_top_u0_vin_dvp_data_c8_func_sel(
        &mut self,
    ) -> U0_DOM_ISP_TOP_U0_VIN_DVP_DATA_C8_FUNC_SEL_W<FUNC_SEL6_SPEC> {
        U0_DOM_ISP_TOP_U0_VIN_DVP_DATA_C8_FUNC_SEL_W::new(self, 9)
    }
    #[doc = "Bits 12:14 - Function Selector of DVP_DATA\\[idx\\], see Function 2 for more information"]
    #[inline(always)]
    #[must_use]
    pub fn u0_dom_isp_top_u0_vin_dvp_data_c9_func_sel(
        &mut self,
    ) -> U0_DOM_ISP_TOP_U0_VIN_DVP_DATA_C9_FUNC_SEL_W<FUNC_SEL6_SPEC> {
        U0_DOM_ISP_TOP_U0_VIN_DVP_DATA_C9_FUNC_SEL_W::new(self, 12)
    }
    #[doc = "Bits 15:17 - Function Selector of DVP_HSYNC, see Function 2 for more information"]
    #[inline(always)]
    #[must_use]
    pub fn u0_dom_isp_top_u0_vin_dvp_hvalid_c_func_sel(
        &mut self,
    ) -> U0_DOM_ISP_TOP_U0_VIN_DVP_HVALID_C_FUNC_SEL_W<FUNC_SEL6_SPEC> {
        U0_DOM_ISP_TOP_U0_VIN_DVP_HVALID_C_FUNC_SEL_W::new(self, 15)
    }
    #[doc = "Bits 18:20 - Function Selector of DVP_VSYNC, see Function 2 for more information"]
    #[inline(always)]
    #[must_use]
    pub fn u0_dom_isp_top_u0_vin_dvp_vvalid_c_func_sel(
        &mut self,
    ) -> U0_DOM_ISP_TOP_U0_VIN_DVP_VVALID_C_FUNC_SEL_W<FUNC_SEL6_SPEC> {
        U0_DOM_ISP_TOP_U0_VIN_DVP_VVALID_C_FUNC_SEL_W::new(self, 18)
    }
    #[doc = "Bits 21:23 - Function Selector of DVP_CLK, see Function 2 for more information"]
    #[inline(always)]
    #[must_use]
    pub fn u0_sys_crg_dvp_clk_func_sel(&mut self) -> U0_SYS_CRG_DVP_CLK_FUNC_SEL_W<FUNC_SEL6_SPEC> {
        U0_SYS_CRG_DVP_CLK_FUNC_SEL_W::new(self, 21)
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
#[doc = "SYS IOMUX CFG SAIF SYSCFG 6\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`func_sel6::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`func_sel6::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct FUNC_SEL6_SPEC;
impl crate::RegisterSpec for FUNC_SEL6_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`func_sel6::R`](R) reader structure"]
impl crate::Readable for FUNC_SEL6_SPEC {}
#[doc = "`write(|w| ..)` method takes [`func_sel6::W`](W) writer structure"]
impl crate::Writable for FUNC_SEL6_SPEC {
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets func_sel6 to value 0"]
impl crate::Resettable for FUNC_SEL6_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
