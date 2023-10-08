#[doc = "Register `ssp_periph_id1` reader"]
pub type R = crate::R<SSP_PERIPH_ID1_SPEC>;
#[doc = "Register `ssp_periph_id1` writer"]
pub type W = crate::W<SSP_PERIPH_ID1_SPEC>;
#[doc = "Field `part_number1` reader - These bits read back as 0x0"]
pub type PART_NUMBER1_R = crate::FieldReader;
#[doc = "Field `designer0` reader - These bits read back as 0x1"]
pub type DESIGNER0_R = crate::FieldReader;
impl R {
    #[doc = "Bits 0:3 - These bits read back as 0x0"]
    #[inline(always)]
    pub fn part_number1(&self) -> PART_NUMBER1_R {
        PART_NUMBER1_R::new((self.bits & 0x0f) as u8)
    }
    #[doc = "Bits 4:7 - These bits read back as 0x1"]
    #[inline(always)]
    pub fn designer0(&self) -> DESIGNER0_R {
        DESIGNER0_R::new(((self.bits >> 4) & 0x0f) as u8)
    }
}
impl W {
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u16) -> &mut Self {
        self.bits = bits;
        self
    }
}
#[doc = "The SSPPeriphID1 register is hard-coded and the fields within the register determine reset value. The SSPPeriphID0-3 registers are four 8-bit registers, that span address locations 0xFE0 to 0xFEC. The registers can conceptually be treated as a single 32-bit register.\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`ssp_periph_id1::R`](R).  You can [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`ssp_periph_id1::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct SSP_PERIPH_ID1_SPEC;
impl crate::RegisterSpec for SSP_PERIPH_ID1_SPEC {
    type Ux = u16;
}
#[doc = "`read()` method returns [`ssp_periph_id1::R`](R) reader structure"]
impl crate::Readable for SSP_PERIPH_ID1_SPEC {}
#[doc = "`write(|w| ..)` method takes [`ssp_periph_id1::W`](W) writer structure"]
impl crate::Writable for SSP_PERIPH_ID1_SPEC {
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
