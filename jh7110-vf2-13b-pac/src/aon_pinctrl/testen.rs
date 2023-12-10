#[doc = "Register `testen` reader"]
pub type R = crate::R<TESTEN_SPEC>;
#[doc = "Register `testen` writer"]
pub type W = crate::W<TESTEN_SPEC>;
#[doc = "Field `testen_pos` reader - Power-on-Start (POS) enabler - 1: Enable active pull down for loss of core power, 0: Active pull-down capability disabled"]
pub type TESTEN_POS_R = crate::BitReader;
#[doc = "Field `testen_pos` writer - Power-on-Start (POS) enabler - 1: Enable active pull down for loss of core power, 0: Active pull-down capability disabled"]
pub type TESTEN_POS_W<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bit 0 - Power-on-Start (POS) enabler - 1: Enable active pull down for loss of core power, 0: Active pull-down capability disabled"]
    #[inline(always)]
    pub fn testen_pos(&self) -> TESTEN_POS_R {
        TESTEN_POS_R::new((self.bits & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - Power-on-Start (POS) enabler - 1: Enable active pull down for loss of core power, 0: Active pull-down capability disabled"]
    #[inline(always)]
    #[must_use]
    pub fn testen_pos(&mut self) -> TESTEN_POS_W<TESTEN_SPEC> {
        TESTEN_POS_W::new(self, 0)
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
#[doc = "AON IOMUX CFG SAIF SYSCFG 48\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`testen::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`testen::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct TESTEN_SPEC;
impl crate::RegisterSpec for TESTEN_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`testen::R`](R) reader structure"]
impl crate::Readable for TESTEN_SPEC {}
#[doc = "`write(|w| ..)` method takes [`testen::W`](W) writer structure"]
impl crate::Writable for TESTEN_SPEC {
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets testen to value 0"]
impl crate::Resettable for TESTEN_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
