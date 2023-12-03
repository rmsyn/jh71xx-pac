#[doc = "Register `stg_syscfg_196` reader"]
pub type R = crate::R<STG_SYSCFG_196_SPEC>;
#[doc = "Register `stg_syscfg_196` writer"]
pub type W = crate::W<STG_SYSCFG_196_SPEC>;
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
    pub fn u1_plda_pcie_pl_wake_in(&mut self) -> U1_PLDA_PCIE_PL_WAKE_IN_W<STG_SYSCFG_196_SPEC> {
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
#[doc = "STG SYSCONSAIF SYSCFG 784\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`stg_syscfg_196::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`stg_syscfg_196::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct STG_SYSCFG_196_SPEC;
impl crate::RegisterSpec for STG_SYSCFG_196_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`stg_syscfg_196::R`](R) reader structure"]
impl crate::Readable for STG_SYSCFG_196_SPEC {}
#[doc = "`write(|w| ..)` method takes [`stg_syscfg_196::W`](W) writer structure"]
impl crate::Writable for STG_SYSCFG_196_SPEC {
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets stg_syscfg_196 to value 0"]
impl crate::Resettable for STG_SYSCFG_196_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
