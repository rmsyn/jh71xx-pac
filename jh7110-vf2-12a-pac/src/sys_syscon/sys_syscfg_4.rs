#[doc = "Register `sys_syscfg_4` reader"]
pub type R = crate::R<SYS_SYSCFG_4_SPEC>;
#[doc = "Register `sys_syscfg_4` writer"]
pub type W = crate::W<SYS_SYSCFG_4_SPEC>;
#[doc = "Field `u0_coda12_o_cur_inst_a` reader - Tie 0 in JPU internal, do not care"]
pub type U0_CODA12_O_CUR_INST_A_R = crate::FieldReader;
#[doc = "Field `u0_wave511_o_vpu_idle` reader - VPU monitoring signal"]
pub type U0_WAVE511_O_VPU_IDLE_R = crate::BitReader;
#[doc = "Field `u0_can_ctrl_can_fd_enable` reader - u0_can_ctrl_can_fd_enable"]
pub type U0_CAN_CTRL_CAN_FD_ENABLE_R = crate::BitReader;
#[doc = "Field `u0_can_ctrl_can_fd_enable` writer - u0_can_ctrl_can_fd_enable"]
pub type U0_CAN_CTRL_CAN_FD_ENABLE_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `u0_can_ctrl_host_ecc_disable` reader - u0_can_ctrl_host_ecc_disable"]
pub type U0_CAN_CTRL_HOST_ECC_DISABLE_R = crate::BitReader;
#[doc = "Field `u0_can_ctrl_host_ecc_disable` writer - u0_can_ctrl_host_ecc_disable"]
pub type U0_CAN_CTRL_HOST_ECC_DISABLE_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `u0_can_ctrl_host_if` reader - u0_can_ctrl_host_if"]
pub type U0_CAN_CTRL_HOST_IF_R = crate::FieldReader<u32>;
#[doc = "Field `u0_cdns_qspi_scfg_qspi_sclk_dlychain_sel` reader - des_qspi_sclk_dla: clock delay"]
pub type U0_CDNS_QSPI_SCFG_QSPI_SCLK_DLYCHAIN_SEL_R = crate::FieldReader;
impl R {
    #[doc = "Bits 0:1 - Tie 0 in JPU internal, do not care"]
    #[inline(always)]
    pub fn u0_coda12_o_cur_inst_a(&self) -> U0_CODA12_O_CUR_INST_A_R {
        U0_CODA12_O_CUR_INST_A_R::new((self.bits & 3) as u8)
    }
    #[doc = "Bit 2 - VPU monitoring signal"]
    #[inline(always)]
    pub fn u0_wave511_o_vpu_idle(&self) -> U0_WAVE511_O_VPU_IDLE_R {
        U0_WAVE511_O_VPU_IDLE_R::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3 - u0_can_ctrl_can_fd_enable"]
    #[inline(always)]
    pub fn u0_can_ctrl_can_fd_enable(&self) -> U0_CAN_CTRL_CAN_FD_ENABLE_R {
        U0_CAN_CTRL_CAN_FD_ENABLE_R::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bit 4 - u0_can_ctrl_host_ecc_disable"]
    #[inline(always)]
    pub fn u0_can_ctrl_host_ecc_disable(&self) -> U0_CAN_CTRL_HOST_ECC_DISABLE_R {
        U0_CAN_CTRL_HOST_ECC_DISABLE_R::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bits 5:23 - u0_can_ctrl_host_if"]
    #[inline(always)]
    pub fn u0_can_ctrl_host_if(&self) -> U0_CAN_CTRL_HOST_IF_R {
        U0_CAN_CTRL_HOST_IF_R::new((self.bits >> 5) & 0x0007_ffff)
    }
    #[doc = "Bits 24:28 - des_qspi_sclk_dla: clock delay"]
    #[inline(always)]
    pub fn u0_cdns_qspi_scfg_qspi_sclk_dlychain_sel(
        &self,
    ) -> U0_CDNS_QSPI_SCFG_QSPI_SCLK_DLYCHAIN_SEL_R {
        U0_CDNS_QSPI_SCFG_QSPI_SCLK_DLYCHAIN_SEL_R::new(((self.bits >> 24) & 0x1f) as u8)
    }
}
impl W {
    #[doc = "Bit 3 - u0_can_ctrl_can_fd_enable"]
    #[inline(always)]
    #[must_use]
    pub fn u0_can_ctrl_can_fd_enable(&mut self) -> U0_CAN_CTRL_CAN_FD_ENABLE_W<SYS_SYSCFG_4_SPEC> {
        U0_CAN_CTRL_CAN_FD_ENABLE_W::new(self, 3)
    }
    #[doc = "Bit 4 - u0_can_ctrl_host_ecc_disable"]
    #[inline(always)]
    #[must_use]
    pub fn u0_can_ctrl_host_ecc_disable(
        &mut self,
    ) -> U0_CAN_CTRL_HOST_ECC_DISABLE_W<SYS_SYSCFG_4_SPEC> {
        U0_CAN_CTRL_HOST_ECC_DISABLE_W::new(self, 4)
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
#[doc = "SYS SYSCONSAIF SYSCFG 16\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`sys_syscfg_4::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`sys_syscfg_4::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct SYS_SYSCFG_4_SPEC;
impl crate::RegisterSpec for SYS_SYSCFG_4_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`sys_syscfg_4::R`](R) reader structure"]
impl crate::Readable for SYS_SYSCFG_4_SPEC {}
#[doc = "`write(|w| ..)` method takes [`sys_syscfg_4::W`](W) writer structure"]
impl crate::Writable for SYS_SYSCFG_4_SPEC {
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets sys_syscfg_4 to value 0"]
impl crate::Resettable for SYS_SYSCFG_4_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
