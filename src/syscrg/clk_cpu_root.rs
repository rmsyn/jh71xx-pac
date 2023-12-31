#[doc = "Register `clk_cpu_root` reader"]
pub type R = crate::R<CLK_CPU_ROOT_SPEC>;
#[doc = "Register `clk_cpu_root` writer"]
pub type W = crate::W<CLK_CPU_ROOT_SPEC>;
#[doc = "Field `clk_mux_sel` reader - Clock multiplexing selector: clk_osc, clk_pll0"]
pub type CLK_MUX_SEL_R = crate::FieldReader;
#[doc = "Field `clk_mux_sel` writer - Clock multiplexing selector: clk_osc, clk_pll0"]
pub type CLK_MUX_SEL_W<'a, REG> = crate::FieldWriter<'a, REG, 6>;
impl R {
    #[doc = "Bits 24:29 - Clock multiplexing selector: clk_osc, clk_pll0"]
    #[inline(always)]
    pub fn clk_mux_sel(&self) -> CLK_MUX_SEL_R {
        CLK_MUX_SEL_R::new(((self.bits >> 24) & 0x3f) as u8)
    }
}
impl W {
    #[doc = "Bits 24:29 - Clock multiplexing selector: clk_osc, clk_pll0"]
    #[inline(always)]
    #[must_use]
    pub fn clk_mux_sel(&mut self) -> CLK_MUX_SEL_W<CLK_CPU_ROOT_SPEC> {
        CLK_MUX_SEL_W::new(self, 24)
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
#[doc = "Clock CPU Root\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`clk_cpu_root::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`clk_cpu_root::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct CLK_CPU_ROOT_SPEC;
impl crate::RegisterSpec for CLK_CPU_ROOT_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`clk_cpu_root::R`](R) reader structure"]
impl crate::Readable for CLK_CPU_ROOT_SPEC {}
#[doc = "`write(|w| ..)` method takes [`clk_cpu_root::W`](W) writer structure"]
impl crate::Writable for CLK_CPU_ROOT_SPEC {
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets clk_cpu_root to value 0"]
impl crate::Resettable for CLK_CPU_ROOT_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
