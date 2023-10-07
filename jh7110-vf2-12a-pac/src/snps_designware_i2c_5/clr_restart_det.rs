#[doc = "Register `clr_restart_det` reader"]
pub type R = crate::R<CLR_RESTART_DET_SPEC>;
#[doc = "Register `clr_restart_det` writer"]
pub type W = crate::W<CLR_RESTART_DET_SPEC>;
#[doc = "Field `clr_restart_det` reader - clr_restart_det"]
pub type CLR_RESTART_DET_R = crate::FieldReader<u32>;
#[doc = "Field `clr_restart_det` writer - clr_restart_det"]
pub type CLR_RESTART_DET_W<'a, REG, const O: u8> = crate::FieldWriter<'a, REG, 32, O, u32>;
impl R {
    #[doc = "Bits 0:31 - clr_restart_det"]
    #[inline(always)]
    pub fn clr_restart_det(&self) -> CLR_RESTART_DET_R {
        CLR_RESTART_DET_R::new(self.bits)
    }
}
impl W {
    #[doc = "Bits 0:31 - clr_restart_det"]
    #[inline(always)]
    #[must_use]
    pub fn clr_restart_det(&mut self) -> CLR_RESTART_DET_W<CLR_RESTART_DET_SPEC, 0> {
        CLR_RESTART_DET_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.bits = bits;
        self
    }
}
#[doc = "DesignWare I2C Clear Restart DET\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`clr_restart_det::R`](R).  You can [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`clr_restart_det::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct CLR_RESTART_DET_SPEC;
impl crate::RegisterSpec for CLR_RESTART_DET_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`clr_restart_det::R`](R) reader structure"]
impl crate::Readable for CLR_RESTART_DET_SPEC {}
#[doc = "`write(|w| ..)` method takes [`clr_restart_det::W`](W) writer structure"]
impl crate::Writable for CLR_RESTART_DET_SPEC {
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
