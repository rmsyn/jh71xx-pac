#[doc = "Register `ioirq0` reader"]
pub type R = crate::R<IOIRQ0_SPEC>;
#[doc = "Register `ioirq0` writer"]
pub type W = crate::W<IOIRQ0_SPEC>;
#[doc = "Field `gpioen0` reader - 1: Enable, 0: Disable"]
pub type GPIOEN0_R = crate::BitReader;
#[doc = "Field `gpioen0` writer - 1: Enable, 0: Disable"]
pub type GPIOEN0_W<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bit 0 - 1: Enable, 0: Disable"]
    #[inline(always)]
    pub fn gpioen0(&self) -> GPIOEN0_R {
        GPIOEN0_R::new((self.bits & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - 1: Enable, 0: Disable"]
    #[inline(always)]
    #[must_use]
    pub fn gpioen0(&mut self) -> GPIOEN0_W<IOIRQ0_SPEC> {
        GPIOEN0_W::new(self, 0)
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
#[doc = "Enable GPIO IRQ function\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`ioirq0::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`ioirq0::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct IOIRQ0_SPEC;
impl crate::RegisterSpec for IOIRQ0_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`ioirq0::R`](R) reader structure"]
impl crate::Readable for IOIRQ0_SPEC {}
#[doc = "`write(|w| ..)` method takes [`ioirq0::W`](W) writer structure"]
impl crate::Writable for IOIRQ0_SPEC {
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets ioirq0 to value 0"]
impl crate::Resettable for IOIRQ0_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
