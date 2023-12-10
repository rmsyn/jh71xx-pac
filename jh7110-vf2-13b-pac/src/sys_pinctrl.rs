#[doc = r"Register block"]
#[repr(C)]
pub struct RegisterBlock {
    gpo_doen_0: GPO_DOEN_0,
    gpo_doen_1: GPO_DOEN_1,
    gpo_doen_2: GPO_DOEN_2,
    gpo_doen_3: GPO_DOEN_3,
    gpo_doen_4: GPO_DOEN_4,
    gpo_doen_5: GPO_DOEN_5,
    gpo_doen_6: GPO_DOEN_6,
    gpo_doen_7: GPO_DOEN_7,
    gpo_doen_8: GPO_DOEN_8,
    gpo_doen_9: GPO_DOEN_9,
    gpo_doen_10: GPO_DOEN_10,
    gpo_doen_11: GPO_DOEN_11,
    gpo_doen_12: GPO_DOEN_12,
    gpo_doen_13: GPO_DOEN_13,
    gpo_doen_14: GPO_DOEN_14,
    gpo_doen_15: GPO_DOEN_15,
    gpo_dout_0_3: GPO_DOUT_0_3,
    gpo_dout_4_7: GPO_DOUT_4_7,
    gpo_dout_8_11: GPO_DOUT_8_11,
    gpo_dout_12_15: GPO_DOUT_12_15,
    gpo_dout_16_19: GPO_DOUT_16_19,
    gpo_dout_20_23: GPO_DOUT_20_23,
    gpo_dout_24_27: GPO_DOUT_24_27,
    gpo_dout_28_31: GPO_DOUT_28_31,
    gpo_dout_32_35: GPO_DOUT_32_35,
    gpo_dout_36_39: GPO_DOUT_36_39,
    gpo_dout_40_43: GPO_DOUT_40_43,
    gpo_dout_44_47: GPO_DOUT_44_47,
    gpo_dout_48_51: GPO_DOUT_48_51,
    gpo_dout_52_55: GPO_DOUT_52_55,
    gpo_dout_56_59: GPO_DOUT_56_59,
    gpo_dout_60_63: GPO_DOUT_60_63,
    gpi_0: GPI_0,
    gpi_1: GPI_1,
    gpi_2: GPI_2,
    gpi_3: GPI_3,
    gpi_4: GPI_4,
    gpi_5: GPI_5,
    gpi_6: GPI_6,
    gpi_7: GPI_7,
    gpi_8: GPI_8,
    gpi_9: GPI_9,
    gpi_10: GPI_10,
    gpi_11: GPI_11,
    gpi_12: GPI_12,
    gpi_13: GPI_13,
    gpi_14: GPI_14,
    gpi_15: GPI_15,
    gpi_16: GPI_16,
    gpi_17: GPI_17,
    gpi_18: GPI_18,
    gpi_19: GPI_19,
    gpi_20: GPI_20,
    gpi_21: GPI_21,
    gpi_22: GPI_22,
    ioirq_0: IOIRQ_0,
    ioirq_1: IOIRQ_1,
    ioirq_2: IOIRQ_2,
    ioirq_3: IOIRQ_3,
    ioirq_4: IOIRQ_4,
    ioirq_5: IOIRQ_5,
    ioirq_6: IOIRQ_6,
    ioirq_7: IOIRQ_7,
    ioirq_8: IOIRQ_8,
    ioirq_9: IOIRQ_9,
    ioirq_10: IOIRQ_10,
    ioirq_11: IOIRQ_11,
    ioirq_12: IOIRQ_12,
    ioirq_13: IOIRQ_13,
    ioirq_14: IOIRQ_14,
    ioirq_15: IOIRQ_15,
    ioirq_16: IOIRQ_16,
    gpio_0: GPIO_0,
    gpio_1: GPIO_1,
    gpio_2: GPIO_2,
    gpio_3: GPIO_3,
    gpio_4: GPIO_4,
    gpio_5: GPIO_5,
    gpio_6: GPIO_6,
    gpio_7: GPIO_7,
    gpio_8: GPIO_8,
    gpio_9: GPIO_9,
    gpio_10: GPIO_10,
    gpio_11: GPIO_11,
    gpio_12: GPIO_12,
    gpio_13: GPIO_13,
    gpio_14: GPIO_14,
    gpio_15: GPIO_15,
    gpio_16: GPIO_16,
    gpio_17: GPIO_17,
    gpio_18: GPIO_18,
    gpio_19: GPIO_19,
    gpio_20: GPIO_20,
    gpio_21: GPIO_21,
    gpio_22: GPIO_22,
    gpio_23: GPIO_23,
    gpio_24: GPIO_24,
    gpio_25: GPIO_25,
    gpio_26: GPIO_26,
    gpio_27: GPIO_27,
    gpio_28: GPIO_28,
    gpio_29: GPIO_29,
    gpio_30: GPIO_30,
    gpio_31: GPIO_31,
    gpio_32: GPIO_32,
    gpio_33: GPIO_33,
    gpio_34: GPIO_34,
    gpio_35: GPIO_35,
    gpio_36: GPIO_36,
    gpio_37: GPIO_37,
    gpio_38: GPIO_38,
    gpio_39: GPIO_39,
    gpio_40: GPIO_40,
    gpio_41: GPIO_41,
    gpio_42: GPIO_42,
    gpio_43: GPIO_43,
    gpio_44: GPIO_44,
    gpio_45: GPIO_45,
    gpio_46: GPIO_46,
    gpio_47: GPIO_47,
    gpio_48: GPIO_48,
    gpio_49: GPIO_49,
    gpio_50: GPIO_50,
    gpio_51: GPIO_51,
    gpio_52: GPIO_52,
    gpio_53: GPIO_53,
    gpio_54: GPIO_54,
    gpio_55: GPIO_55,
    gpio_56: GPIO_56,
    gpio_57: GPIO_57,
    gpio_58: GPIO_58,
    gpio_59: GPIO_59,
    gpio_60: GPIO_60,
    gpio_61: GPIO_61,
    gpio_62: GPIO_62,
    gpio_63: GPIO_63,
    sd0_clk: SD0_CLK,
    sd0_cmd: SD0_CMD,
    sd0_data_0: SD0_DATA_0,
    sd0_data_1: SD0_DATA_1,
    sd0_data_2: SD0_DATA_2,
    sd0_data_3: SD0_DATA_3,
    sd0_data_4: SD0_DATA_4,
    sd0_data_5: SD0_DATA_5,
    sd0_data_6: SD0_DATA_6,
    sd0_data_7: SD0_DATA_7,
    sd0_strb: SD0_STRB,
    gmac1_mdc: GMAC1_MDC,
    gmac1_mdio: GMAC1_MDIO,
    gmac1_rxd_0: GMAC1_RXD_0,
    gmac1_rxd_1: GMAC1_RXD_1,
    gmac1_rxd_2: GMAC1_RXD_2,
    gmac1_rxd_3: GMAC1_RXD_3,
    gmac1_rxdv: GMAC1_RXDV,
    gmac1_rxc: GMAC1_RXC,
    gmac1_txd_0: GMAC1_TXD_0,
    gmac1_txd_1: GMAC1_TXD_1,
    gmac1_txd_2: GMAC1_TXD_2,
    gmac1_txd_3: GMAC1_TXD_3,
    gmac1_txen: GMAC1_TXEN,
    gmac1_txc: GMAC1_TXC,
    qspi_sclk: QSPI_SCLK,
    qspi_csn_0: QSPI_CSN_0,
    qspi_data_0: QSPI_DATA_0,
    qspi_data_1: QSPI_DATA_1,
    qspi_data_2: QSPI_DATA_2,
    qspi_data_3: QSPI_DATA_3,
    func_sel_0: FUNC_SEL_0,
    func_sel_1: FUNC_SEL_1,
    func_sel_2: FUNC_SEL_2,
    func_sel_3: FUNC_SEL_3,
    func_sel_4: FUNC_SEL_4,
    func_sel_5: FUNC_SEL_5,
    func_sel_6: FUNC_SEL_6,
}
impl RegisterBlock {
    #[doc = "0x00 - SYS IOMUX CFG SAIF SYSCFG FMUX 0 DOEN"]
    #[inline(always)]
    pub const fn gpo_doen_0(&self) -> &GPO_DOEN_0 {
        &self.gpo_doen_0
    }
    #[doc = "0x04 - SYS IOMUX CFG SAIF SYSCFG FMUX 1 DOEN"]
    #[inline(always)]
    pub const fn gpo_doen_1(&self) -> &GPO_DOEN_1 {
        &self.gpo_doen_1
    }
    #[doc = "0x08 - SYS IOMUX CFG SAIF SYSCFG FMUX 2 DOEN"]
    #[inline(always)]
    pub const fn gpo_doen_2(&self) -> &GPO_DOEN_2 {
        &self.gpo_doen_2
    }
    #[doc = "0x0c - SYS IOMUX CFG SAIF SYSCFG FMUX 3 DOEN"]
    #[inline(always)]
    pub const fn gpo_doen_3(&self) -> &GPO_DOEN_3 {
        &self.gpo_doen_3
    }
    #[doc = "0x10 - SYS IOMUX CFG SAIF SYSCFG FMUX 4 DOEN"]
    #[inline(always)]
    pub const fn gpo_doen_4(&self) -> &GPO_DOEN_4 {
        &self.gpo_doen_4
    }
    #[doc = "0x14 - SYS IOMUX CFG SAIF SYSCFG FMUX 5 DOEN"]
    #[inline(always)]
    pub const fn gpo_doen_5(&self) -> &GPO_DOEN_5 {
        &self.gpo_doen_5
    }
    #[doc = "0x18 - SYS IOMUX CFG SAIF SYSCFG FMUX 6 DOEN"]
    #[inline(always)]
    pub const fn gpo_doen_6(&self) -> &GPO_DOEN_6 {
        &self.gpo_doen_6
    }
    #[doc = "0x1c - SYS IOMUX CFG SAIF SYSCFG FMUX 7 DOEN"]
    #[inline(always)]
    pub const fn gpo_doen_7(&self) -> &GPO_DOEN_7 {
        &self.gpo_doen_7
    }
    #[doc = "0x20 - SYS IOMUX CFG SAIF SYSCFG FMUX 8 DOEN"]
    #[inline(always)]
    pub const fn gpo_doen_8(&self) -> &GPO_DOEN_8 {
        &self.gpo_doen_8
    }
    #[doc = "0x24 - SYS IOMUX CFG SAIF SYSCFG FMUX 9 DOEN"]
    #[inline(always)]
    pub const fn gpo_doen_9(&self) -> &GPO_DOEN_9 {
        &self.gpo_doen_9
    }
    #[doc = "0x28 - SYS IOMUX CFG SAIF SYSCFG FMUX 10 DOEN"]
    #[inline(always)]
    pub const fn gpo_doen_10(&self) -> &GPO_DOEN_10 {
        &self.gpo_doen_10
    }
    #[doc = "0x2c - SYS IOMUX CFG SAIF SYSCFG FMUX 11 DOEN"]
    #[inline(always)]
    pub const fn gpo_doen_11(&self) -> &GPO_DOEN_11 {
        &self.gpo_doen_11
    }
    #[doc = "0x30 - SYS IOMUX CFG SAIF SYSCFG FMUX 12 DOEN"]
    #[inline(always)]
    pub const fn gpo_doen_12(&self) -> &GPO_DOEN_12 {
        &self.gpo_doen_12
    }
    #[doc = "0x34 - SYS IOMUX CFG SAIF SYSCFG FMUX 13 DOEN"]
    #[inline(always)]
    pub const fn gpo_doen_13(&self) -> &GPO_DOEN_13 {
        &self.gpo_doen_13
    }
    #[doc = "0x38 - SYS IOMUX CFG SAIF SYSCFG FMUX 14 DOEN"]
    #[inline(always)]
    pub const fn gpo_doen_14(&self) -> &GPO_DOEN_14 {
        &self.gpo_doen_14
    }
    #[doc = "0x3c - SYS IOMUX CFG SAIF SYSCFG FMUX 15 DOEN"]
    #[inline(always)]
    pub const fn gpo_doen_15(&self) -> &GPO_DOEN_15 {
        &self.gpo_doen_15
    }
    #[doc = "0x40 - SYS IOMUX CFG SAIF SYSCFG FMUX GPIO 0-3 DOUT"]
    #[inline(always)]
    pub const fn gpo_dout_0_3(&self) -> &GPO_DOUT_0_3 {
        &self.gpo_dout_0_3
    }
    #[doc = "0x44 - SYS IOMUX CFG SAIF SYSCFG FMUX GPIO 4-7 DOUT"]
    #[inline(always)]
    pub const fn gpo_dout_4_7(&self) -> &GPO_DOUT_4_7 {
        &self.gpo_dout_4_7
    }
    #[doc = "0x48 - SYS IOMUX CFG SAIF SYSCFG FMUX GPIO 8-11 DOUT"]
    #[inline(always)]
    pub const fn gpo_dout_8_11(&self) -> &GPO_DOUT_8_11 {
        &self.gpo_dout_8_11
    }
    #[doc = "0x4c - SYS IOMUX CFG SAIF SYSCFG FMUX GPIO 12-15 DOUT"]
    #[inline(always)]
    pub const fn gpo_dout_12_15(&self) -> &GPO_DOUT_12_15 {
        &self.gpo_dout_12_15
    }
    #[doc = "0x50 - SYS IOMUX CFG SAIF SYSCFG FMUX GPIO 16-19 DOUT"]
    #[inline(always)]
    pub const fn gpo_dout_16_19(&self) -> &GPO_DOUT_16_19 {
        &self.gpo_dout_16_19
    }
    #[doc = "0x54 - SYS IOMUX CFG SAIF SYSCFG FMUX GPIO 20-23 DOUT"]
    #[inline(always)]
    pub const fn gpo_dout_20_23(&self) -> &GPO_DOUT_20_23 {
        &self.gpo_dout_20_23
    }
    #[doc = "0x58 - SYS IOMUX CFG SAIF SYSCFG FMUX GPIO 24-27 DOUT"]
    #[inline(always)]
    pub const fn gpo_dout_24_27(&self) -> &GPO_DOUT_24_27 {
        &self.gpo_dout_24_27
    }
    #[doc = "0x5c - SYS IOMUX CFG SAIF SYSCFG FMUX GPIO 28-31 DOUT"]
    #[inline(always)]
    pub const fn gpo_dout_28_31(&self) -> &GPO_DOUT_28_31 {
        &self.gpo_dout_28_31
    }
    #[doc = "0x60 - SYS IOMUX CFG SAIF SYSCFG FMUX GPIO 32-35 DOUT"]
    #[inline(always)]
    pub const fn gpo_dout_32_35(&self) -> &GPO_DOUT_32_35 {
        &self.gpo_dout_32_35
    }
    #[doc = "0x64 - SYS IOMUX CFG SAIF SYSCFG FMUX GPIO 36-39 DOUT"]
    #[inline(always)]
    pub const fn gpo_dout_36_39(&self) -> &GPO_DOUT_36_39 {
        &self.gpo_dout_36_39
    }
    #[doc = "0x68 - SYS IOMUX CFG SAIF SYSCFG FMUX GPIO 40-43 DOUT"]
    #[inline(always)]
    pub const fn gpo_dout_40_43(&self) -> &GPO_DOUT_40_43 {
        &self.gpo_dout_40_43
    }
    #[doc = "0x6c - SYS IOMUX CFG SAIF SYSCFG FMUX GPIO 44-47 DOUT"]
    #[inline(always)]
    pub const fn gpo_dout_44_47(&self) -> &GPO_DOUT_44_47 {
        &self.gpo_dout_44_47
    }
    #[doc = "0x70 - SYS IOMUX CFG SAIF SYSCFG FMUX GPIO 48-51 DOUT"]
    #[inline(always)]
    pub const fn gpo_dout_48_51(&self) -> &GPO_DOUT_48_51 {
        &self.gpo_dout_48_51
    }
    #[doc = "0x74 - SYS IOMUX CFG SAIF SYSCFG FMUX GPIO 52-55 DOUT"]
    #[inline(always)]
    pub const fn gpo_dout_52_55(&self) -> &GPO_DOUT_52_55 {
        &self.gpo_dout_52_55
    }
    #[doc = "0x78 - SYS IOMUX CFG SAIF SYSCFG FMUX GPIO 56-59 DOUT"]
    #[inline(always)]
    pub const fn gpo_dout_56_59(&self) -> &GPO_DOUT_56_59 {
        &self.gpo_dout_56_59
    }
    #[doc = "0x7c - SYS IOMUX CFG SAIF SYSCFG FMUX GPIO 60-63 DOUT"]
    #[inline(always)]
    pub const fn gpo_dout_60_63(&self) -> &GPO_DOUT_60_63 {
        &self.gpo_dout_60_63
    }
    #[doc = "0x80 - SYS IOMUX CFG SAIF SYSCFG FMUX GPIO GPI 0 - The register can be used to configure the selected GPIO connector number for input signals. The signal name is indicated in the \"Name\" column of the following table per StarFive naming conventions. For example, name \"u0_WAVE511_i_uart_rxsin_cfg\" indicates the corresponding input signal is \"u0_WAVE511.i_uart_rxsin\". See GPIO Input Signals (on page 107) for a complete list of the input GPIO signals."]
    #[inline(always)]
    pub const fn gpi_0(&self) -> &GPI_0 {
        &self.gpi_0
    }
    #[doc = "0x84 - SYS IOMUX CFG SAIF SYSCFG FMUX GPIO GPI 4 - The register can be used to configure the selected GPIO connector number for input signals. The signal name is indicated in the \"Name\" column of the following table per StarFive naming conventions. For example, name \"u0_WAVE511_i_uart_rxsin_cfg\" indicates the corresponding input signal is \"u0_WAVE511.i_uart_rxsin\". See GPIO Input Signals (on page 107) for a complete list of the input GPIO signals."]
    #[inline(always)]
    pub const fn gpi_1(&self) -> &GPI_1 {
        &self.gpi_1
    }
    #[doc = "0x88 - SYS IOMUX CFG SAIF SYSCFG FMUX GPIO GPI 8 - The register can be used to configure the selected GPIO connector number for input signals. The signal name is indicated in the \"Name\" column of the following table per StarFive naming conventions. For example, name \"u0_WAVE511_i_uart_rxsin_cfg\" indicates the corresponding input signal is \"u0_WAVE511.i_uart_rxsin\". See GPIO Input Signals (on page 107) for a complete list of the input GPIO signals."]
    #[inline(always)]
    pub const fn gpi_2(&self) -> &GPI_2 {
        &self.gpi_2
    }
    #[doc = "0x8c - SYS IOMUX CFG SAIF SYSCFG FMUX GPIO GPI 12 - The register can be used to configure the selected GPIO connector number for input signals. The signal name is indicated in the \"Name\" column of the following table per StarFive naming conventions. For example, name \"u0_WAVE511_i_uart_rxsin_cfg\" indicates the corresponding input signal is \"u0_WAVE511.i_uart_rxsin\". See GPIO Input Signals (on page 107) for a complete list of the input GPIO signals."]
    #[inline(always)]
    pub const fn gpi_3(&self) -> &GPI_3 {
        &self.gpi_3
    }
    #[doc = "0x90 - SYS IOMUX CFG SAIF SYSCFG FMUX GPIO GPI 16 - The register can be used to configure the selected GPIO connector number for input signals. The signal name is indicated in the \"Name\" column of the following table per StarFive naming conventions. For example, name \"u0_WAVE511_i_uart_rxsin_cfg\" indicates the corresponding input signal is \"u0_WAVE511.i_uart_rxsin\". See GPIO Input Signals (on page 107) for a complete list of the input GPIO signals."]
    #[inline(always)]
    pub const fn gpi_4(&self) -> &GPI_4 {
        &self.gpi_4
    }
    #[doc = "0x94 - SYS IOMUX CFG SAIF SYSCFG FMUX GPIO GPI 20 - The register can be used to configure the selected GPIO connector number for input signals. The signal name is indicated in the \"Name\" column of the following table per StarFive naming conventions. For example, name \"u0_WAVE511_i_uart_rxsin_cfg\" indicates the corresponding input signal is \"u0_WAVE511.i_uart_rxsin\". See GPIO Input Signals (on page 107) for a complete list of the input GPIO signals."]
    #[inline(always)]
    pub const fn gpi_5(&self) -> &GPI_5 {
        &self.gpi_5
    }
    #[doc = "0x98 - SYS IOMUX CFG SAIF SYSCFG FMUX GPIO GPI 24 - The register can be used to configure the selected GPIO connector number for input signals. The signal name is indicated in the \"Name\" column of the following table per StarFive naming conventions. For example, name \"u0_WAVE511_i_uart_rxsin_cfg\" indicates the corresponding input signal is \"u0_WAVE511.i_uart_rxsin\". See GPIO Input Signals (on page 107) for a complete list of the input GPIO signals."]
    #[inline(always)]
    pub const fn gpi_6(&self) -> &GPI_6 {
        &self.gpi_6
    }
    #[doc = "0x9c - SYS IOMUX CFG SAIF SYSCFG FMUX GPIO GPI 28 - The register can be used to configure the selected GPIO connector number for input signals. The signal name is indicated in the \"Name\" column of the following table per StarFive naming conventions. For example, name \"u0_WAVE511_i_uart_rxsin_cfg\" indicates the corresponding input signal is \"u0_WAVE511.i_uart_rxsin\". See GPIO Input Signals (on page 107) for a complete list of the input GPIO signals."]
    #[inline(always)]
    pub const fn gpi_7(&self) -> &GPI_7 {
        &self.gpi_7
    }
    #[doc = "0xa0 - SYS IOMUX CFG SAIF SYSCFG FMUX GPIO GPI 32 - The register can be used to configure the selected GPIO connector number for input signals. The signal name is indicated in the \"Name\" column of the following table per StarFive naming conventions. For example, name \"u0_WAVE511_i_uart_rxsin_cfg\" indicates the corresponding input signal is \"u0_WAVE511.i_uart_rxsin\". See GPIO Input Signals (on page 107) for a complete list of the input GPIO signals."]
    #[inline(always)]
    pub const fn gpi_8(&self) -> &GPI_8 {
        &self.gpi_8
    }
    #[doc = "0xa4 - SYS IOMUX CFG SAIF SYSCFG FMUX GPIO GPI 36 - The register can be used to configure the selected GPIO connector number for input signals. The signal name is indicated in the \"Name\" column of the following table per StarFive naming conventions. For example, name \"u0_WAVE511_i_uart_rxsin_cfg\" indicates the corresponding input signal is \"u0_WAVE511.i_uart_rxsin\". See GPIO Input Signals (on page 107) for a complete list of the input GPIO signals."]
    #[inline(always)]
    pub const fn gpi_9(&self) -> &GPI_9 {
        &self.gpi_9
    }
    #[doc = "0xa8 - SYS IOMUX CFG SAIF SYSCFG FMUX GPIO GPI 40 - The register can be used to configure the selected GPIO connector number for input signals. The signal name is indicated in the \"Name\" column of the following table per StarFive naming conventions. For example, name \"u0_WAVE511_i_uart_rxsin_cfg\" indicates the corresponding input signal is \"u0_WAVE511.i_uart_rxsin\". See GPIO Input Signals (on page 107) for a complete list of the input GPIO signals."]
    #[inline(always)]
    pub const fn gpi_10(&self) -> &GPI_10 {
        &self.gpi_10
    }
    #[doc = "0xac - SYS IOMUX CFG SAIF SYSCFG FMUX GPIO GPI 44 - The register can be used to configure the selected GPIO connector number for input signals. The signal name is indicated in the \"Name\" column of the following table per StarFive naming conventions. For example, name \"u0_WAVE511_i_uart_rxsin_cfg\" indicates the corresponding input signal is \"u0_WAVE511.i_uart_rxsin\". See GPIO Input Signals (on page 107) for a complete list of the input GPIO signals."]
    #[inline(always)]
    pub const fn gpi_11(&self) -> &GPI_11 {
        &self.gpi_11
    }
    #[doc = "0xb0 - SYS IOMUX CFG SAIF SYSCFG FMUX GPIO GPI 48 - The register can be used to configure the selected GPIO connector number for input signals. The signal name is indicated in the \"Name\" column of the following table per StarFive naming conventions. For example, name \"u0_WAVE511_i_uart_rxsin_cfg\" indicates the corresponding input signal is \"u0_WAVE511.i_uart_rxsin\". See GPIO Input Signals (on page 107) for a complete list of the input GPIO signals."]
    #[inline(always)]
    pub const fn gpi_12(&self) -> &GPI_12 {
        &self.gpi_12
    }
    #[doc = "0xb4 - SYS IOMUX CFG SAIF SYSCFG FMUX GPIO GPI 52 - The register can be used to configure the selected GPIO connector number for input signals. The signal name is indicated in the \"Name\" column of the following table per StarFive naming conventions. For example, name \"u0_WAVE511_i_uart_rxsin_cfg\" indicates the corresponding input signal is \"u0_WAVE511.i_uart_rxsin\". See GPIO Input Signals (on page 107) for a complete list of the input GPIO signals."]
    #[inline(always)]
    pub const fn gpi_13(&self) -> &GPI_13 {
        &self.gpi_13
    }
    #[doc = "0xb8 - SYS IOMUX CFG SAIF SYSCFG FMUX GPIO GPI 56 - The register can be used to configure the selected GPIO connector number for input signals. The signal name is indicated in the \"Name\" column of the following table per StarFive naming conventions. For example, name \"u0_WAVE511_i_uart_rxsin_cfg\" indicates the corresponding input signal is \"u0_WAVE511.i_uart_rxsin\". See GPIO Input Signals (on page 107) for a complete list of the input GPIO signals."]
    #[inline(always)]
    pub const fn gpi_14(&self) -> &GPI_14 {
        &self.gpi_14
    }
    #[doc = "0xbc - SYS IOMUX CFG SAIF SYSCFG FMUX GPIO GPI 60 - The register can be used to configure the selected GPIO connector number for input signals. The signal name is indicated in the \"Name\" column of the following table per StarFive naming conventions. For example, name \"u0_WAVE511_i_uart_rxsin_cfg\" indicates the corresponding input signal is \"u0_WAVE511.i_uart_rxsin\". See GPIO Input Signals (on page 107) for a complete list of the input GPIO signals."]
    #[inline(always)]
    pub const fn gpi_15(&self) -> &GPI_15 {
        &self.gpi_15
    }
    #[doc = "0xc0 - SYS IOMUX CFG SAIF SYSCFG FMUX GPIO GPI 64 - The register can be used to configure the selected GPIO connector number for input signals. The signal name is indicated in the \"Name\" column of the following table per StarFive naming conventions. For example, name \"u0_WAVE511_i_uart_rxsin_cfg\" indicates the corresponding input signal is \"u0_WAVE511.i_uart_rxsin\". See GPIO Input Signals (on page 107) for a complete list of the input GPIO signals."]
    #[inline(always)]
    pub const fn gpi_16(&self) -> &GPI_16 {
        &self.gpi_16
    }
    #[doc = "0xc4 - SYS IOMUX CFG SAIF SYSCFG FMUX GPIO GPI 68 - The register can be used to configure the selected GPIO connector number for input signals. The signal name is indicated in the \"Name\" column of the following table per StarFive naming conventions. For example, name \"u0_WAVE511_i_uart_rxsin_cfg\" indicates the corresponding input signal is \"u0_WAVE511.i_uart_rxsin\". See GPIO Input Signals (on page 107) for a complete list of the input GPIO signals."]
    #[inline(always)]
    pub const fn gpi_17(&self) -> &GPI_17 {
        &self.gpi_17
    }
    #[doc = "0xc8 - SYS IOMUX CFG SAIF SYSCFG FMUX GPIO GPI 72 - The register can be used to configure the selected GPIO connector number for input signals. The signal name is indicated in the \"Name\" column of the following table per StarFive naming conventions. For example, name \"u0_WAVE511_i_uart_rxsin_cfg\" indicates the corresponding input signal is \"u0_WAVE511.i_uart_rxsin\". See GPIO Input Signals (on page 107) for a complete list of the input GPIO signals."]
    #[inline(always)]
    pub const fn gpi_18(&self) -> &GPI_18 {
        &self.gpi_18
    }
    #[doc = "0xcc - SYS IOMUX CFG SAIF SYSCFG FMUX GPIO GPI 76 - The register can be used to configure the selected GPIO connector number for input signals. The signal name is indicated in the \"Name\" column of the following table per StarFive naming conventions. For example, name \"u0_WAVE511_i_uart_rxsin_cfg\" indicates the corresponding input signal is \"u0_WAVE511.i_uart_rxsin\". See GPIO Input Signals (on page 107) for a complete list of the input GPIO signals."]
    #[inline(always)]
    pub const fn gpi_19(&self) -> &GPI_19 {
        &self.gpi_19
    }
    #[doc = "0xd0 - SYS IOMUX CFG SAIF SYSCFG FMUX GPIO GPI 80 - The register can be used to configure the selected GPIO connector number for input signals. The signal name is indicated in the \"Name\" column of the following table per StarFive naming conventions. For example, name \"u0_WAVE511_i_uart_rxsin_cfg\" indicates the corresponding input signal is \"u0_WAVE511.i_uart_rxsin\". See GPIO Input Signals (on page 107) for a complete list of the input GPIO signals."]
    #[inline(always)]
    pub const fn gpi_20(&self) -> &GPI_20 {
        &self.gpi_20
    }
    #[doc = "0xd4 - SYS IOMUX CFG SAIF SYSCFG FMUX GPIO GPI 84 - The register can be used to configure the selected GPIO connector number for input signals. The signal name is indicated in the \"Name\" column of the following table per StarFive naming conventions. For example, name \"u0_WAVE511_i_uart_rxsin_cfg\" indicates the corresponding input signal is \"u0_WAVE511.i_uart_rxsin\". See GPIO Input Signals (on page 107) for a complete list of the input GPIO signals."]
    #[inline(always)]
    pub const fn gpi_21(&self) -> &GPI_21 {
        &self.gpi_21
    }
    #[doc = "0xd8 - SYS IOMUX CFG SAIF SYSCFG FMUX GPIO GPI 88 - The register can be used to configure the selected GPIO connector number for input signals. The signal name is indicated in the \"Name\" column of the following table per StarFive naming conventions. For example, name \"u0_WAVE511_i_uart_rxsin_cfg\" indicates the corresponding input signal is \"u0_WAVE511.i_uart_rxsin\". See GPIO Input Signals (on page 107) for a complete list of the input GPIO signals."]
    #[inline(always)]
    pub const fn gpi_22(&self) -> &GPI_22 {
        &self.gpi_22
    }
    #[doc = "0xdc - Enable GPIO IRQ function"]
    #[inline(always)]
    pub const fn ioirq_0(&self) -> &IOIRQ_0 {
        &self.ioirq_0
    }
    #[doc = "0xe0 - SYS IOMUX CFGSAIF SYSCFG IOIRQ 224: GPIO Interrupt Edge Trigger Selector"]
    #[inline(always)]
    pub const fn ioirq_1(&self) -> &IOIRQ_1 {
        &self.ioirq_1
    }
    #[doc = "0xe4 - SYS IOMUX CFGSAIF SYSCFG IOIRQ 228: GPIO Interrupt Edge Trigger Selector"]
    #[inline(always)]
    pub const fn ioirq_2(&self) -> &IOIRQ_2 {
        &self.ioirq_2
    }
    #[doc = "0xe8 - SYS IOMUX CFGSAIF SYSCFG IOIRQ 232: GPIO Interrupt Clear"]
    #[inline(always)]
    pub const fn ioirq_3(&self) -> &IOIRQ_3 {
        &self.ioirq_3
    }
    #[doc = "0xec - SYS IOMUX CFGSAIF SYSCFG IOIRQ 236: GPIO Interrupt Clear"]
    #[inline(always)]
    pub const fn ioirq_4(&self) -> &IOIRQ_4 {
        &self.ioirq_4
    }
    #[doc = "0xf0 - SYS IOMUX CFGSAIF SYSCFG IOIRQ 240: GPIO Interrupt Both Edge Trigger Selector"]
    #[inline(always)]
    pub const fn ioirq_5(&self) -> &IOIRQ_5 {
        &self.ioirq_5
    }
    #[doc = "0xf4 - SYS IOMUX CFGSAIF SYSCFG IOIRQ 244: GPIO Interrupt Both Edge Trigger Selector"]
    #[inline(always)]
    pub const fn ioirq_6(&self) -> &IOIRQ_6 {
        &self.ioirq_6
    }
    #[doc = "0xf8 - SYS IOMUX CFGSAIF SYSCFG IOIRQ 248: GPIO Interrupt Edge Value"]
    #[inline(always)]
    pub const fn ioirq_7(&self) -> &IOIRQ_7 {
        &self.ioirq_7
    }
    #[doc = "0xfc - SYS IOMUX CFGSAIF SYSCFG IOIRQ 252: GPIO Interrupt Edge Value"]
    #[inline(always)]
    pub const fn ioirq_8(&self) -> &IOIRQ_8 {
        &self.ioirq_8
    }
    #[doc = "0x100 - SYS IOMUX CFGSAIF SYSCFG IOIRQ 256: GPIO Interrupt Edge Mask Selector"]
    #[inline(always)]
    pub const fn ioirq_9(&self) -> &IOIRQ_9 {
        &self.ioirq_9
    }
    #[doc = "0x104 - SYS IOMUX CFGSAIF SYSCFG IOIRQ 260: GPIO Interrupt Edge Mask Selector"]
    #[inline(always)]
    pub const fn ioirq_10(&self) -> &IOIRQ_10 {
        &self.ioirq_10
    }
    #[doc = "0x108 - SYS IOMUX CFGSAIF SYSCFG IOIRQ 264: GPIO Register Interrupt Status"]
    #[inline(always)]
    pub const fn ioirq_11(&self) -> &IOIRQ_11 {
        &self.ioirq_11
    }
    #[doc = "0x10c - SYS IOMUX CFGSAIF SYSCFG IOIRQ 268: GPIO Register Interrupt Status"]
    #[inline(always)]
    pub const fn ioirq_12(&self) -> &IOIRQ_12 {
        &self.ioirq_12
    }
    #[doc = "0x110 - SYS IOMUX CFGSAIF SYSCFG IOIRQ 272: GPIO Masked Interrupt Status"]
    #[inline(always)]
    pub const fn ioirq_13(&self) -> &IOIRQ_13 {
        &self.ioirq_13
    }
    #[doc = "0x114 - SYS IOMUX CFGSAIF SYSCFG IOIRQ 276: GPIO Masked Interrupt Status"]
    #[inline(always)]
    pub const fn ioirq_14(&self) -> &IOIRQ_14 {
        &self.ioirq_14
    }
    #[doc = "0x118 - SYS IOMUX CFGSAIF SYSCFG IOIRQ 280: GPIO Synchronization Status"]
    #[inline(always)]
    pub const fn ioirq_15(&self) -> &IOIRQ_15 {
        &self.ioirq_15
    }
    #[doc = "0x11c - SYS IOMUX CFGSAIF SYSCFG IOIRQ 284: GPIO Synchronization Status"]
    #[inline(always)]
    pub const fn ioirq_16(&self) -> &IOIRQ_16 {
        &self.ioirq_16
    }
    #[doc = "0x120 - SYS IOMUX CFG SAIF SYSCFG PADCFG 288: GPIO_0"]
    #[inline(always)]
    pub const fn gpio_0(&self) -> &GPIO_0 {
        &self.gpio_0
    }
    #[doc = "0x124 - SYS IOMUX CFG SAIF SYSCFG PADCFG 292: GPIO_1"]
    #[inline(always)]
    pub const fn gpio_1(&self) -> &GPIO_1 {
        &self.gpio_1
    }
    #[doc = "0x128 - SYS IOMUX CFG SAIF SYSCFG PADCFG 296: GPIO_2"]
    #[inline(always)]
    pub const fn gpio_2(&self) -> &GPIO_2 {
        &self.gpio_2
    }
    #[doc = "0x12c - SYS IOMUX CFG SAIF SYSCFG PADCFG 300: GPIO_3"]
    #[inline(always)]
    pub const fn gpio_3(&self) -> &GPIO_3 {
        &self.gpio_3
    }
    #[doc = "0x130 - SYS IOMUX CFG SAIF SYSCFG PADCFG 304: GPIO_4"]
    #[inline(always)]
    pub const fn gpio_4(&self) -> &GPIO_4 {
        &self.gpio_4
    }
    #[doc = "0x134 - SYS IOMUX CFG SAIF SYSCFG PADCFG 308: GPIO_5"]
    #[inline(always)]
    pub const fn gpio_5(&self) -> &GPIO_5 {
        &self.gpio_5
    }
    #[doc = "0x138 - SYS IOMUX CFG SAIF SYSCFG PADCFG 312: GPIO_6"]
    #[inline(always)]
    pub const fn gpio_6(&self) -> &GPIO_6 {
        &self.gpio_6
    }
    #[doc = "0x13c - SYS IOMUX CFG SAIF SYSCFG PADCFG 316: GPIO_7"]
    #[inline(always)]
    pub const fn gpio_7(&self) -> &GPIO_7 {
        &self.gpio_7
    }
    #[doc = "0x140 - SYS IOMUX CFG SAIF SYSCFG PADCFG 320: GPIO_8"]
    #[inline(always)]
    pub const fn gpio_8(&self) -> &GPIO_8 {
        &self.gpio_8
    }
    #[doc = "0x144 - SYS IOMUX CFG SAIF SYSCFG PADCFG 324: GPIO_9"]
    #[inline(always)]
    pub const fn gpio_9(&self) -> &GPIO_9 {
        &self.gpio_9
    }
    #[doc = "0x148 - SYS IOMUX CFG SAIF SYSCFG PADCFG 328: GPIO_10"]
    #[inline(always)]
    pub const fn gpio_10(&self) -> &GPIO_10 {
        &self.gpio_10
    }
    #[doc = "0x14c - SYS IOMUX CFG SAIF SYSCFG PADCFG 332: GPIO_11"]
    #[inline(always)]
    pub const fn gpio_11(&self) -> &GPIO_11 {
        &self.gpio_11
    }
    #[doc = "0x150 - SYS IOMUX CFG SAIF SYSCFG PADCFG 336: GPIO_12"]
    #[inline(always)]
    pub const fn gpio_12(&self) -> &GPIO_12 {
        &self.gpio_12
    }
    #[doc = "0x154 - SYS IOMUX CFG SAIF SYSCFG PADCFG 340: GPIO_13"]
    #[inline(always)]
    pub const fn gpio_13(&self) -> &GPIO_13 {
        &self.gpio_13
    }
    #[doc = "0x158 - SYS IOMUX CFG SAIF SYSCFG PADCFG 344: GPIO_14"]
    #[inline(always)]
    pub const fn gpio_14(&self) -> &GPIO_14 {
        &self.gpio_14
    }
    #[doc = "0x15c - SYS IOMUX CFG SAIF SYSCFG PADCFG 348: GPIO_15"]
    #[inline(always)]
    pub const fn gpio_15(&self) -> &GPIO_15 {
        &self.gpio_15
    }
    #[doc = "0x160 - SYS IOMUX CFG SAIF SYSCFG PADCFG 352: GPIO_16"]
    #[inline(always)]
    pub const fn gpio_16(&self) -> &GPIO_16 {
        &self.gpio_16
    }
    #[doc = "0x164 - SYS IOMUX CFG SAIF SYSCFG PADCFG 356: GPIO_17"]
    #[inline(always)]
    pub const fn gpio_17(&self) -> &GPIO_17 {
        &self.gpio_17
    }
    #[doc = "0x168 - SYS IOMUX CFG SAIF SYSCFG PADCFG 360: GPIO_18"]
    #[inline(always)]
    pub const fn gpio_18(&self) -> &GPIO_18 {
        &self.gpio_18
    }
    #[doc = "0x16c - SYS IOMUX CFG SAIF SYSCFG PADCFG 364: GPIO_19"]
    #[inline(always)]
    pub const fn gpio_19(&self) -> &GPIO_19 {
        &self.gpio_19
    }
    #[doc = "0x170 - SYS IOMUX CFG SAIF SYSCFG PADCFG 368: GPIO_20"]
    #[inline(always)]
    pub const fn gpio_20(&self) -> &GPIO_20 {
        &self.gpio_20
    }
    #[doc = "0x174 - SYS IOMUX CFG SAIF SYSCFG PADCFG 372: GPIO_21"]
    #[inline(always)]
    pub const fn gpio_21(&self) -> &GPIO_21 {
        &self.gpio_21
    }
    #[doc = "0x178 - SYS IOMUX CFG SAIF SYSCFG PADCFG 376: GPIO_22"]
    #[inline(always)]
    pub const fn gpio_22(&self) -> &GPIO_22 {
        &self.gpio_22
    }
    #[doc = "0x17c - SYS IOMUX CFG SAIF SYSCFG PADCFG 380: GPIO_23"]
    #[inline(always)]
    pub const fn gpio_23(&self) -> &GPIO_23 {
        &self.gpio_23
    }
    #[doc = "0x180 - SYS IOMUX CFG SAIF SYSCFG PADCFG 384: GPIO_24"]
    #[inline(always)]
    pub const fn gpio_24(&self) -> &GPIO_24 {
        &self.gpio_24
    }
    #[doc = "0x184 - SYS IOMUX CFG SAIF SYSCFG PADCFG 388: GPIO_25"]
    #[inline(always)]
    pub const fn gpio_25(&self) -> &GPIO_25 {
        &self.gpio_25
    }
    #[doc = "0x188 - SYS IOMUX CFG SAIF SYSCFG PADCFG 392: GPIO_26"]
    #[inline(always)]
    pub const fn gpio_26(&self) -> &GPIO_26 {
        &self.gpio_26
    }
    #[doc = "0x18c - SYS IOMUX CFG SAIF SYSCFG PADCFG 396: GPIO_27"]
    #[inline(always)]
    pub const fn gpio_27(&self) -> &GPIO_27 {
        &self.gpio_27
    }
    #[doc = "0x190 - SYS IOMUX CFG SAIF SYSCFG PADCFG 400: GPIO_28"]
    #[inline(always)]
    pub const fn gpio_28(&self) -> &GPIO_28 {
        &self.gpio_28
    }
    #[doc = "0x194 - SYS IOMUX CFG SAIF SYSCFG PADCFG 404: GPIO_29"]
    #[inline(always)]
    pub const fn gpio_29(&self) -> &GPIO_29 {
        &self.gpio_29
    }
    #[doc = "0x198 - SYS IOMUX CFG SAIF SYSCFG PADCFG 408: GPIO_30"]
    #[inline(always)]
    pub const fn gpio_30(&self) -> &GPIO_30 {
        &self.gpio_30
    }
    #[doc = "0x19c - SYS IOMUX CFG SAIF SYSCFG PADCFG 412: GPIO_31"]
    #[inline(always)]
    pub const fn gpio_31(&self) -> &GPIO_31 {
        &self.gpio_31
    }
    #[doc = "0x1a0 - SYS IOMUX CFG SAIF SYSCFG PADCFG 416: GPIO_32"]
    #[inline(always)]
    pub const fn gpio_32(&self) -> &GPIO_32 {
        &self.gpio_32
    }
    #[doc = "0x1a4 - SYS IOMUX CFG SAIF SYSCFG PADCFG 420: GPIO_33"]
    #[inline(always)]
    pub const fn gpio_33(&self) -> &GPIO_33 {
        &self.gpio_33
    }
    #[doc = "0x1a8 - SYS IOMUX CFG SAIF SYSCFG PADCFG 424: GPIO_34"]
    #[inline(always)]
    pub const fn gpio_34(&self) -> &GPIO_34 {
        &self.gpio_34
    }
    #[doc = "0x1ac - SYS IOMUX CFG SAIF SYSCFG PADCFG 428: GPIO_35"]
    #[inline(always)]
    pub const fn gpio_35(&self) -> &GPIO_35 {
        &self.gpio_35
    }
    #[doc = "0x1b0 - SYS IOMUX CFG SAIF SYSCFG PADCFG 432: GPIO_36"]
    #[inline(always)]
    pub const fn gpio_36(&self) -> &GPIO_36 {
        &self.gpio_36
    }
    #[doc = "0x1b4 - SYS IOMUX CFG SAIF SYSCFG PADCFG 436: GPIO_37"]
    #[inline(always)]
    pub const fn gpio_37(&self) -> &GPIO_37 {
        &self.gpio_37
    }
    #[doc = "0x1b8 - SYS IOMUX CFG SAIF SYSCFG PADCFG 440: GPIO_38"]
    #[inline(always)]
    pub const fn gpio_38(&self) -> &GPIO_38 {
        &self.gpio_38
    }
    #[doc = "0x1bc - SYS IOMUX CFG SAIF SYSCFG PADCFG 444: GPIO_39"]
    #[inline(always)]
    pub const fn gpio_39(&self) -> &GPIO_39 {
        &self.gpio_39
    }
    #[doc = "0x1c0 - SYS IOMUX CFG SAIF SYSCFG PADCFG 448: GPIO_40"]
    #[inline(always)]
    pub const fn gpio_40(&self) -> &GPIO_40 {
        &self.gpio_40
    }
    #[doc = "0x1c4 - SYS IOMUX CFG SAIF SYSCFG PADCFG 452: GPIO_41"]
    #[inline(always)]
    pub const fn gpio_41(&self) -> &GPIO_41 {
        &self.gpio_41
    }
    #[doc = "0x1c8 - SYS IOMUX CFG SAIF SYSCFG PADCFG 456: GPIO_42"]
    #[inline(always)]
    pub const fn gpio_42(&self) -> &GPIO_42 {
        &self.gpio_42
    }
    #[doc = "0x1cc - SYS IOMUX CFG SAIF SYSCFG PADCFG 460: GPIO_43"]
    #[inline(always)]
    pub const fn gpio_43(&self) -> &GPIO_43 {
        &self.gpio_43
    }
    #[doc = "0x1d0 - SYS IOMUX CFG SAIF SYSCFG PADCFG 464: GPIO_44"]
    #[inline(always)]
    pub const fn gpio_44(&self) -> &GPIO_44 {
        &self.gpio_44
    }
    #[doc = "0x1d4 - SYS IOMUX CFG SAIF SYSCFG PADCFG 468: GPIO_45"]
    #[inline(always)]
    pub const fn gpio_45(&self) -> &GPIO_45 {
        &self.gpio_45
    }
    #[doc = "0x1d8 - SYS IOMUX CFG SAIF SYSCFG PADCFG 472: GPIO_46"]
    #[inline(always)]
    pub const fn gpio_46(&self) -> &GPIO_46 {
        &self.gpio_46
    }
    #[doc = "0x1dc - SYS IOMUX CFG SAIF SYSCFG PADCFG 476: GPIO_47"]
    #[inline(always)]
    pub const fn gpio_47(&self) -> &GPIO_47 {
        &self.gpio_47
    }
    #[doc = "0x1e0 - SYS IOMUX CFG SAIF SYSCFG PADCFG 480: GPIO_48"]
    #[inline(always)]
    pub const fn gpio_48(&self) -> &GPIO_48 {
        &self.gpio_48
    }
    #[doc = "0x1e4 - SYS IOMUX CFG SAIF SYSCFG PADCFG 484: GPIO_49"]
    #[inline(always)]
    pub const fn gpio_49(&self) -> &GPIO_49 {
        &self.gpio_49
    }
    #[doc = "0x1e8 - SYS IOMUX CFG SAIF SYSCFG PADCFG 488: GPIO_50"]
    #[inline(always)]
    pub const fn gpio_50(&self) -> &GPIO_50 {
        &self.gpio_50
    }
    #[doc = "0x1ec - SYS IOMUX CFG SAIF SYSCFG PADCFG 492: GPIO_51"]
    #[inline(always)]
    pub const fn gpio_51(&self) -> &GPIO_51 {
        &self.gpio_51
    }
    #[doc = "0x1f0 - SYS IOMUX CFG SAIF SYSCFG PADCFG 496: GPIO_52"]
    #[inline(always)]
    pub const fn gpio_52(&self) -> &GPIO_52 {
        &self.gpio_52
    }
    #[doc = "0x1f4 - SYS IOMUX CFG SAIF SYSCFG PADCFG 500: GPIO_53"]
    #[inline(always)]
    pub const fn gpio_53(&self) -> &GPIO_53 {
        &self.gpio_53
    }
    #[doc = "0x1f8 - SYS IOMUX CFG SAIF SYSCFG PADCFG 504: GPIO_54"]
    #[inline(always)]
    pub const fn gpio_54(&self) -> &GPIO_54 {
        &self.gpio_54
    }
    #[doc = "0x1fc - SYS IOMUX CFG SAIF SYSCFG PADCFG 508: GPIO_55"]
    #[inline(always)]
    pub const fn gpio_55(&self) -> &GPIO_55 {
        &self.gpio_55
    }
    #[doc = "0x200 - SYS IOMUX CFG SAIF SYSCFG PADCFG 512: GPIO_56"]
    #[inline(always)]
    pub const fn gpio_56(&self) -> &GPIO_56 {
        &self.gpio_56
    }
    #[doc = "0x204 - SYS IOMUX CFG SAIF SYSCFG PADCFG 516: GPIO_57"]
    #[inline(always)]
    pub const fn gpio_57(&self) -> &GPIO_57 {
        &self.gpio_57
    }
    #[doc = "0x208 - SYS IOMUX CFG SAIF SYSCFG PADCFG 520: GPIO_58"]
    #[inline(always)]
    pub const fn gpio_58(&self) -> &GPIO_58 {
        &self.gpio_58
    }
    #[doc = "0x20c - SYS IOMUX CFG SAIF SYSCFG PADCFG 524: GPIO_59"]
    #[inline(always)]
    pub const fn gpio_59(&self) -> &GPIO_59 {
        &self.gpio_59
    }
    #[doc = "0x210 - SYS IOMUX CFG SAIF SYSCFG PADCFG 528: GPIO_60"]
    #[inline(always)]
    pub const fn gpio_60(&self) -> &GPIO_60 {
        &self.gpio_60
    }
    #[doc = "0x214 - SYS IOMUX CFG SAIF SYSCFG PADCFG 532: GPIO_61"]
    #[inline(always)]
    pub const fn gpio_61(&self) -> &GPIO_61 {
        &self.gpio_61
    }
    #[doc = "0x218 - SYS IOMUX CFG SAIF SYSCFG PADCFG 536: GPIO_62"]
    #[inline(always)]
    pub const fn gpio_62(&self) -> &GPIO_62 {
        &self.gpio_62
    }
    #[doc = "0x21c - SYS IOMUX CFG SAIF SYSCFG PADCFG 540: GPIO_63"]
    #[inline(always)]
    pub const fn gpio_63(&self) -> &GPIO_63 {
        &self.gpio_63
    }
    #[doc = "0x220 - SYS IOMUX CFG SAIF SYSCFG PADCFG 544: SD0_CLK"]
    #[inline(always)]
    pub const fn sd0_clk(&self) -> &SD0_CLK {
        &self.sd0_clk
    }
    #[doc = "0x224 - SYS IOMUX CFG SAIF SYSCFG PADCFG 548: SD0_CMD"]
    #[inline(always)]
    pub const fn sd0_cmd(&self) -> &SD0_CMD {
        &self.sd0_cmd
    }
    #[doc = "0x228 - SYS IOMUX CFG SAIF SYSCFG PADCFG 552: SD0_DATA_0"]
    #[inline(always)]
    pub const fn sd0_data_0(&self) -> &SD0_DATA_0 {
        &self.sd0_data_0
    }
    #[doc = "0x22c - SYS IOMUX CFG SAIF SYSCFG PADCFG 556: SD0_DATA_1"]
    #[inline(always)]
    pub const fn sd0_data_1(&self) -> &SD0_DATA_1 {
        &self.sd0_data_1
    }
    #[doc = "0x230 - SYS IOMUX CFG SAIF SYSCFG PADCFG 560: SD0_DATA_2"]
    #[inline(always)]
    pub const fn sd0_data_2(&self) -> &SD0_DATA_2 {
        &self.sd0_data_2
    }
    #[doc = "0x234 - SYS IOMUX CFG SAIF SYSCFG PADCFG 564: SD0_DATA_3"]
    #[inline(always)]
    pub const fn sd0_data_3(&self) -> &SD0_DATA_3 {
        &self.sd0_data_3
    }
    #[doc = "0x238 - SYS IOMUX CFG SAIF SYSCFG PADCFG 568: SD0_DATA_4"]
    #[inline(always)]
    pub const fn sd0_data_4(&self) -> &SD0_DATA_4 {
        &self.sd0_data_4
    }
    #[doc = "0x23c - SYS IOMUX CFG SAIF SYSCFG PADCFG 572: SD0_DATA_5"]
    #[inline(always)]
    pub const fn sd0_data_5(&self) -> &SD0_DATA_5 {
        &self.sd0_data_5
    }
    #[doc = "0x240 - SYS IOMUX CFG SAIF SYSCFG PADCFG 576: SD0_DATA_6"]
    #[inline(always)]
    pub const fn sd0_data_6(&self) -> &SD0_DATA_6 {
        &self.sd0_data_6
    }
    #[doc = "0x244 - SYS IOMUX CFG SAIF SYSCFG PADCFG 580: SD0_DATA_7"]
    #[inline(always)]
    pub const fn sd0_data_7(&self) -> &SD0_DATA_7 {
        &self.sd0_data_7
    }
    #[doc = "0x248 - SYS IOMUX CFG SAIF SYSCFG PADCFG 584: SD0_STRB"]
    #[inline(always)]
    pub const fn sd0_strb(&self) -> &SD0_STRB {
        &self.sd0_strb
    }
    #[doc = "0x24c - GPIO GMAC1 MDC Pad Configuration"]
    #[inline(always)]
    pub const fn gmac1_mdc(&self) -> &GMAC1_MDC {
        &self.gmac1_mdc
    }
    #[doc = "0x250 - GPIO GMAC1 MDIO Pad Configuration"]
    #[inline(always)]
    pub const fn gmac1_mdio(&self) -> &GMAC1_MDIO {
        &self.gmac1_mdio
    }
    #[doc = "0x254 - GPIO GMAC1 RXD_0 Pad Configuration"]
    #[inline(always)]
    pub const fn gmac1_rxd_0(&self) -> &GMAC1_RXD_0 {
        &self.gmac1_rxd_0
    }
    #[doc = "0x258 - GPIO GMAC1 RXD_1 Pad Configuration"]
    #[inline(always)]
    pub const fn gmac1_rxd_1(&self) -> &GMAC1_RXD_1 {
        &self.gmac1_rxd_1
    }
    #[doc = "0x25c - GPIO GMAC1 RXD_2 Pad Configuration"]
    #[inline(always)]
    pub const fn gmac1_rxd_2(&self) -> &GMAC1_RXD_2 {
        &self.gmac1_rxd_2
    }
    #[doc = "0x260 - GPIO GMAC1 RXD_3 Pad Configuration"]
    #[inline(always)]
    pub const fn gmac1_rxd_3(&self) -> &GMAC1_RXD_3 {
        &self.gmac1_rxd_3
    }
    #[doc = "0x264 - GPIO GMAC1 RXDV Pad Configuration"]
    #[inline(always)]
    pub const fn gmac1_rxdv(&self) -> &GMAC1_RXDV {
        &self.gmac1_rxdv
    }
    #[doc = "0x268 - GPIO GMAC1 RXC Pad Configuration"]
    #[inline(always)]
    pub const fn gmac1_rxc(&self) -> &GMAC1_RXC {
        &self.gmac1_rxc
    }
    #[doc = "0x26c - GPIO GMAC1 TXD_0 Pad Configuration"]
    #[inline(always)]
    pub const fn gmac1_txd_0(&self) -> &GMAC1_TXD_0 {
        &self.gmac1_txd_0
    }
    #[doc = "0x270 - GPIO GMAC1 TXD_1 Pad Configuration"]
    #[inline(always)]
    pub const fn gmac1_txd_1(&self) -> &GMAC1_TXD_1 {
        &self.gmac1_txd_1
    }
    #[doc = "0x274 - GPIO GMAC1 TXD_2 Pad Configuration"]
    #[inline(always)]
    pub const fn gmac1_txd_2(&self) -> &GMAC1_TXD_2 {
        &self.gmac1_txd_2
    }
    #[doc = "0x278 - GPIO GMAC1 TXD_3 Pad Configuration"]
    #[inline(always)]
    pub const fn gmac1_txd_3(&self) -> &GMAC1_TXD_3 {
        &self.gmac1_txd_3
    }
    #[doc = "0x27c - GPIO GMAC1 TXEN Pad Configuration"]
    #[inline(always)]
    pub const fn gmac1_txen(&self) -> &GMAC1_TXEN {
        &self.gmac1_txen
    }
    #[doc = "0x280 - GPIO GMAC1 TXC Pad Configuration"]
    #[inline(always)]
    pub const fn gmac1_txc(&self) -> &GMAC1_TXC {
        &self.gmac1_txc
    }
    #[doc = "0x284 - SYS IOMUX CFG SAIF SYSCFG PADCFG 644: QSPI_SCLK"]
    #[inline(always)]
    pub const fn qspi_sclk(&self) -> &QSPI_SCLK {
        &self.qspi_sclk
    }
    #[doc = "0x288 - SYS IOMUX CFG SAIF SYSCFG PADCFG 648: QSPI_CSN_0"]
    #[inline(always)]
    pub const fn qspi_csn_0(&self) -> &QSPI_CSN_0 {
        &self.qspi_csn_0
    }
    #[doc = "0x28c - SYS IOMUX CFG SAIF SYSCFG PADCFG 652: QSPI_DATA_0"]
    #[inline(always)]
    pub const fn qspi_data_0(&self) -> &QSPI_DATA_0 {
        &self.qspi_data_0
    }
    #[doc = "0x290 - SYS IOMUX CFG SAIF SYSCFG PADCFG 656: QSPI_DATA_1"]
    #[inline(always)]
    pub const fn qspi_data_1(&self) -> &QSPI_DATA_1 {
        &self.qspi_data_1
    }
    #[doc = "0x294 - SYS IOMUX CFG SAIF SYSCFG PADCFG 660: QSPI_DATA_2"]
    #[inline(always)]
    pub const fn qspi_data_2(&self) -> &QSPI_DATA_2 {
        &self.qspi_data_2
    }
    #[doc = "0x298 - SYS IOMUX CFG SAIF SYSCFG PADCFG 664: QSPI_DATA_3"]
    #[inline(always)]
    pub const fn qspi_data_3(&self) -> &QSPI_DATA_3 {
        &self.qspi_data_3
    }
    #[doc = "0x29c - SYS IOMUX CFG SAIF SYSCFG 668"]
    #[inline(always)]
    pub const fn func_sel_0(&self) -> &FUNC_SEL_0 {
        &self.func_sel_0
    }
    #[doc = "0x2a0 - SYS IOMUX CFG SAIF SYSCFG 672"]
    #[inline(always)]
    pub const fn func_sel_1(&self) -> &FUNC_SEL_1 {
        &self.func_sel_1
    }
    #[doc = "0x2a4 - SYS IOMUX CFG SAIF SYSCFG 676"]
    #[inline(always)]
    pub const fn func_sel_2(&self) -> &FUNC_SEL_2 {
        &self.func_sel_2
    }
    #[doc = "0x2a8 - SYS IOMUX CFG SAIF SYSCFG 680"]
    #[inline(always)]
    pub const fn func_sel_3(&self) -> &FUNC_SEL_3 {
        &self.func_sel_3
    }
    #[doc = "0x2ac - SYS IOMUX CFG SAIF SYSCFG 684"]
    #[inline(always)]
    pub const fn func_sel_4(&self) -> &FUNC_SEL_4 {
        &self.func_sel_4
    }
    #[doc = "0x2b0 - SYS IOMUX CFG SAIF SYSCFG 688"]
    #[inline(always)]
    pub const fn func_sel_5(&self) -> &FUNC_SEL_5 {
        &self.func_sel_5
    }
    #[doc = "0x2b4 - SYS IOMUX CFG SAIF SYSCFG 692"]
    #[inline(always)]
    pub const fn func_sel_6(&self) -> &FUNC_SEL_6 {
        &self.func_sel_6
    }
}
#[doc = "gpo_doen_0 (rw) register accessor: SYS IOMUX CFG SAIF SYSCFG FMUX 0 DOEN\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`gpo_doen_0::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`gpo_doen_0::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@gpo_doen_0`]
module"]
pub type GPO_DOEN_0 = crate::Reg<gpo_doen_0::GPO_DOEN_0_SPEC>;
#[doc = "SYS IOMUX CFG SAIF SYSCFG FMUX 0 DOEN"]
pub mod gpo_doen_0;
#[doc = "gpo_doen_1 (rw) register accessor: SYS IOMUX CFG SAIF SYSCFG FMUX 1 DOEN\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`gpo_doen_1::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`gpo_doen_1::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@gpo_doen_1`]
module"]
pub type GPO_DOEN_1 = crate::Reg<gpo_doen_1::GPO_DOEN_1_SPEC>;
#[doc = "SYS IOMUX CFG SAIF SYSCFG FMUX 1 DOEN"]
pub mod gpo_doen_1;
#[doc = "gpo_doen_2 (rw) register accessor: SYS IOMUX CFG SAIF SYSCFG FMUX 2 DOEN\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`gpo_doen_2::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`gpo_doen_2::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@gpo_doen_2`]
module"]
pub type GPO_DOEN_2 = crate::Reg<gpo_doen_2::GPO_DOEN_2_SPEC>;
#[doc = "SYS IOMUX CFG SAIF SYSCFG FMUX 2 DOEN"]
pub mod gpo_doen_2;
#[doc = "gpo_doen_3 (rw) register accessor: SYS IOMUX CFG SAIF SYSCFG FMUX 3 DOEN\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`gpo_doen_3::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`gpo_doen_3::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@gpo_doen_3`]
module"]
pub type GPO_DOEN_3 = crate::Reg<gpo_doen_3::GPO_DOEN_3_SPEC>;
#[doc = "SYS IOMUX CFG SAIF SYSCFG FMUX 3 DOEN"]
pub mod gpo_doen_3;
#[doc = "gpo_doen_4 (rw) register accessor: SYS IOMUX CFG SAIF SYSCFG FMUX 4 DOEN\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`gpo_doen_4::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`gpo_doen_4::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@gpo_doen_4`]
module"]
pub type GPO_DOEN_4 = crate::Reg<gpo_doen_4::GPO_DOEN_4_SPEC>;
#[doc = "SYS IOMUX CFG SAIF SYSCFG FMUX 4 DOEN"]
pub mod gpo_doen_4;
#[doc = "gpo_doen_5 (rw) register accessor: SYS IOMUX CFG SAIF SYSCFG FMUX 5 DOEN\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`gpo_doen_5::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`gpo_doen_5::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@gpo_doen_5`]
module"]
pub type GPO_DOEN_5 = crate::Reg<gpo_doen_5::GPO_DOEN_5_SPEC>;
#[doc = "SYS IOMUX CFG SAIF SYSCFG FMUX 5 DOEN"]
pub mod gpo_doen_5;
#[doc = "gpo_doen_6 (rw) register accessor: SYS IOMUX CFG SAIF SYSCFG FMUX 6 DOEN\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`gpo_doen_6::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`gpo_doen_6::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@gpo_doen_6`]
module"]
pub type GPO_DOEN_6 = crate::Reg<gpo_doen_6::GPO_DOEN_6_SPEC>;
#[doc = "SYS IOMUX CFG SAIF SYSCFG FMUX 6 DOEN"]
pub mod gpo_doen_6;
#[doc = "gpo_doen_7 (rw) register accessor: SYS IOMUX CFG SAIF SYSCFG FMUX 7 DOEN\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`gpo_doen_7::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`gpo_doen_7::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@gpo_doen_7`]
module"]
pub type GPO_DOEN_7 = crate::Reg<gpo_doen_7::GPO_DOEN_7_SPEC>;
#[doc = "SYS IOMUX CFG SAIF SYSCFG FMUX 7 DOEN"]
pub mod gpo_doen_7;
#[doc = "gpo_doen_8 (rw) register accessor: SYS IOMUX CFG SAIF SYSCFG FMUX 8 DOEN\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`gpo_doen_8::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`gpo_doen_8::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@gpo_doen_8`]
module"]
pub type GPO_DOEN_8 = crate::Reg<gpo_doen_8::GPO_DOEN_8_SPEC>;
#[doc = "SYS IOMUX CFG SAIF SYSCFG FMUX 8 DOEN"]
pub mod gpo_doen_8;
#[doc = "gpo_doen_9 (rw) register accessor: SYS IOMUX CFG SAIF SYSCFG FMUX 9 DOEN\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`gpo_doen_9::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`gpo_doen_9::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@gpo_doen_9`]
module"]
pub type GPO_DOEN_9 = crate::Reg<gpo_doen_9::GPO_DOEN_9_SPEC>;
#[doc = "SYS IOMUX CFG SAIF SYSCFG FMUX 9 DOEN"]
pub mod gpo_doen_9;
#[doc = "gpo_doen_10 (rw) register accessor: SYS IOMUX CFG SAIF SYSCFG FMUX 10 DOEN\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`gpo_doen_10::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`gpo_doen_10::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@gpo_doen_10`]
module"]
pub type GPO_DOEN_10 = crate::Reg<gpo_doen_10::GPO_DOEN_10_SPEC>;
#[doc = "SYS IOMUX CFG SAIF SYSCFG FMUX 10 DOEN"]
pub mod gpo_doen_10;
#[doc = "gpo_doen_11 (rw) register accessor: SYS IOMUX CFG SAIF SYSCFG FMUX 11 DOEN\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`gpo_doen_11::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`gpo_doen_11::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@gpo_doen_11`]
module"]
pub type GPO_DOEN_11 = crate::Reg<gpo_doen_11::GPO_DOEN_11_SPEC>;
#[doc = "SYS IOMUX CFG SAIF SYSCFG FMUX 11 DOEN"]
pub mod gpo_doen_11;
#[doc = "gpo_doen_12 (rw) register accessor: SYS IOMUX CFG SAIF SYSCFG FMUX 12 DOEN\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`gpo_doen_12::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`gpo_doen_12::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@gpo_doen_12`]
module"]
pub type GPO_DOEN_12 = crate::Reg<gpo_doen_12::GPO_DOEN_12_SPEC>;
#[doc = "SYS IOMUX CFG SAIF SYSCFG FMUX 12 DOEN"]
pub mod gpo_doen_12;
#[doc = "gpo_doen_13 (rw) register accessor: SYS IOMUX CFG SAIF SYSCFG FMUX 13 DOEN\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`gpo_doen_13::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`gpo_doen_13::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@gpo_doen_13`]
module"]
pub type GPO_DOEN_13 = crate::Reg<gpo_doen_13::GPO_DOEN_13_SPEC>;
#[doc = "SYS IOMUX CFG SAIF SYSCFG FMUX 13 DOEN"]
pub mod gpo_doen_13;
#[doc = "gpo_doen_14 (rw) register accessor: SYS IOMUX CFG SAIF SYSCFG FMUX 14 DOEN\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`gpo_doen_14::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`gpo_doen_14::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@gpo_doen_14`]
module"]
pub type GPO_DOEN_14 = crate::Reg<gpo_doen_14::GPO_DOEN_14_SPEC>;
#[doc = "SYS IOMUX CFG SAIF SYSCFG FMUX 14 DOEN"]
pub mod gpo_doen_14;
#[doc = "gpo_doen_15 (rw) register accessor: SYS IOMUX CFG SAIF SYSCFG FMUX 15 DOEN\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`gpo_doen_15::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`gpo_doen_15::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@gpo_doen_15`]
module"]
pub type GPO_DOEN_15 = crate::Reg<gpo_doen_15::GPO_DOEN_15_SPEC>;
#[doc = "SYS IOMUX CFG SAIF SYSCFG FMUX 15 DOEN"]
pub mod gpo_doen_15;
#[doc = "gpo_dout_0_3 (rw) register accessor: SYS IOMUX CFG SAIF SYSCFG FMUX GPIO 0-3 DOUT\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`gpo_dout_0_3::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`gpo_dout_0_3::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@gpo_dout_0_3`]
module"]
pub type GPO_DOUT_0_3 = crate::Reg<gpo_dout_0_3::GPO_DOUT_0_3_SPEC>;
#[doc = "SYS IOMUX CFG SAIF SYSCFG FMUX GPIO 0-3 DOUT"]
pub mod gpo_dout_0_3;
#[doc = "gpo_dout_4_7 (rw) register accessor: SYS IOMUX CFG SAIF SYSCFG FMUX GPIO 4-7 DOUT\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`gpo_dout_4_7::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`gpo_dout_4_7::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@gpo_dout_4_7`]
module"]
pub type GPO_DOUT_4_7 = crate::Reg<gpo_dout_4_7::GPO_DOUT_4_7_SPEC>;
#[doc = "SYS IOMUX CFG SAIF SYSCFG FMUX GPIO 4-7 DOUT"]
pub mod gpo_dout_4_7;
#[doc = "gpo_dout_8_11 (rw) register accessor: SYS IOMUX CFG SAIF SYSCFG FMUX GPIO 8-11 DOUT\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`gpo_dout_8_11::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`gpo_dout_8_11::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@gpo_dout_8_11`]
module"]
pub type GPO_DOUT_8_11 = crate::Reg<gpo_dout_8_11::GPO_DOUT_8_11_SPEC>;
#[doc = "SYS IOMUX CFG SAIF SYSCFG FMUX GPIO 8-11 DOUT"]
pub mod gpo_dout_8_11;
#[doc = "gpo_dout_12_15 (rw) register accessor: SYS IOMUX CFG SAIF SYSCFG FMUX GPIO 12-15 DOUT\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`gpo_dout_12_15::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`gpo_dout_12_15::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@gpo_dout_12_15`]
module"]
pub type GPO_DOUT_12_15 = crate::Reg<gpo_dout_12_15::GPO_DOUT_12_15_SPEC>;
#[doc = "SYS IOMUX CFG SAIF SYSCFG FMUX GPIO 12-15 DOUT"]
pub mod gpo_dout_12_15;
#[doc = "gpo_dout_16_19 (rw) register accessor: SYS IOMUX CFG SAIF SYSCFG FMUX GPIO 16-19 DOUT\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`gpo_dout_16_19::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`gpo_dout_16_19::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@gpo_dout_16_19`]
module"]
pub type GPO_DOUT_16_19 = crate::Reg<gpo_dout_16_19::GPO_DOUT_16_19_SPEC>;
#[doc = "SYS IOMUX CFG SAIF SYSCFG FMUX GPIO 16-19 DOUT"]
pub mod gpo_dout_16_19;
#[doc = "gpo_dout_20_23 (rw) register accessor: SYS IOMUX CFG SAIF SYSCFG FMUX GPIO 20-23 DOUT\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`gpo_dout_20_23::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`gpo_dout_20_23::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@gpo_dout_20_23`]
module"]
pub type GPO_DOUT_20_23 = crate::Reg<gpo_dout_20_23::GPO_DOUT_20_23_SPEC>;
#[doc = "SYS IOMUX CFG SAIF SYSCFG FMUX GPIO 20-23 DOUT"]
pub mod gpo_dout_20_23;
#[doc = "gpo_dout_24_27 (rw) register accessor: SYS IOMUX CFG SAIF SYSCFG FMUX GPIO 24-27 DOUT\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`gpo_dout_24_27::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`gpo_dout_24_27::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@gpo_dout_24_27`]
module"]
pub type GPO_DOUT_24_27 = crate::Reg<gpo_dout_24_27::GPO_DOUT_24_27_SPEC>;
#[doc = "SYS IOMUX CFG SAIF SYSCFG FMUX GPIO 24-27 DOUT"]
pub mod gpo_dout_24_27;
#[doc = "gpo_dout_28_31 (rw) register accessor: SYS IOMUX CFG SAIF SYSCFG FMUX GPIO 28-31 DOUT\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`gpo_dout_28_31::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`gpo_dout_28_31::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@gpo_dout_28_31`]
module"]
pub type GPO_DOUT_28_31 = crate::Reg<gpo_dout_28_31::GPO_DOUT_28_31_SPEC>;
#[doc = "SYS IOMUX CFG SAIF SYSCFG FMUX GPIO 28-31 DOUT"]
pub mod gpo_dout_28_31;
#[doc = "gpo_dout_32_35 (rw) register accessor: SYS IOMUX CFG SAIF SYSCFG FMUX GPIO 32-35 DOUT\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`gpo_dout_32_35::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`gpo_dout_32_35::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@gpo_dout_32_35`]
module"]
pub type GPO_DOUT_32_35 = crate::Reg<gpo_dout_32_35::GPO_DOUT_32_35_SPEC>;
#[doc = "SYS IOMUX CFG SAIF SYSCFG FMUX GPIO 32-35 DOUT"]
pub mod gpo_dout_32_35;
#[doc = "gpo_dout_36_39 (rw) register accessor: SYS IOMUX CFG SAIF SYSCFG FMUX GPIO 36-39 DOUT\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`gpo_dout_36_39::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`gpo_dout_36_39::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@gpo_dout_36_39`]
module"]
pub type GPO_DOUT_36_39 = crate::Reg<gpo_dout_36_39::GPO_DOUT_36_39_SPEC>;
#[doc = "SYS IOMUX CFG SAIF SYSCFG FMUX GPIO 36-39 DOUT"]
pub mod gpo_dout_36_39;
#[doc = "gpo_dout_40_43 (rw) register accessor: SYS IOMUX CFG SAIF SYSCFG FMUX GPIO 40-43 DOUT\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`gpo_dout_40_43::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`gpo_dout_40_43::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@gpo_dout_40_43`]
module"]
pub type GPO_DOUT_40_43 = crate::Reg<gpo_dout_40_43::GPO_DOUT_40_43_SPEC>;
#[doc = "SYS IOMUX CFG SAIF SYSCFG FMUX GPIO 40-43 DOUT"]
pub mod gpo_dout_40_43;
#[doc = "gpo_dout_44_47 (rw) register accessor: SYS IOMUX CFG SAIF SYSCFG FMUX GPIO 44-47 DOUT\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`gpo_dout_44_47::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`gpo_dout_44_47::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@gpo_dout_44_47`]
module"]
pub type GPO_DOUT_44_47 = crate::Reg<gpo_dout_44_47::GPO_DOUT_44_47_SPEC>;
#[doc = "SYS IOMUX CFG SAIF SYSCFG FMUX GPIO 44-47 DOUT"]
pub mod gpo_dout_44_47;
#[doc = "gpo_dout_48_51 (rw) register accessor: SYS IOMUX CFG SAIF SYSCFG FMUX GPIO 48-51 DOUT\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`gpo_dout_48_51::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`gpo_dout_48_51::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@gpo_dout_48_51`]
module"]
pub type GPO_DOUT_48_51 = crate::Reg<gpo_dout_48_51::GPO_DOUT_48_51_SPEC>;
#[doc = "SYS IOMUX CFG SAIF SYSCFG FMUX GPIO 48-51 DOUT"]
pub mod gpo_dout_48_51;
#[doc = "gpo_dout_52_55 (rw) register accessor: SYS IOMUX CFG SAIF SYSCFG FMUX GPIO 52-55 DOUT\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`gpo_dout_52_55::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`gpo_dout_52_55::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@gpo_dout_52_55`]
module"]
pub type GPO_DOUT_52_55 = crate::Reg<gpo_dout_52_55::GPO_DOUT_52_55_SPEC>;
#[doc = "SYS IOMUX CFG SAIF SYSCFG FMUX GPIO 52-55 DOUT"]
pub mod gpo_dout_52_55;
#[doc = "gpo_dout_56_59 (rw) register accessor: SYS IOMUX CFG SAIF SYSCFG FMUX GPIO 56-59 DOUT\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`gpo_dout_56_59::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`gpo_dout_56_59::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@gpo_dout_56_59`]
module"]
pub type GPO_DOUT_56_59 = crate::Reg<gpo_dout_56_59::GPO_DOUT_56_59_SPEC>;
#[doc = "SYS IOMUX CFG SAIF SYSCFG FMUX GPIO 56-59 DOUT"]
pub mod gpo_dout_56_59;
#[doc = "gpo_dout_60_63 (rw) register accessor: SYS IOMUX CFG SAIF SYSCFG FMUX GPIO 60-63 DOUT\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`gpo_dout_60_63::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`gpo_dout_60_63::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@gpo_dout_60_63`]
module"]
pub type GPO_DOUT_60_63 = crate::Reg<gpo_dout_60_63::GPO_DOUT_60_63_SPEC>;
#[doc = "SYS IOMUX CFG SAIF SYSCFG FMUX GPIO 60-63 DOUT"]
pub mod gpo_dout_60_63;
#[doc = "gpi_0 (rw) register accessor: SYS IOMUX CFG SAIF SYSCFG FMUX GPIO GPI 0 - The register can be used to configure the selected GPIO connector number for input signals. The signal name is indicated in the \"Name\" column of the following table per StarFive naming conventions. For example, name \"u0_WAVE511_i_uart_rxsin_cfg\" indicates the corresponding input signal is \"u0_WAVE511.i_uart_rxsin\". See GPIO Input Signals (on page 107) for a complete list of the input GPIO signals.\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`gpi_0::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`gpi_0::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@gpi_0`]
module"]
pub type GPI_0 = crate::Reg<gpi_0::GPI_0_SPEC>;
#[doc = "SYS IOMUX CFG SAIF SYSCFG FMUX GPIO GPI 0 - The register can be used to configure the selected GPIO connector number for input signals. The signal name is indicated in the \"Name\" column of the following table per StarFive naming conventions. For example, name \"u0_WAVE511_i_uart_rxsin_cfg\" indicates the corresponding input signal is \"u0_WAVE511.i_uart_rxsin\". See GPIO Input Signals (on page 107) for a complete list of the input GPIO signals."]
pub mod gpi_0;
#[doc = "gpi_1 (rw) register accessor: SYS IOMUX CFG SAIF SYSCFG FMUX GPIO GPI 4 - The register can be used to configure the selected GPIO connector number for input signals. The signal name is indicated in the \"Name\" column of the following table per StarFive naming conventions. For example, name \"u0_WAVE511_i_uart_rxsin_cfg\" indicates the corresponding input signal is \"u0_WAVE511.i_uart_rxsin\". See GPIO Input Signals (on page 107) for a complete list of the input GPIO signals.\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`gpi_1::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`gpi_1::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@gpi_1`]
module"]
pub type GPI_1 = crate::Reg<gpi_1::GPI_1_SPEC>;
#[doc = "SYS IOMUX CFG SAIF SYSCFG FMUX GPIO GPI 4 - The register can be used to configure the selected GPIO connector number for input signals. The signal name is indicated in the \"Name\" column of the following table per StarFive naming conventions. For example, name \"u0_WAVE511_i_uart_rxsin_cfg\" indicates the corresponding input signal is \"u0_WAVE511.i_uart_rxsin\". See GPIO Input Signals (on page 107) for a complete list of the input GPIO signals."]
pub mod gpi_1;
#[doc = "gpi_2 (rw) register accessor: SYS IOMUX CFG SAIF SYSCFG FMUX GPIO GPI 8 - The register can be used to configure the selected GPIO connector number for input signals. The signal name is indicated in the \"Name\" column of the following table per StarFive naming conventions. For example, name \"u0_WAVE511_i_uart_rxsin_cfg\" indicates the corresponding input signal is \"u0_WAVE511.i_uart_rxsin\". See GPIO Input Signals (on page 107) for a complete list of the input GPIO signals.\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`gpi_2::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`gpi_2::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@gpi_2`]
module"]
pub type GPI_2 = crate::Reg<gpi_2::GPI_2_SPEC>;
#[doc = "SYS IOMUX CFG SAIF SYSCFG FMUX GPIO GPI 8 - The register can be used to configure the selected GPIO connector number for input signals. The signal name is indicated in the \"Name\" column of the following table per StarFive naming conventions. For example, name \"u0_WAVE511_i_uart_rxsin_cfg\" indicates the corresponding input signal is \"u0_WAVE511.i_uart_rxsin\". See GPIO Input Signals (on page 107) for a complete list of the input GPIO signals."]
pub mod gpi_2;
#[doc = "gpi_3 (rw) register accessor: SYS IOMUX CFG SAIF SYSCFG FMUX GPIO GPI 12 - The register can be used to configure the selected GPIO connector number for input signals. The signal name is indicated in the \"Name\" column of the following table per StarFive naming conventions. For example, name \"u0_WAVE511_i_uart_rxsin_cfg\" indicates the corresponding input signal is \"u0_WAVE511.i_uart_rxsin\". See GPIO Input Signals (on page 107) for a complete list of the input GPIO signals.\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`gpi_3::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`gpi_3::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@gpi_3`]
module"]
pub type GPI_3 = crate::Reg<gpi_3::GPI_3_SPEC>;
#[doc = "SYS IOMUX CFG SAIF SYSCFG FMUX GPIO GPI 12 - The register can be used to configure the selected GPIO connector number for input signals. The signal name is indicated in the \"Name\" column of the following table per StarFive naming conventions. For example, name \"u0_WAVE511_i_uart_rxsin_cfg\" indicates the corresponding input signal is \"u0_WAVE511.i_uart_rxsin\". See GPIO Input Signals (on page 107) for a complete list of the input GPIO signals."]
pub mod gpi_3;
#[doc = "gpi_4 (rw) register accessor: SYS IOMUX CFG SAIF SYSCFG FMUX GPIO GPI 16 - The register can be used to configure the selected GPIO connector number for input signals. The signal name is indicated in the \"Name\" column of the following table per StarFive naming conventions. For example, name \"u0_WAVE511_i_uart_rxsin_cfg\" indicates the corresponding input signal is \"u0_WAVE511.i_uart_rxsin\". See GPIO Input Signals (on page 107) for a complete list of the input GPIO signals.\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`gpi_4::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`gpi_4::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@gpi_4`]
module"]
pub type GPI_4 = crate::Reg<gpi_4::GPI_4_SPEC>;
#[doc = "SYS IOMUX CFG SAIF SYSCFG FMUX GPIO GPI 16 - The register can be used to configure the selected GPIO connector number for input signals. The signal name is indicated in the \"Name\" column of the following table per StarFive naming conventions. For example, name \"u0_WAVE511_i_uart_rxsin_cfg\" indicates the corresponding input signal is \"u0_WAVE511.i_uart_rxsin\". See GPIO Input Signals (on page 107) for a complete list of the input GPIO signals."]
pub mod gpi_4;
#[doc = "gpi_5 (rw) register accessor: SYS IOMUX CFG SAIF SYSCFG FMUX GPIO GPI 20 - The register can be used to configure the selected GPIO connector number for input signals. The signal name is indicated in the \"Name\" column of the following table per StarFive naming conventions. For example, name \"u0_WAVE511_i_uart_rxsin_cfg\" indicates the corresponding input signal is \"u0_WAVE511.i_uart_rxsin\". See GPIO Input Signals (on page 107) for a complete list of the input GPIO signals.\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`gpi_5::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`gpi_5::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@gpi_5`]
module"]
pub type GPI_5 = crate::Reg<gpi_5::GPI_5_SPEC>;
#[doc = "SYS IOMUX CFG SAIF SYSCFG FMUX GPIO GPI 20 - The register can be used to configure the selected GPIO connector number for input signals. The signal name is indicated in the \"Name\" column of the following table per StarFive naming conventions. For example, name \"u0_WAVE511_i_uart_rxsin_cfg\" indicates the corresponding input signal is \"u0_WAVE511.i_uart_rxsin\". See GPIO Input Signals (on page 107) for a complete list of the input GPIO signals."]
pub mod gpi_5;
#[doc = "gpi_6 (rw) register accessor: SYS IOMUX CFG SAIF SYSCFG FMUX GPIO GPI 24 - The register can be used to configure the selected GPIO connector number for input signals. The signal name is indicated in the \"Name\" column of the following table per StarFive naming conventions. For example, name \"u0_WAVE511_i_uart_rxsin_cfg\" indicates the corresponding input signal is \"u0_WAVE511.i_uart_rxsin\". See GPIO Input Signals (on page 107) for a complete list of the input GPIO signals.\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`gpi_6::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`gpi_6::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@gpi_6`]
module"]
pub type GPI_6 = crate::Reg<gpi_6::GPI_6_SPEC>;
#[doc = "SYS IOMUX CFG SAIF SYSCFG FMUX GPIO GPI 24 - The register can be used to configure the selected GPIO connector number for input signals. The signal name is indicated in the \"Name\" column of the following table per StarFive naming conventions. For example, name \"u0_WAVE511_i_uart_rxsin_cfg\" indicates the corresponding input signal is \"u0_WAVE511.i_uart_rxsin\". See GPIO Input Signals (on page 107) for a complete list of the input GPIO signals."]
pub mod gpi_6;
#[doc = "gpi_7 (rw) register accessor: SYS IOMUX CFG SAIF SYSCFG FMUX GPIO GPI 28 - The register can be used to configure the selected GPIO connector number for input signals. The signal name is indicated in the \"Name\" column of the following table per StarFive naming conventions. For example, name \"u0_WAVE511_i_uart_rxsin_cfg\" indicates the corresponding input signal is \"u0_WAVE511.i_uart_rxsin\". See GPIO Input Signals (on page 107) for a complete list of the input GPIO signals.\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`gpi_7::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`gpi_7::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@gpi_7`]
module"]
pub type GPI_7 = crate::Reg<gpi_7::GPI_7_SPEC>;
#[doc = "SYS IOMUX CFG SAIF SYSCFG FMUX GPIO GPI 28 - The register can be used to configure the selected GPIO connector number for input signals. The signal name is indicated in the \"Name\" column of the following table per StarFive naming conventions. For example, name \"u0_WAVE511_i_uart_rxsin_cfg\" indicates the corresponding input signal is \"u0_WAVE511.i_uart_rxsin\". See GPIO Input Signals (on page 107) for a complete list of the input GPIO signals."]
pub mod gpi_7;
#[doc = "gpi_8 (rw) register accessor: SYS IOMUX CFG SAIF SYSCFG FMUX GPIO GPI 32 - The register can be used to configure the selected GPIO connector number for input signals. The signal name is indicated in the \"Name\" column of the following table per StarFive naming conventions. For example, name \"u0_WAVE511_i_uart_rxsin_cfg\" indicates the corresponding input signal is \"u0_WAVE511.i_uart_rxsin\". See GPIO Input Signals (on page 107) for a complete list of the input GPIO signals.\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`gpi_8::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`gpi_8::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@gpi_8`]
module"]
pub type GPI_8 = crate::Reg<gpi_8::GPI_8_SPEC>;
#[doc = "SYS IOMUX CFG SAIF SYSCFG FMUX GPIO GPI 32 - The register can be used to configure the selected GPIO connector number for input signals. The signal name is indicated in the \"Name\" column of the following table per StarFive naming conventions. For example, name \"u0_WAVE511_i_uart_rxsin_cfg\" indicates the corresponding input signal is \"u0_WAVE511.i_uart_rxsin\". See GPIO Input Signals (on page 107) for a complete list of the input GPIO signals."]
pub mod gpi_8;
#[doc = "gpi_9 (rw) register accessor: SYS IOMUX CFG SAIF SYSCFG FMUX GPIO GPI 36 - The register can be used to configure the selected GPIO connector number for input signals. The signal name is indicated in the \"Name\" column of the following table per StarFive naming conventions. For example, name \"u0_WAVE511_i_uart_rxsin_cfg\" indicates the corresponding input signal is \"u0_WAVE511.i_uart_rxsin\". See GPIO Input Signals (on page 107) for a complete list of the input GPIO signals.\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`gpi_9::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`gpi_9::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@gpi_9`]
module"]
pub type GPI_9 = crate::Reg<gpi_9::GPI_9_SPEC>;
#[doc = "SYS IOMUX CFG SAIF SYSCFG FMUX GPIO GPI 36 - The register can be used to configure the selected GPIO connector number for input signals. The signal name is indicated in the \"Name\" column of the following table per StarFive naming conventions. For example, name \"u0_WAVE511_i_uart_rxsin_cfg\" indicates the corresponding input signal is \"u0_WAVE511.i_uart_rxsin\". See GPIO Input Signals (on page 107) for a complete list of the input GPIO signals."]
pub mod gpi_9;
#[doc = "gpi_10 (rw) register accessor: SYS IOMUX CFG SAIF SYSCFG FMUX GPIO GPI 40 - The register can be used to configure the selected GPIO connector number for input signals. The signal name is indicated in the \"Name\" column of the following table per StarFive naming conventions. For example, name \"u0_WAVE511_i_uart_rxsin_cfg\" indicates the corresponding input signal is \"u0_WAVE511.i_uart_rxsin\". See GPIO Input Signals (on page 107) for a complete list of the input GPIO signals.\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`gpi_10::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`gpi_10::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@gpi_10`]
module"]
pub type GPI_10 = crate::Reg<gpi_10::GPI_10_SPEC>;
#[doc = "SYS IOMUX CFG SAIF SYSCFG FMUX GPIO GPI 40 - The register can be used to configure the selected GPIO connector number for input signals. The signal name is indicated in the \"Name\" column of the following table per StarFive naming conventions. For example, name \"u0_WAVE511_i_uart_rxsin_cfg\" indicates the corresponding input signal is \"u0_WAVE511.i_uart_rxsin\". See GPIO Input Signals (on page 107) for a complete list of the input GPIO signals."]
pub mod gpi_10;
#[doc = "gpi_11 (rw) register accessor: SYS IOMUX CFG SAIF SYSCFG FMUX GPIO GPI 44 - The register can be used to configure the selected GPIO connector number for input signals. The signal name is indicated in the \"Name\" column of the following table per StarFive naming conventions. For example, name \"u0_WAVE511_i_uart_rxsin_cfg\" indicates the corresponding input signal is \"u0_WAVE511.i_uart_rxsin\". See GPIO Input Signals (on page 107) for a complete list of the input GPIO signals.\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`gpi_11::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`gpi_11::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@gpi_11`]
module"]
pub type GPI_11 = crate::Reg<gpi_11::GPI_11_SPEC>;
#[doc = "SYS IOMUX CFG SAIF SYSCFG FMUX GPIO GPI 44 - The register can be used to configure the selected GPIO connector number for input signals. The signal name is indicated in the \"Name\" column of the following table per StarFive naming conventions. For example, name \"u0_WAVE511_i_uart_rxsin_cfg\" indicates the corresponding input signal is \"u0_WAVE511.i_uart_rxsin\". See GPIO Input Signals (on page 107) for a complete list of the input GPIO signals."]
pub mod gpi_11;
#[doc = "gpi_12 (rw) register accessor: SYS IOMUX CFG SAIF SYSCFG FMUX GPIO GPI 48 - The register can be used to configure the selected GPIO connector number for input signals. The signal name is indicated in the \"Name\" column of the following table per StarFive naming conventions. For example, name \"u0_WAVE511_i_uart_rxsin_cfg\" indicates the corresponding input signal is \"u0_WAVE511.i_uart_rxsin\". See GPIO Input Signals (on page 107) for a complete list of the input GPIO signals.\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`gpi_12::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`gpi_12::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@gpi_12`]
module"]
pub type GPI_12 = crate::Reg<gpi_12::GPI_12_SPEC>;
#[doc = "SYS IOMUX CFG SAIF SYSCFG FMUX GPIO GPI 48 - The register can be used to configure the selected GPIO connector number for input signals. The signal name is indicated in the \"Name\" column of the following table per StarFive naming conventions. For example, name \"u0_WAVE511_i_uart_rxsin_cfg\" indicates the corresponding input signal is \"u0_WAVE511.i_uart_rxsin\". See GPIO Input Signals (on page 107) for a complete list of the input GPIO signals."]
pub mod gpi_12;
#[doc = "gpi_13 (rw) register accessor: SYS IOMUX CFG SAIF SYSCFG FMUX GPIO GPI 52 - The register can be used to configure the selected GPIO connector number for input signals. The signal name is indicated in the \"Name\" column of the following table per StarFive naming conventions. For example, name \"u0_WAVE511_i_uart_rxsin_cfg\" indicates the corresponding input signal is \"u0_WAVE511.i_uart_rxsin\". See GPIO Input Signals (on page 107) for a complete list of the input GPIO signals.\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`gpi_13::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`gpi_13::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@gpi_13`]
module"]
pub type GPI_13 = crate::Reg<gpi_13::GPI_13_SPEC>;
#[doc = "SYS IOMUX CFG SAIF SYSCFG FMUX GPIO GPI 52 - The register can be used to configure the selected GPIO connector number for input signals. The signal name is indicated in the \"Name\" column of the following table per StarFive naming conventions. For example, name \"u0_WAVE511_i_uart_rxsin_cfg\" indicates the corresponding input signal is \"u0_WAVE511.i_uart_rxsin\". See GPIO Input Signals (on page 107) for a complete list of the input GPIO signals."]
pub mod gpi_13;
#[doc = "gpi_14 (rw) register accessor: SYS IOMUX CFG SAIF SYSCFG FMUX GPIO GPI 56 - The register can be used to configure the selected GPIO connector number for input signals. The signal name is indicated in the \"Name\" column of the following table per StarFive naming conventions. For example, name \"u0_WAVE511_i_uart_rxsin_cfg\" indicates the corresponding input signal is \"u0_WAVE511.i_uart_rxsin\". See GPIO Input Signals (on page 107) for a complete list of the input GPIO signals.\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`gpi_14::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`gpi_14::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@gpi_14`]
module"]
pub type GPI_14 = crate::Reg<gpi_14::GPI_14_SPEC>;
#[doc = "SYS IOMUX CFG SAIF SYSCFG FMUX GPIO GPI 56 - The register can be used to configure the selected GPIO connector number for input signals. The signal name is indicated in the \"Name\" column of the following table per StarFive naming conventions. For example, name \"u0_WAVE511_i_uart_rxsin_cfg\" indicates the corresponding input signal is \"u0_WAVE511.i_uart_rxsin\". See GPIO Input Signals (on page 107) for a complete list of the input GPIO signals."]
pub mod gpi_14;
#[doc = "gpi_15 (rw) register accessor: SYS IOMUX CFG SAIF SYSCFG FMUX GPIO GPI 60 - The register can be used to configure the selected GPIO connector number for input signals. The signal name is indicated in the \"Name\" column of the following table per StarFive naming conventions. For example, name \"u0_WAVE511_i_uart_rxsin_cfg\" indicates the corresponding input signal is \"u0_WAVE511.i_uart_rxsin\". See GPIO Input Signals (on page 107) for a complete list of the input GPIO signals.\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`gpi_15::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`gpi_15::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@gpi_15`]
module"]
pub type GPI_15 = crate::Reg<gpi_15::GPI_15_SPEC>;
#[doc = "SYS IOMUX CFG SAIF SYSCFG FMUX GPIO GPI 60 - The register can be used to configure the selected GPIO connector number for input signals. The signal name is indicated in the \"Name\" column of the following table per StarFive naming conventions. For example, name \"u0_WAVE511_i_uart_rxsin_cfg\" indicates the corresponding input signal is \"u0_WAVE511.i_uart_rxsin\". See GPIO Input Signals (on page 107) for a complete list of the input GPIO signals."]
pub mod gpi_15;
#[doc = "gpi_16 (rw) register accessor: SYS IOMUX CFG SAIF SYSCFG FMUX GPIO GPI 64 - The register can be used to configure the selected GPIO connector number for input signals. The signal name is indicated in the \"Name\" column of the following table per StarFive naming conventions. For example, name \"u0_WAVE511_i_uart_rxsin_cfg\" indicates the corresponding input signal is \"u0_WAVE511.i_uart_rxsin\". See GPIO Input Signals (on page 107) for a complete list of the input GPIO signals.\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`gpi_16::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`gpi_16::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@gpi_16`]
module"]
pub type GPI_16 = crate::Reg<gpi_16::GPI_16_SPEC>;
#[doc = "SYS IOMUX CFG SAIF SYSCFG FMUX GPIO GPI 64 - The register can be used to configure the selected GPIO connector number for input signals. The signal name is indicated in the \"Name\" column of the following table per StarFive naming conventions. For example, name \"u0_WAVE511_i_uart_rxsin_cfg\" indicates the corresponding input signal is \"u0_WAVE511.i_uart_rxsin\". See GPIO Input Signals (on page 107) for a complete list of the input GPIO signals."]
pub mod gpi_16;
#[doc = "gpi_17 (rw) register accessor: SYS IOMUX CFG SAIF SYSCFG FMUX GPIO GPI 68 - The register can be used to configure the selected GPIO connector number for input signals. The signal name is indicated in the \"Name\" column of the following table per StarFive naming conventions. For example, name \"u0_WAVE511_i_uart_rxsin_cfg\" indicates the corresponding input signal is \"u0_WAVE511.i_uart_rxsin\". See GPIO Input Signals (on page 107) for a complete list of the input GPIO signals.\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`gpi_17::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`gpi_17::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@gpi_17`]
module"]
pub type GPI_17 = crate::Reg<gpi_17::GPI_17_SPEC>;
#[doc = "SYS IOMUX CFG SAIF SYSCFG FMUX GPIO GPI 68 - The register can be used to configure the selected GPIO connector number for input signals. The signal name is indicated in the \"Name\" column of the following table per StarFive naming conventions. For example, name \"u0_WAVE511_i_uart_rxsin_cfg\" indicates the corresponding input signal is \"u0_WAVE511.i_uart_rxsin\". See GPIO Input Signals (on page 107) for a complete list of the input GPIO signals."]
pub mod gpi_17;
#[doc = "gpi_18 (rw) register accessor: SYS IOMUX CFG SAIF SYSCFG FMUX GPIO GPI 72 - The register can be used to configure the selected GPIO connector number for input signals. The signal name is indicated in the \"Name\" column of the following table per StarFive naming conventions. For example, name \"u0_WAVE511_i_uart_rxsin_cfg\" indicates the corresponding input signal is \"u0_WAVE511.i_uart_rxsin\". See GPIO Input Signals (on page 107) for a complete list of the input GPIO signals.\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`gpi_18::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`gpi_18::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@gpi_18`]
module"]
pub type GPI_18 = crate::Reg<gpi_18::GPI_18_SPEC>;
#[doc = "SYS IOMUX CFG SAIF SYSCFG FMUX GPIO GPI 72 - The register can be used to configure the selected GPIO connector number for input signals. The signal name is indicated in the \"Name\" column of the following table per StarFive naming conventions. For example, name \"u0_WAVE511_i_uart_rxsin_cfg\" indicates the corresponding input signal is \"u0_WAVE511.i_uart_rxsin\". See GPIO Input Signals (on page 107) for a complete list of the input GPIO signals."]
pub mod gpi_18;
#[doc = "gpi_19 (rw) register accessor: SYS IOMUX CFG SAIF SYSCFG FMUX GPIO GPI 76 - The register can be used to configure the selected GPIO connector number for input signals. The signal name is indicated in the \"Name\" column of the following table per StarFive naming conventions. For example, name \"u0_WAVE511_i_uart_rxsin_cfg\" indicates the corresponding input signal is \"u0_WAVE511.i_uart_rxsin\". See GPIO Input Signals (on page 107) for a complete list of the input GPIO signals.\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`gpi_19::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`gpi_19::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@gpi_19`]
module"]
pub type GPI_19 = crate::Reg<gpi_19::GPI_19_SPEC>;
#[doc = "SYS IOMUX CFG SAIF SYSCFG FMUX GPIO GPI 76 - The register can be used to configure the selected GPIO connector number for input signals. The signal name is indicated in the \"Name\" column of the following table per StarFive naming conventions. For example, name \"u0_WAVE511_i_uart_rxsin_cfg\" indicates the corresponding input signal is \"u0_WAVE511.i_uart_rxsin\". See GPIO Input Signals (on page 107) for a complete list of the input GPIO signals."]
pub mod gpi_19;
#[doc = "gpi_20 (rw) register accessor: SYS IOMUX CFG SAIF SYSCFG FMUX GPIO GPI 80 - The register can be used to configure the selected GPIO connector number for input signals. The signal name is indicated in the \"Name\" column of the following table per StarFive naming conventions. For example, name \"u0_WAVE511_i_uart_rxsin_cfg\" indicates the corresponding input signal is \"u0_WAVE511.i_uart_rxsin\". See GPIO Input Signals (on page 107) for a complete list of the input GPIO signals.\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`gpi_20::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`gpi_20::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@gpi_20`]
module"]
pub type GPI_20 = crate::Reg<gpi_20::GPI_20_SPEC>;
#[doc = "SYS IOMUX CFG SAIF SYSCFG FMUX GPIO GPI 80 - The register can be used to configure the selected GPIO connector number for input signals. The signal name is indicated in the \"Name\" column of the following table per StarFive naming conventions. For example, name \"u0_WAVE511_i_uart_rxsin_cfg\" indicates the corresponding input signal is \"u0_WAVE511.i_uart_rxsin\". See GPIO Input Signals (on page 107) for a complete list of the input GPIO signals."]
pub mod gpi_20;
#[doc = "gpi_21 (rw) register accessor: SYS IOMUX CFG SAIF SYSCFG FMUX GPIO GPI 84 - The register can be used to configure the selected GPIO connector number for input signals. The signal name is indicated in the \"Name\" column of the following table per StarFive naming conventions. For example, name \"u0_WAVE511_i_uart_rxsin_cfg\" indicates the corresponding input signal is \"u0_WAVE511.i_uart_rxsin\". See GPIO Input Signals (on page 107) for a complete list of the input GPIO signals.\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`gpi_21::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`gpi_21::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@gpi_21`]
module"]
pub type GPI_21 = crate::Reg<gpi_21::GPI_21_SPEC>;
#[doc = "SYS IOMUX CFG SAIF SYSCFG FMUX GPIO GPI 84 - The register can be used to configure the selected GPIO connector number for input signals. The signal name is indicated in the \"Name\" column of the following table per StarFive naming conventions. For example, name \"u0_WAVE511_i_uart_rxsin_cfg\" indicates the corresponding input signal is \"u0_WAVE511.i_uart_rxsin\". See GPIO Input Signals (on page 107) for a complete list of the input GPIO signals."]
pub mod gpi_21;
#[doc = "gpi_22 (rw) register accessor: SYS IOMUX CFG SAIF SYSCFG FMUX GPIO GPI 88 - The register can be used to configure the selected GPIO connector number for input signals. The signal name is indicated in the \"Name\" column of the following table per StarFive naming conventions. For example, name \"u0_WAVE511_i_uart_rxsin_cfg\" indicates the corresponding input signal is \"u0_WAVE511.i_uart_rxsin\". See GPIO Input Signals (on page 107) for a complete list of the input GPIO signals.\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`gpi_22::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`gpi_22::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@gpi_22`]
module"]
pub type GPI_22 = crate::Reg<gpi_22::GPI_22_SPEC>;
#[doc = "SYS IOMUX CFG SAIF SYSCFG FMUX GPIO GPI 88 - The register can be used to configure the selected GPIO connector number for input signals. The signal name is indicated in the \"Name\" column of the following table per StarFive naming conventions. For example, name \"u0_WAVE511_i_uart_rxsin_cfg\" indicates the corresponding input signal is \"u0_WAVE511.i_uart_rxsin\". See GPIO Input Signals (on page 107) for a complete list of the input GPIO signals."]
pub mod gpi_22;
#[doc = "ioirq_0 (rw) register accessor: Enable GPIO IRQ function\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`ioirq_0::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`ioirq_0::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@ioirq_0`]
module"]
pub type IOIRQ_0 = crate::Reg<ioirq_0::IOIRQ_0_SPEC>;
#[doc = "Enable GPIO IRQ function"]
pub mod ioirq_0;
#[doc = "ioirq_1 (rw) register accessor: SYS IOMUX CFGSAIF SYSCFG IOIRQ 224: GPIO Interrupt Edge Trigger Selector\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`ioirq_1::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`ioirq_1::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@ioirq_1`]
module"]
pub type IOIRQ_1 = crate::Reg<ioirq_1::IOIRQ_1_SPEC>;
#[doc = "SYS IOMUX CFGSAIF SYSCFG IOIRQ 224: GPIO Interrupt Edge Trigger Selector"]
pub mod ioirq_1;
#[doc = "ioirq_2 (rw) register accessor: SYS IOMUX CFGSAIF SYSCFG IOIRQ 228: GPIO Interrupt Edge Trigger Selector\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`ioirq_2::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`ioirq_2::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@ioirq_2`]
module"]
pub type IOIRQ_2 = crate::Reg<ioirq_2::IOIRQ_2_SPEC>;
#[doc = "SYS IOMUX CFGSAIF SYSCFG IOIRQ 228: GPIO Interrupt Edge Trigger Selector"]
pub mod ioirq_2;
#[doc = "ioirq_3 (rw) register accessor: SYS IOMUX CFGSAIF SYSCFG IOIRQ 232: GPIO Interrupt Clear\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`ioirq_3::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`ioirq_3::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@ioirq_3`]
module"]
pub type IOIRQ_3 = crate::Reg<ioirq_3::IOIRQ_3_SPEC>;
#[doc = "SYS IOMUX CFGSAIF SYSCFG IOIRQ 232: GPIO Interrupt Clear"]
pub mod ioirq_3;
#[doc = "ioirq_4 (rw) register accessor: SYS IOMUX CFGSAIF SYSCFG IOIRQ 236: GPIO Interrupt Clear\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`ioirq_4::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`ioirq_4::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@ioirq_4`]
module"]
pub type IOIRQ_4 = crate::Reg<ioirq_4::IOIRQ_4_SPEC>;
#[doc = "SYS IOMUX CFGSAIF SYSCFG IOIRQ 236: GPIO Interrupt Clear"]
pub mod ioirq_4;
#[doc = "ioirq_5 (rw) register accessor: SYS IOMUX CFGSAIF SYSCFG IOIRQ 240: GPIO Interrupt Both Edge Trigger Selector\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`ioirq_5::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`ioirq_5::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@ioirq_5`]
module"]
pub type IOIRQ_5 = crate::Reg<ioirq_5::IOIRQ_5_SPEC>;
#[doc = "SYS IOMUX CFGSAIF SYSCFG IOIRQ 240: GPIO Interrupt Both Edge Trigger Selector"]
pub mod ioirq_5;
#[doc = "ioirq_6 (rw) register accessor: SYS IOMUX CFGSAIF SYSCFG IOIRQ 244: GPIO Interrupt Both Edge Trigger Selector\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`ioirq_6::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`ioirq_6::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@ioirq_6`]
module"]
pub type IOIRQ_6 = crate::Reg<ioirq_6::IOIRQ_6_SPEC>;
#[doc = "SYS IOMUX CFGSAIF SYSCFG IOIRQ 244: GPIO Interrupt Both Edge Trigger Selector"]
pub mod ioirq_6;
#[doc = "ioirq_7 (rw) register accessor: SYS IOMUX CFGSAIF SYSCFG IOIRQ 248: GPIO Interrupt Edge Value\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`ioirq_7::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`ioirq_7::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@ioirq_7`]
module"]
pub type IOIRQ_7 = crate::Reg<ioirq_7::IOIRQ_7_SPEC>;
#[doc = "SYS IOMUX CFGSAIF SYSCFG IOIRQ 248: GPIO Interrupt Edge Value"]
pub mod ioirq_7;
#[doc = "ioirq_8 (rw) register accessor: SYS IOMUX CFGSAIF SYSCFG IOIRQ 252: GPIO Interrupt Edge Value\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`ioirq_8::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`ioirq_8::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@ioirq_8`]
module"]
pub type IOIRQ_8 = crate::Reg<ioirq_8::IOIRQ_8_SPEC>;
#[doc = "SYS IOMUX CFGSAIF SYSCFG IOIRQ 252: GPIO Interrupt Edge Value"]
pub mod ioirq_8;
#[doc = "ioirq_9 (rw) register accessor: SYS IOMUX CFGSAIF SYSCFG IOIRQ 256: GPIO Interrupt Edge Mask Selector\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`ioirq_9::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`ioirq_9::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@ioirq_9`]
module"]
pub type IOIRQ_9 = crate::Reg<ioirq_9::IOIRQ_9_SPEC>;
#[doc = "SYS IOMUX CFGSAIF SYSCFG IOIRQ 256: GPIO Interrupt Edge Mask Selector"]
pub mod ioirq_9;
#[doc = "ioirq_10 (rw) register accessor: SYS IOMUX CFGSAIF SYSCFG IOIRQ 260: GPIO Interrupt Edge Mask Selector\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`ioirq_10::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`ioirq_10::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@ioirq_10`]
module"]
pub type IOIRQ_10 = crate::Reg<ioirq_10::IOIRQ_10_SPEC>;
#[doc = "SYS IOMUX CFGSAIF SYSCFG IOIRQ 260: GPIO Interrupt Edge Mask Selector"]
pub mod ioirq_10;
#[doc = "ioirq_11 (rw) register accessor: SYS IOMUX CFGSAIF SYSCFG IOIRQ 264: GPIO Register Interrupt Status\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`ioirq_11::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`ioirq_11::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@ioirq_11`]
module"]
pub type IOIRQ_11 = crate::Reg<ioirq_11::IOIRQ_11_SPEC>;
#[doc = "SYS IOMUX CFGSAIF SYSCFG IOIRQ 264: GPIO Register Interrupt Status"]
pub mod ioirq_11;
#[doc = "ioirq_12 (rw) register accessor: SYS IOMUX CFGSAIF SYSCFG IOIRQ 268: GPIO Register Interrupt Status\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`ioirq_12::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`ioirq_12::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@ioirq_12`]
module"]
pub type IOIRQ_12 = crate::Reg<ioirq_12::IOIRQ_12_SPEC>;
#[doc = "SYS IOMUX CFGSAIF SYSCFG IOIRQ 268: GPIO Register Interrupt Status"]
pub mod ioirq_12;
#[doc = "ioirq_13 (rw) register accessor: SYS IOMUX CFGSAIF SYSCFG IOIRQ 272: GPIO Masked Interrupt Status\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`ioirq_13::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`ioirq_13::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@ioirq_13`]
module"]
pub type IOIRQ_13 = crate::Reg<ioirq_13::IOIRQ_13_SPEC>;
#[doc = "SYS IOMUX CFGSAIF SYSCFG IOIRQ 272: GPIO Masked Interrupt Status"]
pub mod ioirq_13;
#[doc = "ioirq_14 (rw) register accessor: SYS IOMUX CFGSAIF SYSCFG IOIRQ 276: GPIO Masked Interrupt Status\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`ioirq_14::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`ioirq_14::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@ioirq_14`]
module"]
pub type IOIRQ_14 = crate::Reg<ioirq_14::IOIRQ_14_SPEC>;
#[doc = "SYS IOMUX CFGSAIF SYSCFG IOIRQ 276: GPIO Masked Interrupt Status"]
pub mod ioirq_14;
#[doc = "ioirq_15 (rw) register accessor: SYS IOMUX CFGSAIF SYSCFG IOIRQ 280: GPIO Synchronization Status\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`ioirq_15::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`ioirq_15::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@ioirq_15`]
module"]
pub type IOIRQ_15 = crate::Reg<ioirq_15::IOIRQ_15_SPEC>;
#[doc = "SYS IOMUX CFGSAIF SYSCFG IOIRQ 280: GPIO Synchronization Status"]
pub mod ioirq_15;
#[doc = "ioirq_16 (rw) register accessor: SYS IOMUX CFGSAIF SYSCFG IOIRQ 284: GPIO Synchronization Status\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`ioirq_16::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`ioirq_16::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@ioirq_16`]
module"]
pub type IOIRQ_16 = crate::Reg<ioirq_16::IOIRQ_16_SPEC>;
#[doc = "SYS IOMUX CFGSAIF SYSCFG IOIRQ 284: GPIO Synchronization Status"]
pub mod ioirq_16;
#[doc = "gpio_0 (rw) register accessor: SYS IOMUX CFG SAIF SYSCFG PADCFG 288: GPIO_0\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`gpio_0::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`gpio_0::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@gpio_0`]
module"]
pub type GPIO_0 = crate::Reg<gpio_0::GPIO_0_SPEC>;
#[doc = "SYS IOMUX CFG SAIF SYSCFG PADCFG 288: GPIO_0"]
pub mod gpio_0;
#[doc = "gpio_1 (rw) register accessor: SYS IOMUX CFG SAIF SYSCFG PADCFG 292: GPIO_1\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`gpio_1::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`gpio_1::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@gpio_1`]
module"]
pub type GPIO_1 = crate::Reg<gpio_1::GPIO_1_SPEC>;
#[doc = "SYS IOMUX CFG SAIF SYSCFG PADCFG 292: GPIO_1"]
pub mod gpio_1;
#[doc = "gpio_2 (rw) register accessor: SYS IOMUX CFG SAIF SYSCFG PADCFG 296: GPIO_2\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`gpio_2::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`gpio_2::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@gpio_2`]
module"]
pub type GPIO_2 = crate::Reg<gpio_2::GPIO_2_SPEC>;
#[doc = "SYS IOMUX CFG SAIF SYSCFG PADCFG 296: GPIO_2"]
pub mod gpio_2;
#[doc = "gpio_3 (rw) register accessor: SYS IOMUX CFG SAIF SYSCFG PADCFG 300: GPIO_3\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`gpio_3::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`gpio_3::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@gpio_3`]
module"]
pub type GPIO_3 = crate::Reg<gpio_3::GPIO_3_SPEC>;
#[doc = "SYS IOMUX CFG SAIF SYSCFG PADCFG 300: GPIO_3"]
pub mod gpio_3;
#[doc = "gpio_4 (rw) register accessor: SYS IOMUX CFG SAIF SYSCFG PADCFG 304: GPIO_4\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`gpio_4::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`gpio_4::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@gpio_4`]
module"]
pub type GPIO_4 = crate::Reg<gpio_4::GPIO_4_SPEC>;
#[doc = "SYS IOMUX CFG SAIF SYSCFG PADCFG 304: GPIO_4"]
pub mod gpio_4;
#[doc = "gpio_5 (rw) register accessor: SYS IOMUX CFG SAIF SYSCFG PADCFG 308: GPIO_5\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`gpio_5::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`gpio_5::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@gpio_5`]
module"]
pub type GPIO_5 = crate::Reg<gpio_5::GPIO_5_SPEC>;
#[doc = "SYS IOMUX CFG SAIF SYSCFG PADCFG 308: GPIO_5"]
pub mod gpio_5;
#[doc = "gpio_6 (rw) register accessor: SYS IOMUX CFG SAIF SYSCFG PADCFG 312: GPIO_6\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`gpio_6::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`gpio_6::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@gpio_6`]
module"]
pub type GPIO_6 = crate::Reg<gpio_6::GPIO_6_SPEC>;
#[doc = "SYS IOMUX CFG SAIF SYSCFG PADCFG 312: GPIO_6"]
pub mod gpio_6;
#[doc = "gpio_7 (rw) register accessor: SYS IOMUX CFG SAIF SYSCFG PADCFG 316: GPIO_7\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`gpio_7::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`gpio_7::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@gpio_7`]
module"]
pub type GPIO_7 = crate::Reg<gpio_7::GPIO_7_SPEC>;
#[doc = "SYS IOMUX CFG SAIF SYSCFG PADCFG 316: GPIO_7"]
pub mod gpio_7;
#[doc = "gpio_8 (rw) register accessor: SYS IOMUX CFG SAIF SYSCFG PADCFG 320: GPIO_8\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`gpio_8::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`gpio_8::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@gpio_8`]
module"]
pub type GPIO_8 = crate::Reg<gpio_8::GPIO_8_SPEC>;
#[doc = "SYS IOMUX CFG SAIF SYSCFG PADCFG 320: GPIO_8"]
pub mod gpio_8;
#[doc = "gpio_9 (rw) register accessor: SYS IOMUX CFG SAIF SYSCFG PADCFG 324: GPIO_9\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`gpio_9::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`gpio_9::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@gpio_9`]
module"]
pub type GPIO_9 = crate::Reg<gpio_9::GPIO_9_SPEC>;
#[doc = "SYS IOMUX CFG SAIF SYSCFG PADCFG 324: GPIO_9"]
pub mod gpio_9;
#[doc = "gpio_10 (rw) register accessor: SYS IOMUX CFG SAIF SYSCFG PADCFG 328: GPIO_10\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`gpio_10::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`gpio_10::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@gpio_10`]
module"]
pub type GPIO_10 = crate::Reg<gpio_10::GPIO_10_SPEC>;
#[doc = "SYS IOMUX CFG SAIF SYSCFG PADCFG 328: GPIO_10"]
pub mod gpio_10;
#[doc = "gpio_11 (rw) register accessor: SYS IOMUX CFG SAIF SYSCFG PADCFG 332: GPIO_11\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`gpio_11::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`gpio_11::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@gpio_11`]
module"]
pub type GPIO_11 = crate::Reg<gpio_11::GPIO_11_SPEC>;
#[doc = "SYS IOMUX CFG SAIF SYSCFG PADCFG 332: GPIO_11"]
pub mod gpio_11;
#[doc = "gpio_12 (rw) register accessor: SYS IOMUX CFG SAIF SYSCFG PADCFG 336: GPIO_12\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`gpio_12::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`gpio_12::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@gpio_12`]
module"]
pub type GPIO_12 = crate::Reg<gpio_12::GPIO_12_SPEC>;
#[doc = "SYS IOMUX CFG SAIF SYSCFG PADCFG 336: GPIO_12"]
pub mod gpio_12;
#[doc = "gpio_13 (rw) register accessor: SYS IOMUX CFG SAIF SYSCFG PADCFG 340: GPIO_13\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`gpio_13::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`gpio_13::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@gpio_13`]
module"]
pub type GPIO_13 = crate::Reg<gpio_13::GPIO_13_SPEC>;
#[doc = "SYS IOMUX CFG SAIF SYSCFG PADCFG 340: GPIO_13"]
pub mod gpio_13;
#[doc = "gpio_14 (rw) register accessor: SYS IOMUX CFG SAIF SYSCFG PADCFG 344: GPIO_14\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`gpio_14::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`gpio_14::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@gpio_14`]
module"]
pub type GPIO_14 = crate::Reg<gpio_14::GPIO_14_SPEC>;
#[doc = "SYS IOMUX CFG SAIF SYSCFG PADCFG 344: GPIO_14"]
pub mod gpio_14;
#[doc = "gpio_15 (rw) register accessor: SYS IOMUX CFG SAIF SYSCFG PADCFG 348: GPIO_15\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`gpio_15::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`gpio_15::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@gpio_15`]
module"]
pub type GPIO_15 = crate::Reg<gpio_15::GPIO_15_SPEC>;
#[doc = "SYS IOMUX CFG SAIF SYSCFG PADCFG 348: GPIO_15"]
pub mod gpio_15;
#[doc = "gpio_16 (rw) register accessor: SYS IOMUX CFG SAIF SYSCFG PADCFG 352: GPIO_16\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`gpio_16::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`gpio_16::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@gpio_16`]
module"]
pub type GPIO_16 = crate::Reg<gpio_16::GPIO_16_SPEC>;
#[doc = "SYS IOMUX CFG SAIF SYSCFG PADCFG 352: GPIO_16"]
pub mod gpio_16;
#[doc = "gpio_17 (rw) register accessor: SYS IOMUX CFG SAIF SYSCFG PADCFG 356: GPIO_17\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`gpio_17::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`gpio_17::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@gpio_17`]
module"]
pub type GPIO_17 = crate::Reg<gpio_17::GPIO_17_SPEC>;
#[doc = "SYS IOMUX CFG SAIF SYSCFG PADCFG 356: GPIO_17"]
pub mod gpio_17;
#[doc = "gpio_18 (rw) register accessor: SYS IOMUX CFG SAIF SYSCFG PADCFG 360: GPIO_18\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`gpio_18::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`gpio_18::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@gpio_18`]
module"]
pub type GPIO_18 = crate::Reg<gpio_18::GPIO_18_SPEC>;
#[doc = "SYS IOMUX CFG SAIF SYSCFG PADCFG 360: GPIO_18"]
pub mod gpio_18;
#[doc = "gpio_19 (rw) register accessor: SYS IOMUX CFG SAIF SYSCFG PADCFG 364: GPIO_19\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`gpio_19::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`gpio_19::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@gpio_19`]
module"]
pub type GPIO_19 = crate::Reg<gpio_19::GPIO_19_SPEC>;
#[doc = "SYS IOMUX CFG SAIF SYSCFG PADCFG 364: GPIO_19"]
pub mod gpio_19;
#[doc = "gpio_20 (rw) register accessor: SYS IOMUX CFG SAIF SYSCFG PADCFG 368: GPIO_20\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`gpio_20::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`gpio_20::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@gpio_20`]
module"]
pub type GPIO_20 = crate::Reg<gpio_20::GPIO_20_SPEC>;
#[doc = "SYS IOMUX CFG SAIF SYSCFG PADCFG 368: GPIO_20"]
pub mod gpio_20;
#[doc = "gpio_21 (rw) register accessor: SYS IOMUX CFG SAIF SYSCFG PADCFG 372: GPIO_21\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`gpio_21::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`gpio_21::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@gpio_21`]
module"]
pub type GPIO_21 = crate::Reg<gpio_21::GPIO_21_SPEC>;
#[doc = "SYS IOMUX CFG SAIF SYSCFG PADCFG 372: GPIO_21"]
pub mod gpio_21;
#[doc = "gpio_22 (rw) register accessor: SYS IOMUX CFG SAIF SYSCFG PADCFG 376: GPIO_22\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`gpio_22::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`gpio_22::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@gpio_22`]
module"]
pub type GPIO_22 = crate::Reg<gpio_22::GPIO_22_SPEC>;
#[doc = "SYS IOMUX CFG SAIF SYSCFG PADCFG 376: GPIO_22"]
pub mod gpio_22;
#[doc = "gpio_23 (rw) register accessor: SYS IOMUX CFG SAIF SYSCFG PADCFG 380: GPIO_23\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`gpio_23::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`gpio_23::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@gpio_23`]
module"]
pub type GPIO_23 = crate::Reg<gpio_23::GPIO_23_SPEC>;
#[doc = "SYS IOMUX CFG SAIF SYSCFG PADCFG 380: GPIO_23"]
pub mod gpio_23;
#[doc = "gpio_24 (rw) register accessor: SYS IOMUX CFG SAIF SYSCFG PADCFG 384: GPIO_24\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`gpio_24::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`gpio_24::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@gpio_24`]
module"]
pub type GPIO_24 = crate::Reg<gpio_24::GPIO_24_SPEC>;
#[doc = "SYS IOMUX CFG SAIF SYSCFG PADCFG 384: GPIO_24"]
pub mod gpio_24;
#[doc = "gpio_25 (rw) register accessor: SYS IOMUX CFG SAIF SYSCFG PADCFG 388: GPIO_25\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`gpio_25::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`gpio_25::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@gpio_25`]
module"]
pub type GPIO_25 = crate::Reg<gpio_25::GPIO_25_SPEC>;
#[doc = "SYS IOMUX CFG SAIF SYSCFG PADCFG 388: GPIO_25"]
pub mod gpio_25;
#[doc = "gpio_26 (rw) register accessor: SYS IOMUX CFG SAIF SYSCFG PADCFG 392: GPIO_26\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`gpio_26::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`gpio_26::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@gpio_26`]
module"]
pub type GPIO_26 = crate::Reg<gpio_26::GPIO_26_SPEC>;
#[doc = "SYS IOMUX CFG SAIF SYSCFG PADCFG 392: GPIO_26"]
pub mod gpio_26;
#[doc = "gpio_27 (rw) register accessor: SYS IOMUX CFG SAIF SYSCFG PADCFG 396: GPIO_27\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`gpio_27::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`gpio_27::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@gpio_27`]
module"]
pub type GPIO_27 = crate::Reg<gpio_27::GPIO_27_SPEC>;
#[doc = "SYS IOMUX CFG SAIF SYSCFG PADCFG 396: GPIO_27"]
pub mod gpio_27;
#[doc = "gpio_28 (rw) register accessor: SYS IOMUX CFG SAIF SYSCFG PADCFG 400: GPIO_28\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`gpio_28::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`gpio_28::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@gpio_28`]
module"]
pub type GPIO_28 = crate::Reg<gpio_28::GPIO_28_SPEC>;
#[doc = "SYS IOMUX CFG SAIF SYSCFG PADCFG 400: GPIO_28"]
pub mod gpio_28;
#[doc = "gpio_29 (rw) register accessor: SYS IOMUX CFG SAIF SYSCFG PADCFG 404: GPIO_29\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`gpio_29::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`gpio_29::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@gpio_29`]
module"]
pub type GPIO_29 = crate::Reg<gpio_29::GPIO_29_SPEC>;
#[doc = "SYS IOMUX CFG SAIF SYSCFG PADCFG 404: GPIO_29"]
pub mod gpio_29;
#[doc = "gpio_30 (rw) register accessor: SYS IOMUX CFG SAIF SYSCFG PADCFG 408: GPIO_30\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`gpio_30::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`gpio_30::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@gpio_30`]
module"]
pub type GPIO_30 = crate::Reg<gpio_30::GPIO_30_SPEC>;
#[doc = "SYS IOMUX CFG SAIF SYSCFG PADCFG 408: GPIO_30"]
pub mod gpio_30;
#[doc = "gpio_31 (rw) register accessor: SYS IOMUX CFG SAIF SYSCFG PADCFG 412: GPIO_31\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`gpio_31::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`gpio_31::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@gpio_31`]
module"]
pub type GPIO_31 = crate::Reg<gpio_31::GPIO_31_SPEC>;
#[doc = "SYS IOMUX CFG SAIF SYSCFG PADCFG 412: GPIO_31"]
pub mod gpio_31;
#[doc = "gpio_32 (rw) register accessor: SYS IOMUX CFG SAIF SYSCFG PADCFG 416: GPIO_32\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`gpio_32::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`gpio_32::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@gpio_32`]
module"]
pub type GPIO_32 = crate::Reg<gpio_32::GPIO_32_SPEC>;
#[doc = "SYS IOMUX CFG SAIF SYSCFG PADCFG 416: GPIO_32"]
pub mod gpio_32;
#[doc = "gpio_33 (rw) register accessor: SYS IOMUX CFG SAIF SYSCFG PADCFG 420: GPIO_33\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`gpio_33::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`gpio_33::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@gpio_33`]
module"]
pub type GPIO_33 = crate::Reg<gpio_33::GPIO_33_SPEC>;
#[doc = "SYS IOMUX CFG SAIF SYSCFG PADCFG 420: GPIO_33"]
pub mod gpio_33;
#[doc = "gpio_34 (rw) register accessor: SYS IOMUX CFG SAIF SYSCFG PADCFG 424: GPIO_34\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`gpio_34::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`gpio_34::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@gpio_34`]
module"]
pub type GPIO_34 = crate::Reg<gpio_34::GPIO_34_SPEC>;
#[doc = "SYS IOMUX CFG SAIF SYSCFG PADCFG 424: GPIO_34"]
pub mod gpio_34;
#[doc = "gpio_35 (rw) register accessor: SYS IOMUX CFG SAIF SYSCFG PADCFG 428: GPIO_35\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`gpio_35::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`gpio_35::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@gpio_35`]
module"]
pub type GPIO_35 = crate::Reg<gpio_35::GPIO_35_SPEC>;
#[doc = "SYS IOMUX CFG SAIF SYSCFG PADCFG 428: GPIO_35"]
pub mod gpio_35;
#[doc = "gpio_36 (rw) register accessor: SYS IOMUX CFG SAIF SYSCFG PADCFG 432: GPIO_36\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`gpio_36::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`gpio_36::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@gpio_36`]
module"]
pub type GPIO_36 = crate::Reg<gpio_36::GPIO_36_SPEC>;
#[doc = "SYS IOMUX CFG SAIF SYSCFG PADCFG 432: GPIO_36"]
pub mod gpio_36;
#[doc = "gpio_37 (rw) register accessor: SYS IOMUX CFG SAIF SYSCFG PADCFG 436: GPIO_37\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`gpio_37::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`gpio_37::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@gpio_37`]
module"]
pub type GPIO_37 = crate::Reg<gpio_37::GPIO_37_SPEC>;
#[doc = "SYS IOMUX CFG SAIF SYSCFG PADCFG 436: GPIO_37"]
pub mod gpio_37;
#[doc = "gpio_38 (rw) register accessor: SYS IOMUX CFG SAIF SYSCFG PADCFG 440: GPIO_38\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`gpio_38::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`gpio_38::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@gpio_38`]
module"]
pub type GPIO_38 = crate::Reg<gpio_38::GPIO_38_SPEC>;
#[doc = "SYS IOMUX CFG SAIF SYSCFG PADCFG 440: GPIO_38"]
pub mod gpio_38;
#[doc = "gpio_39 (rw) register accessor: SYS IOMUX CFG SAIF SYSCFG PADCFG 444: GPIO_39\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`gpio_39::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`gpio_39::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@gpio_39`]
module"]
pub type GPIO_39 = crate::Reg<gpio_39::GPIO_39_SPEC>;
#[doc = "SYS IOMUX CFG SAIF SYSCFG PADCFG 444: GPIO_39"]
pub mod gpio_39;
#[doc = "gpio_40 (rw) register accessor: SYS IOMUX CFG SAIF SYSCFG PADCFG 448: GPIO_40\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`gpio_40::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`gpio_40::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@gpio_40`]
module"]
pub type GPIO_40 = crate::Reg<gpio_40::GPIO_40_SPEC>;
#[doc = "SYS IOMUX CFG SAIF SYSCFG PADCFG 448: GPIO_40"]
pub mod gpio_40;
#[doc = "gpio_41 (rw) register accessor: SYS IOMUX CFG SAIF SYSCFG PADCFG 452: GPIO_41\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`gpio_41::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`gpio_41::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@gpio_41`]
module"]
pub type GPIO_41 = crate::Reg<gpio_41::GPIO_41_SPEC>;
#[doc = "SYS IOMUX CFG SAIF SYSCFG PADCFG 452: GPIO_41"]
pub mod gpio_41;
#[doc = "gpio_42 (rw) register accessor: SYS IOMUX CFG SAIF SYSCFG PADCFG 456: GPIO_42\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`gpio_42::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`gpio_42::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@gpio_42`]
module"]
pub type GPIO_42 = crate::Reg<gpio_42::GPIO_42_SPEC>;
#[doc = "SYS IOMUX CFG SAIF SYSCFG PADCFG 456: GPIO_42"]
pub mod gpio_42;
#[doc = "gpio_43 (rw) register accessor: SYS IOMUX CFG SAIF SYSCFG PADCFG 460: GPIO_43\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`gpio_43::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`gpio_43::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@gpio_43`]
module"]
pub type GPIO_43 = crate::Reg<gpio_43::GPIO_43_SPEC>;
#[doc = "SYS IOMUX CFG SAIF SYSCFG PADCFG 460: GPIO_43"]
pub mod gpio_43;
#[doc = "gpio_44 (rw) register accessor: SYS IOMUX CFG SAIF SYSCFG PADCFG 464: GPIO_44\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`gpio_44::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`gpio_44::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@gpio_44`]
module"]
pub type GPIO_44 = crate::Reg<gpio_44::GPIO_44_SPEC>;
#[doc = "SYS IOMUX CFG SAIF SYSCFG PADCFG 464: GPIO_44"]
pub mod gpio_44;
#[doc = "gpio_45 (rw) register accessor: SYS IOMUX CFG SAIF SYSCFG PADCFG 468: GPIO_45\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`gpio_45::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`gpio_45::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@gpio_45`]
module"]
pub type GPIO_45 = crate::Reg<gpio_45::GPIO_45_SPEC>;
#[doc = "SYS IOMUX CFG SAIF SYSCFG PADCFG 468: GPIO_45"]
pub mod gpio_45;
#[doc = "gpio_46 (rw) register accessor: SYS IOMUX CFG SAIF SYSCFG PADCFG 472: GPIO_46\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`gpio_46::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`gpio_46::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@gpio_46`]
module"]
pub type GPIO_46 = crate::Reg<gpio_46::GPIO_46_SPEC>;
#[doc = "SYS IOMUX CFG SAIF SYSCFG PADCFG 472: GPIO_46"]
pub mod gpio_46;
#[doc = "gpio_47 (rw) register accessor: SYS IOMUX CFG SAIF SYSCFG PADCFG 476: GPIO_47\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`gpio_47::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`gpio_47::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@gpio_47`]
module"]
pub type GPIO_47 = crate::Reg<gpio_47::GPIO_47_SPEC>;
#[doc = "SYS IOMUX CFG SAIF SYSCFG PADCFG 476: GPIO_47"]
pub mod gpio_47;
#[doc = "gpio_48 (rw) register accessor: SYS IOMUX CFG SAIF SYSCFG PADCFG 480: GPIO_48\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`gpio_48::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`gpio_48::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@gpio_48`]
module"]
pub type GPIO_48 = crate::Reg<gpio_48::GPIO_48_SPEC>;
#[doc = "SYS IOMUX CFG SAIF SYSCFG PADCFG 480: GPIO_48"]
pub mod gpio_48;
#[doc = "gpio_49 (rw) register accessor: SYS IOMUX CFG SAIF SYSCFG PADCFG 484: GPIO_49\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`gpio_49::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`gpio_49::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@gpio_49`]
module"]
pub type GPIO_49 = crate::Reg<gpio_49::GPIO_49_SPEC>;
#[doc = "SYS IOMUX CFG SAIF SYSCFG PADCFG 484: GPIO_49"]
pub mod gpio_49;
#[doc = "gpio_50 (rw) register accessor: SYS IOMUX CFG SAIF SYSCFG PADCFG 488: GPIO_50\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`gpio_50::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`gpio_50::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@gpio_50`]
module"]
pub type GPIO_50 = crate::Reg<gpio_50::GPIO_50_SPEC>;
#[doc = "SYS IOMUX CFG SAIF SYSCFG PADCFG 488: GPIO_50"]
pub mod gpio_50;
#[doc = "gpio_51 (rw) register accessor: SYS IOMUX CFG SAIF SYSCFG PADCFG 492: GPIO_51\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`gpio_51::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`gpio_51::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@gpio_51`]
module"]
pub type GPIO_51 = crate::Reg<gpio_51::GPIO_51_SPEC>;
#[doc = "SYS IOMUX CFG SAIF SYSCFG PADCFG 492: GPIO_51"]
pub mod gpio_51;
#[doc = "gpio_52 (rw) register accessor: SYS IOMUX CFG SAIF SYSCFG PADCFG 496: GPIO_52\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`gpio_52::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`gpio_52::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@gpio_52`]
module"]
pub type GPIO_52 = crate::Reg<gpio_52::GPIO_52_SPEC>;
#[doc = "SYS IOMUX CFG SAIF SYSCFG PADCFG 496: GPIO_52"]
pub mod gpio_52;
#[doc = "gpio_53 (rw) register accessor: SYS IOMUX CFG SAIF SYSCFG PADCFG 500: GPIO_53\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`gpio_53::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`gpio_53::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@gpio_53`]
module"]
pub type GPIO_53 = crate::Reg<gpio_53::GPIO_53_SPEC>;
#[doc = "SYS IOMUX CFG SAIF SYSCFG PADCFG 500: GPIO_53"]
pub mod gpio_53;
#[doc = "gpio_54 (rw) register accessor: SYS IOMUX CFG SAIF SYSCFG PADCFG 504: GPIO_54\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`gpio_54::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`gpio_54::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@gpio_54`]
module"]
pub type GPIO_54 = crate::Reg<gpio_54::GPIO_54_SPEC>;
#[doc = "SYS IOMUX CFG SAIF SYSCFG PADCFG 504: GPIO_54"]
pub mod gpio_54;
#[doc = "gpio_55 (rw) register accessor: SYS IOMUX CFG SAIF SYSCFG PADCFG 508: GPIO_55\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`gpio_55::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`gpio_55::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@gpio_55`]
module"]
pub type GPIO_55 = crate::Reg<gpio_55::GPIO_55_SPEC>;
#[doc = "SYS IOMUX CFG SAIF SYSCFG PADCFG 508: GPIO_55"]
pub mod gpio_55;
#[doc = "gpio_56 (rw) register accessor: SYS IOMUX CFG SAIF SYSCFG PADCFG 512: GPIO_56\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`gpio_56::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`gpio_56::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@gpio_56`]
module"]
pub type GPIO_56 = crate::Reg<gpio_56::GPIO_56_SPEC>;
#[doc = "SYS IOMUX CFG SAIF SYSCFG PADCFG 512: GPIO_56"]
pub mod gpio_56;
#[doc = "gpio_57 (rw) register accessor: SYS IOMUX CFG SAIF SYSCFG PADCFG 516: GPIO_57\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`gpio_57::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`gpio_57::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@gpio_57`]
module"]
pub type GPIO_57 = crate::Reg<gpio_57::GPIO_57_SPEC>;
#[doc = "SYS IOMUX CFG SAIF SYSCFG PADCFG 516: GPIO_57"]
pub mod gpio_57;
#[doc = "gpio_58 (rw) register accessor: SYS IOMUX CFG SAIF SYSCFG PADCFG 520: GPIO_58\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`gpio_58::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`gpio_58::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@gpio_58`]
module"]
pub type GPIO_58 = crate::Reg<gpio_58::GPIO_58_SPEC>;
#[doc = "SYS IOMUX CFG SAIF SYSCFG PADCFG 520: GPIO_58"]
pub mod gpio_58;
#[doc = "gpio_59 (rw) register accessor: SYS IOMUX CFG SAIF SYSCFG PADCFG 524: GPIO_59\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`gpio_59::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`gpio_59::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@gpio_59`]
module"]
pub type GPIO_59 = crate::Reg<gpio_59::GPIO_59_SPEC>;
#[doc = "SYS IOMUX CFG SAIF SYSCFG PADCFG 524: GPIO_59"]
pub mod gpio_59;
#[doc = "gpio_60 (rw) register accessor: SYS IOMUX CFG SAIF SYSCFG PADCFG 528: GPIO_60\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`gpio_60::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`gpio_60::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@gpio_60`]
module"]
pub type GPIO_60 = crate::Reg<gpio_60::GPIO_60_SPEC>;
#[doc = "SYS IOMUX CFG SAIF SYSCFG PADCFG 528: GPIO_60"]
pub mod gpio_60;
#[doc = "gpio_61 (rw) register accessor: SYS IOMUX CFG SAIF SYSCFG PADCFG 532: GPIO_61\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`gpio_61::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`gpio_61::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@gpio_61`]
module"]
pub type GPIO_61 = crate::Reg<gpio_61::GPIO_61_SPEC>;
#[doc = "SYS IOMUX CFG SAIF SYSCFG PADCFG 532: GPIO_61"]
pub mod gpio_61;
#[doc = "gpio_62 (rw) register accessor: SYS IOMUX CFG SAIF SYSCFG PADCFG 536: GPIO_62\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`gpio_62::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`gpio_62::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@gpio_62`]
module"]
pub type GPIO_62 = crate::Reg<gpio_62::GPIO_62_SPEC>;
#[doc = "SYS IOMUX CFG SAIF SYSCFG PADCFG 536: GPIO_62"]
pub mod gpio_62;
#[doc = "gpio_63 (rw) register accessor: SYS IOMUX CFG SAIF SYSCFG PADCFG 540: GPIO_63\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`gpio_63::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`gpio_63::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@gpio_63`]
module"]
pub type GPIO_63 = crate::Reg<gpio_63::GPIO_63_SPEC>;
#[doc = "SYS IOMUX CFG SAIF SYSCFG PADCFG 540: GPIO_63"]
pub mod gpio_63;
#[doc = "sd0_clk (rw) register accessor: SYS IOMUX CFG SAIF SYSCFG PADCFG 544: SD0_CLK\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`sd0_clk::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`sd0_clk::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@sd0_clk`]
module"]
pub type SD0_CLK = crate::Reg<sd0_clk::SD0_CLK_SPEC>;
#[doc = "SYS IOMUX CFG SAIF SYSCFG PADCFG 544: SD0_CLK"]
pub mod sd0_clk;
#[doc = "sd0_cmd (rw) register accessor: SYS IOMUX CFG SAIF SYSCFG PADCFG 548: SD0_CMD\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`sd0_cmd::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`sd0_cmd::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@sd0_cmd`]
module"]
pub type SD0_CMD = crate::Reg<sd0_cmd::SD0_CMD_SPEC>;
#[doc = "SYS IOMUX CFG SAIF SYSCFG PADCFG 548: SD0_CMD"]
pub mod sd0_cmd;
#[doc = "sd0_data_0 (rw) register accessor: SYS IOMUX CFG SAIF SYSCFG PADCFG 552: SD0_DATA_0\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`sd0_data_0::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`sd0_data_0::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@sd0_data_0`]
module"]
pub type SD0_DATA_0 = crate::Reg<sd0_data_0::SD0_DATA_0_SPEC>;
#[doc = "SYS IOMUX CFG SAIF SYSCFG PADCFG 552: SD0_DATA_0"]
pub mod sd0_data_0;
#[doc = "sd0_data_1 (rw) register accessor: SYS IOMUX CFG SAIF SYSCFG PADCFG 556: SD0_DATA_1\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`sd0_data_1::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`sd0_data_1::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@sd0_data_1`]
module"]
pub type SD0_DATA_1 = crate::Reg<sd0_data_1::SD0_DATA_1_SPEC>;
#[doc = "SYS IOMUX CFG SAIF SYSCFG PADCFG 556: SD0_DATA_1"]
pub mod sd0_data_1;
#[doc = "sd0_data_2 (rw) register accessor: SYS IOMUX CFG SAIF SYSCFG PADCFG 560: SD0_DATA_2\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`sd0_data_2::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`sd0_data_2::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@sd0_data_2`]
module"]
pub type SD0_DATA_2 = crate::Reg<sd0_data_2::SD0_DATA_2_SPEC>;
#[doc = "SYS IOMUX CFG SAIF SYSCFG PADCFG 560: SD0_DATA_2"]
pub mod sd0_data_2;
#[doc = "sd0_data_3 (rw) register accessor: SYS IOMUX CFG SAIF SYSCFG PADCFG 564: SD0_DATA_3\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`sd0_data_3::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`sd0_data_3::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@sd0_data_3`]
module"]
pub type SD0_DATA_3 = crate::Reg<sd0_data_3::SD0_DATA_3_SPEC>;
#[doc = "SYS IOMUX CFG SAIF SYSCFG PADCFG 564: SD0_DATA_3"]
pub mod sd0_data_3;
#[doc = "sd0_data_4 (rw) register accessor: SYS IOMUX CFG SAIF SYSCFG PADCFG 568: SD0_DATA_4\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`sd0_data_4::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`sd0_data_4::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@sd0_data_4`]
module"]
pub type SD0_DATA_4 = crate::Reg<sd0_data_4::SD0_DATA_4_SPEC>;
#[doc = "SYS IOMUX CFG SAIF SYSCFG PADCFG 568: SD0_DATA_4"]
pub mod sd0_data_4;
#[doc = "sd0_data_5 (rw) register accessor: SYS IOMUX CFG SAIF SYSCFG PADCFG 572: SD0_DATA_5\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`sd0_data_5::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`sd0_data_5::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@sd0_data_5`]
module"]
pub type SD0_DATA_5 = crate::Reg<sd0_data_5::SD0_DATA_5_SPEC>;
#[doc = "SYS IOMUX CFG SAIF SYSCFG PADCFG 572: SD0_DATA_5"]
pub mod sd0_data_5;
#[doc = "sd0_data_6 (rw) register accessor: SYS IOMUX CFG SAIF SYSCFG PADCFG 576: SD0_DATA_6\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`sd0_data_6::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`sd0_data_6::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@sd0_data_6`]
module"]
pub type SD0_DATA_6 = crate::Reg<sd0_data_6::SD0_DATA_6_SPEC>;
#[doc = "SYS IOMUX CFG SAIF SYSCFG PADCFG 576: SD0_DATA_6"]
pub mod sd0_data_6;
#[doc = "sd0_data_7 (rw) register accessor: SYS IOMUX CFG SAIF SYSCFG PADCFG 580: SD0_DATA_7\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`sd0_data_7::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`sd0_data_7::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@sd0_data_7`]
module"]
pub type SD0_DATA_7 = crate::Reg<sd0_data_7::SD0_DATA_7_SPEC>;
#[doc = "SYS IOMUX CFG SAIF SYSCFG PADCFG 580: SD0_DATA_7"]
pub mod sd0_data_7;
#[doc = "sd0_strb (rw) register accessor: SYS IOMUX CFG SAIF SYSCFG PADCFG 584: SD0_STRB\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`sd0_strb::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`sd0_strb::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@sd0_strb`]
module"]
pub type SD0_STRB = crate::Reg<sd0_strb::SD0_STRB_SPEC>;
#[doc = "SYS IOMUX CFG SAIF SYSCFG PADCFG 584: SD0_STRB"]
pub mod sd0_strb;
#[doc = "gmac1_mdc (rw) register accessor: GPIO GMAC1 MDC Pad Configuration\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`gmac1_mdc::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`gmac1_mdc::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@gmac1_mdc`]
module"]
pub type GMAC1_MDC = crate::Reg<gmac1_mdc::GMAC1_MDC_SPEC>;
#[doc = "GPIO GMAC1 MDC Pad Configuration"]
pub mod gmac1_mdc;
#[doc = "gmac1_mdio (rw) register accessor: GPIO GMAC1 MDIO Pad Configuration\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`gmac1_mdio::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`gmac1_mdio::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@gmac1_mdio`]
module"]
pub type GMAC1_MDIO = crate::Reg<gmac1_mdio::GMAC1_MDIO_SPEC>;
#[doc = "GPIO GMAC1 MDIO Pad Configuration"]
pub mod gmac1_mdio;
#[doc = "gmac1_rxd_0 (rw) register accessor: GPIO GMAC1 RXD_0 Pad Configuration\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`gmac1_rxd_0::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`gmac1_rxd_0::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@gmac1_rxd_0`]
module"]
pub type GMAC1_RXD_0 = crate::Reg<gmac1_rxd_0::GMAC1_RXD_0_SPEC>;
#[doc = "GPIO GMAC1 RXD_0 Pad Configuration"]
pub mod gmac1_rxd_0;
#[doc = "gmac1_rxd_1 (rw) register accessor: GPIO GMAC1 RXD_1 Pad Configuration\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`gmac1_rxd_1::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`gmac1_rxd_1::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@gmac1_rxd_1`]
module"]
pub type GMAC1_RXD_1 = crate::Reg<gmac1_rxd_1::GMAC1_RXD_1_SPEC>;
#[doc = "GPIO GMAC1 RXD_1 Pad Configuration"]
pub mod gmac1_rxd_1;
#[doc = "gmac1_rxd_2 (rw) register accessor: GPIO GMAC1 RXD_2 Pad Configuration\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`gmac1_rxd_2::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`gmac1_rxd_2::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@gmac1_rxd_2`]
module"]
pub type GMAC1_RXD_2 = crate::Reg<gmac1_rxd_2::GMAC1_RXD_2_SPEC>;
#[doc = "GPIO GMAC1 RXD_2 Pad Configuration"]
pub mod gmac1_rxd_2;
#[doc = "gmac1_rxd_3 (rw) register accessor: GPIO GMAC1 RXD_3 Pad Configuration\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`gmac1_rxd_3::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`gmac1_rxd_3::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@gmac1_rxd_3`]
module"]
pub type GMAC1_RXD_3 = crate::Reg<gmac1_rxd_3::GMAC1_RXD_3_SPEC>;
#[doc = "GPIO GMAC1 RXD_3 Pad Configuration"]
pub mod gmac1_rxd_3;
#[doc = "gmac1_rxdv (rw) register accessor: GPIO GMAC1 RXDV Pad Configuration\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`gmac1_rxdv::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`gmac1_rxdv::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@gmac1_rxdv`]
module"]
pub type GMAC1_RXDV = crate::Reg<gmac1_rxdv::GMAC1_RXDV_SPEC>;
#[doc = "GPIO GMAC1 RXDV Pad Configuration"]
pub mod gmac1_rxdv;
#[doc = "gmac1_rxc (rw) register accessor: GPIO GMAC1 RXC Pad Configuration\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`gmac1_rxc::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`gmac1_rxc::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@gmac1_rxc`]
module"]
pub type GMAC1_RXC = crate::Reg<gmac1_rxc::GMAC1_RXC_SPEC>;
#[doc = "GPIO GMAC1 RXC Pad Configuration"]
pub mod gmac1_rxc;
#[doc = "gmac1_txd_0 (rw) register accessor: GPIO GMAC1 TXD_0 Pad Configuration\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`gmac1_txd_0::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`gmac1_txd_0::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@gmac1_txd_0`]
module"]
pub type GMAC1_TXD_0 = crate::Reg<gmac1_txd_0::GMAC1_TXD_0_SPEC>;
#[doc = "GPIO GMAC1 TXD_0 Pad Configuration"]
pub mod gmac1_txd_0;
#[doc = "gmac1_txd_1 (rw) register accessor: GPIO GMAC1 TXD_1 Pad Configuration\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`gmac1_txd_1::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`gmac1_txd_1::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@gmac1_txd_1`]
module"]
pub type GMAC1_TXD_1 = crate::Reg<gmac1_txd_1::GMAC1_TXD_1_SPEC>;
#[doc = "GPIO GMAC1 TXD_1 Pad Configuration"]
pub mod gmac1_txd_1;
#[doc = "gmac1_txd_2 (rw) register accessor: GPIO GMAC1 TXD_2 Pad Configuration\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`gmac1_txd_2::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`gmac1_txd_2::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@gmac1_txd_2`]
module"]
pub type GMAC1_TXD_2 = crate::Reg<gmac1_txd_2::GMAC1_TXD_2_SPEC>;
#[doc = "GPIO GMAC1 TXD_2 Pad Configuration"]
pub mod gmac1_txd_2;
#[doc = "gmac1_txd_3 (rw) register accessor: GPIO GMAC1 TXD_3 Pad Configuration\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`gmac1_txd_3::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`gmac1_txd_3::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@gmac1_txd_3`]
module"]
pub type GMAC1_TXD_3 = crate::Reg<gmac1_txd_3::GMAC1_TXD_3_SPEC>;
#[doc = "GPIO GMAC1 TXD_3 Pad Configuration"]
pub mod gmac1_txd_3;
#[doc = "gmac1_txen (rw) register accessor: GPIO GMAC1 TXEN Pad Configuration\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`gmac1_txen::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`gmac1_txen::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@gmac1_txen`]
module"]
pub type GMAC1_TXEN = crate::Reg<gmac1_txen::GMAC1_TXEN_SPEC>;
#[doc = "GPIO GMAC1 TXEN Pad Configuration"]
pub mod gmac1_txen;
#[doc = "gmac1_txc (rw) register accessor: GPIO GMAC1 TXC Pad Configuration\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`gmac1_txc::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`gmac1_txc::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@gmac1_txc`]
module"]
pub type GMAC1_TXC = crate::Reg<gmac1_txc::GMAC1_TXC_SPEC>;
#[doc = "GPIO GMAC1 TXC Pad Configuration"]
pub mod gmac1_txc;
#[doc = "qspi_sclk (rw) register accessor: SYS IOMUX CFG SAIF SYSCFG PADCFG 644: QSPI_SCLK\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`qspi_sclk::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`qspi_sclk::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@qspi_sclk`]
module"]
pub type QSPI_SCLK = crate::Reg<qspi_sclk::QSPI_SCLK_SPEC>;
#[doc = "SYS IOMUX CFG SAIF SYSCFG PADCFG 644: QSPI_SCLK"]
pub mod qspi_sclk;
#[doc = "qspi_csn_0 (rw) register accessor: SYS IOMUX CFG SAIF SYSCFG PADCFG 648: QSPI_CSN_0\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`qspi_csn_0::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`qspi_csn_0::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@qspi_csn_0`]
module"]
pub type QSPI_CSN_0 = crate::Reg<qspi_csn_0::QSPI_CSN_0_SPEC>;
#[doc = "SYS IOMUX CFG SAIF SYSCFG PADCFG 648: QSPI_CSN_0"]
pub mod qspi_csn_0;
#[doc = "qspi_data_0 (rw) register accessor: SYS IOMUX CFG SAIF SYSCFG PADCFG 652: QSPI_DATA_0\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`qspi_data_0::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`qspi_data_0::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@qspi_data_0`]
module"]
pub type QSPI_DATA_0 = crate::Reg<qspi_data_0::QSPI_DATA_0_SPEC>;
#[doc = "SYS IOMUX CFG SAIF SYSCFG PADCFG 652: QSPI_DATA_0"]
pub mod qspi_data_0;
#[doc = "qspi_data_1 (rw) register accessor: SYS IOMUX CFG SAIF SYSCFG PADCFG 656: QSPI_DATA_1\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`qspi_data_1::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`qspi_data_1::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@qspi_data_1`]
module"]
pub type QSPI_DATA_1 = crate::Reg<qspi_data_1::QSPI_DATA_1_SPEC>;
#[doc = "SYS IOMUX CFG SAIF SYSCFG PADCFG 656: QSPI_DATA_1"]
pub mod qspi_data_1;
#[doc = "qspi_data_2 (rw) register accessor: SYS IOMUX CFG SAIF SYSCFG PADCFG 660: QSPI_DATA_2\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`qspi_data_2::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`qspi_data_2::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@qspi_data_2`]
module"]
pub type QSPI_DATA_2 = crate::Reg<qspi_data_2::QSPI_DATA_2_SPEC>;
#[doc = "SYS IOMUX CFG SAIF SYSCFG PADCFG 660: QSPI_DATA_2"]
pub mod qspi_data_2;
#[doc = "qspi_data_3 (rw) register accessor: SYS IOMUX CFG SAIF SYSCFG PADCFG 664: QSPI_DATA_3\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`qspi_data_3::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`qspi_data_3::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@qspi_data_3`]
module"]
pub type QSPI_DATA_3 = crate::Reg<qspi_data_3::QSPI_DATA_3_SPEC>;
#[doc = "SYS IOMUX CFG SAIF SYSCFG PADCFG 664: QSPI_DATA_3"]
pub mod qspi_data_3;
#[doc = "func_sel_0 (rw) register accessor: SYS IOMUX CFG SAIF SYSCFG 668\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`func_sel_0::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`func_sel_0::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@func_sel_0`]
module"]
pub type FUNC_SEL_0 = crate::Reg<func_sel_0::FUNC_SEL_0_SPEC>;
#[doc = "SYS IOMUX CFG SAIF SYSCFG 668"]
pub mod func_sel_0;
#[doc = "func_sel_1 (rw) register accessor: SYS IOMUX CFG SAIF SYSCFG 672\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`func_sel_1::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`func_sel_1::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@func_sel_1`]
module"]
pub type FUNC_SEL_1 = crate::Reg<func_sel_1::FUNC_SEL_1_SPEC>;
#[doc = "SYS IOMUX CFG SAIF SYSCFG 672"]
pub mod func_sel_1;
#[doc = "func_sel_2 (rw) register accessor: SYS IOMUX CFG SAIF SYSCFG 676\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`func_sel_2::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`func_sel_2::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@func_sel_2`]
module"]
pub type FUNC_SEL_2 = crate::Reg<func_sel_2::FUNC_SEL_2_SPEC>;
#[doc = "SYS IOMUX CFG SAIF SYSCFG 676"]
pub mod func_sel_2;
#[doc = "func_sel_3 (rw) register accessor: SYS IOMUX CFG SAIF SYSCFG 680\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`func_sel_3::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`func_sel_3::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@func_sel_3`]
module"]
pub type FUNC_SEL_3 = crate::Reg<func_sel_3::FUNC_SEL_3_SPEC>;
#[doc = "SYS IOMUX CFG SAIF SYSCFG 680"]
pub mod func_sel_3;
#[doc = "func_sel_4 (rw) register accessor: SYS IOMUX CFG SAIF SYSCFG 684\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`func_sel_4::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`func_sel_4::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@func_sel_4`]
module"]
pub type FUNC_SEL_4 = crate::Reg<func_sel_4::FUNC_SEL_4_SPEC>;
#[doc = "SYS IOMUX CFG SAIF SYSCFG 684"]
pub mod func_sel_4;
#[doc = "func_sel_5 (rw) register accessor: SYS IOMUX CFG SAIF SYSCFG 688\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`func_sel_5::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`func_sel_5::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@func_sel_5`]
module"]
pub type FUNC_SEL_5 = crate::Reg<func_sel_5::FUNC_SEL_5_SPEC>;
#[doc = "SYS IOMUX CFG SAIF SYSCFG 688"]
pub mod func_sel_5;
#[doc = "func_sel_6 (rw) register accessor: SYS IOMUX CFG SAIF SYSCFG 692\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`func_sel_6::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`func_sel_6::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@func_sel_6`]
module"]
pub type FUNC_SEL_6 = crate::Reg<func_sel_6::FUNC_SEL_6_SPEC>;
#[doc = "SYS IOMUX CFG SAIF SYSCFG 692"]
pub mod func_sel_6;
