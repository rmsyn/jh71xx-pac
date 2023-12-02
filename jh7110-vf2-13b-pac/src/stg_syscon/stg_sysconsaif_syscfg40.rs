#[doc = "Register `stg_sysconsaif_syscfg40` reader"]
pub type R = crate::R<STG_SYSCONSAIF_SYSCFG40_SPEC>;
#[doc = "Register `stg_sysconsaif_syscfg40` writer"]
pub type W = crate::W<STG_SYSCONSAIF_SYSCFG40_SPEC>;
#[doc = "Field `u0_e2_sft7110_wfi_from_tile_0` reader - u0_e2_sft7110_wfi_from_tile_0"]
pub type U0_E2_SFT7110_WFI_FROM_TILE_0_R = crate::BitReader;
impl R {
    #[doc = "Bit 0 - u0_e2_sft7110_wfi_from_tile_0"]
    #[inline(always)]
    pub fn u0_e2_sft7110_wfi_from_tile_0(&self) -> U0_E2_SFT7110_WFI_FROM_TILE_0_R {
        U0_E2_SFT7110_WFI_FROM_TILE_0_R::new((self.bits & 1) != 0)
    }
}
impl W {
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
#[doc = "STG SYSCONSAIF SYSCFG 40\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`stg_sysconsaif_syscfg40::R`](R).  You can [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`stg_sysconsaif_syscfg40::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct STG_SYSCONSAIF_SYSCFG40_SPEC;
impl crate::RegisterSpec for STG_SYSCONSAIF_SYSCFG40_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`stg_sysconsaif_syscfg40::R`](R) reader structure"]
impl crate::Readable for STG_SYSCONSAIF_SYSCFG40_SPEC {}
#[doc = "`write(|w| ..)` method takes [`stg_sysconsaif_syscfg40::W`](W) writer structure"]
impl crate::Writable for STG_SYSCONSAIF_SYSCFG40_SPEC {
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
