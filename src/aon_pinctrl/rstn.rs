#[doc = "Register `rstn` reader"]
pub type R = crate::R<RSTN_SPEC>;
#[doc = "Register `rstn` writer"]
pub type W = crate::W<RSTN_SPEC>;
#[doc = "Field `smt` reader - Active high Schmitt (SMT) trigger selector - 0: No hysteresis, 1: Schmitt trigger enabled"]
pub type SMT_R = crate::BitReader;
#[doc = "Field `smt` writer - Active high Schmitt (SMT) trigger selector - 0: No hysteresis, 1: Schmitt trigger enabled"]
pub type SMT_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `pos` reader - Power-on-Start (POS) enabler - 1: Enable active pull-down for loss of core power, 0: Active pull-down capability disabled"]
pub type POS_R = crate::BitReader;
#[doc = "Field `pos` writer - Power-on-Start (POS) enabler - 1: Enable active pull-down for loss of core power, 0: Active pull-down capability disabled"]
pub type POS_W<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bit 0 - Active high Schmitt (SMT) trigger selector - 0: No hysteresis, 1: Schmitt trigger enabled"]
    #[inline(always)]
    pub fn smt(&self) -> SMT_R {
        SMT_R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - Power-on-Start (POS) enabler - 1: Enable active pull-down for loss of core power, 0: Active pull-down capability disabled"]
    #[inline(always)]
    pub fn pos(&self) -> POS_R {
        POS_R::new(((self.bits >> 1) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - Active high Schmitt (SMT) trigger selector - 0: No hysteresis, 1: Schmitt trigger enabled"]
    #[inline(always)]
    #[must_use]
    pub fn smt(&mut self) -> SMT_W<RSTN_SPEC> {
        SMT_W::new(self, 0)
    }
    #[doc = "Bit 1 - Power-on-Start (POS) enabler - 1: Enable active pull-down for loss of core power, 0: Active pull-down capability disabled"]
    #[inline(always)]
    #[must_use]
    pub fn pos(&mut self) -> POS_W<RSTN_SPEC> {
        POS_W::new(self, 1)
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
#[doc = "AON IOMUX CFG SAIF SYSCFG 68\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`rstn::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`rstn::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct RSTN_SPEC;
impl crate::RegisterSpec for RSTN_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`rstn::R`](R) reader structure"]
impl crate::Readable for RSTN_SPEC {}
#[doc = "`write(|w| ..)` method takes [`rstn::W`](W) writer structure"]
impl crate::Writable for RSTN_SPEC {
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets rstn to value 0"]
impl crate::Resettable for RSTN_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
