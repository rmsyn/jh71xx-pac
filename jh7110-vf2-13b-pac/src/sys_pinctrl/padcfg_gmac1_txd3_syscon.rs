#[doc = "Register `padcfg_gmac1_txd3_syscon` reader"]
pub type R = crate::R<PADCFG_GMAC1_TXD3_SYSCON_SPEC>;
#[doc = "Register `padcfg_gmac1_txd3_syscon` writer"]
pub type W = crate::W<PADCFG_GMAC1_TXD3_SYSCON_SPEC>;
#[doc = "Field `padcfg_pad_gmac1_txd3_syscon` reader - padcfg_pad_gmac1_txd3_syscon"]
pub type PADCFG_PAD_GMAC1_TXD3_SYSCON_R = crate::FieldReader;
#[doc = "Field `padcfg_pad_gmac1_txd3_syscon` writer - padcfg_pad_gmac1_txd3_syscon"]
pub type PADCFG_PAD_GMAC1_TXD3_SYSCON_W<'a, REG, const O: u8> = crate::FieldWriter<'a, REG, 2, O>;
impl R {
    #[doc = "Bits 0:1 - padcfg_pad_gmac1_txd3_syscon"]
    #[inline(always)]
    pub fn padcfg_pad_gmac1_txd3_syscon(&self) -> PADCFG_PAD_GMAC1_TXD3_SYSCON_R {
        PADCFG_PAD_GMAC1_TXD3_SYSCON_R::new((self.bits & 3) as u8)
    }
}
impl W {
    #[doc = "Bits 0:1 - padcfg_pad_gmac1_txd3_syscon"]
    #[inline(always)]
    #[must_use]
    pub fn padcfg_pad_gmac1_txd3_syscon(
        &mut self,
    ) -> PADCFG_PAD_GMAC1_TXD3_SYSCON_W<PADCFG_GMAC1_TXD3_SYSCON_SPEC, 0> {
        PADCFG_PAD_GMAC1_TXD3_SYSCON_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.bits = bits;
        self
    }
}
#[doc = "GPIO GMAC1 TXD3 Pad Configuration\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`padcfg_gmac1_txd3_syscon::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`padcfg_gmac1_txd3_syscon::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct PADCFG_GMAC1_TXD3_SYSCON_SPEC;
impl crate::RegisterSpec for PADCFG_GMAC1_TXD3_SYSCON_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`padcfg_gmac1_txd3_syscon::R`](R) reader structure"]
impl crate::Readable for PADCFG_GMAC1_TXD3_SYSCON_SPEC {}
#[doc = "`write(|w| ..)` method takes [`padcfg_gmac1_txd3_syscon::W`](W) writer structure"]
impl crate::Writable for PADCFG_GMAC1_TXD3_SYSCON_SPEC {
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets padcfg_gmac1_txd3_syscon to value 0x02"]
impl crate::Resettable for PADCFG_GMAC1_TXD3_SYSCON_SPEC {
    const RESET_VALUE: Self::Ux = 0x02;
}