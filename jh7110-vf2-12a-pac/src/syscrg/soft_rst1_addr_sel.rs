#[doc = "Register `soft_rst1_addr_sel` reader"]
pub type R = crate::R<SOFT_RST1_ADDR_SEL_SPEC>;
#[doc = "Register `soft_rst1_addr_sel` writer"]
pub type W = crate::W<SOFT_RST1_ADDR_SEL_SPEC>;
#[doc = "Field `rstn_u0_sft7100_noc_bus_reset_venc_axi_n` reader - 1: Assert reset, 0: De-assert reset"]
pub type RSTN_U0_SFT7100_NOC_BUS_RESET_VENC_AXI_N_R = crate::BitReader;
#[doc = "Field `rstn_u0_sft7100_noc_bus_reset_venc_axi_n` writer - 1: Assert reset, 0: De-assert reset"]
pub type RSTN_U0_SFT7100_NOC_BUS_RESET_VENC_AXI_N_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `rstn_u0_axi_cfg1_dec_rstn_ahb` reader - 1: Assert reset, 0: De-assert reset"]
pub type RSTN_U0_AXI_CFG1_DEC_RSTN_AHB_R = crate::BitReader;
#[doc = "Field `rstn_u0_axi_cfg1_dec_rstn_ahb` writer - 1: Assert reset, 0: De-assert reset"]
pub type RSTN_U0_AXI_CFG1_DEC_RSTN_AHB_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `rstn_u0_axi_cfg1_dec_rstn_main` reader - 1: Assert reset, 0: De-assert reset"]
pub type RSTN_U0_AXI_CFG1_DEC_RSTN_MAIN_R = crate::BitReader;
#[doc = "Field `rstn_u0_axi_cfg1_dec_rstn_main` writer - 1: Assert reset, 0: De-assert reset"]
pub type RSTN_U0_AXI_CFG1_DEC_RSTN_MAIN_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `rstn_u0_axi_cfg0_dec_rstn_main` reader - 1: Assert reset, 0: De-assert reset"]
pub type RSTN_U0_AXI_CFG0_DEC_RSTN_MAIN_R = crate::BitReader;
#[doc = "Field `rstn_u0_axi_cfg0_dec_rstn_main` writer - 1: Assert reset, 0: De-assert reset"]
pub type RSTN_U0_AXI_CFG0_DEC_RSTN_MAIN_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `rstn_u0_axi_cfg0_dec_rstn_main_div` reader - 1: Assert reset, 0: De-assert reset"]
pub type RSTN_U0_AXI_CFG0_DEC_RSTN_MAIN_DIV_R = crate::BitReader;
#[doc = "Field `rstn_u0_axi_cfg0_dec_rstn_main_div` writer - 1: Assert reset, 0: De-assert reset"]
pub type RSTN_U0_AXI_CFG0_DEC_RSTN_MAIN_DIV_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `rstn_u0_axi_cfg0_dec_rstn_hifi4` reader - 1: Assert reset, 0: De-assert reset"]
pub type RSTN_U0_AXI_CFG0_DEC_RSTN_HIFI4_R = crate::BitReader;
#[doc = "Field `rstn_u0_axi_cfg0_dec_rstn_hifi4` writer - 1: Assert reset, 0: De-assert reset"]
pub type RSTN_U0_AXI_CFG0_DEC_RSTN_HIFI4_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `rstn_u0_ddr_sft7110_rstn_axi` reader - 1: Assert reset, 0: De-assert reset"]
pub type RSTN_U0_DDR_SFT7110_RSTN_AXI_R = crate::BitReader;
#[doc = "Field `rstn_u0_ddr_sft7110_rstn_axi` writer - 1: Assert reset, 0: De-assert reset"]
pub type RSTN_U0_DDR_SFT7110_RSTN_AXI_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `rstn_u0_ddr_sft7110_rstn_osc` reader - 1: Assert reset, 0: De-assert reset"]
pub type RSTN_U0_DDR_SFT7110_RSTN_OSC_R = crate::BitReader;
#[doc = "Field `rstn_u0_ddr_sft7110_rstn_osc` writer - 1: Assert reset, 0: De-assert reset"]
pub type RSTN_U0_DDR_SFT7110_RSTN_OSC_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `rstn_u0_ddr_sft7110_rstn_apb` reader - 1: Assert reset, 0: De-assert reset"]
pub type RSTN_U0_DDR_SFT7110_RSTN_APB_R = crate::BitReader;
#[doc = "Field `rstn_u0_ddr_sft7110_rstn_apb` writer - 1: Assert reset, 0: De-assert reset"]
pub type RSTN_U0_DDR_SFT7110_RSTN_APB_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `rstn_u0_dom_isp_top_rstn_dom_isp_top_ip_top_reset_n` reader - 1: Assert reset, 0: De-assert reset"]
pub type RSTN_U0_DOM_ISP_TOP_RSTN_DOM_ISP_TOP_IP_TOP_RESET_N_R = crate::BitReader;
#[doc = "Field `rstn_u0_dom_isp_top_rstn_dom_isp_top_ip_top_reset_n` writer - 1: Assert reset, 0: De-assert reset"]
pub type RSTN_U0_DOM_ISP_TOP_RSTN_DOM_ISP_TOP_IP_TOP_RESET_N_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `rstn_u0_dom_isp_top_rstn_dom_isp_top_rstn_isp_axi` reader - 1: Assert reset, 0: De-assert reset"]
pub type RSTN_U0_DOM_ISP_TOP_RSTN_DOM_ISP_TOP_RSTN_ISP_AXI_R = crate::BitReader;
#[doc = "Field `rstn_u0_dom_isp_top_rstn_dom_isp_top_rstn_isp_axi` writer - 1: Assert reset, 0: De-assert reset"]
pub type RSTN_U0_DOM_ISP_TOP_RSTN_DOM_ISP_TOP_RSTN_ISP_AXI_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `rstn_u0_dom_vout_top_rstn_dom_vout_top_rstn_vout_src` reader - 1: Assert reset, 0: De-assert reset"]
pub type RSTN_U0_DOM_VOUT_TOP_RSTN_DOM_VOUT_TOP_RSTN_VOUT_SRC_R = crate::BitReader;
#[doc = "Field `rstn_u0_dom_vout_top_rstn_dom_vout_top_rstn_vout_src` writer - 1: Assert reset, 0: De-assert reset"]
pub type RSTN_U0_DOM_VOUT_TOP_RSTN_DOM_VOUT_TOP_RSTN_VOUT_SRC_W<'a, REG> =
    crate::BitWriter<'a, REG>;
#[doc = "Field `rstn_u0_codaj12_rstn_axi` reader - 1: Assert reset, 0: De-assert reset"]
pub type RSTN_U0_CODAJ12_RSTN_AXI_R = crate::BitReader;
#[doc = "Field `rstn_u0_codaj12_rstn_axi` writer - 1: Assert reset, 0: De-assert reset"]
pub type RSTN_U0_CODAJ12_RSTN_AXI_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `rstn_u0_codaj12_rstn_core` reader - 1: Assert reset, 0: De-assert reset"]
pub type RSTN_U0_CODAJ12_RSTN_CORE_R = crate::BitReader;
#[doc = "Field `rstn_u0_codaj12_rstn_core` writer - 1: Assert reset, 0: De-assert reset"]
pub type RSTN_U0_CODAJ12_RSTN_CORE_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `rstn_u0_codaj12_rstn_apb` reader - 1: Assert reset, 0: De-assert reset"]
pub type RSTN_U0_CODAJ12_RSTN_APB_R = crate::BitReader;
#[doc = "Field `rstn_u0_codaj12_rstn_apb` writer - 1: Assert reset, 0: De-assert reset"]
pub type RSTN_U0_CODAJ12_RSTN_APB_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `rstn_u0_wave511_rstn_axi` reader - 1: Assert reset, 0: De-assert reset"]
pub type RSTN_U0_WAVE511_RSTN_AXI_R = crate::BitReader;
#[doc = "Field `rstn_u0_wave511_rstn_axi` writer - 1: Assert reset, 0: De-assert reset"]
pub type RSTN_U0_WAVE511_RSTN_AXI_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `rstn_u0_wave511_rstn_bpu` reader - 1: Assert reset, 0: De-assert reset"]
pub type RSTN_U0_WAVE511_RSTN_BPU_R = crate::BitReader;
#[doc = "Field `rstn_u0_wave511_rstn_bpu` writer - 1: Assert reset, 0: De-assert reset"]
pub type RSTN_U0_WAVE511_RSTN_BPU_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `rstn_u0_wave511_rstn_vce` reader - 1: Assert reset, 0: De-assert reset"]
pub type RSTN_U0_WAVE511_RSTN_VCE_R = crate::BitReader;
#[doc = "Field `rstn_u0_wave511_rstn_vce` writer - 1: Assert reset, 0: De-assert reset"]
pub type RSTN_U0_WAVE511_RSTN_VCE_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `rstn_u0_wave511_rstn_apb` reader - 1: Assert reset, 0: De-assert reset"]
pub type RSTN_U0_WAVE511_RSTN_APB_R = crate::BitReader;
#[doc = "Field `rstn_u0_wave511_rstn_apb` writer - 1: Assert reset, 0: De-assert reset"]
pub type RSTN_U0_WAVE511_RSTN_APB_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `rstn_u0_vdec_jpg_arb_jpgresetn` reader - 1: Assert reset, 0: De-assert reset"]
pub type RSTN_U0_VDEC_JPG_ARB_JPGRESETN_R = crate::BitReader;
#[doc = "Field `rstn_u0_vdec_jpg_arb_jpgresetn` writer - 1: Assert reset, 0: De-assert reset"]
pub type RSTN_U0_VDEC_JPG_ARB_JPGRESETN_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `rstn_u0_vdec_jpg_arb_mainresetn` reader - 1: Assert reset, 0: De-assert reset"]
pub type RSTN_U0_VDEC_JPG_ARB_MAINRESETN_R = crate::BitReader;
#[doc = "Field `rstn_u0_vdec_jpg_arb_mainresetn` writer - 1: Assert reset, 0: De-assert reset"]
pub type RSTN_U0_VDEC_JPG_ARB_MAINRESETN_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `rstn_u0_aximem_128b_rstn_axi` reader - 1: Assert reset, 0: De-assert reset"]
pub type RSTN_U0_AXIMEM_128B_RSTN_AXI_R = crate::BitReader;
#[doc = "Field `rstn_u0_aximem_128b_rstn_axi` writer - 1: Assert reset, 0: De-assert reset"]
pub type RSTN_U0_AXIMEM_128B_RSTN_AXI_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `rstn_u0_wave420l_rstn_axi` reader - 1: Assert reset, 0: De-assert reset"]
pub type RSTN_U0_WAVE420L_RSTN_AXI_R = crate::BitReader;
#[doc = "Field `rstn_u0_wave420l_rstn_axi` writer - 1: Assert reset, 0: De-assert reset"]
pub type RSTN_U0_WAVE420L_RSTN_AXI_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `rstn_u0_wave420l_rstn_bpu` reader - 1: Assert reset, 0: De-assert reset"]
pub type RSTN_U0_WAVE420L_RSTN_BPU_R = crate::BitReader;
#[doc = "Field `rstn_u0_wave420l_rstn_bpu` writer - 1: Assert reset, 0: De-assert reset"]
pub type RSTN_U0_WAVE420L_RSTN_BPU_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `rstn_u0_wave420l_rstn_vce` reader - 1: Assert reset, 0: De-assert reset"]
pub type RSTN_U0_WAVE420L_RSTN_VCE_R = crate::BitReader;
#[doc = "Field `rstn_u0_wave420l_rstn_vce` writer - 1: Assert reset, 0: De-assert reset"]
pub type RSTN_U0_WAVE420L_RSTN_VCE_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `rstn_u0_wave420l_rstn_apb` reader - 1: Assert reset, 0: De-assert reset"]
pub type RSTN_U0_WAVE420L_RSTN_APB_R = crate::BitReader;
#[doc = "Field `rstn_u0_wave420l_rstn_apb` writer - 1: Assert reset, 0: De-assert reset"]
pub type RSTN_U0_WAVE420L_RSTN_APB_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `rstn_u1_aximem_128b_rstn_axi` reader - 1: Assert reset, 0: De-assert reset"]
pub type RSTN_U1_AXIMEM_128B_RSTN_AXI_R = crate::BitReader;
#[doc = "Field `rstn_u1_aximem_128b_rstn_axi` writer - 1: Assert reset, 0: De-assert reset"]
pub type RSTN_U1_AXIMEM_128B_RSTN_AXI_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `rstn_u2_aximem_128b_rstn_axi` reader - 1: Assert reset, 0: De-assert reset"]
pub type RSTN_U2_AXIMEM_128B_RSTN_AXI_R = crate::BitReader;
#[doc = "Field `rstn_u2_aximem_128b_rstn_axi` writer - 1: Assert reset, 0: De-assert reset"]
pub type RSTN_U2_AXIMEM_128B_RSTN_AXI_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `rstn_u0_intmem_rom_sram_rstn_rom` reader - 1: Assert reset, 0: De-assert reset"]
pub type RSTN_U0_INTMEM_ROM_SRAM_RSTN_ROM_R = crate::BitReader;
#[doc = "Field `rstn_u0_intmem_rom_sram_rstn_rom` writer - 1: Assert reset, 0: De-assert reset"]
pub type RSTN_U0_INTMEM_ROM_SRAM_RSTN_ROM_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `rstn_u0_cdns_qspi_rstn_ahb` reader - 1: Assert reset, 0: De-assert reset"]
pub type RSTN_U0_CDNS_QSPI_RSTN_AHB_R = crate::BitReader;
#[doc = "Field `rstn_u0_cdns_qspi_rstn_ahb` writer - 1: Assert reset, 0: De-assert reset"]
pub type RSTN_U0_CDNS_QSPI_RSTN_AHB_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `rstn_u0_cdns_qspi_rstn_apb` reader - 1: Assert reset, 0: De-assert reset"]
pub type RSTN_U0_CDNS_QSPI_RSTN_APB_R = crate::BitReader;
#[doc = "Field `rstn_u0_cdns_qspi_rstn_apb` writer - 1: Assert reset, 0: De-assert reset"]
pub type RSTN_U0_CDNS_QSPI_RSTN_APB_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `rstn_u0_cdns_qspi_rstn_ref` reader - 1: Assert reset, 0: De-assert reset"]
pub type RSTN_U0_CDNS_QSPI_RSTN_REF_R = crate::BitReader;
#[doc = "Field `rstn_u0_cdns_qspi_rstn_ref` writer - 1: Assert reset, 0: De-assert reset"]
pub type RSTN_U0_CDNS_QSPI_RSTN_REF_W<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bit 0 - 1: Assert reset, 0: De-assert reset"]
    #[inline(always)]
    pub fn rstn_u0_sft7100_noc_bus_reset_venc_axi_n(
        &self,
    ) -> RSTN_U0_SFT7100_NOC_BUS_RESET_VENC_AXI_N_R {
        RSTN_U0_SFT7100_NOC_BUS_RESET_VENC_AXI_N_R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - 1: Assert reset, 0: De-assert reset"]
    #[inline(always)]
    pub fn rstn_u0_axi_cfg1_dec_rstn_ahb(&self) -> RSTN_U0_AXI_CFG1_DEC_RSTN_AHB_R {
        RSTN_U0_AXI_CFG1_DEC_RSTN_AHB_R::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - 1: Assert reset, 0: De-assert reset"]
    #[inline(always)]
    pub fn rstn_u0_axi_cfg1_dec_rstn_main(&self) -> RSTN_U0_AXI_CFG1_DEC_RSTN_MAIN_R {
        RSTN_U0_AXI_CFG1_DEC_RSTN_MAIN_R::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3 - 1: Assert reset, 0: De-assert reset"]
    #[inline(always)]
    pub fn rstn_u0_axi_cfg0_dec_rstn_main(&self) -> RSTN_U0_AXI_CFG0_DEC_RSTN_MAIN_R {
        RSTN_U0_AXI_CFG0_DEC_RSTN_MAIN_R::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bit 4 - 1: Assert reset, 0: De-assert reset"]
    #[inline(always)]
    pub fn rstn_u0_axi_cfg0_dec_rstn_main_div(&self) -> RSTN_U0_AXI_CFG0_DEC_RSTN_MAIN_DIV_R {
        RSTN_U0_AXI_CFG0_DEC_RSTN_MAIN_DIV_R::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bit 5 - 1: Assert reset, 0: De-assert reset"]
    #[inline(always)]
    pub fn rstn_u0_axi_cfg0_dec_rstn_hifi4(&self) -> RSTN_U0_AXI_CFG0_DEC_RSTN_HIFI4_R {
        RSTN_U0_AXI_CFG0_DEC_RSTN_HIFI4_R::new(((self.bits >> 5) & 1) != 0)
    }
    #[doc = "Bit 6 - 1: Assert reset, 0: De-assert reset"]
    #[inline(always)]
    pub fn rstn_u0_ddr_sft7110_rstn_axi(&self) -> RSTN_U0_DDR_SFT7110_RSTN_AXI_R {
        RSTN_U0_DDR_SFT7110_RSTN_AXI_R::new(((self.bits >> 6) & 1) != 0)
    }
    #[doc = "Bit 7 - 1: Assert reset, 0: De-assert reset"]
    #[inline(always)]
    pub fn rstn_u0_ddr_sft7110_rstn_osc(&self) -> RSTN_U0_DDR_SFT7110_RSTN_OSC_R {
        RSTN_U0_DDR_SFT7110_RSTN_OSC_R::new(((self.bits >> 7) & 1) != 0)
    }
    #[doc = "Bit 8 - 1: Assert reset, 0: De-assert reset"]
    #[inline(always)]
    pub fn rstn_u0_ddr_sft7110_rstn_apb(&self) -> RSTN_U0_DDR_SFT7110_RSTN_APB_R {
        RSTN_U0_DDR_SFT7110_RSTN_APB_R::new(((self.bits >> 8) & 1) != 0)
    }
    #[doc = "Bit 9 - 1: Assert reset, 0: De-assert reset"]
    #[inline(always)]
    pub fn rstn_u0_dom_isp_top_rstn_dom_isp_top_ip_top_reset_n(
        &self,
    ) -> RSTN_U0_DOM_ISP_TOP_RSTN_DOM_ISP_TOP_IP_TOP_RESET_N_R {
        RSTN_U0_DOM_ISP_TOP_RSTN_DOM_ISP_TOP_IP_TOP_RESET_N_R::new(((self.bits >> 9) & 1) != 0)
    }
    #[doc = "Bit 10 - 1: Assert reset, 0: De-assert reset"]
    #[inline(always)]
    pub fn rstn_u0_dom_isp_top_rstn_dom_isp_top_rstn_isp_axi(
        &self,
    ) -> RSTN_U0_DOM_ISP_TOP_RSTN_DOM_ISP_TOP_RSTN_ISP_AXI_R {
        RSTN_U0_DOM_ISP_TOP_RSTN_DOM_ISP_TOP_RSTN_ISP_AXI_R::new(((self.bits >> 10) & 1) != 0)
    }
    #[doc = "Bit 11 - 1: Assert reset, 0: De-assert reset"]
    #[inline(always)]
    pub fn rstn_u0_dom_vout_top_rstn_dom_vout_top_rstn_vout_src(
        &self,
    ) -> RSTN_U0_DOM_VOUT_TOP_RSTN_DOM_VOUT_TOP_RSTN_VOUT_SRC_R {
        RSTN_U0_DOM_VOUT_TOP_RSTN_DOM_VOUT_TOP_RSTN_VOUT_SRC_R::new(((self.bits >> 11) & 1) != 0)
    }
    #[doc = "Bit 12 - 1: Assert reset, 0: De-assert reset"]
    #[inline(always)]
    pub fn rstn_u0_codaj12_rstn_axi(&self) -> RSTN_U0_CODAJ12_RSTN_AXI_R {
        RSTN_U0_CODAJ12_RSTN_AXI_R::new(((self.bits >> 12) & 1) != 0)
    }
    #[doc = "Bit 13 - 1: Assert reset, 0: De-assert reset"]
    #[inline(always)]
    pub fn rstn_u0_codaj12_rstn_core(&self) -> RSTN_U0_CODAJ12_RSTN_CORE_R {
        RSTN_U0_CODAJ12_RSTN_CORE_R::new(((self.bits >> 13) & 1) != 0)
    }
    #[doc = "Bit 14 - 1: Assert reset, 0: De-assert reset"]
    #[inline(always)]
    pub fn rstn_u0_codaj12_rstn_apb(&self) -> RSTN_U0_CODAJ12_RSTN_APB_R {
        RSTN_U0_CODAJ12_RSTN_APB_R::new(((self.bits >> 14) & 1) != 0)
    }
    #[doc = "Bit 15 - 1: Assert reset, 0: De-assert reset"]
    #[inline(always)]
    pub fn rstn_u0_wave511_rstn_axi(&self) -> RSTN_U0_WAVE511_RSTN_AXI_R {
        RSTN_U0_WAVE511_RSTN_AXI_R::new(((self.bits >> 15) & 1) != 0)
    }
    #[doc = "Bit 16 - 1: Assert reset, 0: De-assert reset"]
    #[inline(always)]
    pub fn rstn_u0_wave511_rstn_bpu(&self) -> RSTN_U0_WAVE511_RSTN_BPU_R {
        RSTN_U0_WAVE511_RSTN_BPU_R::new(((self.bits >> 16) & 1) != 0)
    }
    #[doc = "Bit 17 - 1: Assert reset, 0: De-assert reset"]
    #[inline(always)]
    pub fn rstn_u0_wave511_rstn_vce(&self) -> RSTN_U0_WAVE511_RSTN_VCE_R {
        RSTN_U0_WAVE511_RSTN_VCE_R::new(((self.bits >> 17) & 1) != 0)
    }
    #[doc = "Bit 18 - 1: Assert reset, 0: De-assert reset"]
    #[inline(always)]
    pub fn rstn_u0_wave511_rstn_apb(&self) -> RSTN_U0_WAVE511_RSTN_APB_R {
        RSTN_U0_WAVE511_RSTN_APB_R::new(((self.bits >> 18) & 1) != 0)
    }
    #[doc = "Bit 19 - 1: Assert reset, 0: De-assert reset"]
    #[inline(always)]
    pub fn rstn_u0_vdec_jpg_arb_jpgresetn(&self) -> RSTN_U0_VDEC_JPG_ARB_JPGRESETN_R {
        RSTN_U0_VDEC_JPG_ARB_JPGRESETN_R::new(((self.bits >> 19) & 1) != 0)
    }
    #[doc = "Bit 20 - 1: Assert reset, 0: De-assert reset"]
    #[inline(always)]
    pub fn rstn_u0_vdec_jpg_arb_mainresetn(&self) -> RSTN_U0_VDEC_JPG_ARB_MAINRESETN_R {
        RSTN_U0_VDEC_JPG_ARB_MAINRESETN_R::new(((self.bits >> 20) & 1) != 0)
    }
    #[doc = "Bit 21 - 1: Assert reset, 0: De-assert reset"]
    #[inline(always)]
    pub fn rstn_u0_aximem_128b_rstn_axi(&self) -> RSTN_U0_AXIMEM_128B_RSTN_AXI_R {
        RSTN_U0_AXIMEM_128B_RSTN_AXI_R::new(((self.bits >> 21) & 1) != 0)
    }
    #[doc = "Bit 22 - 1: Assert reset, 0: De-assert reset"]
    #[inline(always)]
    pub fn rstn_u0_wave420l_rstn_axi(&self) -> RSTN_U0_WAVE420L_RSTN_AXI_R {
        RSTN_U0_WAVE420L_RSTN_AXI_R::new(((self.bits >> 22) & 1) != 0)
    }
    #[doc = "Bit 23 - 1: Assert reset, 0: De-assert reset"]
    #[inline(always)]
    pub fn rstn_u0_wave420l_rstn_bpu(&self) -> RSTN_U0_WAVE420L_RSTN_BPU_R {
        RSTN_U0_WAVE420L_RSTN_BPU_R::new(((self.bits >> 23) & 1) != 0)
    }
    #[doc = "Bit 24 - 1: Assert reset, 0: De-assert reset"]
    #[inline(always)]
    pub fn rstn_u0_wave420l_rstn_vce(&self) -> RSTN_U0_WAVE420L_RSTN_VCE_R {
        RSTN_U0_WAVE420L_RSTN_VCE_R::new(((self.bits >> 24) & 1) != 0)
    }
    #[doc = "Bit 25 - 1: Assert reset, 0: De-assert reset"]
    #[inline(always)]
    pub fn rstn_u0_wave420l_rstn_apb(&self) -> RSTN_U0_WAVE420L_RSTN_APB_R {
        RSTN_U0_WAVE420L_RSTN_APB_R::new(((self.bits >> 25) & 1) != 0)
    }
    #[doc = "Bit 26 - 1: Assert reset, 0: De-assert reset"]
    #[inline(always)]
    pub fn rstn_u1_aximem_128b_rstn_axi(&self) -> RSTN_U1_AXIMEM_128B_RSTN_AXI_R {
        RSTN_U1_AXIMEM_128B_RSTN_AXI_R::new(((self.bits >> 26) & 1) != 0)
    }
    #[doc = "Bit 27 - 1: Assert reset, 0: De-assert reset"]
    #[inline(always)]
    pub fn rstn_u2_aximem_128b_rstn_axi(&self) -> RSTN_U2_AXIMEM_128B_RSTN_AXI_R {
        RSTN_U2_AXIMEM_128B_RSTN_AXI_R::new(((self.bits >> 27) & 1) != 0)
    }
    #[doc = "Bit 28 - 1: Assert reset, 0: De-assert reset"]
    #[inline(always)]
    pub fn rstn_u0_intmem_rom_sram_rstn_rom(&self) -> RSTN_U0_INTMEM_ROM_SRAM_RSTN_ROM_R {
        RSTN_U0_INTMEM_ROM_SRAM_RSTN_ROM_R::new(((self.bits >> 28) & 1) != 0)
    }
    #[doc = "Bit 29 - 1: Assert reset, 0: De-assert reset"]
    #[inline(always)]
    pub fn rstn_u0_cdns_qspi_rstn_ahb(&self) -> RSTN_U0_CDNS_QSPI_RSTN_AHB_R {
        RSTN_U0_CDNS_QSPI_RSTN_AHB_R::new(((self.bits >> 29) & 1) != 0)
    }
    #[doc = "Bit 30 - 1: Assert reset, 0: De-assert reset"]
    #[inline(always)]
    pub fn rstn_u0_cdns_qspi_rstn_apb(&self) -> RSTN_U0_CDNS_QSPI_RSTN_APB_R {
        RSTN_U0_CDNS_QSPI_RSTN_APB_R::new(((self.bits >> 30) & 1) != 0)
    }
    #[doc = "Bit 31 - 1: Assert reset, 0: De-assert reset"]
    #[inline(always)]
    pub fn rstn_u0_cdns_qspi_rstn_ref(&self) -> RSTN_U0_CDNS_QSPI_RSTN_REF_R {
        RSTN_U0_CDNS_QSPI_RSTN_REF_R::new(((self.bits >> 31) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - 1: Assert reset, 0: De-assert reset"]
    #[inline(always)]
    #[must_use]
    pub fn rstn_u0_sft7100_noc_bus_reset_venc_axi_n(
        &mut self,
    ) -> RSTN_U0_SFT7100_NOC_BUS_RESET_VENC_AXI_N_W<SOFT_RST1_ADDR_SEL_SPEC> {
        RSTN_U0_SFT7100_NOC_BUS_RESET_VENC_AXI_N_W::new(self, 0)
    }
    #[doc = "Bit 1 - 1: Assert reset, 0: De-assert reset"]
    #[inline(always)]
    #[must_use]
    pub fn rstn_u0_axi_cfg1_dec_rstn_ahb(
        &mut self,
    ) -> RSTN_U0_AXI_CFG1_DEC_RSTN_AHB_W<SOFT_RST1_ADDR_SEL_SPEC> {
        RSTN_U0_AXI_CFG1_DEC_RSTN_AHB_W::new(self, 1)
    }
    #[doc = "Bit 2 - 1: Assert reset, 0: De-assert reset"]
    #[inline(always)]
    #[must_use]
    pub fn rstn_u0_axi_cfg1_dec_rstn_main(
        &mut self,
    ) -> RSTN_U0_AXI_CFG1_DEC_RSTN_MAIN_W<SOFT_RST1_ADDR_SEL_SPEC> {
        RSTN_U0_AXI_CFG1_DEC_RSTN_MAIN_W::new(self, 2)
    }
    #[doc = "Bit 3 - 1: Assert reset, 0: De-assert reset"]
    #[inline(always)]
    #[must_use]
    pub fn rstn_u0_axi_cfg0_dec_rstn_main(
        &mut self,
    ) -> RSTN_U0_AXI_CFG0_DEC_RSTN_MAIN_W<SOFT_RST1_ADDR_SEL_SPEC> {
        RSTN_U0_AXI_CFG0_DEC_RSTN_MAIN_W::new(self, 3)
    }
    #[doc = "Bit 4 - 1: Assert reset, 0: De-assert reset"]
    #[inline(always)]
    #[must_use]
    pub fn rstn_u0_axi_cfg0_dec_rstn_main_div(
        &mut self,
    ) -> RSTN_U0_AXI_CFG0_DEC_RSTN_MAIN_DIV_W<SOFT_RST1_ADDR_SEL_SPEC> {
        RSTN_U0_AXI_CFG0_DEC_RSTN_MAIN_DIV_W::new(self, 4)
    }
    #[doc = "Bit 5 - 1: Assert reset, 0: De-assert reset"]
    #[inline(always)]
    #[must_use]
    pub fn rstn_u0_axi_cfg0_dec_rstn_hifi4(
        &mut self,
    ) -> RSTN_U0_AXI_CFG0_DEC_RSTN_HIFI4_W<SOFT_RST1_ADDR_SEL_SPEC> {
        RSTN_U0_AXI_CFG0_DEC_RSTN_HIFI4_W::new(self, 5)
    }
    #[doc = "Bit 6 - 1: Assert reset, 0: De-assert reset"]
    #[inline(always)]
    #[must_use]
    pub fn rstn_u0_ddr_sft7110_rstn_axi(
        &mut self,
    ) -> RSTN_U0_DDR_SFT7110_RSTN_AXI_W<SOFT_RST1_ADDR_SEL_SPEC> {
        RSTN_U0_DDR_SFT7110_RSTN_AXI_W::new(self, 6)
    }
    #[doc = "Bit 7 - 1: Assert reset, 0: De-assert reset"]
    #[inline(always)]
    #[must_use]
    pub fn rstn_u0_ddr_sft7110_rstn_osc(
        &mut self,
    ) -> RSTN_U0_DDR_SFT7110_RSTN_OSC_W<SOFT_RST1_ADDR_SEL_SPEC> {
        RSTN_U0_DDR_SFT7110_RSTN_OSC_W::new(self, 7)
    }
    #[doc = "Bit 8 - 1: Assert reset, 0: De-assert reset"]
    #[inline(always)]
    #[must_use]
    pub fn rstn_u0_ddr_sft7110_rstn_apb(
        &mut self,
    ) -> RSTN_U0_DDR_SFT7110_RSTN_APB_W<SOFT_RST1_ADDR_SEL_SPEC> {
        RSTN_U0_DDR_SFT7110_RSTN_APB_W::new(self, 8)
    }
    #[doc = "Bit 9 - 1: Assert reset, 0: De-assert reset"]
    #[inline(always)]
    #[must_use]
    pub fn rstn_u0_dom_isp_top_rstn_dom_isp_top_ip_top_reset_n(
        &mut self,
    ) -> RSTN_U0_DOM_ISP_TOP_RSTN_DOM_ISP_TOP_IP_TOP_RESET_N_W<SOFT_RST1_ADDR_SEL_SPEC> {
        RSTN_U0_DOM_ISP_TOP_RSTN_DOM_ISP_TOP_IP_TOP_RESET_N_W::new(self, 9)
    }
    #[doc = "Bit 10 - 1: Assert reset, 0: De-assert reset"]
    #[inline(always)]
    #[must_use]
    pub fn rstn_u0_dom_isp_top_rstn_dom_isp_top_rstn_isp_axi(
        &mut self,
    ) -> RSTN_U0_DOM_ISP_TOP_RSTN_DOM_ISP_TOP_RSTN_ISP_AXI_W<SOFT_RST1_ADDR_SEL_SPEC> {
        RSTN_U0_DOM_ISP_TOP_RSTN_DOM_ISP_TOP_RSTN_ISP_AXI_W::new(self, 10)
    }
    #[doc = "Bit 11 - 1: Assert reset, 0: De-assert reset"]
    #[inline(always)]
    #[must_use]
    pub fn rstn_u0_dom_vout_top_rstn_dom_vout_top_rstn_vout_src(
        &mut self,
    ) -> RSTN_U0_DOM_VOUT_TOP_RSTN_DOM_VOUT_TOP_RSTN_VOUT_SRC_W<SOFT_RST1_ADDR_SEL_SPEC> {
        RSTN_U0_DOM_VOUT_TOP_RSTN_DOM_VOUT_TOP_RSTN_VOUT_SRC_W::new(self, 11)
    }
    #[doc = "Bit 12 - 1: Assert reset, 0: De-assert reset"]
    #[inline(always)]
    #[must_use]
    pub fn rstn_u0_codaj12_rstn_axi(
        &mut self,
    ) -> RSTN_U0_CODAJ12_RSTN_AXI_W<SOFT_RST1_ADDR_SEL_SPEC> {
        RSTN_U0_CODAJ12_RSTN_AXI_W::new(self, 12)
    }
    #[doc = "Bit 13 - 1: Assert reset, 0: De-assert reset"]
    #[inline(always)]
    #[must_use]
    pub fn rstn_u0_codaj12_rstn_core(
        &mut self,
    ) -> RSTN_U0_CODAJ12_RSTN_CORE_W<SOFT_RST1_ADDR_SEL_SPEC> {
        RSTN_U0_CODAJ12_RSTN_CORE_W::new(self, 13)
    }
    #[doc = "Bit 14 - 1: Assert reset, 0: De-assert reset"]
    #[inline(always)]
    #[must_use]
    pub fn rstn_u0_codaj12_rstn_apb(
        &mut self,
    ) -> RSTN_U0_CODAJ12_RSTN_APB_W<SOFT_RST1_ADDR_SEL_SPEC> {
        RSTN_U0_CODAJ12_RSTN_APB_W::new(self, 14)
    }
    #[doc = "Bit 15 - 1: Assert reset, 0: De-assert reset"]
    #[inline(always)]
    #[must_use]
    pub fn rstn_u0_wave511_rstn_axi(
        &mut self,
    ) -> RSTN_U0_WAVE511_RSTN_AXI_W<SOFT_RST1_ADDR_SEL_SPEC> {
        RSTN_U0_WAVE511_RSTN_AXI_W::new(self, 15)
    }
    #[doc = "Bit 16 - 1: Assert reset, 0: De-assert reset"]
    #[inline(always)]
    #[must_use]
    pub fn rstn_u0_wave511_rstn_bpu(
        &mut self,
    ) -> RSTN_U0_WAVE511_RSTN_BPU_W<SOFT_RST1_ADDR_SEL_SPEC> {
        RSTN_U0_WAVE511_RSTN_BPU_W::new(self, 16)
    }
    #[doc = "Bit 17 - 1: Assert reset, 0: De-assert reset"]
    #[inline(always)]
    #[must_use]
    pub fn rstn_u0_wave511_rstn_vce(
        &mut self,
    ) -> RSTN_U0_WAVE511_RSTN_VCE_W<SOFT_RST1_ADDR_SEL_SPEC> {
        RSTN_U0_WAVE511_RSTN_VCE_W::new(self, 17)
    }
    #[doc = "Bit 18 - 1: Assert reset, 0: De-assert reset"]
    #[inline(always)]
    #[must_use]
    pub fn rstn_u0_wave511_rstn_apb(
        &mut self,
    ) -> RSTN_U0_WAVE511_RSTN_APB_W<SOFT_RST1_ADDR_SEL_SPEC> {
        RSTN_U0_WAVE511_RSTN_APB_W::new(self, 18)
    }
    #[doc = "Bit 19 - 1: Assert reset, 0: De-assert reset"]
    #[inline(always)]
    #[must_use]
    pub fn rstn_u0_vdec_jpg_arb_jpgresetn(
        &mut self,
    ) -> RSTN_U0_VDEC_JPG_ARB_JPGRESETN_W<SOFT_RST1_ADDR_SEL_SPEC> {
        RSTN_U0_VDEC_JPG_ARB_JPGRESETN_W::new(self, 19)
    }
    #[doc = "Bit 20 - 1: Assert reset, 0: De-assert reset"]
    #[inline(always)]
    #[must_use]
    pub fn rstn_u0_vdec_jpg_arb_mainresetn(
        &mut self,
    ) -> RSTN_U0_VDEC_JPG_ARB_MAINRESETN_W<SOFT_RST1_ADDR_SEL_SPEC> {
        RSTN_U0_VDEC_JPG_ARB_MAINRESETN_W::new(self, 20)
    }
    #[doc = "Bit 21 - 1: Assert reset, 0: De-assert reset"]
    #[inline(always)]
    #[must_use]
    pub fn rstn_u0_aximem_128b_rstn_axi(
        &mut self,
    ) -> RSTN_U0_AXIMEM_128B_RSTN_AXI_W<SOFT_RST1_ADDR_SEL_SPEC> {
        RSTN_U0_AXIMEM_128B_RSTN_AXI_W::new(self, 21)
    }
    #[doc = "Bit 22 - 1: Assert reset, 0: De-assert reset"]
    #[inline(always)]
    #[must_use]
    pub fn rstn_u0_wave420l_rstn_axi(
        &mut self,
    ) -> RSTN_U0_WAVE420L_RSTN_AXI_W<SOFT_RST1_ADDR_SEL_SPEC> {
        RSTN_U0_WAVE420L_RSTN_AXI_W::new(self, 22)
    }
    #[doc = "Bit 23 - 1: Assert reset, 0: De-assert reset"]
    #[inline(always)]
    #[must_use]
    pub fn rstn_u0_wave420l_rstn_bpu(
        &mut self,
    ) -> RSTN_U0_WAVE420L_RSTN_BPU_W<SOFT_RST1_ADDR_SEL_SPEC> {
        RSTN_U0_WAVE420L_RSTN_BPU_W::new(self, 23)
    }
    #[doc = "Bit 24 - 1: Assert reset, 0: De-assert reset"]
    #[inline(always)]
    #[must_use]
    pub fn rstn_u0_wave420l_rstn_vce(
        &mut self,
    ) -> RSTN_U0_WAVE420L_RSTN_VCE_W<SOFT_RST1_ADDR_SEL_SPEC> {
        RSTN_U0_WAVE420L_RSTN_VCE_W::new(self, 24)
    }
    #[doc = "Bit 25 - 1: Assert reset, 0: De-assert reset"]
    #[inline(always)]
    #[must_use]
    pub fn rstn_u0_wave420l_rstn_apb(
        &mut self,
    ) -> RSTN_U0_WAVE420L_RSTN_APB_W<SOFT_RST1_ADDR_SEL_SPEC> {
        RSTN_U0_WAVE420L_RSTN_APB_W::new(self, 25)
    }
    #[doc = "Bit 26 - 1: Assert reset, 0: De-assert reset"]
    #[inline(always)]
    #[must_use]
    pub fn rstn_u1_aximem_128b_rstn_axi(
        &mut self,
    ) -> RSTN_U1_AXIMEM_128B_RSTN_AXI_W<SOFT_RST1_ADDR_SEL_SPEC> {
        RSTN_U1_AXIMEM_128B_RSTN_AXI_W::new(self, 26)
    }
    #[doc = "Bit 27 - 1: Assert reset, 0: De-assert reset"]
    #[inline(always)]
    #[must_use]
    pub fn rstn_u2_aximem_128b_rstn_axi(
        &mut self,
    ) -> RSTN_U2_AXIMEM_128B_RSTN_AXI_W<SOFT_RST1_ADDR_SEL_SPEC> {
        RSTN_U2_AXIMEM_128B_RSTN_AXI_W::new(self, 27)
    }
    #[doc = "Bit 28 - 1: Assert reset, 0: De-assert reset"]
    #[inline(always)]
    #[must_use]
    pub fn rstn_u0_intmem_rom_sram_rstn_rom(
        &mut self,
    ) -> RSTN_U0_INTMEM_ROM_SRAM_RSTN_ROM_W<SOFT_RST1_ADDR_SEL_SPEC> {
        RSTN_U0_INTMEM_ROM_SRAM_RSTN_ROM_W::new(self, 28)
    }
    #[doc = "Bit 29 - 1: Assert reset, 0: De-assert reset"]
    #[inline(always)]
    #[must_use]
    pub fn rstn_u0_cdns_qspi_rstn_ahb(
        &mut self,
    ) -> RSTN_U0_CDNS_QSPI_RSTN_AHB_W<SOFT_RST1_ADDR_SEL_SPEC> {
        RSTN_U0_CDNS_QSPI_RSTN_AHB_W::new(self, 29)
    }
    #[doc = "Bit 30 - 1: Assert reset, 0: De-assert reset"]
    #[inline(always)]
    #[must_use]
    pub fn rstn_u0_cdns_qspi_rstn_apb(
        &mut self,
    ) -> RSTN_U0_CDNS_QSPI_RSTN_APB_W<SOFT_RST1_ADDR_SEL_SPEC> {
        RSTN_U0_CDNS_QSPI_RSTN_APB_W::new(self, 30)
    }
    #[doc = "Bit 31 - 1: Assert reset, 0: De-assert reset"]
    #[inline(always)]
    #[must_use]
    pub fn rstn_u0_cdns_qspi_rstn_ref(
        &mut self,
    ) -> RSTN_U0_CDNS_QSPI_RSTN_REF_W<SOFT_RST1_ADDR_SEL_SPEC> {
        RSTN_U0_CDNS_QSPI_RSTN_REF_W::new(self, 31)
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
#[doc = "Software RESET 1 Address Selector\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`soft_rst1_addr_sel::R`](R).  You can [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`soft_rst1_addr_sel::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct SOFT_RST1_ADDR_SEL_SPEC;
impl crate::RegisterSpec for SOFT_RST1_ADDR_SEL_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`soft_rst1_addr_sel::R`](R) reader structure"]
impl crate::Readable for SOFT_RST1_ADDR_SEL_SPEC {}
#[doc = "`write(|w| ..)` method takes [`soft_rst1_addr_sel::W`](W) writer structure"]
impl crate::Writable for SOFT_RST1_ADDR_SEL_SPEC {
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
