#[doc = "Register `indirect_wr_bytes` reader"]
pub type R = crate::R<INDIRECT_WR_BYTES_SPEC>;
#[doc = "Register `indirect_wr_bytes` writer"]
pub type W = crate::W<INDIRECT_WR_BYTES_SPEC>;
#[doc = "Field `bytes` reader - bytes"]
pub type BYTES_R = crate::FieldReader<u32>;
#[doc = "Field `bytes` writer - bytes"]
pub type BYTES_W<'a, REG> = crate::FieldWriter<'a, REG, 32, u32>;
impl R {
    #[doc = "Bits 0:31 - bytes"]
    #[inline(always)]
    pub fn bytes(&self) -> BYTES_R {
        BYTES_R::new(self.bits)
    }
}
impl W {
    #[doc = "Bits 0:31 - bytes"]
    #[inline(always)]
    #[must_use]
    pub fn bytes(&mut self) -> BYTES_W<INDIRECT_WR_BYTES_SPEC> {
        BYTES_W::new(self, 0)
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
#[doc = "Cadence QSPI Indirect Write Bytes\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`indirect_wr_bytes::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`indirect_wr_bytes::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct INDIRECT_WR_BYTES_SPEC;
impl crate::RegisterSpec for INDIRECT_WR_BYTES_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`indirect_wr_bytes::R`](R) reader structure"]
impl crate::Readable for INDIRECT_WR_BYTES_SPEC {}
#[doc = "`write(|w| ..)` method takes [`indirect_wr_bytes::W`](W) writer structure"]
impl crate::Writable for INDIRECT_WR_BYTES_SPEC {
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets indirect_wr_bytes to value 0"]
impl crate::Resettable for INDIRECT_WR_BYTES_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
