#[doc = "Register `aon_syscfg_6` reader"]
pub type R = crate::R<AON_SYSCFG_6_SPEC>;
#[doc = "Register `aon_syscfg_6` writer"]
pub type W = crate::W<AON_SYSCFG_6_SPEC>;
#[doc = "Field `u0_otpc_chip_mode` reader - u0_otpc_chip_mode"]
pub type U0_OTPC_CHIP_MODE_R = crate::BitReader;
#[doc = "Field `u0_otpc_crc_pass` reader - u0_otpc_crc_pass"]
pub type U0_OTPC_CRC_PASS_R = crate::BitReader;
#[doc = "Field `u0_otpc_dbg_enable` reader - u0_otpc_dbg_enable"]
pub type U0_OTPC_DBG_ENABLE_R = crate::BitReader;
impl R {
    #[doc = "Bit 0 - u0_otpc_chip_mode"]
    #[inline(always)]
    pub fn u0_otpc_chip_mode(&self) -> U0_OTPC_CHIP_MODE_R {
        U0_OTPC_CHIP_MODE_R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - u0_otpc_crc_pass"]
    #[inline(always)]
    pub fn u0_otpc_crc_pass(&self) -> U0_OTPC_CRC_PASS_R {
        U0_OTPC_CRC_PASS_R::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - u0_otpc_dbg_enable"]
    #[inline(always)]
    pub fn u0_otpc_dbg_enable(&self) -> U0_OTPC_DBG_ENABLE_R {
        U0_OTPC_DBG_ENABLE_R::new(((self.bits >> 2) & 1) != 0)
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
#[doc = "AON SYSCONSAIF SYSCFG 24\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`aon_syscfg_6::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`aon_syscfg_6::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct AON_SYSCFG_6_SPEC;
impl crate::RegisterSpec for AON_SYSCFG_6_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`aon_syscfg_6::R`](R) reader structure"]
impl crate::Readable for AON_SYSCFG_6_SPEC {}
#[doc = "`write(|w| ..)` method takes [`aon_syscfg_6::W`](W) writer structure"]
impl crate::Writable for AON_SYSCFG_6_SPEC {
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets aon_syscfg_6 to value 0"]
impl crate::Resettable for AON_SYSCFG_6_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
