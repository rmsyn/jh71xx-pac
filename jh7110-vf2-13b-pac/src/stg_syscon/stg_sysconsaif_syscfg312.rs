#[doc = "Register `stg_sysconsaif_syscfg312` reader"]
pub type R = crate::R<STG_SYSCONSAIF_SYSCFG312_SPEC>;
#[doc = "Register `stg_sysconsaif_syscfg312` writer"]
pub type W = crate::W<STG_SYSCONSAIF_SYSCFG312_SPEC>;
#[doc = "Field `u0_plda_pcie_mperstn` reader - u0_plda_pcie_mperstn"]
pub type U0_PLDA_PCIE_MPERSTN_R = crate::BitReader;
#[doc = "Field `u0_plda_pcie_mperstn` writer - u0_plda_pcie_mperstn"]
pub type U0_PLDA_PCIE_MPERSTN_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `u0_plda_pcie_pcie_ebuf_mode` reader - u0_plda_pcie_pcie_ebuf_mode"]
pub type U0_PLDA_PCIE_PCIE_EBUF_MODE_R = crate::BitReader;
#[doc = "Field `u0_plda_pcie_pcie_ebuf_mode` writer - u0_plda_pcie_pcie_ebuf_mode"]
pub type U0_PLDA_PCIE_PCIE_EBUF_MODE_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `u0_plda_pcie_pcie_phy_test_cfg` reader - u0_plda_pcie_pcie_phy_test_cfg"]
pub type U0_PLDA_PCIE_PCIE_PHY_TEST_CFG_R = crate::FieldReader<u32>;
#[doc = "Field `u0_plda_pcie_pcie_phy_test_cfg` writer - u0_plda_pcie_pcie_phy_test_cfg"]
pub type U0_PLDA_PCIE_PCIE_PHY_TEST_CFG_W<'a, REG> = crate::FieldWriter<'a, REG, 23, u32>;
#[doc = "Field `u0_plda_pcie_pcie_rx_eq_training` reader - u0_plda_pcie_pcie_rx_eq_training"]
pub type U0_PLDA_PCIE_PCIE_RX_EQ_TRAINING_R = crate::BitReader;
#[doc = "Field `u0_plda_pcie_pcie_rx_eq_training` writer - u0_plda_pcie_pcie_rx_eq_training"]
pub type U0_PLDA_PCIE_PCIE_RX_EQ_TRAINING_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `u0_plda_pcie_pcie_rxterm_en` reader - u0_plda_pcie_pcie_rxterm_en"]
pub type U0_PLDA_PCIE_PCIE_RXTERM_EN_R = crate::BitReader;
#[doc = "Field `u0_plda_pcie_pcie_rxterm_en` writer - u0_plda_pcie_pcie_rxterm_en"]
pub type U0_PLDA_PCIE_PCIE_RXTERM_EN_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `u0_plda_pcie_pcie_tx_onezeros` reader - u0_plda_pcie_pcie_tx_onezeros"]
pub type U0_PLDA_PCIE_PCIE_TX_ONEZEROS_R = crate::BitReader;
#[doc = "Field `u0_plda_pcie_pcie_tx_onezeros` writer - u0_plda_pcie_pcie_tx_onezeros"]
pub type U0_PLDA_PCIE_PCIE_TX_ONEZEROS_W<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bit 0 - u0_plda_pcie_mperstn"]
    #[inline(always)]
    pub fn u0_plda_pcie_mperstn(&self) -> U0_PLDA_PCIE_MPERSTN_R {
        U0_PLDA_PCIE_MPERSTN_R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - u0_plda_pcie_pcie_ebuf_mode"]
    #[inline(always)]
    pub fn u0_plda_pcie_pcie_ebuf_mode(&self) -> U0_PLDA_PCIE_PCIE_EBUF_MODE_R {
        U0_PLDA_PCIE_PCIE_EBUF_MODE_R::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bits 2:24 - u0_plda_pcie_pcie_phy_test_cfg"]
    #[inline(always)]
    pub fn u0_plda_pcie_pcie_phy_test_cfg(&self) -> U0_PLDA_PCIE_PCIE_PHY_TEST_CFG_R {
        U0_PLDA_PCIE_PCIE_PHY_TEST_CFG_R::new((self.bits >> 2) & 0x007f_ffff)
    }
    #[doc = "Bit 25 - u0_plda_pcie_pcie_rx_eq_training"]
    #[inline(always)]
    pub fn u0_plda_pcie_pcie_rx_eq_training(&self) -> U0_PLDA_PCIE_PCIE_RX_EQ_TRAINING_R {
        U0_PLDA_PCIE_PCIE_RX_EQ_TRAINING_R::new(((self.bits >> 25) & 1) != 0)
    }
    #[doc = "Bit 26 - u0_plda_pcie_pcie_rxterm_en"]
    #[inline(always)]
    pub fn u0_plda_pcie_pcie_rxterm_en(&self) -> U0_PLDA_PCIE_PCIE_RXTERM_EN_R {
        U0_PLDA_PCIE_PCIE_RXTERM_EN_R::new(((self.bits >> 26) & 1) != 0)
    }
    #[doc = "Bit 27 - u0_plda_pcie_pcie_tx_onezeros"]
    #[inline(always)]
    pub fn u0_plda_pcie_pcie_tx_onezeros(&self) -> U0_PLDA_PCIE_PCIE_TX_ONEZEROS_R {
        U0_PLDA_PCIE_PCIE_TX_ONEZEROS_R::new(((self.bits >> 27) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - u0_plda_pcie_mperstn"]
    #[inline(always)]
    #[must_use]
    pub fn u0_plda_pcie_mperstn(
        &mut self,
    ) -> U0_PLDA_PCIE_MPERSTN_W<STG_SYSCONSAIF_SYSCFG312_SPEC> {
        U0_PLDA_PCIE_MPERSTN_W::new(self, 0)
    }
    #[doc = "Bit 1 - u0_plda_pcie_pcie_ebuf_mode"]
    #[inline(always)]
    #[must_use]
    pub fn u0_plda_pcie_pcie_ebuf_mode(
        &mut self,
    ) -> U0_PLDA_PCIE_PCIE_EBUF_MODE_W<STG_SYSCONSAIF_SYSCFG312_SPEC> {
        U0_PLDA_PCIE_PCIE_EBUF_MODE_W::new(self, 1)
    }
    #[doc = "Bits 2:24 - u0_plda_pcie_pcie_phy_test_cfg"]
    #[inline(always)]
    #[must_use]
    pub fn u0_plda_pcie_pcie_phy_test_cfg(
        &mut self,
    ) -> U0_PLDA_PCIE_PCIE_PHY_TEST_CFG_W<STG_SYSCONSAIF_SYSCFG312_SPEC> {
        U0_PLDA_PCIE_PCIE_PHY_TEST_CFG_W::new(self, 2)
    }
    #[doc = "Bit 25 - u0_plda_pcie_pcie_rx_eq_training"]
    #[inline(always)]
    #[must_use]
    pub fn u0_plda_pcie_pcie_rx_eq_training(
        &mut self,
    ) -> U0_PLDA_PCIE_PCIE_RX_EQ_TRAINING_W<STG_SYSCONSAIF_SYSCFG312_SPEC> {
        U0_PLDA_PCIE_PCIE_RX_EQ_TRAINING_W::new(self, 25)
    }
    #[doc = "Bit 26 - u0_plda_pcie_pcie_rxterm_en"]
    #[inline(always)]
    #[must_use]
    pub fn u0_plda_pcie_pcie_rxterm_en(
        &mut self,
    ) -> U0_PLDA_PCIE_PCIE_RXTERM_EN_W<STG_SYSCONSAIF_SYSCFG312_SPEC> {
        U0_PLDA_PCIE_PCIE_RXTERM_EN_W::new(self, 26)
    }
    #[doc = "Bit 27 - u0_plda_pcie_pcie_tx_onezeros"]
    #[inline(always)]
    #[must_use]
    pub fn u0_plda_pcie_pcie_tx_onezeros(
        &mut self,
    ) -> U0_PLDA_PCIE_PCIE_TX_ONEZEROS_W<STG_SYSCONSAIF_SYSCFG312_SPEC> {
        U0_PLDA_PCIE_PCIE_TX_ONEZEROS_W::new(self, 27)
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
#[doc = "STG SYSCONSAIF SYSCFG 312\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`stg_sysconsaif_syscfg312::R`](R).  You can [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`stg_sysconsaif_syscfg312::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct STG_SYSCONSAIF_SYSCFG312_SPEC;
impl crate::RegisterSpec for STG_SYSCONSAIF_SYSCFG312_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`stg_sysconsaif_syscfg312::R`](R) reader structure"]
impl crate::Readable for STG_SYSCONSAIF_SYSCFG312_SPEC {}
#[doc = "`write(|w| ..)` method takes [`stg_sysconsaif_syscfg312::W`](W) writer structure"]
impl crate::Writable for STG_SYSCONSAIF_SYSCFG312_SPEC {
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
