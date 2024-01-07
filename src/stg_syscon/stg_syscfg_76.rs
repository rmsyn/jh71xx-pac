#[doc = "Register `stg_syscfg_76` reader"]
pub type R = crate::R<STG_SYSCFG_76_SPEC>;
#[doc = "Register `stg_syscfg_76` writer"]
pub type W = crate::W<STG_SYSCFG_76_SPEC>;
#[doc = "Field `u0_pcie_k_phyparam_839_832` reader - u0_pcie_k_phyparam_839_832"]
pub type U0_PCIE_K_PHYPARAM_839_832_R = crate::FieldReader;
#[doc = "Field `u0_pcie_k_phyparam_839_832` writer - u0_pcie_k_phyparam_839_832"]
pub type U0_PCIE_K_PHYPARAM_839_832_W<'a, REG> = crate::FieldWriter<'a, REG, 8>;
#[doc = "Field `u0_pcie_k_rp_nep` reader - u0_pcie_k_rp_nep"]
pub type U0_PCIE_K_RP_NEP_R = crate::BitReader;
#[doc = "Field `u0_pcie_k_rp_nep` writer - u0_pcie_k_rp_nep"]
pub type U0_PCIE_K_RP_NEP_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `u0_pcie_l1sub_entack` reader - u0_pcie_l1sub_entack"]
pub type U0_PCIE_L1SUB_ENTACK_R = crate::BitReader;
#[doc = "Field `u0_pcie_l1sub_entreq` reader - u0_pcie_l1sub_entreq"]
pub type U0_PCIE_L1SUB_ENTREQ_R = crate::BitReader;
#[doc = "Field `u0_pcie_l1sub_entreq` writer - u0_pcie_l1sub_entreq"]
pub type U0_PCIE_L1SUB_ENTREQ_W<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bits 0:7 - u0_pcie_k_phyparam_839_832"]
    #[inline(always)]
    pub fn u0_pcie_k_phyparam_839_832(&self) -> U0_PCIE_K_PHYPARAM_839_832_R {
        U0_PCIE_K_PHYPARAM_839_832_R::new((self.bits & 0xff) as u8)
    }
    #[doc = "Bit 8 - u0_pcie_k_rp_nep"]
    #[inline(always)]
    pub fn u0_pcie_k_rp_nep(&self) -> U0_PCIE_K_RP_NEP_R {
        U0_PCIE_K_RP_NEP_R::new(((self.bits >> 8) & 1) != 0)
    }
    #[doc = "Bit 9 - u0_pcie_l1sub_entack"]
    #[inline(always)]
    pub fn u0_pcie_l1sub_entack(&self) -> U0_PCIE_L1SUB_ENTACK_R {
        U0_PCIE_L1SUB_ENTACK_R::new(((self.bits >> 9) & 1) != 0)
    }
    #[doc = "Bit 10 - u0_pcie_l1sub_entreq"]
    #[inline(always)]
    pub fn u0_pcie_l1sub_entreq(&self) -> U0_PCIE_L1SUB_ENTREQ_R {
        U0_PCIE_L1SUB_ENTREQ_R::new(((self.bits >> 10) & 1) != 0)
    }
}
impl W {
    #[doc = "Bits 0:7 - u0_pcie_k_phyparam_839_832"]
    #[inline(always)]
    #[must_use]
    pub fn u0_pcie_k_phyparam_839_832(
        &mut self,
    ) -> U0_PCIE_K_PHYPARAM_839_832_W<STG_SYSCFG_76_SPEC> {
        U0_PCIE_K_PHYPARAM_839_832_W::new(self, 0)
    }
    #[doc = "Bit 8 - u0_pcie_k_rp_nep"]
    #[inline(always)]
    #[must_use]
    pub fn u0_pcie_k_rp_nep(&mut self) -> U0_PCIE_K_RP_NEP_W<STG_SYSCFG_76_SPEC> {
        U0_PCIE_K_RP_NEP_W::new(self, 8)
    }
    #[doc = "Bit 10 - u0_pcie_l1sub_entreq"]
    #[inline(always)]
    #[must_use]
    pub fn u0_pcie_l1sub_entreq(&mut self) -> U0_PCIE_L1SUB_ENTREQ_W<STG_SYSCFG_76_SPEC> {
        U0_PCIE_L1SUB_ENTREQ_W::new(self, 10)
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
#[doc = "STG SYSCONSAIF SYSCFG 304\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`stg_syscfg_76::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`stg_syscfg_76::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct STG_SYSCFG_76_SPEC;
impl crate::RegisterSpec for STG_SYSCFG_76_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`stg_syscfg_76::R`](R) reader structure"]
impl crate::Readable for STG_SYSCFG_76_SPEC {}
#[doc = "`write(|w| ..)` method takes [`stg_syscfg_76::W`](W) writer structure"]
impl crate::Writable for STG_SYSCFG_76_SPEC {
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets stg_syscfg_76 to value 0"]
impl crate::Resettable for STG_SYSCFG_76_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
