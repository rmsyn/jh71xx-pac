#[doc = "Register `stg_sysconsaif_syscfg196` reader"]
pub type R = crate::R<STG_SYSCONSAIF_SYSCFG196_SPEC>;
#[doc = "Register `stg_sysconsaif_syscfg196` writer"]
pub type W = crate::W<STG_SYSCONSAIF_SYSCFG196_SPEC>;
#[doc = "Field `u0_plda_pcie_axi4_slvl_awfunc` reader - u0_plda_pcie_axi4_slvl_awfunc"]
pub type U0_PLDA_PCIE_AXI4_SLVL_AWFUNC_R = crate::FieldReader<u16>;
#[doc = "Field `u0_plda_pcie_axi4_slvl_awfunc` writer - u0_plda_pcie_axi4_slvl_awfunc"]
pub type U0_PLDA_PCIE_AXI4_SLVL_AWFUNC_W<'a, REG, const O: u8> =
    crate::FieldWriter<'a, REG, 15, O, u16>;
#[doc = "Field `u0_plda_pcie_bus_width_o` reader - u0_plda_pcie_bus_width_o"]
pub type U0_PLDA_PCIE_BUS_WIDTH_O_R = crate::FieldReader;
#[doc = "Field `u0_plda_pcie_bypass_codec` reader - u0_plda_pcie_bypass_codec"]
pub type U0_PLDA_PCIE_BYPASS_CODEC_R = crate::BitReader;
#[doc = "Field `u0_plda_pcie_bypass_codec` writer - u0_plda_pcie_bypass_codec"]
pub type U0_PLDA_PCIE_BYPASS_CODEC_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O>;
#[doc = "Field `u0_plda_pcie_ckref_src` reader - u0_plda_pcie_ckref_src"]
pub type U0_PLDA_PCIE_CKREF_SRC_R = crate::FieldReader;
#[doc = "Field `u0_plda_pcie_ckref_src` writer - u0_plda_pcie_ckref_src"]
pub type U0_PLDA_PCIE_CKREF_SRC_W<'a, REG, const O: u8> = crate::FieldWriter<'a, REG, 2, O>;
#[doc = "Field `u0_plda_pcie_clk_sel` reader - u0_plda_pcie_clk_sel"]
pub type U0_PLDA_PCIE_CLK_SEL_R = crate::FieldReader;
#[doc = "Field `u0_plda_pcie_clk_sel` writer - u0_plda_pcie_clk_sel"]
pub type U0_PLDA_PCIE_CLK_SEL_W<'a, REG, const O: u8> = crate::FieldWriter<'a, REG, 2, O>;
#[doc = "Field `u0_plda_pcie_clkreq` reader - u0_plda_pcie_clkreq"]
pub type U0_PLDA_PCIE_CLKREQ_R = crate::BitReader;
#[doc = "Field `u0_plda_pcie_clkreq` writer - u0_plda_pcie_clkreq"]
pub type U0_PLDA_PCIE_CLKREQ_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O>;
impl R {
    #[doc = "Bits 0:14 - u0_plda_pcie_axi4_slvl_awfunc"]
    #[inline(always)]
    pub fn u0_plda_pcie_axi4_slvl_awfunc(&self) -> U0_PLDA_PCIE_AXI4_SLVL_AWFUNC_R {
        U0_PLDA_PCIE_AXI4_SLVL_AWFUNC_R::new((self.bits & 0x7fff) as u16)
    }
    #[doc = "Bits 15:16 - u0_plda_pcie_bus_width_o"]
    #[inline(always)]
    pub fn u0_plda_pcie_bus_width_o(&self) -> U0_PLDA_PCIE_BUS_WIDTH_O_R {
        U0_PLDA_PCIE_BUS_WIDTH_O_R::new(((self.bits >> 15) & 3) as u8)
    }
    #[doc = "Bit 17 - u0_plda_pcie_bypass_codec"]
    #[inline(always)]
    pub fn u0_plda_pcie_bypass_codec(&self) -> U0_PLDA_PCIE_BYPASS_CODEC_R {
        U0_PLDA_PCIE_BYPASS_CODEC_R::new(((self.bits >> 17) & 1) != 0)
    }
    #[doc = "Bits 18:19 - u0_plda_pcie_ckref_src"]
    #[inline(always)]
    pub fn u0_plda_pcie_ckref_src(&self) -> U0_PLDA_PCIE_CKREF_SRC_R {
        U0_PLDA_PCIE_CKREF_SRC_R::new(((self.bits >> 18) & 3) as u8)
    }
    #[doc = "Bits 20:21 - u0_plda_pcie_clk_sel"]
    #[inline(always)]
    pub fn u0_plda_pcie_clk_sel(&self) -> U0_PLDA_PCIE_CLK_SEL_R {
        U0_PLDA_PCIE_CLK_SEL_R::new(((self.bits >> 20) & 3) as u8)
    }
    #[doc = "Bit 22 - u0_plda_pcie_clkreq"]
    #[inline(always)]
    pub fn u0_plda_pcie_clkreq(&self) -> U0_PLDA_PCIE_CLKREQ_R {
        U0_PLDA_PCIE_CLKREQ_R::new(((self.bits >> 22) & 1) != 0)
    }
}
impl W {
    #[doc = "Bits 0:14 - u0_plda_pcie_axi4_slvl_awfunc"]
    #[inline(always)]
    #[must_use]
    pub fn u0_plda_pcie_axi4_slvl_awfunc(
        &mut self,
    ) -> U0_PLDA_PCIE_AXI4_SLVL_AWFUNC_W<STG_SYSCONSAIF_SYSCFG196_SPEC, 0> {
        U0_PLDA_PCIE_AXI4_SLVL_AWFUNC_W::new(self)
    }
    #[doc = "Bit 17 - u0_plda_pcie_bypass_codec"]
    #[inline(always)]
    #[must_use]
    pub fn u0_plda_pcie_bypass_codec(
        &mut self,
    ) -> U0_PLDA_PCIE_BYPASS_CODEC_W<STG_SYSCONSAIF_SYSCFG196_SPEC, 17> {
        U0_PLDA_PCIE_BYPASS_CODEC_W::new(self)
    }
    #[doc = "Bits 18:19 - u0_plda_pcie_ckref_src"]
    #[inline(always)]
    #[must_use]
    pub fn u0_plda_pcie_ckref_src(
        &mut self,
    ) -> U0_PLDA_PCIE_CKREF_SRC_W<STG_SYSCONSAIF_SYSCFG196_SPEC, 18> {
        U0_PLDA_PCIE_CKREF_SRC_W::new(self)
    }
    #[doc = "Bits 20:21 - u0_plda_pcie_clk_sel"]
    #[inline(always)]
    #[must_use]
    pub fn u0_plda_pcie_clk_sel(
        &mut self,
    ) -> U0_PLDA_PCIE_CLK_SEL_W<STG_SYSCONSAIF_SYSCFG196_SPEC, 20> {
        U0_PLDA_PCIE_CLK_SEL_W::new(self)
    }
    #[doc = "Bit 22 - u0_plda_pcie_clkreq"]
    #[inline(always)]
    #[must_use]
    pub fn u0_plda_pcie_clkreq(
        &mut self,
    ) -> U0_PLDA_PCIE_CLKREQ_W<STG_SYSCONSAIF_SYSCFG196_SPEC, 22> {
        U0_PLDA_PCIE_CLKREQ_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.bits = bits;
        self
    }
}
#[doc = "STG SYSCONSAIF SYSCFG 196\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`stg_sysconsaif_syscfg196::R`](R).  You can [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`stg_sysconsaif_syscfg196::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct STG_SYSCONSAIF_SYSCFG196_SPEC;
impl crate::RegisterSpec for STG_SYSCONSAIF_SYSCFG196_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`stg_sysconsaif_syscfg196::R`](R) reader structure"]
impl crate::Readable for STG_SYSCONSAIF_SYSCFG196_SPEC {}
#[doc = "`write(|w| ..)` method takes [`stg_sysconsaif_syscfg196::W`](W) writer structure"]
impl crate::Writable for STG_SYSCONSAIF_SYSCFG196_SPEC {
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
