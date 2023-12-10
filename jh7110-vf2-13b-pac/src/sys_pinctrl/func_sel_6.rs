#[doc = "Register `func_sel_6` reader"]
pub type R = crate::R<FUNC_SEL_6_SPEC>;
#[doc = "Register `func_sel_6` writer"]
pub type W = crate::W<FUNC_SEL_6_SPEC>;
#[doc = "Field `vin_dvp_data_5` reader - Function Selector of DVP_DATA\\[idx\\], see Function 2 for more information"]
pub type VIN_DVP_DATA_5_R = crate::FieldReader;
#[doc = "Field `vin_dvp_data_5` writer - Function Selector of DVP_DATA\\[idx\\], see Function 2 for more information"]
pub type VIN_DVP_DATA_5_W<'a, REG> = crate::FieldWriter<'a, REG, 3>;
#[doc = "Field `vin_dvp_data_6` reader - Function Selector of DVP_DATA\\[idx\\], see Function 2 for more information"]
pub type VIN_DVP_DATA_6_R = crate::FieldReader;
#[doc = "Field `vin_dvp_data_6` writer - Function Selector of DVP_DATA\\[idx\\], see Function 2 for more information"]
pub type VIN_DVP_DATA_6_W<'a, REG> = crate::FieldWriter<'a, REG, 3>;
#[doc = "Field `vin_dvp_data_7` reader - Function Selector of DVP_DATA\\[idx\\], see Function 2 for more information"]
pub type VIN_DVP_DATA_7_R = crate::FieldReader;
#[doc = "Field `vin_dvp_data_7` writer - Function Selector of DVP_DATA\\[idx\\], see Function 2 for more information"]
pub type VIN_DVP_DATA_7_W<'a, REG> = crate::FieldWriter<'a, REG, 3>;
#[doc = "Field `vin_dvp_data_8` reader - Function Selector of DVP_DATA\\[idx\\], see Function 2 for more information"]
pub type VIN_DVP_DATA_8_R = crate::FieldReader;
#[doc = "Field `vin_dvp_data_8` writer - Function Selector of DVP_DATA\\[idx\\], see Function 2 for more information"]
pub type VIN_DVP_DATA_8_W<'a, REG> = crate::FieldWriter<'a, REG, 3>;
#[doc = "Field `vin_dvp_data_9` reader - Function Selector of DVP_DATA\\[idx\\], see Function 2 for more information"]
pub type VIN_DVP_DATA_9_R = crate::FieldReader;
#[doc = "Field `vin_dvp_data_9` writer - Function Selector of DVP_DATA\\[idx\\], see Function 2 for more information"]
pub type VIN_DVP_DATA_9_W<'a, REG> = crate::FieldWriter<'a, REG, 3>;
#[doc = "Field `vin_dvp_hvalid` reader - Function Selector of DVP_HSYNC, see Function 2 for more information"]
pub type VIN_DVP_HVALID_R = crate::FieldReader;
#[doc = "Field `vin_dvp_hvalid` writer - Function Selector of DVP_HSYNC, see Function 2 for more information"]
pub type VIN_DVP_HVALID_W<'a, REG> = crate::FieldWriter<'a, REG, 3>;
#[doc = "Field `vin_dvp_vvalid` reader - Function Selector of DVP_VSYNC, see Function 2 for more information"]
pub type VIN_DVP_VVALID_R = crate::FieldReader;
#[doc = "Field `vin_dvp_vvalid` writer - Function Selector of DVP_VSYNC, see Function 2 for more information"]
pub type VIN_DVP_VVALID_W<'a, REG> = crate::FieldWriter<'a, REG, 3>;
#[doc = "Field `dvp_clk` reader - Function Selector of DVP_CLK, see Function 2 for more information"]
pub type DVP_CLK_R = crate::FieldReader;
#[doc = "Field `dvp_clk` writer - Function Selector of DVP_CLK, see Function 2 for more information"]
pub type DVP_CLK_W<'a, REG> = crate::FieldWriter<'a, REG, 3>;
impl R {
    #[doc = "Bits 0:2 - Function Selector of DVP_DATA\\[idx\\], see Function 2 for more information"]
    #[inline(always)]
    pub fn vin_dvp_data_5(&self) -> VIN_DVP_DATA_5_R {
        VIN_DVP_DATA_5_R::new((self.bits & 7) as u8)
    }
    #[doc = "Bits 3:5 - Function Selector of DVP_DATA\\[idx\\], see Function 2 for more information"]
    #[inline(always)]
    pub fn vin_dvp_data_6(&self) -> VIN_DVP_DATA_6_R {
        VIN_DVP_DATA_6_R::new(((self.bits >> 3) & 7) as u8)
    }
    #[doc = "Bits 6:8 - Function Selector of DVP_DATA\\[idx\\], see Function 2 for more information"]
    #[inline(always)]
    pub fn vin_dvp_data_7(&self) -> VIN_DVP_DATA_7_R {
        VIN_DVP_DATA_7_R::new(((self.bits >> 6) & 7) as u8)
    }
    #[doc = "Bits 9:11 - Function Selector of DVP_DATA\\[idx\\], see Function 2 for more information"]
    #[inline(always)]
    pub fn vin_dvp_data_8(&self) -> VIN_DVP_DATA_8_R {
        VIN_DVP_DATA_8_R::new(((self.bits >> 9) & 7) as u8)
    }
    #[doc = "Bits 12:14 - Function Selector of DVP_DATA\\[idx\\], see Function 2 for more information"]
    #[inline(always)]
    pub fn vin_dvp_data_9(&self) -> VIN_DVP_DATA_9_R {
        VIN_DVP_DATA_9_R::new(((self.bits >> 12) & 7) as u8)
    }
    #[doc = "Bits 15:17 - Function Selector of DVP_HSYNC, see Function 2 for more information"]
    #[inline(always)]
    pub fn vin_dvp_hvalid(&self) -> VIN_DVP_HVALID_R {
        VIN_DVP_HVALID_R::new(((self.bits >> 15) & 7) as u8)
    }
    #[doc = "Bits 18:20 - Function Selector of DVP_VSYNC, see Function 2 for more information"]
    #[inline(always)]
    pub fn vin_dvp_vvalid(&self) -> VIN_DVP_VVALID_R {
        VIN_DVP_VVALID_R::new(((self.bits >> 18) & 7) as u8)
    }
    #[doc = "Bits 21:23 - Function Selector of DVP_CLK, see Function 2 for more information"]
    #[inline(always)]
    pub fn dvp_clk(&self) -> DVP_CLK_R {
        DVP_CLK_R::new(((self.bits >> 21) & 7) as u8)
    }
}
impl W {
    #[doc = "Bits 0:2 - Function Selector of DVP_DATA\\[idx\\], see Function 2 for more information"]
    #[inline(always)]
    #[must_use]
    pub fn vin_dvp_data_5(&mut self) -> VIN_DVP_DATA_5_W<FUNC_SEL_6_SPEC> {
        VIN_DVP_DATA_5_W::new(self, 0)
    }
    #[doc = "Bits 3:5 - Function Selector of DVP_DATA\\[idx\\], see Function 2 for more information"]
    #[inline(always)]
    #[must_use]
    pub fn vin_dvp_data_6(&mut self) -> VIN_DVP_DATA_6_W<FUNC_SEL_6_SPEC> {
        VIN_DVP_DATA_6_W::new(self, 3)
    }
    #[doc = "Bits 6:8 - Function Selector of DVP_DATA\\[idx\\], see Function 2 for more information"]
    #[inline(always)]
    #[must_use]
    pub fn vin_dvp_data_7(&mut self) -> VIN_DVP_DATA_7_W<FUNC_SEL_6_SPEC> {
        VIN_DVP_DATA_7_W::new(self, 6)
    }
    #[doc = "Bits 9:11 - Function Selector of DVP_DATA\\[idx\\], see Function 2 for more information"]
    #[inline(always)]
    #[must_use]
    pub fn vin_dvp_data_8(&mut self) -> VIN_DVP_DATA_8_W<FUNC_SEL_6_SPEC> {
        VIN_DVP_DATA_8_W::new(self, 9)
    }
    #[doc = "Bits 12:14 - Function Selector of DVP_DATA\\[idx\\], see Function 2 for more information"]
    #[inline(always)]
    #[must_use]
    pub fn vin_dvp_data_9(&mut self) -> VIN_DVP_DATA_9_W<FUNC_SEL_6_SPEC> {
        VIN_DVP_DATA_9_W::new(self, 12)
    }
    #[doc = "Bits 15:17 - Function Selector of DVP_HSYNC, see Function 2 for more information"]
    #[inline(always)]
    #[must_use]
    pub fn vin_dvp_hvalid(&mut self) -> VIN_DVP_HVALID_W<FUNC_SEL_6_SPEC> {
        VIN_DVP_HVALID_W::new(self, 15)
    }
    #[doc = "Bits 18:20 - Function Selector of DVP_VSYNC, see Function 2 for more information"]
    #[inline(always)]
    #[must_use]
    pub fn vin_dvp_vvalid(&mut self) -> VIN_DVP_VVALID_W<FUNC_SEL_6_SPEC> {
        VIN_DVP_VVALID_W::new(self, 18)
    }
    #[doc = "Bits 21:23 - Function Selector of DVP_CLK, see Function 2 for more information"]
    #[inline(always)]
    #[must_use]
    pub fn dvp_clk(&mut self) -> DVP_CLK_W<FUNC_SEL_6_SPEC> {
        DVP_CLK_W::new(self, 21)
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
#[doc = "SYS IOMUX CFG SAIF SYSCFG 692\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`func_sel_6::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`func_sel_6::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct FUNC_SEL_6_SPEC;
impl crate::RegisterSpec for FUNC_SEL_6_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`func_sel_6::R`](R) reader structure"]
impl crate::Readable for FUNC_SEL_6_SPEC {}
#[doc = "`write(|w| ..)` method takes [`func_sel_6::W`](W) writer structure"]
impl crate::Writable for FUNC_SEL_6_SPEC {
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets func_sel_6 to value 0"]
impl crate::Resettable for FUNC_SEL_6_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
