#[doc = "Register `stg_syscfg_157` reader"]
pub type R = crate::R<STG_SYSCFG_157_SPEC>;
#[doc = "Register `stg_syscfg_157` writer"]
pub type W = crate::W<STG_SYSCFG_157_SPEC>;
#[doc = "Field `u1_pcie_axi4_slvl_awfunc` reader - u1_pcie_axi4_slvl_awfunc"]
pub type U1_PCIE_AXI4_SLVL_AWFUNC_R = crate::FieldReader<u16>;
#[doc = "Field `u1_pcie_axi4_slvl_awfunc` writer - u1_pcie_axi4_slvl_awfunc"]
pub type U1_PCIE_AXI4_SLVL_AWFUNC_W<'a, REG> = crate::FieldWriter<'a, REG, 15, u16>;
#[doc = "Field `u1_pcie_bus_width_o` reader - u1_pcie_bus_width_o"]
pub type U1_PCIE_BUS_WIDTH_O_R = crate::FieldReader;
#[doc = "Field `u1_pcie_bypass_codec` reader - u1_pcie_bypass_codec"]
pub type U1_PCIE_BYPASS_CODEC_R = crate::BitReader;
#[doc = "Field `u1_pcie_bypass_codec` writer - u1_pcie_bypass_codec"]
pub type U1_PCIE_BYPASS_CODEC_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `u1_pcie_ckref_src` reader - u1_pcie_ckref_src"]
pub type U1_PCIE_CKREF_SRC_R = crate::FieldReader;
#[doc = "Field `u1_pcie_ckref_src` writer - u1_pcie_ckref_src"]
pub type U1_PCIE_CKREF_SRC_W<'a, REG> = crate::FieldWriter<'a, REG, 2>;
#[doc = "Field `u1_pcie_clk_sel` reader - u1_pcie_clk_sel"]
pub type U1_PCIE_CLK_SEL_R = crate::FieldReader;
#[doc = "Field `u1_pcie_clk_sel` writer - u1_pcie_clk_sel"]
pub type U1_PCIE_CLK_SEL_W<'a, REG> = crate::FieldWriter<'a, REG, 2>;
#[doc = "Field `u1_pcie_clkreq` reader - u1_pcie_clkreq"]
pub type U1_PCIE_CLKREQ_R = crate::BitReader;
#[doc = "Field `u1_pcie_clkreq` writer - u1_pcie_clkreq"]
pub type U1_PCIE_CLKREQ_W<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bits 0:14 - u1_pcie_axi4_slvl_awfunc"]
    #[inline(always)]
    pub fn u1_pcie_axi4_slvl_awfunc(&self) -> U1_PCIE_AXI4_SLVL_AWFUNC_R {
        U1_PCIE_AXI4_SLVL_AWFUNC_R::new((self.bits & 0x7fff) as u16)
    }
    #[doc = "Bits 15:16 - u1_pcie_bus_width_o"]
    #[inline(always)]
    pub fn u1_pcie_bus_width_o(&self) -> U1_PCIE_BUS_WIDTH_O_R {
        U1_PCIE_BUS_WIDTH_O_R::new(((self.bits >> 15) & 3) as u8)
    }
    #[doc = "Bit 17 - u1_pcie_bypass_codec"]
    #[inline(always)]
    pub fn u1_pcie_bypass_codec(&self) -> U1_PCIE_BYPASS_CODEC_R {
        U1_PCIE_BYPASS_CODEC_R::new(((self.bits >> 17) & 1) != 0)
    }
    #[doc = "Bits 18:19 - u1_pcie_ckref_src"]
    #[inline(always)]
    pub fn u1_pcie_ckref_src(&self) -> U1_PCIE_CKREF_SRC_R {
        U1_PCIE_CKREF_SRC_R::new(((self.bits >> 18) & 3) as u8)
    }
    #[doc = "Bits 20:21 - u1_pcie_clk_sel"]
    #[inline(always)]
    pub fn u1_pcie_clk_sel(&self) -> U1_PCIE_CLK_SEL_R {
        U1_PCIE_CLK_SEL_R::new(((self.bits >> 20) & 3) as u8)
    }
    #[doc = "Bit 22 - u1_pcie_clkreq"]
    #[inline(always)]
    pub fn u1_pcie_clkreq(&self) -> U1_PCIE_CLKREQ_R {
        U1_PCIE_CLKREQ_R::new(((self.bits >> 22) & 1) != 0)
    }
}
impl W {
    #[doc = "Bits 0:14 - u1_pcie_axi4_slvl_awfunc"]
    #[inline(always)]
    #[must_use]
    pub fn u1_pcie_axi4_slvl_awfunc(&mut self) -> U1_PCIE_AXI4_SLVL_AWFUNC_W<STG_SYSCFG_157_SPEC> {
        U1_PCIE_AXI4_SLVL_AWFUNC_W::new(self, 0)
    }
    #[doc = "Bit 17 - u1_pcie_bypass_codec"]
    #[inline(always)]
    #[must_use]
    pub fn u1_pcie_bypass_codec(&mut self) -> U1_PCIE_BYPASS_CODEC_W<STG_SYSCFG_157_SPEC> {
        U1_PCIE_BYPASS_CODEC_W::new(self, 17)
    }
    #[doc = "Bits 18:19 - u1_pcie_ckref_src"]
    #[inline(always)]
    #[must_use]
    pub fn u1_pcie_ckref_src(&mut self) -> U1_PCIE_CKREF_SRC_W<STG_SYSCFG_157_SPEC> {
        U1_PCIE_CKREF_SRC_W::new(self, 18)
    }
    #[doc = "Bits 20:21 - u1_pcie_clk_sel"]
    #[inline(always)]
    #[must_use]
    pub fn u1_pcie_clk_sel(&mut self) -> U1_PCIE_CLK_SEL_W<STG_SYSCFG_157_SPEC> {
        U1_PCIE_CLK_SEL_W::new(self, 20)
    }
    #[doc = "Bit 22 - u1_pcie_clkreq"]
    #[inline(always)]
    #[must_use]
    pub fn u1_pcie_clkreq(&mut self) -> U1_PCIE_CLKREQ_W<STG_SYSCFG_157_SPEC> {
        U1_PCIE_CLKREQ_W::new(self, 22)
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
#[doc = "STG SYSCONSAIF SYSCFG 628\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`stg_syscfg_157::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`stg_syscfg_157::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct STG_SYSCFG_157_SPEC;
impl crate::RegisterSpec for STG_SYSCFG_157_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`stg_syscfg_157::R`](R) reader structure"]
impl crate::Readable for STG_SYSCFG_157_SPEC {}
#[doc = "`write(|w| ..)` method takes [`stg_syscfg_157::W`](W) writer structure"]
impl crate::Writable for STG_SYSCFG_157_SPEC {
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets stg_syscfg_157 to value 0"]
impl crate::Resettable for STG_SYSCFG_157_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
