#[doc = "Register `stg_sysconsaif_syscfg932` reader"]
pub type R = crate::R<STG_SYSCONSAIF_SYSCFG932_SPEC>;
#[doc = "Register `stg_sysconsaif_syscfg932` writer"]
pub type W = crate::W<STG_SYSCONSAIF_SYSCFG932_SPEC>;
#[doc = "Field `u1_plda_pcie_tx_pattern` reader - u1_plda_pcie_tx_pattern"]
pub type U1_PLDA_PCIE_TX_PATTERN_R = crate::FieldReader;
#[doc = "Field `u1_plda_pcie_tx_pattern` writer - u1_plda_pcie_tx_pattern"]
pub type U1_PLDA_PCIE_TX_PATTERN_W<'a, REG, const O: u8> = crate::FieldWriter<'a, REG, 2, O>;
#[doc = "Field `u1_plda_pcie_usb3_bus_width` reader - u1_plda_pcie_usb3_bus_width"]
pub type U1_PLDA_PCIE_USB3_BUS_WIDTH_R = crate::FieldReader;
#[doc = "Field `u1_plda_pcie_usb3_bus_width` writer - u1_plda_pcie_usb3_bus_width"]
pub type U1_PLDA_PCIE_USB3_BUS_WIDTH_W<'a, REG, const O: u8> = crate::FieldWriter<'a, REG, 2, O>;
#[doc = "Field `u1_plda_pcie_usb3_phy_enable` reader - u1_plda_pcie_usb3_phy_enable"]
pub type U1_PLDA_PCIE_USB3_PHY_ENABLE_R = crate::BitReader;
#[doc = "Field `u1_plda_pcie_usb3_phy_enable` writer - u1_plda_pcie_usb3_phy_enable"]
pub type U1_PLDA_PCIE_USB3_PHY_ENABLE_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O>;
#[doc = "Field `u1_plda_pcie_usb3_rate` reader - u1_plda_pcie_usb3_rate"]
pub type U1_PLDA_PCIE_USB3_RATE_R = crate::FieldReader;
#[doc = "Field `u1_plda_pcie_usb3_rate` writer - u1_plda_pcie_usb3_rate"]
pub type U1_PLDA_PCIE_USB3_RATE_W<'a, REG, const O: u8> = crate::FieldWriter<'a, REG, 2, O>;
#[doc = "Field `u1_plda_pcie_usb3_rx_standby` reader - u1_plda_pcie_usb3_rx_standby"]
pub type U1_PLDA_PCIE_USB3_RX_STANDBY_R = crate::BitReader;
#[doc = "Field `u1_plda_pcie_usb3_rx_standby` writer - u1_plda_pcie_usb3_rx_standby"]
pub type U1_PLDA_PCIE_USB3_RX_STANDBY_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O>;
#[doc = "Field `u1_plda_pcie_xwdecerr` reader - u1_plda_pcie_xwdecerr"]
pub type U1_PLDA_PCIE_XWDECERR_R = crate::BitReader;
#[doc = "Field `u1_plda_pcie_xwerrclr` reader - u1_plda_pcie_xwerrclr"]
pub type U1_PLDA_PCIE_XWERRCLR_R = crate::BitReader;
#[doc = "Field `u1_plda_pcie_xwerrclr` writer - u1_plda_pcie_xwerrclr"]
pub type U1_PLDA_PCIE_XWERRCLR_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O>;
#[doc = "Field `u1_plda_pcie_xwslverr` reader - u1_plda_pcie_xwslverr"]
pub type U1_PLDA_PCIE_XWSLVERR_R = crate::BitReader;
impl R {
    #[doc = "Bits 0:1 - u1_plda_pcie_tx_pattern"]
    #[inline(always)]
    pub fn u1_plda_pcie_tx_pattern(&self) -> U1_PLDA_PCIE_TX_PATTERN_R {
        U1_PLDA_PCIE_TX_PATTERN_R::new((self.bits & 3) as u8)
    }
    #[doc = "Bits 2:3 - u1_plda_pcie_usb3_bus_width"]
    #[inline(always)]
    pub fn u1_plda_pcie_usb3_bus_width(&self) -> U1_PLDA_PCIE_USB3_BUS_WIDTH_R {
        U1_PLDA_PCIE_USB3_BUS_WIDTH_R::new(((self.bits >> 2) & 3) as u8)
    }
    #[doc = "Bit 4 - u1_plda_pcie_usb3_phy_enable"]
    #[inline(always)]
    pub fn u1_plda_pcie_usb3_phy_enable(&self) -> U1_PLDA_PCIE_USB3_PHY_ENABLE_R {
        U1_PLDA_PCIE_USB3_PHY_ENABLE_R::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bits 5:6 - u1_plda_pcie_usb3_rate"]
    #[inline(always)]
    pub fn u1_plda_pcie_usb3_rate(&self) -> U1_PLDA_PCIE_USB3_RATE_R {
        U1_PLDA_PCIE_USB3_RATE_R::new(((self.bits >> 5) & 3) as u8)
    }
    #[doc = "Bit 7 - u1_plda_pcie_usb3_rx_standby"]
    #[inline(always)]
    pub fn u1_plda_pcie_usb3_rx_standby(&self) -> U1_PLDA_PCIE_USB3_RX_STANDBY_R {
        U1_PLDA_PCIE_USB3_RX_STANDBY_R::new(((self.bits >> 7) & 1) != 0)
    }
    #[doc = "Bit 8 - u1_plda_pcie_xwdecerr"]
    #[inline(always)]
    pub fn u1_plda_pcie_xwdecerr(&self) -> U1_PLDA_PCIE_XWDECERR_R {
        U1_PLDA_PCIE_XWDECERR_R::new(((self.bits >> 8) & 1) != 0)
    }
    #[doc = "Bit 9 - u1_plda_pcie_xwerrclr"]
    #[inline(always)]
    pub fn u1_plda_pcie_xwerrclr(&self) -> U1_PLDA_PCIE_XWERRCLR_R {
        U1_PLDA_PCIE_XWERRCLR_R::new(((self.bits >> 9) & 1) != 0)
    }
    #[doc = "Bit 10 - u1_plda_pcie_xwslverr"]
    #[inline(always)]
    pub fn u1_plda_pcie_xwslverr(&self) -> U1_PLDA_PCIE_XWSLVERR_R {
        U1_PLDA_PCIE_XWSLVERR_R::new(((self.bits >> 10) & 1) != 0)
    }
}
impl W {
    #[doc = "Bits 0:1 - u1_plda_pcie_tx_pattern"]
    #[inline(always)]
    #[must_use]
    pub fn u1_plda_pcie_tx_pattern(
        &mut self,
    ) -> U1_PLDA_PCIE_TX_PATTERN_W<STG_SYSCONSAIF_SYSCFG932_SPEC, 0> {
        U1_PLDA_PCIE_TX_PATTERN_W::new(self)
    }
    #[doc = "Bits 2:3 - u1_plda_pcie_usb3_bus_width"]
    #[inline(always)]
    #[must_use]
    pub fn u1_plda_pcie_usb3_bus_width(
        &mut self,
    ) -> U1_PLDA_PCIE_USB3_BUS_WIDTH_W<STG_SYSCONSAIF_SYSCFG932_SPEC, 2> {
        U1_PLDA_PCIE_USB3_BUS_WIDTH_W::new(self)
    }
    #[doc = "Bit 4 - u1_plda_pcie_usb3_phy_enable"]
    #[inline(always)]
    #[must_use]
    pub fn u1_plda_pcie_usb3_phy_enable(
        &mut self,
    ) -> U1_PLDA_PCIE_USB3_PHY_ENABLE_W<STG_SYSCONSAIF_SYSCFG932_SPEC, 4> {
        U1_PLDA_PCIE_USB3_PHY_ENABLE_W::new(self)
    }
    #[doc = "Bits 5:6 - u1_plda_pcie_usb3_rate"]
    #[inline(always)]
    #[must_use]
    pub fn u1_plda_pcie_usb3_rate(
        &mut self,
    ) -> U1_PLDA_PCIE_USB3_RATE_W<STG_SYSCONSAIF_SYSCFG932_SPEC, 5> {
        U1_PLDA_PCIE_USB3_RATE_W::new(self)
    }
    #[doc = "Bit 7 - u1_plda_pcie_usb3_rx_standby"]
    #[inline(always)]
    #[must_use]
    pub fn u1_plda_pcie_usb3_rx_standby(
        &mut self,
    ) -> U1_PLDA_PCIE_USB3_RX_STANDBY_W<STG_SYSCONSAIF_SYSCFG932_SPEC, 7> {
        U1_PLDA_PCIE_USB3_RX_STANDBY_W::new(self)
    }
    #[doc = "Bit 9 - u1_plda_pcie_xwerrclr"]
    #[inline(always)]
    #[must_use]
    pub fn u1_plda_pcie_xwerrclr(
        &mut self,
    ) -> U1_PLDA_PCIE_XWERRCLR_W<STG_SYSCONSAIF_SYSCFG932_SPEC, 9> {
        U1_PLDA_PCIE_XWERRCLR_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.bits = bits;
        self
    }
}
#[doc = "STG SYSCONSAIF SYSCFG 932\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`stg_sysconsaif_syscfg932::R`](R).  You can [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`stg_sysconsaif_syscfg932::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct STG_SYSCONSAIF_SYSCFG932_SPEC;
impl crate::RegisterSpec for STG_SYSCONSAIF_SYSCFG932_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`stg_sysconsaif_syscfg932::R`](R) reader structure"]
impl crate::Readable for STG_SYSCONSAIF_SYSCFG932_SPEC {}
#[doc = "`write(|w| ..)` method takes [`stg_sysconsaif_syscfg932::W`](W) writer structure"]
impl crate::Writable for STG_SYSCONSAIF_SYSCFG932_SPEC {
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}