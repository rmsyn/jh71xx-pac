#[doc = "Register `stg_syscfg_82` reader"]
pub type R = crate::R<STG_SYSCFG_82_SPEC>;
#[doc = "Register `stg_syscfg_82` writer"]
pub type W = crate::W<STG_SYSCFG_82_SPEC>;
#[doc = "Field `u0_pcie_pf3_offset` reader - u0_pcie_pf3_offset"]
pub type U0_PCIE_PF3_OFFSET_R = crate::FieldReader<u32>;
#[doc = "Field `u0_pcie_pf3_offset` writer - u0_pcie_pf3_offset"]
pub type U0_PCIE_PF3_OFFSET_W<'a, REG> = crate::FieldWriter<'a, REG, 20, u32>;
#[doc = "Field `u0_pcie_phy_mode` reader - u0_pcie_phy_mode"]
pub type U0_PCIE_PHY_MODE_R = crate::FieldReader;
#[doc = "Field `u0_pcie_phy_mode` writer - u0_pcie_phy_mode"]
pub type U0_PCIE_PHY_MODE_W<'a, REG> = crate::FieldWriter<'a, REG, 2>;
#[doc = "Field `u0_pcie_pl_clkrem_allow` reader - u0_pcie_pl_clkrem_allow"]
pub type U0_PCIE_PL_CLKREM_ALLOW_R = crate::BitReader;
#[doc = "Field `u0_pcie_pl_clkrem_allow` writer - u0_pcie_pl_clkrem_allow"]
pub type U0_PCIE_PL_CLKREM_ALLOW_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `u0_pcie_pl_clkreq_oen` reader - u0_pcie_pl_clkreq_oen"]
pub type U0_PCIE_PL_CLKREQ_OEN_R = crate::BitReader;
#[doc = "Field `u0_pcie_pl_equ_phase` reader - u0_pcie_pl_equ_phase"]
pub type U0_PCIE_PL_EQU_PHASE_R = crate::FieldReader;
#[doc = "Field `u0_pcie_pl_ltssm` reader - u0_pcie_pl_ltssm"]
pub type U0_PCIE_PL_LTSSM_R = crate::FieldReader;
impl R {
    #[doc = "Bits 0:19 - u0_pcie_pf3_offset"]
    #[inline(always)]
    pub fn u0_pcie_pf3_offset(&self) -> U0_PCIE_PF3_OFFSET_R {
        U0_PCIE_PF3_OFFSET_R::new(self.bits & 0x000f_ffff)
    }
    #[doc = "Bits 20:21 - u0_pcie_phy_mode"]
    #[inline(always)]
    pub fn u0_pcie_phy_mode(&self) -> U0_PCIE_PHY_MODE_R {
        U0_PCIE_PHY_MODE_R::new(((self.bits >> 20) & 3) as u8)
    }
    #[doc = "Bit 22 - u0_pcie_pl_clkrem_allow"]
    #[inline(always)]
    pub fn u0_pcie_pl_clkrem_allow(&self) -> U0_PCIE_PL_CLKREM_ALLOW_R {
        U0_PCIE_PL_CLKREM_ALLOW_R::new(((self.bits >> 22) & 1) != 0)
    }
    #[doc = "Bit 23 - u0_pcie_pl_clkreq_oen"]
    #[inline(always)]
    pub fn u0_pcie_pl_clkreq_oen(&self) -> U0_PCIE_PL_CLKREQ_OEN_R {
        U0_PCIE_PL_CLKREQ_OEN_R::new(((self.bits >> 23) & 1) != 0)
    }
    #[doc = "Bits 24:25 - u0_pcie_pl_equ_phase"]
    #[inline(always)]
    pub fn u0_pcie_pl_equ_phase(&self) -> U0_PCIE_PL_EQU_PHASE_R {
        U0_PCIE_PL_EQU_PHASE_R::new(((self.bits >> 24) & 3) as u8)
    }
    #[doc = "Bits 26:30 - u0_pcie_pl_ltssm"]
    #[inline(always)]
    pub fn u0_pcie_pl_ltssm(&self) -> U0_PCIE_PL_LTSSM_R {
        U0_PCIE_PL_LTSSM_R::new(((self.bits >> 26) & 0x1f) as u8)
    }
}
impl W {
    #[doc = "Bits 0:19 - u0_pcie_pf3_offset"]
    #[inline(always)]
    #[must_use]
    pub fn u0_pcie_pf3_offset(&mut self) -> U0_PCIE_PF3_OFFSET_W<STG_SYSCFG_82_SPEC> {
        U0_PCIE_PF3_OFFSET_W::new(self, 0)
    }
    #[doc = "Bits 20:21 - u0_pcie_phy_mode"]
    #[inline(always)]
    #[must_use]
    pub fn u0_pcie_phy_mode(&mut self) -> U0_PCIE_PHY_MODE_W<STG_SYSCFG_82_SPEC> {
        U0_PCIE_PHY_MODE_W::new(self, 20)
    }
    #[doc = "Bit 22 - u0_pcie_pl_clkrem_allow"]
    #[inline(always)]
    #[must_use]
    pub fn u0_pcie_pl_clkrem_allow(&mut self) -> U0_PCIE_PL_CLKREM_ALLOW_W<STG_SYSCFG_82_SPEC> {
        U0_PCIE_PL_CLKREM_ALLOW_W::new(self, 22)
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
#[doc = "STG SYSCONSAIF SYSCFG 328\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`stg_syscfg_82::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`stg_syscfg_82::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct STG_SYSCFG_82_SPEC;
impl crate::RegisterSpec for STG_SYSCFG_82_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`stg_syscfg_82::R`](R) reader structure"]
impl crate::Readable for STG_SYSCFG_82_SPEC {}
#[doc = "`write(|w| ..)` method takes [`stg_syscfg_82::W`](W) writer structure"]
impl crate::Writable for STG_SYSCFG_82_SPEC {
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets stg_syscfg_82 to value 0"]
impl crate::Resettable for STG_SYSCFG_82_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
