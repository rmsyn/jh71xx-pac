#[doc = "Register `sys_syscfg_36` reader"]
pub type R = crate::R<SYS_SYSCFG_36_SPEC>;
#[doc = "Register `sys_syscfg_36` writer"]
pub type W = crate::W<SYS_SYSCFG_36_SPEC>;
#[doc = "Field `gmac5_axi64_mac_speed` reader - gmac5_axi64_mac_speed"]
pub type GMAC5_AXI64_MAC_SPEED_R = crate::FieldReader;
#[doc = "Field `gmac5_axi64_phy_intf_sel` reader - Active PHY Selected | When you have multiple GMAC PHY interfaces in your configuration, this field indicates the sampled value of the PHY selector during reset de-assertion. | Values: 0x0:(GMII or MII), 0x01:RGMII, 0x2:SGMII, 0x3:TBI, 0x4:RMII, 0x5:RTBI, 0x6:SMII, 0x7:REVMII"]
pub type GMAC5_AXI64_PHY_INTF_SEL_R = crate::FieldReader;
#[doc = "Field `gmac5_axi64_phy_intf_sel` writer - Active PHY Selected | When you have multiple GMAC PHY interfaces in your configuration, this field indicates the sampled value of the PHY selector during reset de-assertion. | Values: 0x0:(GMII or MII), 0x01:RGMII, 0x2:SGMII, 0x3:TBI, 0x4:RMII, 0x5:RTBI, 0x6:SMII, 0x7:REVMII"]
pub type GMAC5_AXI64_PHY_INTF_SEL_W<'a, REG> = crate::FieldWriter<'a, REG, 3>;
impl R {
    #[doc = "Bits 0:1 - gmac5_axi64_mac_speed"]
    #[inline(always)]
    pub fn gmac5_axi64_mac_speed(&self) -> GMAC5_AXI64_MAC_SPEED_R {
        GMAC5_AXI64_MAC_SPEED_R::new((self.bits & 3) as u8)
    }
    #[doc = "Bits 2:4 - Active PHY Selected | When you have multiple GMAC PHY interfaces in your configuration, this field indicates the sampled value of the PHY selector during reset de-assertion. | Values: 0x0:(GMII or MII), 0x01:RGMII, 0x2:SGMII, 0x3:TBI, 0x4:RMII, 0x5:RTBI, 0x6:SMII, 0x7:REVMII"]
    #[inline(always)]
    pub fn gmac5_axi64_phy_intf_sel(&self) -> GMAC5_AXI64_PHY_INTF_SEL_R {
        GMAC5_AXI64_PHY_INTF_SEL_R::new(((self.bits >> 2) & 7) as u8)
    }
}
impl W {
    #[doc = "Bits 2:4 - Active PHY Selected | When you have multiple GMAC PHY interfaces in your configuration, this field indicates the sampled value of the PHY selector during reset de-assertion. | Values: 0x0:(GMII or MII), 0x01:RGMII, 0x2:SGMII, 0x3:TBI, 0x4:RMII, 0x5:RTBI, 0x6:SMII, 0x7:REVMII"]
    #[inline(always)]
    #[must_use]
    pub fn gmac5_axi64_phy_intf_sel(&mut self) -> GMAC5_AXI64_PHY_INTF_SEL_W<SYS_SYSCFG_36_SPEC> {
        GMAC5_AXI64_PHY_INTF_SEL_W::new(self, 2)
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
#[doc = "SYS SYSCONSAIF SYSCFG 144\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`sys_syscfg_36::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`sys_syscfg_36::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct SYS_SYSCFG_36_SPEC;
impl crate::RegisterSpec for SYS_SYSCFG_36_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`sys_syscfg_36::R`](R) reader structure"]
impl crate::Readable for SYS_SYSCFG_36_SPEC {}
#[doc = "`write(|w| ..)` method takes [`sys_syscfg_36::W`](W) writer structure"]
impl crate::Writable for SYS_SYSCFG_36_SPEC {
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets sys_syscfg_36 to value 0"]
impl crate::Resettable for SYS_SYSCFG_36_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
