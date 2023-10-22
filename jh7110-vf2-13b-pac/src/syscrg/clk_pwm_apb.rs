#[doc = "Register `clk_pwm_apb` reader"]
pub type R = crate::R<CLK_PWM_APB_SPEC>;
#[doc = "Register `clk_pwm_apb` writer"]
pub type W = crate::W<CLK_PWM_APB_SPEC>;
#[doc = "Field `clk_icg` reader - 1: Clock enable, 0: Clock disable"]
pub type CLK_ICG_R = crate::BitReader;
#[doc = "Field `clk_icg` writer - 1: Clock enable, 0: Clock disable"]
pub type CLK_ICG_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O>;
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
    pub fn clk_icg(&mut self) -> CLK_ICG_W<CLK_PWM_APB_SPEC, 31> {
        CLK_ICG_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.bits = bits;
        self
    }
}
#[doc = "Clock PWM APB\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`clk_pwm_apb::R`](R).  You can [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`clk_pwm_apb::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct CLK_PWM_APB_SPEC;
impl crate::RegisterSpec for CLK_PWM_APB_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`clk_pwm_apb::R`](R) reader structure"]
impl crate::Readable for CLK_PWM_APB_SPEC {}
#[doc = "`write(|w| ..)` method takes [`clk_pwm_apb::W`](W) writer structure"]
impl crate::Writable for CLK_PWM_APB_SPEC {
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
