#[doc = "Register `cmd_address` reader"]
pub type R = crate::R<CMD_ADDRESS_SPEC>;
#[doc = "Register `cmd_address` writer"]
pub type W = crate::W<CMD_ADDRESS_SPEC>;
#[doc = "Field `address` reader - address"]
pub type ADDRESS_R = crate::FieldReader<u32>;
#[doc = "Field `address` writer - address"]
pub type ADDRESS_W<'a, REG> = crate::FieldWriter<'a, REG, 32, u32>;
impl R {
    #[doc = "Bits 0:31 - address"]
    #[inline(always)]
    pub fn address(&self) -> ADDRESS_R {
        ADDRESS_R::new(self.bits)
    }
}
impl W {
    #[doc = "Bits 0:31 - address"]
    #[inline(always)]
    #[must_use]
    pub fn address(&mut self) -> ADDRESS_W<CMD_ADDRESS_SPEC> {
        ADDRESS_W::new(self, 0)
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
#[doc = "Cadence QSPI Command Address\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`cmd_address::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`cmd_address::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct CMD_ADDRESS_SPEC;
impl crate::RegisterSpec for CMD_ADDRESS_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`cmd_address::R`](R) reader structure"]
impl crate::Readable for CMD_ADDRESS_SPEC {}
#[doc = "`write(|w| ..)` method takes [`cmd_address::W`](W) writer structure"]
impl crate::Writable for CMD_ADDRESS_SPEC {
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets cmd_address to value 0"]
impl crate::Resettable for CMD_ADDRESS_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
