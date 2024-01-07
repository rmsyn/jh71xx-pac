#[doc = "Register `clk_u7mc_core_4` reader"]
pub type R = crate::R<CLK_U7MC_CORE_4_SPEC>;
#[doc = "Register `clk_u7mc_core_4` writer"]
pub type W = crate::W<CLK_U7MC_CORE_4_SPEC>;
#[doc = "Field `clk_icg` reader - 1: Clock enable, 0: Clock disable"]
pub type CLK_ICG_R = crate::BitReader;
#[doc = "Field `clk_icg` writer - 1: Clock enable, 0: Clock disable"]
pub type CLK_ICG_W<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bit 31 - 1: Clock enable, 0: Clock disable"]
    #[inline(always)]
    pub fn clk_icg(&self) -> CLK_ICG_R {
        CLK_ICG_R::new(((self.bits >> 31) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 31 - 1: Clock enable, 0: Clock disable"]
    #[inline(always)]
    #[must_use]
    pub fn clk_icg(&mut self) -> CLK_ICG_W<CLK_U7MC_CORE_4_SPEC> {
        CLK_ICG_W::new(self, 31)
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
#[doc = "U7MC Core Clock 4\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`clk_u7mc_core_4::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`clk_u7mc_core_4::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct CLK_U7MC_CORE_4_SPEC;
impl crate::RegisterSpec for CLK_U7MC_CORE_4_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`clk_u7mc_core_4::R`](R) reader structure"]
impl crate::Readable for CLK_U7MC_CORE_4_SPEC {}
#[doc = "`write(|w| ..)` method takes [`clk_u7mc_core_4::W`](W) writer structure"]
impl crate::Writable for CLK_U7MC_CORE_4_SPEC {
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets clk_u7mc_core_4 to value 0"]
impl crate::Resettable for CLK_U7MC_CORE_4_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
