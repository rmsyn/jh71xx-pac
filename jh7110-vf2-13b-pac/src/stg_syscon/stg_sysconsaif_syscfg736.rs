#[doc = "Register `stg_sysconsaif_syscfg736` reader"]
pub type R = crate::R<STG_SYSCONSAIF_SYSCFG736_SPEC>;
#[doc = "Register `stg_sysconsaif_syscfg736` writer"]
pub type W = crate::W<STG_SYSCONSAIF_SYSCFG736_SPEC>;
#[doc = "Field `u1_plda_pcie_k_phyparam_839_832` reader - u1_plda_pcie_k_phyparam_839_832"]
pub type U1_PLDA_PCIE_K_PHYPARAM_839_832_R = crate::FieldReader;
#[doc = "Field `u1_plda_pcie_k_phyparam_839_832` writer - u1_plda_pcie_k_phyparam_839_832"]
pub type U1_PLDA_PCIE_K_PHYPARAM_839_832_W<'a, REG> = crate::FieldWriter<'a, REG, 8>;
#[doc = "Field `u1_plda_pcie_k_rp_nep` reader - u1_plda_pcie_k_rp_nep"]
pub type U1_PLDA_PCIE_K_RP_NEP_R = crate::BitReader;
#[doc = "Field `u1_plda_pcie_k_rp_nep` writer - u1_plda_pcie_k_rp_nep"]
pub type U1_PLDA_PCIE_K_RP_NEP_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `u1_plda_pcie_l1sub_entack` reader - u1_plda_pcie_l1sub_entack"]
pub type U1_PLDA_PCIE_L1SUB_ENTACK_R = crate::BitReader;
#[doc = "Field `u1_plda_pcie_l1sub_entreq` reader - u1_plda_pcie_l1sub_entreq"]
pub type U1_PLDA_PCIE_L1SUB_ENTREQ_R = crate::BitReader;
#[doc = "Field `u1_plda_pcie_l1sub_entreq` writer - u1_plda_pcie_l1sub_entreq"]
pub type U1_PLDA_PCIE_L1SUB_ENTREQ_W<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bits 0:7 - u1_plda_pcie_k_phyparam_839_832"]
    #[inline(always)]
    pub fn u1_plda_pcie_k_phyparam_839_832(&self) -> U1_PLDA_PCIE_K_PHYPARAM_839_832_R {
        U1_PLDA_PCIE_K_PHYPARAM_839_832_R::new((self.bits & 0xff) as u8)
    }
    #[doc = "Bit 8 - u1_plda_pcie_k_rp_nep"]
    #[inline(always)]
    pub fn u1_plda_pcie_k_rp_nep(&self) -> U1_PLDA_PCIE_K_RP_NEP_R {
        U1_PLDA_PCIE_K_RP_NEP_R::new(((self.bits >> 8) & 1) != 0)
    }
    #[doc = "Bit 9 - u1_plda_pcie_l1sub_entack"]
    #[inline(always)]
    pub fn u1_plda_pcie_l1sub_entack(&self) -> U1_PLDA_PCIE_L1SUB_ENTACK_R {
        U1_PLDA_PCIE_L1SUB_ENTACK_R::new(((self.bits >> 9) & 1) != 0)
    }
    #[doc = "Bit 10 - u1_plda_pcie_l1sub_entreq"]
    #[inline(always)]
    pub fn u1_plda_pcie_l1sub_entreq(&self) -> U1_PLDA_PCIE_L1SUB_ENTREQ_R {
        U1_PLDA_PCIE_L1SUB_ENTREQ_R::new(((self.bits >> 10) & 1) != 0)
    }
}
impl W {
    #[doc = "Bits 0:7 - u1_plda_pcie_k_phyparam_839_832"]
    #[inline(always)]
    #[must_use]
    pub fn u1_plda_pcie_k_phyparam_839_832(
        &mut self,
    ) -> U1_PLDA_PCIE_K_PHYPARAM_839_832_W<STG_SYSCONSAIF_SYSCFG736_SPEC> {
        U1_PLDA_PCIE_K_PHYPARAM_839_832_W::new(self, 0)
    }
    #[doc = "Bit 8 - u1_plda_pcie_k_rp_nep"]
    #[inline(always)]
    #[must_use]
    pub fn u1_plda_pcie_k_rp_nep(
        &mut self,
    ) -> U1_PLDA_PCIE_K_RP_NEP_W<STG_SYSCONSAIF_SYSCFG736_SPEC> {
        U1_PLDA_PCIE_K_RP_NEP_W::new(self, 8)
    }
    #[doc = "Bit 10 - u1_plda_pcie_l1sub_entreq"]
    #[inline(always)]
    #[must_use]
    pub fn u1_plda_pcie_l1sub_entreq(
        &mut self,
    ) -> U1_PLDA_PCIE_L1SUB_ENTREQ_W<STG_SYSCONSAIF_SYSCFG736_SPEC> {
        U1_PLDA_PCIE_L1SUB_ENTREQ_W::new(self, 10)
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
#[doc = "STG SYSCONSAIF SYSCFG 736\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`stg_sysconsaif_syscfg736::R`](R).  You can [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`stg_sysconsaif_syscfg736::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct STG_SYSCONSAIF_SYSCFG736_SPEC;
impl crate::RegisterSpec for STG_SYSCONSAIF_SYSCFG736_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`stg_sysconsaif_syscfg736::R`](R) reader structure"]
impl crate::Readable for STG_SYSCONSAIF_SYSCFG736_SPEC {}
#[doc = "`write(|w| ..)` method takes [`stg_sysconsaif_syscfg736::W`](W) writer structure"]
impl crate::Writable for STG_SYSCONSAIF_SYSCFG736_SPEC {
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
