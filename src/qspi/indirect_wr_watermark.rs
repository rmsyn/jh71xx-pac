#[doc = "Register `indirect_wr_watermark` reader"]
pub type R = crate::R<INDIRECT_WR_WATERMARK_SPEC>;
#[doc = "Register `indirect_wr_watermark` writer"]
pub type W = crate::W<INDIRECT_WR_WATERMARK_SPEC>;
#[doc = "Field `watermark` reader - watermark"]
pub type WATERMARK_R = crate::FieldReader<u32>;
#[doc = "Field `watermark` writer - watermark"]
pub type WATERMARK_W<'a, REG> = crate::FieldWriter<'a, REG, 32, u32>;
impl R {
    #[doc = "Bits 0:31 - watermark"]
    #[inline(always)]
    pub fn watermark(&self) -> WATERMARK_R {
        WATERMARK_R::new(self.bits)
    }
}
impl W {
    #[doc = "Bits 0:31 - watermark"]
    #[inline(always)]
    #[must_use]
    pub fn watermark(&mut self) -> WATERMARK_W<INDIRECT_WR_WATERMARK_SPEC> {
        WATERMARK_W::new(self, 0)
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
#[doc = "Cadence QSPI Indirect Write Watermark\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`indirect_wr_watermark::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`indirect_wr_watermark::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct INDIRECT_WR_WATERMARK_SPEC;
impl crate::RegisterSpec for INDIRECT_WR_WATERMARK_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`indirect_wr_watermark::R`](R) reader structure"]
impl crate::Readable for INDIRECT_WR_WATERMARK_SPEC {}
#[doc = "`write(|w| ..)` method takes [`indirect_wr_watermark::W`](W) writer structure"]
impl crate::Writable for INDIRECT_WR_WATERMARK_SPEC {
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets indirect_wr_watermark to value 0"]
impl crate::Resettable for INDIRECT_WR_WATERMARK_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
