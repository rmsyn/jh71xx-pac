#[doc = "Register `ssp_cr0` reader"]
pub type R = crate::R<SSP_CR0_SPEC>;
#[doc = "Register `ssp_cr0` writer"]
pub type W = crate::W<SSP_CR0_SPEC>;
#[doc = "Field `dss` reader - Data Size Select - 0000, 0001, 0010: Reserved, 0011: 4-bit data, 0100: 5-bit data, 6-bit data, 0110: 7-bit data, 0111: 8-bit data, 1000: 9-bit data, 1001: 10-bit data, 1010: 11-bit data, 1011: 12-bit data, 1100: 13-bit data, 1101: 14-bit data, 1110: 15-bit data, 1111: 16-bit data"]
pub type DSS_R = crate::FieldReader;
#[doc = "Field `dss` writer - Data Size Select - 0000, 0001, 0010: Reserved, 0011: 4-bit data, 0100: 5-bit data, 6-bit data, 0110: 7-bit data, 0111: 8-bit data, 1000: 9-bit data, 1001: 10-bit data, 1010: 11-bit data, 1011: 12-bit data, 1100: 13-bit data, 1101: 14-bit data, 1110: 15-bit data, 1111: 16-bit data"]
pub type DSS_W<'a, REG> = crate::FieldWriter<'a, REG, 4>;
#[doc = "Field `frf` reader - Frame format - 00: Motorola SPI frame format, 01: TI synchronous serial frame format, 10: National Microwire frame format, 11: Reserved"]
pub type FRF_R = crate::FieldReader;
#[doc = "Field `frf` writer - Frame format - 00: Motorola SPI frame format, 01: TI synchronous serial frame format, 10: National Microwire frame format, 11: Reserved"]
pub type FRF_W<'a, REG> = crate::FieldWriter<'a, REG, 2>;
#[doc = "Field `spo` reader - SSPCLKOUT polarity, applicable to Motorola SPI frame format only."]
pub type SPO_R = crate::BitReader;
#[doc = "Field `spo` writer - SSPCLKOUT polarity, applicable to Motorola SPI frame format only."]
pub type SPO_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `sph` reader - SSPCLKOUT phase, applicable to Motorola SPI frame format only."]
pub type SPH_R = crate::BitReader;
#[doc = "Field `sph` writer - SSPCLKOUT phase, applicable to Motorola SPI frame format only."]
pub type SPH_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `scr` reader - Serial clock rate. The value SCR is used to generate the transmit and receive bit rate of the PrimeCell SSP. The bit rate is: (F\\[sspclk\\]
/ (CPSDVR * (1 + SCR))), where CPSDVSR is an even value from \\[2:254\\], programmed through the SSPCPSR register and SCR is a value from \\[0:255\\]"]
pub type SCR_R = crate::FieldReader;
#[doc = "Field `scr` writer - Serial clock rate. The value SCR is used to generate the transmit and receive bit rate of the PrimeCell SSP. The bit rate is: (F\\[sspclk\\]
/ (CPSDVR * (1 + SCR))), where CPSDVSR is an even value from \\[2:254\\], programmed through the SSPCPSR register and SCR is a value from \\[0:255\\]"]
pub type SCR_W<'a, REG> = crate::FieldWriter<'a, REG, 8>;
impl R {
    #[doc = "Bits 0:3 - Data Size Select - 0000, 0001, 0010: Reserved, 0011: 4-bit data, 0100: 5-bit data, 6-bit data, 0110: 7-bit data, 0111: 8-bit data, 1000: 9-bit data, 1001: 10-bit data, 1010: 11-bit data, 1011: 12-bit data, 1100: 13-bit data, 1101: 14-bit data, 1110: 15-bit data, 1111: 16-bit data"]
    #[inline(always)]
    pub fn dss(&self) -> DSS_R {
        DSS_R::new((self.bits & 0x0f) as u8)
    }
    #[doc = "Bits 4:5 - Frame format - 00: Motorola SPI frame format, 01: TI synchronous serial frame format, 10: National Microwire frame format, 11: Reserved"]
    #[inline(always)]
    pub fn frf(&self) -> FRF_R {
        FRF_R::new(((self.bits >> 4) & 3) as u8)
    }
    #[doc = "Bit 6 - SSPCLKOUT polarity, applicable to Motorola SPI frame format only."]
    #[inline(always)]
    pub fn spo(&self) -> SPO_R {
        SPO_R::new(((self.bits >> 6) & 1) != 0)
    }
    #[doc = "Bit 6 - SSPCLKOUT phase, applicable to Motorola SPI frame format only."]
    #[inline(always)]
    pub fn sph(&self) -> SPH_R {
        SPH_R::new(((self.bits >> 6) & 1) != 0)
    }
    #[doc = "Bits 8:15 - Serial clock rate. The value SCR is used to generate the transmit and receive bit rate of the PrimeCell SSP. The bit rate is: (F\\[sspclk\\]
/ (CPSDVR * (1 + SCR))), where CPSDVSR is an even value from \\[2:254\\], programmed through the SSPCPSR register and SCR is a value from \\[0:255\\]"]
    #[inline(always)]
    pub fn scr(&self) -> SCR_R {
        SCR_R::new(((self.bits >> 8) & 0xff) as u8)
    }
}
impl W {
    #[doc = "Bits 0:3 - Data Size Select - 0000, 0001, 0010: Reserved, 0011: 4-bit data, 0100: 5-bit data, 6-bit data, 0110: 7-bit data, 0111: 8-bit data, 1000: 9-bit data, 1001: 10-bit data, 1010: 11-bit data, 1011: 12-bit data, 1100: 13-bit data, 1101: 14-bit data, 1110: 15-bit data, 1111: 16-bit data"]
    #[inline(always)]
    #[must_use]
    pub fn dss(&mut self) -> DSS_W<SSP_CR0_SPEC> {
        DSS_W::new(self, 0)
    }
    #[doc = "Bits 4:5 - Frame format - 00: Motorola SPI frame format, 01: TI synchronous serial frame format, 10: National Microwire frame format, 11: Reserved"]
    #[inline(always)]
    #[must_use]
    pub fn frf(&mut self) -> FRF_W<SSP_CR0_SPEC> {
        FRF_W::new(self, 4)
    }
    #[doc = "Bit 6 - SSPCLKOUT polarity, applicable to Motorola SPI frame format only."]
    #[inline(always)]
    #[must_use]
    pub fn spo(&mut self) -> SPO_W<SSP_CR0_SPEC> {
        SPO_W::new(self, 6)
    }
    #[doc = "Bit 6 - SSPCLKOUT phase, applicable to Motorola SPI frame format only."]
    #[inline(always)]
    #[must_use]
    pub fn sph(&mut self) -> SPH_W<SSP_CR0_SPEC> {
        SPH_W::new(self, 6)
    }
    #[doc = "Bits 8:15 - Serial clock rate. The value SCR is used to generate the transmit and receive bit rate of the PrimeCell SSP. The bit rate is: (F\\[sspclk\\]
/ (CPSDVR * (1 + SCR))), where CPSDVSR is an even value from \\[2:254\\], programmed through the SSPCPSR register and SCR is a value from \\[0:255\\]"]
    #[inline(always)]
    #[must_use]
    pub fn scr(&mut self) -> SCR_W<SSP_CR0_SPEC> {
        SCR_W::new(self, 8)
    }
    #[doc = r" Writes raw bits to the register."]
    #[doc = r""]
    #[doc = r" # Safety"]
    #[doc = r""]
    #[doc = r" Passing incorrect value can cause undefined behaviour. See reference manual"]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u16) -> &mut Self {
        self.bits = bits;
        self
    }
}
#[doc = "SSPCR0 is control register 0 and contains five bit fields that control various functions within the PrimeCell SSP.\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`ssp_cr0::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`ssp_cr0::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct SSP_CR0_SPEC;
impl crate::RegisterSpec for SSP_CR0_SPEC {
    type Ux = u16;
}
#[doc = "`read()` method returns [`ssp_cr0::R`](R) reader structure"]
impl crate::Readable for SSP_CR0_SPEC {}
#[doc = "`write(|w| ..)` method takes [`ssp_cr0::W`](W) writer structure"]
impl crate::Writable for SSP_CR0_SPEC {
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets ssp_cr0 to value 0"]
impl crate::Resettable for SSP_CR0_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
