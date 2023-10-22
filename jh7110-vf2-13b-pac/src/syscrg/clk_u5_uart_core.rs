#[doc = "Register `clk_u5_uart_core` reader"]
pub type R = crate::R<CLK_U5_UART_CORE_SPEC>;
#[doc = "Register `clk_u5_uart_core` writer"]
pub type W = crate::W<CLK_U5_UART_CORE_SPEC>;
#[doc = "Field `clk_divcfg` reader - Clock divider coefficient: Max=131071, Default=2560, Min=2560, Typical=2560"]
pub type CLK_DIVCFG_R = crate::FieldReader<u32>;
#[doc = "Field `clk_divcfg` writer - Clock divider coefficient: Max=131071, Default=2560, Min=2560, Typical=2560"]
pub type CLK_DIVCFG_W<'a, REG, const O: u8> = crate::FieldWriter<'a, REG, 24, O, u32>;
#[doc = "Field `clk_icg` reader - 1: Clock enable, 0: Clock disable"]
pub type CLK_ICG_R = crate::BitReader;
#[doc = "Field `clk_icg` writer - 1: Clock enable, 0: Clock disable"]
pub type CLK_ICG_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O>;
impl R {
    #[doc = "Bits 0:23 - Clock divider coefficient: Max=131071, Default=2560, Min=2560, Typical=2560"]
    #[inline(always)]
    pub fn clk_divcfg(&self) -> CLK_DIVCFG_R {
        CLK_DIVCFG_R::new(self.bits & 0x00ff_ffff)
    }
    #[doc = "Bit 31 - 1: Clock enable, 0: Clock disable"]
    #[inline(always)]
    pub fn clk_icg(&self) -> CLK_ICG_R {
        CLK_ICG_R::new(((self.bits >> 31) & 1) != 0)
    }
}
impl W {
    #[doc = "Bits 0:23 - Clock divider coefficient: Max=131071, Default=2560, Min=2560, Typical=2560"]
    #[inline(always)]
    #[must_use]
    pub fn clk_divcfg(&mut self) -> CLK_DIVCFG_W<CLK_U5_UART_CORE_SPEC, 0> {
        CLK_DIVCFG_W::new(self)
    }
    #[doc = "Bit 31 - 1: Clock enable, 0: Clock disable"]
    #[inline(always)]
    #[must_use]
    pub fn clk_icg(&mut self) -> CLK_ICG_W<CLK_U5_UART_CORE_SPEC, 31> {
        CLK_ICG_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.bits = bits;
        self
    }
}
#[doc = "U5 Clock UART Core\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`clk_u5_uart_core::R`](R).  You can [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`clk_u5_uart_core::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct CLK_U5_UART_CORE_SPEC;
impl crate::RegisterSpec for CLK_U5_UART_CORE_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`clk_u5_uart_core::R`](R) reader structure"]
impl crate::Readable for CLK_U5_UART_CORE_SPEC {}
#[doc = "`write(|w| ..)` method takes [`clk_u5_uart_core::W`](W) writer structure"]
impl crate::Writable for CLK_U5_UART_CORE_SPEC {
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}