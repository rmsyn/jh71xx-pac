#[doc = "Register `ssp_imsc` reader"]
pub type R = crate::R<SSP_IMSC_SPEC>;
#[doc = "Register `ssp_imsc` writer"]
pub type W = crate::W<SSP_IMSC_SPEC>;
#[doc = "Field `rorim` reader - Receive overrun interrupt mask - 0: Receive FIFO written to while full condition interrupt is masked, 1: Receive FIFO written to while full condition interrupt is not masked"]
pub type RORIM_R = crate::BitReader;
#[doc = "Field `rorim` writer - Receive overrun interrupt mask - 0: Receive FIFO written to while full condition interrupt is masked, 1: Receive FIFO written to while full condition interrupt is not masked"]
pub type RORIM_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O>;
#[doc = "Field `rtim` reader - Receive timeout interrupt mask - 0: Receive FIFO not empty and no read prior to timeout period interrupt is masked, 1: Receive FIFO not empty and no read prior to timeout period interrupt is not masked"]
pub type RTIM_R = crate::BitReader;
#[doc = "Field `rtim` writer - Receive timeout interrupt mask - 0: Receive FIFO not empty and no read prior to timeout period interrupt is masked, 1: Receive FIFO not empty and no read prior to timeout period interrupt is not masked"]
pub type RTIM_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O>;
#[doc = "Field `rxim` reader - Receive FIFO interrupt mask - 0: Receive FIFO half full or less condition interrupt is masked, 1: Receive FIFO half full or less condition interrupt is not masked"]
pub type RXIM_R = crate::BitReader;
#[doc = "Field `rxim` writer - Receive FIFO interrupt mask - 0: Receive FIFO half full or less condition interrupt is masked, 1: Receive FIFO half full or less condition interrupt is not masked"]
pub type RXIM_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O>;
#[doc = "Field `txim` reader - Transmit FIFO interrupt mask - 0: Transmit FIFO half empty or less condition interrupt is masked, 1: Transmit FIFO half empty or less condition interrupt is not masked"]
pub type TXIM_R = crate::BitReader;
#[doc = "Field `txim` writer - Transmit FIFO interrupt mask - 0: Transmit FIFO half empty or less condition interrupt is masked, 1: Transmit FIFO half empty or less condition interrupt is not masked"]
pub type TXIM_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O>;
impl R {
    #[doc = "Bit 0 - Receive overrun interrupt mask - 0: Receive FIFO written to while full condition interrupt is masked, 1: Receive FIFO written to while full condition interrupt is not masked"]
    #[inline(always)]
    pub fn rorim(&self) -> RORIM_R {
        RORIM_R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - Receive timeout interrupt mask - 0: Receive FIFO not empty and no read prior to timeout period interrupt is masked, 1: Receive FIFO not empty and no read prior to timeout period interrupt is not masked"]
    #[inline(always)]
    pub fn rtim(&self) -> RTIM_R {
        RTIM_R::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - Receive FIFO interrupt mask - 0: Receive FIFO half full or less condition interrupt is masked, 1: Receive FIFO half full or less condition interrupt is not masked"]
    #[inline(always)]
    pub fn rxim(&self) -> RXIM_R {
        RXIM_R::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 2 - Transmit FIFO interrupt mask - 0: Transmit FIFO half empty or less condition interrupt is masked, 1: Transmit FIFO half empty or less condition interrupt is not masked"]
    #[inline(always)]
    pub fn txim(&self) -> TXIM_R {
        TXIM_R::new(((self.bits >> 2) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - Receive overrun interrupt mask - 0: Receive FIFO written to while full condition interrupt is masked, 1: Receive FIFO written to while full condition interrupt is not masked"]
    #[inline(always)]
    #[must_use]
    pub fn rorim(&mut self) -> RORIM_W<SSP_IMSC_SPEC, 0> {
        RORIM_W::new(self)
    }
    #[doc = "Bit 1 - Receive timeout interrupt mask - 0: Receive FIFO not empty and no read prior to timeout period interrupt is masked, 1: Receive FIFO not empty and no read prior to timeout period interrupt is not masked"]
    #[inline(always)]
    #[must_use]
    pub fn rtim(&mut self) -> RTIM_W<SSP_IMSC_SPEC, 1> {
        RTIM_W::new(self)
    }
    #[doc = "Bit 2 - Receive FIFO interrupt mask - 0: Receive FIFO half full or less condition interrupt is masked, 1: Receive FIFO half full or less condition interrupt is not masked"]
    #[inline(always)]
    #[must_use]
    pub fn rxim(&mut self) -> RXIM_W<SSP_IMSC_SPEC, 2> {
        RXIM_W::new(self)
    }
    #[doc = "Bit 2 - Transmit FIFO interrupt mask - 0: Transmit FIFO half empty or less condition interrupt is masked, 1: Transmit FIFO half empty or less condition interrupt is not masked"]
    #[inline(always)]
    #[must_use]
    pub fn txim(&mut self) -> TXIM_W<SSP_IMSC_SPEC, 2> {
        TXIM_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u16) -> &mut Self {
        self.bits = bits;
        self
    }
}
#[doc = "The SSPIMSC register is the interrupt mask set or clear register. It is a RW register. On a read this register gives the current value of the mask on the relevant interrupt. A write of 1 to the particular bit sets the mask, enabling the interrupt to be read. A write of 0 clears the corresponding mask. All the bits are cleared to 0 when reset.\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`ssp_imsc::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`ssp_imsc::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct SSP_IMSC_SPEC;
impl crate::RegisterSpec for SSP_IMSC_SPEC {
    type Ux = u16;
}
#[doc = "`read()` method returns [`ssp_imsc::R`](R) reader structure"]
impl crate::Readable for SSP_IMSC_SPEC {}
#[doc = "`write(|w| ..)` method takes [`ssp_imsc::W`](W) writer structure"]
impl crate::Writable for SSP_IMSC_SPEC {
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets ssp_imsc to value 0"]
impl crate::Resettable for SSP_IMSC_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
