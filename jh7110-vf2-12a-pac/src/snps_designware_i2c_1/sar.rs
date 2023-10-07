#[doc = "Register `sar` reader"]
pub type R = crate::R<SAR_SPEC>;
#[doc = "Register `sar` writer"]
pub type W = crate::W<SAR_SPEC>;
#[doc = "Field `sar` reader - sar"]
pub type SAR_R = crate::FieldReader<u32>;
#[doc = "Field `sar` writer - sar"]
pub type SAR_W<'a, REG, const O: u8> = crate::FieldWriter<'a, REG, 32, O, u32>;
impl R {
    #[doc = "Bits 0:31 - sar"]
    #[inline(always)]
    pub fn sar(&self) -> SAR_R {
        SAR_R::new(self.bits)
    }
}
impl W {
    #[doc = "Bits 0:31 - sar"]
    #[inline(always)]
    #[must_use]
    pub fn sar(&mut self) -> SAR_W<SAR_SPEC, 0> {
        SAR_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.bits = bits;
        self
    }
}
#[doc = "DesignWare I2C SAR\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`sar::R`](R).  You can [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`sar::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct SAR_SPEC;
impl crate::RegisterSpec for SAR_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`sar::R`](R) reader structure"]
impl crate::Readable for SAR_SPEC {}
#[doc = "`write(|w| ..)` method takes [`sar::W`](W) writer structure"]
impl crate::Writable for SAR_SPEC {
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
