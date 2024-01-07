#[doc = "Register `dma` reader"]
pub type R = crate::R<DMA_SPEC>;
#[doc = "Register `dma` writer"]
pub type W = crate::W<DMA_SPEC>;
#[doc = "Field `single` reader - single"]
pub type SINGLE_R = crate::FieldReader;
#[doc = "Field `single` writer - single"]
pub type SINGLE_W<'a, REG> = crate::FieldWriter<'a, REG, 8>;
#[doc = "Field `burst` reader - burst"]
pub type BURST_R = crate::FieldReader;
#[doc = "Field `burst` writer - burst"]
pub type BURST_W<'a, REG> = crate::FieldWriter<'a, REG, 8>;
impl R {
    #[doc = "Bits 0:7 - single"]
    #[inline(always)]
    pub fn single(&self) -> SINGLE_R {
        SINGLE_R::new((self.bits & 0xff) as u8)
    }
    #[doc = "Bits 8:15 - burst"]
    #[inline(always)]
    pub fn burst(&self) -> BURST_R {
        BURST_R::new(((self.bits >> 8) & 0xff) as u8)
    }
}
impl W {
    #[doc = "Bits 0:7 - single"]
    #[inline(always)]
    #[must_use]
    pub fn single(&mut self) -> SINGLE_W<DMA_SPEC> {
        SINGLE_W::new(self, 0)
    }
    #[doc = "Bits 8:15 - burst"]
    #[inline(always)]
    #[must_use]
    pub fn burst(&mut self) -> BURST_W<DMA_SPEC> {
        BURST_W::new(self, 8)
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
#[doc = "Cadence QSPI Direct Memory Access\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`dma::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`dma::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct DMA_SPEC;
impl crate::RegisterSpec for DMA_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`dma::R`](R) reader structure"]
impl crate::Readable for DMA_SPEC {}
#[doc = "`write(|w| ..)` method takes [`dma::W`](W) writer structure"]
impl crate::Writable for DMA_SPEC {
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets dma to value 0"]
impl crate::Resettable for DMA_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
