#[doc = "Register `clk_stg_mtrx_group0_stg` reader"]
pub type R = crate::R<CLK_STG_MTRX_GROUP0_STG_SPEC>;
#[doc = "Register `clk_stg_mtrx_group0_stg` writer"]
pub type W = crate::W<CLK_STG_MTRX_GROUP0_STG_SPEC>;
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
    pub fn clk_icg(&mut self) -> CLK_ICG_W<CLK_STG_MTRX_GROUP0_STG_SPEC> {
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
#[doc = "Clock STG MTRX Group 0 STG\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`clk_stg_mtrx_group0_stg::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`clk_stg_mtrx_group0_stg::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct CLK_STG_MTRX_GROUP0_STG_SPEC;
impl crate::RegisterSpec for CLK_STG_MTRX_GROUP0_STG_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`clk_stg_mtrx_group0_stg::R`](R) reader structure"]
impl crate::Readable for CLK_STG_MTRX_GROUP0_STG_SPEC {}
#[doc = "`write(|w| ..)` method takes [`clk_stg_mtrx_group0_stg::W`](W) writer structure"]
impl crate::Writable for CLK_STG_MTRX_GROUP0_STG_SPEC {
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets clk_stg_mtrx_group0_stg to value 0"]
impl crate::Resettable for CLK_STG_MTRX_GROUP0_STG_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
