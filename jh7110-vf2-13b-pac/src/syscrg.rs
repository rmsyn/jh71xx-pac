#[doc = r"Register block"]
#[repr(C)]
pub struct RegisterBlock {
    clk_cpu_root: CLK_CPU_ROOT,
    clk_cpu_core: CLK_CPU_CORE,
    clk_cpu_bus: CLK_CPU_BUS,
    clk_gpu_root: CLK_GPU_ROOT,
    clk_peripheral_root: CLK_PERIPHERAL_ROOT,
    clk_bus_root: CLK_BUS_ROOT,
    clk_nocstg_bus: CLK_NOCSTG_BUS,
    clk_axi_cfg0: CLK_AXI_CFG0,
    clk_stg_axiahb: CLK_STG_AXIAHB,
    clk_ahb0: CLK_AHB0,
    clk_ahb1: CLK_AHB1,
    clk_apb_bus: CLK_APB_BUS,
    clk_apb0: CLK_APB0,
    clk_pll0_div2: CLK_PLL0_DIV2,
    clk_pll1_div2: CLK_PLL1_DIV2,
    clk_pll2_div2: CLK_PLL2_DIV2,
    clk_audio_root: CLK_AUDIO_ROOT,
    clk_mclk_inner: CLK_MCLK_INNER,
    clk_mclk: CLK_MCLK,
    clk_mclk_out: CLK_MCLK_OUT,
    clk_isp_2x: CLK_ISP_2X,
    clk_isp_axi: CLK_ISP_AXI,
    clk_gclk0: CLK_GCLK0,
    clk_gclk1: CLK_GCLK1,
    clk_gclk2: CLK_GCLK2,
    clk_u7mc_core0: CLK_U7MC_CORE0,
    clk_u7mc_core1: CLK_U7MC_CORE1,
    clk_u7mc_core2: CLK_U7MC_CORE2,
    clk_u7mc_core3: CLK_U7MC_CORE3,
    clk_u7mc_core4: CLK_U7MC_CORE4,
    clk_u7mc_debug: CLK_U7MC_DEBUG,
    u7mc_rtc_toggle: U7MC_RTC_TOGGLE,
    clk_u7mc_trace0: CLK_U7MC_TRACE0,
    clk_u7mc_trace1: CLK_U7MC_TRACE1,
    clk_u7mc_trace2: CLK_U7MC_TRACE2,
    clk_u7mc_trace3: CLK_U7MC_TRACE3,
    clk_u7mc_trace4: CLK_U7MC_TRACE4,
    clk_u7mc_trace_com: CLK_U7MC_TRACE_COM,
    clk_u0_sft7110_noc_bus_clk_cpu_axi: CLK_U0_SFT7110_NOC_BUS_CLK_CPU_AXI,
    clk_u0_sft7110_noc_bus_clk_axicfg0_axi: CLK_U0_SFT7110_NOC_BUS_CLK_AXICFG0_AXI,
    clk_osc_div2: CLK_OSC_DIV2,
    clk_pll1_div4: CLK_PLL1_DIV4,
    clk_pll1_div8: CLK_PLL1_DIV8,
    clk_ddr_bus: CLK_DDR_BUS,
    clk_u0_ddr_sft7110_clk_axi: CLK_U0_DDR_SFT7110_CLK_AXI,
    clk_gpu_core: CLK_GPU_CORE,
    clk_u0_img_gpu_core_clk: CLK_U0_IMG_GPU_CORE_CLK,
    clk_u0_img_gpu_sys_clk: CLK_U0_IMG_GPU_SYS_CLK,
    clk_u0_img_gpu_clk_apb: CLK_U0_IMG_GPU_CLK_APB,
    clk_u0_gpu_rtc_toggle: CLK_U0_GPU_RTC_TOGGLE,
    clk_u0_sft7110_noc_bus_clk_gpu_axi: CLK_U0_SFT7110_NOC_BUS_CLK_GPU_AXI,
    clk_u0_dom_isp_top_clk_dom_isp_top_clk_ispcore_2x:
        CLK_U0_DOM_ISP_TOP_CLK_DOM_ISP_TOP_CLK_ISPCORE_2X,
    clk_u0_dom_isp_top_clk_dom_isp_top_clk_isp_axi: CLK_U0_DOM_ISP_TOP_CLK_DOM_ISP_TOP_CLK_ISP_AXI,
    clk_u0_sft7110_noc_bux_clk_isp_axi: CLK_U0_SFT7110_NOC_BUX_CLK_ISP_AXI,
    clk_hifi4_core: CLK_HIFI4_CORE,
    clk_hifi4_axi: CLK_HIFI4_AXI,
    clk_u0_axi_cfg1_dec_clk_main: CLK_U0_AXI_CFG1_DEC_CLK_MAIN,
    clk_u0_axi_cfg1_dec_clk_ahb: CLK_U0_AXI_CFG1_DEC_CLK_AHB,
    clk_u0_dom_vout_top_clk_dom_vout_top_clk_vout_src:
        CLK_U0_DOM_VOUT_TOP_CLK_DOM_VOUT_TOP_CLK_VOUT_SRC,
    clk_vout_axi_divcfg: CLK_VOUT_AXI_DIVCFG,
    clk_noc_display_axi: CLK_NOC_DISPLAY_AXI,
    clk_vout_ahb: CLK_VOUT_AHB,
    clk_vout_axi_icg: CLK_VOUT_AXI_ICG,
    clk_vout_hdmi_tx0_mclk: CLK_VOUT_HDMI_TX0_MCLK,
    clk_vout_mipi_phy: CLK_VOUT_MIPI_PHY,
    clk_jpeg_codec_axi: CLK_JPEG_CODEC_AXI,
    clk_codaj12_axi: CLK_CODAJ12_AXI,
    clk_codaj12_core: CLK_CODAJ12_CORE,
    clk_codaj12_apb: CLK_CODAJ12_APB,
    clk_vdec_axi: CLK_VDEC_AXI,
    clk_wave511_axi: CLK_WAVE511_AXI,
    clk_wave511_bpu: CLK_WAVE511_BPU,
    clk_wave511_vce: CLK_WAVE511_VCE,
    clk_wave511_apb: CLK_WAVE511_APB,
    clk_wave511_jpg_arb: CLK_WAVE511_JPG_ARB,
    clk_wave511_jpg_main: CLK_WAVE511_JPG_MAIN,
    clk_noc_vdec_axi: CLK_NOC_VDEC_AXI,
    clk_venc_axi: CLK_VENC_AXI,
    clk_wave420l_axi: CLK_WAVE420L_AXI,
    clk_wave420l_bpu: CLK_WAVE420L_BPU,
    clk_wave420l_vce: CLK_WAVE420L_VCE,
    clk_wave420l_apb: CLK_WAVE420L_APB,
    clk_noc_venc_axi: CLK_NOC_VENC_AXI,
    clk_axi_cfg0_dec_main_div: CLK_AXI_CFG0_DEC_MAIN_DIV,
    clk_axi_cfg0_dec_main: CLK_AXI_CFG0_DEC_MAIN,
    clk_axi_cfg0_dec_hifi4: CLK_AXI_CFG0_DEC_HIFI4,
    clk_aximem_128b_axi: CLK_AXIMEM_128B_AXI,
    clk_qspi_ahb: CLK_QSPI_AHB,
    clk_qspi_apb: CLK_QSPI_APB,
    clk_qspi_ref_src: CLK_QSPI_REF_SRC,
    clk_qspi_ref: CLK_QSPI_REF,
    clk_u0_sd_ahb: CLK_U0_SD_AHB,
    clk_u1_sd_ahb: CLK_U1_SD_AHB,
    clk_u0_sd_card: CLK_U0_SD_CARD,
    clk_u1_sd_card: CLK_U1_SD_CARD,
    clk_usb_125m: CLK_USB_125M,
    clk_noc_stg_axi: CLK_NOC_STG_AXI,
    clk_gmac5_axi64_ahb: CLK_GMAC5_AXI64_AHB,
    clk_gmac5_axi64_axi: CLK_GMAC5_AXI64_AXI,
    clk_gmac_src: CLK_GMAC_SRC,
    clk_gmac1_gtx: CLK_GMAC1_GTX,
    clk_gmac1_rmii_rtx: CLK_GMAC1_RMII_RTX,
    clk_gmac5_axi64_ptp: CLK_GMAC5_AXI64_PTP,
    clk_gmac5_axi64_rx: CLK_GMAC5_AXI64_RX,
    clk_gmac5_axi64_rxi: CLK_GMAC5_AXI64_RXI,
    clk_gmac5_axi64_tx: CLK_GMAC5_AXI64_TX,
    clk_gmac5_axi64_txi: CLK_GMAC5_AXI64_TXI,
    clk_gmac1_gtxclk: CLK_GMAC1_GTXCLK,
    clk_gmac0_gtx: CLK_GMAC0_GTX,
    clk_gmac0_ptp: CLK_GMAC0_PTP,
    clk_gmac_phy: CLK_GMAC_PHY,
    clk_gmac0_gtxclk: CLK_GMAC0_GTXCLK,
    clk_sys_iomux_pclk: CLK_SYS_IOMUX_PCLK,
    clk_mbox_apb: CLK_MBOX_APB,
    clk_internal_ctrl_apb: CLK_INTERNAL_CTRL_APB,
    clk_u0_can_ctrl_apb: CLK_U0_CAN_CTRL_APB,
    clk_u0_can_ctrl_tim: CLK_U0_CAN_CTRL_TIM,
    clk_u0_can_ctrl_can: CLK_U0_CAN_CTRL_CAN,
    clk_u1_can_ctrl_apb: CLK_U1_CAN_CTRL_APB,
    clk_u1_can_ctrl_tim: CLK_U1_CAN_CTRL_TIM,
    clk_u1_can_ctrl_can: CLK_U1_CAN_CTRL_CAN,
    clk_pwm_apb: CLK_PWM_APB,
    clk_wdt_apb: CLK_WDT_APB,
    clk_wdt: CLK_WDT,
    clk_tim_apb: CLK_TIM_APB,
    clk_tim0: CLK_TIM0,
    clk_tim1: CLK_TIM1,
    clk_tim2: CLK_TIM2,
    clk_tim3: CLK_TIM3,
    clk_temp_sensor_apb: CLK_TEMP_SENSOR_APB,
    clk_temp_sensor: CLK_TEMP_SENSOR,
    clk_u0_spi_apb: CLK_U0_SPI_APB,
    clk_u1_spi_apb: CLK_U1_SPI_APB,
    clk_u2_spi_apb: CLK_U2_SPI_APB,
    clk_u3_spi_apb: CLK_U3_SPI_APB,
    clk_u4_spi_apb: CLK_U4_SPI_APB,
    clk_u5_spi_apb: CLK_U5_SPI_APB,
    clk_u6_spi_apb: CLK_U6_SPI_APB,
    clk_u0_i2c_apb: CLK_U0_I2C_APB,
    clk_u1_i2c_apb: CLK_U1_I2C_APB,
    clk_u2_i2c_apb: CLK_U2_I2C_APB,
    clk_u3_i2c_apb: CLK_U3_I2C_APB,
    clk_u4_i2c_apb: CLK_U4_I2C_APB,
    clk_u5_i2c_apb: CLK_U5_I2C_APB,
    clk_u6_i2c_apb: CLK_U6_I2C_APB,
    clk_u0_uart_apb: CLK_U0_UART_APB,
    clk_u0_uart_core: CLK_U0_UART_CORE,
    clk_u1_uart_apb: CLK_U1_UART_APB,
    clk_u1_uart_core: CLK_U1_UART_CORE,
    clk_u2_uart_apb: CLK_U2_UART_APB,
    clk_u2_uart_core: CLK_U2_UART_CORE,
    clk_u3_uart_apb: CLK_U3_UART_APB,
    clk_u3_uart_core: CLK_U3_UART_CORE,
    clk_u4_uart_apb: CLK_U4_UART_APB,
    clk_u4_uart_core: CLK_U4_UART_CORE,
    clk_u5_uart_apb: CLK_U5_UART_APB,
    clk_u5_uart_core: CLK_U5_UART_CORE,
    clk_pwmdac_apb: CLK_PWMDAC_APB,
    clk_pwmdac_core: CLK_PWMDAC_CORE,
    clk_spdif_apb: CLK_SPDIF_APB,
    clk_spdif_core: CLK_SPDIF_CORE,
    clk_u0_i2s_tx_apb: CLK_U0_I2S_TX_APB,
    clk_u0_i2stx_4ch0_bclk_mst: CLK_U0_I2STX_4CH0_BCLK_MST,
    clk_u0_i2stx_4ch0_bclk_mst_inv: CLK_U0_I2STX_4CH0_BCLK_MST_INV,
    clk_i2stx0_lrck_mst: CLK_I2STX0_LRCK_MST,
    clk_u0_i2stx_bclk: CLK_U0_I2STX_BCLK,
    clk_u0_i2stx_bclk_neg: CLK_U0_I2STX_BCLK_NEG,
    clk_u0_i2stx_lrck: CLK_U0_I2STX_LRCK,
    clk_u1_i2s_tx_apb: CLK_U1_I2S_TX_APB,
    clk_u1_i2stx_4ch1_bclk_mst: CLK_U1_I2STX_4CH1_BCLK_MST,
    clk_u1_i2stx_4ch1_bclk_mst_inv: CLK_U1_I2STX_4CH1_BCLK_MST_INV,
    clk_i2stx1_lrck_mst: CLK_I2STX1_LRCK_MST,
    clk_u1_i2stx_bclk: CLK_U1_I2STX_BCLK,
    clk_u1_i2stx_bclk_neg: CLK_U1_I2STX_BCLK_NEG,
    clk_u1_i2stx_lrck: CLK_U1_I2STX_LRCK,
    clk_i2s_apb: CLK_I2S_APB,
    clk_i2s_bclk_mst: CLK_I2S_BCLK_MST,
    clk_i2s_bclk_mst_inv: CLK_I2S_BCLK_MST_INV,
    clk_i2s_lrck_mst: CLK_I2S_LRCK_MST,
    clk_i2s_bclk: CLK_I2S_BCLK,
    clk_i2s_bclk_neg: CLK_I2S_BCLK_NEG,
    clk_i2s_lrck: CLK_I2S_LRCK,
    clk_pdm_dmic: CLK_PDM_DMIC,
    clk_pdm_apb: CLK_PDM_APB,
    clk_tdm_ahb: CLK_TDM_AHB,
    clk_tdm_apb: CLK_TDM_APB,
    clk_tdm_internal: CLK_TDM_INTERNAL,
    clk_tdm: CLK_TDM,
    clk_tdm_neg: CLK_TDM_NEG,
    clk_jtag_cert_trng: CLK_JTAG_CERT_TRNG,
    soft_rst0_addr_sel: SOFT_RST0_ADDR_SEL,
    soft_rst1_addr_sel: SOFT_RST1_ADDR_SEL,
    soft_rst2_addr_sel: SOFT_RST2_ADDR_SEL,
    soft_rst3_addr_sel: SOFT_RST3_ADDR_SEL,
    syscrg_rst0_status: SYSCRG_RST0_STATUS,
    syscrg_rst1_status: SYSCRG_RST1_STATUS,
    syscrg_rst2_status: SYSCRG_RST2_STATUS,
    syscrg_rst3_status: SYSCRG_RST3_STATUS,
}
impl RegisterBlock {
    #[doc = "0x00 - Clock CPU Root"]
    #[inline(always)]
    pub const fn clk_cpu_root(&self) -> &CLK_CPU_ROOT {
        &self.clk_cpu_root
    }
    #[doc = "0x04 - Clock CPU Core"]
    #[inline(always)]
    pub const fn clk_cpu_core(&self) -> &CLK_CPU_CORE {
        &self.clk_cpu_core
    }
    #[doc = "0x08 - Clock CPU Bus"]
    #[inline(always)]
    pub const fn clk_cpu_bus(&self) -> &CLK_CPU_BUS {
        &self.clk_cpu_bus
    }
    #[doc = "0x0c - Clock GPU Root"]
    #[inline(always)]
    pub const fn clk_gpu_root(&self) -> &CLK_GPU_ROOT {
        &self.clk_gpu_root
    }
    #[doc = "0x10 - Clock Peripheral Root"]
    #[inline(always)]
    pub const fn clk_peripheral_root(&self) -> &CLK_PERIPHERAL_ROOT {
        &self.clk_peripheral_root
    }
    #[doc = "0x14 - Clock Bus Root"]
    #[inline(always)]
    pub const fn clk_bus_root(&self) -> &CLK_BUS_ROOT {
        &self.clk_bus_root
    }
    #[doc = "0x18 - Clock NOCSTG Bus"]
    #[inline(always)]
    pub const fn clk_nocstg_bus(&self) -> &CLK_NOCSTG_BUS {
        &self.clk_nocstg_bus
    }
    #[doc = "0x1c - Clock AXI Configuration 0"]
    #[inline(always)]
    pub const fn clk_axi_cfg0(&self) -> &CLK_AXI_CFG0 {
        &self.clk_axi_cfg0
    }
    #[doc = "0x20 - Clock STG AXI AHB"]
    #[inline(always)]
    pub const fn clk_stg_axiahb(&self) -> &CLK_STG_AXIAHB {
        &self.clk_stg_axiahb
    }
    #[doc = "0x24 - Clock AHB 0"]
    #[inline(always)]
    pub const fn clk_ahb0(&self) -> &CLK_AHB0 {
        &self.clk_ahb0
    }
    #[doc = "0x28 - Clock AHB 1"]
    #[inline(always)]
    pub const fn clk_ahb1(&self) -> &CLK_AHB1 {
        &self.clk_ahb1
    }
    #[doc = "0x2c - Clock APB Bus"]
    #[inline(always)]
    pub const fn clk_apb_bus(&self) -> &CLK_APB_BUS {
        &self.clk_apb_bus
    }
    #[doc = "0x30 - Clock APB 0"]
    #[inline(always)]
    pub const fn clk_apb0(&self) -> &CLK_APB0 {
        &self.clk_apb0
    }
    #[doc = "0x34 - Clock PLL 0 Divider 2"]
    #[inline(always)]
    pub const fn clk_pll0_div2(&self) -> &CLK_PLL0_DIV2 {
        &self.clk_pll0_div2
    }
    #[doc = "0x38 - Clock PLL 1 Divider 2"]
    #[inline(always)]
    pub const fn clk_pll1_div2(&self) -> &CLK_PLL1_DIV2 {
        &self.clk_pll1_div2
    }
    #[doc = "0x3c - Clock PLL 2 Divider 2"]
    #[inline(always)]
    pub const fn clk_pll2_div2(&self) -> &CLK_PLL2_DIV2 {
        &self.clk_pll2_div2
    }
    #[doc = "0x40 - Clock Audio Root"]
    #[inline(always)]
    pub const fn clk_audio_root(&self) -> &CLK_AUDIO_ROOT {
        &self.clk_audio_root
    }
    #[doc = "0x44 - Clock MCLK Inner"]
    #[inline(always)]
    pub const fn clk_mclk_inner(&self) -> &CLK_MCLK_INNER {
        &self.clk_mclk_inner
    }
    #[doc = "0x48 - Clock MCLK"]
    #[inline(always)]
    pub const fn clk_mclk(&self) -> &CLK_MCLK {
        &self.clk_mclk
    }
    #[doc = "0x4c - Clock MCLK Out"]
    #[inline(always)]
    pub const fn clk_mclk_out(&self) -> &CLK_MCLK_OUT {
        &self.clk_mclk_out
    }
    #[doc = "0x50 - Clock ISP 2x"]
    #[inline(always)]
    pub const fn clk_isp_2x(&self) -> &CLK_ISP_2X {
        &self.clk_isp_2x
    }
    #[doc = "0x54 - Clock ISP AXI"]
    #[inline(always)]
    pub const fn clk_isp_axi(&self) -> &CLK_ISP_AXI {
        &self.clk_isp_axi
    }
    #[doc = "0x58 - Clock GCLK 0"]
    #[inline(always)]
    pub const fn clk_gclk0(&self) -> &CLK_GCLK0 {
        &self.clk_gclk0
    }
    #[doc = "0x5c - Clock GCLK 1"]
    #[inline(always)]
    pub const fn clk_gclk1(&self) -> &CLK_GCLK1 {
        &self.clk_gclk1
    }
    #[doc = "0x60 - Clock GCLK 2"]
    #[inline(always)]
    pub const fn clk_gclk2(&self) -> &CLK_GCLK2 {
        &self.clk_gclk2
    }
    #[doc = "0x64 - U7MC Core Clock 0"]
    #[inline(always)]
    pub const fn clk_u7mc_core0(&self) -> &CLK_U7MC_CORE0 {
        &self.clk_u7mc_core0
    }
    #[doc = "0x68 - U7MC Core Clock 1"]
    #[inline(always)]
    pub const fn clk_u7mc_core1(&self) -> &CLK_U7MC_CORE1 {
        &self.clk_u7mc_core1
    }
    #[doc = "0x6c - U7MC Core Clock 2"]
    #[inline(always)]
    pub const fn clk_u7mc_core2(&self) -> &CLK_U7MC_CORE2 {
        &self.clk_u7mc_core2
    }
    #[doc = "0x70 - U7MC Core Clock 3"]
    #[inline(always)]
    pub const fn clk_u7mc_core3(&self) -> &CLK_U7MC_CORE3 {
        &self.clk_u7mc_core3
    }
    #[doc = "0x74 - U7MC Core Clock 4"]
    #[inline(always)]
    pub const fn clk_u7mc_core4(&self) -> &CLK_U7MC_CORE4 {
        &self.clk_u7mc_core4
    }
    #[doc = "0x78 - U7MC Debug Clock"]
    #[inline(always)]
    pub const fn clk_u7mc_debug(&self) -> &CLK_U7MC_DEBUG {
        &self.clk_u7mc_debug
    }
    #[doc = "0x7c - U7MC RTC Toggle"]
    #[inline(always)]
    pub const fn u7mc_rtc_toggle(&self) -> &U7MC_RTC_TOGGLE {
        &self.u7mc_rtc_toggle
    }
    #[doc = "0x80 - U7MC Trace Clock 0"]
    #[inline(always)]
    pub const fn clk_u7mc_trace0(&self) -> &CLK_U7MC_TRACE0 {
        &self.clk_u7mc_trace0
    }
    #[doc = "0x84 - U7MC Trace Clock 1"]
    #[inline(always)]
    pub const fn clk_u7mc_trace1(&self) -> &CLK_U7MC_TRACE1 {
        &self.clk_u7mc_trace1
    }
    #[doc = "0x88 - U7MC Trace Clock 2"]
    #[inline(always)]
    pub const fn clk_u7mc_trace2(&self) -> &CLK_U7MC_TRACE2 {
        &self.clk_u7mc_trace2
    }
    #[doc = "0x8c - U7MC Trace Clock 3"]
    #[inline(always)]
    pub const fn clk_u7mc_trace3(&self) -> &CLK_U7MC_TRACE3 {
        &self.clk_u7mc_trace3
    }
    #[doc = "0x90 - U7MC Trace Clock 4"]
    #[inline(always)]
    pub const fn clk_u7mc_trace4(&self) -> &CLK_U7MC_TRACE4 {
        &self.clk_u7mc_trace4
    }
    #[doc = "0x94 - U7MC Trace Clock COM"]
    #[inline(always)]
    pub const fn clk_u7mc_trace_com(&self) -> &CLK_U7MC_TRACE_COM {
        &self.clk_u7mc_trace_com
    }
    #[doc = "0x98 - clk_u0_sft7110_noc_bus_clk_cpu_axi"]
    #[inline(always)]
    pub const fn clk_u0_sft7110_noc_bus_clk_cpu_axi(&self) -> &CLK_U0_SFT7110_NOC_BUS_CLK_CPU_AXI {
        &self.clk_u0_sft7110_noc_bus_clk_cpu_axi
    }
    #[doc = "0x9c - clk_u0_sft7110_noc_bus_clk_axicfg0_axi"]
    #[inline(always)]
    pub const fn clk_u0_sft7110_noc_bus_clk_axicfg0_axi(
        &self,
    ) -> &CLK_U0_SFT7110_NOC_BUS_CLK_AXICFG0_AXI {
        &self.clk_u0_sft7110_noc_bus_clk_axicfg0_axi
    }
    #[doc = "0xa0 - clk_osc_div2"]
    #[inline(always)]
    pub const fn clk_osc_div2(&self) -> &CLK_OSC_DIV2 {
        &self.clk_osc_div2
    }
    #[doc = "0xa4 - clk_pll1_div4"]
    #[inline(always)]
    pub const fn clk_pll1_div4(&self) -> &CLK_PLL1_DIV4 {
        &self.clk_pll1_div4
    }
    #[doc = "0xa8 - clk_pll1_div8"]
    #[inline(always)]
    pub const fn clk_pll1_div8(&self) -> &CLK_PLL1_DIV8 {
        &self.clk_pll1_div8
    }
    #[doc = "0xac - clk_ddr_bus"]
    #[inline(always)]
    pub const fn clk_ddr_bus(&self) -> &CLK_DDR_BUS {
        &self.clk_ddr_bus
    }
    #[doc = "0xb0 - clk_u0_ddr_sfft7110_clk_axi"]
    #[inline(always)]
    pub const fn clk_u0_ddr_sft7110_clk_axi(&self) -> &CLK_U0_DDR_SFT7110_CLK_AXI {
        &self.clk_u0_ddr_sft7110_clk_axi
    }
    #[doc = "0xb4 - clk_gpu_core"]
    #[inline(always)]
    pub const fn clk_gpu_core(&self) -> &CLK_GPU_CORE {
        &self.clk_gpu_core
    }
    #[doc = "0xb8 - clk_u0_img_gpu_core_clk"]
    #[inline(always)]
    pub const fn clk_u0_img_gpu_core_clk(&self) -> &CLK_U0_IMG_GPU_CORE_CLK {
        &self.clk_u0_img_gpu_core_clk
    }
    #[doc = "0xbc - clk_u0_img_gpu_sys_clk"]
    #[inline(always)]
    pub const fn clk_u0_img_gpu_sys_clk(&self) -> &CLK_U0_IMG_GPU_SYS_CLK {
        &self.clk_u0_img_gpu_sys_clk
    }
    #[doc = "0xc0 - clk_u0_img_gpu_clk_apb"]
    #[inline(always)]
    pub const fn clk_u0_img_gpu_clk_apb(&self) -> &CLK_U0_IMG_GPU_CLK_APB {
        &self.clk_u0_img_gpu_clk_apb
    }
    #[doc = "0xc4 - clk_u0_gpu_rtc_toggle"]
    #[inline(always)]
    pub const fn clk_u0_gpu_rtc_toggle(&self) -> &CLK_U0_GPU_RTC_TOGGLE {
        &self.clk_u0_gpu_rtc_toggle
    }
    #[doc = "0xc8 - clk_u0_sft7110_noc_bus_clk_gpu_axi"]
    #[inline(always)]
    pub const fn clk_u0_sft7110_noc_bus_clk_gpu_axi(&self) -> &CLK_U0_SFT7110_NOC_BUS_CLK_GPU_AXI {
        &self.clk_u0_sft7110_noc_bus_clk_gpu_axi
    }
    #[doc = "0xcc - clk_u0_dom_isp_top_clk_dom_isp_top_clk_ispcore_2x"]
    #[inline(always)]
    pub const fn clk_u0_dom_isp_top_clk_dom_isp_top_clk_ispcore_2x(
        &self,
    ) -> &CLK_U0_DOM_ISP_TOP_CLK_DOM_ISP_TOP_CLK_ISPCORE_2X {
        &self.clk_u0_dom_isp_top_clk_dom_isp_top_clk_ispcore_2x
    }
    #[doc = "0xd0 - clk_u0_dom_isp_top_clk_dom_isp_top_clk_isp_axi"]
    #[inline(always)]
    pub const fn clk_u0_dom_isp_top_clk_dom_isp_top_clk_isp_axi(
        &self,
    ) -> &CLK_U0_DOM_ISP_TOP_CLK_DOM_ISP_TOP_CLK_ISP_AXI {
        &self.clk_u0_dom_isp_top_clk_dom_isp_top_clk_isp_axi
    }
    #[doc = "0xd4 - clk_u0_sft7110_noc_bux_clk_isp_axi"]
    #[inline(always)]
    pub const fn clk_u0_sft7110_noc_bux_clk_isp_axi(&self) -> &CLK_U0_SFT7110_NOC_BUX_CLK_ISP_AXI {
        &self.clk_u0_sft7110_noc_bux_clk_isp_axi
    }
    #[doc = "0xd8 - clk_hifi4_core"]
    #[inline(always)]
    pub const fn clk_hifi4_core(&self) -> &CLK_HIFI4_CORE {
        &self.clk_hifi4_core
    }
    #[doc = "0xdc - clk_hifi4_axi"]
    #[inline(always)]
    pub const fn clk_hifi4_axi(&self) -> &CLK_HIFI4_AXI {
        &self.clk_hifi4_axi
    }
    #[doc = "0xe0 - clk_u0_axi_cfg1_dec_clk_main"]
    #[inline(always)]
    pub const fn clk_u0_axi_cfg1_dec_clk_main(&self) -> &CLK_U0_AXI_CFG1_DEC_CLK_MAIN {
        &self.clk_u0_axi_cfg1_dec_clk_main
    }
    #[doc = "0xe4 - clk_u0_axi_cfg1_dec_clk_ahb"]
    #[inline(always)]
    pub const fn clk_u0_axi_cfg1_dec_clk_ahb(&self) -> &CLK_U0_AXI_CFG1_DEC_CLK_AHB {
        &self.clk_u0_axi_cfg1_dec_clk_ahb
    }
    #[doc = "0xe8 - clk_u0_dom_vout_top_clk_dom_vout_top_clk_vout_src"]
    #[inline(always)]
    pub const fn clk_u0_dom_vout_top_clk_dom_vout_top_clk_vout_src(
        &self,
    ) -> &CLK_U0_DOM_VOUT_TOP_CLK_DOM_VOUT_TOP_CLK_VOUT_SRC {
        &self.clk_u0_dom_vout_top_clk_dom_vout_top_clk_vout_src
    }
    #[doc = "0xec - Clock Video Output AXI DIVCFG"]
    #[inline(always)]
    pub const fn clk_vout_axi_divcfg(&self) -> &CLK_VOUT_AXI_DIVCFG {
        &self.clk_vout_axi_divcfg
    }
    #[doc = "0xf0 - Clock NOC Display AXI"]
    #[inline(always)]
    pub const fn clk_noc_display_axi(&self) -> &CLK_NOC_DISPLAY_AXI {
        &self.clk_noc_display_axi
    }
    #[doc = "0xf4 - Clock Video Output AHB"]
    #[inline(always)]
    pub const fn clk_vout_ahb(&self) -> &CLK_VOUT_AHB {
        &self.clk_vout_ahb
    }
    #[doc = "0xf8 - Clock Video Output AXI ICG"]
    #[inline(always)]
    pub const fn clk_vout_axi_icg(&self) -> &CLK_VOUT_AXI_ICG {
        &self.clk_vout_axi_icg
    }
    #[doc = "0xfc - Clock Video Output HDMI TX0 MCLK"]
    #[inline(always)]
    pub const fn clk_vout_hdmi_tx0_mclk(&self) -> &CLK_VOUT_HDMI_TX0_MCLK {
        &self.clk_vout_hdmi_tx0_mclk
    }
    #[doc = "0x100 - Clock Video Output MIPI PHY Reference"]
    #[inline(always)]
    pub const fn clk_vout_mipi_phy(&self) -> &CLK_VOUT_MIPI_PHY {
        &self.clk_vout_mipi_phy
    }
    #[doc = "0x104 - Clock JPEG Codec AXI"]
    #[inline(always)]
    pub const fn clk_jpeg_codec_axi(&self) -> &CLK_JPEG_CODEC_AXI {
        &self.clk_jpeg_codec_axi
    }
    #[doc = "0x108 - CODAJ12 Clock AXI"]
    #[inline(always)]
    pub const fn clk_codaj12_axi(&self) -> &CLK_CODAJ12_AXI {
        &self.clk_codaj12_axi
    }
    #[doc = "0x10c - CODAJ12 Clock Core"]
    #[inline(always)]
    pub const fn clk_codaj12_core(&self) -> &CLK_CODAJ12_CORE {
        &self.clk_codaj12_core
    }
    #[doc = "0x110 - CODAJ12 Clock APB"]
    #[inline(always)]
    pub const fn clk_codaj12_apb(&self) -> &CLK_CODAJ12_APB {
        &self.clk_codaj12_apb
    }
    #[doc = "0x114 - Clock Video Decoder AXI"]
    #[inline(always)]
    pub const fn clk_vdec_axi(&self) -> &CLK_VDEC_AXI {
        &self.clk_vdec_axi
    }
    #[doc = "0x118 - Clock WAVE511 AXI"]
    #[inline(always)]
    pub const fn clk_wave511_axi(&self) -> &CLK_WAVE511_AXI {
        &self.clk_wave511_axi
    }
    #[doc = "0x11c - Clock WAVE511 BPU"]
    #[inline(always)]
    pub const fn clk_wave511_bpu(&self) -> &CLK_WAVE511_BPU {
        &self.clk_wave511_bpu
    }
    #[doc = "0x120 - Clock WAVE511 VCE"]
    #[inline(always)]
    pub const fn clk_wave511_vce(&self) -> &CLK_WAVE511_VCE {
        &self.clk_wave511_vce
    }
    #[doc = "0x124 - Clock WAVE511 APB"]
    #[inline(always)]
    pub const fn clk_wave511_apb(&self) -> &CLK_WAVE511_APB {
        &self.clk_wave511_apb
    }
    #[doc = "0x128 - Clock WAVE511 JPG ARB"]
    #[inline(always)]
    pub const fn clk_wave511_jpg_arb(&self) -> &CLK_WAVE511_JPG_ARB {
        &self.clk_wave511_jpg_arb
    }
    #[doc = "0x12c - Clock WAVE511 JPG Main"]
    #[inline(always)]
    pub const fn clk_wave511_jpg_main(&self) -> &CLK_WAVE511_JPG_MAIN {
        &self.clk_wave511_jpg_main
    }
    #[doc = "0x130 - Clock NOC Video Decoder AXI"]
    #[inline(always)]
    pub const fn clk_noc_vdec_axi(&self) -> &CLK_NOC_VDEC_AXI {
        &self.clk_noc_vdec_axi
    }
    #[doc = "0x134 - Clock Video Encoder AXI"]
    #[inline(always)]
    pub const fn clk_venc_axi(&self) -> &CLK_VENC_AXI {
        &self.clk_venc_axi
    }
    #[doc = "0x138 - Clock WAVE420L AXI"]
    #[inline(always)]
    pub const fn clk_wave420l_axi(&self) -> &CLK_WAVE420L_AXI {
        &self.clk_wave420l_axi
    }
    #[doc = "0x13c - Clock WAVE420L BPU"]
    #[inline(always)]
    pub const fn clk_wave420l_bpu(&self) -> &CLK_WAVE420L_BPU {
        &self.clk_wave420l_bpu
    }
    #[doc = "0x140 - Clock WAVE420L VCE"]
    #[inline(always)]
    pub const fn clk_wave420l_vce(&self) -> &CLK_WAVE420L_VCE {
        &self.clk_wave420l_vce
    }
    #[doc = "0x144 - Clock WAVE420L APB"]
    #[inline(always)]
    pub const fn clk_wave420l_apb(&self) -> &CLK_WAVE420L_APB {
        &self.clk_wave420l_apb
    }
    #[doc = "0x148 - Clock NOC Video Encoder AXI"]
    #[inline(always)]
    pub const fn clk_noc_venc_axi(&self) -> &CLK_NOC_VENC_AXI {
        &self.clk_noc_venc_axi
    }
    #[doc = "0x14c - Clock AXI Config 0 DEC Main Divider"]
    #[inline(always)]
    pub const fn clk_axi_cfg0_dec_main_div(&self) -> &CLK_AXI_CFG0_DEC_MAIN_DIV {
        &self.clk_axi_cfg0_dec_main_div
    }
    #[doc = "0x150 - Clock AXI Config 0 DEC Main"]
    #[inline(always)]
    pub const fn clk_axi_cfg0_dec_main(&self) -> &CLK_AXI_CFG0_DEC_MAIN {
        &self.clk_axi_cfg0_dec_main
    }
    #[doc = "0x154 - Clock AXI Config 0 DEC HIFI4"]
    #[inline(always)]
    pub const fn clk_axi_cfg0_dec_hifi4(&self) -> &CLK_AXI_CFG0_DEC_HIFI4 {
        &self.clk_axi_cfg0_dec_hifi4
    }
    #[doc = "0x158 - Clock AXIMEM 128B AXI"]
    #[inline(always)]
    pub const fn clk_aximem_128b_axi(&self) -> &CLK_AXIMEM_128B_AXI {
        &self.clk_aximem_128b_axi
    }
    #[doc = "0x15c - Clock QSPI AHB"]
    #[inline(always)]
    pub const fn clk_qspi_ahb(&self) -> &CLK_QSPI_AHB {
        &self.clk_qspi_ahb
    }
    #[doc = "0x160 - Clock QSPI APB"]
    #[inline(always)]
    pub const fn clk_qspi_apb(&self) -> &CLK_QSPI_APB {
        &self.clk_qspi_apb
    }
    #[doc = "0x164 - Clock QSPI Reference Source"]
    #[inline(always)]
    pub const fn clk_qspi_ref_src(&self) -> &CLK_QSPI_REF_SRC {
        &self.clk_qspi_ref_src
    }
    #[doc = "0x168 - Clock QSPI Reference"]
    #[inline(always)]
    pub const fn clk_qspi_ref(&self) -> &CLK_QSPI_REF {
        &self.clk_qspi_ref
    }
    #[doc = "0x16c - U0 SD Clock AHB"]
    #[inline(always)]
    pub const fn clk_u0_sd_ahb(&self) -> &CLK_U0_SD_AHB {
        &self.clk_u0_sd_ahb
    }
    #[doc = "0x170 - U1 SD Clock AHB"]
    #[inline(always)]
    pub const fn clk_u1_sd_ahb(&self) -> &CLK_U1_SD_AHB {
        &self.clk_u1_sd_ahb
    }
    #[doc = "0x174 - U0 SD Card Clock"]
    #[inline(always)]
    pub const fn clk_u0_sd_card(&self) -> &CLK_U0_SD_CARD {
        &self.clk_u0_sd_card
    }
    #[doc = "0x178 - U1 SD Card Clock"]
    #[inline(always)]
    pub const fn clk_u1_sd_card(&self) -> &CLK_U1_SD_CARD {
        &self.clk_u1_sd_card
    }
    #[doc = "0x17c - Clock USB 125M"]
    #[inline(always)]
    pub const fn clk_usb_125m(&self) -> &CLK_USB_125M {
        &self.clk_usb_125m
    }
    #[doc = "0x180 - Clock NOC STG AXI"]
    #[inline(always)]
    pub const fn clk_noc_stg_axi(&self) -> &CLK_NOC_STG_AXI {
        &self.clk_noc_stg_axi
    }
    #[doc = "0x184 - Clock GMAC 5 AXI 64 AHB"]
    #[inline(always)]
    pub const fn clk_gmac5_axi64_ahb(&self) -> &CLK_GMAC5_AXI64_AHB {
        &self.clk_gmac5_axi64_ahb
    }
    #[doc = "0x188 - Clock GMAC 5 AXI 64 AXI"]
    #[inline(always)]
    pub const fn clk_gmac5_axi64_axi(&self) -> &CLK_GMAC5_AXI64_AXI {
        &self.clk_gmac5_axi64_axi
    }
    #[doc = "0x18c - Clock GMAC Source"]
    #[inline(always)]
    pub const fn clk_gmac_src(&self) -> &CLK_GMAC_SRC {
        &self.clk_gmac_src
    }
    #[doc = "0x190 - Clock GMAC 1 GTX"]
    #[inline(always)]
    pub const fn clk_gmac1_gtx(&self) -> &CLK_GMAC1_GTX {
        &self.clk_gmac1_gtx
    }
    #[doc = "0x194 - Clock GMAC 1 RMII RTX"]
    #[inline(always)]
    pub const fn clk_gmac1_rmii_rtx(&self) -> &CLK_GMAC1_RMII_RTX {
        &self.clk_gmac1_rmii_rtx
    }
    #[doc = "0x198 - Clock GMAC 5 AXI 64 PTP"]
    #[inline(always)]
    pub const fn clk_gmac5_axi64_ptp(&self) -> &CLK_GMAC5_AXI64_PTP {
        &self.clk_gmac5_axi64_ptp
    }
    #[doc = "0x19c - Clock GMAC 5 AXI 64 RX"]
    #[inline(always)]
    pub const fn clk_gmac5_axi64_rx(&self) -> &CLK_GMAC5_AXI64_RX {
        &self.clk_gmac5_axi64_rx
    }
    #[doc = "0x1a0 - Clock GMAC 5 AXI 64 RX Inverter"]
    #[inline(always)]
    pub const fn clk_gmac5_axi64_rxi(&self) -> &CLK_GMAC5_AXI64_RXI {
        &self.clk_gmac5_axi64_rxi
    }
    #[doc = "0x1a4 - Clock GMAC 5 AXI 64 TX"]
    #[inline(always)]
    pub const fn clk_gmac5_axi64_tx(&self) -> &CLK_GMAC5_AXI64_TX {
        &self.clk_gmac5_axi64_tx
    }
    #[doc = "0x1a8 - Clock GMAC 5 AXI 64 TX Inverter"]
    #[inline(always)]
    pub const fn clk_gmac5_axi64_txi(&self) -> &CLK_GMAC5_AXI64_TXI {
        &self.clk_gmac5_axi64_txi
    }
    #[doc = "0x1ac - Clock GMAC 1 GTXC"]
    #[inline(always)]
    pub const fn clk_gmac1_gtxclk(&self) -> &CLK_GMAC1_GTXCLK {
        &self.clk_gmac1_gtxclk
    }
    #[doc = "0x1b0 - Clock GMAC 0 GTX"]
    #[inline(always)]
    pub const fn clk_gmac0_gtx(&self) -> &CLK_GMAC0_GTX {
        &self.clk_gmac0_gtx
    }
    #[doc = "0x1b4 - Clock GMAC 0 PTP"]
    #[inline(always)]
    pub const fn clk_gmac0_ptp(&self) -> &CLK_GMAC0_PTP {
        &self.clk_gmac0_ptp
    }
    #[doc = "0x1b8 - Clock GMAC PHY"]
    #[inline(always)]
    pub const fn clk_gmac_phy(&self) -> &CLK_GMAC_PHY {
        &self.clk_gmac_phy
    }
    #[doc = "0x1bc - Clock GMAC 0 GTXC"]
    #[inline(always)]
    pub const fn clk_gmac0_gtxclk(&self) -> &CLK_GMAC0_GTXCLK {
        &self.clk_gmac0_gtxclk
    }
    #[doc = "0x1c0 - Clock SYS IOMUX PCLK"]
    #[inline(always)]
    pub const fn clk_sys_iomux_pclk(&self) -> &CLK_SYS_IOMUX_PCLK {
        &self.clk_sys_iomux_pclk
    }
    #[doc = "0x1c4 - Clock Mailbox APB"]
    #[inline(always)]
    pub const fn clk_mbox_apb(&self) -> &CLK_MBOX_APB {
        &self.clk_mbox_apb
    }
    #[doc = "0x1c8 - Clock Internal Controller APB"]
    #[inline(always)]
    pub const fn clk_internal_ctrl_apb(&self) -> &CLK_INTERNAL_CTRL_APB {
        &self.clk_internal_ctrl_apb
    }
    #[doc = "0x1cc - U0 Clock CAN Controller APB"]
    #[inline(always)]
    pub const fn clk_u0_can_ctrl_apb(&self) -> &CLK_U0_CAN_CTRL_APB {
        &self.clk_u0_can_ctrl_apb
    }
    #[doc = "0x1d0 - U0 Clock CAN Controller Timer"]
    #[inline(always)]
    pub const fn clk_u0_can_ctrl_tim(&self) -> &CLK_U0_CAN_CTRL_TIM {
        &self.clk_u0_can_ctrl_tim
    }
    #[doc = "0x1d4 - U0 Clock CAN Controller CAN"]
    #[inline(always)]
    pub const fn clk_u0_can_ctrl_can(&self) -> &CLK_U0_CAN_CTRL_CAN {
        &self.clk_u0_can_ctrl_can
    }
    #[doc = "0x1d8 - U1 Clock CAN Controller APB"]
    #[inline(always)]
    pub const fn clk_u1_can_ctrl_apb(&self) -> &CLK_U1_CAN_CTRL_APB {
        &self.clk_u1_can_ctrl_apb
    }
    #[doc = "0x1dc - U1 Clock CAN Controller Timer"]
    #[inline(always)]
    pub const fn clk_u1_can_ctrl_tim(&self) -> &CLK_U1_CAN_CTRL_TIM {
        &self.clk_u1_can_ctrl_tim
    }
    #[doc = "0x1e0 - U1 Clock CAN Controller CAN"]
    #[inline(always)]
    pub const fn clk_u1_can_ctrl_can(&self) -> &CLK_U1_CAN_CTRL_CAN {
        &self.clk_u1_can_ctrl_can
    }
    #[doc = "0x1e4 - Clock PWM APB"]
    #[inline(always)]
    pub const fn clk_pwm_apb(&self) -> &CLK_PWM_APB {
        &self.clk_pwm_apb
    }
    #[doc = "0x1e8 - Clock WDT APB"]
    #[inline(always)]
    pub const fn clk_wdt_apb(&self) -> &CLK_WDT_APB {
        &self.clk_wdt_apb
    }
    #[doc = "0x1ec - Clock WDT"]
    #[inline(always)]
    pub const fn clk_wdt(&self) -> &CLK_WDT {
        &self.clk_wdt
    }
    #[doc = "0x1f0 - Clock Timer APB"]
    #[inline(always)]
    pub const fn clk_tim_apb(&self) -> &CLK_TIM_APB {
        &self.clk_tim_apb
    }
    #[doc = "0x1f4 - Clock Timer 0"]
    #[inline(always)]
    pub const fn clk_tim0(&self) -> &CLK_TIM0 {
        &self.clk_tim0
    }
    #[doc = "0x1f8 - Clock Timer 1"]
    #[inline(always)]
    pub const fn clk_tim1(&self) -> &CLK_TIM1 {
        &self.clk_tim1
    }
    #[doc = "0x1fc - Clock Timer 2"]
    #[inline(always)]
    pub const fn clk_tim2(&self) -> &CLK_TIM2 {
        &self.clk_tim2
    }
    #[doc = "0x200 - Clock Timer 3"]
    #[inline(always)]
    pub const fn clk_tim3(&self) -> &CLK_TIM3 {
        &self.clk_tim3
    }
    #[doc = "0x204 - Clock Temperature Sensor APB"]
    #[inline(always)]
    pub const fn clk_temp_sensor_apb(&self) -> &CLK_TEMP_SENSOR_APB {
        &self.clk_temp_sensor_apb
    }
    #[doc = "0x208 - Clock Temperature Sensor"]
    #[inline(always)]
    pub const fn clk_temp_sensor(&self) -> &CLK_TEMP_SENSOR {
        &self.clk_temp_sensor
    }
    #[doc = "0x20c - U0 Clock SPI APB"]
    #[inline(always)]
    pub const fn clk_u0_spi_apb(&self) -> &CLK_U0_SPI_APB {
        &self.clk_u0_spi_apb
    }
    #[doc = "0x210 - U1 Clock SPI APB"]
    #[inline(always)]
    pub const fn clk_u1_spi_apb(&self) -> &CLK_U1_SPI_APB {
        &self.clk_u1_spi_apb
    }
    #[doc = "0x214 - U2 Clock SPI APB"]
    #[inline(always)]
    pub const fn clk_u2_spi_apb(&self) -> &CLK_U2_SPI_APB {
        &self.clk_u2_spi_apb
    }
    #[doc = "0x218 - U3 Clock SPI APB"]
    #[inline(always)]
    pub const fn clk_u3_spi_apb(&self) -> &CLK_U3_SPI_APB {
        &self.clk_u3_spi_apb
    }
    #[doc = "0x21c - U4 Clock SPI APB"]
    #[inline(always)]
    pub const fn clk_u4_spi_apb(&self) -> &CLK_U4_SPI_APB {
        &self.clk_u4_spi_apb
    }
    #[doc = "0x220 - U5 Clock SPI APB"]
    #[inline(always)]
    pub const fn clk_u5_spi_apb(&self) -> &CLK_U5_SPI_APB {
        &self.clk_u5_spi_apb
    }
    #[doc = "0x224 - U6 Clock SPI APB"]
    #[inline(always)]
    pub const fn clk_u6_spi_apb(&self) -> &CLK_U6_SPI_APB {
        &self.clk_u6_spi_apb
    }
    #[doc = "0x228 - U0 Clock I2C APB"]
    #[inline(always)]
    pub const fn clk_u0_i2c_apb(&self) -> &CLK_U0_I2C_APB {
        &self.clk_u0_i2c_apb
    }
    #[doc = "0x22c - U1 Clock I2C APB"]
    #[inline(always)]
    pub const fn clk_u1_i2c_apb(&self) -> &CLK_U1_I2C_APB {
        &self.clk_u1_i2c_apb
    }
    #[doc = "0x230 - U2 Clock I2C APB"]
    #[inline(always)]
    pub const fn clk_u2_i2c_apb(&self) -> &CLK_U2_I2C_APB {
        &self.clk_u2_i2c_apb
    }
    #[doc = "0x234 - U3 Clock I2C APB"]
    #[inline(always)]
    pub const fn clk_u3_i2c_apb(&self) -> &CLK_U3_I2C_APB {
        &self.clk_u3_i2c_apb
    }
    #[doc = "0x238 - U4 Clock I2C APB"]
    #[inline(always)]
    pub const fn clk_u4_i2c_apb(&self) -> &CLK_U4_I2C_APB {
        &self.clk_u4_i2c_apb
    }
    #[doc = "0x23c - U5 Clock I2C APB"]
    #[inline(always)]
    pub const fn clk_u5_i2c_apb(&self) -> &CLK_U5_I2C_APB {
        &self.clk_u5_i2c_apb
    }
    #[doc = "0x240 - U6 Clock I2C APB"]
    #[inline(always)]
    pub const fn clk_u6_i2c_apb(&self) -> &CLK_U6_I2C_APB {
        &self.clk_u6_i2c_apb
    }
    #[doc = "0x244 - U0 Clock UART APB"]
    #[inline(always)]
    pub const fn clk_u0_uart_apb(&self) -> &CLK_U0_UART_APB {
        &self.clk_u0_uart_apb
    }
    #[doc = "0x248 - U0 Clock UART Core"]
    #[inline(always)]
    pub const fn clk_u0_uart_core(&self) -> &CLK_U0_UART_CORE {
        &self.clk_u0_uart_core
    }
    #[doc = "0x24c - U1 Clock UART APB"]
    #[inline(always)]
    pub const fn clk_u1_uart_apb(&self) -> &CLK_U1_UART_APB {
        &self.clk_u1_uart_apb
    }
    #[doc = "0x250 - U1 Clock UART Core"]
    #[inline(always)]
    pub const fn clk_u1_uart_core(&self) -> &CLK_U1_UART_CORE {
        &self.clk_u1_uart_core
    }
    #[doc = "0x254 - U2 Clock UART APB"]
    #[inline(always)]
    pub const fn clk_u2_uart_apb(&self) -> &CLK_U2_UART_APB {
        &self.clk_u2_uart_apb
    }
    #[doc = "0x258 - U2 Clock UART Core"]
    #[inline(always)]
    pub const fn clk_u2_uart_core(&self) -> &CLK_U2_UART_CORE {
        &self.clk_u2_uart_core
    }
    #[doc = "0x25c - U3 Clock UART APB"]
    #[inline(always)]
    pub const fn clk_u3_uart_apb(&self) -> &CLK_U3_UART_APB {
        &self.clk_u3_uart_apb
    }
    #[doc = "0x260 - U3 Clock UART Core"]
    #[inline(always)]
    pub const fn clk_u3_uart_core(&self) -> &CLK_U3_UART_CORE {
        &self.clk_u3_uart_core
    }
    #[doc = "0x264 - U4 Clock UART APB"]
    #[inline(always)]
    pub const fn clk_u4_uart_apb(&self) -> &CLK_U4_UART_APB {
        &self.clk_u4_uart_apb
    }
    #[doc = "0x268 - U4 Clock UART Core"]
    #[inline(always)]
    pub const fn clk_u4_uart_core(&self) -> &CLK_U4_UART_CORE {
        &self.clk_u4_uart_core
    }
    #[doc = "0x26c - U5 Clock UART APB"]
    #[inline(always)]
    pub const fn clk_u5_uart_apb(&self) -> &CLK_U5_UART_APB {
        &self.clk_u5_uart_apb
    }
    #[doc = "0x270 - U5 Clock UART Core"]
    #[inline(always)]
    pub const fn clk_u5_uart_core(&self) -> &CLK_U5_UART_CORE {
        &self.clk_u5_uart_core
    }
    #[doc = "0x274 - Clock PWMDAC APB"]
    #[inline(always)]
    pub const fn clk_pwmdac_apb(&self) -> &CLK_PWMDAC_APB {
        &self.clk_pwmdac_apb
    }
    #[doc = "0x278 - Clock PWMDAC Core"]
    #[inline(always)]
    pub const fn clk_pwmdac_core(&self) -> &CLK_PWMDAC_CORE {
        &self.clk_pwmdac_core
    }
    #[doc = "0x27c - Clock SPDIF APB"]
    #[inline(always)]
    pub const fn clk_spdif_apb(&self) -> &CLK_SPDIF_APB {
        &self.clk_spdif_apb
    }
    #[doc = "0x280 - Clock SPDIF Core"]
    #[inline(always)]
    pub const fn clk_spdif_core(&self) -> &CLK_SPDIF_CORE {
        &self.clk_spdif_core
    }
    #[doc = "0x284 - U0 Clock I2S TX APB"]
    #[inline(always)]
    pub const fn clk_u0_i2s_tx_apb(&self) -> &CLK_U0_I2S_TX_APB {
        &self.clk_u0_i2s_tx_apb
    }
    #[doc = "0x288 - U0 Clock I2S TX 0 BCLK MST"]
    #[inline(always)]
    pub const fn clk_u0_i2stx_4ch0_bclk_mst(&self) -> &CLK_U0_I2STX_4CH0_BCLK_MST {
        &self.clk_u0_i2stx_4ch0_bclk_mst
    }
    #[doc = "0x28c - U0 Clock I2S TX 0 BCLK MST Inverter"]
    #[inline(always)]
    pub const fn clk_u0_i2stx_4ch0_bclk_mst_inv(&self) -> &CLK_U0_I2STX_4CH0_BCLK_MST_INV {
        &self.clk_u0_i2stx_4ch0_bclk_mst_inv
    }
    #[doc = "0x290 - Clock I2S TX 0 LRCK MST"]
    #[inline(always)]
    pub const fn clk_i2stx0_lrck_mst(&self) -> &CLK_I2STX0_LRCK_MST {
        &self.clk_i2stx0_lrck_mst
    }
    #[doc = "0x294 - U0 Clock I2S TX BCLK"]
    #[inline(always)]
    pub const fn clk_u0_i2stx_bclk(&self) -> &CLK_U0_I2STX_BCLK {
        &self.clk_u0_i2stx_bclk
    }
    #[doc = "0x298 - U0 Clock I2S TX BCLK Negative"]
    #[inline(always)]
    pub const fn clk_u0_i2stx_bclk_neg(&self) -> &CLK_U0_I2STX_BCLK_NEG {
        &self.clk_u0_i2stx_bclk_neg
    }
    #[doc = "0x29c - U0 Clock I2S TX LRCK"]
    #[inline(always)]
    pub const fn clk_u0_i2stx_lrck(&self) -> &CLK_U0_I2STX_LRCK {
        &self.clk_u0_i2stx_lrck
    }
    #[doc = "0x2a0 - U1 Clock I2S TX APB"]
    #[inline(always)]
    pub const fn clk_u1_i2s_tx_apb(&self) -> &CLK_U1_I2S_TX_APB {
        &self.clk_u1_i2s_tx_apb
    }
    #[doc = "0x2a4 - U1 Clock I2S TX 1 BCLK MST"]
    #[inline(always)]
    pub const fn clk_u1_i2stx_4ch1_bclk_mst(&self) -> &CLK_U1_I2STX_4CH1_BCLK_MST {
        &self.clk_u1_i2stx_4ch1_bclk_mst
    }
    #[doc = "0x2a8 - U1 Clock I2S TX 1 BCLK MST Inverter"]
    #[inline(always)]
    pub const fn clk_u1_i2stx_4ch1_bclk_mst_inv(&self) -> &CLK_U1_I2STX_4CH1_BCLK_MST_INV {
        &self.clk_u1_i2stx_4ch1_bclk_mst_inv
    }
    #[doc = "0x2ac - Clock I2S TX 1 LRCK MST"]
    #[inline(always)]
    pub const fn clk_i2stx1_lrck_mst(&self) -> &CLK_I2STX1_LRCK_MST {
        &self.clk_i2stx1_lrck_mst
    }
    #[doc = "0x2b0 - U1 Clock I2S TX BCLK"]
    #[inline(always)]
    pub const fn clk_u1_i2stx_bclk(&self) -> &CLK_U1_I2STX_BCLK {
        &self.clk_u1_i2stx_bclk
    }
    #[doc = "0x2b4 - U1 Clock I2S TX BCLK Negative"]
    #[inline(always)]
    pub const fn clk_u1_i2stx_bclk_neg(&self) -> &CLK_U1_I2STX_BCLK_NEG {
        &self.clk_u1_i2stx_bclk_neg
    }
    #[doc = "0x2b8 - U1 Clock I2S TX LRCK"]
    #[inline(always)]
    pub const fn clk_u1_i2stx_lrck(&self) -> &CLK_U1_I2STX_LRCK {
        &self.clk_u1_i2stx_lrck
    }
    #[doc = "0x2bc - Clock I2S APB"]
    #[inline(always)]
    pub const fn clk_i2s_apb(&self) -> &CLK_I2S_APB {
        &self.clk_i2s_apb
    }
    #[doc = "0x2c0 - Clock I2S BCLK MST"]
    #[inline(always)]
    pub const fn clk_i2s_bclk_mst(&self) -> &CLK_I2S_BCLK_MST {
        &self.clk_i2s_bclk_mst
    }
    #[doc = "0x2c4 - Clock I2S BCLK MST Inverter"]
    #[inline(always)]
    pub const fn clk_i2s_bclk_mst_inv(&self) -> &CLK_I2S_BCLK_MST_INV {
        &self.clk_i2s_bclk_mst_inv
    }
    #[doc = "0x2c8 - Clock I2S LRCK MST"]
    #[inline(always)]
    pub const fn clk_i2s_lrck_mst(&self) -> &CLK_I2S_LRCK_MST {
        &self.clk_i2s_lrck_mst
    }
    #[doc = "0x2cc - Clock I2S BCLK"]
    #[inline(always)]
    pub const fn clk_i2s_bclk(&self) -> &CLK_I2S_BCLK {
        &self.clk_i2s_bclk
    }
    #[doc = "0x2d0 - Clock I2S BCLK Negative"]
    #[inline(always)]
    pub const fn clk_i2s_bclk_neg(&self) -> &CLK_I2S_BCLK_NEG {
        &self.clk_i2s_bclk_neg
    }
    #[doc = "0x2d4 - Clock I2S LRCK"]
    #[inline(always)]
    pub const fn clk_i2s_lrck(&self) -> &CLK_I2S_LRCK {
        &self.clk_i2s_lrck
    }
    #[doc = "0x2d8 - Clock PDM DMIC"]
    #[inline(always)]
    pub const fn clk_pdm_dmic(&self) -> &CLK_PDM_DMIC {
        &self.clk_pdm_dmic
    }
    #[doc = "0x2dc - Clock PDM APB"]
    #[inline(always)]
    pub const fn clk_pdm_apb(&self) -> &CLK_PDM_APB {
        &self.clk_pdm_apb
    }
    #[doc = "0x2e0 - Clock TDM AHB"]
    #[inline(always)]
    pub const fn clk_tdm_ahb(&self) -> &CLK_TDM_AHB {
        &self.clk_tdm_ahb
    }
    #[doc = "0x2e4 - Clock TDM APB"]
    #[inline(always)]
    pub const fn clk_tdm_apb(&self) -> &CLK_TDM_APB {
        &self.clk_tdm_apb
    }
    #[doc = "0x2e8 - Clock TDM Internal"]
    #[inline(always)]
    pub const fn clk_tdm_internal(&self) -> &CLK_TDM_INTERNAL {
        &self.clk_tdm_internal
    }
    #[doc = "0x2ec - Clock TDM"]
    #[inline(always)]
    pub const fn clk_tdm(&self) -> &CLK_TDM {
        &self.clk_tdm
    }
    #[doc = "0x2f0 - Clock TDM Negative"]
    #[inline(always)]
    pub const fn clk_tdm_neg(&self) -> &CLK_TDM_NEG {
        &self.clk_tdm_neg
    }
    #[doc = "0x2f4 - Clock JTAG Certification TRNG"]
    #[inline(always)]
    pub const fn clk_jtag_cert_trng(&self) -> &CLK_JTAG_CERT_TRNG {
        &self.clk_jtag_cert_trng
    }
    #[doc = "0x2f8 - Software RESET 0 Address Selector"]
    #[inline(always)]
    pub const fn soft_rst0_addr_sel(&self) -> &SOFT_RST0_ADDR_SEL {
        &self.soft_rst0_addr_sel
    }
    #[doc = "0x2fc - Software RESET 1 Address Selector"]
    #[inline(always)]
    pub const fn soft_rst1_addr_sel(&self) -> &SOFT_RST1_ADDR_SEL {
        &self.soft_rst1_addr_sel
    }
    #[doc = "0x300 - Software RESET 2 Address Selector"]
    #[inline(always)]
    pub const fn soft_rst2_addr_sel(&self) -> &SOFT_RST2_ADDR_SEL {
        &self.soft_rst2_addr_sel
    }
    #[doc = "0x304 - Software RESET 3 Address Selector"]
    #[inline(always)]
    pub const fn soft_rst3_addr_sel(&self) -> &SOFT_RST3_ADDR_SEL {
        &self.soft_rst3_addr_sel
    }
    #[doc = "0x308 - SYSCRG RESET Status 0"]
    #[inline(always)]
    pub const fn syscrg_rst0_status(&self) -> &SYSCRG_RST0_STATUS {
        &self.syscrg_rst0_status
    }
    #[doc = "0x30c - SYSCRG RESET Status 1"]
    #[inline(always)]
    pub const fn syscrg_rst1_status(&self) -> &SYSCRG_RST1_STATUS {
        &self.syscrg_rst1_status
    }
    #[doc = "0x310 - SYSCRG RESET Status 2"]
    #[inline(always)]
    pub const fn syscrg_rst2_status(&self) -> &SYSCRG_RST2_STATUS {
        &self.syscrg_rst2_status
    }
    #[doc = "0x314 - SYSCRG RESET Status 3"]
    #[inline(always)]
    pub const fn syscrg_rst3_status(&self) -> &SYSCRG_RST3_STATUS {
        &self.syscrg_rst3_status
    }
}
#[doc = "clk_cpu_root (rw) register accessor: Clock CPU Root\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`clk_cpu_root::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`clk_cpu_root::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@clk_cpu_root`]
module"]
pub type CLK_CPU_ROOT = crate::Reg<clk_cpu_root::CLK_CPU_ROOT_SPEC>;
#[doc = "Clock CPU Root"]
pub mod clk_cpu_root;
#[doc = "clk_cpu_core (rw) register accessor: Clock CPU Core\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`clk_cpu_core::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`clk_cpu_core::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@clk_cpu_core`]
module"]
pub type CLK_CPU_CORE = crate::Reg<clk_cpu_core::CLK_CPU_CORE_SPEC>;
#[doc = "Clock CPU Core"]
pub mod clk_cpu_core;
#[doc = "clk_cpu_bus (rw) register accessor: Clock CPU Bus\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`clk_cpu_bus::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`clk_cpu_bus::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@clk_cpu_bus`]
module"]
pub type CLK_CPU_BUS = crate::Reg<clk_cpu_bus::CLK_CPU_BUS_SPEC>;
#[doc = "Clock CPU Bus"]
pub mod clk_cpu_bus;
#[doc = "clk_gpu_root (rw) register accessor: Clock GPU Root\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`clk_gpu_root::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`clk_gpu_root::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@clk_gpu_root`]
module"]
pub type CLK_GPU_ROOT = crate::Reg<clk_gpu_root::CLK_GPU_ROOT_SPEC>;
#[doc = "Clock GPU Root"]
pub mod clk_gpu_root;
#[doc = "clk_peripheral_root (rw) register accessor: Clock Peripheral Root\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`clk_peripheral_root::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`clk_peripheral_root::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@clk_peripheral_root`]
module"]
pub type CLK_PERIPHERAL_ROOT = crate::Reg<clk_peripheral_root::CLK_PERIPHERAL_ROOT_SPEC>;
#[doc = "Clock Peripheral Root"]
pub mod clk_peripheral_root;
#[doc = "clk_bus_root (rw) register accessor: Clock Bus Root\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`clk_bus_root::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`clk_bus_root::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@clk_bus_root`]
module"]
pub type CLK_BUS_ROOT = crate::Reg<clk_bus_root::CLK_BUS_ROOT_SPEC>;
#[doc = "Clock Bus Root"]
pub mod clk_bus_root;
#[doc = "clk_nocstg_bus (rw) register accessor: Clock NOCSTG Bus\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`clk_nocstg_bus::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`clk_nocstg_bus::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@clk_nocstg_bus`]
module"]
pub type CLK_NOCSTG_BUS = crate::Reg<clk_nocstg_bus::CLK_NOCSTG_BUS_SPEC>;
#[doc = "Clock NOCSTG Bus"]
pub mod clk_nocstg_bus;
#[doc = "clk_axi_cfg0 (rw) register accessor: Clock AXI Configuration 0\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`clk_axi_cfg0::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`clk_axi_cfg0::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@clk_axi_cfg0`]
module"]
pub type CLK_AXI_CFG0 = crate::Reg<clk_axi_cfg0::CLK_AXI_CFG0_SPEC>;
#[doc = "Clock AXI Configuration 0"]
pub mod clk_axi_cfg0;
#[doc = "clk_stg_axiahb (rw) register accessor: Clock STG AXI AHB\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`clk_stg_axiahb::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`clk_stg_axiahb::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@clk_stg_axiahb`]
module"]
pub type CLK_STG_AXIAHB = crate::Reg<clk_stg_axiahb::CLK_STG_AXIAHB_SPEC>;
#[doc = "Clock STG AXI AHB"]
pub mod clk_stg_axiahb;
#[doc = "clk_ahb0 (rw) register accessor: Clock AHB 0\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`clk_ahb0::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`clk_ahb0::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@clk_ahb0`]
module"]
pub type CLK_AHB0 = crate::Reg<clk_ahb0::CLK_AHB0_SPEC>;
#[doc = "Clock AHB 0"]
pub mod clk_ahb0;
#[doc = "clk_ahb1 (rw) register accessor: Clock AHB 1\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`clk_ahb1::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`clk_ahb1::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@clk_ahb1`]
module"]
pub type CLK_AHB1 = crate::Reg<clk_ahb1::CLK_AHB1_SPEC>;
#[doc = "Clock AHB 1"]
pub mod clk_ahb1;
#[doc = "clk_apb_bus (rw) register accessor: Clock APB Bus\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`clk_apb_bus::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`clk_apb_bus::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@clk_apb_bus`]
module"]
pub type CLK_APB_BUS = crate::Reg<clk_apb_bus::CLK_APB_BUS_SPEC>;
#[doc = "Clock APB Bus"]
pub mod clk_apb_bus;
#[doc = "clk_apb0 (rw) register accessor: Clock APB 0\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`clk_apb0::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`clk_apb0::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@clk_apb0`]
module"]
pub type CLK_APB0 = crate::Reg<clk_apb0::CLK_APB0_SPEC>;
#[doc = "Clock APB 0"]
pub mod clk_apb0;
#[doc = "clk_pll0_div2 (rw) register accessor: Clock PLL 0 Divider 2\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`clk_pll0_div2::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`clk_pll0_div2::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@clk_pll0_div2`]
module"]
pub type CLK_PLL0_DIV2 = crate::Reg<clk_pll0_div2::CLK_PLL0_DIV2_SPEC>;
#[doc = "Clock PLL 0 Divider 2"]
pub mod clk_pll0_div2;
#[doc = "clk_pll1_div2 (rw) register accessor: Clock PLL 1 Divider 2\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`clk_pll1_div2::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`clk_pll1_div2::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@clk_pll1_div2`]
module"]
pub type CLK_PLL1_DIV2 = crate::Reg<clk_pll1_div2::CLK_PLL1_DIV2_SPEC>;
#[doc = "Clock PLL 1 Divider 2"]
pub mod clk_pll1_div2;
#[doc = "clk_pll2_div2 (rw) register accessor: Clock PLL 2 Divider 2\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`clk_pll2_div2::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`clk_pll2_div2::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@clk_pll2_div2`]
module"]
pub type CLK_PLL2_DIV2 = crate::Reg<clk_pll2_div2::CLK_PLL2_DIV2_SPEC>;
#[doc = "Clock PLL 2 Divider 2"]
pub mod clk_pll2_div2;
#[doc = "clk_audio_root (rw) register accessor: Clock Audio Root\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`clk_audio_root::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`clk_audio_root::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@clk_audio_root`]
module"]
pub type CLK_AUDIO_ROOT = crate::Reg<clk_audio_root::CLK_AUDIO_ROOT_SPEC>;
#[doc = "Clock Audio Root"]
pub mod clk_audio_root;
#[doc = "clk_mclk_inner (rw) register accessor: Clock MCLK Inner\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`clk_mclk_inner::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`clk_mclk_inner::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@clk_mclk_inner`]
module"]
pub type CLK_MCLK_INNER = crate::Reg<clk_mclk_inner::CLK_MCLK_INNER_SPEC>;
#[doc = "Clock MCLK Inner"]
pub mod clk_mclk_inner;
#[doc = "clk_mclk (rw) register accessor: Clock MCLK\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`clk_mclk::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`clk_mclk::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@clk_mclk`]
module"]
pub type CLK_MCLK = crate::Reg<clk_mclk::CLK_MCLK_SPEC>;
#[doc = "Clock MCLK"]
pub mod clk_mclk;
#[doc = "clk_mclk_out (rw) register accessor: Clock MCLK Out\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`clk_mclk_out::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`clk_mclk_out::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@clk_mclk_out`]
module"]
pub type CLK_MCLK_OUT = crate::Reg<clk_mclk_out::CLK_MCLK_OUT_SPEC>;
#[doc = "Clock MCLK Out"]
pub mod clk_mclk_out;
#[doc = "clk_isp_2x (rw) register accessor: Clock ISP 2x\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`clk_isp_2x::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`clk_isp_2x::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@clk_isp_2x`]
module"]
pub type CLK_ISP_2X = crate::Reg<clk_isp_2x::CLK_ISP_2X_SPEC>;
#[doc = "Clock ISP 2x"]
pub mod clk_isp_2x;
#[doc = "clk_isp_axi (rw) register accessor: Clock ISP AXI\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`clk_isp_axi::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`clk_isp_axi::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@clk_isp_axi`]
module"]
pub type CLK_ISP_AXI = crate::Reg<clk_isp_axi::CLK_ISP_AXI_SPEC>;
#[doc = "Clock ISP AXI"]
pub mod clk_isp_axi;
#[doc = "clk_gclk0 (rw) register accessor: Clock GCLK 0\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`clk_gclk0::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`clk_gclk0::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@clk_gclk0`]
module"]
pub type CLK_GCLK0 = crate::Reg<clk_gclk0::CLK_GCLK0_SPEC>;
#[doc = "Clock GCLK 0"]
pub mod clk_gclk0;
#[doc = "clk_gclk1 (rw) register accessor: Clock GCLK 1\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`clk_gclk1::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`clk_gclk1::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@clk_gclk1`]
module"]
pub type CLK_GCLK1 = crate::Reg<clk_gclk1::CLK_GCLK1_SPEC>;
#[doc = "Clock GCLK 1"]
pub mod clk_gclk1;
#[doc = "clk_gclk2 (rw) register accessor: Clock GCLK 2\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`clk_gclk2::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`clk_gclk2::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@clk_gclk2`]
module"]
pub type CLK_GCLK2 = crate::Reg<clk_gclk2::CLK_GCLK2_SPEC>;
#[doc = "Clock GCLK 2"]
pub mod clk_gclk2;
#[doc = "clk_u7mc_core0 (rw) register accessor: U7MC Core Clock 0\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`clk_u7mc_core0::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`clk_u7mc_core0::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@clk_u7mc_core0`]
module"]
pub type CLK_U7MC_CORE0 = crate::Reg<clk_u7mc_core0::CLK_U7MC_CORE0_SPEC>;
#[doc = "U7MC Core Clock 0"]
pub mod clk_u7mc_core0;
#[doc = "clk_u7mc_core1 (rw) register accessor: U7MC Core Clock 1\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`clk_u7mc_core1::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`clk_u7mc_core1::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@clk_u7mc_core1`]
module"]
pub type CLK_U7MC_CORE1 = crate::Reg<clk_u7mc_core1::CLK_U7MC_CORE1_SPEC>;
#[doc = "U7MC Core Clock 1"]
pub mod clk_u7mc_core1;
#[doc = "clk_u7mc_core2 (rw) register accessor: U7MC Core Clock 2\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`clk_u7mc_core2::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`clk_u7mc_core2::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@clk_u7mc_core2`]
module"]
pub type CLK_U7MC_CORE2 = crate::Reg<clk_u7mc_core2::CLK_U7MC_CORE2_SPEC>;
#[doc = "U7MC Core Clock 2"]
pub mod clk_u7mc_core2;
#[doc = "clk_u7mc_core3 (rw) register accessor: U7MC Core Clock 3\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`clk_u7mc_core3::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`clk_u7mc_core3::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@clk_u7mc_core3`]
module"]
pub type CLK_U7MC_CORE3 = crate::Reg<clk_u7mc_core3::CLK_U7MC_CORE3_SPEC>;
#[doc = "U7MC Core Clock 3"]
pub mod clk_u7mc_core3;
#[doc = "clk_u7mc_core4 (rw) register accessor: U7MC Core Clock 4\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`clk_u7mc_core4::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`clk_u7mc_core4::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@clk_u7mc_core4`]
module"]
pub type CLK_U7MC_CORE4 = crate::Reg<clk_u7mc_core4::CLK_U7MC_CORE4_SPEC>;
#[doc = "U7MC Core Clock 4"]
pub mod clk_u7mc_core4;
#[doc = "clk_u7mc_debug (rw) register accessor: U7MC Debug Clock\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`clk_u7mc_debug::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`clk_u7mc_debug::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@clk_u7mc_debug`]
module"]
pub type CLK_U7MC_DEBUG = crate::Reg<clk_u7mc_debug::CLK_U7MC_DEBUG_SPEC>;
#[doc = "U7MC Debug Clock"]
pub mod clk_u7mc_debug;
#[doc = "u7mc_rtc_toggle (rw) register accessor: U7MC RTC Toggle\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`u7mc_rtc_toggle::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`u7mc_rtc_toggle::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@u7mc_rtc_toggle`]
module"]
pub type U7MC_RTC_TOGGLE = crate::Reg<u7mc_rtc_toggle::U7MC_RTC_TOGGLE_SPEC>;
#[doc = "U7MC RTC Toggle"]
pub mod u7mc_rtc_toggle;
#[doc = "clk_u7mc_trace0 (rw) register accessor: U7MC Trace Clock 0\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`clk_u7mc_trace0::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`clk_u7mc_trace0::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@clk_u7mc_trace0`]
module"]
pub type CLK_U7MC_TRACE0 = crate::Reg<clk_u7mc_trace0::CLK_U7MC_TRACE0_SPEC>;
#[doc = "U7MC Trace Clock 0"]
pub mod clk_u7mc_trace0;
#[doc = "clk_u7mc_trace1 (rw) register accessor: U7MC Trace Clock 1\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`clk_u7mc_trace1::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`clk_u7mc_trace1::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@clk_u7mc_trace1`]
module"]
pub type CLK_U7MC_TRACE1 = crate::Reg<clk_u7mc_trace1::CLK_U7MC_TRACE1_SPEC>;
#[doc = "U7MC Trace Clock 1"]
pub mod clk_u7mc_trace1;
#[doc = "clk_u7mc_trace2 (rw) register accessor: U7MC Trace Clock 2\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`clk_u7mc_trace2::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`clk_u7mc_trace2::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@clk_u7mc_trace2`]
module"]
pub type CLK_U7MC_TRACE2 = crate::Reg<clk_u7mc_trace2::CLK_U7MC_TRACE2_SPEC>;
#[doc = "U7MC Trace Clock 2"]
pub mod clk_u7mc_trace2;
#[doc = "clk_u7mc_trace3 (rw) register accessor: U7MC Trace Clock 3\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`clk_u7mc_trace3::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`clk_u7mc_trace3::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@clk_u7mc_trace3`]
module"]
pub type CLK_U7MC_TRACE3 = crate::Reg<clk_u7mc_trace3::CLK_U7MC_TRACE3_SPEC>;
#[doc = "U7MC Trace Clock 3"]
pub mod clk_u7mc_trace3;
#[doc = "clk_u7mc_trace4 (rw) register accessor: U7MC Trace Clock 4\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`clk_u7mc_trace4::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`clk_u7mc_trace4::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@clk_u7mc_trace4`]
module"]
pub type CLK_U7MC_TRACE4 = crate::Reg<clk_u7mc_trace4::CLK_U7MC_TRACE4_SPEC>;
#[doc = "U7MC Trace Clock 4"]
pub mod clk_u7mc_trace4;
#[doc = "clk_u7mc_trace_com (rw) register accessor: U7MC Trace Clock COM\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`clk_u7mc_trace_com::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`clk_u7mc_trace_com::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@clk_u7mc_trace_com`]
module"]
pub type CLK_U7MC_TRACE_COM = crate::Reg<clk_u7mc_trace_com::CLK_U7MC_TRACE_COM_SPEC>;
#[doc = "U7MC Trace Clock COM"]
pub mod clk_u7mc_trace_com;
#[doc = "clk_u0_sft7110_noc_bus_clk_cpu_axi (rw) register accessor: clk_u0_sft7110_noc_bus_clk_cpu_axi\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`clk_u0_sft7110_noc_bus_clk_cpu_axi::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`clk_u0_sft7110_noc_bus_clk_cpu_axi::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@clk_u0_sft7110_noc_bus_clk_cpu_axi`]
module"]
pub type CLK_U0_SFT7110_NOC_BUS_CLK_CPU_AXI =
    crate::Reg<clk_u0_sft7110_noc_bus_clk_cpu_axi::CLK_U0_SFT7110_NOC_BUS_CLK_CPU_AXI_SPEC>;
#[doc = "clk_u0_sft7110_noc_bus_clk_cpu_axi"]
pub mod clk_u0_sft7110_noc_bus_clk_cpu_axi;
#[doc = "clk_u0_sft7110_noc_bus_clk_axicfg0_axi (rw) register accessor: clk_u0_sft7110_noc_bus_clk_axicfg0_axi\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`clk_u0_sft7110_noc_bus_clk_axicfg0_axi::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`clk_u0_sft7110_noc_bus_clk_axicfg0_axi::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@clk_u0_sft7110_noc_bus_clk_axicfg0_axi`]
module"]
pub type CLK_U0_SFT7110_NOC_BUS_CLK_AXICFG0_AXI =
    crate::Reg<clk_u0_sft7110_noc_bus_clk_axicfg0_axi::CLK_U0_SFT7110_NOC_BUS_CLK_AXICFG0_AXI_SPEC>;
#[doc = "clk_u0_sft7110_noc_bus_clk_axicfg0_axi"]
pub mod clk_u0_sft7110_noc_bus_clk_axicfg0_axi;
#[doc = "clk_osc_div2 (rw) register accessor: clk_osc_div2\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`clk_osc_div2::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`clk_osc_div2::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@clk_osc_div2`]
module"]
pub type CLK_OSC_DIV2 = crate::Reg<clk_osc_div2::CLK_OSC_DIV2_SPEC>;
#[doc = "clk_osc_div2"]
pub mod clk_osc_div2;
#[doc = "clk_pll1_div4 (rw) register accessor: clk_pll1_div4\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`clk_pll1_div4::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`clk_pll1_div4::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@clk_pll1_div4`]
module"]
pub type CLK_PLL1_DIV4 = crate::Reg<clk_pll1_div4::CLK_PLL1_DIV4_SPEC>;
#[doc = "clk_pll1_div4"]
pub mod clk_pll1_div4;
#[doc = "clk_pll1_div8 (rw) register accessor: clk_pll1_div8\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`clk_pll1_div8::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`clk_pll1_div8::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@clk_pll1_div8`]
module"]
pub type CLK_PLL1_DIV8 = crate::Reg<clk_pll1_div8::CLK_PLL1_DIV8_SPEC>;
#[doc = "clk_pll1_div8"]
pub mod clk_pll1_div8;
#[doc = "clk_ddr_bus (rw) register accessor: clk_ddr_bus\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`clk_ddr_bus::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`clk_ddr_bus::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@clk_ddr_bus`]
module"]
pub type CLK_DDR_BUS = crate::Reg<clk_ddr_bus::CLK_DDR_BUS_SPEC>;
#[doc = "clk_ddr_bus"]
pub mod clk_ddr_bus;
#[doc = "clk_u0_ddr_sft7110_clk_axi (rw) register accessor: clk_u0_ddr_sfft7110_clk_axi\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`clk_u0_ddr_sft7110_clk_axi::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`clk_u0_ddr_sft7110_clk_axi::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@clk_u0_ddr_sft7110_clk_axi`]
module"]
pub type CLK_U0_DDR_SFT7110_CLK_AXI =
    crate::Reg<clk_u0_ddr_sft7110_clk_axi::CLK_U0_DDR_SFT7110_CLK_AXI_SPEC>;
#[doc = "clk_u0_ddr_sfft7110_clk_axi"]
pub mod clk_u0_ddr_sft7110_clk_axi;
#[doc = "clk_gpu_core (rw) register accessor: clk_gpu_core\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`clk_gpu_core::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`clk_gpu_core::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@clk_gpu_core`]
module"]
pub type CLK_GPU_CORE = crate::Reg<clk_gpu_core::CLK_GPU_CORE_SPEC>;
#[doc = "clk_gpu_core"]
pub mod clk_gpu_core;
#[doc = "clk_u0_img_gpu_core_clk (rw) register accessor: clk_u0_img_gpu_core_clk\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`clk_u0_img_gpu_core_clk::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`clk_u0_img_gpu_core_clk::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@clk_u0_img_gpu_core_clk`]
module"]
pub type CLK_U0_IMG_GPU_CORE_CLK =
    crate::Reg<clk_u0_img_gpu_core_clk::CLK_U0_IMG_GPU_CORE_CLK_SPEC>;
#[doc = "clk_u0_img_gpu_core_clk"]
pub mod clk_u0_img_gpu_core_clk;
#[doc = "clk_u0_img_gpu_sys_clk (rw) register accessor: clk_u0_img_gpu_sys_clk\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`clk_u0_img_gpu_sys_clk::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`clk_u0_img_gpu_sys_clk::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@clk_u0_img_gpu_sys_clk`]
module"]
pub type CLK_U0_IMG_GPU_SYS_CLK = crate::Reg<clk_u0_img_gpu_sys_clk::CLK_U0_IMG_GPU_SYS_CLK_SPEC>;
#[doc = "clk_u0_img_gpu_sys_clk"]
pub mod clk_u0_img_gpu_sys_clk;
#[doc = "clk_u0_img_gpu_clk_apb (rw) register accessor: clk_u0_img_gpu_clk_apb\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`clk_u0_img_gpu_clk_apb::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`clk_u0_img_gpu_clk_apb::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@clk_u0_img_gpu_clk_apb`]
module"]
pub type CLK_U0_IMG_GPU_CLK_APB = crate::Reg<clk_u0_img_gpu_clk_apb::CLK_U0_IMG_GPU_CLK_APB_SPEC>;
#[doc = "clk_u0_img_gpu_clk_apb"]
pub mod clk_u0_img_gpu_clk_apb;
#[doc = "clk_u0_gpu_rtc_toggle (rw) register accessor: clk_u0_gpu_rtc_toggle\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`clk_u0_gpu_rtc_toggle::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`clk_u0_gpu_rtc_toggle::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@clk_u0_gpu_rtc_toggle`]
module"]
pub type CLK_U0_GPU_RTC_TOGGLE = crate::Reg<clk_u0_gpu_rtc_toggle::CLK_U0_GPU_RTC_TOGGLE_SPEC>;
#[doc = "clk_u0_gpu_rtc_toggle"]
pub mod clk_u0_gpu_rtc_toggle;
#[doc = "clk_u0_sft7110_noc_bus_clk_gpu_axi (rw) register accessor: clk_u0_sft7110_noc_bus_clk_gpu_axi\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`clk_u0_sft7110_noc_bus_clk_gpu_axi::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`clk_u0_sft7110_noc_bus_clk_gpu_axi::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@clk_u0_sft7110_noc_bus_clk_gpu_axi`]
module"]
pub type CLK_U0_SFT7110_NOC_BUS_CLK_GPU_AXI =
    crate::Reg<clk_u0_sft7110_noc_bus_clk_gpu_axi::CLK_U0_SFT7110_NOC_BUS_CLK_GPU_AXI_SPEC>;
#[doc = "clk_u0_sft7110_noc_bus_clk_gpu_axi"]
pub mod clk_u0_sft7110_noc_bus_clk_gpu_axi;
#[doc = "clk_u0_dom_isp_top_clk_dom_isp_top_clk_ispcore_2x (rw) register accessor: clk_u0_dom_isp_top_clk_dom_isp_top_clk_ispcore_2x\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`clk_u0_dom_isp_top_clk_dom_isp_top_clk_ispcore_2x::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`clk_u0_dom_isp_top_clk_dom_isp_top_clk_ispcore_2x::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@clk_u0_dom_isp_top_clk_dom_isp_top_clk_ispcore_2x`]
module"]
pub type CLK_U0_DOM_ISP_TOP_CLK_DOM_ISP_TOP_CLK_ISPCORE_2X = crate :: Reg < clk_u0_dom_isp_top_clk_dom_isp_top_clk_ispcore_2x :: CLK_U0_DOM_ISP_TOP_CLK_DOM_ISP_TOP_CLK_ISPCORE_2X_SPEC > ;
#[doc = "clk_u0_dom_isp_top_clk_dom_isp_top_clk_ispcore_2x"]
pub mod clk_u0_dom_isp_top_clk_dom_isp_top_clk_ispcore_2x;
#[doc = "clk_u0_dom_isp_top_clk_dom_isp_top_clk_isp_axi (rw) register accessor: clk_u0_dom_isp_top_clk_dom_isp_top_clk_isp_axi\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`clk_u0_dom_isp_top_clk_dom_isp_top_clk_isp_axi::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`clk_u0_dom_isp_top_clk_dom_isp_top_clk_isp_axi::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@clk_u0_dom_isp_top_clk_dom_isp_top_clk_isp_axi`]
module"]
pub type CLK_U0_DOM_ISP_TOP_CLK_DOM_ISP_TOP_CLK_ISP_AXI = crate :: Reg < clk_u0_dom_isp_top_clk_dom_isp_top_clk_isp_axi :: CLK_U0_DOM_ISP_TOP_CLK_DOM_ISP_TOP_CLK_ISP_AXI_SPEC > ;
#[doc = "clk_u0_dom_isp_top_clk_dom_isp_top_clk_isp_axi"]
pub mod clk_u0_dom_isp_top_clk_dom_isp_top_clk_isp_axi;
#[doc = "clk_u0_sft7110_noc_bux_clk_isp_axi (rw) register accessor: clk_u0_sft7110_noc_bux_clk_isp_axi\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`clk_u0_sft7110_noc_bux_clk_isp_axi::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`clk_u0_sft7110_noc_bux_clk_isp_axi::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@clk_u0_sft7110_noc_bux_clk_isp_axi`]
module"]
pub type CLK_U0_SFT7110_NOC_BUX_CLK_ISP_AXI =
    crate::Reg<clk_u0_sft7110_noc_bux_clk_isp_axi::CLK_U0_SFT7110_NOC_BUX_CLK_ISP_AXI_SPEC>;
#[doc = "clk_u0_sft7110_noc_bux_clk_isp_axi"]
pub mod clk_u0_sft7110_noc_bux_clk_isp_axi;
#[doc = "clk_hifi4_core (rw) register accessor: clk_hifi4_core\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`clk_hifi4_core::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`clk_hifi4_core::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@clk_hifi4_core`]
module"]
pub type CLK_HIFI4_CORE = crate::Reg<clk_hifi4_core::CLK_HIFI4_CORE_SPEC>;
#[doc = "clk_hifi4_core"]
pub mod clk_hifi4_core;
#[doc = "clk_hifi4_axi (rw) register accessor: clk_hifi4_axi\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`clk_hifi4_axi::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`clk_hifi4_axi::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@clk_hifi4_axi`]
module"]
pub type CLK_HIFI4_AXI = crate::Reg<clk_hifi4_axi::CLK_HIFI4_AXI_SPEC>;
#[doc = "clk_hifi4_axi"]
pub mod clk_hifi4_axi;
#[doc = "clk_u0_axi_cfg1_dec_clk_main (rw) register accessor: clk_u0_axi_cfg1_dec_clk_main\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`clk_u0_axi_cfg1_dec_clk_main::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`clk_u0_axi_cfg1_dec_clk_main::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@clk_u0_axi_cfg1_dec_clk_main`]
module"]
pub type CLK_U0_AXI_CFG1_DEC_CLK_MAIN =
    crate::Reg<clk_u0_axi_cfg1_dec_clk_main::CLK_U0_AXI_CFG1_DEC_CLK_MAIN_SPEC>;
#[doc = "clk_u0_axi_cfg1_dec_clk_main"]
pub mod clk_u0_axi_cfg1_dec_clk_main;
#[doc = "clk_u0_axi_cfg1_dec_clk_ahb (rw) register accessor: clk_u0_axi_cfg1_dec_clk_ahb\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`clk_u0_axi_cfg1_dec_clk_ahb::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`clk_u0_axi_cfg1_dec_clk_ahb::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@clk_u0_axi_cfg1_dec_clk_ahb`]
module"]
pub type CLK_U0_AXI_CFG1_DEC_CLK_AHB =
    crate::Reg<clk_u0_axi_cfg1_dec_clk_ahb::CLK_U0_AXI_CFG1_DEC_CLK_AHB_SPEC>;
#[doc = "clk_u0_axi_cfg1_dec_clk_ahb"]
pub mod clk_u0_axi_cfg1_dec_clk_ahb;
#[doc = "clk_u0_dom_vout_top_clk_dom_vout_top_clk_vout_src (rw) register accessor: clk_u0_dom_vout_top_clk_dom_vout_top_clk_vout_src\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`clk_u0_dom_vout_top_clk_dom_vout_top_clk_vout_src::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`clk_u0_dom_vout_top_clk_dom_vout_top_clk_vout_src::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@clk_u0_dom_vout_top_clk_dom_vout_top_clk_vout_src`]
module"]
pub type CLK_U0_DOM_VOUT_TOP_CLK_DOM_VOUT_TOP_CLK_VOUT_SRC = crate :: Reg < clk_u0_dom_vout_top_clk_dom_vout_top_clk_vout_src :: CLK_U0_DOM_VOUT_TOP_CLK_DOM_VOUT_TOP_CLK_VOUT_SRC_SPEC > ;
#[doc = "clk_u0_dom_vout_top_clk_dom_vout_top_clk_vout_src"]
pub mod clk_u0_dom_vout_top_clk_dom_vout_top_clk_vout_src;
#[doc = "clk_vout_axi_divcfg (rw) register accessor: Clock Video Output AXI DIVCFG\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`clk_vout_axi_divcfg::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`clk_vout_axi_divcfg::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@clk_vout_axi_divcfg`]
module"]
pub type CLK_VOUT_AXI_DIVCFG = crate::Reg<clk_vout_axi_divcfg::CLK_VOUT_AXI_DIVCFG_SPEC>;
#[doc = "Clock Video Output AXI DIVCFG"]
pub mod clk_vout_axi_divcfg;
#[doc = "clk_noc_display_axi (rw) register accessor: Clock NOC Display AXI\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`clk_noc_display_axi::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`clk_noc_display_axi::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@clk_noc_display_axi`]
module"]
pub type CLK_NOC_DISPLAY_AXI = crate::Reg<clk_noc_display_axi::CLK_NOC_DISPLAY_AXI_SPEC>;
#[doc = "Clock NOC Display AXI"]
pub mod clk_noc_display_axi;
#[doc = "clk_vout_ahb (rw) register accessor: Clock Video Output AHB\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`clk_vout_ahb::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`clk_vout_ahb::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@clk_vout_ahb`]
module"]
pub type CLK_VOUT_AHB = crate::Reg<clk_vout_ahb::CLK_VOUT_AHB_SPEC>;
#[doc = "Clock Video Output AHB"]
pub mod clk_vout_ahb;
#[doc = "clk_vout_axi_icg (rw) register accessor: Clock Video Output AXI ICG\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`clk_vout_axi_icg::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`clk_vout_axi_icg::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@clk_vout_axi_icg`]
module"]
pub type CLK_VOUT_AXI_ICG = crate::Reg<clk_vout_axi_icg::CLK_VOUT_AXI_ICG_SPEC>;
#[doc = "Clock Video Output AXI ICG"]
pub mod clk_vout_axi_icg;
#[doc = "clk_vout_hdmi_tx0_mclk (rw) register accessor: Clock Video Output HDMI TX0 MCLK\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`clk_vout_hdmi_tx0_mclk::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`clk_vout_hdmi_tx0_mclk::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@clk_vout_hdmi_tx0_mclk`]
module"]
pub type CLK_VOUT_HDMI_TX0_MCLK = crate::Reg<clk_vout_hdmi_tx0_mclk::CLK_VOUT_HDMI_TX0_MCLK_SPEC>;
#[doc = "Clock Video Output HDMI TX0 MCLK"]
pub mod clk_vout_hdmi_tx0_mclk;
#[doc = "clk_vout_mipi_phy (rw) register accessor: Clock Video Output MIPI PHY Reference\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`clk_vout_mipi_phy::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`clk_vout_mipi_phy::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@clk_vout_mipi_phy`]
module"]
pub type CLK_VOUT_MIPI_PHY = crate::Reg<clk_vout_mipi_phy::CLK_VOUT_MIPI_PHY_SPEC>;
#[doc = "Clock Video Output MIPI PHY Reference"]
pub mod clk_vout_mipi_phy;
#[doc = "clk_jpeg_codec_axi (rw) register accessor: Clock JPEG Codec AXI\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`clk_jpeg_codec_axi::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`clk_jpeg_codec_axi::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@clk_jpeg_codec_axi`]
module"]
pub type CLK_JPEG_CODEC_AXI = crate::Reg<clk_jpeg_codec_axi::CLK_JPEG_CODEC_AXI_SPEC>;
#[doc = "Clock JPEG Codec AXI"]
pub mod clk_jpeg_codec_axi;
#[doc = "clk_codaj12_axi (rw) register accessor: CODAJ12 Clock AXI\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`clk_codaj12_axi::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`clk_codaj12_axi::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@clk_codaj12_axi`]
module"]
pub type CLK_CODAJ12_AXI = crate::Reg<clk_codaj12_axi::CLK_CODAJ12_AXI_SPEC>;
#[doc = "CODAJ12 Clock AXI"]
pub mod clk_codaj12_axi;
#[doc = "clk_codaj12_core (rw) register accessor: CODAJ12 Clock Core\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`clk_codaj12_core::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`clk_codaj12_core::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@clk_codaj12_core`]
module"]
pub type CLK_CODAJ12_CORE = crate::Reg<clk_codaj12_core::CLK_CODAJ12_CORE_SPEC>;
#[doc = "CODAJ12 Clock Core"]
pub mod clk_codaj12_core;
#[doc = "clk_codaj12_apb (rw) register accessor: CODAJ12 Clock APB\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`clk_codaj12_apb::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`clk_codaj12_apb::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@clk_codaj12_apb`]
module"]
pub type CLK_CODAJ12_APB = crate::Reg<clk_codaj12_apb::CLK_CODAJ12_APB_SPEC>;
#[doc = "CODAJ12 Clock APB"]
pub mod clk_codaj12_apb;
#[doc = "clk_vdec_axi (rw) register accessor: Clock Video Decoder AXI\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`clk_vdec_axi::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`clk_vdec_axi::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@clk_vdec_axi`]
module"]
pub type CLK_VDEC_AXI = crate::Reg<clk_vdec_axi::CLK_VDEC_AXI_SPEC>;
#[doc = "Clock Video Decoder AXI"]
pub mod clk_vdec_axi;
#[doc = "clk_wave511_axi (rw) register accessor: Clock WAVE511 AXI\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`clk_wave511_axi::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`clk_wave511_axi::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@clk_wave511_axi`]
module"]
pub type CLK_WAVE511_AXI = crate::Reg<clk_wave511_axi::CLK_WAVE511_AXI_SPEC>;
#[doc = "Clock WAVE511 AXI"]
pub mod clk_wave511_axi;
#[doc = "clk_wave511_bpu (rw) register accessor: Clock WAVE511 BPU\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`clk_wave511_bpu::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`clk_wave511_bpu::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@clk_wave511_bpu`]
module"]
pub type CLK_WAVE511_BPU = crate::Reg<clk_wave511_bpu::CLK_WAVE511_BPU_SPEC>;
#[doc = "Clock WAVE511 BPU"]
pub mod clk_wave511_bpu;
#[doc = "clk_wave511_vce (rw) register accessor: Clock WAVE511 VCE\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`clk_wave511_vce::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`clk_wave511_vce::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@clk_wave511_vce`]
module"]
pub type CLK_WAVE511_VCE = crate::Reg<clk_wave511_vce::CLK_WAVE511_VCE_SPEC>;
#[doc = "Clock WAVE511 VCE"]
pub mod clk_wave511_vce;
#[doc = "clk_wave511_apb (rw) register accessor: Clock WAVE511 APB\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`clk_wave511_apb::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`clk_wave511_apb::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@clk_wave511_apb`]
module"]
pub type CLK_WAVE511_APB = crate::Reg<clk_wave511_apb::CLK_WAVE511_APB_SPEC>;
#[doc = "Clock WAVE511 APB"]
pub mod clk_wave511_apb;
#[doc = "clk_wave511_jpg_arb (rw) register accessor: Clock WAVE511 JPG ARB\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`clk_wave511_jpg_arb::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`clk_wave511_jpg_arb::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@clk_wave511_jpg_arb`]
module"]
pub type CLK_WAVE511_JPG_ARB = crate::Reg<clk_wave511_jpg_arb::CLK_WAVE511_JPG_ARB_SPEC>;
#[doc = "Clock WAVE511 JPG ARB"]
pub mod clk_wave511_jpg_arb;
#[doc = "clk_wave511_jpg_main (rw) register accessor: Clock WAVE511 JPG Main\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`clk_wave511_jpg_main::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`clk_wave511_jpg_main::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@clk_wave511_jpg_main`]
module"]
pub type CLK_WAVE511_JPG_MAIN = crate::Reg<clk_wave511_jpg_main::CLK_WAVE511_JPG_MAIN_SPEC>;
#[doc = "Clock WAVE511 JPG Main"]
pub mod clk_wave511_jpg_main;
#[doc = "clk_noc_vdec_axi (rw) register accessor: Clock NOC Video Decoder AXI\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`clk_noc_vdec_axi::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`clk_noc_vdec_axi::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@clk_noc_vdec_axi`]
module"]
pub type CLK_NOC_VDEC_AXI = crate::Reg<clk_noc_vdec_axi::CLK_NOC_VDEC_AXI_SPEC>;
#[doc = "Clock NOC Video Decoder AXI"]
pub mod clk_noc_vdec_axi;
#[doc = "clk_venc_axi (rw) register accessor: Clock Video Encoder AXI\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`clk_venc_axi::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`clk_venc_axi::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@clk_venc_axi`]
module"]
pub type CLK_VENC_AXI = crate::Reg<clk_venc_axi::CLK_VENC_AXI_SPEC>;
#[doc = "Clock Video Encoder AXI"]
pub mod clk_venc_axi;
#[doc = "clk_wave420l_axi (rw) register accessor: Clock WAVE420L AXI\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`clk_wave420l_axi::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`clk_wave420l_axi::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@clk_wave420l_axi`]
module"]
pub type CLK_WAVE420L_AXI = crate::Reg<clk_wave420l_axi::CLK_WAVE420L_AXI_SPEC>;
#[doc = "Clock WAVE420L AXI"]
pub mod clk_wave420l_axi;
#[doc = "clk_wave420l_bpu (rw) register accessor: Clock WAVE420L BPU\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`clk_wave420l_bpu::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`clk_wave420l_bpu::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@clk_wave420l_bpu`]
module"]
pub type CLK_WAVE420L_BPU = crate::Reg<clk_wave420l_bpu::CLK_WAVE420L_BPU_SPEC>;
#[doc = "Clock WAVE420L BPU"]
pub mod clk_wave420l_bpu;
#[doc = "clk_wave420l_vce (rw) register accessor: Clock WAVE420L VCE\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`clk_wave420l_vce::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`clk_wave420l_vce::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@clk_wave420l_vce`]
module"]
pub type CLK_WAVE420L_VCE = crate::Reg<clk_wave420l_vce::CLK_WAVE420L_VCE_SPEC>;
#[doc = "Clock WAVE420L VCE"]
pub mod clk_wave420l_vce;
#[doc = "clk_wave420l_apb (rw) register accessor: Clock WAVE420L APB\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`clk_wave420l_apb::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`clk_wave420l_apb::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@clk_wave420l_apb`]
module"]
pub type CLK_WAVE420L_APB = crate::Reg<clk_wave420l_apb::CLK_WAVE420L_APB_SPEC>;
#[doc = "Clock WAVE420L APB"]
pub mod clk_wave420l_apb;
#[doc = "clk_noc_venc_axi (rw) register accessor: Clock NOC Video Encoder AXI\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`clk_noc_venc_axi::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`clk_noc_venc_axi::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@clk_noc_venc_axi`]
module"]
pub type CLK_NOC_VENC_AXI = crate::Reg<clk_noc_venc_axi::CLK_NOC_VENC_AXI_SPEC>;
#[doc = "Clock NOC Video Encoder AXI"]
pub mod clk_noc_venc_axi;
#[doc = "clk_axi_cfg0_dec_main_div (rw) register accessor: Clock AXI Config 0 DEC Main Divider\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`clk_axi_cfg0_dec_main_div::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`clk_axi_cfg0_dec_main_div::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@clk_axi_cfg0_dec_main_div`]
module"]
pub type CLK_AXI_CFG0_DEC_MAIN_DIV =
    crate::Reg<clk_axi_cfg0_dec_main_div::CLK_AXI_CFG0_DEC_MAIN_DIV_SPEC>;
#[doc = "Clock AXI Config 0 DEC Main Divider"]
pub mod clk_axi_cfg0_dec_main_div;
#[doc = "clk_axi_cfg0_dec_main (rw) register accessor: Clock AXI Config 0 DEC Main\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`clk_axi_cfg0_dec_main::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`clk_axi_cfg0_dec_main::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@clk_axi_cfg0_dec_main`]
module"]
pub type CLK_AXI_CFG0_DEC_MAIN = crate::Reg<clk_axi_cfg0_dec_main::CLK_AXI_CFG0_DEC_MAIN_SPEC>;
#[doc = "Clock AXI Config 0 DEC Main"]
pub mod clk_axi_cfg0_dec_main;
#[doc = "clk_axi_cfg0_dec_hifi4 (rw) register accessor: Clock AXI Config 0 DEC HIFI4\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`clk_axi_cfg0_dec_hifi4::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`clk_axi_cfg0_dec_hifi4::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@clk_axi_cfg0_dec_hifi4`]
module"]
pub type CLK_AXI_CFG0_DEC_HIFI4 = crate::Reg<clk_axi_cfg0_dec_hifi4::CLK_AXI_CFG0_DEC_HIFI4_SPEC>;
#[doc = "Clock AXI Config 0 DEC HIFI4"]
pub mod clk_axi_cfg0_dec_hifi4;
#[doc = "clk_aximem_128b_axi (rw) register accessor: Clock AXIMEM 128B AXI\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`clk_aximem_128b_axi::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`clk_aximem_128b_axi::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@clk_aximem_128b_axi`]
module"]
pub type CLK_AXIMEM_128B_AXI = crate::Reg<clk_aximem_128b_axi::CLK_AXIMEM_128B_AXI_SPEC>;
#[doc = "Clock AXIMEM 128B AXI"]
pub mod clk_aximem_128b_axi;
#[doc = "clk_qspi_ahb (rw) register accessor: Clock QSPI AHB\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`clk_qspi_ahb::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`clk_qspi_ahb::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@clk_qspi_ahb`]
module"]
pub type CLK_QSPI_AHB = crate::Reg<clk_qspi_ahb::CLK_QSPI_AHB_SPEC>;
#[doc = "Clock QSPI AHB"]
pub mod clk_qspi_ahb;
#[doc = "clk_qspi_apb (rw) register accessor: Clock QSPI APB\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`clk_qspi_apb::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`clk_qspi_apb::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@clk_qspi_apb`]
module"]
pub type CLK_QSPI_APB = crate::Reg<clk_qspi_apb::CLK_QSPI_APB_SPEC>;
#[doc = "Clock QSPI APB"]
pub mod clk_qspi_apb;
#[doc = "clk_qspi_ref_src (rw) register accessor: Clock QSPI Reference Source\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`clk_qspi_ref_src::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`clk_qspi_ref_src::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@clk_qspi_ref_src`]
module"]
pub type CLK_QSPI_REF_SRC = crate::Reg<clk_qspi_ref_src::CLK_QSPI_REF_SRC_SPEC>;
#[doc = "Clock QSPI Reference Source"]
pub mod clk_qspi_ref_src;
#[doc = "clk_qspi_ref (rw) register accessor: Clock QSPI Reference\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`clk_qspi_ref::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`clk_qspi_ref::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@clk_qspi_ref`]
module"]
pub type CLK_QSPI_REF = crate::Reg<clk_qspi_ref::CLK_QSPI_REF_SPEC>;
#[doc = "Clock QSPI Reference"]
pub mod clk_qspi_ref;
#[doc = "clk_u0_sd_ahb (rw) register accessor: U0 SD Clock AHB\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`clk_u0_sd_ahb::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`clk_u0_sd_ahb::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@clk_u0_sd_ahb`]
module"]
pub type CLK_U0_SD_AHB = crate::Reg<clk_u0_sd_ahb::CLK_U0_SD_AHB_SPEC>;
#[doc = "U0 SD Clock AHB"]
pub mod clk_u0_sd_ahb;
#[doc = "clk_u1_sd_ahb (rw) register accessor: U1 SD Clock AHB\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`clk_u1_sd_ahb::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`clk_u1_sd_ahb::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@clk_u1_sd_ahb`]
module"]
pub type CLK_U1_SD_AHB = crate::Reg<clk_u1_sd_ahb::CLK_U1_SD_AHB_SPEC>;
#[doc = "U1 SD Clock AHB"]
pub mod clk_u1_sd_ahb;
#[doc = "clk_u0_sd_card (rw) register accessor: U0 SD Card Clock\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`clk_u0_sd_card::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`clk_u0_sd_card::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@clk_u0_sd_card`]
module"]
pub type CLK_U0_SD_CARD = crate::Reg<clk_u0_sd_card::CLK_U0_SD_CARD_SPEC>;
#[doc = "U0 SD Card Clock"]
pub mod clk_u0_sd_card;
#[doc = "clk_u1_sd_card (rw) register accessor: U1 SD Card Clock\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`clk_u1_sd_card::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`clk_u1_sd_card::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@clk_u1_sd_card`]
module"]
pub type CLK_U1_SD_CARD = crate::Reg<clk_u1_sd_card::CLK_U1_SD_CARD_SPEC>;
#[doc = "U1 SD Card Clock"]
pub mod clk_u1_sd_card;
#[doc = "clk_usb_125m (rw) register accessor: Clock USB 125M\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`clk_usb_125m::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`clk_usb_125m::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@clk_usb_125m`]
module"]
pub type CLK_USB_125M = crate::Reg<clk_usb_125m::CLK_USB_125M_SPEC>;
#[doc = "Clock USB 125M"]
pub mod clk_usb_125m;
#[doc = "clk_noc_stg_axi (rw) register accessor: Clock NOC STG AXI\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`clk_noc_stg_axi::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`clk_noc_stg_axi::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@clk_noc_stg_axi`]
module"]
pub type CLK_NOC_STG_AXI = crate::Reg<clk_noc_stg_axi::CLK_NOC_STG_AXI_SPEC>;
#[doc = "Clock NOC STG AXI"]
pub mod clk_noc_stg_axi;
#[doc = "clk_gmac5_axi64_ahb (rw) register accessor: Clock GMAC 5 AXI 64 AHB\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`clk_gmac5_axi64_ahb::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`clk_gmac5_axi64_ahb::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@clk_gmac5_axi64_ahb`]
module"]
pub type CLK_GMAC5_AXI64_AHB = crate::Reg<clk_gmac5_axi64_ahb::CLK_GMAC5_AXI64_AHB_SPEC>;
#[doc = "Clock GMAC 5 AXI 64 AHB"]
pub mod clk_gmac5_axi64_ahb;
#[doc = "clk_gmac5_axi64_axi (rw) register accessor: Clock GMAC 5 AXI 64 AXI\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`clk_gmac5_axi64_axi::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`clk_gmac5_axi64_axi::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@clk_gmac5_axi64_axi`]
module"]
pub type CLK_GMAC5_AXI64_AXI = crate::Reg<clk_gmac5_axi64_axi::CLK_GMAC5_AXI64_AXI_SPEC>;
#[doc = "Clock GMAC 5 AXI 64 AXI"]
pub mod clk_gmac5_axi64_axi;
#[doc = "clk_gmac_src (rw) register accessor: Clock GMAC Source\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`clk_gmac_src::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`clk_gmac_src::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@clk_gmac_src`]
module"]
pub type CLK_GMAC_SRC = crate::Reg<clk_gmac_src::CLK_GMAC_SRC_SPEC>;
#[doc = "Clock GMAC Source"]
pub mod clk_gmac_src;
#[doc = "clk_gmac1_gtx (rw) register accessor: Clock GMAC 1 GTX\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`clk_gmac1_gtx::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`clk_gmac1_gtx::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@clk_gmac1_gtx`]
module"]
pub type CLK_GMAC1_GTX = crate::Reg<clk_gmac1_gtx::CLK_GMAC1_GTX_SPEC>;
#[doc = "Clock GMAC 1 GTX"]
pub mod clk_gmac1_gtx;
#[doc = "clk_gmac1_rmii_rtx (rw) register accessor: Clock GMAC 1 RMII RTX\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`clk_gmac1_rmii_rtx::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`clk_gmac1_rmii_rtx::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@clk_gmac1_rmii_rtx`]
module"]
pub type CLK_GMAC1_RMII_RTX = crate::Reg<clk_gmac1_rmii_rtx::CLK_GMAC1_RMII_RTX_SPEC>;
#[doc = "Clock GMAC 1 RMII RTX"]
pub mod clk_gmac1_rmii_rtx;
#[doc = "clk_gmac5_axi64_ptp (rw) register accessor: Clock GMAC 5 AXI 64 PTP\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`clk_gmac5_axi64_ptp::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`clk_gmac5_axi64_ptp::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@clk_gmac5_axi64_ptp`]
module"]
pub type CLK_GMAC5_AXI64_PTP = crate::Reg<clk_gmac5_axi64_ptp::CLK_GMAC5_AXI64_PTP_SPEC>;
#[doc = "Clock GMAC 5 AXI 64 PTP"]
pub mod clk_gmac5_axi64_ptp;
#[doc = "clk_gmac5_axi64_rx (rw) register accessor: Clock GMAC 5 AXI 64 RX\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`clk_gmac5_axi64_rx::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`clk_gmac5_axi64_rx::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@clk_gmac5_axi64_rx`]
module"]
pub type CLK_GMAC5_AXI64_RX = crate::Reg<clk_gmac5_axi64_rx::CLK_GMAC5_AXI64_RX_SPEC>;
#[doc = "Clock GMAC 5 AXI 64 RX"]
pub mod clk_gmac5_axi64_rx;
#[doc = "clk_gmac5_axi64_rxi (rw) register accessor: Clock GMAC 5 AXI 64 RX Inverter\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`clk_gmac5_axi64_rxi::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`clk_gmac5_axi64_rxi::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@clk_gmac5_axi64_rxi`]
module"]
pub type CLK_GMAC5_AXI64_RXI = crate::Reg<clk_gmac5_axi64_rxi::CLK_GMAC5_AXI64_RXI_SPEC>;
#[doc = "Clock GMAC 5 AXI 64 RX Inverter"]
pub mod clk_gmac5_axi64_rxi;
#[doc = "clk_gmac5_axi64_tx (rw) register accessor: Clock GMAC 5 AXI 64 TX\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`clk_gmac5_axi64_tx::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`clk_gmac5_axi64_tx::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@clk_gmac5_axi64_tx`]
module"]
pub type CLK_GMAC5_AXI64_TX = crate::Reg<clk_gmac5_axi64_tx::CLK_GMAC5_AXI64_TX_SPEC>;
#[doc = "Clock GMAC 5 AXI 64 TX"]
pub mod clk_gmac5_axi64_tx;
#[doc = "clk_gmac5_axi64_txi (rw) register accessor: Clock GMAC 5 AXI 64 TX Inverter\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`clk_gmac5_axi64_txi::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`clk_gmac5_axi64_txi::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@clk_gmac5_axi64_txi`]
module"]
pub type CLK_GMAC5_AXI64_TXI = crate::Reg<clk_gmac5_axi64_txi::CLK_GMAC5_AXI64_TXI_SPEC>;
#[doc = "Clock GMAC 5 AXI 64 TX Inverter"]
pub mod clk_gmac5_axi64_txi;
#[doc = "clk_gmac1_gtxclk (rw) register accessor: Clock GMAC 1 GTXC\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`clk_gmac1_gtxclk::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`clk_gmac1_gtxclk::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@clk_gmac1_gtxclk`]
module"]
pub type CLK_GMAC1_GTXCLK = crate::Reg<clk_gmac1_gtxclk::CLK_GMAC1_GTXCLK_SPEC>;
#[doc = "Clock GMAC 1 GTXC"]
pub mod clk_gmac1_gtxclk;
#[doc = "clk_gmac0_gtx (rw) register accessor: Clock GMAC 0 GTX\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`clk_gmac0_gtx::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`clk_gmac0_gtx::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@clk_gmac0_gtx`]
module"]
pub type CLK_GMAC0_GTX = crate::Reg<clk_gmac0_gtx::CLK_GMAC0_GTX_SPEC>;
#[doc = "Clock GMAC 0 GTX"]
pub mod clk_gmac0_gtx;
#[doc = "clk_gmac0_ptp (rw) register accessor: Clock GMAC 0 PTP\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`clk_gmac0_ptp::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`clk_gmac0_ptp::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@clk_gmac0_ptp`]
module"]
pub type CLK_GMAC0_PTP = crate::Reg<clk_gmac0_ptp::CLK_GMAC0_PTP_SPEC>;
#[doc = "Clock GMAC 0 PTP"]
pub mod clk_gmac0_ptp;
#[doc = "clk_gmac_phy (rw) register accessor: Clock GMAC PHY\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`clk_gmac_phy::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`clk_gmac_phy::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@clk_gmac_phy`]
module"]
pub type CLK_GMAC_PHY = crate::Reg<clk_gmac_phy::CLK_GMAC_PHY_SPEC>;
#[doc = "Clock GMAC PHY"]
pub mod clk_gmac_phy;
#[doc = "clk_gmac0_gtxclk (rw) register accessor: Clock GMAC 0 GTXC\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`clk_gmac0_gtxclk::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`clk_gmac0_gtxclk::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@clk_gmac0_gtxclk`]
module"]
pub type CLK_GMAC0_GTXCLK = crate::Reg<clk_gmac0_gtxclk::CLK_GMAC0_GTXCLK_SPEC>;
#[doc = "Clock GMAC 0 GTXC"]
pub mod clk_gmac0_gtxclk;
#[doc = "clk_sys_iomux_pclk (rw) register accessor: Clock SYS IOMUX PCLK\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`clk_sys_iomux_pclk::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`clk_sys_iomux_pclk::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@clk_sys_iomux_pclk`]
module"]
pub type CLK_SYS_IOMUX_PCLK = crate::Reg<clk_sys_iomux_pclk::CLK_SYS_IOMUX_PCLK_SPEC>;
#[doc = "Clock SYS IOMUX PCLK"]
pub mod clk_sys_iomux_pclk;
#[doc = "clk_mbox_apb (rw) register accessor: Clock Mailbox APB\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`clk_mbox_apb::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`clk_mbox_apb::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@clk_mbox_apb`]
module"]
pub type CLK_MBOX_APB = crate::Reg<clk_mbox_apb::CLK_MBOX_APB_SPEC>;
#[doc = "Clock Mailbox APB"]
pub mod clk_mbox_apb;
#[doc = "clk_internal_ctrl_apb (rw) register accessor: Clock Internal Controller APB\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`clk_internal_ctrl_apb::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`clk_internal_ctrl_apb::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@clk_internal_ctrl_apb`]
module"]
pub type CLK_INTERNAL_CTRL_APB = crate::Reg<clk_internal_ctrl_apb::CLK_INTERNAL_CTRL_APB_SPEC>;
#[doc = "Clock Internal Controller APB"]
pub mod clk_internal_ctrl_apb;
#[doc = "clk_u0_can_ctrl_apb (rw) register accessor: U0 Clock CAN Controller APB\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`clk_u0_can_ctrl_apb::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`clk_u0_can_ctrl_apb::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@clk_u0_can_ctrl_apb`]
module"]
pub type CLK_U0_CAN_CTRL_APB = crate::Reg<clk_u0_can_ctrl_apb::CLK_U0_CAN_CTRL_APB_SPEC>;
#[doc = "U0 Clock CAN Controller APB"]
pub mod clk_u0_can_ctrl_apb;
#[doc = "clk_u0_can_ctrl_tim (rw) register accessor: U0 Clock CAN Controller Timer\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`clk_u0_can_ctrl_tim::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`clk_u0_can_ctrl_tim::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@clk_u0_can_ctrl_tim`]
module"]
pub type CLK_U0_CAN_CTRL_TIM = crate::Reg<clk_u0_can_ctrl_tim::CLK_U0_CAN_CTRL_TIM_SPEC>;
#[doc = "U0 Clock CAN Controller Timer"]
pub mod clk_u0_can_ctrl_tim;
#[doc = "clk_u0_can_ctrl_can (rw) register accessor: U0 Clock CAN Controller CAN\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`clk_u0_can_ctrl_can::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`clk_u0_can_ctrl_can::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@clk_u0_can_ctrl_can`]
module"]
pub type CLK_U0_CAN_CTRL_CAN = crate::Reg<clk_u0_can_ctrl_can::CLK_U0_CAN_CTRL_CAN_SPEC>;
#[doc = "U0 Clock CAN Controller CAN"]
pub mod clk_u0_can_ctrl_can;
#[doc = "clk_u1_can_ctrl_apb (rw) register accessor: U1 Clock CAN Controller APB\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`clk_u1_can_ctrl_apb::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`clk_u1_can_ctrl_apb::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@clk_u1_can_ctrl_apb`]
module"]
pub type CLK_U1_CAN_CTRL_APB = crate::Reg<clk_u1_can_ctrl_apb::CLK_U1_CAN_CTRL_APB_SPEC>;
#[doc = "U1 Clock CAN Controller APB"]
pub mod clk_u1_can_ctrl_apb;
#[doc = "clk_u1_can_ctrl_tim (rw) register accessor: U1 Clock CAN Controller Timer\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`clk_u1_can_ctrl_tim::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`clk_u1_can_ctrl_tim::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@clk_u1_can_ctrl_tim`]
module"]
pub type CLK_U1_CAN_CTRL_TIM = crate::Reg<clk_u1_can_ctrl_tim::CLK_U1_CAN_CTRL_TIM_SPEC>;
#[doc = "U1 Clock CAN Controller Timer"]
pub mod clk_u1_can_ctrl_tim;
#[doc = "clk_u1_can_ctrl_can (rw) register accessor: U1 Clock CAN Controller CAN\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`clk_u1_can_ctrl_can::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`clk_u1_can_ctrl_can::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@clk_u1_can_ctrl_can`]
module"]
pub type CLK_U1_CAN_CTRL_CAN = crate::Reg<clk_u1_can_ctrl_can::CLK_U1_CAN_CTRL_CAN_SPEC>;
#[doc = "U1 Clock CAN Controller CAN"]
pub mod clk_u1_can_ctrl_can;
#[doc = "clk_pwm_apb (rw) register accessor: Clock PWM APB\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`clk_pwm_apb::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`clk_pwm_apb::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@clk_pwm_apb`]
module"]
pub type CLK_PWM_APB = crate::Reg<clk_pwm_apb::CLK_PWM_APB_SPEC>;
#[doc = "Clock PWM APB"]
pub mod clk_pwm_apb;
#[doc = "clk_wdt_apb (rw) register accessor: Clock WDT APB\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`clk_wdt_apb::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`clk_wdt_apb::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@clk_wdt_apb`]
module"]
pub type CLK_WDT_APB = crate::Reg<clk_wdt_apb::CLK_WDT_APB_SPEC>;
#[doc = "Clock WDT APB"]
pub mod clk_wdt_apb;
#[doc = "clk_wdt (rw) register accessor: Clock WDT\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`clk_wdt::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`clk_wdt::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@clk_wdt`]
module"]
pub type CLK_WDT = crate::Reg<clk_wdt::CLK_WDT_SPEC>;
#[doc = "Clock WDT"]
pub mod clk_wdt;
#[doc = "clk_tim_apb (rw) register accessor: Clock Timer APB\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`clk_tim_apb::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`clk_tim_apb::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@clk_tim_apb`]
module"]
pub type CLK_TIM_APB = crate::Reg<clk_tim_apb::CLK_TIM_APB_SPEC>;
#[doc = "Clock Timer APB"]
pub mod clk_tim_apb;
#[doc = "clk_tim0 (rw) register accessor: Clock Timer 0\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`clk_tim0::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`clk_tim0::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@clk_tim0`]
module"]
pub type CLK_TIM0 = crate::Reg<clk_tim0::CLK_TIM0_SPEC>;
#[doc = "Clock Timer 0"]
pub mod clk_tim0;
#[doc = "clk_tim1 (rw) register accessor: Clock Timer 1\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`clk_tim1::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`clk_tim1::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@clk_tim1`]
module"]
pub type CLK_TIM1 = crate::Reg<clk_tim1::CLK_TIM1_SPEC>;
#[doc = "Clock Timer 1"]
pub mod clk_tim1;
#[doc = "clk_tim2 (rw) register accessor: Clock Timer 2\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`clk_tim2::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`clk_tim2::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@clk_tim2`]
module"]
pub type CLK_TIM2 = crate::Reg<clk_tim2::CLK_TIM2_SPEC>;
#[doc = "Clock Timer 2"]
pub mod clk_tim2;
#[doc = "clk_tim3 (rw) register accessor: Clock Timer 3\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`clk_tim3::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`clk_tim3::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@clk_tim3`]
module"]
pub type CLK_TIM3 = crate::Reg<clk_tim3::CLK_TIM3_SPEC>;
#[doc = "Clock Timer 3"]
pub mod clk_tim3;
#[doc = "clk_temp_sensor_apb (rw) register accessor: Clock Temperature Sensor APB\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`clk_temp_sensor_apb::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`clk_temp_sensor_apb::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@clk_temp_sensor_apb`]
module"]
pub type CLK_TEMP_SENSOR_APB = crate::Reg<clk_temp_sensor_apb::CLK_TEMP_SENSOR_APB_SPEC>;
#[doc = "Clock Temperature Sensor APB"]
pub mod clk_temp_sensor_apb;
#[doc = "clk_temp_sensor (rw) register accessor: Clock Temperature Sensor\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`clk_temp_sensor::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`clk_temp_sensor::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@clk_temp_sensor`]
module"]
pub type CLK_TEMP_SENSOR = crate::Reg<clk_temp_sensor::CLK_TEMP_SENSOR_SPEC>;
#[doc = "Clock Temperature Sensor"]
pub mod clk_temp_sensor;
#[doc = "clk_u0_spi_apb (rw) register accessor: U0 Clock SPI APB\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`clk_u0_spi_apb::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`clk_u0_spi_apb::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@clk_u0_spi_apb`]
module"]
pub type CLK_U0_SPI_APB = crate::Reg<clk_u0_spi_apb::CLK_U0_SPI_APB_SPEC>;
#[doc = "U0 Clock SPI APB"]
pub mod clk_u0_spi_apb;
#[doc = "clk_u1_spi_apb (rw) register accessor: U1 Clock SPI APB\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`clk_u1_spi_apb::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`clk_u1_spi_apb::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@clk_u1_spi_apb`]
module"]
pub type CLK_U1_SPI_APB = crate::Reg<clk_u1_spi_apb::CLK_U1_SPI_APB_SPEC>;
#[doc = "U1 Clock SPI APB"]
pub mod clk_u1_spi_apb;
#[doc = "clk_u2_spi_apb (rw) register accessor: U2 Clock SPI APB\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`clk_u2_spi_apb::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`clk_u2_spi_apb::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@clk_u2_spi_apb`]
module"]
pub type CLK_U2_SPI_APB = crate::Reg<clk_u2_spi_apb::CLK_U2_SPI_APB_SPEC>;
#[doc = "U2 Clock SPI APB"]
pub mod clk_u2_spi_apb;
#[doc = "clk_u3_spi_apb (rw) register accessor: U3 Clock SPI APB\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`clk_u3_spi_apb::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`clk_u3_spi_apb::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@clk_u3_spi_apb`]
module"]
pub type CLK_U3_SPI_APB = crate::Reg<clk_u3_spi_apb::CLK_U3_SPI_APB_SPEC>;
#[doc = "U3 Clock SPI APB"]
pub mod clk_u3_spi_apb;
#[doc = "clk_u4_spi_apb (rw) register accessor: U4 Clock SPI APB\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`clk_u4_spi_apb::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`clk_u4_spi_apb::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@clk_u4_spi_apb`]
module"]
pub type CLK_U4_SPI_APB = crate::Reg<clk_u4_spi_apb::CLK_U4_SPI_APB_SPEC>;
#[doc = "U4 Clock SPI APB"]
pub mod clk_u4_spi_apb;
#[doc = "clk_u5_spi_apb (rw) register accessor: U5 Clock SPI APB\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`clk_u5_spi_apb::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`clk_u5_spi_apb::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@clk_u5_spi_apb`]
module"]
pub type CLK_U5_SPI_APB = crate::Reg<clk_u5_spi_apb::CLK_U5_SPI_APB_SPEC>;
#[doc = "U5 Clock SPI APB"]
pub mod clk_u5_spi_apb;
#[doc = "clk_u6_spi_apb (rw) register accessor: U6 Clock SPI APB\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`clk_u6_spi_apb::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`clk_u6_spi_apb::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@clk_u6_spi_apb`]
module"]
pub type CLK_U6_SPI_APB = crate::Reg<clk_u6_spi_apb::CLK_U6_SPI_APB_SPEC>;
#[doc = "U6 Clock SPI APB"]
pub mod clk_u6_spi_apb;
#[doc = "clk_u0_i2c_apb (rw) register accessor: U0 Clock I2C APB\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`clk_u0_i2c_apb::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`clk_u0_i2c_apb::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@clk_u0_i2c_apb`]
module"]
pub type CLK_U0_I2C_APB = crate::Reg<clk_u0_i2c_apb::CLK_U0_I2C_APB_SPEC>;
#[doc = "U0 Clock I2C APB"]
pub mod clk_u0_i2c_apb;
#[doc = "clk_u1_i2c_apb (rw) register accessor: U1 Clock I2C APB\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`clk_u1_i2c_apb::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`clk_u1_i2c_apb::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@clk_u1_i2c_apb`]
module"]
pub type CLK_U1_I2C_APB = crate::Reg<clk_u1_i2c_apb::CLK_U1_I2C_APB_SPEC>;
#[doc = "U1 Clock I2C APB"]
pub mod clk_u1_i2c_apb;
#[doc = "clk_u2_i2c_apb (rw) register accessor: U2 Clock I2C APB\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`clk_u2_i2c_apb::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`clk_u2_i2c_apb::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@clk_u2_i2c_apb`]
module"]
pub type CLK_U2_I2C_APB = crate::Reg<clk_u2_i2c_apb::CLK_U2_I2C_APB_SPEC>;
#[doc = "U2 Clock I2C APB"]
pub mod clk_u2_i2c_apb;
#[doc = "clk_u3_i2c_apb (rw) register accessor: U3 Clock I2C APB\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`clk_u3_i2c_apb::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`clk_u3_i2c_apb::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@clk_u3_i2c_apb`]
module"]
pub type CLK_U3_I2C_APB = crate::Reg<clk_u3_i2c_apb::CLK_U3_I2C_APB_SPEC>;
#[doc = "U3 Clock I2C APB"]
pub mod clk_u3_i2c_apb;
#[doc = "clk_u4_i2c_apb (rw) register accessor: U4 Clock I2C APB\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`clk_u4_i2c_apb::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`clk_u4_i2c_apb::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@clk_u4_i2c_apb`]
module"]
pub type CLK_U4_I2C_APB = crate::Reg<clk_u4_i2c_apb::CLK_U4_I2C_APB_SPEC>;
#[doc = "U4 Clock I2C APB"]
pub mod clk_u4_i2c_apb;
#[doc = "clk_u5_i2c_apb (rw) register accessor: U5 Clock I2C APB\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`clk_u5_i2c_apb::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`clk_u5_i2c_apb::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@clk_u5_i2c_apb`]
module"]
pub type CLK_U5_I2C_APB = crate::Reg<clk_u5_i2c_apb::CLK_U5_I2C_APB_SPEC>;
#[doc = "U5 Clock I2C APB"]
pub mod clk_u5_i2c_apb;
#[doc = "clk_u6_i2c_apb (rw) register accessor: U6 Clock I2C APB\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`clk_u6_i2c_apb::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`clk_u6_i2c_apb::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@clk_u6_i2c_apb`]
module"]
pub type CLK_U6_I2C_APB = crate::Reg<clk_u6_i2c_apb::CLK_U6_I2C_APB_SPEC>;
#[doc = "U6 Clock I2C APB"]
pub mod clk_u6_i2c_apb;
#[doc = "clk_u0_uart_apb (rw) register accessor: U0 Clock UART APB\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`clk_u0_uart_apb::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`clk_u0_uart_apb::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@clk_u0_uart_apb`]
module"]
pub type CLK_U0_UART_APB = crate::Reg<clk_u0_uart_apb::CLK_U0_UART_APB_SPEC>;
#[doc = "U0 Clock UART APB"]
pub mod clk_u0_uart_apb;
#[doc = "clk_u0_uart_core (rw) register accessor: U0 Clock UART Core\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`clk_u0_uart_core::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`clk_u0_uart_core::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@clk_u0_uart_core`]
module"]
pub type CLK_U0_UART_CORE = crate::Reg<clk_u0_uart_core::CLK_U0_UART_CORE_SPEC>;
#[doc = "U0 Clock UART Core"]
pub mod clk_u0_uart_core;
#[doc = "clk_u1_uart_apb (rw) register accessor: U1 Clock UART APB\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`clk_u1_uart_apb::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`clk_u1_uart_apb::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@clk_u1_uart_apb`]
module"]
pub type CLK_U1_UART_APB = crate::Reg<clk_u1_uart_apb::CLK_U1_UART_APB_SPEC>;
#[doc = "U1 Clock UART APB"]
pub mod clk_u1_uart_apb;
#[doc = "clk_u1_uart_core (rw) register accessor: U1 Clock UART Core\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`clk_u1_uart_core::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`clk_u1_uart_core::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@clk_u1_uart_core`]
module"]
pub type CLK_U1_UART_CORE = crate::Reg<clk_u1_uart_core::CLK_U1_UART_CORE_SPEC>;
#[doc = "U1 Clock UART Core"]
pub mod clk_u1_uart_core;
#[doc = "clk_u2_uart_apb (rw) register accessor: U2 Clock UART APB\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`clk_u2_uart_apb::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`clk_u2_uart_apb::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@clk_u2_uart_apb`]
module"]
pub type CLK_U2_UART_APB = crate::Reg<clk_u2_uart_apb::CLK_U2_UART_APB_SPEC>;
#[doc = "U2 Clock UART APB"]
pub mod clk_u2_uart_apb;
#[doc = "clk_u2_uart_core (rw) register accessor: U2 Clock UART Core\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`clk_u2_uart_core::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`clk_u2_uart_core::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@clk_u2_uart_core`]
module"]
pub type CLK_U2_UART_CORE = crate::Reg<clk_u2_uart_core::CLK_U2_UART_CORE_SPEC>;
#[doc = "U2 Clock UART Core"]
pub mod clk_u2_uart_core;
#[doc = "clk_u3_uart_apb (rw) register accessor: U3 Clock UART APB\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`clk_u3_uart_apb::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`clk_u3_uart_apb::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@clk_u3_uart_apb`]
module"]
pub type CLK_U3_UART_APB = crate::Reg<clk_u3_uart_apb::CLK_U3_UART_APB_SPEC>;
#[doc = "U3 Clock UART APB"]
pub mod clk_u3_uart_apb;
#[doc = "clk_u3_uart_core (rw) register accessor: U3 Clock UART Core\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`clk_u3_uart_core::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`clk_u3_uart_core::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@clk_u3_uart_core`]
module"]
pub type CLK_U3_UART_CORE = crate::Reg<clk_u3_uart_core::CLK_U3_UART_CORE_SPEC>;
#[doc = "U3 Clock UART Core"]
pub mod clk_u3_uart_core;
#[doc = "clk_u4_uart_apb (rw) register accessor: U4 Clock UART APB\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`clk_u4_uart_apb::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`clk_u4_uart_apb::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@clk_u4_uart_apb`]
module"]
pub type CLK_U4_UART_APB = crate::Reg<clk_u4_uart_apb::CLK_U4_UART_APB_SPEC>;
#[doc = "U4 Clock UART APB"]
pub mod clk_u4_uart_apb;
#[doc = "clk_u4_uart_core (rw) register accessor: U4 Clock UART Core\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`clk_u4_uart_core::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`clk_u4_uart_core::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@clk_u4_uart_core`]
module"]
pub type CLK_U4_UART_CORE = crate::Reg<clk_u4_uart_core::CLK_U4_UART_CORE_SPEC>;
#[doc = "U4 Clock UART Core"]
pub mod clk_u4_uart_core;
#[doc = "clk_u5_uart_apb (rw) register accessor: U5 Clock UART APB\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`clk_u5_uart_apb::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`clk_u5_uart_apb::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@clk_u5_uart_apb`]
module"]
pub type CLK_U5_UART_APB = crate::Reg<clk_u5_uart_apb::CLK_U5_UART_APB_SPEC>;
#[doc = "U5 Clock UART APB"]
pub mod clk_u5_uart_apb;
#[doc = "clk_u5_uart_core (rw) register accessor: U5 Clock UART Core\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`clk_u5_uart_core::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`clk_u5_uart_core::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@clk_u5_uart_core`]
module"]
pub type CLK_U5_UART_CORE = crate::Reg<clk_u5_uart_core::CLK_U5_UART_CORE_SPEC>;
#[doc = "U5 Clock UART Core"]
pub mod clk_u5_uart_core;
#[doc = "clk_pwmdac_apb (rw) register accessor: Clock PWMDAC APB\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`clk_pwmdac_apb::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`clk_pwmdac_apb::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@clk_pwmdac_apb`]
module"]
pub type CLK_PWMDAC_APB = crate::Reg<clk_pwmdac_apb::CLK_PWMDAC_APB_SPEC>;
#[doc = "Clock PWMDAC APB"]
pub mod clk_pwmdac_apb;
#[doc = "clk_pwmdac_core (rw) register accessor: Clock PWMDAC Core\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`clk_pwmdac_core::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`clk_pwmdac_core::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@clk_pwmdac_core`]
module"]
pub type CLK_PWMDAC_CORE = crate::Reg<clk_pwmdac_core::CLK_PWMDAC_CORE_SPEC>;
#[doc = "Clock PWMDAC Core"]
pub mod clk_pwmdac_core;
#[doc = "clk_spdif_apb (rw) register accessor: Clock SPDIF APB\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`clk_spdif_apb::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`clk_spdif_apb::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@clk_spdif_apb`]
module"]
pub type CLK_SPDIF_APB = crate::Reg<clk_spdif_apb::CLK_SPDIF_APB_SPEC>;
#[doc = "Clock SPDIF APB"]
pub mod clk_spdif_apb;
#[doc = "clk_spdif_core (rw) register accessor: Clock SPDIF Core\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`clk_spdif_core::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`clk_spdif_core::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@clk_spdif_core`]
module"]
pub type CLK_SPDIF_CORE = crate::Reg<clk_spdif_core::CLK_SPDIF_CORE_SPEC>;
#[doc = "Clock SPDIF Core"]
pub mod clk_spdif_core;
#[doc = "clk_u0_i2s_tx_apb (rw) register accessor: U0 Clock I2S TX APB\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`clk_u0_i2s_tx_apb::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`clk_u0_i2s_tx_apb::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@clk_u0_i2s_tx_apb`]
module"]
pub type CLK_U0_I2S_TX_APB = crate::Reg<clk_u0_i2s_tx_apb::CLK_U0_I2S_TX_APB_SPEC>;
#[doc = "U0 Clock I2S TX APB"]
pub mod clk_u0_i2s_tx_apb;
#[doc = "clk_u0_i2stx_4ch0_bclk_mst (rw) register accessor: U0 Clock I2S TX 0 BCLK MST\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`clk_u0_i2stx_4ch0_bclk_mst::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`clk_u0_i2stx_4ch0_bclk_mst::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@clk_u0_i2stx_4ch0_bclk_mst`]
module"]
pub type CLK_U0_I2STX_4CH0_BCLK_MST =
    crate::Reg<clk_u0_i2stx_4ch0_bclk_mst::CLK_U0_I2STX_4CH0_BCLK_MST_SPEC>;
#[doc = "U0 Clock I2S TX 0 BCLK MST"]
pub mod clk_u0_i2stx_4ch0_bclk_mst;
#[doc = "clk_u0_i2stx_4ch0_bclk_mst_inv (rw) register accessor: U0 Clock I2S TX 0 BCLK MST Inverter\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`clk_u0_i2stx_4ch0_bclk_mst_inv::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`clk_u0_i2stx_4ch0_bclk_mst_inv::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@clk_u0_i2stx_4ch0_bclk_mst_inv`]
module"]
pub type CLK_U0_I2STX_4CH0_BCLK_MST_INV =
    crate::Reg<clk_u0_i2stx_4ch0_bclk_mst_inv::CLK_U0_I2STX_4CH0_BCLK_MST_INV_SPEC>;
#[doc = "U0 Clock I2S TX 0 BCLK MST Inverter"]
pub mod clk_u0_i2stx_4ch0_bclk_mst_inv;
#[doc = "clk_i2stx0_lrck_mst (rw) register accessor: Clock I2S TX 0 LRCK MST\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`clk_i2stx0_lrck_mst::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`clk_i2stx0_lrck_mst::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@clk_i2stx0_lrck_mst`]
module"]
pub type CLK_I2STX0_LRCK_MST = crate::Reg<clk_i2stx0_lrck_mst::CLK_I2STX0_LRCK_MST_SPEC>;
#[doc = "Clock I2S TX 0 LRCK MST"]
pub mod clk_i2stx0_lrck_mst;
#[doc = "clk_u0_i2stx_bclk (rw) register accessor: U0 Clock I2S TX BCLK\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`clk_u0_i2stx_bclk::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`clk_u0_i2stx_bclk::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@clk_u0_i2stx_bclk`]
module"]
pub type CLK_U0_I2STX_BCLK = crate::Reg<clk_u0_i2stx_bclk::CLK_U0_I2STX_BCLK_SPEC>;
#[doc = "U0 Clock I2S TX BCLK"]
pub mod clk_u0_i2stx_bclk;
#[doc = "clk_u0_i2stx_bclk_neg (rw) register accessor: U0 Clock I2S TX BCLK Negative\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`clk_u0_i2stx_bclk_neg::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`clk_u0_i2stx_bclk_neg::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@clk_u0_i2stx_bclk_neg`]
module"]
pub type CLK_U0_I2STX_BCLK_NEG = crate::Reg<clk_u0_i2stx_bclk_neg::CLK_U0_I2STX_BCLK_NEG_SPEC>;
#[doc = "U0 Clock I2S TX BCLK Negative"]
pub mod clk_u0_i2stx_bclk_neg;
#[doc = "clk_u0_i2stx_lrck (rw) register accessor: U0 Clock I2S TX LRCK\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`clk_u0_i2stx_lrck::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`clk_u0_i2stx_lrck::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@clk_u0_i2stx_lrck`]
module"]
pub type CLK_U0_I2STX_LRCK = crate::Reg<clk_u0_i2stx_lrck::CLK_U0_I2STX_LRCK_SPEC>;
#[doc = "U0 Clock I2S TX LRCK"]
pub mod clk_u0_i2stx_lrck;
#[doc = "clk_u1_i2s_tx_apb (rw) register accessor: U1 Clock I2S TX APB\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`clk_u1_i2s_tx_apb::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`clk_u1_i2s_tx_apb::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@clk_u1_i2s_tx_apb`]
module"]
pub type CLK_U1_I2S_TX_APB = crate::Reg<clk_u1_i2s_tx_apb::CLK_U1_I2S_TX_APB_SPEC>;
#[doc = "U1 Clock I2S TX APB"]
pub mod clk_u1_i2s_tx_apb;
#[doc = "clk_u1_i2stx_4ch1_bclk_mst (rw) register accessor: U1 Clock I2S TX 1 BCLK MST\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`clk_u1_i2stx_4ch1_bclk_mst::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`clk_u1_i2stx_4ch1_bclk_mst::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@clk_u1_i2stx_4ch1_bclk_mst`]
module"]
pub type CLK_U1_I2STX_4CH1_BCLK_MST =
    crate::Reg<clk_u1_i2stx_4ch1_bclk_mst::CLK_U1_I2STX_4CH1_BCLK_MST_SPEC>;
#[doc = "U1 Clock I2S TX 1 BCLK MST"]
pub mod clk_u1_i2stx_4ch1_bclk_mst;
#[doc = "clk_u1_i2stx_4ch1_bclk_mst_inv (rw) register accessor: U1 Clock I2S TX 1 BCLK MST Inverter\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`clk_u1_i2stx_4ch1_bclk_mst_inv::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`clk_u1_i2stx_4ch1_bclk_mst_inv::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@clk_u1_i2stx_4ch1_bclk_mst_inv`]
module"]
pub type CLK_U1_I2STX_4CH1_BCLK_MST_INV =
    crate::Reg<clk_u1_i2stx_4ch1_bclk_mst_inv::CLK_U1_I2STX_4CH1_BCLK_MST_INV_SPEC>;
#[doc = "U1 Clock I2S TX 1 BCLK MST Inverter"]
pub mod clk_u1_i2stx_4ch1_bclk_mst_inv;
#[doc = "clk_i2stx1_lrck_mst (rw) register accessor: Clock I2S TX 1 LRCK MST\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`clk_i2stx1_lrck_mst::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`clk_i2stx1_lrck_mst::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@clk_i2stx1_lrck_mst`]
module"]
pub type CLK_I2STX1_LRCK_MST = crate::Reg<clk_i2stx1_lrck_mst::CLK_I2STX1_LRCK_MST_SPEC>;
#[doc = "Clock I2S TX 1 LRCK MST"]
pub mod clk_i2stx1_lrck_mst;
#[doc = "clk_u1_i2stx_bclk (rw) register accessor: U1 Clock I2S TX BCLK\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`clk_u1_i2stx_bclk::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`clk_u1_i2stx_bclk::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@clk_u1_i2stx_bclk`]
module"]
pub type CLK_U1_I2STX_BCLK = crate::Reg<clk_u1_i2stx_bclk::CLK_U1_I2STX_BCLK_SPEC>;
#[doc = "U1 Clock I2S TX BCLK"]
pub mod clk_u1_i2stx_bclk;
#[doc = "clk_u1_i2stx_bclk_neg (rw) register accessor: U1 Clock I2S TX BCLK Negative\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`clk_u1_i2stx_bclk_neg::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`clk_u1_i2stx_bclk_neg::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@clk_u1_i2stx_bclk_neg`]
module"]
pub type CLK_U1_I2STX_BCLK_NEG = crate::Reg<clk_u1_i2stx_bclk_neg::CLK_U1_I2STX_BCLK_NEG_SPEC>;
#[doc = "U1 Clock I2S TX BCLK Negative"]
pub mod clk_u1_i2stx_bclk_neg;
#[doc = "clk_u1_i2stx_lrck (rw) register accessor: U1 Clock I2S TX LRCK\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`clk_u1_i2stx_lrck::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`clk_u1_i2stx_lrck::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@clk_u1_i2stx_lrck`]
module"]
pub type CLK_U1_I2STX_LRCK = crate::Reg<clk_u1_i2stx_lrck::CLK_U1_I2STX_LRCK_SPEC>;
#[doc = "U1 Clock I2S TX LRCK"]
pub mod clk_u1_i2stx_lrck;
#[doc = "clk_i2s_apb (rw) register accessor: Clock I2S APB\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`clk_i2s_apb::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`clk_i2s_apb::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@clk_i2s_apb`]
module"]
pub type CLK_I2S_APB = crate::Reg<clk_i2s_apb::CLK_I2S_APB_SPEC>;
#[doc = "Clock I2S APB"]
pub mod clk_i2s_apb;
#[doc = "clk_i2s_bclk_mst (rw) register accessor: Clock I2S BCLK MST\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`clk_i2s_bclk_mst::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`clk_i2s_bclk_mst::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@clk_i2s_bclk_mst`]
module"]
pub type CLK_I2S_BCLK_MST = crate::Reg<clk_i2s_bclk_mst::CLK_I2S_BCLK_MST_SPEC>;
#[doc = "Clock I2S BCLK MST"]
pub mod clk_i2s_bclk_mst;
#[doc = "clk_i2s_bclk_mst_inv (rw) register accessor: Clock I2S BCLK MST Inverter\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`clk_i2s_bclk_mst_inv::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`clk_i2s_bclk_mst_inv::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@clk_i2s_bclk_mst_inv`]
module"]
pub type CLK_I2S_BCLK_MST_INV = crate::Reg<clk_i2s_bclk_mst_inv::CLK_I2S_BCLK_MST_INV_SPEC>;
#[doc = "Clock I2S BCLK MST Inverter"]
pub mod clk_i2s_bclk_mst_inv;
#[doc = "clk_i2s_lrck_mst (rw) register accessor: Clock I2S LRCK MST\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`clk_i2s_lrck_mst::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`clk_i2s_lrck_mst::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@clk_i2s_lrck_mst`]
module"]
pub type CLK_I2S_LRCK_MST = crate::Reg<clk_i2s_lrck_mst::CLK_I2S_LRCK_MST_SPEC>;
#[doc = "Clock I2S LRCK MST"]
pub mod clk_i2s_lrck_mst;
#[doc = "clk_i2s_bclk (rw) register accessor: Clock I2S BCLK\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`clk_i2s_bclk::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`clk_i2s_bclk::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@clk_i2s_bclk`]
module"]
pub type CLK_I2S_BCLK = crate::Reg<clk_i2s_bclk::CLK_I2S_BCLK_SPEC>;
#[doc = "Clock I2S BCLK"]
pub mod clk_i2s_bclk;
#[doc = "clk_i2s_bclk_neg (rw) register accessor: Clock I2S BCLK Negative\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`clk_i2s_bclk_neg::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`clk_i2s_bclk_neg::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@clk_i2s_bclk_neg`]
module"]
pub type CLK_I2S_BCLK_NEG = crate::Reg<clk_i2s_bclk_neg::CLK_I2S_BCLK_NEG_SPEC>;
#[doc = "Clock I2S BCLK Negative"]
pub mod clk_i2s_bclk_neg;
#[doc = "clk_i2s_lrck (rw) register accessor: Clock I2S LRCK\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`clk_i2s_lrck::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`clk_i2s_lrck::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@clk_i2s_lrck`]
module"]
pub type CLK_I2S_LRCK = crate::Reg<clk_i2s_lrck::CLK_I2S_LRCK_SPEC>;
#[doc = "Clock I2S LRCK"]
pub mod clk_i2s_lrck;
#[doc = "clk_pdm_dmic (rw) register accessor: Clock PDM DMIC\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`clk_pdm_dmic::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`clk_pdm_dmic::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@clk_pdm_dmic`]
module"]
pub type CLK_PDM_DMIC = crate::Reg<clk_pdm_dmic::CLK_PDM_DMIC_SPEC>;
#[doc = "Clock PDM DMIC"]
pub mod clk_pdm_dmic;
#[doc = "clk_pdm_apb (rw) register accessor: Clock PDM APB\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`clk_pdm_apb::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`clk_pdm_apb::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@clk_pdm_apb`]
module"]
pub type CLK_PDM_APB = crate::Reg<clk_pdm_apb::CLK_PDM_APB_SPEC>;
#[doc = "Clock PDM APB"]
pub mod clk_pdm_apb;
#[doc = "clk_tdm_ahb (rw) register accessor: Clock TDM AHB\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`clk_tdm_ahb::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`clk_tdm_ahb::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@clk_tdm_ahb`]
module"]
pub type CLK_TDM_AHB = crate::Reg<clk_tdm_ahb::CLK_TDM_AHB_SPEC>;
#[doc = "Clock TDM AHB"]
pub mod clk_tdm_ahb;
#[doc = "clk_tdm_apb (rw) register accessor: Clock TDM APB\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`clk_tdm_apb::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`clk_tdm_apb::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@clk_tdm_apb`]
module"]
pub type CLK_TDM_APB = crate::Reg<clk_tdm_apb::CLK_TDM_APB_SPEC>;
#[doc = "Clock TDM APB"]
pub mod clk_tdm_apb;
#[doc = "clk_tdm_internal (rw) register accessor: Clock TDM Internal\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`clk_tdm_internal::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`clk_tdm_internal::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@clk_tdm_internal`]
module"]
pub type CLK_TDM_INTERNAL = crate::Reg<clk_tdm_internal::CLK_TDM_INTERNAL_SPEC>;
#[doc = "Clock TDM Internal"]
pub mod clk_tdm_internal;
#[doc = "clk_tdm (rw) register accessor: Clock TDM\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`clk_tdm::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`clk_tdm::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@clk_tdm`]
module"]
pub type CLK_TDM = crate::Reg<clk_tdm::CLK_TDM_SPEC>;
#[doc = "Clock TDM"]
pub mod clk_tdm;
#[doc = "clk_tdm_neg (rw) register accessor: Clock TDM Negative\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`clk_tdm_neg::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`clk_tdm_neg::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@clk_tdm_neg`]
module"]
pub type CLK_TDM_NEG = crate::Reg<clk_tdm_neg::CLK_TDM_NEG_SPEC>;
#[doc = "Clock TDM Negative"]
pub mod clk_tdm_neg;
#[doc = "clk_jtag_cert_trng (rw) register accessor: Clock JTAG Certification TRNG\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`clk_jtag_cert_trng::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`clk_jtag_cert_trng::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@clk_jtag_cert_trng`]
module"]
pub type CLK_JTAG_CERT_TRNG = crate::Reg<clk_jtag_cert_trng::CLK_JTAG_CERT_TRNG_SPEC>;
#[doc = "Clock JTAG Certification TRNG"]
pub mod clk_jtag_cert_trng;
#[doc = "soft_rst0_addr_sel (rw) register accessor: Software RESET 0 Address Selector\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`soft_rst0_addr_sel::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`soft_rst0_addr_sel::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@soft_rst0_addr_sel`]
module"]
pub type SOFT_RST0_ADDR_SEL = crate::Reg<soft_rst0_addr_sel::SOFT_RST0_ADDR_SEL_SPEC>;
#[doc = "Software RESET 0 Address Selector"]
pub mod soft_rst0_addr_sel;
#[doc = "soft_rst1_addr_sel (rw) register accessor: Software RESET 1 Address Selector\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`soft_rst1_addr_sel::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`soft_rst1_addr_sel::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@soft_rst1_addr_sel`]
module"]
pub type SOFT_RST1_ADDR_SEL = crate::Reg<soft_rst1_addr_sel::SOFT_RST1_ADDR_SEL_SPEC>;
#[doc = "Software RESET 1 Address Selector"]
pub mod soft_rst1_addr_sel;
#[doc = "soft_rst2_addr_sel (rw) register accessor: Software RESET 2 Address Selector\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`soft_rst2_addr_sel::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`soft_rst2_addr_sel::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@soft_rst2_addr_sel`]
module"]
pub type SOFT_RST2_ADDR_SEL = crate::Reg<soft_rst2_addr_sel::SOFT_RST2_ADDR_SEL_SPEC>;
#[doc = "Software RESET 2 Address Selector"]
pub mod soft_rst2_addr_sel;
#[doc = "soft_rst3_addr_sel (rw) register accessor: Software RESET 3 Address Selector\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`soft_rst3_addr_sel::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`soft_rst3_addr_sel::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@soft_rst3_addr_sel`]
module"]
pub type SOFT_RST3_ADDR_SEL = crate::Reg<soft_rst3_addr_sel::SOFT_RST3_ADDR_SEL_SPEC>;
#[doc = "Software RESET 3 Address Selector"]
pub mod soft_rst3_addr_sel;
#[doc = "syscrg_rst0_status (rw) register accessor: SYSCRG RESET Status 0\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`syscrg_rst0_status::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`syscrg_rst0_status::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@syscrg_rst0_status`]
module"]
pub type SYSCRG_RST0_STATUS = crate::Reg<syscrg_rst0_status::SYSCRG_RST0_STATUS_SPEC>;
#[doc = "SYSCRG RESET Status 0"]
pub mod syscrg_rst0_status;
#[doc = "syscrg_rst1_status (rw) register accessor: SYSCRG RESET Status 1\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`syscrg_rst1_status::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`syscrg_rst1_status::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@syscrg_rst1_status`]
module"]
pub type SYSCRG_RST1_STATUS = crate::Reg<syscrg_rst1_status::SYSCRG_RST1_STATUS_SPEC>;
#[doc = "SYSCRG RESET Status 1"]
pub mod syscrg_rst1_status;
#[doc = "syscrg_rst2_status (rw) register accessor: SYSCRG RESET Status 2\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`syscrg_rst2_status::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`syscrg_rst2_status::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@syscrg_rst2_status`]
module"]
pub type SYSCRG_RST2_STATUS = crate::Reg<syscrg_rst2_status::SYSCRG_RST2_STATUS_SPEC>;
#[doc = "SYSCRG RESET Status 2"]
pub mod syscrg_rst2_status;
#[doc = "syscrg_rst3_status (rw) register accessor: SYSCRG RESET Status 3\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`syscrg_rst3_status::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`syscrg_rst3_status::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@syscrg_rst3_status`]
module"]
pub type SYSCRG_RST3_STATUS = crate::Reg<syscrg_rst3_status::SYSCRG_RST3_STATUS_SPEC>;
#[doc = "SYSCRG RESET Status 3"]
pub mod syscrg_rst3_status;
