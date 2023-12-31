#[doc = "Register `ioirq_0` reader"]
pub type R = crate::R<IOIRQ_0_SPEC>;
#[doc = "Register `ioirq_0` writer"]
pub type W = crate::W<IOIRQ_0_SPEC>;
#[doc = "Field `gpen_0` reader - 1: Enable, 0: Disable"]
pub type GPEN_0_R = crate::BitReader;
#[doc = "Field `gpen_0` writer - 1: Enable, 0: Disable"]
pub type GPEN_0_W<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bit 0 - 1: Enable, 0: Disable"]
    #[inline(always)]
    pub fn gpen_0(&self) -> GPEN_0_R {
        GPEN_0_R::new((self.bits & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - 1: Enable, 0: Disable"]
    #[inline(always)]
    #[must_use]
    pub fn gpen_0(&mut self) -> GPEN_0_W<IOIRQ_0_SPEC> {
        GPEN_0_W::new(self, 0)
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
#[doc = "Enable GPIO IRQ function\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`ioirq_0::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`ioirq_0::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct IOIRQ_0_SPEC;
impl crate::RegisterSpec for IOIRQ_0_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`ioirq_0::R`](R) reader structure"]
impl crate::Readable for IOIRQ_0_SPEC {}
#[doc = "`write(|w| ..)` method takes [`ioirq_0::W`](W) writer structure"]
impl crate::Writable for IOIRQ_0_SPEC {
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets ioirq_0 to value 0"]
impl crate::Resettable for IOIRQ_0_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
