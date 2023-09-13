#[doc = "Register `stg_sysconsaif_syscfg304` reader"]
pub type R = crate::R<STG_SYSCONSAIF_SYSCFG304_SPEC>;
#[doc = "Register `stg_sysconsaif_syscfg304` writer"]
pub type W = crate::W<STG_SYSCONSAIF_SYSCFG304_SPEC>;
#[doc = "Field `u0_plda_pcie_k_phyparam_839_832` reader - u0_plda_pcie_k_phyparam_839_832"]
pub type U0_PLDA_PCIE_K_PHYPARAM_839_832_R = crate::FieldReader;
#[doc = "Field `u0_plda_pcie_k_phyparam_839_832` writer - u0_plda_pcie_k_phyparam_839_832"]
pub type U0_PLDA_PCIE_K_PHYPARAM_839_832_W<'a, REG, const O: u8> =
    crate::FieldWriter<'a, REG, 8, O>;
#[doc = "Field `u0_plda_pcie_k_rp_nep` reader - u0_plda_pcie_k_rp_nep"]
pub type U0_PLDA_PCIE_K_RP_NEP_R = crate::BitReader;
#[doc = "Field `u0_plda_pcie_k_rp_nep` writer - u0_plda_pcie_k_rp_nep"]
pub type U0_PLDA_PCIE_K_RP_NEP_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O>;
#[doc = "Field `u0_plda_pcie_l1sub_entack` reader - u0_plda_pcie_l1sub_entack"]
pub type U0_PLDA_PCIE_L1SUB_ENTACK_R = crate::BitReader;
#[doc = "Field `u0_plda_pcie_l1sub_entreq` reader - u0_plda_pcie_l1sub_entreq"]
pub type U0_PLDA_PCIE_L1SUB_ENTREQ_R = crate::BitReader;
#[doc = "Field `u0_plda_pcie_l1sub_entreq` writer - u0_plda_pcie_l1sub_entreq"]
pub type U0_PLDA_PCIE_L1SUB_ENTREQ_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O>;
impl R {
    #[doc = "Bits 0:7 - u0_plda_pcie_k_phyparam_839_832"]
    #[inline(always)]
    pub fn u0_plda_pcie_k_phyparam_839_832(&self) -> U0_PLDA_PCIE_K_PHYPARAM_839_832_R {
        U0_PLDA_PCIE_K_PHYPARAM_839_832_R::new((self.bits & 0xff) as u8)
    }
    #[doc = "Bit 8 - u0_plda_pcie_k_rp_nep"]
    #[inline(always)]
    pub fn u0_plda_pcie_k_rp_nep(&self) -> U0_PLDA_PCIE_K_RP_NEP_R {
        U0_PLDA_PCIE_K_RP_NEP_R::new(((self.bits >> 8) & 1) != 0)
    }
    #[doc = "Bit 9 - u0_plda_pcie_l1sub_entack"]
    #[inline(always)]
    pub fn u0_plda_pcie_l1sub_entack(&self) -> U0_PLDA_PCIE_L1SUB_ENTACK_R {
        U0_PLDA_PCIE_L1SUB_ENTACK_R::new(((self.bits >> 9) & 1) != 0)
    }
    #[doc = "Bit 10 - u0_plda_pcie_l1sub_entreq"]
    #[inline(always)]
    pub fn u0_plda_pcie_l1sub_entreq(&self) -> U0_PLDA_PCIE_L1SUB_ENTREQ_R {
        U0_PLDA_PCIE_L1SUB_ENTREQ_R::new(((self.bits >> 10) & 1) != 0)
    }
}
impl W {
    #[doc = "Bits 0:7 - u0_plda_pcie_k_phyparam_839_832"]
    #[inline(always)]
    #[must_use]
    pub fn u0_plda_pcie_k_phyparam_839_832(
        &mut self,
    ) -> U0_PLDA_PCIE_K_PHYPARAM_839_832_W<STG_SYSCONSAIF_SYSCFG304_SPEC, 0> {
        U0_PLDA_PCIE_K_PHYPARAM_839_832_W::new(self)
    }
    #[doc = "Bit 8 - u0_plda_pcie_k_rp_nep"]
    #[inline(always)]
    #[must_use]
    pub fn u0_plda_pcie_k_rp_nep(
        &mut self,
    ) -> U0_PLDA_PCIE_K_RP_NEP_W<STG_SYSCONSAIF_SYSCFG304_SPEC, 8> {
        U0_PLDA_PCIE_K_RP_NEP_W::new(self)
    }
    #[doc = "Bit 10 - u0_plda_pcie_l1sub_entreq"]
    #[inline(always)]
    #[must_use]
    pub fn u0_plda_pcie_l1sub_entreq(
        &mut self,
    ) -> U0_PLDA_PCIE_L1SUB_ENTREQ_W<STG_SYSCONSAIF_SYSCFG304_SPEC, 10> {
        U0_PLDA_PCIE_L1SUB_ENTREQ_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.bits = bits;
        self
    }
}
#[doc = "STG SYSCONSAIF SYSCFG 304\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`stg_sysconsaif_syscfg304::R`](R).  You can [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`stg_sysconsaif_syscfg304::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct STG_SYSCONSAIF_SYSCFG304_SPEC;
impl crate::RegisterSpec for STG_SYSCONSAIF_SYSCFG304_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`stg_sysconsaif_syscfg304::R`](R) reader structure"]
impl crate::Readable for STG_SYSCONSAIF_SYSCFG304_SPEC {}
#[doc = "`write(|w| ..)` method takes [`stg_sysconsaif_syscfg304::W`](W) writer structure"]
impl crate::Writable for STG_SYSCONSAIF_SYSCFG304_SPEC {
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
