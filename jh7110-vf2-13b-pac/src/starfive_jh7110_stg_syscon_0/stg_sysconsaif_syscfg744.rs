#[doc = "Register `stg_sysconsaif_syscfg744` reader"]
pub type R = crate::R<STG_SYSCONSAIF_SYSCFG744_SPEC>;
#[doc = "Register `stg_sysconsaif_syscfg744` writer"]
pub type W = crate::W<STG_SYSCONSAIF_SYSCFG744_SPEC>;
#[doc = "Field `u1_plda_pcie_mperstn` reader - u1_plda_pcie_mperstn"]
pub type U1_PLDA_PCIE_MPERSTN_R = crate::BitReader;
#[doc = "Field `u1_plda_pcie_mperstn` writer - u1_plda_pcie_mperstn"]
pub type U1_PLDA_PCIE_MPERSTN_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O>;
#[doc = "Field `u1_plda_pcie_pcie_ebuf_mode` reader - u1_plda_pcie_pcie_ebuf_mode"]
pub type U1_PLDA_PCIE_PCIE_EBUF_MODE_R = crate::BitReader;
#[doc = "Field `u1_plda_pcie_pcie_ebuf_mode` writer - u1_plda_pcie_pcie_ebuf_mode"]
pub type U1_PLDA_PCIE_PCIE_EBUF_MODE_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O>;
#[doc = "Field `u1_plda_pcie_pcie_phy_test_cfg` reader - u1_plda_pcie_pcie_phy_test_cfg"]
pub type U1_PLDA_PCIE_PCIE_PHY_TEST_CFG_R = crate::FieldReader<u32>;
#[doc = "Field `u1_plda_pcie_pcie_phy_test_cfg` writer - u1_plda_pcie_pcie_phy_test_cfg"]
pub type U1_PLDA_PCIE_PCIE_PHY_TEST_CFG_W<'a, REG, const O: u8> =
    crate::FieldWriter<'a, REG, 23, O, u32>;
#[doc = "Field `u1_plda_pcie_pcie_rx_eq_training` reader - u1_plda_pcie_pcie_rx_eq_training"]
pub type U1_PLDA_PCIE_PCIE_RX_EQ_TRAINING_R = crate::BitReader;
#[doc = "Field `u1_plda_pcie_pcie_rx_eq_training` writer - u1_plda_pcie_pcie_rx_eq_training"]
pub type U1_PLDA_PCIE_PCIE_RX_EQ_TRAINING_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O>;
#[doc = "Field `u1_plda_pcie_pcie_rxterm_en` reader - u1_plda_pcie_pcie_rxterm_en"]
pub type U1_PLDA_PCIE_PCIE_RXTERM_EN_R = crate::BitReader;
#[doc = "Field `u1_plda_pcie_pcie_rxterm_en` writer - u1_plda_pcie_pcie_rxterm_en"]
pub type U1_PLDA_PCIE_PCIE_RXTERM_EN_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O>;
#[doc = "Field `u1_plda_pcie_pcie_tx_oneszeros` reader - u1_plda_pcie_pcie_tx_oneszeros"]
pub type U1_PLDA_PCIE_PCIE_TX_ONESZEROS_R = crate::BitReader;
#[doc = "Field `u1_plda_pcie_pcie_tx_oneszeros` writer - u1_plda_pcie_pcie_tx_oneszeros"]
pub type U1_PLDA_PCIE_PCIE_TX_ONESZEROS_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O>;
impl R {
    #[doc = "Bit 0 - u1_plda_pcie_mperstn"]
    #[inline(always)]
    pub fn u1_plda_pcie_mperstn(&self) -> U1_PLDA_PCIE_MPERSTN_R {
        U1_PLDA_PCIE_MPERSTN_R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - u1_plda_pcie_pcie_ebuf_mode"]
    #[inline(always)]
    pub fn u1_plda_pcie_pcie_ebuf_mode(&self) -> U1_PLDA_PCIE_PCIE_EBUF_MODE_R {
        U1_PLDA_PCIE_PCIE_EBUF_MODE_R::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bits 2:24 - u1_plda_pcie_pcie_phy_test_cfg"]
    #[inline(always)]
    pub fn u1_plda_pcie_pcie_phy_test_cfg(&self) -> U1_PLDA_PCIE_PCIE_PHY_TEST_CFG_R {
        U1_PLDA_PCIE_PCIE_PHY_TEST_CFG_R::new((self.bits >> 2) & 0x007f_ffff)
    }
    #[doc = "Bit 25 - u1_plda_pcie_pcie_rx_eq_training"]
    #[inline(always)]
    pub fn u1_plda_pcie_pcie_rx_eq_training(&self) -> U1_PLDA_PCIE_PCIE_RX_EQ_TRAINING_R {
        U1_PLDA_PCIE_PCIE_RX_EQ_TRAINING_R::new(((self.bits >> 25) & 1) != 0)
    }
    #[doc = "Bit 26 - u1_plda_pcie_pcie_rxterm_en"]
    #[inline(always)]
    pub fn u1_plda_pcie_pcie_rxterm_en(&self) -> U1_PLDA_PCIE_PCIE_RXTERM_EN_R {
        U1_PLDA_PCIE_PCIE_RXTERM_EN_R::new(((self.bits >> 26) & 1) != 0)
    }
    #[doc = "Bit 27 - u1_plda_pcie_pcie_tx_oneszeros"]
    #[inline(always)]
    pub fn u1_plda_pcie_pcie_tx_oneszeros(&self) -> U1_PLDA_PCIE_PCIE_TX_ONESZEROS_R {
        U1_PLDA_PCIE_PCIE_TX_ONESZEROS_R::new(((self.bits >> 27) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - u1_plda_pcie_mperstn"]
    #[inline(always)]
    #[must_use]
    pub fn u1_plda_pcie_mperstn(
        &mut self,
    ) -> U1_PLDA_PCIE_MPERSTN_W<STG_SYSCONSAIF_SYSCFG744_SPEC, 0> {
        U1_PLDA_PCIE_MPERSTN_W::new(self)
    }
    #[doc = "Bit 1 - u1_plda_pcie_pcie_ebuf_mode"]
    #[inline(always)]
    #[must_use]
    pub fn u1_plda_pcie_pcie_ebuf_mode(
        &mut self,
    ) -> U1_PLDA_PCIE_PCIE_EBUF_MODE_W<STG_SYSCONSAIF_SYSCFG744_SPEC, 1> {
        U1_PLDA_PCIE_PCIE_EBUF_MODE_W::new(self)
    }
    #[doc = "Bits 2:24 - u1_plda_pcie_pcie_phy_test_cfg"]
    #[inline(always)]
    #[must_use]
    pub fn u1_plda_pcie_pcie_phy_test_cfg(
        &mut self,
    ) -> U1_PLDA_PCIE_PCIE_PHY_TEST_CFG_W<STG_SYSCONSAIF_SYSCFG744_SPEC, 2> {
        U1_PLDA_PCIE_PCIE_PHY_TEST_CFG_W::new(self)
    }
    #[doc = "Bit 25 - u1_plda_pcie_pcie_rx_eq_training"]
    #[inline(always)]
    #[must_use]
    pub fn u1_plda_pcie_pcie_rx_eq_training(
        &mut self,
    ) -> U1_PLDA_PCIE_PCIE_RX_EQ_TRAINING_W<STG_SYSCONSAIF_SYSCFG744_SPEC, 25> {
        U1_PLDA_PCIE_PCIE_RX_EQ_TRAINING_W::new(self)
    }
    #[doc = "Bit 26 - u1_plda_pcie_pcie_rxterm_en"]
    #[inline(always)]
    #[must_use]
    pub fn u1_plda_pcie_pcie_rxterm_en(
        &mut self,
    ) -> U1_PLDA_PCIE_PCIE_RXTERM_EN_W<STG_SYSCONSAIF_SYSCFG744_SPEC, 26> {
        U1_PLDA_PCIE_PCIE_RXTERM_EN_W::new(self)
    }
    #[doc = "Bit 27 - u1_plda_pcie_pcie_tx_oneszeros"]
    #[inline(always)]
    #[must_use]
    pub fn u1_plda_pcie_pcie_tx_oneszeros(
        &mut self,
    ) -> U1_PLDA_PCIE_PCIE_TX_ONESZEROS_W<STG_SYSCONSAIF_SYSCFG744_SPEC, 27> {
        U1_PLDA_PCIE_PCIE_TX_ONESZEROS_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.bits = bits;
        self
    }
}
#[doc = "STG SYSCONSAIF SYSCFG 744\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`stg_sysconsaif_syscfg744::R`](R).  You can [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`stg_sysconsaif_syscfg744::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct STG_SYSCONSAIF_SYSCFG744_SPEC;
impl crate::RegisterSpec for STG_SYSCONSAIF_SYSCFG744_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`stg_sysconsaif_syscfg744::R`](R) reader structure"]
impl crate::Readable for STG_SYSCONSAIF_SYSCFG744_SPEC {}
#[doc = "`write(|w| ..)` method takes [`stg_sysconsaif_syscfg744::W`](W) writer structure"]
impl crate::Writable for STG_SYSCONSAIF_SYSCFG744_SPEC {
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
