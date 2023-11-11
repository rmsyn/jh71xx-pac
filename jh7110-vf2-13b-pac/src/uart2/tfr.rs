#[doc = "Register `tfr` reader"]
pub type R = crate::R<TFR_SPEC>;
#[doc = "Register `tfr` writer"]
pub type W = crate::W<TFR_SPEC>;
#[doc = "Field `tfr` reader - Transmit FIFO Read. These bits are only valid when FIFO access mode is enabled (FAR\\[0\\]
is set to one). When FIFOs are implemented and enabled, reading this register gives the data at the top of the transmit FIFO. Each consecutive read pops the transmit FIFO and gives the next data value that is currently at the top of the FIFO. When FIFOs are not implemented or not enabled, reading this register gives the data in the THR."]
pub type TFR_R = crate::FieldReader;
impl R {
    #[doc = "Bits 0:7 - Transmit FIFO Read. These bits are only valid when FIFO access mode is enabled (FAR\\[0\\]
is set to one). When FIFOs are implemented and enabled, reading this register gives the data at the top of the transmit FIFO. Each consecutive read pops the transmit FIFO and gives the next data value that is currently at the top of the FIFO. When FIFOs are not implemented or not enabled, reading this register gives the data in the THR."]
    #[inline(always)]
    pub fn tfr(&self) -> TFR_R {
        TFR_R::new((self.bits & 0xff) as u8)
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
#[doc = "Transmit FIFO Read\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`tfr::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`tfr::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct TFR_SPEC;
impl crate::RegisterSpec for TFR_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`tfr::R`](R) reader structure"]
impl crate::Readable for TFR_SPEC {}
#[doc = "`write(|w| ..)` method takes [`tfr::W`](W) writer structure"]
impl crate::Writable for TFR_SPEC {
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets tfr to value 0"]
impl crate::Resettable for TFR_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
