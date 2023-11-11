#[doc = "Register `iir` reader"]
pub type R = crate::R<IIR_SPEC>;
#[doc = "Register `iir` writer"]
pub type W = crate::W<IIR_SPEC>;
#[doc = "Field `iid` reader - Interrupt ID. This indicates the highest priority pending interrupt which can be one of the following types: 0000 = modem status 0001 = no interrupt pending 0010 = THR empty 0100 = received data available 0110 = receiver line status 0111 = busy detect 1100 = character timeout The interrupt priorities are split into four levels that are detailed in Table 8 on page 97. Bit 3 indicates an interrupt can only occur when the FIFOs are enabled and used to distinguish a Character Timeout condition interrupt."]
pub type IID_R = crate::FieldReader;
#[doc = "Field `fifose` reader - FIFOs Enabled. This is used to indicate whether the FIFOs are enabled or disabled. 00 = disabled 11 = enabled"]
pub type FIFOSE_R = crate::FieldReader;
impl R {
    #[doc = "Bits 0:3 - Interrupt ID. This indicates the highest priority pending interrupt which can be one of the following types: 0000 = modem status 0001 = no interrupt pending 0010 = THR empty 0100 = received data available 0110 = receiver line status 0111 = busy detect 1100 = character timeout The interrupt priorities are split into four levels that are detailed in Table 8 on page 97. Bit 3 indicates an interrupt can only occur when the FIFOs are enabled and used to distinguish a Character Timeout condition interrupt."]
    #[inline(always)]
    pub fn iid(&self) -> IID_R {
        IID_R::new((self.bits & 0x0f) as u8)
    }
    #[doc = "Bits 6:7 - FIFOs Enabled. This is used to indicate whether the FIFOs are enabled or disabled. 00 = disabled 11 = enabled"]
    #[inline(always)]
    pub fn fifose(&self) -> FIFOSE_R {
        FIFOSE_R::new(((self.bits >> 6) & 3) as u8)
    }
}
impl W {
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.bits = bits;
        self
    }
}
#[doc = "Interrupt Identity Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`iir::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`iir::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct IIR_SPEC;
impl crate::RegisterSpec for IIR_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`iir::R`](R) reader structure"]
impl crate::Readable for IIR_SPEC {}
#[doc = "`write(|w| ..)` method takes [`iir::W`](W) writer structure"]
impl crate::Writable for IIR_SPEC {
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets iir to value 0x01"]
impl crate::Resettable for IIR_SPEC {
    const RESET_VALUE: Self::Ux = 0x01;
}
