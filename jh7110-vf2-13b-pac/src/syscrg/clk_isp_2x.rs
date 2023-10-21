#[doc = "Register `clk_isp_2x` reader"]
pub type R = crate::R<CLK_ISP_2X_SPEC>;
#[doc = "Register `clk_isp_2x` writer"]
pub type W = crate::W<CLK_ISP_2X_SPEC>;
#[doc = "Field `clk_divcfg` reader - Clock divider coefficient: Max=8, Default=2, Min=2, Typical=2"]
pub type CLK_DIVCFG_R = crate::FieldReader<u32>;
#[doc = "Field `clk_divcfg` writer - Clock divider coefficient: Max=8, Default=2, Min=2, Typical=2"]
pub type CLK_DIVCFG_W<'a, REG, const O: u8> = crate::FieldWriter<'a, REG, 24, O, u32>;
#[doc = "Field `clk_mux_sel` reader - Clock multiplexing selector: clk_pll2, clk_pll1"]
pub type CLK_MUX_SEL_R = crate::FieldReader;
#[doc = "Field `clk_mux_sel` writer - Clock multiplexing selector: clk_pll2, clk_pll1"]
pub type CLK_MUX_SEL_W<'a, REG, const O: u8> = crate::FieldWriter<'a, REG, 6, O>;
impl R {
    #[doc = "Bits 0:23 - Clock divider coefficient: Max=8, Default=2, Min=2, Typical=2"]
    #[inline(always)]
    pub fn clk_divcfg(&self) -> CLK_DIVCFG_R {
        CLK_DIVCFG_R::new(self.bits & 0x00ff_ffff)
    }
    #[doc = "Bits 24:29 - Clock multiplexing selector: clk_pll2, clk_pll1"]
    #[inline(always)]
    pub fn clk_mux_sel(&self) -> CLK_MUX_SEL_R {
        CLK_MUX_SEL_R::new(((self.bits >> 24) & 0x3f) as u8)
    }
}
impl W {
    #[doc = "Bits 0:23 - Clock divider coefficient: Max=8, Default=2, Min=2, Typical=2"]
    #[inline(always)]
    #[must_use]
    pub fn clk_divcfg(&mut self) -> CLK_DIVCFG_W<CLK_ISP_2X_SPEC, 0> {
        CLK_DIVCFG_W::new(self)
    }
    #[doc = "Bits 24:29 - Clock multiplexing selector: clk_pll2, clk_pll1"]
    #[inline(always)]
    #[must_use]
    pub fn clk_mux_sel(&mut self) -> CLK_MUX_SEL_W<CLK_ISP_2X_SPEC, 24> {
        CLK_MUX_SEL_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.bits = bits;
        self
    }
}
#[doc = "Clock ISP 2x\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`clk_isp_2x::R`](R).  You can [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`clk_isp_2x::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct CLK_ISP_2X_SPEC;
impl crate::RegisterSpec for CLK_ISP_2X_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`clk_isp_2x::R`](R) reader structure"]
impl crate::Readable for CLK_ISP_2X_SPEC {}
#[doc = "`write(|w| ..)` method takes [`clk_isp_2x::W`](W) writer structure"]
impl crate::Writable for CLK_ISP_2X_SPEC {
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
