#[doc = "Register `func_sel_5` reader"]
pub type R = crate::R<FUNC_SEL_5_SPEC>;
#[doc = "Register `func_sel_5` writer"]
pub type W = crate::W<FUNC_SEL_5_SPEC>;
#[doc = "Field `pad_gpio_6` reader - GPIO function selector: * Function 0: See Function Description no page 84 for more information, * Function 1: See Full Multiplexing for more information, * Function 2: See Function 2 for more information, * Function 3: See Function 3 for more information"]
pub type PAD_GPIO_6_R = crate::FieldReader;
#[doc = "Field `pad_gpio_6` writer - GPIO function selector: * Function 0: See Function Description no page 84 for more information, * Function 1: See Full Multiplexing for more information, * Function 2: See Function 2 for more information, * Function 3: See Function 3 for more information"]
pub type PAD_GPIO_6_W<'a, REG> = crate::FieldWriter<'a, REG, 2>;
#[doc = "Field `pad_gpio_7` reader - GPIO function selector: * Function 0: See Function Description no page 84 for more information, * Function 1: See Full Multiplexing for more information, * Function 2: See Function 2 for more information, * Function 3: See Function 3 for more information"]
pub type PAD_GPIO_7_R = crate::FieldReader;
#[doc = "Field `pad_gpio_7` writer - GPIO function selector: * Function 0: See Function Description no page 84 for more information, * Function 1: See Full Multiplexing for more information, * Function 2: See Function 2 for more information, * Function 3: See Function 3 for more information"]
pub type PAD_GPIO_7_W<'a, REG> = crate::FieldWriter<'a, REG, 3>;
#[doc = "Field `pad_gpio_8` reader - GPIO function selector: * Function 0: See Function Description no page 84 for more information, * Function 1: See Full Multiplexing for more information, * Function 2: See Function 2 for more information, * Function 3: See Function 3 for more information"]
pub type PAD_GPIO_8_R = crate::FieldReader;
#[doc = "Field `pad_gpio_8` writer - GPIO function selector: * Function 0: See Function Description no page 84 for more information, * Function 1: See Full Multiplexing for more information, * Function 2: See Function 2 for more information, * Function 3: See Function 3 for more information"]
pub type PAD_GPIO_8_W<'a, REG> = crate::FieldWriter<'a, REG, 3>;
#[doc = "Field `pad_gpio_9` reader - GPIO function selector: * Function 0: See Function Description no page 84 for more information, * Function 1: See Full Multiplexing for more information, * Function 2: See Function 2 for more information, * Function 3: See Function 3 for more information"]
pub type PAD_GPIO_9_R = crate::FieldReader;
#[doc = "Field `pad_gpio_9` writer - GPIO function selector: * Function 0: See Function Description no page 84 for more information, * Function 1: See Full Multiplexing for more information, * Function 2: See Function 2 for more information, * Function 3: See Function 3 for more information"]
pub type PAD_GPIO_9_W<'a, REG> = crate::FieldWriter<'a, REG, 3>;
#[doc = "Field `vin_dvp_data_0` reader - Function Selector of DVP_DATA\\[idx\\], see Function 2 for more information"]
pub type VIN_DVP_DATA_0_R = crate::FieldReader;
#[doc = "Field `vin_dvp_data_0` writer - Function Selector of DVP_DATA\\[idx\\], see Function 2 for more information"]
pub type VIN_DVP_DATA_0_W<'a, REG> = crate::FieldWriter<'a, REG, 3>;
#[doc = "Field `vin_dvp_data_10` reader - Function Selector of DVP_DATA\\[idx\\], see Function 2 for more information"]
pub type VIN_DVP_DATA_10_R = crate::FieldReader;
#[doc = "Field `vin_dvp_data_10` writer - Function Selector of DVP_DATA\\[idx\\], see Function 2 for more information"]
pub type VIN_DVP_DATA_10_W<'a, REG> = crate::FieldWriter<'a, REG, 3>;
#[doc = "Field `vin_dvp_data_11` reader - Function Selector of DVP_DATA\\[idx\\], see Function 2 for more information"]
pub type VIN_DVP_DATA_11_R = crate::FieldReader;
#[doc = "Field `vin_dvp_data_11` writer - Function Selector of DVP_DATA\\[idx\\], see Function 2 for more information"]
pub type VIN_DVP_DATA_11_W<'a, REG> = crate::FieldWriter<'a, REG, 3>;
#[doc = "Field `vin_dvp_data_1` reader - Function Selector of DVP_DATA\\[idx\\], see Function 2 for more information"]
pub type VIN_DVP_DATA_1_R = crate::FieldReader;
#[doc = "Field `vin_dvp_data_1` writer - Function Selector of DVP_DATA\\[idx\\], see Function 2 for more information"]
pub type VIN_DVP_DATA_1_W<'a, REG> = crate::FieldWriter<'a, REG, 3>;
#[doc = "Field `vin_dvp_data_2` reader - Function Selector of DVP_DATA\\[idx\\], see Function 2 for more information"]
pub type VIN_DVP_DATA_2_R = crate::FieldReader;
#[doc = "Field `vin_dvp_data_2` writer - Function Selector of DVP_DATA\\[idx\\], see Function 2 for more information"]
pub type VIN_DVP_DATA_2_W<'a, REG> = crate::FieldWriter<'a, REG, 3>;
#[doc = "Field `vin_dvp_data_3` reader - Function Selector of DVP_DATA\\[idx\\], see Function 2 for more information"]
pub type VIN_DVP_DATA_3_R = crate::FieldReader;
#[doc = "Field `vin_dvp_data_3` writer - Function Selector of DVP_DATA\\[idx\\], see Function 2 for more information"]
pub type VIN_DVP_DATA_3_W<'a, REG> = crate::FieldWriter<'a, REG, 3>;
#[doc = "Field `vin_dvp_data_4` reader - Function Selector of DVP_DATA\\[idx\\], see Function 2 for more information"]
pub type VIN_DVP_DATA_4_R = crate::FieldReader;
#[doc = "Field `vin_dvp_data_4` writer - Function Selector of DVP_DATA\\[idx\\], see Function 2 for more information"]
pub type VIN_DVP_DATA_4_W<'a, REG> = crate::FieldWriter<'a, REG, 3>;
impl R {
    #[doc = "Bits 0:1 - GPIO function selector: * Function 0: See Function Description no page 84 for more information, * Function 1: See Full Multiplexing for more information, * Function 2: See Function 2 for more information, * Function 3: See Function 3 for more information"]
    #[inline(always)]
    pub fn pad_gpio_6(&self) -> PAD_GPIO_6_R {
        PAD_GPIO_6_R::new((self.bits & 3) as u8)
    }
    #[doc = "Bits 3:5 - GPIO function selector: * Function 0: See Function Description no page 84 for more information, * Function 1: See Full Multiplexing for more information, * Function 2: See Function 2 for more information, * Function 3: See Function 3 for more information"]
    #[inline(always)]
    pub fn pad_gpio_7(&self) -> PAD_GPIO_7_R {
        PAD_GPIO_7_R::new(((self.bits >> 3) & 7) as u8)
    }
    #[doc = "Bits 6:8 - GPIO function selector: * Function 0: See Function Description no page 84 for more information, * Function 1: See Full Multiplexing for more information, * Function 2: See Function 2 for more information, * Function 3: See Function 3 for more information"]
    #[inline(always)]
    pub fn pad_gpio_8(&self) -> PAD_GPIO_8_R {
        PAD_GPIO_8_R::new(((self.bits >> 6) & 7) as u8)
    }
    #[doc = "Bits 9:11 - GPIO function selector: * Function 0: See Function Description no page 84 for more information, * Function 1: See Full Multiplexing for more information, * Function 2: See Function 2 for more information, * Function 3: See Function 3 for more information"]
    #[inline(always)]
    pub fn pad_gpio_9(&self) -> PAD_GPIO_9_R {
        PAD_GPIO_9_R::new(((self.bits >> 9) & 7) as u8)
    }
    #[doc = "Bits 11:13 - Function Selector of DVP_DATA\\[idx\\], see Function 2 for more information"]
    #[inline(always)]
    pub fn vin_dvp_data_0(&self) -> VIN_DVP_DATA_0_R {
        VIN_DVP_DATA_0_R::new(((self.bits >> 11) & 7) as u8)
    }
    #[doc = "Bits 14:16 - Function Selector of DVP_DATA\\[idx\\], see Function 2 for more information"]
    #[inline(always)]
    pub fn vin_dvp_data_10(&self) -> VIN_DVP_DATA_10_R {
        VIN_DVP_DATA_10_R::new(((self.bits >> 14) & 7) as u8)
    }
    #[doc = "Bits 17:19 - Function Selector of DVP_DATA\\[idx\\], see Function 2 for more information"]
    #[inline(always)]
    pub fn vin_dvp_data_11(&self) -> VIN_DVP_DATA_11_R {
        VIN_DVP_DATA_11_R::new(((self.bits >> 17) & 7) as u8)
    }
    #[doc = "Bits 20:22 - Function Selector of DVP_DATA\\[idx\\], see Function 2 for more information"]
    #[inline(always)]
    pub fn vin_dvp_data_1(&self) -> VIN_DVP_DATA_1_R {
        VIN_DVP_DATA_1_R::new(((self.bits >> 20) & 7) as u8)
    }
    #[doc = "Bits 23:25 - Function Selector of DVP_DATA\\[idx\\], see Function 2 for more information"]
    #[inline(always)]
    pub fn vin_dvp_data_2(&self) -> VIN_DVP_DATA_2_R {
        VIN_DVP_DATA_2_R::new(((self.bits >> 23) & 7) as u8)
    }
    #[doc = "Bits 26:28 - Function Selector of DVP_DATA\\[idx\\], see Function 2 for more information"]
    #[inline(always)]
    pub fn vin_dvp_data_3(&self) -> VIN_DVP_DATA_3_R {
        VIN_DVP_DATA_3_R::new(((self.bits >> 26) & 7) as u8)
    }
    #[doc = "Bits 29:31 - Function Selector of DVP_DATA\\[idx\\], see Function 2 for more information"]
    #[inline(always)]
    pub fn vin_dvp_data_4(&self) -> VIN_DVP_DATA_4_R {
        VIN_DVP_DATA_4_R::new(((self.bits >> 29) & 7) as u8)
    }
}
impl W {
    #[doc = "Bits 0:1 - GPIO function selector: * Function 0: See Function Description no page 84 for more information, * Function 1: See Full Multiplexing for more information, * Function 2: See Function 2 for more information, * Function 3: See Function 3 for more information"]
    #[inline(always)]
    #[must_use]
    pub fn pad_gpio_6(&mut self) -> PAD_GPIO_6_W<FUNC_SEL_5_SPEC> {
        PAD_GPIO_6_W::new(self, 0)
    }
    #[doc = "Bits 3:5 - GPIO function selector: * Function 0: See Function Description no page 84 for more information, * Function 1: See Full Multiplexing for more information, * Function 2: See Function 2 for more information, * Function 3: See Function 3 for more information"]
    #[inline(always)]
    #[must_use]
    pub fn pad_gpio_7(&mut self) -> PAD_GPIO_7_W<FUNC_SEL_5_SPEC> {
        PAD_GPIO_7_W::new(self, 3)
    }
    #[doc = "Bits 6:8 - GPIO function selector: * Function 0: See Function Description no page 84 for more information, * Function 1: See Full Multiplexing for more information, * Function 2: See Function 2 for more information, * Function 3: See Function 3 for more information"]
    #[inline(always)]
    #[must_use]
    pub fn pad_gpio_8(&mut self) -> PAD_GPIO_8_W<FUNC_SEL_5_SPEC> {
        PAD_GPIO_8_W::new(self, 6)
    }
    #[doc = "Bits 9:11 - GPIO function selector: * Function 0: See Function Description no page 84 for more information, * Function 1: See Full Multiplexing for more information, * Function 2: See Function 2 for more information, * Function 3: See Function 3 for more information"]
    #[inline(always)]
    #[must_use]
    pub fn pad_gpio_9(&mut self) -> PAD_GPIO_9_W<FUNC_SEL_5_SPEC> {
        PAD_GPIO_9_W::new(self, 9)
    }
    #[doc = "Bits 11:13 - Function Selector of DVP_DATA\\[idx\\], see Function 2 for more information"]
    #[inline(always)]
    #[must_use]
    pub fn vin_dvp_data_0(&mut self) -> VIN_DVP_DATA_0_W<FUNC_SEL_5_SPEC> {
        VIN_DVP_DATA_0_W::new(self, 11)
    }
    #[doc = "Bits 14:16 - Function Selector of DVP_DATA\\[idx\\], see Function 2 for more information"]
    #[inline(always)]
    #[must_use]
    pub fn vin_dvp_data_10(&mut self) -> VIN_DVP_DATA_10_W<FUNC_SEL_5_SPEC> {
        VIN_DVP_DATA_10_W::new(self, 14)
    }
    #[doc = "Bits 17:19 - Function Selector of DVP_DATA\\[idx\\], see Function 2 for more information"]
    #[inline(always)]
    #[must_use]
    pub fn vin_dvp_data_11(&mut self) -> VIN_DVP_DATA_11_W<FUNC_SEL_5_SPEC> {
        VIN_DVP_DATA_11_W::new(self, 17)
    }
    #[doc = "Bits 20:22 - Function Selector of DVP_DATA\\[idx\\], see Function 2 for more information"]
    #[inline(always)]
    #[must_use]
    pub fn vin_dvp_data_1(&mut self) -> VIN_DVP_DATA_1_W<FUNC_SEL_5_SPEC> {
        VIN_DVP_DATA_1_W::new(self, 20)
    }
    #[doc = "Bits 23:25 - Function Selector of DVP_DATA\\[idx\\], see Function 2 for more information"]
    #[inline(always)]
    #[must_use]
    pub fn vin_dvp_data_2(&mut self) -> VIN_DVP_DATA_2_W<FUNC_SEL_5_SPEC> {
        VIN_DVP_DATA_2_W::new(self, 23)
    }
    #[doc = "Bits 26:28 - Function Selector of DVP_DATA\\[idx\\], see Function 2 for more information"]
    #[inline(always)]
    #[must_use]
    pub fn vin_dvp_data_3(&mut self) -> VIN_DVP_DATA_3_W<FUNC_SEL_5_SPEC> {
        VIN_DVP_DATA_3_W::new(self, 26)
    }
    #[doc = "Bits 29:31 - Function Selector of DVP_DATA\\[idx\\], see Function 2 for more information"]
    #[inline(always)]
    #[must_use]
    pub fn vin_dvp_data_4(&mut self) -> VIN_DVP_DATA_4_W<FUNC_SEL_5_SPEC> {
        VIN_DVP_DATA_4_W::new(self, 29)
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
#[doc = "SYS IOMUX CFG SAIF SYSCFG 688\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`func_sel_5::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`func_sel_5::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct FUNC_SEL_5_SPEC;
impl crate::RegisterSpec for FUNC_SEL_5_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`func_sel_5::R`](R) reader structure"]
impl crate::Readable for FUNC_SEL_5_SPEC {}
#[doc = "`write(|w| ..)` method takes [`func_sel_5::W`](W) writer structure"]
impl crate::Writable for FUNC_SEL_5_SPEC {
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets func_sel_5 to value 0"]
impl crate::Resettable for FUNC_SEL_5_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
