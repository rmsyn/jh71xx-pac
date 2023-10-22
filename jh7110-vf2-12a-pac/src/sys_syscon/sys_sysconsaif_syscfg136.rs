#[doc = "Register `sys_sysconsaif_syscfg136` reader"]
pub type R = crate::R<SYS_SYSCONSAIF_SYSCFG136_SPEC>;
#[doc = "Register `sys_sysconsaif_syscfg136` writer"]
pub type W = crate::W<SYS_SYSCONSAIF_SYSCFG136_SPEC>;
#[doc = "Field `u0_venc_intsram_sram_config_slp` reader - SRAM/ROM configuration. SLP: sleep enable, high active, default is low."]
pub type U0_VENC_INTSRAM_SRAM_CONFIG_SLP_R = crate::BitReader;
#[doc = "Field `u0_venc_intsram_sram_config_slp` writer - SRAM/ROM configuration. SLP: sleep enable, high active, default is low."]
pub type U0_VENC_INTSRAM_SRAM_CONFIG_SLP_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O>;
#[doc = "Field `u0_venc_intsram_sram_config_sram_config_sd` reader - SRAM/ROM configuration. SD: shutdown enable, high active, default is low."]
pub type U0_VENC_INTSRAM_SRAM_CONFIG_SRAM_CONFIG_SD_R = crate::BitReader;
#[doc = "Field `u0_venc_intsram_sram_config_sram_config_sd` writer - SRAM/ROM configuration. SD: shutdown enable, high active, default is low."]
pub type U0_VENC_INTSRAM_SRAM_CONFIG_SRAM_CONFIG_SD_W<'a, REG, const O: u8> =
    crate::BitWriter<'a, REG, O>;
#[doc = "Field `u0_venc_intsram_sram_config_rtsel` reader - SRAM/ROM configuration. RTSEL: timing setting for debug purpose, default is 2'b01."]
pub type U0_VENC_INTSRAM_SRAM_CONFIG_RTSEL_R = crate::FieldReader;
#[doc = "Field `u0_venc_intsram_sram_config_rtsel` writer - SRAM/ROM configuration. RTSEL: timing setting for debug purpose, default is 2'b01."]
pub type U0_VENC_INTSRAM_SRAM_CONFIG_RTSEL_W<'a, REG, const O: u8> =
    crate::FieldWriter<'a, REG, 2, O>;
#[doc = "Field `u0_venc_intsram_sram_config_ptsel` reader - SRAM/ROM configuration. PTSEL: timing setting for debug purpose, default is 2'b01."]
pub type U0_VENC_INTSRAM_SRAM_CONFIG_PTSEL_R = crate::FieldReader;
#[doc = "Field `u0_venc_intsram_sram_config_ptsel` writer - SRAM/ROM configuration. PTSEL: timing setting for debug purpose, default is 2'b01."]
pub type U0_VENC_INTSRAM_SRAM_CONFIG_PTSEL_W<'a, REG, const O: u8> =
    crate::FieldWriter<'a, REG, 2, O>;
#[doc = "Field `u0_venc_intsram_sram_config_trb` reader - SRAM/ROM configuration. TRB: timing setting for debug purpose, default is 2'b01."]
pub type U0_VENC_INTSRAM_SRAM_CONFIG_TRB_R = crate::FieldReader;
#[doc = "Field `u0_venc_intsram_sram_config_trb` writer - SRAM/ROM configuration. TRB: timing setting for debug purpose, default is 2'b01."]
pub type U0_VENC_INTSRAM_SRAM_CONFIG_TRB_W<'a, REG, const O: u8> =
    crate::FieldWriter<'a, REG, 2, O>;
#[doc = "Field `u0_venc_intsram_sram_config_wtsel` reader - SRAM/ROM configuration. WTSEL: timing setting for debug purpose, default is 2'b01."]
pub type U0_VENC_INTSRAM_SRAM_CONFIG_WTSEL_R = crate::FieldReader;
#[doc = "Field `u0_venc_intsram_sram_config_wtsel` writer - SRAM/ROM configuration. WTSEL: timing setting for debug purpose, default is 2'b01."]
pub type U0_VENC_INTSRAM_SRAM_CONFIG_WTSEL_W<'a, REG, const O: u8> =
    crate::FieldWriter<'a, REG, 2, O>;
#[doc = "Field `u0_venc_intsram_sram_config_vs` reader - SRAM/ROM configuration. VS: timing setting for debug purpose, default is 1'b1."]
pub type U0_VENC_INTSRAM_SRAM_CONFIG_VS_R = crate::BitReader;
#[doc = "Field `u0_venc_intsram_sram_config_vs` writer - SRAM/ROM configuration. VS: timing setting for debug purpose, default is 1'b1."]
pub type U0_VENC_INTSRAM_SRAM_CONFIG_VS_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O>;
#[doc = "Field `u0_venc_intsram_sram_config_vg` reader - SRAM/ROM configuration. VG: timing setting for debug purpose, default is 1'b1."]
pub type U0_VENC_INTSRAM_SRAM_CONFIG_VG_R = crate::BitReader;
#[doc = "Field `u0_venc_intsram_sram_config_vg` writer - SRAM/ROM configuration. VG: timing setting for debug purpose, default is 1'b1."]
pub type U0_VENC_INTSRAM_SRAM_CONFIG_VG_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O>;
#[doc = "Field `u0_wave420l_i_ipu_current_buffer` reader - This signal indicates which buffer is currently active so that the VPU can correctly use the ipu_end_of_row signal for row counter."]
pub type U0_WAVE420L_I_IPU_CURRENT_BUFFER_R = crate::FieldReader;
#[doc = "Field `u0_wave420l_i_ipu_current_buffer` writer - This signal indicates which buffer is currently active so that the VPU can correctly use the ipu_end_of_row signal for row counter."]
pub type U0_WAVE420L_I_IPU_CURRENT_BUFFER_W<'a, REG, const O: u8> =
    crate::FieldWriter<'a, REG, 3, O>;
#[doc = "Field `u0_wave420l_i_ipu_end_of_row` reader - This signal is flipped every time when the IPU completes writing a row."]
pub type U0_WAVE420L_I_IPU_END_OF_ROW_R = crate::BitReader;
#[doc = "Field `u0_wave420l_i_ipu_end_of_row` writer - This signal is flipped every time when the IPU completes writing a row."]
pub type U0_WAVE420L_I_IPU_END_OF_ROW_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O>;
#[doc = "Field `u0_wave420l_i_ipu_new_frame` reader - This signal is flipped every time when the IPU completes writing a new frame."]
pub type U0_WAVE420L_I_IPU_NEW_FRAME_R = crate::BitReader;
#[doc = "Field `u0_wave420l_i_ipu_new_frame` writer - This signal is flipped every time when the IPU completes writing a new frame."]
pub type U0_WAVE420L_I_IPU_NEW_FRAME_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O>;
#[doc = "Field `u0_wave420l_o_vpu_idle` reader - VPU monitoring signal. This signal gives out an opposite value of VPU_BUSY register."]
pub type U0_WAVE420L_O_VPU_IDLE_R = crate::BitReader;
#[doc = "Field `u1_can_ctrl_can_fd_enable` reader - u1_can_ctrl_can_fd_enable"]
pub type U1_CAN_CTRL_CAN_FD_ENABLE_R = crate::BitReader;
#[doc = "Field `u1_can_ctrl_can_fd_enable` writer - u1_can_ctrl_can_fd_enable"]
pub type U1_CAN_CTRL_CAN_FD_ENABLE_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O>;
#[doc = "Field `u1_can_ctrl_host_ecc_disable` reader - u1_can_ctrl_host_ecc_disable"]
pub type U1_CAN_CTRL_HOST_ECC_DISABLE_R = crate::BitReader;
#[doc = "Field `u1_can_ctrl_host_ecc_disable` writer - u1_can_ctrl_host_ecc_disable"]
pub type U1_CAN_CTRL_HOST_ECC_DISABLE_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O>;
impl R {
    #[doc = "Bit 0 - SRAM/ROM configuration. SLP: sleep enable, high active, default is low."]
    #[inline(always)]
    pub fn u0_venc_intsram_sram_config_slp(&self) -> U0_VENC_INTSRAM_SRAM_CONFIG_SLP_R {
        U0_VENC_INTSRAM_SRAM_CONFIG_SLP_R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - SRAM/ROM configuration. SD: shutdown enable, high active, default is low."]
    #[inline(always)]
    pub fn u0_venc_intsram_sram_config_sram_config_sd(
        &self,
    ) -> U0_VENC_INTSRAM_SRAM_CONFIG_SRAM_CONFIG_SD_R {
        U0_VENC_INTSRAM_SRAM_CONFIG_SRAM_CONFIG_SD_R::new(((self.bits >> 1) & 1) != 0)
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
    pub fn u0_wave420l_i_ipu_current_buffer(&self) -> U0_WAVE420L_I_IPU_CURRENT_BUFFER_R {
        U0_WAVE420L_I_IPU_CURRENT_BUFFER_R::new(((self.bits >> 12) & 7) as u8)
    }
    #[doc = "Bit 15 - This signal is flipped every time when the IPU completes writing a row."]
    #[inline(always)]
    pub fn u0_wave420l_i_ipu_end_of_row(&self) -> U0_WAVE420L_I_IPU_END_OF_ROW_R {
        U0_WAVE420L_I_IPU_END_OF_ROW_R::new(((self.bits >> 15) & 1) != 0)
    }
    #[doc = "Bit 16 - This signal is flipped every time when the IPU completes writing a new frame."]
    #[inline(always)]
    pub fn u0_wave420l_i_ipu_new_frame(&self) -> U0_WAVE420L_I_IPU_NEW_FRAME_R {
        U0_WAVE420L_I_IPU_NEW_FRAME_R::new(((self.bits >> 16) & 1) != 0)
    }
    #[doc = "Bit 17 - VPU monitoring signal. This signal gives out an opposite value of VPU_BUSY register."]
    #[inline(always)]
    pub fn u0_wave420l_o_vpu_idle(&self) -> U0_WAVE420L_O_VPU_IDLE_R {
        U0_WAVE420L_O_VPU_IDLE_R::new(((self.bits >> 17) & 1) != 0)
    }
    #[doc = "Bit 18 - u1_can_ctrl_can_fd_enable"]
    #[inline(always)]
    pub fn u1_can_ctrl_can_fd_enable(&self) -> U1_CAN_CTRL_CAN_FD_ENABLE_R {
        U1_CAN_CTRL_CAN_FD_ENABLE_R::new(((self.bits >> 18) & 1) != 0)
    }
    #[doc = "Bit 19 - u1_can_ctrl_host_ecc_disable"]
    #[inline(always)]
    pub fn u1_can_ctrl_host_ecc_disable(&self) -> U1_CAN_CTRL_HOST_ECC_DISABLE_R {
        U1_CAN_CTRL_HOST_ECC_DISABLE_R::new(((self.bits >> 19) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - SRAM/ROM configuration. SLP: sleep enable, high active, default is low."]
    #[inline(always)]
    #[must_use]
    pub fn u0_venc_intsram_sram_config_slp(
        &mut self,
    ) -> U0_VENC_INTSRAM_SRAM_CONFIG_SLP_W<SYS_SYSCONSAIF_SYSCFG136_SPEC, 0> {
        U0_VENC_INTSRAM_SRAM_CONFIG_SLP_W::new(self)
    }
    #[doc = "Bit 1 - SRAM/ROM configuration. SD: shutdown enable, high active, default is low."]
    #[inline(always)]
    #[must_use]
    pub fn u0_venc_intsram_sram_config_sram_config_sd(
        &mut self,
    ) -> U0_VENC_INTSRAM_SRAM_CONFIG_SRAM_CONFIG_SD_W<SYS_SYSCONSAIF_SYSCFG136_SPEC, 1> {
        U0_VENC_INTSRAM_SRAM_CONFIG_SRAM_CONFIG_SD_W::new(self)
    }
    #[doc = "Bits 2:3 - SRAM/ROM configuration. RTSEL: timing setting for debug purpose, default is 2'b01."]
    #[inline(always)]
    #[must_use]
    pub fn u0_venc_intsram_sram_config_rtsel(
        &mut self,
    ) -> U0_VENC_INTSRAM_SRAM_CONFIG_RTSEL_W<SYS_SYSCONSAIF_SYSCFG136_SPEC, 2> {
        U0_VENC_INTSRAM_SRAM_CONFIG_RTSEL_W::new(self)
    }
    #[doc = "Bits 4:5 - SRAM/ROM configuration. PTSEL: timing setting for debug purpose, default is 2'b01."]
    #[inline(always)]
    #[must_use]
    pub fn u0_venc_intsram_sram_config_ptsel(
        &mut self,
    ) -> U0_VENC_INTSRAM_SRAM_CONFIG_PTSEL_W<SYS_SYSCONSAIF_SYSCFG136_SPEC, 4> {
        U0_VENC_INTSRAM_SRAM_CONFIG_PTSEL_W::new(self)
    }
    #[doc = "Bits 6:7 - SRAM/ROM configuration. TRB: timing setting for debug purpose, default is 2'b01."]
    #[inline(always)]
    #[must_use]
    pub fn u0_venc_intsram_sram_config_trb(
        &mut self,
    ) -> U0_VENC_INTSRAM_SRAM_CONFIG_TRB_W<SYS_SYSCONSAIF_SYSCFG136_SPEC, 6> {
        U0_VENC_INTSRAM_SRAM_CONFIG_TRB_W::new(self)
    }
    #[doc = "Bits 8:9 - SRAM/ROM configuration. WTSEL: timing setting for debug purpose, default is 2'b01."]
    #[inline(always)]
    #[must_use]
    pub fn u0_venc_intsram_sram_config_wtsel(
        &mut self,
    ) -> U0_VENC_INTSRAM_SRAM_CONFIG_WTSEL_W<SYS_SYSCONSAIF_SYSCFG136_SPEC, 8> {
        U0_VENC_INTSRAM_SRAM_CONFIG_WTSEL_W::new(self)
    }
    #[doc = "Bit 10 - SRAM/ROM configuration. VS: timing setting for debug purpose, default is 1'b1."]
    #[inline(always)]
    #[must_use]
    pub fn u0_venc_intsram_sram_config_vs(
        &mut self,
    ) -> U0_VENC_INTSRAM_SRAM_CONFIG_VS_W<SYS_SYSCONSAIF_SYSCFG136_SPEC, 10> {
        U0_VENC_INTSRAM_SRAM_CONFIG_VS_W::new(self)
    }
    #[doc = "Bit 11 - SRAM/ROM configuration. VG: timing setting for debug purpose, default is 1'b1."]
    #[inline(always)]
    #[must_use]
    pub fn u0_venc_intsram_sram_config_vg(
        &mut self,
    ) -> U0_VENC_INTSRAM_SRAM_CONFIG_VG_W<SYS_SYSCONSAIF_SYSCFG136_SPEC, 11> {
        U0_VENC_INTSRAM_SRAM_CONFIG_VG_W::new(self)
    }
    #[doc = "Bits 12:14 - This signal indicates which buffer is currently active so that the VPU can correctly use the ipu_end_of_row signal for row counter."]
    #[inline(always)]
    #[must_use]
    pub fn u0_wave420l_i_ipu_current_buffer(
        &mut self,
    ) -> U0_WAVE420L_I_IPU_CURRENT_BUFFER_W<SYS_SYSCONSAIF_SYSCFG136_SPEC, 12> {
        U0_WAVE420L_I_IPU_CURRENT_BUFFER_W::new(self)
    }
    #[doc = "Bit 15 - This signal is flipped every time when the IPU completes writing a row."]
    #[inline(always)]
    #[must_use]
    pub fn u0_wave420l_i_ipu_end_of_row(
        &mut self,
    ) -> U0_WAVE420L_I_IPU_END_OF_ROW_W<SYS_SYSCONSAIF_SYSCFG136_SPEC, 15> {
        U0_WAVE420L_I_IPU_END_OF_ROW_W::new(self)
    }
    #[doc = "Bit 16 - This signal is flipped every time when the IPU completes writing a new frame."]
    #[inline(always)]
    #[must_use]
    pub fn u0_wave420l_i_ipu_new_frame(
        &mut self,
    ) -> U0_WAVE420L_I_IPU_NEW_FRAME_W<SYS_SYSCONSAIF_SYSCFG136_SPEC, 16> {
        U0_WAVE420L_I_IPU_NEW_FRAME_W::new(self)
    }
    #[doc = "Bit 18 - u1_can_ctrl_can_fd_enable"]
    #[inline(always)]
    #[must_use]
    pub fn u1_can_ctrl_can_fd_enable(
        &mut self,
    ) -> U1_CAN_CTRL_CAN_FD_ENABLE_W<SYS_SYSCONSAIF_SYSCFG136_SPEC, 18> {
        U1_CAN_CTRL_CAN_FD_ENABLE_W::new(self)
    }
    #[doc = "Bit 19 - u1_can_ctrl_host_ecc_disable"]
    #[inline(always)]
    #[must_use]
    pub fn u1_can_ctrl_host_ecc_disable(
        &mut self,
    ) -> U1_CAN_CTRL_HOST_ECC_DISABLE_W<SYS_SYSCONSAIF_SYSCFG136_SPEC, 19> {
        U1_CAN_CTRL_HOST_ECC_DISABLE_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.bits = bits;
        self
    }
}
#[doc = "SYS SYSCONSAIF SYSCFG 136\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`sys_sysconsaif_syscfg136::R`](R).  You can [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`sys_sysconsaif_syscfg136::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct SYS_SYSCONSAIF_SYSCFG136_SPEC;
impl crate::RegisterSpec for SYS_SYSCONSAIF_SYSCFG136_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`sys_sysconsaif_syscfg136::R`](R) reader structure"]
impl crate::Readable for SYS_SYSCONSAIF_SYSCFG136_SPEC {}
#[doc = "`write(|w| ..)` method takes [`sys_sysconsaif_syscfg136::W`](W) writer structure"]
impl crate::Writable for SYS_SYSCONSAIF_SYSCFG136_SPEC {
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
