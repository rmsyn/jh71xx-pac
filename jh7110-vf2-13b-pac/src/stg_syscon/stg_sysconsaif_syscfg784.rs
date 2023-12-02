#[doc = "Register `stg_sysconsaif_syscfg784` reader"]
pub type R = crate::R<STG_SYSCONSAIF_SYSCFG784_SPEC>;
#[doc = "Register `stg_sysconsaif_syscfg784` writer"]
pub type W = crate::W<STG_SYSCONSAIF_SYSCFG784_SPEC>;
#[doc = "Field `u1_plda_pcie_pl_wake_in` reader - u1_plda_pcie_pl_wake_in"]
pub type U1_PLDA_PCIE_PL_WAKE_IN_R = crate::BitReader;
#[doc = "Field `u1_plda_pcie_pl_wake_in` writer - u1_plda_pcie_pl_wake_in"]
pub type U1_PLDA_PCIE_PL_WAKE_IN_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `u1_plda_pcie_pl_wake_oen` reader - u1_plda_pcie_pl_wake_oen"]
pub type U1_PLDA_PCIE_PL_WAKE_OEN_R = crate::BitReader;
#[doc = "Field `u1_plda_pcie_rx_standby_o` reader - u1_plda_pcie_rx_standby_o"]
pub type U1_PLDA_PCIE_RX_STANDBY_O_R = crate::BitReader;
impl R {
    #[doc = "Bit 0 - u1_plda_pcie_pl_wake_in"]
    #[inline(always)]
    pub fn u1_plda_pcie_pl_wake_in(&self) -> U1_PLDA_PCIE_PL_WAKE_IN_R {
        U1_PLDA_PCIE_PL_WAKE_IN_R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - u1_plda_pcie_pl_wake_oen"]
    #[inline(always)]
    pub fn u1_plda_pcie_pl_wake_oen(&self) -> U1_PLDA_PCIE_PL_WAKE_OEN_R {
        U1_PLDA_PCIE_PL_WAKE_OEN_R::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - u1_plda_pcie_rx_standby_o"]
    #[inline(always)]
    pub fn u1_plda_pcie_rx_standby_o(&self) -> U1_PLDA_PCIE_RX_STANDBY_O_R {
        U1_PLDA_PCIE_RX_STANDBY_O_R::new(((self.bits >> 2) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - u1_plda_pcie_pl_wake_in"]
    #[inline(always)]
    #[must_use]
    pub fn u1_plda_pcie_pl_wake_in(
        &mut self,
    ) -> U1_PLDA_PCIE_PL_WAKE_IN_W<STG_SYSCONSAIF_SYSCFG784_SPEC> {
        U1_PLDA_PCIE_PL_WAKE_IN_W::new(self, 0)
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
#[doc = "STG SYSCONSAIF SYSCFG 784\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`stg_sysconsaif_syscfg784::R`](R).  You can [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`stg_sysconsaif_syscfg784::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct STG_SYSCONSAIF_SYSCFG784_SPEC;
impl crate::RegisterSpec for STG_SYSCONSAIF_SYSCFG784_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`stg_sysconsaif_syscfg784::R`](R) reader structure"]
impl crate::Readable for STG_SYSCONSAIF_SYSCFG784_SPEC {}
#[doc = "`write(|w| ..)` method takes [`stg_sysconsaif_syscfg784::W`](W) writer structure"]
impl crate::Writable for STG_SYSCONSAIF_SYSCFG784_SPEC {
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
