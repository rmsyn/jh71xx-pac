#[doc = "Register `aon_syscfg_10` reader"]
pub type R = crate::R<AON_SYSCFG_10_SPEC>;
#[doc = "Register `aon_syscfg_10` writer"]
pub type W = crate::W<AON_SYSCFG_10_SPEC>;
#[doc = "Field `u0_otpc_fl_sec_boot_lmt` reader - u0_otpc_fl_sec_boot_lmt"]
pub type U0_OTPC_FL_SEC_BOOT_LMT_R = crate::BitReader;
#[doc = "Field `u0_otpc_fl_xip` reader - u0_otpc_fl_xip"]
pub type U0_OTPC_FL_XIP_R = crate::BitReader;
#[doc = "Field `u0_otpc_load_busy` reader - u0_otpc_load_busy"]
pub type U0_OTPC_LOAD_BUSY_R = crate::BitReader;
#[doc = "Field `u0_reset_ctrl_clr_reset_status` reader - u0_reset_ctrl_clr_reset_status"]
pub type U0_RESET_CTRL_CLR_RESET_STATUS_R = crate::BitReader;
#[doc = "Field `u0_reset_ctrl_clr_reset_status` writer - u0_reset_ctrl_clr_reset_status"]
pub type U0_RESET_CTRL_CLR_RESET_STATUS_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `u0_reset_ctrl_pll_timecnt_finish` reader - u0_reset_ctrl_pll_timecnt_finish"]
pub type U0_RESET_CTRL_PLL_TIMECNT_FINISH_R = crate::BitReader;
#[doc = "Field `u0_reset_ctrl_rstn_sw` reader - u0_reset_ctrl_rstn_sw"]
pub type U0_RESET_CTRL_RSTN_SW_R = crate::BitReader;
#[doc = "Field `u0_reset_ctrl_rstn_sw` writer - u0_reset_ctrl_rstn_sw"]
pub type U0_RESET_CTRL_RSTN_SW_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `u0_reset_ctrl_sys_reset_status` reader - u0_reset_ctrl_sys_reset_status"]
pub type U0_RESET_CTRL_SYS_RESET_STATUS_R = crate::FieldReader;
impl R {
    #[doc = "Bit 0 - u0_otpc_fl_sec_boot_lmt"]
    #[inline(always)]
    pub fn u0_otpc_fl_sec_boot_lmt(&self) -> U0_OTPC_FL_SEC_BOOT_LMT_R {
        U0_OTPC_FL_SEC_BOOT_LMT_R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - u0_otpc_fl_xip"]
    #[inline(always)]
    pub fn u0_otpc_fl_xip(&self) -> U0_OTPC_FL_XIP_R {
        U0_OTPC_FL_XIP_R::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - u0_otpc_load_busy"]
    #[inline(always)]
    pub fn u0_otpc_load_busy(&self) -> U0_OTPC_LOAD_BUSY_R {
        U0_OTPC_LOAD_BUSY_R::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3 - u0_reset_ctrl_clr_reset_status"]
    #[inline(always)]
    pub fn u0_reset_ctrl_clr_reset_status(&self) -> U0_RESET_CTRL_CLR_RESET_STATUS_R {
        U0_RESET_CTRL_CLR_RESET_STATUS_R::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bit 4 - u0_reset_ctrl_pll_timecnt_finish"]
    #[inline(always)]
    pub fn u0_reset_ctrl_pll_timecnt_finish(&self) -> U0_RESET_CTRL_PLL_TIMECNT_FINISH_R {
        U0_RESET_CTRL_PLL_TIMECNT_FINISH_R::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bit 5 - u0_reset_ctrl_rstn_sw"]
    #[inline(always)]
    pub fn u0_reset_ctrl_rstn_sw(&self) -> U0_RESET_CTRL_RSTN_SW_R {
        U0_RESET_CTRL_RSTN_SW_R::new(((self.bits >> 5) & 1) != 0)
    }
    #[doc = "Bits 6:9 - u0_reset_ctrl_sys_reset_status"]
    #[inline(always)]
    pub fn u0_reset_ctrl_sys_reset_status(&self) -> U0_RESET_CTRL_SYS_RESET_STATUS_R {
        U0_RESET_CTRL_SYS_RESET_STATUS_R::new(((self.bits >> 6) & 0x0f) as u8)
    }
}
impl W {
    #[doc = "Bit 3 - u0_reset_ctrl_clr_reset_status"]
    #[inline(always)]
    #[must_use]
    pub fn u0_reset_ctrl_clr_reset_status(
        &mut self,
    ) -> U0_RESET_CTRL_CLR_RESET_STATUS_W<AON_SYSCFG_10_SPEC> {
        U0_RESET_CTRL_CLR_RESET_STATUS_W::new(self, 3)
    }
    #[doc = "Bit 5 - u0_reset_ctrl_rstn_sw"]
    #[inline(always)]
    #[must_use]
    pub fn u0_reset_ctrl_rstn_sw(&mut self) -> U0_RESET_CTRL_RSTN_SW_W<AON_SYSCFG_10_SPEC> {
        U0_RESET_CTRL_RSTN_SW_W::new(self, 5)
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
#[doc = "AON SYSCONSAIF SYSCFG 40\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`aon_syscfg_10::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`aon_syscfg_10::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct AON_SYSCFG_10_SPEC;
impl crate::RegisterSpec for AON_SYSCFG_10_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`aon_syscfg_10::R`](R) reader structure"]
impl crate::Readable for AON_SYSCFG_10_SPEC {}
#[doc = "`write(|w| ..)` method takes [`aon_syscfg_10::W`](W) writer structure"]
impl crate::Writable for AON_SYSCFG_10_SPEC {
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets aon_syscfg_10 to value 0"]
impl crate::Resettable for AON_SYSCFG_10_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
