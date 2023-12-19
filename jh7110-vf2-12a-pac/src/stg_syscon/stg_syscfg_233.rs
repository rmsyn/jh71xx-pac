#[doc = "Register `stg_syscfg_233` reader"]
pub type R = crate::R<STG_SYSCFG_233_SPEC>;
#[doc = "Register `stg_syscfg_233` writer"]
pub type W = crate::W<STG_SYSCFG_233_SPEC>;
#[doc = "Field `u1_pcie_tx_pattern` reader - u1_pcie_tx_pattern"]
pub type U1_PCIE_TX_PATTERN_R = crate::FieldReader;
#[doc = "Field `u1_pcie_tx_pattern` writer - u1_pcie_tx_pattern"]
pub type U1_PCIE_TX_PATTERN_W<'a, REG> = crate::FieldWriter<'a, REG, 2>;
#[doc = "Field `u1_pcie_usb3_bus_width` reader - u1_pcie_usb3_bus_width"]
pub type U1_PCIE_USB3_BUS_WIDTH_R = crate::FieldReader;
#[doc = "Field `u1_pcie_usb3_bus_width` writer - u1_pcie_usb3_bus_width"]
pub type U1_PCIE_USB3_BUS_WIDTH_W<'a, REG> = crate::FieldWriter<'a, REG, 2>;
#[doc = "Field `u1_pcie_usb3_phy_enable` reader - u1_pcie_usb3_phy_enable"]
pub type U1_PCIE_USB3_PHY_ENABLE_R = crate::BitReader;
#[doc = "Field `u1_pcie_usb3_phy_enable` writer - u1_pcie_usb3_phy_enable"]
pub type U1_PCIE_USB3_PHY_ENABLE_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `u1_pcie_usb3_rate` reader - u1_pcie_usb3_rate"]
pub type U1_PCIE_USB3_RATE_R = crate::FieldReader;
#[doc = "Field `u1_pcie_usb3_rate` writer - u1_pcie_usb3_rate"]
pub type U1_PCIE_USB3_RATE_W<'a, REG> = crate::FieldWriter<'a, REG, 2>;
#[doc = "Field `u1_pcie_usb3_rx_standby` reader - u1_pcie_usb3_rx_standby"]
pub type U1_PCIE_USB3_RX_STANDBY_R = crate::BitReader;
#[doc = "Field `u1_pcie_usb3_rx_standby` writer - u1_pcie_usb3_rx_standby"]
pub type U1_PCIE_USB3_RX_STANDBY_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `u1_pcie_xwdecerr` reader - u1_pcie_xwdecerr"]
pub type U1_PCIE_XWDECERR_R = crate::BitReader;
#[doc = "Field `u1_pcie_xwerrclr` reader - u1_pcie_xwerrclr"]
pub type U1_PCIE_XWERRCLR_R = crate::BitReader;
#[doc = "Field `u1_pcie_xwerrclr` writer - u1_pcie_xwerrclr"]
pub type U1_PCIE_XWERRCLR_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `u1_pcie_xwslverr` reader - u1_pcie_xwslverr"]
pub type U1_PCIE_XWSLVERR_R = crate::BitReader;
impl R {
    #[doc = "Bits 0:1 - u1_pcie_tx_pattern"]
    #[inline(always)]
    pub fn u1_pcie_tx_pattern(&self) -> U1_PCIE_TX_PATTERN_R {
        U1_PCIE_TX_PATTERN_R::new((self.bits & 3) as u8)
    }
    #[doc = "Bits 2:3 - u1_pcie_usb3_bus_width"]
    #[inline(always)]
    pub fn u1_pcie_usb3_bus_width(&self) -> U1_PCIE_USB3_BUS_WIDTH_R {
        U1_PCIE_USB3_BUS_WIDTH_R::new(((self.bits >> 2) & 3) as u8)
    }
    #[doc = "Bit 4 - u1_pcie_usb3_phy_enable"]
    #[inline(always)]
    pub fn u1_pcie_usb3_phy_enable(&self) -> U1_PCIE_USB3_PHY_ENABLE_R {
        U1_PCIE_USB3_PHY_ENABLE_R::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bits 5:6 - u1_pcie_usb3_rate"]
    #[inline(always)]
    pub fn u1_pcie_usb3_rate(&self) -> U1_PCIE_USB3_RATE_R {
        U1_PCIE_USB3_RATE_R::new(((self.bits >> 5) & 3) as u8)
    }
    #[doc = "Bit 7 - u1_pcie_usb3_rx_standby"]
    #[inline(always)]
    pub fn u1_pcie_usb3_rx_standby(&self) -> U1_PCIE_USB3_RX_STANDBY_R {
        U1_PCIE_USB3_RX_STANDBY_R::new(((self.bits >> 7) & 1) != 0)
    }
    #[doc = "Bit 8 - u1_pcie_xwdecerr"]
    #[inline(always)]
    pub fn u1_pcie_xwdecerr(&self) -> U1_PCIE_XWDECERR_R {
        U1_PCIE_XWDECERR_R::new(((self.bits >> 8) & 1) != 0)
    }
    #[doc = "Bit 9 - u1_pcie_xwerrclr"]
    #[inline(always)]
    pub fn u1_pcie_xwerrclr(&self) -> U1_PCIE_XWERRCLR_R {
        U1_PCIE_XWERRCLR_R::new(((self.bits >> 9) & 1) != 0)
    }
    #[doc = "Bit 10 - u1_pcie_xwslverr"]
    #[inline(always)]
    pub fn u1_pcie_xwslverr(&self) -> U1_PCIE_XWSLVERR_R {
        U1_PCIE_XWSLVERR_R::new(((self.bits >> 10) & 1) != 0)
    }
}
impl W {
    #[doc = "Bits 0:1 - u1_pcie_tx_pattern"]
    #[inline(always)]
    #[must_use]
    pub fn u1_pcie_tx_pattern(&mut self) -> U1_PCIE_TX_PATTERN_W<STG_SYSCFG_233_SPEC> {
        U1_PCIE_TX_PATTERN_W::new(self, 0)
    }
    #[doc = "Bits 2:3 - u1_pcie_usb3_bus_width"]
    #[inline(always)]
    #[must_use]
    pub fn u1_pcie_usb3_bus_width(&mut self) -> U1_PCIE_USB3_BUS_WIDTH_W<STG_SYSCFG_233_SPEC> {
        U1_PCIE_USB3_BUS_WIDTH_W::new(self, 2)
    }
    #[doc = "Bit 4 - u1_pcie_usb3_phy_enable"]
    #[inline(always)]
    #[must_use]
    pub fn u1_pcie_usb3_phy_enable(&mut self) -> U1_PCIE_USB3_PHY_ENABLE_W<STG_SYSCFG_233_SPEC> {
        U1_PCIE_USB3_PHY_ENABLE_W::new(self, 4)
    }
    #[doc = "Bits 5:6 - u1_pcie_usb3_rate"]
    #[inline(always)]
    #[must_use]
    pub fn u1_pcie_usb3_rate(&mut self) -> U1_PCIE_USB3_RATE_W<STG_SYSCFG_233_SPEC> {
        U1_PCIE_USB3_RATE_W::new(self, 5)
    }
    #[doc = "Bit 7 - u1_pcie_usb3_rx_standby"]
    #[inline(always)]
    #[must_use]
    pub fn u1_pcie_usb3_rx_standby(&mut self) -> U1_PCIE_USB3_RX_STANDBY_W<STG_SYSCFG_233_SPEC> {
        U1_PCIE_USB3_RX_STANDBY_W::new(self, 7)
    }
    #[doc = "Bit 9 - u1_pcie_xwerrclr"]
    #[inline(always)]
    #[must_use]
    pub fn u1_pcie_xwerrclr(&mut self) -> U1_PCIE_XWERRCLR_W<STG_SYSCFG_233_SPEC> {
        U1_PCIE_XWERRCLR_W::new(self, 9)
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
#[doc = "STG SYSCONSAIF SYSCFG 932\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`stg_syscfg_233::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`stg_syscfg_233::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct STG_SYSCFG_233_SPEC;
impl crate::RegisterSpec for STG_SYSCFG_233_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`stg_syscfg_233::R`](R) reader structure"]
impl crate::Readable for STG_SYSCFG_233_SPEC {}
#[doc = "`write(|w| ..)` method takes [`stg_syscfg_233::W`](W) writer structure"]
impl crate::Writable for STG_SYSCFG_233_SPEC {
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets stg_syscfg_233 to value 0"]
impl crate::Resettable for STG_SYSCFG_233_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
