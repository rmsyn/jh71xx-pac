#[doc = "Register `ssp_periph_id3` reader"]
pub type R = crate::R<SSP_PERIPH_ID3_SPEC>;
#[doc = "Register `ssp_periph_id3` writer"]
pub type W = crate::W<SSP_PERIPH_ID3_SPEC>;
#[doc = "Field `configuration` reader - These bits read back as 0x80"]
pub type CONFIGURATION_R = crate::FieldReader;
impl R {
    #[doc = "Bits 0:7 - These bits read back as 0x80"]
    #[inline(always)]
    pub fn configuration(&self) -> CONFIGURATION_R {
        CONFIGURATION_R::new((self.bits & 0xff) as u8)
    }
}
impl W {
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
#[doc = "The SSPPeriphID3 register is hard-coded and the fields within the register determine reset value. The SSPPeriphID0-3 registers are four 8-bit registers, that span address locations 0xFE0 to 0xFEC. The registers can conceptually be treated as a single 32-bit register.\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`ssp_periph_id3::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`ssp_periph_id3::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct SSP_PERIPH_ID3_SPEC;
impl crate::RegisterSpec for SSP_PERIPH_ID3_SPEC {
    type Ux = u16;
}
#[doc = "`read()` method returns [`ssp_periph_id3::R`](R) reader structure"]
impl crate::Readable for SSP_PERIPH_ID3_SPEC {}
#[doc = "`write(|w| ..)` method takes [`ssp_periph_id3::W`](W) writer structure"]
impl crate::Writable for SSP_PERIPH_ID3_SPEC {
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets ssp_periph_id3 to value 0"]
impl crate::Resettable for SSP_PERIPH_ID3_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
