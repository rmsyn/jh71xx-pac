#[doc = "Register `cmd_write_at_upper` reader"]
pub type R = crate::R<CMD_WRITE_AT_UPPER_SPEC>;
#[doc = "Register `cmd_write_at_upper` writer"]
pub type W = crate::W<CMD_WRITE_AT_UPPER_SPEC>;
#[doc = "Field `write_at_upper` reader - write_at_upper"]
pub type WRITE_AT_UPPER_R = crate::FieldReader<u32>;
#[doc = "Field `write_at_upper` writer - write_at_upper"]
pub type WRITE_AT_UPPER_W<'a, REG> = crate::FieldWriter<'a, REG, 32, u32>;
impl R {
    #[doc = "Bits 0:31 - write_at_upper"]
    #[inline(always)]
    pub fn write_at_upper(&self) -> WRITE_AT_UPPER_R {
        WRITE_AT_UPPER_R::new(self.bits)
    }
}
impl W {
    #[doc = "Bits 0:31 - write_at_upper"]
    #[inline(always)]
    #[must_use]
    pub fn write_at_upper(&mut self) -> WRITE_AT_UPPER_W<CMD_WRITE_AT_UPPER_SPEC> {
        WRITE_AT_UPPER_W::new(self, 0)
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
#[doc = "Cadence QSPI Command Write at Upper\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`cmd_write_at_upper::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`cmd_write_at_upper::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct CMD_WRITE_AT_UPPER_SPEC;
impl crate::RegisterSpec for CMD_WRITE_AT_UPPER_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`cmd_write_at_upper::R`](R) reader structure"]
impl crate::Readable for CMD_WRITE_AT_UPPER_SPEC {}
#[doc = "`write(|w| ..)` method takes [`cmd_write_at_upper::W`](W) writer structure"]
impl crate::Writable for CMD_WRITE_AT_UPPER_SPEC {
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets cmd_write_at_upper to value 0"]
impl crate::Resettable for CMD_WRITE_AT_UPPER_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
