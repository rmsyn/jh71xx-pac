#[doc = "Register `syscrg_rst_status_1` reader"]
pub type R = crate::R<SYSCRG_RST_STATUS_1_SPEC>;
#[doc = "Register `syscrg_rst_status_1` writer"]
pub type W = crate::W<SYSCRG_RST_STATUS_1_SPEC>;
#[doc = "Field `u0_noc_bus_venc_axi` reader - 1: Assert reset, 0: De-assert reset"]
pub type U0_NOC_BUS_VENC_AXI_R = crate::BitReader;
#[doc = "Field `u0_noc_bus_venc_axi` writer - 1: Assert reset, 0: De-assert reset"]
pub type U0_NOC_BUS_VENC_AXI_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `u0_axi_cfg1_dec_ahb` reader - 1: Assert reset, 0: De-assert reset"]
pub type U0_AXI_CFG1_DEC_AHB_R = crate::BitReader;
#[doc = "Field `u0_axi_cfg1_dec_ahb` writer - 1: Assert reset, 0: De-assert reset"]
pub type U0_AXI_CFG1_DEC_AHB_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `u0_axi_cfg1_dec_main` reader - 1: Assert reset, 0: De-assert reset"]
pub type U0_AXI_CFG1_DEC_MAIN_R = crate::BitReader;
#[doc = "Field `u0_axi_cfg1_dec_main` writer - 1: Assert reset, 0: De-assert reset"]
pub type U0_AXI_CFG1_DEC_MAIN_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `u0_axi_cfg0_dec_main` reader - 1: Assert reset, 0: De-assert reset"]
pub type U0_AXI_CFG0_DEC_MAIN_R = crate::BitReader;
#[doc = "Field `u0_axi_cfg0_dec_main` writer - 1: Assert reset, 0: De-assert reset"]
pub type U0_AXI_CFG0_DEC_MAIN_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `u0_axi_cfg0_dec_main_div` reader - 1: Assert reset, 0: De-assert reset"]
pub type U0_AXI_CFG0_DEC_MAIN_DIV_R = crate::BitReader;
#[doc = "Field `u0_axi_cfg0_dec_main_div` writer - 1: Assert reset, 0: De-assert reset"]
pub type U0_AXI_CFG0_DEC_MAIN_DIV_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `u0_axi_cfg0_dec_hifi4` reader - 1: Assert reset, 0: De-assert reset"]
pub type U0_AXI_CFG0_DEC_HIFI4_R = crate::BitReader;
#[doc = "Field `u0_axi_cfg0_dec_hifi4` writer - 1: Assert reset, 0: De-assert reset"]
pub type U0_AXI_CFG0_DEC_HIFI4_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `u0_ddr_axi` reader - 1: Assert reset, 0: De-assert reset"]
pub type U0_DDR_AXI_R = crate::BitReader;
#[doc = "Field `u0_ddr_axi` writer - 1: Assert reset, 0: De-assert reset"]
pub type U0_DDR_AXI_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `u0_ddr_osc` reader - 1: Assert reset, 0: De-assert reset"]
pub type U0_DDR_OSC_R = crate::BitReader;
#[doc = "Field `u0_ddr_osc` writer - 1: Assert reset, 0: De-assert reset"]
pub type U0_DDR_OSC_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `u0_ddr_apb` reader - 1: Assert reset, 0: De-assert reset"]
pub type U0_DDR_APB_R = crate::BitReader;
#[doc = "Field `u0_ddr_apb` writer - 1: Assert reset, 0: De-assert reset"]
pub type U0_DDR_APB_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `u0_isp_top` reader - 1: Assert reset, 0: De-assert reset"]
pub type U0_ISP_TOP_R = crate::BitReader;
#[doc = "Field `u0_isp_top` writer - 1: Assert reset, 0: De-assert reset"]
pub type U0_ISP_TOP_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `u0_isp_axi` reader - 1: Assert reset, 0: De-assert reset"]
pub type U0_ISP_AXI_R = crate::BitReader;
#[doc = "Field `u0_isp_axi` writer - 1: Assert reset, 0: De-assert reset"]
pub type U0_ISP_AXI_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `u0_vout_src` reader - 1: Assert reset, 0: De-assert reset"]
pub type U0_VOUT_SRC_R = crate::BitReader;
#[doc = "Field `u0_vout_src` writer - 1: Assert reset, 0: De-assert reset"]
pub type U0_VOUT_SRC_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `u0_codaj12_axi` reader - 1: Assert reset, 0: De-assert reset"]
pub type U0_CODAJ12_AXI_R = crate::BitReader;
#[doc = "Field `u0_codaj12_axi` writer - 1: Assert reset, 0: De-assert reset"]
pub type U0_CODAJ12_AXI_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `u0_codaj12_core` reader - 1: Assert reset, 0: De-assert reset"]
pub type U0_CODAJ12_CORE_R = crate::BitReader;
#[doc = "Field `u0_codaj12_core` writer - 1: Assert reset, 0: De-assert reset"]
pub type U0_CODAJ12_CORE_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `u0_codaj12_apb` reader - 1: Assert reset, 0: De-assert reset"]
pub type U0_CODAJ12_APB_R = crate::BitReader;
#[doc = "Field `u0_codaj12_apb` writer - 1: Assert reset, 0: De-assert reset"]
pub type U0_CODAJ12_APB_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `u0_wave511_axi` reader - 1: Assert reset, 0: De-assert reset"]
pub type U0_WAVE511_AXI_R = crate::BitReader;
#[doc = "Field `u0_wave511_axi` writer - 1: Assert reset, 0: De-assert reset"]
pub type U0_WAVE511_AXI_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `u0_wave511_bpu` reader - 1: Assert reset, 0: De-assert reset"]
pub type U0_WAVE511_BPU_R = crate::BitReader;
#[doc = "Field `u0_wave511_bpu` writer - 1: Assert reset, 0: De-assert reset"]
pub type U0_WAVE511_BPU_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `u0_wave511_vce` reader - 1: Assert reset, 0: De-assert reset"]
pub type U0_WAVE511_VCE_R = crate::BitReader;
#[doc = "Field `u0_wave511_vce` writer - 1: Assert reset, 0: De-assert reset"]
pub type U0_WAVE511_VCE_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `u0_wave511_apb` reader - 1: Assert reset, 0: De-assert reset"]
pub type U0_WAVE511_APB_R = crate::BitReader;
#[doc = "Field `u0_wave511_apb` writer - 1: Assert reset, 0: De-assert reset"]
pub type U0_WAVE511_APB_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `u0_vdec_jpg_arb` reader - 1: Assert reset, 0: De-assert reset"]
pub type U0_VDEC_JPG_ARB_R = crate::BitReader;
#[doc = "Field `u0_vdec_jpg_arb` writer - 1: Assert reset, 0: De-assert reset"]
pub type U0_VDEC_JPG_ARB_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `u0_vdec_jpg_arb_main` reader - 1: Assert reset, 0: De-assert reset"]
pub type U0_VDEC_JPG_ARB_MAIN_R = crate::BitReader;
#[doc = "Field `u0_vdec_jpg_arb_main` writer - 1: Assert reset, 0: De-assert reset"]
pub type U0_VDEC_JPG_ARB_MAIN_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `u0_aximem_128b_axi` reader - 1: Assert reset, 0: De-assert reset"]
pub type U0_AXIMEM_128B_AXI_R = crate::BitReader;
#[doc = "Field `u0_aximem_128b_axi` writer - 1: Assert reset, 0: De-assert reset"]
pub type U0_AXIMEM_128B_AXI_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `u0_wave420l_axi` reader - 1: Assert reset, 0: De-assert reset"]
pub type U0_WAVE420L_AXI_R = crate::BitReader;
#[doc = "Field `u0_wave420l_axi` writer - 1: Assert reset, 0: De-assert reset"]
pub type U0_WAVE420L_AXI_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `u0_wave420l_bpu` reader - 1: Assert reset, 0: De-assert reset"]
pub type U0_WAVE420L_BPU_R = crate::BitReader;
#[doc = "Field `u0_wave420l_bpu` writer - 1: Assert reset, 0: De-assert reset"]
pub type U0_WAVE420L_BPU_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `u0_wave420l_vce` reader - 1: Assert reset, 0: De-assert reset"]
pub type U0_WAVE420L_VCE_R = crate::BitReader;
#[doc = "Field `u0_wave420l_vce` writer - 1: Assert reset, 0: De-assert reset"]
pub type U0_WAVE420L_VCE_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `u0_wave420l_apb` reader - 1: Assert reset, 0: De-assert reset"]
pub type U0_WAVE420L_APB_R = crate::BitReader;
#[doc = "Field `u0_wave420l_apb` writer - 1: Assert reset, 0: De-assert reset"]
pub type U0_WAVE420L_APB_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `u1_aximem` reader - 1: Assert reset, 0: De-assert reset"]
pub type U1_AXIMEM_R = crate::BitReader;
#[doc = "Field `u1_aximem` writer - 1: Assert reset, 0: De-assert reset"]
pub type U1_AXIMEM_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `u2_aximem` reader - 1: Assert reset, 0: De-assert reset"]
pub type U2_AXIMEM_R = crate::BitReader;
#[doc = "Field `u2_aximem` writer - 1: Assert reset, 0: De-assert reset"]
pub type U2_AXIMEM_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `u0_intmem_rom_sram` reader - 1: Assert reset, 0: De-assert reset"]
pub type U0_INTMEM_ROM_SRAM_R = crate::BitReader;
#[doc = "Field `u0_intmem_rom_sram` writer - 1: Assert reset, 0: De-assert reset"]
pub type U0_INTMEM_ROM_SRAM_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `u0_qspi_ahb` reader - 1: Assert reset, 0: De-assert reset"]
pub type U0_QSPI_AHB_R = crate::BitReader;
#[doc = "Field `u0_qspi_ahb` writer - 1: Assert reset, 0: De-assert reset"]
pub type U0_QSPI_AHB_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `u0_qspi_apb` reader - 1: Assert reset, 0: De-assert reset"]
pub type U0_QSPI_APB_R = crate::BitReader;
#[doc = "Field `u0_qspi_apb` writer - 1: Assert reset, 0: De-assert reset"]
pub type U0_QSPI_APB_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `u0_qspi_ref` reader - 1: Assert reset, 0: De-assert reset"]
pub type U0_QSPI_REF_R = crate::BitReader;
#[doc = "Field `u0_qspi_ref` writer - 1: Assert reset, 0: De-assert reset"]
pub type U0_QSPI_REF_W<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bit 0 - 1: Assert reset, 0: De-assert reset"]
    #[inline(always)]
    pub fn u0_noc_bus_venc_axi(&self) -> U0_NOC_BUS_VENC_AXI_R {
        U0_NOC_BUS_VENC_AXI_R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - 1: Assert reset, 0: De-assert reset"]
    #[inline(always)]
    pub fn u0_axi_cfg1_dec_ahb(&self) -> U0_AXI_CFG1_DEC_AHB_R {
        U0_AXI_CFG1_DEC_AHB_R::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - 1: Assert reset, 0: De-assert reset"]
    #[inline(always)]
    pub fn u0_axi_cfg1_dec_main(&self) -> U0_AXI_CFG1_DEC_MAIN_R {
        U0_AXI_CFG1_DEC_MAIN_R::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3 - 1: Assert reset, 0: De-assert reset"]
    #[inline(always)]
    pub fn u0_axi_cfg0_dec_main(&self) -> U0_AXI_CFG0_DEC_MAIN_R {
        U0_AXI_CFG0_DEC_MAIN_R::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bit 4 - 1: Assert reset, 0: De-assert reset"]
    #[inline(always)]
    pub fn u0_axi_cfg0_dec_main_div(&self) -> U0_AXI_CFG0_DEC_MAIN_DIV_R {
        U0_AXI_CFG0_DEC_MAIN_DIV_R::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bit 5 - 1: Assert reset, 0: De-assert reset"]
    #[inline(always)]
    pub fn u0_axi_cfg0_dec_hifi4(&self) -> U0_AXI_CFG0_DEC_HIFI4_R {
        U0_AXI_CFG0_DEC_HIFI4_R::new(((self.bits >> 5) & 1) != 0)
    }
    #[doc = "Bit 6 - 1: Assert reset, 0: De-assert reset"]
    #[inline(always)]
    pub fn u0_ddr_axi(&self) -> U0_DDR_AXI_R {
        U0_DDR_AXI_R::new(((self.bits >> 6) & 1) != 0)
    }
    #[doc = "Bit 7 - 1: Assert reset, 0: De-assert reset"]
    #[inline(always)]
    pub fn u0_ddr_osc(&self) -> U0_DDR_OSC_R {
        U0_DDR_OSC_R::new(((self.bits >> 7) & 1) != 0)
    }
    #[doc = "Bit 8 - 1: Assert reset, 0: De-assert reset"]
    #[inline(always)]
    pub fn u0_ddr_apb(&self) -> U0_DDR_APB_R {
        U0_DDR_APB_R::new(((self.bits >> 8) & 1) != 0)
    }
    #[doc = "Bit 9 - 1: Assert reset, 0: De-assert reset"]
    #[inline(always)]
    pub fn u0_isp_top(&self) -> U0_ISP_TOP_R {
        U0_ISP_TOP_R::new(((self.bits >> 9) & 1) != 0)
    }
    #[doc = "Bit 10 - 1: Assert reset, 0: De-assert reset"]
    #[inline(always)]
    pub fn u0_isp_axi(&self) -> U0_ISP_AXI_R {
        U0_ISP_AXI_R::new(((self.bits >> 10) & 1) != 0)
    }
    #[doc = "Bit 11 - 1: Assert reset, 0: De-assert reset"]
    #[inline(always)]
    pub fn u0_vout_src(&self) -> U0_VOUT_SRC_R {
        U0_VOUT_SRC_R::new(((self.bits >> 11) & 1) != 0)
    }
    #[doc = "Bit 12 - 1: Assert reset, 0: De-assert reset"]
    #[inline(always)]
    pub fn u0_codaj12_axi(&self) -> U0_CODAJ12_AXI_R {
        U0_CODAJ12_AXI_R::new(((self.bits >> 12) & 1) != 0)
    }
    #[doc = "Bit 13 - 1: Assert reset, 0: De-assert reset"]
    #[inline(always)]
    pub fn u0_codaj12_core(&self) -> U0_CODAJ12_CORE_R {
        U0_CODAJ12_CORE_R::new(((self.bits >> 13) & 1) != 0)
    }
    #[doc = "Bit 14 - 1: Assert reset, 0: De-assert reset"]
    #[inline(always)]
    pub fn u0_codaj12_apb(&self) -> U0_CODAJ12_APB_R {
        U0_CODAJ12_APB_R::new(((self.bits >> 14) & 1) != 0)
    }
    #[doc = "Bit 15 - 1: Assert reset, 0: De-assert reset"]
    #[inline(always)]
    pub fn u0_wave511_axi(&self) -> U0_WAVE511_AXI_R {
        U0_WAVE511_AXI_R::new(((self.bits >> 15) & 1) != 0)
    }
    #[doc = "Bit 16 - 1: Assert reset, 0: De-assert reset"]
    #[inline(always)]
    pub fn u0_wave511_bpu(&self) -> U0_WAVE511_BPU_R {
        U0_WAVE511_BPU_R::new(((self.bits >> 16) & 1) != 0)
    }
    #[doc = "Bit 17 - 1: Assert reset, 0: De-assert reset"]
    #[inline(always)]
    pub fn u0_wave511_vce(&self) -> U0_WAVE511_VCE_R {
        U0_WAVE511_VCE_R::new(((self.bits >> 17) & 1) != 0)
    }
    #[doc = "Bit 18 - 1: Assert reset, 0: De-assert reset"]
    #[inline(always)]
    pub fn u0_wave511_apb(&self) -> U0_WAVE511_APB_R {
        U0_WAVE511_APB_R::new(((self.bits >> 18) & 1) != 0)
    }
    #[doc = "Bit 19 - 1: Assert reset, 0: De-assert reset"]
    #[inline(always)]
    pub fn u0_vdec_jpg_arb(&self) -> U0_VDEC_JPG_ARB_R {
        U0_VDEC_JPG_ARB_R::new(((self.bits >> 19) & 1) != 0)
    }
    #[doc = "Bit 20 - 1: Assert reset, 0: De-assert reset"]
    #[inline(always)]
    pub fn u0_vdec_jpg_arb_main(&self) -> U0_VDEC_JPG_ARB_MAIN_R {
        U0_VDEC_JPG_ARB_MAIN_R::new(((self.bits >> 20) & 1) != 0)
    }
    #[doc = "Bit 21 - 1: Assert reset, 0: De-assert reset"]
    #[inline(always)]
    pub fn u0_aximem_128b_axi(&self) -> U0_AXIMEM_128B_AXI_R {
        U0_AXIMEM_128B_AXI_R::new(((self.bits >> 21) & 1) != 0)
    }
    #[doc = "Bit 22 - 1: Assert reset, 0: De-assert reset"]
    #[inline(always)]
    pub fn u0_wave420l_axi(&self) -> U0_WAVE420L_AXI_R {
        U0_WAVE420L_AXI_R::new(((self.bits >> 22) & 1) != 0)
    }
    #[doc = "Bit 23 - 1: Assert reset, 0: De-assert reset"]
    #[inline(always)]
    pub fn u0_wave420l_bpu(&self) -> U0_WAVE420L_BPU_R {
        U0_WAVE420L_BPU_R::new(((self.bits >> 23) & 1) != 0)
    }
    #[doc = "Bit 24 - 1: Assert reset, 0: De-assert reset"]
    #[inline(always)]
    pub fn u0_wave420l_vce(&self) -> U0_WAVE420L_VCE_R {
        U0_WAVE420L_VCE_R::new(((self.bits >> 24) & 1) != 0)
    }
    #[doc = "Bit 25 - 1: Assert reset, 0: De-assert reset"]
    #[inline(always)]
    pub fn u0_wave420l_apb(&self) -> U0_WAVE420L_APB_R {
        U0_WAVE420L_APB_R::new(((self.bits >> 25) & 1) != 0)
    }
    #[doc = "Bit 26 - 1: Assert reset, 0: De-assert reset"]
    #[inline(always)]
    pub fn u1_aximem(&self) -> U1_AXIMEM_R {
        U1_AXIMEM_R::new(((self.bits >> 26) & 1) != 0)
    }
    #[doc = "Bit 27 - 1: Assert reset, 0: De-assert reset"]
    #[inline(always)]
    pub fn u2_aximem(&self) -> U2_AXIMEM_R {
        U2_AXIMEM_R::new(((self.bits >> 27) & 1) != 0)
    }
    #[doc = "Bit 28 - 1: Assert reset, 0: De-assert reset"]
    #[inline(always)]
    pub fn u0_intmem_rom_sram(&self) -> U0_INTMEM_ROM_SRAM_R {
        U0_INTMEM_ROM_SRAM_R::new(((self.bits >> 28) & 1) != 0)
    }
    #[doc = "Bit 29 - 1: Assert reset, 0: De-assert reset"]
    #[inline(always)]
    pub fn u0_qspi_ahb(&self) -> U0_QSPI_AHB_R {
        U0_QSPI_AHB_R::new(((self.bits >> 29) & 1) != 0)
    }
    #[doc = "Bit 30 - 1: Assert reset, 0: De-assert reset"]
    #[inline(always)]
    pub fn u0_qspi_apb(&self) -> U0_QSPI_APB_R {
        U0_QSPI_APB_R::new(((self.bits >> 30) & 1) != 0)
    }
    #[doc = "Bit 31 - 1: Assert reset, 0: De-assert reset"]
    #[inline(always)]
    pub fn u0_qspi_ref(&self) -> U0_QSPI_REF_R {
        U0_QSPI_REF_R::new(((self.bits >> 31) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - 1: Assert reset, 0: De-assert reset"]
    #[inline(always)]
    #[must_use]
    pub fn u0_noc_bus_venc_axi(&mut self) -> U0_NOC_BUS_VENC_AXI_W<SYSCRG_RST_STATUS_1_SPEC> {
        U0_NOC_BUS_VENC_AXI_W::new(self, 0)
    }
    #[doc = "Bit 1 - 1: Assert reset, 0: De-assert reset"]
    #[inline(always)]
    #[must_use]
    pub fn u0_axi_cfg1_dec_ahb(&mut self) -> U0_AXI_CFG1_DEC_AHB_W<SYSCRG_RST_STATUS_1_SPEC> {
        U0_AXI_CFG1_DEC_AHB_W::new(self, 1)
    }
    #[doc = "Bit 2 - 1: Assert reset, 0: De-assert reset"]
    #[inline(always)]
    #[must_use]
    pub fn u0_axi_cfg1_dec_main(&mut self) -> U0_AXI_CFG1_DEC_MAIN_W<SYSCRG_RST_STATUS_1_SPEC> {
        U0_AXI_CFG1_DEC_MAIN_W::new(self, 2)
    }
    #[doc = "Bit 3 - 1: Assert reset, 0: De-assert reset"]
    #[inline(always)]
    #[must_use]
    pub fn u0_axi_cfg0_dec_main(&mut self) -> U0_AXI_CFG0_DEC_MAIN_W<SYSCRG_RST_STATUS_1_SPEC> {
        U0_AXI_CFG0_DEC_MAIN_W::new(self, 3)
    }
    #[doc = "Bit 4 - 1: Assert reset, 0: De-assert reset"]
    #[inline(always)]
    #[must_use]
    pub fn u0_axi_cfg0_dec_main_div(
        &mut self,
    ) -> U0_AXI_CFG0_DEC_MAIN_DIV_W<SYSCRG_RST_STATUS_1_SPEC> {
        U0_AXI_CFG0_DEC_MAIN_DIV_W::new(self, 4)
    }
    #[doc = "Bit 5 - 1: Assert reset, 0: De-assert reset"]
    #[inline(always)]
    #[must_use]
    pub fn u0_axi_cfg0_dec_hifi4(&mut self) -> U0_AXI_CFG0_DEC_HIFI4_W<SYSCRG_RST_STATUS_1_SPEC> {
        U0_AXI_CFG0_DEC_HIFI4_W::new(self, 5)
    }
    #[doc = "Bit 6 - 1: Assert reset, 0: De-assert reset"]
    #[inline(always)]
    #[must_use]
    pub fn u0_ddr_axi(&mut self) -> U0_DDR_AXI_W<SYSCRG_RST_STATUS_1_SPEC> {
        U0_DDR_AXI_W::new(self, 6)
    }
    #[doc = "Bit 7 - 1: Assert reset, 0: De-assert reset"]
    #[inline(always)]
    #[must_use]
    pub fn u0_ddr_osc(&mut self) -> U0_DDR_OSC_W<SYSCRG_RST_STATUS_1_SPEC> {
        U0_DDR_OSC_W::new(self, 7)
    }
    #[doc = "Bit 8 - 1: Assert reset, 0: De-assert reset"]
    #[inline(always)]
    #[must_use]
    pub fn u0_ddr_apb(&mut self) -> U0_DDR_APB_W<SYSCRG_RST_STATUS_1_SPEC> {
        U0_DDR_APB_W::new(self, 8)
    }
    #[doc = "Bit 9 - 1: Assert reset, 0: De-assert reset"]
    #[inline(always)]
    #[must_use]
    pub fn u0_isp_top(&mut self) -> U0_ISP_TOP_W<SYSCRG_RST_STATUS_1_SPEC> {
        U0_ISP_TOP_W::new(self, 9)
    }
    #[doc = "Bit 10 - 1: Assert reset, 0: De-assert reset"]
    #[inline(always)]
    #[must_use]
    pub fn u0_isp_axi(&mut self) -> U0_ISP_AXI_W<SYSCRG_RST_STATUS_1_SPEC> {
        U0_ISP_AXI_W::new(self, 10)
    }
    #[doc = "Bit 11 - 1: Assert reset, 0: De-assert reset"]
    #[inline(always)]
    #[must_use]
    pub fn u0_vout_src(&mut self) -> U0_VOUT_SRC_W<SYSCRG_RST_STATUS_1_SPEC> {
        U0_VOUT_SRC_W::new(self, 11)
    }
    #[doc = "Bit 12 - 1: Assert reset, 0: De-assert reset"]
    #[inline(always)]
    #[must_use]
    pub fn u0_codaj12_axi(&mut self) -> U0_CODAJ12_AXI_W<SYSCRG_RST_STATUS_1_SPEC> {
        U0_CODAJ12_AXI_W::new(self, 12)
    }
    #[doc = "Bit 13 - 1: Assert reset, 0: De-assert reset"]
    #[inline(always)]
    #[must_use]
    pub fn u0_codaj12_core(&mut self) -> U0_CODAJ12_CORE_W<SYSCRG_RST_STATUS_1_SPEC> {
        U0_CODAJ12_CORE_W::new(self, 13)
    }
    #[doc = "Bit 14 - 1: Assert reset, 0: De-assert reset"]
    #[inline(always)]
    #[must_use]
    pub fn u0_codaj12_apb(&mut self) -> U0_CODAJ12_APB_W<SYSCRG_RST_STATUS_1_SPEC> {
        U0_CODAJ12_APB_W::new(self, 14)
    }
    #[doc = "Bit 15 - 1: Assert reset, 0: De-assert reset"]
    #[inline(always)]
    #[must_use]
    pub fn u0_wave511_axi(&mut self) -> U0_WAVE511_AXI_W<SYSCRG_RST_STATUS_1_SPEC> {
        U0_WAVE511_AXI_W::new(self, 15)
    }
    #[doc = "Bit 16 - 1: Assert reset, 0: De-assert reset"]
    #[inline(always)]
    #[must_use]
    pub fn u0_wave511_bpu(&mut self) -> U0_WAVE511_BPU_W<SYSCRG_RST_STATUS_1_SPEC> {
        U0_WAVE511_BPU_W::new(self, 16)
    }
    #[doc = "Bit 17 - 1: Assert reset, 0: De-assert reset"]
    #[inline(always)]
    #[must_use]
    pub fn u0_wave511_vce(&mut self) -> U0_WAVE511_VCE_W<SYSCRG_RST_STATUS_1_SPEC> {
        U0_WAVE511_VCE_W::new(self, 17)
    }
    #[doc = "Bit 18 - 1: Assert reset, 0: De-assert reset"]
    #[inline(always)]
    #[must_use]
    pub fn u0_wave511_apb(&mut self) -> U0_WAVE511_APB_W<SYSCRG_RST_STATUS_1_SPEC> {
        U0_WAVE511_APB_W::new(self, 18)
    }
    #[doc = "Bit 19 - 1: Assert reset, 0: De-assert reset"]
    #[inline(always)]
    #[must_use]
    pub fn u0_vdec_jpg_arb(&mut self) -> U0_VDEC_JPG_ARB_W<SYSCRG_RST_STATUS_1_SPEC> {
        U0_VDEC_JPG_ARB_W::new(self, 19)
    }
    #[doc = "Bit 20 - 1: Assert reset, 0: De-assert reset"]
    #[inline(always)]
    #[must_use]
    pub fn u0_vdec_jpg_arb_main(&mut self) -> U0_VDEC_JPG_ARB_MAIN_W<SYSCRG_RST_STATUS_1_SPEC> {
        U0_VDEC_JPG_ARB_MAIN_W::new(self, 20)
    }
    #[doc = "Bit 21 - 1: Assert reset, 0: De-assert reset"]
    #[inline(always)]
    #[must_use]
    pub fn u0_aximem_128b_axi(&mut self) -> U0_AXIMEM_128B_AXI_W<SYSCRG_RST_STATUS_1_SPEC> {
        U0_AXIMEM_128B_AXI_W::new(self, 21)
    }
    #[doc = "Bit 22 - 1: Assert reset, 0: De-assert reset"]
    #[inline(always)]
    #[must_use]
    pub fn u0_wave420l_axi(&mut self) -> U0_WAVE420L_AXI_W<SYSCRG_RST_STATUS_1_SPEC> {
        U0_WAVE420L_AXI_W::new(self, 22)
    }
    #[doc = "Bit 23 - 1: Assert reset, 0: De-assert reset"]
    #[inline(always)]
    #[must_use]
    pub fn u0_wave420l_bpu(&mut self) -> U0_WAVE420L_BPU_W<SYSCRG_RST_STATUS_1_SPEC> {
        U0_WAVE420L_BPU_W::new(self, 23)
    }
    #[doc = "Bit 24 - 1: Assert reset, 0: De-assert reset"]
    #[inline(always)]
    #[must_use]
    pub fn u0_wave420l_vce(&mut self) -> U0_WAVE420L_VCE_W<SYSCRG_RST_STATUS_1_SPEC> {
        U0_WAVE420L_VCE_W::new(self, 24)
    }
    #[doc = "Bit 25 - 1: Assert reset, 0: De-assert reset"]
    #[inline(always)]
    #[must_use]
    pub fn u0_wave420l_apb(&mut self) -> U0_WAVE420L_APB_W<SYSCRG_RST_STATUS_1_SPEC> {
        U0_WAVE420L_APB_W::new(self, 25)
    }
    #[doc = "Bit 26 - 1: Assert reset, 0: De-assert reset"]
    #[inline(always)]
    #[must_use]
    pub fn u1_aximem(&mut self) -> U1_AXIMEM_W<SYSCRG_RST_STATUS_1_SPEC> {
        U1_AXIMEM_W::new(self, 26)
    }
    #[doc = "Bit 27 - 1: Assert reset, 0: De-assert reset"]
    #[inline(always)]
    #[must_use]
    pub fn u2_aximem(&mut self) -> U2_AXIMEM_W<SYSCRG_RST_STATUS_1_SPEC> {
        U2_AXIMEM_W::new(self, 27)
    }
    #[doc = "Bit 28 - 1: Assert reset, 0: De-assert reset"]
    #[inline(always)]
    #[must_use]
    pub fn u0_intmem_rom_sram(&mut self) -> U0_INTMEM_ROM_SRAM_W<SYSCRG_RST_STATUS_1_SPEC> {
        U0_INTMEM_ROM_SRAM_W::new(self, 28)
    }
    #[doc = "Bit 29 - 1: Assert reset, 0: De-assert reset"]
    #[inline(always)]
    #[must_use]
    pub fn u0_qspi_ahb(&mut self) -> U0_QSPI_AHB_W<SYSCRG_RST_STATUS_1_SPEC> {
        U0_QSPI_AHB_W::new(self, 29)
    }
    #[doc = "Bit 30 - 1: Assert reset, 0: De-assert reset"]
    #[inline(always)]
    #[must_use]
    pub fn u0_qspi_apb(&mut self) -> U0_QSPI_APB_W<SYSCRG_RST_STATUS_1_SPEC> {
        U0_QSPI_APB_W::new(self, 30)
    }
    #[doc = "Bit 31 - 1: Assert reset, 0: De-assert reset"]
    #[inline(always)]
    #[must_use]
    pub fn u0_qspi_ref(&mut self) -> U0_QSPI_REF_W<SYSCRG_RST_STATUS_1_SPEC> {
        U0_QSPI_REF_W::new(self, 31)
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
#[doc = "SYSCRG RESET Status 1\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`syscrg_rst_status_1::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`syscrg_rst_status_1::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct SYSCRG_RST_STATUS_1_SPEC;
impl crate::RegisterSpec for SYSCRG_RST_STATUS_1_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`syscrg_rst_status_1::R`](R) reader structure"]
impl crate::Readable for SYSCRG_RST_STATUS_1_SPEC {}
#[doc = "`write(|w| ..)` method takes [`syscrg_rst_status_1::W`](W) writer structure"]
impl crate::Writable for SYSCRG_RST_STATUS_1_SPEC {
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets syscrg_rst_status_1 to value 0"]
impl crate::Resettable for SYSCRG_RST_STATUS_1_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
