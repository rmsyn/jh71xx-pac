#[doc = "Register `clr_rd_req` reader"]
pub type R = crate::R<CLR_RD_REQ_SPEC>;
#[doc = "Register `clr_rd_req` writer"]
pub type W = crate::W<CLR_RD_REQ_SPEC>;
#[doc = "Field `clr_rd_req` reader - clr_rd_req"]
pub type CLR_RD_REQ_R = crate::FieldReader<u32>;
#[doc = "Field `clr_rd_req` writer - clr_rd_req"]
pub type CLR_RD_REQ_W<'a, REG, const O: u8> = crate::FieldWriter<'a, REG, 32, O, u32>;
impl R {
    #[doc = "Bits 0:31 - clr_rd_req"]
    #[inline(always)]
    pub fn clr_rd_req(&self) -> CLR_RD_REQ_R {
        CLR_RD_REQ_R::new(self.bits)
    }
}
impl W {
    #[doc = "Bits 0:31 - clr_rd_req"]
    #[inline(always)]
    #[must_use]
    pub fn clr_rd_req(&mut self) -> CLR_RD_REQ_W<CLR_RD_REQ_SPEC, 0> {
        CLR_RD_REQ_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.bits = bits;
        self
    }
}
#[doc = "DesignWare I2C Clear Read Request\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`clr_rd_req::R`](R).  You can [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`clr_rd_req::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct CLR_RD_REQ_SPEC;
impl crate::RegisterSpec for CLR_RD_REQ_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`clr_rd_req::R`](R) reader structure"]
impl crate::Readable for CLR_RD_REQ_SPEC {}
#[doc = "`write(|w| ..)` method takes [`clr_rd_req::W`](W) writer structure"]
impl crate::Writable for CLR_RD_REQ_SPEC {
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
