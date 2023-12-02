#[doc = "Register `clk_gmac5_axi64_tx` reader"]
pub type R = crate::R<CLK_GMAC5_AXI64_TX_SPEC>;
#[doc = "Register `clk_gmac5_axi64_tx` writer"]
pub type W = crate::W<CLK_GMAC5_AXI64_TX_SPEC>;
#[doc = "Field `clk_mux_sel` reader - Clock multiplexing selector: clk_gmac1_gtxclk, clk_gmac1_rmii_rtx"]
pub type CLK_MUX_SEL_R = crate::FieldReader;
#[doc = "Field `clk_mux_sel` writer - Clock multiplexing selector: clk_gmac1_gtxclk, clk_gmac1_rmii_rtx"]
pub type CLK_MUX_SEL_W<'a, REG> = crate::FieldWriter<'a, REG, 6>;
#[doc = "Field `clk_icg` reader - 1: Clock enable, 0: Clock disable"]
pub type CLK_ICG_R = crate::BitReader;
#[doc = "Field `clk_icg` writer - 1: Clock enable, 0: Clock disable"]
pub type CLK_ICG_W<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bits 24:29 - Clock multiplexing selector: clk_gmac1_gtxclk, clk_gmac1_rmii_rtx"]
    #[inline(always)]
    pub fn clk_mux_sel(&self) -> CLK_MUX_SEL_R {
        CLK_MUX_SEL_R::new(((self.bits >> 24) & 0x3f) as u8)
    }
    #[doc = "Bit 31 - 1: Clock enable, 0: Clock disable"]
    #[inline(always)]
    pub fn clk_icg(&self) -> CLK_ICG_R {
        CLK_ICG_R::new(((self.bits >> 31) & 1) != 0)
    }
}
impl W {
    #[doc = "Bits 24:29 - Clock multiplexing selector: clk_gmac1_gtxclk, clk_gmac1_rmii_rtx"]
    #[inline(always)]
    #[must_use]
    pub fn clk_mux_sel(&mut self) -> CLK_MUX_SEL_W<CLK_GMAC5_AXI64_TX_SPEC> {
        CLK_MUX_SEL_W::new(self, 24)
    }
    #[doc = "Bit 31 - 1: Clock enable, 0: Clock disable"]
    #[inline(always)]
    #[must_use]
    pub fn clk_icg(&mut self) -> CLK_ICG_W<CLK_GMAC5_AXI64_TX_SPEC> {
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
#[doc = "Clock GMAC 5 AXI 64 TX\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`clk_gmac5_axi64_tx::R`](R).  You can [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`clk_gmac5_axi64_tx::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct CLK_GMAC5_AXI64_TX_SPEC;
impl crate::RegisterSpec for CLK_GMAC5_AXI64_TX_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`clk_gmac5_axi64_tx::R`](R) reader structure"]
impl crate::Readable for CLK_GMAC5_AXI64_TX_SPEC {}
#[doc = "`write(|w| ..)` method takes [`clk_gmac5_axi64_tx::W`](W) writer structure"]
impl crate::Writable for CLK_GMAC5_AXI64_TX_SPEC {
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
