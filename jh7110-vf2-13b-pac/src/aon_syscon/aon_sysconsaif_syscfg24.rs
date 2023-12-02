#[doc = "Register `aon_sysconsaif_syscfg24` reader"]
pub type R = crate::R<AON_SYSCONSAIF_SYSCFG24_SPEC>;
#[doc = "Register `aon_sysconsaif_syscfg24` writer"]
pub type W = crate::W<AON_SYSCONSAIF_SYSCFG24_SPEC>;
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
#[doc = "AON SYSCONSAIF SYSCFG 24\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`aon_sysconsaif_syscfg24::R`](R).  You can [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`aon_sysconsaif_syscfg24::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct AON_SYSCONSAIF_SYSCFG24_SPEC;
impl crate::RegisterSpec for AON_SYSCONSAIF_SYSCFG24_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`aon_sysconsaif_syscfg24::R`](R) reader structure"]
impl crate::Readable for AON_SYSCONSAIF_SYSCFG24_SPEC {}
#[doc = "`write(|w| ..)` method takes [`aon_sysconsaif_syscfg24::W`](W) writer structure"]
impl crate::Writable for AON_SYSCONSAIF_SYSCFG24_SPEC {
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
