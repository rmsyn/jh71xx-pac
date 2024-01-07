#[doc = "Register `clr_gen_call` reader"]
pub type R = crate::R<CLR_GEN_CALL_SPEC>;
#[doc = "Register `clr_gen_call` writer"]
pub type W = crate::W<CLR_GEN_CALL_SPEC>;
#[doc = "Field `clr_gen_call` reader - clr_gen_call"]
pub type CLR_GEN_CALL_R = crate::FieldReader<u32>;
#[doc = "Field `clr_gen_call` writer - clr_gen_call"]
pub type CLR_GEN_CALL_W<'a, REG> = crate::FieldWriter<'a, REG, 32, u32>;
impl R {
    #[doc = "Bits 0:31 - clr_gen_call"]
    #[inline(always)]
    pub fn clr_gen_call(&self) -> CLR_GEN_CALL_R {
        CLR_GEN_CALL_R::new(self.bits)
    }
}
impl W {
    #[doc = "Bits 0:31 - clr_gen_call"]
    #[inline(always)]
    #[must_use]
    pub fn clr_gen_call(&mut self) -> CLR_GEN_CALL_W<CLR_GEN_CALL_SPEC> {
        CLR_GEN_CALL_W::new(self, 0)
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
#[doc = "DesignWare I2C Clear General Call\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`clr_gen_call::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`clr_gen_call::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct CLR_GEN_CALL_SPEC;
impl crate::RegisterSpec for CLR_GEN_CALL_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`clr_gen_call::R`](R) reader structure"]
impl crate::Readable for CLR_GEN_CALL_SPEC {}
#[doc = "`write(|w| ..)` method takes [`clr_gen_call::W`](W) writer structure"]
impl crate::Writable for CLR_GEN_CALL_SPEC {
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets clr_gen_call to value 0"]
impl crate::Resettable for CLR_GEN_CALL_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
