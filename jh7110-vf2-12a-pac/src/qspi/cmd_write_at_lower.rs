#[doc = "Register `cmd_write_at_lower` reader"]
pub type R = crate::R<CMD_WRITE_AT_LOWER_SPEC>;
#[doc = "Register `cmd_write_at_lower` writer"]
pub type W = crate::W<CMD_WRITE_AT_LOWER_SPEC>;
#[doc = "Field `write_at_lower` reader - write_at_lower"]
pub type WRITE_AT_LOWER_R = crate::FieldReader<u32>;
#[doc = "Field `write_at_lower` writer - write_at_lower"]
pub type WRITE_AT_LOWER_W<'a, REG, const O: u8> = crate::FieldWriter<'a, REG, 32, O, u32>;
impl R {
    #[doc = "Bits 0:31 - write_at_lower"]
    #[inline(always)]
    pub fn write_at_lower(&self) -> WRITE_AT_LOWER_R {
        WRITE_AT_LOWER_R::new(self.bits)
    }
}
impl W {
    #[doc = "Bits 0:31 - write_at_lower"]
    #[inline(always)]
    #[must_use]
    pub fn write_at_lower(&mut self) -> WRITE_AT_LOWER_W<CMD_WRITE_AT_LOWER_SPEC, 0> {
        WRITE_AT_LOWER_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.bits = bits;
        self
    }
}
#[doc = "Cadence QSPI Command Write at Lower\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`cmd_write_at_lower::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`cmd_write_at_lower::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct CMD_WRITE_AT_LOWER_SPEC;
impl crate::RegisterSpec for CMD_WRITE_AT_LOWER_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`cmd_write_at_lower::R`](R) reader structure"]
impl crate::Readable for CMD_WRITE_AT_LOWER_SPEC {}
#[doc = "`write(|w| ..)` method takes [`cmd_write_at_lower::W`](W) writer structure"]
impl crate::Writable for CMD_WRITE_AT_LOWER_SPEC {
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets cmd_write_at_lower to value 0"]
impl crate::Resettable for CMD_WRITE_AT_LOWER_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
