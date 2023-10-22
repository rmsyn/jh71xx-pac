#[doc = "Register `stg_sysconsaif_syscfg328` reader"]
pub type R = crate::R<STG_SYSCONSAIF_SYSCFG328_SPEC>;
#[doc = "Register `stg_sysconsaif_syscfg328` writer"]
pub type W = crate::W<STG_SYSCONSAIF_SYSCFG328_SPEC>;
#[doc = "Field `u0_plda_pcie_pf3_offset` reader - u0_plda_pcie_pf3_offset"]
pub type U0_PLDA_PCIE_PF3_OFFSET_R = crate::FieldReader<u32>;
#[doc = "Field `u0_plda_pcie_pf3_offset` writer - u0_plda_pcie_pf3_offset"]
pub type U0_PLDA_PCIE_PF3_OFFSET_W<'a, REG, const O: u8> = crate::FieldWriter<'a, REG, 20, O, u32>;
#[doc = "Field `u0_plda_pcie_phy_mode` reader - u0_plda_pcie_phy_mode"]
pub type U0_PLDA_PCIE_PHY_MODE_R = crate::FieldReader;
#[doc = "Field `u0_plda_pcie_phy_mode` writer - u0_plda_pcie_phy_mode"]
pub type U0_PLDA_PCIE_PHY_MODE_W<'a, REG, const O: u8> = crate::FieldWriter<'a, REG, 2, O>;
#[doc = "Field `u0_plda_pcie_pl_clkrem_allow` reader - u0_plda_pcie_pl_clkrem_allow"]
pub type U0_PLDA_PCIE_PL_CLKREM_ALLOW_R = crate::BitReader;
#[doc = "Field `u0_plda_pcie_pl_clkrem_allow` writer - u0_plda_pcie_pl_clkrem_allow"]
pub type U0_PLDA_PCIE_PL_CLKREM_ALLOW_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O>;
#[doc = "Field `u0_plda_pcie_pl_clkreq_oen` reader - u0_plda_pcie_pl_clkreq_oen"]
pub type U0_PLDA_PCIE_PL_CLKREQ_OEN_R = crate::BitReader;
#[doc = "Field `u0_plda_pcie_pl_equ_phase` reader - u0_plda_pcie_pl_equ_phase"]
pub type U0_PLDA_PCIE_PL_EQU_PHASE_R = crate::FieldReader;
#[doc = "Field `u0_plda_pcie_pl_ltssm` reader - u0_plda_pcie_pl_ltssm"]
pub type U0_PLDA_PCIE_PL_LTSSM_R = crate::FieldReader;
impl R {
    #[doc = "Bits 0:19 - u0_plda_pcie_pf3_offset"]
    #[inline(always)]
    pub fn u0_plda_pcie_pf3_offset(&self) -> U0_PLDA_PCIE_PF3_OFFSET_R {
        U0_PLDA_PCIE_PF3_OFFSET_R::new(self.bits & 0x000f_ffff)
    }
    #[doc = "Bits 20:21 - u0_plda_pcie_phy_mode"]
    #[inline(always)]
    pub fn u0_plda_pcie_phy_mode(&self) -> U0_PLDA_PCIE_PHY_MODE_R {
        U0_PLDA_PCIE_PHY_MODE_R::new(((self.bits >> 20) & 3) as u8)
    }
    #[doc = "Bit 22 - u0_plda_pcie_pl_clkrem_allow"]
    #[inline(always)]
    pub fn u0_plda_pcie_pl_clkrem_allow(&self) -> U0_PLDA_PCIE_PL_CLKREM_ALLOW_R {
        U0_PLDA_PCIE_PL_CLKREM_ALLOW_R::new(((self.bits >> 22) & 1) != 0)
    }
    #[doc = "Bit 23 - u0_plda_pcie_pl_clkreq_oen"]
    #[inline(always)]
    pub fn u0_plda_pcie_pl_clkreq_oen(&self) -> U0_PLDA_PCIE_PL_CLKREQ_OEN_R {
        U0_PLDA_PCIE_PL_CLKREQ_OEN_R::new(((self.bits >> 23) & 1) != 0)
    }
    #[doc = "Bits 24:25 - u0_plda_pcie_pl_equ_phase"]
    #[inline(always)]
    pub fn u0_plda_pcie_pl_equ_phase(&self) -> U0_PLDA_PCIE_PL_EQU_PHASE_R {
        U0_PLDA_PCIE_PL_EQU_PHASE_R::new(((self.bits >> 24) & 3) as u8)
    }
    #[doc = "Bits 26:30 - u0_plda_pcie_pl_ltssm"]
    #[inline(always)]
    pub fn u0_plda_pcie_pl_ltssm(&self) -> U0_PLDA_PCIE_PL_LTSSM_R {
        U0_PLDA_PCIE_PL_LTSSM_R::new(((self.bits >> 26) & 0x1f) as u8)
    }
}
impl W {
    #[doc = "Bits 0:19 - u0_plda_pcie_pf3_offset"]
    #[inline(always)]
    #[must_use]
    pub fn u0_plda_pcie_pf3_offset(
        &mut self,
    ) -> U0_PLDA_PCIE_PF3_OFFSET_W<STG_SYSCONSAIF_SYSCFG328_SPEC, 0> {
        U0_PLDA_PCIE_PF3_OFFSET_W::new(self)
    }
    #[doc = "Bits 20:21 - u0_plda_pcie_phy_mode"]
    #[inline(always)]
    #[must_use]
    pub fn u0_plda_pcie_phy_mode(
        &mut self,
    ) -> U0_PLDA_PCIE_PHY_MODE_W<STG_SYSCONSAIF_SYSCFG328_SPEC, 20> {
        U0_PLDA_PCIE_PHY_MODE_W::new(self)
    }
    #[doc = "Bit 22 - u0_plda_pcie_pl_clkrem_allow"]
    #[inline(always)]
    #[must_use]
    pub fn u0_plda_pcie_pl_clkrem_allow(
        &mut self,
    ) -> U0_PLDA_PCIE_PL_CLKREM_ALLOW_W<STG_SYSCONSAIF_SYSCFG328_SPEC, 22> {
        U0_PLDA_PCIE_PL_CLKREM_ALLOW_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.bits = bits;
        self
    }
}
#[doc = "STG SYSCONSAIF SYSCFG 328\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`stg_sysconsaif_syscfg328::R`](R).  You can [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`stg_sysconsaif_syscfg328::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct STG_SYSCONSAIF_SYSCFG328_SPEC;
impl crate::RegisterSpec for STG_SYSCONSAIF_SYSCFG328_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`stg_sysconsaif_syscfg328::R`](R) reader structure"]
impl crate::Readable for STG_SYSCONSAIF_SYSCFG328_SPEC {}
#[doc = "`write(|w| ..)` method takes [`stg_sysconsaif_syscfg328::W`](W) writer structure"]
impl crate::Writable for STG_SYSCONSAIF_SYSCFG328_SPEC {
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
