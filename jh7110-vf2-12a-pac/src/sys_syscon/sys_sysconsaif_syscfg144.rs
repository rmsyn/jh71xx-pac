#[doc = "Register `sys_sysconsaif_syscfg144` reader"]
pub type R = crate::R<SYS_SYSCONSAIF_SYSCFG144_SPEC>;
#[doc = "Register `sys_sysconsaif_syscfg144` writer"]
pub type W = crate::W<SYS_SYSCONSAIF_SYSCFG144_SPEC>;
#[doc = "Field `u1_gmac5_axi64_mac_speed_0` reader - u1_gmac5_axi64_mac_speed_0"]
pub type U1_GMAC5_AXI64_MAC_SPEED_0_R = crate::FieldReader;
#[doc = "Field `u1_gmac5_axi64_phy_intf_sel_i` reader - Active PHY Selected | When you have multiple GMAC PHY interfaces in your configuration, this field indicates the sampled value of the PHY selector during reset de-assertion. | Values: 0x0:(GMII or MII), 0x01:RGMII, 0x2:SGMII, 0x3:TBI, 0x4:RMII, 0x5:RTBI, 0x6:SMII, 0x7:REVMII"]
pub type U1_GMAC5_AXI64_PHY_INTF_SEL_I_R = crate::FieldReader;
#[doc = "Field `u1_gmac5_axi64_phy_intf_sel_i` writer - Active PHY Selected | When you have multiple GMAC PHY interfaces in your configuration, this field indicates the sampled value of the PHY selector during reset de-assertion. | Values: 0x0:(GMII or MII), 0x01:RGMII, 0x2:SGMII, 0x3:TBI, 0x4:RMII, 0x5:RTBI, 0x6:SMII, 0x7:REVMII"]
pub type U1_GMAC5_AXI64_PHY_INTF_SEL_I_W<'a, REG, const O: u8> = crate::FieldWriter<'a, REG, 3, O>;
impl R {
    #[doc = "Bits 0:1 - u1_gmac5_axi64_mac_speed_0"]
    #[inline(always)]
    pub fn u1_gmac5_axi64_mac_speed_0(&self) -> U1_GMAC5_AXI64_MAC_SPEED_0_R {
        U1_GMAC5_AXI64_MAC_SPEED_0_R::new((self.bits & 3) as u8)
    }
    #[doc = "Bits 2:4 - Active PHY Selected | When you have multiple GMAC PHY interfaces in your configuration, this field indicates the sampled value of the PHY selector during reset de-assertion. | Values: 0x0:(GMII or MII), 0x01:RGMII, 0x2:SGMII, 0x3:TBI, 0x4:RMII, 0x5:RTBI, 0x6:SMII, 0x7:REVMII"]
    #[inline(always)]
    pub fn u1_gmac5_axi64_phy_intf_sel_i(&self) -> U1_GMAC5_AXI64_PHY_INTF_SEL_I_R {
        U1_GMAC5_AXI64_PHY_INTF_SEL_I_R::new(((self.bits >> 2) & 7) as u8)
    }
}
impl W {
    #[doc = "Bits 2:4 - Active PHY Selected | When you have multiple GMAC PHY interfaces in your configuration, this field indicates the sampled value of the PHY selector during reset de-assertion. | Values: 0x0:(GMII or MII), 0x01:RGMII, 0x2:SGMII, 0x3:TBI, 0x4:RMII, 0x5:RTBI, 0x6:SMII, 0x7:REVMII"]
    #[inline(always)]
    #[must_use]
    pub fn u1_gmac5_axi64_phy_intf_sel_i(
        &mut self,
    ) -> U1_GMAC5_AXI64_PHY_INTF_SEL_I_W<SYS_SYSCONSAIF_SYSCFG144_SPEC, 2> {
        U1_GMAC5_AXI64_PHY_INTF_SEL_I_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.bits = bits;
        self
    }
}
#[doc = "SYS SYSCONSAIF SYSCFG 144\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`sys_sysconsaif_syscfg144::R`](R).  You can [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`sys_sysconsaif_syscfg144::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct SYS_SYSCONSAIF_SYSCFG144_SPEC;
impl crate::RegisterSpec for SYS_SYSCONSAIF_SYSCFG144_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`sys_sysconsaif_syscfg144::R`](R) reader structure"]
impl crate::Readable for SYS_SYSCONSAIF_SYSCFG144_SPEC {}
#[doc = "`write(|w| ..)` method takes [`sys_sysconsaif_syscfg144::W`](W) writer structure"]
impl crate::Writable for SYS_SYSCONSAIF_SYSCFG144_SPEC {
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
