#[doc = "Register `sys_syscfg_34` reader"]
pub type R = crate::R<SYS_SYSCFG_34_SPEC>;
#[doc = "Register `sys_syscfg_34` writer"]
pub type W = crate::W<SYS_SYSCFG_34_SPEC>;
#[doc = "Field `u0_venc_intsram_sram_config_slp` reader - SRAM/ROM configuration. SLP: sleep enable, high active, default is low."]
pub type U0_VENC_INTSRAM_SRAM_CONFIG_SLP_R = crate::BitReader;
#[doc = "Field `u0_venc_intsram_sram_config_slp` writer - SRAM/ROM configuration. SLP: sleep enable, high active, default is low."]
pub type U0_VENC_INTSRAM_SRAM_CONFIG_SLP_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `u0_venc_intsram_sram_config_sd` reader - SRAM/ROM configuration. SD: shutdown enable, high active, default is low."]
pub type U0_VENC_INTSRAM_SRAM_CONFIG_SD_R = crate::BitReader;
#[doc = "Field `u0_venc_intsram_sram_config_sd` writer - SRAM/ROM configuration. SD: shutdown enable, high active, default is low."]
pub type U0_VENC_INTSRAM_SRAM_CONFIG_SD_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `u0_venc_intsram_sram_config_rtsel` reader - SRAM/ROM configuration. RTSEL: timing setting for debug purpose, default is 2'b01."]
pub type U0_VENC_INTSRAM_SRAM_CONFIG_RTSEL_R = crate::FieldReader;
#[doc = "Field `u0_venc_intsram_sram_config_rtsel` writer - SRAM/ROM configuration. RTSEL: timing setting for debug purpose, default is 2'b01."]
pub type U0_VENC_INTSRAM_SRAM_CONFIG_RTSEL_W<'a, REG> = crate::FieldWriter<'a, REG, 2>;
#[doc = "Field `u0_venc_intsram_sram_config_ptsel` reader - SRAM/ROM configuration. PTSEL: timing setting for debug purpose, default is 2'b01."]
pub type U0_VENC_INTSRAM_SRAM_CONFIG_PTSEL_R = crate::FieldReader;
#[doc = "Field `u0_venc_intsram_sram_config_ptsel` writer - SRAM/ROM configuration. PTSEL: timing setting for debug purpose, default is 2'b01."]
pub type U0_VENC_INTSRAM_SRAM_CONFIG_PTSEL_W<'a, REG> = crate::FieldWriter<'a, REG, 2>;
#[doc = "Field `u0_venc_intsram_sram_config_trb` reader - SRAM/ROM configuration. TRB: timing setting for debug purpose, default is 2'b01."]
pub type U0_VENC_INTSRAM_SRAM_CONFIG_TRB_R = crate::FieldReader;
#[doc = "Field `u0_venc_intsram_sram_config_trb` writer - SRAM/ROM configuration. TRB: timing setting for debug purpose, default is 2'b01."]
pub type U0_VENC_INTSRAM_SRAM_CONFIG_TRB_W<'a, REG> = crate::FieldWriter<'a, REG, 2>;
#[doc = "Field `u0_venc_intsram_sram_config_wtsel` reader - SRAM/ROM configuration. WTSEL: timing setting for debug purpose, default is 2'b01."]
pub type U0_VENC_INTSRAM_SRAM_CONFIG_WTSEL_R = crate::FieldReader;
#[doc = "Field `u0_venc_intsram_sram_config_wtsel` writer - SRAM/ROM configuration. WTSEL: timing setting for debug purpose, default is 2'b01."]
pub type U0_VENC_INTSRAM_SRAM_CONFIG_WTSEL_W<'a, REG> = crate::FieldWriter<'a, REG, 2>;
#[doc = "Field `u0_venc_intsram_sram_config_vs` reader - SRAM/ROM configuration. VS: timing setting for debug purpose, default is 1'b1."]
pub type U0_VENC_INTSRAM_SRAM_CONFIG_VS_R = crate::BitReader;
#[doc = "Field `u0_venc_intsram_sram_config_vs` writer - SRAM/ROM configuration. VS: timing setting for debug purpose, default is 1'b1."]
pub type U0_VENC_INTSRAM_SRAM_CONFIG_VS_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `u0_venc_intsram_sram_config_vg` reader - SRAM/ROM configuration. VG: timing setting for debug purpose, default is 1'b1."]
pub type U0_VENC_INTSRAM_SRAM_CONFIG_VG_R = crate::BitReader;
#[doc = "Field `u0_venc_intsram_sram_config_vg` writer - SRAM/ROM configuration. VG: timing setting for debug purpose, default is 1'b1."]
pub type U0_VENC_INTSRAM_SRAM_CONFIG_VG_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `wave420l_ipu_current_buffer` reader - This signal indicates which buffer is currently active so that the VPU can correctly use the ipu_end_of_row signal for row counter."]
pub type WAVE420L_IPU_CURRENT_BUFFER_R = crate::FieldReader;
#[doc = "Field `wave420l_ipu_current_buffer` writer - This signal indicates which buffer is currently active so that the VPU can correctly use the ipu_end_of_row signal for row counter."]
pub type WAVE420L_IPU_CURRENT_BUFFER_W<'a, REG> = crate::FieldWriter<'a, REG, 3>;
#[doc = "Field `wave420l_ipu_end_of_row` reader - This signal is flipped every time when the IPU completes writing a row."]
pub type WAVE420L_IPU_END_OF_ROW_R = crate::BitReader;
#[doc = "Field `wave420l_ipu_end_of_row` writer - This signal is flipped every time when the IPU completes writing a row."]
pub type WAVE420L_IPU_END_OF_ROW_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `wave420l_ipu_new_frame` reader - This signal is flipped every time when the IPU completes writing a new frame."]
pub type WAVE420L_IPU_NEW_FRAME_R = crate::BitReader;
#[doc = "Field `wave420l_ipu_new_frame` writer - This signal is flipped every time when the IPU completes writing a new frame."]
pub type WAVE420L_IPU_NEW_FRAME_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `wave420l_vpu_idle` reader - VPU monitoring signal. This signal gives out an opposite value of VPU_BUSY register."]
pub type WAVE420L_VPU_IDLE_R = crate::BitReader;
#[doc = "Field `can_ctrl_fd_enable_1` reader - can_ctrl_fd_enable_1"]
pub type CAN_CTRL_FD_ENABLE_1_R = crate::BitReader;
#[doc = "Field `can_ctrl_fd_enable_1` writer - can_ctrl_fd_enable_1"]
pub type CAN_CTRL_FD_ENABLE_1_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `can_ctrl_host_ecc_disable_1` reader - can_ctrl_host_ecc_disable_1"]
pub type CAN_CTRL_HOST_ECC_DISABLE_1_R = crate::BitReader;
#[doc = "Field `can_ctrl_host_ecc_disable_1` writer - can_ctrl_host_ecc_disable_1"]
pub type CAN_CTRL_HOST_ECC_DISABLE_1_W<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bit 0 - SRAM/ROM configuration. SLP: sleep enable, high active, default is low."]
    #[inline(always)]
    pub fn u0_venc_intsram_sram_config_slp(&self) -> U0_VENC_INTSRAM_SRAM_CONFIG_SLP_R {
        U0_VENC_INTSRAM_SRAM_CONFIG_SLP_R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - SRAM/ROM configuration. SD: shutdown enable, high active, default is low."]
    #[inline(always)]
    pub fn u0_venc_intsram_sram_config_sd(&self) -> U0_VENC_INTSRAM_SRAM_CONFIG_SD_R {
        U0_VENC_INTSRAM_SRAM_CONFIG_SD_R::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bits 2:3 - SRAM/ROM configuration. RTSEL: timing setting for debug purpose, default is 2'b01."]
    #[inline(always)]
    pub fn u0_venc_intsram_sram_config_rtsel(&self) -> U0_VENC_INTSRAM_SRAM_CONFIG_RTSEL_R {
        U0_VENC_INTSRAM_SRAM_CONFIG_RTSEL_R::new(((self.bits >> 2) & 3) as u8)
    }
    #[doc = "Bits 4:5 - SRAM/ROM configuration. PTSEL: timing setting for debug purpose, default is 2'b01."]
    #[inline(always)]
    pub fn u0_venc_intsram_sram_config_ptsel(&self) -> U0_VENC_INTSRAM_SRAM_CONFIG_PTSEL_R {
        U0_VENC_INTSRAM_SRAM_CONFIG_PTSEL_R::new(((self.bits >> 4) & 3) as u8)
    }
    #[doc = "Bits 6:7 - SRAM/ROM configuration. TRB: timing setting for debug purpose, default is 2'b01."]
    #[inline(always)]
    pub fn u0_venc_intsram_sram_config_trb(&self) -> U0_VENC_INTSRAM_SRAM_CONFIG_TRB_R {
        U0_VENC_INTSRAM_SRAM_CONFIG_TRB_R::new(((self.bits >> 6) & 3) as u8)
    }
    #[doc = "Bits 8:9 - SRAM/ROM configuration. WTSEL: timing setting for debug purpose, default is 2'b01."]
    #[inline(always)]
    pub fn u0_venc_intsram_sram_config_wtsel(&self) -> U0_VENC_INTSRAM_SRAM_CONFIG_WTSEL_R {
        U0_VENC_INTSRAM_SRAM_CONFIG_WTSEL_R::new(((self.bits >> 8) & 3) as u8)
    }
    #[doc = "Bit 10 - SRAM/ROM configuration. VS: timing setting for debug purpose, default is 1'b1."]
    #[inline(always)]
    pub fn u0_venc_intsram_sram_config_vs(&self) -> U0_VENC_INTSRAM_SRAM_CONFIG_VS_R {
        U0_VENC_INTSRAM_SRAM_CONFIG_VS_R::new(((self.bits >> 10) & 1) != 0)
    }
    #[doc = "Bit 11 - SRAM/ROM configuration. VG: timing setting for debug purpose, default is 1'b1."]
    #[inline(always)]
    pub fn u0_venc_intsram_sram_config_vg(&self) -> U0_VENC_INTSRAM_SRAM_CONFIG_VG_R {
        U0_VENC_INTSRAM_SRAM_CONFIG_VG_R::new(((self.bits >> 11) & 1) != 0)
    }
    #[doc = "Bits 12:14 - This signal indicates which buffer is currently active so that the VPU can correctly use the ipu_end_of_row signal for row counter."]
    #[inline(always)]
    pub fn wave420l_ipu_current_buffer(&self) -> WAVE420L_IPU_CURRENT_BUFFER_R {
        WAVE420L_IPU_CURRENT_BUFFER_R::new(((self.bits >> 12) & 7) as u8)
    }
    #[doc = "Bit 15 - This signal is flipped every time when the IPU completes writing a row."]
    #[inline(always)]
    pub fn wave420l_ipu_end_of_row(&self) -> WAVE420L_IPU_END_OF_ROW_R {
        WAVE420L_IPU_END_OF_ROW_R::new(((self.bits >> 15) & 1) != 0)
    }
    #[doc = "Bit 16 - This signal is flipped every time when the IPU completes writing a new frame."]
    #[inline(always)]
    pub fn wave420l_ipu_new_frame(&self) -> WAVE420L_IPU_NEW_FRAME_R {
        WAVE420L_IPU_NEW_FRAME_R::new(((self.bits >> 16) & 1) != 0)
    }
    #[doc = "Bit 17 - VPU monitoring signal. This signal gives out an opposite value of VPU_BUSY register."]
    #[inline(always)]
    pub fn wave420l_vpu_idle(&self) -> WAVE420L_VPU_IDLE_R {
        WAVE420L_VPU_IDLE_R::new(((self.bits >> 17) & 1) != 0)
    }
    #[doc = "Bit 18 - can_ctrl_fd_enable_1"]
    #[inline(always)]
    pub fn can_ctrl_fd_enable_1(&self) -> CAN_CTRL_FD_ENABLE_1_R {
        CAN_CTRL_FD_ENABLE_1_R::new(((self.bits >> 18) & 1) != 0)
    }
    #[doc = "Bit 19 - can_ctrl_host_ecc_disable_1"]
    #[inline(always)]
    pub fn can_ctrl_host_ecc_disable_1(&self) -> CAN_CTRL_HOST_ECC_DISABLE_1_R {
        CAN_CTRL_HOST_ECC_DISABLE_1_R::new(((self.bits >> 19) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - SRAM/ROM configuration. SLP: sleep enable, high active, default is low."]
    #[inline(always)]
    #[must_use]
    pub fn u0_venc_intsram_sram_config_slp(
        &mut self,
    ) -> U0_VENC_INTSRAM_SRAM_CONFIG_SLP_W<SYS_SYSCFG_34_SPEC> {
        U0_VENC_INTSRAM_SRAM_CONFIG_SLP_W::new(self, 0)
    }
    #[doc = "Bit 1 - SRAM/ROM configuration. SD: shutdown enable, high active, default is low."]
    #[inline(always)]
    #[must_use]
    pub fn u0_venc_intsram_sram_config_sd(
        &mut self,
    ) -> U0_VENC_INTSRAM_SRAM_CONFIG_SD_W<SYS_SYSCFG_34_SPEC> {
        U0_VENC_INTSRAM_SRAM_CONFIG_SD_W::new(self, 1)
    }
    #[doc = "Bits 2:3 - SRAM/ROM configuration. RTSEL: timing setting for debug purpose, default is 2'b01."]
    #[inline(always)]
    #[must_use]
    pub fn u0_venc_intsram_sram_config_rtsel(
        &mut self,
    ) -> U0_VENC_INTSRAM_SRAM_CONFIG_RTSEL_W<SYS_SYSCFG_34_SPEC> {
        U0_VENC_INTSRAM_SRAM_CONFIG_RTSEL_W::new(self, 2)
    }
    #[doc = "Bits 4:5 - SRAM/ROM configuration. PTSEL: timing setting for debug purpose, default is 2'b01."]
    #[inline(always)]
    #[must_use]
    pub fn u0_venc_intsram_sram_config_ptsel(
        &mut self,
    ) -> U0_VENC_INTSRAM_SRAM_CONFIG_PTSEL_W<SYS_SYSCFG_34_SPEC> {
        U0_VENC_INTSRAM_SRAM_CONFIG_PTSEL_W::new(self, 4)
    }
    #[doc = "Bits 6:7 - SRAM/ROM configuration. TRB: timing setting for debug purpose, default is 2'b01."]
    #[inline(always)]
    #[must_use]
    pub fn u0_venc_intsram_sram_config_trb(
        &mut self,
    ) -> U0_VENC_INTSRAM_SRAM_CONFIG_TRB_W<SYS_SYSCFG_34_SPEC> {
        U0_VENC_INTSRAM_SRAM_CONFIG_TRB_W::new(self, 6)
    }
    #[doc = "Bits 8:9 - SRAM/ROM configuration. WTSEL: timing setting for debug purpose, default is 2'b01."]
    #[inline(always)]
    #[must_use]
    pub fn u0_venc_intsram_sram_config_wtsel(
        &mut self,
    ) -> U0_VENC_INTSRAM_SRAM_CONFIG_WTSEL_W<SYS_SYSCFG_34_SPEC> {
        U0_VENC_INTSRAM_SRAM_CONFIG_WTSEL_W::new(self, 8)
    }
    #[doc = "Bit 10 - SRAM/ROM configuration. VS: timing setting for debug purpose, default is 1'b1."]
    #[inline(always)]
    #[must_use]
    pub fn u0_venc_intsram_sram_config_vs(
        &mut self,
    ) -> U0_VENC_INTSRAM_SRAM_CONFIG_VS_W<SYS_SYSCFG_34_SPEC> {
        U0_VENC_INTSRAM_SRAM_CONFIG_VS_W::new(self, 10)
    }
    #[doc = "Bit 11 - SRAM/ROM configuration. VG: timing setting for debug purpose, default is 1'b1."]
    #[inline(always)]
    #[must_use]
    pub fn u0_venc_intsram_sram_config_vg(
        &mut self,
    ) -> U0_VENC_INTSRAM_SRAM_CONFIG_VG_W<SYS_SYSCFG_34_SPEC> {
        U0_VENC_INTSRAM_SRAM_CONFIG_VG_W::new(self, 11)
    }
    #[doc = "Bits 12:14 - This signal indicates which buffer is currently active so that the VPU can correctly use the ipu_end_of_row signal for row counter."]
    #[inline(always)]
    #[must_use]
    pub fn wave420l_ipu_current_buffer(
        &mut self,
    ) -> WAVE420L_IPU_CURRENT_BUFFER_W<SYS_SYSCFG_34_SPEC> {
        WAVE420L_IPU_CURRENT_BUFFER_W::new(self, 12)
    }
    #[doc = "Bit 15 - This signal is flipped every time when the IPU completes writing a row."]
    #[inline(always)]
    #[must_use]
    pub fn wave420l_ipu_end_of_row(&mut self) -> WAVE420L_IPU_END_OF_ROW_W<SYS_SYSCFG_34_SPEC> {
        WAVE420L_IPU_END_OF_ROW_W::new(self, 15)
    }
    #[doc = "Bit 16 - This signal is flipped every time when the IPU completes writing a new frame."]
    #[inline(always)]
    #[must_use]
    pub fn wave420l_ipu_new_frame(&mut self) -> WAVE420L_IPU_NEW_FRAME_W<SYS_SYSCFG_34_SPEC> {
        WAVE420L_IPU_NEW_FRAME_W::new(self, 16)
    }
    #[doc = "Bit 18 - can_ctrl_fd_enable_1"]
    #[inline(always)]
    #[must_use]
    pub fn can_ctrl_fd_enable_1(&mut self) -> CAN_CTRL_FD_ENABLE_1_W<SYS_SYSCFG_34_SPEC> {
        CAN_CTRL_FD_ENABLE_1_W::new(self, 18)
    }
    #[doc = "Bit 19 - can_ctrl_host_ecc_disable_1"]
    #[inline(always)]
    #[must_use]
    pub fn can_ctrl_host_ecc_disable_1(
        &mut self,
    ) -> CAN_CTRL_HOST_ECC_DISABLE_1_W<SYS_SYSCFG_34_SPEC> {
        CAN_CTRL_HOST_ECC_DISABLE_1_W::new(self, 19)
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
#[doc = "SYS SYSCONSAIF SYSCFG 136\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`sys_syscfg_34::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`sys_syscfg_34::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct SYS_SYSCFG_34_SPEC;
impl crate::RegisterSpec for SYS_SYSCFG_34_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`sys_syscfg_34::R`](R) reader structure"]
impl crate::Readable for SYS_SYSCFG_34_SPEC {}
#[doc = "`write(|w| ..)` method takes [`sys_syscfg_34::W`](W) writer structure"]
impl crate::Writable for SYS_SYSCFG_34_SPEC {
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets sys_syscfg_34 to value 0"]
impl crate::Resettable for SYS_SYSCFG_34_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
