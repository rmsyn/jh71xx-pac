#[doc = "Register `clr_stop_det` reader"]
pub type R = crate::R<CLR_STOP_DET_SPEC>;
#[doc = "Register `clr_stop_det` writer"]
pub type W = crate::W<CLR_STOP_DET_SPEC>;
#[doc = "Field `clr_stop_det` reader - clr_stop_det"]
pub type CLR_STOP_DET_R = crate::FieldReader<u32>;
#[doc = "Field `clr_stop_det` writer - clr_stop_det"]
pub type CLR_STOP_DET_W<'a, REG> = crate::FieldWriter<'a, REG, 32, u32>;
impl R {
    #[doc = "Bits 0:31 - clr_stop_det"]
    #[inline(always)]
    pub fn clr_stop_det(&self) -> CLR_STOP_DET_R {
        CLR_STOP_DET_R::new(self.bits)
    }
}
impl W {
    #[doc = "Bits 0:31 - clr_stop_det"]
    #[inline(always)]
    #[must_use]
    pub fn clr_stop_det(&mut self) -> CLR_STOP_DET_W<CLR_STOP_DET_SPEC> {
        CLR_STOP_DET_W::new(self, 0)
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
#[doc = "DesignWare I2C Clear Stop DET\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`clr_stop_det::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`clr_stop_det::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct CLR_STOP_DET_SPEC;
impl crate::RegisterSpec for CLR_STOP_DET_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`clr_stop_det::R`](R) reader structure"]
impl crate::Readable for CLR_STOP_DET_SPEC {}
#[doc = "`write(|w| ..)` method takes [`clr_stop_det::W`](W) writer structure"]
impl crate::Writable for CLR_STOP_DET_SPEC {
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets clr_stop_det to value 0"]
impl crate::Resettable for CLR_STOP_DET_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
