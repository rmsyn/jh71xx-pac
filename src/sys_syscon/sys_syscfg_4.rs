#[doc = "Register `sys_syscfg_4` reader"]
pub type R = crate::R<SYS_SYSCFG_4_SPEC>;
#[doc = "Register `sys_syscfg_4` writer"]
pub type W = crate::W<SYS_SYSCFG_4_SPEC>;
#[doc = "Field `coda12_cur_inst` reader - Tie 0 in JPU internal, do not care"]
pub type CODA12_CUR_INST_R = crate::FieldReader;
#[doc = "Field `wave511_vpu_idle` reader - VPU monitoring signal"]
pub type WAVE511_VPU_IDLE_R = crate::BitReader;
#[doc = "Field `can_ctrl_fd_enable_0` reader - can_ctrl_fd_enable_0"]
pub type CAN_CTRL_FD_ENABLE_0_R = crate::BitReader;
#[doc = "Field `can_ctrl_fd_enable_0` writer - can_ctrl_fd_enable_0"]
pub type CAN_CTRL_FD_ENABLE_0_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `can_ctrl_host_ecc_disable_0` reader - can_ctrl_host_ecc_disable_0"]
pub type CAN_CTRL_HOST_ECC_DISABLE_0_R = crate::BitReader;
#[doc = "Field `can_ctrl_host_ecc_disable_0` writer - can_ctrl_host_ecc_disable_0"]
pub type CAN_CTRL_HOST_ECC_DISABLE_0_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `can_ctrl_host_if_0` reader - can_ctrl_host_if_0"]
pub type CAN_CTRL_HOST_IF_0_R = crate::FieldReader<u32>;
#[doc = "Field `qspi_sclk_dlychain_sel` reader - des_qspi_sclk_dla: clock delay"]
pub type QSPI_SCLK_DLYCHAIN_SEL_R = crate::FieldReader;
impl R {
    #[doc = "Bits 0:1 - Tie 0 in JPU internal, do not care"]
    #[inline(always)]
    pub fn coda12_cur_inst(&self) -> CODA12_CUR_INST_R {
        CODA12_CUR_INST_R::new((self.bits & 3) as u8)
    }
    #[doc = "Bit 2 - VPU monitoring signal"]
    #[inline(always)]
    pub fn wave511_vpu_idle(&self) -> WAVE511_VPU_IDLE_R {
        WAVE511_VPU_IDLE_R::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3 - can_ctrl_fd_enable_0"]
    #[inline(always)]
    pub fn can_ctrl_fd_enable_0(&self) -> CAN_CTRL_FD_ENABLE_0_R {
        CAN_CTRL_FD_ENABLE_0_R::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bit 4 - can_ctrl_host_ecc_disable_0"]
    #[inline(always)]
    pub fn can_ctrl_host_ecc_disable_0(&self) -> CAN_CTRL_HOST_ECC_DISABLE_0_R {
        CAN_CTRL_HOST_ECC_DISABLE_0_R::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bits 5:23 - can_ctrl_host_if_0"]
    #[inline(always)]
    pub fn can_ctrl_host_if_0(&self) -> CAN_CTRL_HOST_IF_0_R {
        CAN_CTRL_HOST_IF_0_R::new((self.bits >> 5) & 0x0007_ffff)
    }
    #[doc = "Bits 24:28 - des_qspi_sclk_dla: clock delay"]
    #[inline(always)]
    pub fn qspi_sclk_dlychain_sel(&self) -> QSPI_SCLK_DLYCHAIN_SEL_R {
        QSPI_SCLK_DLYCHAIN_SEL_R::new(((self.bits >> 24) & 0x1f) as u8)
    }
}
impl W {
    #[doc = "Bit 3 - can_ctrl_fd_enable_0"]
    #[inline(always)]
    #[must_use]
    pub fn can_ctrl_fd_enable_0(&mut self) -> CAN_CTRL_FD_ENABLE_0_W<SYS_SYSCFG_4_SPEC> {
        CAN_CTRL_FD_ENABLE_0_W::new(self, 3)
    }
    #[doc = "Bit 4 - can_ctrl_host_ecc_disable_0"]
    #[inline(always)]
    #[must_use]
    pub fn can_ctrl_host_ecc_disable_0(
        &mut self,
    ) -> CAN_CTRL_HOST_ECC_DISABLE_0_W<SYS_SYSCFG_4_SPEC> {
        CAN_CTRL_HOST_ECC_DISABLE_0_W::new(self, 4)
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
