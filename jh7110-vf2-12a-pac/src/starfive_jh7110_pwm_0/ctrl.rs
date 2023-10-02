#[doc = "Register `ctrl` reader"]
pub type R = crate::R<CTRL_SPEC>;
#[doc = "Register `ctrl` writer"]
pub type W = crate::W<CTRL_SPEC>;
#[doc = "Field `en` reader - PWM PTC enable"]
pub type EN_R = crate::BitReader;
#[doc = "Field `en` writer - PWM PTC enable"]
pub type EN_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O>;
#[doc = "Field `eclk` reader - PWM PTC enable clock"]
pub type ECLK_R = crate::BitReader;
#[doc = "Field `eclk` writer - PWM PTC enable clock"]
pub type ECLK_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O>;
#[doc = "Field `nec` reader - PWM PTC nec"]
pub type NEC_R = crate::BitReader;
#[doc = "Field `nec` writer - PWM PTC nec"]
pub type NEC_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O>;
#[doc = "Field `oe` reader - PWM PTC oe"]
pub type OE_R = crate::BitReader;
#[doc = "Field `oe` writer - PWM PTC oe"]
pub type OE_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O>;
#[doc = "Field `single` reader - PWM PTC single"]
pub type SINGLE_R = crate::BitReader;
#[doc = "Field `single` writer - PWM PTC single"]
pub type SINGLE_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O>;
#[doc = "Field `inte` reader - PWM PTC interrupt enable"]
pub type INTE_R = crate::BitReader;
#[doc = "Field `inte` writer - PWM PTC interrupt enable"]
pub type INTE_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O>;
#[doc = "Field `int` reader - PWM PTC interrupt"]
pub type INT_R = crate::BitReader;
#[doc = "Field `int` writer - PWM PTC interrupt"]
pub type INT_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O>;
#[doc = "Field `cntrrst` reader - PWM PTC counter reset"]
pub type CNTRRST_R = crate::BitReader;
#[doc = "Field `cntrrst` writer - PWM PTC counter reset"]
pub type CNTRRST_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O>;
#[doc = "Field `capte` reader - PWM PTC capte"]
pub type CAPTE_R = crate::BitReader;
#[doc = "Field `capte` writer - PWM PTC capte"]
pub type CAPTE_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O>;
impl R {
    #[doc = "Bit 0 - PWM PTC enable"]
    #[inline(always)]
    pub fn en(&self) -> EN_R {
        EN_R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - PWM PTC enable clock"]
    #[inline(always)]
    pub fn eclk(&self) -> ECLK_R {
        ECLK_R::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - PWM PTC nec"]
    #[inline(always)]
    pub fn nec(&self) -> NEC_R {
        NEC_R::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3 - PWM PTC oe"]
    #[inline(always)]
    pub fn oe(&self) -> OE_R {
        OE_R::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bit 4 - PWM PTC single"]
    #[inline(always)]
    pub fn single(&self) -> SINGLE_R {
        SINGLE_R::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bit 5 - PWM PTC interrupt enable"]
    #[inline(always)]
    pub fn inte(&self) -> INTE_R {
        INTE_R::new(((self.bits >> 5) & 1) != 0)
    }
    #[doc = "Bit 6 - PWM PTC interrupt"]
    #[inline(always)]
    pub fn int(&self) -> INT_R {
        INT_R::new(((self.bits >> 6) & 1) != 0)
    }
    #[doc = "Bit 7 - PWM PTC counter reset"]
    #[inline(always)]
    pub fn cntrrst(&self) -> CNTRRST_R {
        CNTRRST_R::new(((self.bits >> 7) & 1) != 0)
    }
    #[doc = "Bit 8 - PWM PTC capte"]
    #[inline(always)]
    pub fn capte(&self) -> CAPTE_R {
        CAPTE_R::new(((self.bits >> 8) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - PWM PTC enable"]
    #[inline(always)]
    #[must_use]
    pub fn en(&mut self) -> EN_W<CTRL_SPEC, 0> {
        EN_W::new(self)
    }
    #[doc = "Bit 1 - PWM PTC enable clock"]
    #[inline(always)]
    #[must_use]
    pub fn eclk(&mut self) -> ECLK_W<CTRL_SPEC, 1> {
        ECLK_W::new(self)
    }
    #[doc = "Bit 2 - PWM PTC nec"]
    #[inline(always)]
    #[must_use]
    pub fn nec(&mut self) -> NEC_W<CTRL_SPEC, 2> {
        NEC_W::new(self)
    }
    #[doc = "Bit 3 - PWM PTC oe"]
    #[inline(always)]
    #[must_use]
    pub fn oe(&mut self) -> OE_W<CTRL_SPEC, 3> {
        OE_W::new(self)
    }
    #[doc = "Bit 4 - PWM PTC single"]
    #[inline(always)]
    #[must_use]
    pub fn single(&mut self) -> SINGLE_W<CTRL_SPEC, 4> {
        SINGLE_W::new(self)
    }
    #[doc = "Bit 5 - PWM PTC interrupt enable"]
    #[inline(always)]
    #[must_use]
    pub fn inte(&mut self) -> INTE_W<CTRL_SPEC, 5> {
        INTE_W::new(self)
    }
    #[doc = "Bit 6 - PWM PTC interrupt"]
    #[inline(always)]
    #[must_use]
    pub fn int(&mut self) -> INT_W<CTRL_SPEC, 6> {
        INT_W::new(self)
    }
    #[doc = "Bit 7 - PWM PTC counter reset"]
    #[inline(always)]
    #[must_use]
    pub fn cntrrst(&mut self) -> CNTRRST_W<CTRL_SPEC, 7> {
        CNTRRST_W::new(self)
    }
    #[doc = "Bit 8 - PWM PTC capte"]
    #[inline(always)]
    #[must_use]
    pub fn capte(&mut self) -> CAPTE_W<CTRL_SPEC, 8> {
        CAPTE_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.bits = bits;
        self
    }
}
#[doc = "PTC control register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`ctrl::R`](R).  You can [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`ctrl::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct CTRL_SPEC;
impl crate::RegisterSpec for CTRL_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`ctrl::R`](R) reader structure"]
impl crate::Readable for CTRL_SPEC {}
#[doc = "`write(|w| ..)` method takes [`ctrl::W`](W) writer structure"]
impl crate::Writable for CTRL_SPEC {
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
