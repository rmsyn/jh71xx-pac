#[doc = r"Register block"]
#[repr(C)]
pub struct RegisterBlock {
    gpo_doen0: GPO_DOEN0,
    gpo_doen1: GPO_DOEN1,
    gpo_doen2: GPO_DOEN2,
    gpo_doen3: GPO_DOEN3,
    gpo_doen4: GPO_DOEN4,
    gpo_doen5: GPO_DOEN5,
    gpo_doen6: GPO_DOEN6,
    gpo_doen7: GPO_DOEN7,
    gpo_doen8: GPO_DOEN8,
    gpo_doen9: GPO_DOEN9,
    gpo_doen10: GPO_DOEN10,
    gpo_doen11: GPO_DOEN11,
    gpo_doen12: GPO_DOEN12,
    gpo_doen13: GPO_DOEN13,
    _reserved_14_gpi0: [u8; 0x4c],
    gpi4: GPI4,
    gpi8: GPI8,
    gpi12: GPI12,
    gpi16: GPI16,
    gpi20: GPI20,
    gpi24: GPI24,
    gpi28: GPI28,
    gpi32: GPI32,
    gpi36: GPI36,
    gpi40: GPI40,
    gpi44: GPI44,
    gpi48: GPI48,
    gpi52: GPI52,
    gpi56: GPI56,
    gpi60: GPI60,
    gpi64: GPI64,
    gpi68: GPI68,
    gpi72: GPI72,
    gpi76: GPI76,
    gpi80: GPI80,
    gpi84: GPI84,
    gpi88: GPI88,
    ioirq0: IOIRQ0,
    _reserved38: [u8; 0x40],
    padcfg_gpio0: PADCFG_GPIO0,
    padcfg_gpio1: PADCFG_GPIO1,
    padcfg_gpio2: PADCFG_GPIO2,
    padcfg_gpio3: PADCFG_GPIO3,
    padcfg_gpio4: PADCFG_GPIO4,
    padcfg_gpio5: PADCFG_GPIO5,
    padcfg_gpio6: PADCFG_GPIO6,
    padcfg_gpio7: PADCFG_GPIO7,
    padcfg_gpio8: PADCFG_GPIO8,
    padcfg_gpio9: PADCFG_GPIO9,
    padcfg_gpio10: PADCFG_GPIO10,
    padcfg_gpio11: PADCFG_GPIO11,
    padcfg_gpio12: PADCFG_GPIO12,
    padcfg_gpio13: PADCFG_GPIO13,
    padcfg_gpio14: PADCFG_GPIO14,
    padcfg_gpio15: PADCFG_GPIO15,
    padcfg_gpio16: PADCFG_GPIO16,
    padcfg_gpio17: PADCFG_GPIO17,
    padcfg_gpio18: PADCFG_GPIO18,
    padcfg_gpio19: PADCFG_GPIO19,
    padcfg_gpio20: PADCFG_GPIO20,
    padcfg_gpio21: PADCFG_GPIO21,
    padcfg_gpio22: PADCFG_GPIO22,
    padcfg_gpio23: PADCFG_GPIO23,
    padcfg_gpio24: PADCFG_GPIO24,
    padcfg_gpio25: PADCFG_GPIO25,
    padcfg_gpio26: PADCFG_GPIO26,
    padcfg_gpio27: PADCFG_GPIO27,
    padcfg_gpio28: PADCFG_GPIO28,
    padcfg_gpio29: PADCFG_GPIO29,
    padcfg_gpio30: PADCFG_GPIO30,
    padcfg_gpio31: PADCFG_GPIO31,
    padcfg_gpio32: PADCFG_GPIO32,
    padcfg_gpio33: PADCFG_GPIO33,
    padcfg_gpio34: PADCFG_GPIO34,
    padcfg_gpio35: PADCFG_GPIO35,
    padcfg_gpio36: PADCFG_GPIO36,
    padcfg_gpio37: PADCFG_GPIO37,
    padcfg_gpio38: PADCFG_GPIO38,
    padcfg_gpio39: PADCFG_GPIO39,
    padcfg_gpio40: PADCFG_GPIO40,
    padcfg_gpio41: PADCFG_GPIO41,
    padcfg_gpio42: PADCFG_GPIO42,
    padcfg_gpio43: PADCFG_GPIO43,
    padcfg_gpio44: PADCFG_GPIO44,
    padcfg_gpio45: PADCFG_GPIO45,
    padcfg_gpio46: PADCFG_GPIO46,
    padcfg_gpio47: PADCFG_GPIO47,
    padcfg_gpio48: PADCFG_GPIO48,
    padcfg_gpio49: PADCFG_GPIO49,
    padcfg_gpio50: PADCFG_GPIO50,
    padcfg_gpio51: PADCFG_GPIO51,
    padcfg_gpio52: PADCFG_GPIO52,
    padcfg_gpio53: PADCFG_GPIO53,
    padcfg_gpio54: PADCFG_GPIO54,
    padcfg_gpio55: PADCFG_GPIO55,
    padcfg_gpio56: PADCFG_GPIO56,
    padcfg_gpio57: PADCFG_GPIO57,
    padcfg_gpio58: PADCFG_GPIO58,
    padcfg_gpio59: PADCFG_GPIO59,
    padcfg_gpio60: PADCFG_GPIO60,
    padcfg_gpio61: PADCFG_GPIO61,
    padcfg_gpio62: PADCFG_GPIO62,
    padcfg_gpio63: PADCFG_GPIO63,
    padcfg_sd0_clk: PADCFG_SD0_CLK,
    padcfg_sd0_cmd: PADCFG_SD0_CMD,
    padcfg_sd0_data0: PADCFG_SD0_DATA0,
    _reserved105: [u8; 0x0c],
    padcfg_sd0_data1: PADCFG_SD0_DATA1,
    _reserved106: [u8; 0x0c],
    _reserved_106_padcfg_sd0: [u8; 0x04],
    _reserved107: [u8; 0x0c],
    padcfg_sd0_data3: PADCFG_SD0_DATA3,
    _reserved108: [u8; 0x0c],
    padcfg_sd0_data4: PADCFG_SD0_DATA4,
    _reserved109: [u8; 0x0c],
    padcfg_sd0_data5: PADCFG_SD0_DATA5,
    _reserved110: [u8; 0x08],
    padcfg_qspi_sclk: PADCFG_QSPI_SCLK,
    _reserved_111_padcfg: [u8; 0x04],
    padcfg_qspi_data0: PADCFG_QSPI_DATA0,
    _reserved113: [u8; 0x08],
    padcfg_sd0_data7: PADCFG_SD0_DATA7,
    _reserved_114_func_sel0: [u8; 0x04],
    func_sel1: FUNC_SEL1,
    func_sel2: FUNC_SEL2,
    func_sel3: FUNC_SEL3,
    _reserved_118_func_sel4: [u8; 0x04],
    func_sel5: FUNC_SEL5,
    func_sel6: FUNC_SEL6,
    _reserved121: [u8; 0x04],
    padcfg_qspi_data3: PADCFG_QSPI_DATA3,
}
impl RegisterBlock {
    #[doc = "0x00 - SYS IOMUX CFG SAIF SYSCFG FMUX 0 DOEN"]
    #[inline(always)]
    pub const fn gpo_doen0(&self) -> &GPO_DOEN0 {
        &self.gpo_doen0
    }
    #[doc = "0x04 - SYS IOMUX CFG SAIF SYSCFG FMUX 1 DOEN"]
    #[inline(always)]
    pub const fn gpo_doen1(&self) -> &GPO_DOEN1 {
        &self.gpo_doen1
    }
    #[doc = "0x08 - SYS IOMUX CFG SAIF SYSCFG FMUX 2 DOEN"]
    #[inline(always)]
    pub const fn gpo_doen2(&self) -> &GPO_DOEN2 {
        &self.gpo_doen2
    }
    #[doc = "0x0c - SYS IOMUX CFG SAIF SYSCFG FMUX 3 DOEN"]
    #[inline(always)]
    pub const fn gpo_doen3(&self) -> &GPO_DOEN3 {
        &self.gpo_doen3
    }
    #[doc = "0x10 - SYS IOMUX CFG SAIF SYSCFG FMUX 4 DOEN"]
    #[inline(always)]
    pub const fn gpo_doen4(&self) -> &GPO_DOEN4 {
        &self.gpo_doen4
    }
    #[doc = "0x14 - SYS IOMUX CFG SAIF SYSCFG FMUX 5 DOEN"]
    #[inline(always)]
    pub const fn gpo_doen5(&self) -> &GPO_DOEN5 {
        &self.gpo_doen5
    }
    #[doc = "0x18 - SYS IOMUX CFG SAIF SYSCFG FMUX 6 DOEN"]
    #[inline(always)]
    pub const fn gpo_doen6(&self) -> &GPO_DOEN6 {
        &self.gpo_doen6
    }
    #[doc = "0x1c - SYS IOMUX CFG SAIF SYSCFG FMUX 7 DOEN"]
    #[inline(always)]
    pub const fn gpo_doen7(&self) -> &GPO_DOEN7 {
        &self.gpo_doen7
    }
    #[doc = "0x20 - SYS IOMUX CFG SAIF SYSCFG FMUX 8 DOEN"]
    #[inline(always)]
    pub const fn gpo_doen8(&self) -> &GPO_DOEN8 {
        &self.gpo_doen8
    }
    #[doc = "0x24 - SYS IOMUX CFG SAIF SYSCFG FMUX 9 DOEN"]
    #[inline(always)]
    pub const fn gpo_doen9(&self) -> &GPO_DOEN9 {
        &self.gpo_doen9
    }
    #[doc = "0x28 - SYS IOMUX CFG SAIF SYSCFG FMUX 10 DOEN"]
    #[inline(always)]
    pub const fn gpo_doen10(&self) -> &GPO_DOEN10 {
        &self.gpo_doen10
    }
    #[doc = "0x2c - SYS IOMUX CFG SAIF SYSCFG FMUX 11 DOEN"]
    #[inline(always)]
    pub const fn gpo_doen11(&self) -> &GPO_DOEN11 {
        &self.gpo_doen11
    }
    #[doc = "0x30 - SYS IOMUX CFG SAIF SYSCFG FMUX 12 DOEN"]
    #[inline(always)]
    pub const fn gpo_doen12(&self) -> &GPO_DOEN12 {
        &self.gpo_doen12
    }
    #[doc = "0x34 - SYS IOMUX CFG SAIF SYSCFG FMUX 13 DOEN"]
    #[inline(always)]
    pub const fn gpo_doen13(&self) -> &GPO_DOEN13 {
        &self.gpo_doen13
    }
    #[doc = "0x38 - SYS IOMUX CFGSAIF SYSCFG IOIRQ 56: GPIO Interrupt Edge Trigger Selector"]
    #[inline(always)]
    pub const fn ioirq1(&self) -> &IOIRQ1 {
        unsafe { &*(self as *const Self).cast::<u8>().add(56).cast() }
    }
    #[doc = "0x38 - SYS IOMUX CFG SAIF SYSCFG FMUX 14 DOEN"]
    #[inline(always)]
    pub const fn gpo_doen14(&self) -> &GPO_DOEN14 {
        unsafe { &*(self as *const Self).cast::<u8>().add(56).cast() }
    }
    #[doc = "0x39 - SYS IOMUX CFGSAIF SYSCFG IOIRQ 57: GPIO Interrupt Edge Trigger Selector"]
    #[inline(always)]
    pub const fn ioirq2(&self) -> &IOIRQ2 {
        unsafe { &*(self as *const Self).cast::<u8>().add(57).cast() }
    }
    #[doc = "0x3a - SYS IOMUX CFGSAIF SYSCFG IOIRQ 58: GPIO Interrupt Clear"]
    #[inline(always)]
    pub const fn ioirq3(&self) -> &IOIRQ3 {
        unsafe { &*(self as *const Self).cast::<u8>().add(58).cast() }
    }
    #[doc = "0x3b - SYS IOMUX CFGSAIF SYSCFG IOIRQ 59: GPIO Interrupt Clear"]
    #[inline(always)]
    pub const fn ioirq4(&self) -> &IOIRQ4 {
        unsafe { &*(self as *const Self).cast::<u8>().add(59).cast() }
    }
    #[doc = "0x3c - SYS IOMUX CFGSAIF SYSCFG IOIRQ 60: GPIO Interrupt Both Edge Trigger Selector"]
    #[inline(always)]
    pub const fn ioirq5(&self) -> &IOIRQ5 {
        unsafe { &*(self as *const Self).cast::<u8>().add(60).cast() }
    }
    #[doc = "0x3c - SYS IOMUX CFG SAIF SYSCFG FMUX 15 DOEN"]
    #[inline(always)]
    pub const fn gpo_doen15(&self) -> &GPO_DOEN15 {
        unsafe { &*(self as *const Self).cast::<u8>().add(60).cast() }
    }
    #[doc = "0x3d - SYS IOMUX CFGSAIF SYSCFG IOIRQ 61: GPIO Interrupt Both Edge Trigger Selector"]
    #[inline(always)]
    pub const fn ioirq6(&self) -> &IOIRQ6 {
        unsafe { &*(self as *const Self).cast::<u8>().add(61).cast() }
    }
    #[doc = "0x3e - SYS IOMUX CFGSAIF SYSCFG IOIRQ 62: GPIO Interrupt Edge Value"]
    #[inline(always)]
    pub const fn ioirq7(&self) -> &IOIRQ7 {
        unsafe { &*(self as *const Self).cast::<u8>().add(62).cast() }
    }
    #[doc = "0x3f - SYS IOMUX CFGSAIF SYSCFG IOIRQ 63: GPIO Interrupt Edge Value"]
    #[inline(always)]
    pub const fn ioirq8(&self) -> &IOIRQ8 {
        unsafe { &*(self as *const Self).cast::<u8>().add(63).cast() }
    }
    #[doc = "0x40 - SYS IOMUX CFGSAIF SYSCFG IOIRQ 64: GPIO Interrupt Edge Mask Selector"]
    #[inline(always)]
    pub const fn ioirq9(&self) -> &IOIRQ9 {
        unsafe { &*(self as *const Self).cast::<u8>().add(64).cast() }
    }
    #[doc = "0x40 - SYS IOMUX CFG SAIF SYSCFG FMUX GPIO 0-3 DOUT"]
    #[inline(always)]
    pub const fn gpo_dout0_3(&self) -> &GPO_DOUT0_3 {
        unsafe { &*(self as *const Self).cast::<u8>().add(64).cast() }
    }
    #[doc = "0x41 - SYS IOMUX CFGSAIF SYSCFG IOIRQ 65: GPIO Interrupt Edge Mask Selector"]
    #[inline(always)]
    pub const fn ioirq10(&self) -> &IOIRQ10 {
        unsafe { &*(self as *const Self).cast::<u8>().add(65).cast() }
    }
    #[doc = "0x42 - SYS IOMUX CFGSAIF SYSCFG IOIRQ 66: GPIO Register Interrupt Status"]
    #[inline(always)]
    pub const fn ioirq11(&self) -> &IOIRQ11 {
        unsafe { &*(self as *const Self).cast::<u8>().add(66).cast() }
    }
    #[doc = "0x43 - SYS IOMUX CFGSAIF SYSCFG IOIRQ 67: GPIO Register Interrupt Status"]
    #[inline(always)]
    pub const fn ioirq12(&self) -> &IOIRQ12 {
        unsafe { &*(self as *const Self).cast::<u8>().add(67).cast() }
    }
    #[doc = "0x44 - SYS IOMUX CFGSAIF SYSCFG IOIRQ 68: GPIO Masked Interrupt Status"]
    #[inline(always)]
    pub const fn ioirq13(&self) -> &IOIRQ13 {
        unsafe { &*(self as *const Self).cast::<u8>().add(68).cast() }
    }
    #[doc = "0x44 - SYS IOMUX CFG SAIF SYSCFG FMUX GPIO 4-7 DOUT"]
    #[inline(always)]
    pub const fn gpo_dout4_7(&self) -> &GPO_DOUT4_7 {
        unsafe { &*(self as *const Self).cast::<u8>().add(68).cast() }
    }
    #[doc = "0x45 - SYS IOMUX CFGSAIF SYSCFG IOIRQ 69: GPIO Masked Interrupt Status"]
    #[inline(always)]
    pub const fn ioirq14(&self) -> &IOIRQ14 {
        unsafe { &*(self as *const Self).cast::<u8>().add(69).cast() }
    }
    #[doc = "0x46 - SYS IOMUX CFGSAIF SYSCFG IOIRQ 70: GPIO Synchronization Status"]
    #[inline(always)]
    pub const fn ioirq15(&self) -> &IOIRQ15 {
        unsafe { &*(self as *const Self).cast::<u8>().add(70).cast() }
    }
    #[doc = "0x47 - SYS IOMUX CFGSAIF SYSCFG IOIRQ 71: GPIO Synchronization Status"]
    #[inline(always)]
    pub const fn ioirq16(&self) -> &IOIRQ16 {
        unsafe { &*(self as *const Self).cast::<u8>().add(71).cast() }
    }
    #[doc = "0x48 - SYS IOMUX CFG SAIF SYSCFG FMUX GPIO 8-11 DOUT"]
    #[inline(always)]
    pub const fn gpo_dout8_11(&self) -> &GPO_DOUT8_11 {
        unsafe { &*(self as *const Self).cast::<u8>().add(72).cast() }
    }
    #[doc = "0x4b - GPIO GMAC1 MDC Pad Configuration"]
    #[inline(always)]
    pub const fn padcfg_gmac1_mdc_syscon(&self) -> &PADCFG_GMAC1_MDC_SYSCON {
        unsafe { &*(self as *const Self).cast::<u8>().add(75).cast() }
    }
    #[doc = "0x4c - SYS IOMUX CFG SAIF SYSCFG FMUX GPIO 12-15 DOUT"]
    #[inline(always)]
    pub const fn gpo_dout12_15(&self) -> &GPO_DOUT12_15 {
        unsafe { &*(self as *const Self).cast::<u8>().add(76).cast() }
    }
    #[doc = "0x4f - GPIO GMAC1 MDIO Pad Configuration"]
    #[inline(always)]
    pub const fn padcfg_gmac1_mdio_syscon(&self) -> &PADCFG_GMAC1_MDIO_SYSCON {
        unsafe { &*(self as *const Self).cast::<u8>().add(79).cast() }
    }
    #[doc = "0x50 - SYS IOMUX CFG SAIF SYSCFG FMUX GPIO 16-19 DOUT"]
    #[inline(always)]
    pub const fn gpo_dout16_19(&self) -> &GPO_DOUT16_19 {
        unsafe { &*(self as *const Self).cast::<u8>().add(80).cast() }
    }
    #[doc = "0x53 - GPIO GMAC1 RXD0 Pad Configuration"]
    #[inline(always)]
    pub const fn padcfg_gmac1_rxd0_syscon(&self) -> &PADCFG_GMAC1_RXD0_SYSCON {
        unsafe { &*(self as *const Self).cast::<u8>().add(83).cast() }
    }
    #[doc = "0x54 - SYS IOMUX CFG SAIF SYSCFG FMUX GPIO 20-23 DOUT"]
    #[inline(always)]
    pub const fn gpo_dout20_23(&self) -> &GPO_DOUT20_23 {
        unsafe { &*(self as *const Self).cast::<u8>().add(84).cast() }
    }
    #[doc = "0x57 - GPIO GMAC1 RXD1 Pad Configuration"]
    #[inline(always)]
    pub const fn padcfg_gmac1_rxd1_syscon(&self) -> &PADCFG_GMAC1_RXD1_SYSCON {
        unsafe { &*(self as *const Self).cast::<u8>().add(87).cast() }
    }
    #[doc = "0x58 - SYS IOMUX CFG SAIF SYSCFG FMUX GPIO 24-27 DOUT"]
    #[inline(always)]
    pub const fn gpo_dout24_27(&self) -> &GPO_DOUT24_27 {
        unsafe { &*(self as *const Self).cast::<u8>().add(88).cast() }
    }
    #[doc = "0x5b - GPIO GMAC1 RXD2 Pad Configuration"]
    #[inline(always)]
    pub const fn padcfg_gmac1_rxd2_syscon(&self) -> &PADCFG_GMAC1_RXD2_SYSCON {
        unsafe { &*(self as *const Self).cast::<u8>().add(91).cast() }
    }
    #[doc = "0x5c - SYS IOMUX CFG SAIF SYSCFG FMUX GPIO 28-31 DOUT"]
    #[inline(always)]
    pub const fn gpo_dout28_31(&self) -> &GPO_DOUT28_31 {
        unsafe { &*(self as *const Self).cast::<u8>().add(92).cast() }
    }
    #[doc = "0x5f - GPIO GMAC1 RXD3 Pad Configuration"]
    #[inline(always)]
    pub const fn padcfg_gmac1_rxd3_syscon(&self) -> &PADCFG_GMAC1_RXD3_SYSCON {
        unsafe { &*(self as *const Self).cast::<u8>().add(95).cast() }
    }
    #[doc = "0x60 - SYS IOMUX CFG SAIF SYSCFG FMUX GPIO 32-35 DOUT"]
    #[inline(always)]
    pub const fn gpo_dout32_35(&self) -> &GPO_DOUT32_35 {
        unsafe { &*(self as *const Self).cast::<u8>().add(96).cast() }
    }
    #[doc = "0x63 - GPIO GMAC1 RXDV Pad Configuration"]
    #[inline(always)]
    pub const fn padcfg_gmac1_rxdv_syscon(&self) -> &PADCFG_GMAC1_RXDV_SYSCON {
        unsafe { &*(self as *const Self).cast::<u8>().add(99).cast() }
    }
    #[doc = "0x64 - SYS IOMUX CFG SAIF SYSCFG FMUX GPIO 36-39 DOUT"]
    #[inline(always)]
    pub const fn gpo_dout36_39(&self) -> &GPO_DOUT36_39 {
        unsafe { &*(self as *const Self).cast::<u8>().add(100).cast() }
    }
    #[doc = "0x67 - GPIO GMAC1 RXC Pad Configuration"]
    #[inline(always)]
    pub const fn padcfg_gmac1_rxc_syscon(&self) -> &PADCFG_GMAC1_RXC_SYSCON {
        unsafe { &*(self as *const Self).cast::<u8>().add(103).cast() }
    }
    #[doc = "0x68 - SYS IOMUX CFG SAIF SYSCFG FMUX GPIO 40-43 DOUT"]
    #[inline(always)]
    pub const fn gpo_dout40_43(&self) -> &GPO_DOUT40_43 {
        unsafe { &*(self as *const Self).cast::<u8>().add(104).cast() }
    }
    #[doc = "0x6b - GPIO GMAC1 TXD0 Pad Configuration"]
    #[inline(always)]
    pub const fn padcfg_gmac1_txd0_syscon(&self) -> &PADCFG_GMAC1_TXD0_SYSCON {
        unsafe { &*(self as *const Self).cast::<u8>().add(107).cast() }
    }
    #[doc = "0x6c - SYS IOMUX CFG SAIF SYSCFG FMUX GPIO 44-47 DOUT"]
    #[inline(always)]
    pub const fn gpo_dout44_47(&self) -> &GPO_DOUT44_47 {
        unsafe { &*(self as *const Self).cast::<u8>().add(108).cast() }
    }
    #[doc = "0x6f - GPIO GMAC1 TXD1 Pad Configuration"]
    #[inline(always)]
    pub const fn padcfg_gmac1_txd1_syscon(&self) -> &PADCFG_GMAC1_TXD1_SYSCON {
        unsafe { &*(self as *const Self).cast::<u8>().add(111).cast() }
    }
    #[doc = "0x70 - SYS IOMUX CFG SAIF SYSCFG FMUX GPIO 48-51 DOUT"]
    #[inline(always)]
    pub const fn gpo_dout48_51(&self) -> &GPO_DOUT48_51 {
        unsafe { &*(self as *const Self).cast::<u8>().add(112).cast() }
    }
    #[doc = "0x73 - GPIO GMAC1 TXD2 Pad Configuration"]
    #[inline(always)]
    pub const fn padcfg_gmac1_txd2_syscon(&self) -> &PADCFG_GMAC1_TXD2_SYSCON {
        unsafe { &*(self as *const Self).cast::<u8>().add(115).cast() }
    }
    #[doc = "0x74 - SYS IOMUX CFG SAIF SYSCFG FMUX GPIO 52-55 DOUT"]
    #[inline(always)]
    pub const fn gpo_dout52_55(&self) -> &GPO_DOUT52_55 {
        unsafe { &*(self as *const Self).cast::<u8>().add(116).cast() }
    }
    #[doc = "0x77 - GPIO GMAC1 TXD3 Pad Configuration"]
    #[inline(always)]
    pub const fn padcfg_gmac1_txd3_syscon(&self) -> &PADCFG_GMAC1_TXD3_SYSCON {
        unsafe { &*(self as *const Self).cast::<u8>().add(119).cast() }
    }
    #[doc = "0x78 - SYS IOMUX CFG SAIF SYSCFG FMUX GPIO 56-59 DOUT"]
    #[inline(always)]
    pub const fn gpo_dout56_59(&self) -> &GPO_DOUT56_59 {
        unsafe { &*(self as *const Self).cast::<u8>().add(120).cast() }
    }
    #[doc = "0x7b - GPIO GMAC1 TXEN Pad Configuration"]
    #[inline(always)]
    pub const fn padcfg_gmac1_txen_syscon(&self) -> &PADCFG_GMAC1_TXEN_SYSCON {
        unsafe { &*(self as *const Self).cast::<u8>().add(123).cast() }
    }
    #[doc = "0x7c - SYS IOMUX CFG SAIF SYSCFG FMUX GPIO 60-63 DOUT"]
    #[inline(always)]
    pub const fn gpo_dout60_63(&self) -> &GPO_DOUT60_63 {
        unsafe { &*(self as *const Self).cast::<u8>().add(124).cast() }
    }
    #[doc = "0x7f - GPIO GMAC1 TXC Pad Configuration"]
    #[inline(always)]
    pub const fn padcfg_gmac1_txc_syscon(&self) -> &PADCFG_GMAC1_TXC_SYSCON {
        unsafe { &*(self as *const Self).cast::<u8>().add(127).cast() }
    }
    #[doc = "0x80 - SYS IOMUX CFG SAIF SYSCFG FMUX GPIO GPI 0 - The register can be used to configure the selected GPIO connector number for input signals. The signal name is indicated in the \"Name\" column of the following table per StarFive naming conventions. For example, name \"u0_WAVE511_i_uart_rxsin_cfg\" indicates the corresponding input signal is \"u0_WAVE511.i_uart_rxsin\". See GPIO Input Signals (on page 107) for a complete list of the input GPIO signals."]
    #[inline(always)]
    pub const fn gpi0(&self) -> &GPI0 {
        unsafe { &*(self as *const Self).cast::<u8>().add(128).cast() }
    }
    #[doc = "0x84 - SYS IOMUX CFG SAIF SYSCFG FMUX GPIO GPI 4 - The register can be used to configure the selected GPIO connector number for input signals. The signal name is indicated in the \"Name\" column of the following table per StarFive naming conventions. For example, name \"u0_WAVE511_i_uart_rxsin_cfg\" indicates the corresponding input signal is \"u0_WAVE511.i_uart_rxsin\". See GPIO Input Signals (on page 107) for a complete list of the input GPIO signals."]
    #[inline(always)]
    pub const fn gpi4(&self) -> &GPI4 {
        &self.gpi4
    }
    #[doc = "0x88 - SYS IOMUX CFG SAIF SYSCFG FMUX GPIO GPI 8 - The register can be used to configure the selected GPIO connector number for input signals. The signal name is indicated in the \"Name\" column of the following table per StarFive naming conventions. For example, name \"u0_WAVE511_i_uart_rxsin_cfg\" indicates the corresponding input signal is \"u0_WAVE511.i_uart_rxsin\". See GPIO Input Signals (on page 107) for a complete list of the input GPIO signals."]
    #[inline(always)]
    pub const fn gpi8(&self) -> &GPI8 {
        &self.gpi8
    }
    #[doc = "0x8c - SYS IOMUX CFG SAIF SYSCFG FMUX GPIO GPI 12 - The register can be used to configure the selected GPIO connector number for input signals. The signal name is indicated in the \"Name\" column of the following table per StarFive naming conventions. For example, name \"u0_WAVE511_i_uart_rxsin_cfg\" indicates the corresponding input signal is \"u0_WAVE511.i_uart_rxsin\". See GPIO Input Signals (on page 107) for a complete list of the input GPIO signals."]
    #[inline(always)]
    pub const fn gpi12(&self) -> &GPI12 {
        &self.gpi12
    }
    #[doc = "0x90 - SYS IOMUX CFG SAIF SYSCFG FMUX GPIO GPI 16 - The register can be used to configure the selected GPIO connector number for input signals. The signal name is indicated in the \"Name\" column of the following table per StarFive naming conventions. For example, name \"u0_WAVE511_i_uart_rxsin_cfg\" indicates the corresponding input signal is \"u0_WAVE511.i_uart_rxsin\". See GPIO Input Signals (on page 107) for a complete list of the input GPIO signals."]
    #[inline(always)]
    pub const fn gpi16(&self) -> &GPI16 {
        &self.gpi16
    }
    #[doc = "0x94 - SYS IOMUX CFG SAIF SYSCFG FMUX GPIO GPI 20 - The register can be used to configure the selected GPIO connector number for input signals. The signal name is indicated in the \"Name\" column of the following table per StarFive naming conventions. For example, name \"u0_WAVE511_i_uart_rxsin_cfg\" indicates the corresponding input signal is \"u0_WAVE511.i_uart_rxsin\". See GPIO Input Signals (on page 107) for a complete list of the input GPIO signals."]
    #[inline(always)]
    pub const fn gpi20(&self) -> &GPI20 {
        &self.gpi20
    }
    #[doc = "0x98 - SYS IOMUX CFG SAIF SYSCFG FMUX GPIO GPI 24 - The register can be used to configure the selected GPIO connector number for input signals. The signal name is indicated in the \"Name\" column of the following table per StarFive naming conventions. For example, name \"u0_WAVE511_i_uart_rxsin_cfg\" indicates the corresponding input signal is \"u0_WAVE511.i_uart_rxsin\". See GPIO Input Signals (on page 107) for a complete list of the input GPIO signals."]
    #[inline(always)]
    pub const fn gpi24(&self) -> &GPI24 {
        &self.gpi24
    }
    #[doc = "0x9c - SYS IOMUX CFG SAIF SYSCFG FMUX GPIO GPI 28 - The register can be used to configure the selected GPIO connector number for input signals. The signal name is indicated in the \"Name\" column of the following table per StarFive naming conventions. For example, name \"u0_WAVE511_i_uart_rxsin_cfg\" indicates the corresponding input signal is \"u0_WAVE511.i_uart_rxsin\". See GPIO Input Signals (on page 107) for a complete list of the input GPIO signals."]
    #[inline(always)]
    pub const fn gpi28(&self) -> &GPI28 {
        &self.gpi28
    }
    #[doc = "0xa0 - SYS IOMUX CFG SAIF SYSCFG FMUX GPIO GPI 32 - The register can be used to configure the selected GPIO connector number for input signals. The signal name is indicated in the \"Name\" column of the following table per StarFive naming conventions. For example, name \"u0_WAVE511_i_uart_rxsin_cfg\" indicates the corresponding input signal is \"u0_WAVE511.i_uart_rxsin\". See GPIO Input Signals (on page 107) for a complete list of the input GPIO signals."]
    #[inline(always)]
    pub const fn gpi32(&self) -> &GPI32 {
        &self.gpi32
    }
    #[doc = "0xa4 - SYS IOMUX CFG SAIF SYSCFG FMUX GPIO GPI 36 - The register can be used to configure the selected GPIO connector number for input signals. The signal name is indicated in the \"Name\" column of the following table per StarFive naming conventions. For example, name \"u0_WAVE511_i_uart_rxsin_cfg\" indicates the corresponding input signal is \"u0_WAVE511.i_uart_rxsin\". See GPIO Input Signals (on page 107) for a complete list of the input GPIO signals."]
    #[inline(always)]
    pub const fn gpi36(&self) -> &GPI36 {
        &self.gpi36
    }
    #[doc = "0xa8 - SYS IOMUX CFG SAIF SYSCFG FMUX GPIO GPI 40 - The register can be used to configure the selected GPIO connector number for input signals. The signal name is indicated in the \"Name\" column of the following table per StarFive naming conventions. For example, name \"u0_WAVE511_i_uart_rxsin_cfg\" indicates the corresponding input signal is \"u0_WAVE511.i_uart_rxsin\". See GPIO Input Signals (on page 107) for a complete list of the input GPIO signals."]
    #[inline(always)]
    pub const fn gpi40(&self) -> &GPI40 {
        &self.gpi40
    }
    #[doc = "0xac - SYS IOMUX CFG SAIF SYSCFG FMUX GPIO GPI 44 - The register can be used to configure the selected GPIO connector number for input signals. The signal name is indicated in the \"Name\" column of the following table per StarFive naming conventions. For example, name \"u0_WAVE511_i_uart_rxsin_cfg\" indicates the corresponding input signal is \"u0_WAVE511.i_uart_rxsin\". See GPIO Input Signals (on page 107) for a complete list of the input GPIO signals."]
    #[inline(always)]
    pub const fn gpi44(&self) -> &GPI44 {
        &self.gpi44
    }
    #[doc = "0xb0 - SYS IOMUX CFG SAIF SYSCFG FMUX GPIO GPI 48 - The register can be used to configure the selected GPIO connector number for input signals. The signal name is indicated in the \"Name\" column of the following table per StarFive naming conventions. For example, name \"u0_WAVE511_i_uart_rxsin_cfg\" indicates the corresponding input signal is \"u0_WAVE511.i_uart_rxsin\". See GPIO Input Signals (on page 107) for a complete list of the input GPIO signals."]
    #[inline(always)]
    pub const fn gpi48(&self) -> &GPI48 {
        &self.gpi48
    }
    #[doc = "0xb4 - SYS IOMUX CFG SAIF SYSCFG FMUX GPIO GPI 52 - The register can be used to configure the selected GPIO connector number for input signals. The signal name is indicated in the \"Name\" column of the following table per StarFive naming conventions. For example, name \"u0_WAVE511_i_uart_rxsin_cfg\" indicates the corresponding input signal is \"u0_WAVE511.i_uart_rxsin\". See GPIO Input Signals (on page 107) for a complete list of the input GPIO signals."]
    #[inline(always)]
    pub const fn gpi52(&self) -> &GPI52 {
        &self.gpi52
    }
    #[doc = "0xb8 - SYS IOMUX CFG SAIF SYSCFG FMUX GPIO GPI 56 - The register can be used to configure the selected GPIO connector number for input signals. The signal name is indicated in the \"Name\" column of the following table per StarFive naming conventions. For example, name \"u0_WAVE511_i_uart_rxsin_cfg\" indicates the corresponding input signal is \"u0_WAVE511.i_uart_rxsin\". See GPIO Input Signals (on page 107) for a complete list of the input GPIO signals."]
    #[inline(always)]
    pub const fn gpi56(&self) -> &GPI56 {
        &self.gpi56
    }
    #[doc = "0xbc - SYS IOMUX CFG SAIF SYSCFG FMUX GPIO GPI 60 - The register can be used to configure the selected GPIO connector number for input signals. The signal name is indicated in the \"Name\" column of the following table per StarFive naming conventions. For example, name \"u0_WAVE511_i_uart_rxsin_cfg\" indicates the corresponding input signal is \"u0_WAVE511.i_uart_rxsin\". See GPIO Input Signals (on page 107) for a complete list of the input GPIO signals."]
    #[inline(always)]
    pub const fn gpi60(&self) -> &GPI60 {
        &self.gpi60
    }
    #[doc = "0xc0 - SYS IOMUX CFG SAIF SYSCFG FMUX GPIO GPI 64 - The register can be used to configure the selected GPIO connector number for input signals. The signal name is indicated in the \"Name\" column of the following table per StarFive naming conventions. For example, name \"u0_WAVE511_i_uart_rxsin_cfg\" indicates the corresponding input signal is \"u0_WAVE511.i_uart_rxsin\". See GPIO Input Signals (on page 107) for a complete list of the input GPIO signals."]
    #[inline(always)]
    pub const fn gpi64(&self) -> &GPI64 {
        &self.gpi64
    }
    #[doc = "0xc4 - SYS IOMUX CFG SAIF SYSCFG FMUX GPIO GPI 68 - The register can be used to configure the selected GPIO connector number for input signals. The signal name is indicated in the \"Name\" column of the following table per StarFive naming conventions. For example, name \"u0_WAVE511_i_uart_rxsin_cfg\" indicates the corresponding input signal is \"u0_WAVE511.i_uart_rxsin\". See GPIO Input Signals (on page 107) for a complete list of the input GPIO signals."]
    #[inline(always)]
    pub const fn gpi68(&self) -> &GPI68 {
        &self.gpi68
    }
    #[doc = "0xc8 - SYS IOMUX CFG SAIF SYSCFG FMUX GPIO GPI 72 - The register can be used to configure the selected GPIO connector number for input signals. The signal name is indicated in the \"Name\" column of the following table per StarFive naming conventions. For example, name \"u0_WAVE511_i_uart_rxsin_cfg\" indicates the corresponding input signal is \"u0_WAVE511.i_uart_rxsin\". See GPIO Input Signals (on page 107) for a complete list of the input GPIO signals."]
    #[inline(always)]
    pub const fn gpi72(&self) -> &GPI72 {
        &self.gpi72
    }
    #[doc = "0xcc - SYS IOMUX CFG SAIF SYSCFG FMUX GPIO GPI 76 - The register can be used to configure the selected GPIO connector number for input signals. The signal name is indicated in the \"Name\" column of the following table per StarFive naming conventions. For example, name \"u0_WAVE511_i_uart_rxsin_cfg\" indicates the corresponding input signal is \"u0_WAVE511.i_uart_rxsin\". See GPIO Input Signals (on page 107) for a complete list of the input GPIO signals."]
    #[inline(always)]
    pub const fn gpi76(&self) -> &GPI76 {
        &self.gpi76
    }
    #[doc = "0xd0 - SYS IOMUX CFG SAIF SYSCFG FMUX GPIO GPI 80 - The register can be used to configure the selected GPIO connector number for input signals. The signal name is indicated in the \"Name\" column of the following table per StarFive naming conventions. For example, name \"u0_WAVE511_i_uart_rxsin_cfg\" indicates the corresponding input signal is \"u0_WAVE511.i_uart_rxsin\". See GPIO Input Signals (on page 107) for a complete list of the input GPIO signals."]
    #[inline(always)]
    pub const fn gpi80(&self) -> &GPI80 {
        &self.gpi80
    }
    #[doc = "0xd4 - SYS IOMUX CFG SAIF SYSCFG FMUX GPIO GPI 84 - The register can be used to configure the selected GPIO connector number for input signals. The signal name is indicated in the \"Name\" column of the following table per StarFive naming conventions. For example, name \"u0_WAVE511_i_uart_rxsin_cfg\" indicates the corresponding input signal is \"u0_WAVE511.i_uart_rxsin\". See GPIO Input Signals (on page 107) for a complete list of the input GPIO signals."]
    #[inline(always)]
    pub const fn gpi84(&self) -> &GPI84 {
        &self.gpi84
    }
    #[doc = "0xd8 - SYS IOMUX CFG SAIF SYSCFG FMUX GPIO GPI 88 - The register can be used to configure the selected GPIO connector number for input signals. The signal name is indicated in the \"Name\" column of the following table per StarFive naming conventions. For example, name \"u0_WAVE511_i_uart_rxsin_cfg\" indicates the corresponding input signal is \"u0_WAVE511.i_uart_rxsin\". See GPIO Input Signals (on page 107) for a complete list of the input GPIO signals."]
    #[inline(always)]
    pub const fn gpi88(&self) -> &GPI88 {
        &self.gpi88
    }
    #[doc = "0xdc - Enable GPIO IRQ function"]
    #[inline(always)]
    pub const fn ioirq0(&self) -> &IOIRQ0 {
        &self.ioirq0
    }
    #[doc = "0x120 - SYS IOMUX CFG SAIF SYSCFG PADCFG 288: GPIO0"]
    #[inline(always)]
    pub const fn padcfg_gpio0(&self) -> &PADCFG_GPIO0 {
        &self.padcfg_gpio0
    }
    #[doc = "0x124 - SYS IOMUX CFG SAIF SYSCFG PADCFG 292: GPIO1"]
    #[inline(always)]
    pub const fn padcfg_gpio1(&self) -> &PADCFG_GPIO1 {
        &self.padcfg_gpio1
    }
    #[doc = "0x128 - SYS IOMUX CFG SAIF SYSCFG PADCFG 296: GPIO2"]
    #[inline(always)]
    pub const fn padcfg_gpio2(&self) -> &PADCFG_GPIO2 {
        &self.padcfg_gpio2
    }
    #[doc = "0x12c - SYS IOMUX CFG SAIF SYSCFG PADCFG 300: GPIO3"]
    #[inline(always)]
    pub const fn padcfg_gpio3(&self) -> &PADCFG_GPIO3 {
        &self.padcfg_gpio3
    }
    #[doc = "0x130 - SYS IOMUX CFG SAIF SYSCFG PADCFG 304: GPIO4"]
    #[inline(always)]
    pub const fn padcfg_gpio4(&self) -> &PADCFG_GPIO4 {
        &self.padcfg_gpio4
    }
    #[doc = "0x134 - SYS IOMUX CFG SAIF SYSCFG PADCFG 308: GPIO5"]
    #[inline(always)]
    pub const fn padcfg_gpio5(&self) -> &PADCFG_GPIO5 {
        &self.padcfg_gpio5
    }
    #[doc = "0x138 - SYS IOMUX CFG SAIF SYSCFG PADCFG 312: GPIO6"]
    #[inline(always)]
    pub const fn padcfg_gpio6(&self) -> &PADCFG_GPIO6 {
        &self.padcfg_gpio6
    }
    #[doc = "0x13c - SYS IOMUX CFG SAIF SYSCFG PADCFG 316: GPIO7"]
    #[inline(always)]
    pub const fn padcfg_gpio7(&self) -> &PADCFG_GPIO7 {
        &self.padcfg_gpio7
    }
    #[doc = "0x140 - SYS IOMUX CFG SAIF SYSCFG PADCFG 320: GPIO8"]
    #[inline(always)]
    pub const fn padcfg_gpio8(&self) -> &PADCFG_GPIO8 {
        &self.padcfg_gpio8
    }
    #[doc = "0x144 - SYS IOMUX CFG SAIF SYSCFG PADCFG 324: GPIO9"]
    #[inline(always)]
    pub const fn padcfg_gpio9(&self) -> &PADCFG_GPIO9 {
        &self.padcfg_gpio9
    }
    #[doc = "0x148 - SYS IOMUX CFG SAIF SYSCFG PADCFG 328: GPIO10"]
    #[inline(always)]
    pub const fn padcfg_gpio10(&self) -> &PADCFG_GPIO10 {
        &self.padcfg_gpio10
    }
    #[doc = "0x14c - SYS IOMUX CFG SAIF SYSCFG PADCFG 332: GPIO11"]
    #[inline(always)]
    pub const fn padcfg_gpio11(&self) -> &PADCFG_GPIO11 {
        &self.padcfg_gpio11
    }
    #[doc = "0x150 - SYS IOMUX CFG SAIF SYSCFG PADCFG 336: GPIO12"]
    #[inline(always)]
    pub const fn padcfg_gpio12(&self) -> &PADCFG_GPIO12 {
        &self.padcfg_gpio12
    }
    #[doc = "0x154 - SYS IOMUX CFG SAIF SYSCFG PADCFG 340: GPIO13"]
    #[inline(always)]
    pub const fn padcfg_gpio13(&self) -> &PADCFG_GPIO13 {
        &self.padcfg_gpio13
    }
    #[doc = "0x158 - SYS IOMUX CFG SAIF SYSCFG PADCFG 344: GPIO14"]
    #[inline(always)]
    pub const fn padcfg_gpio14(&self) -> &PADCFG_GPIO14 {
        &self.padcfg_gpio14
    }
    #[doc = "0x15c - SYS IOMUX CFG SAIF SYSCFG PADCFG 348: GPIO15"]
    #[inline(always)]
    pub const fn padcfg_gpio15(&self) -> &PADCFG_GPIO15 {
        &self.padcfg_gpio15
    }
    #[doc = "0x160 - SYS IOMUX CFG SAIF SYSCFG PADCFG 352: GPIO16"]
    #[inline(always)]
    pub const fn padcfg_gpio16(&self) -> &PADCFG_GPIO16 {
        &self.padcfg_gpio16
    }
    #[doc = "0x164 - SYS IOMUX CFG SAIF SYSCFG PADCFG 356: GPIO17"]
    #[inline(always)]
    pub const fn padcfg_gpio17(&self) -> &PADCFG_GPIO17 {
        &self.padcfg_gpio17
    }
    #[doc = "0x168 - SYS IOMUX CFG SAIF SYSCFG PADCFG 360: GPIO18"]
    #[inline(always)]
    pub const fn padcfg_gpio18(&self) -> &PADCFG_GPIO18 {
        &self.padcfg_gpio18
    }
    #[doc = "0x16c - SYS IOMUX CFG SAIF SYSCFG PADCFG 364: GPIO19"]
    #[inline(always)]
    pub const fn padcfg_gpio19(&self) -> &PADCFG_GPIO19 {
        &self.padcfg_gpio19
    }
    #[doc = "0x170 - SYS IOMUX CFG SAIF SYSCFG PADCFG 368: GPIO20"]
    #[inline(always)]
    pub const fn padcfg_gpio20(&self) -> &PADCFG_GPIO20 {
        &self.padcfg_gpio20
    }
    #[doc = "0x174 - SYS IOMUX CFG SAIF SYSCFG PADCFG 372: GPIO21"]
    #[inline(always)]
    pub const fn padcfg_gpio21(&self) -> &PADCFG_GPIO21 {
        &self.padcfg_gpio21
    }
    #[doc = "0x178 - SYS IOMUX CFG SAIF SYSCFG PADCFG 376: GPIO22"]
    #[inline(always)]
    pub const fn padcfg_gpio22(&self) -> &PADCFG_GPIO22 {
        &self.padcfg_gpio22
    }
    #[doc = "0x17c - SYS IOMUX CFG SAIF SYSCFG PADCFG 380: GPIO23"]
    #[inline(always)]
    pub const fn padcfg_gpio23(&self) -> &PADCFG_GPIO23 {
        &self.padcfg_gpio23
    }
    #[doc = "0x180 - SYS IOMUX CFG SAIF SYSCFG PADCFG 384: GPIO24"]
    #[inline(always)]
    pub const fn padcfg_gpio24(&self) -> &PADCFG_GPIO24 {
        &self.padcfg_gpio24
    }
    #[doc = "0x184 - SYS IOMUX CFG SAIF SYSCFG PADCFG 388: GPIO25"]
    #[inline(always)]
    pub const fn padcfg_gpio25(&self) -> &PADCFG_GPIO25 {
        &self.padcfg_gpio25
    }
    #[doc = "0x188 - SYS IOMUX CFG SAIF SYSCFG PADCFG 392: GPIO26"]
    #[inline(always)]
    pub const fn padcfg_gpio26(&self) -> &PADCFG_GPIO26 {
        &self.padcfg_gpio26
    }
    #[doc = "0x18c - SYS IOMUX CFG SAIF SYSCFG PADCFG 396: GPIO27"]
    #[inline(always)]
    pub const fn padcfg_gpio27(&self) -> &PADCFG_GPIO27 {
        &self.padcfg_gpio27
    }
    #[doc = "0x190 - SYS IOMUX CFG SAIF SYSCFG PADCFG 400: GPIO28"]
    #[inline(always)]
    pub const fn padcfg_gpio28(&self) -> &PADCFG_GPIO28 {
        &self.padcfg_gpio28
    }
    #[doc = "0x194 - SYS IOMUX CFG SAIF SYSCFG PADCFG 404: GPIO29"]
    #[inline(always)]
    pub const fn padcfg_gpio29(&self) -> &PADCFG_GPIO29 {
        &self.padcfg_gpio29
    }
    #[doc = "0x198 - SYS IOMUX CFG SAIF SYSCFG PADCFG 408: GPIO30"]
    #[inline(always)]
    pub const fn padcfg_gpio30(&self) -> &PADCFG_GPIO30 {
        &self.padcfg_gpio30
    }
    #[doc = "0x19c - SYS IOMUX CFG SAIF SYSCFG PADCFG 412: GPIO31"]
    #[inline(always)]
    pub const fn padcfg_gpio31(&self) -> &PADCFG_GPIO31 {
        &self.padcfg_gpio31
    }
    #[doc = "0x1a0 - SYS IOMUX CFG SAIF SYSCFG PADCFG 416: GPIO32"]
    #[inline(always)]
    pub const fn padcfg_gpio32(&self) -> &PADCFG_GPIO32 {
        &self.padcfg_gpio32
    }
    #[doc = "0x1a4 - SYS IOMUX CFG SAIF SYSCFG PADCFG 420: GPIO33"]
    #[inline(always)]
    pub const fn padcfg_gpio33(&self) -> &PADCFG_GPIO33 {
        &self.padcfg_gpio33
    }
    #[doc = "0x1a8 - SYS IOMUX CFG SAIF SYSCFG PADCFG 424: GPIO34"]
    #[inline(always)]
    pub const fn padcfg_gpio34(&self) -> &PADCFG_GPIO34 {
        &self.padcfg_gpio34
    }
    #[doc = "0x1ac - SYS IOMUX CFG SAIF SYSCFG PADCFG 428: GPIO35"]
    #[inline(always)]
    pub const fn padcfg_gpio35(&self) -> &PADCFG_GPIO35 {
        &self.padcfg_gpio35
    }
    #[doc = "0x1b0 - SYS IOMUX CFG SAIF SYSCFG PADCFG 432: GPIO36"]
    #[inline(always)]
    pub const fn padcfg_gpio36(&self) -> &PADCFG_GPIO36 {
        &self.padcfg_gpio36
    }
    #[doc = "0x1b4 - SYS IOMUX CFG SAIF SYSCFG PADCFG 436: GPIO37"]
    #[inline(always)]
    pub const fn padcfg_gpio37(&self) -> &PADCFG_GPIO37 {
        &self.padcfg_gpio37
    }
    #[doc = "0x1b8 - SYS IOMUX CFG SAIF SYSCFG PADCFG 440: GPIO38"]
    #[inline(always)]
    pub const fn padcfg_gpio38(&self) -> &PADCFG_GPIO38 {
        &self.padcfg_gpio38
    }
    #[doc = "0x1bc - SYS IOMUX CFG SAIF SYSCFG PADCFG 444: GPIO39"]
    #[inline(always)]
    pub const fn padcfg_gpio39(&self) -> &PADCFG_GPIO39 {
        &self.padcfg_gpio39
    }
    #[doc = "0x1c0 - SYS IOMUX CFG SAIF SYSCFG PADCFG 448: GPIO40"]
    #[inline(always)]
    pub const fn padcfg_gpio40(&self) -> &PADCFG_GPIO40 {
        &self.padcfg_gpio40
    }
    #[doc = "0x1c4 - SYS IOMUX CFG SAIF SYSCFG PADCFG 452: GPIO41"]
    #[inline(always)]
    pub const fn padcfg_gpio41(&self) -> &PADCFG_GPIO41 {
        &self.padcfg_gpio41
    }
    #[doc = "0x1c8 - SYS IOMUX CFG SAIF SYSCFG PADCFG 456: GPIO42"]
    #[inline(always)]
    pub const fn padcfg_gpio42(&self) -> &PADCFG_GPIO42 {
        &self.padcfg_gpio42
    }
    #[doc = "0x1cc - SYS IOMUX CFG SAIF SYSCFG PADCFG 460: GPIO43"]
    #[inline(always)]
    pub const fn padcfg_gpio43(&self) -> &PADCFG_GPIO43 {
        &self.padcfg_gpio43
    }
    #[doc = "0x1d0 - SYS IOMUX CFG SAIF SYSCFG PADCFG 464: GPIO44"]
    #[inline(always)]
    pub const fn padcfg_gpio44(&self) -> &PADCFG_GPIO44 {
        &self.padcfg_gpio44
    }
    #[doc = "0x1d4 - SYS IOMUX CFG SAIF SYSCFG PADCFG 468: GPIO45"]
    #[inline(always)]
    pub const fn padcfg_gpio45(&self) -> &PADCFG_GPIO45 {
        &self.padcfg_gpio45
    }
    #[doc = "0x1d8 - SYS IOMUX CFG SAIF SYSCFG PADCFG 472: GPIO46"]
    #[inline(always)]
    pub const fn padcfg_gpio46(&self) -> &PADCFG_GPIO46 {
        &self.padcfg_gpio46
    }
    #[doc = "0x1dc - SYS IOMUX CFG SAIF SYSCFG PADCFG 476: GPIO47"]
    #[inline(always)]
    pub const fn padcfg_gpio47(&self) -> &PADCFG_GPIO47 {
        &self.padcfg_gpio47
    }
    #[doc = "0x1e0 - SYS IOMUX CFG SAIF SYSCFG PADCFG 480: GPIO48"]
    #[inline(always)]
    pub const fn padcfg_gpio48(&self) -> &PADCFG_GPIO48 {
        &self.padcfg_gpio48
    }
    #[doc = "0x1e4 - SYS IOMUX CFG SAIF SYSCFG PADCFG 484: GPIO49"]
    #[inline(always)]
    pub const fn padcfg_gpio49(&self) -> &PADCFG_GPIO49 {
        &self.padcfg_gpio49
    }
    #[doc = "0x1e8 - SYS IOMUX CFG SAIF SYSCFG PADCFG 488: GPIO50"]
    #[inline(always)]
    pub const fn padcfg_gpio50(&self) -> &PADCFG_GPIO50 {
        &self.padcfg_gpio50
    }
    #[doc = "0x1ec - SYS IOMUX CFG SAIF SYSCFG PADCFG 492: GPIO51"]
    #[inline(always)]
    pub const fn padcfg_gpio51(&self) -> &PADCFG_GPIO51 {
        &self.padcfg_gpio51
    }
    #[doc = "0x1f0 - SYS IOMUX CFG SAIF SYSCFG PADCFG 496: GPIO52"]
    #[inline(always)]
    pub const fn padcfg_gpio52(&self) -> &PADCFG_GPIO52 {
        &self.padcfg_gpio52
    }
    #[doc = "0x1f4 - SYS IOMUX CFG SAIF SYSCFG PADCFG 500: GPIO53"]
    #[inline(always)]
    pub const fn padcfg_gpio53(&self) -> &PADCFG_GPIO53 {
        &self.padcfg_gpio53
    }
    #[doc = "0x1f8 - SYS IOMUX CFG SAIF SYSCFG PADCFG 504: GPIO54"]
    #[inline(always)]
    pub const fn padcfg_gpio54(&self) -> &PADCFG_GPIO54 {
        &self.padcfg_gpio54
    }
    #[doc = "0x1fc - SYS IOMUX CFG SAIF SYSCFG PADCFG 508: GPIO55"]
    #[inline(always)]
    pub const fn padcfg_gpio55(&self) -> &PADCFG_GPIO55 {
        &self.padcfg_gpio55
    }
    #[doc = "0x200 - SYS IOMUX CFG SAIF SYSCFG PADCFG 512: GPIO56"]
    #[inline(always)]
    pub const fn padcfg_gpio56(&self) -> &PADCFG_GPIO56 {
        &self.padcfg_gpio56
    }
    #[doc = "0x204 - SYS IOMUX CFG SAIF SYSCFG PADCFG 516: GPIO57"]
    #[inline(always)]
    pub const fn padcfg_gpio57(&self) -> &PADCFG_GPIO57 {
        &self.padcfg_gpio57
    }
    #[doc = "0x208 - SYS IOMUX CFG SAIF SYSCFG PADCFG 520: GPIO58"]
    #[inline(always)]
    pub const fn padcfg_gpio58(&self) -> &PADCFG_GPIO58 {
        &self.padcfg_gpio58
    }
    #[doc = "0x20c - SYS IOMUX CFG SAIF SYSCFG PADCFG 524: GPIO59"]
    #[inline(always)]
    pub const fn padcfg_gpio59(&self) -> &PADCFG_GPIO59 {
        &self.padcfg_gpio59
    }
    #[doc = "0x210 - SYS IOMUX CFG SAIF SYSCFG PADCFG 528: GPIO60"]
    #[inline(always)]
    pub const fn padcfg_gpio60(&self) -> &PADCFG_GPIO60 {
        &self.padcfg_gpio60
    }
    #[doc = "0x214 - SYS IOMUX CFG SAIF SYSCFG PADCFG 532: GPIO61"]
    #[inline(always)]
    pub const fn padcfg_gpio61(&self) -> &PADCFG_GPIO61 {
        &self.padcfg_gpio61
    }
    #[doc = "0x218 - SYS IOMUX CFG SAIF SYSCFG PADCFG 536: GPIO62"]
    #[inline(always)]
    pub const fn padcfg_gpio62(&self) -> &PADCFG_GPIO62 {
        &self.padcfg_gpio62
    }
    #[doc = "0x21c - SYS IOMUX CFG SAIF SYSCFG PADCFG 540: GPIO63"]
    #[inline(always)]
    pub const fn padcfg_gpio63(&self) -> &PADCFG_GPIO63 {
        &self.padcfg_gpio63
    }
    #[doc = "0x220 - SYS IOMUX CFG SAIF SYSCFG PADCFG 544: SD0_CLK"]
    #[inline(always)]
    pub const fn padcfg_sd0_clk(&self) -> &PADCFG_SD0_CLK {
        &self.padcfg_sd0_clk
    }
    #[doc = "0x224 - SYS IOMUX CFG SAIF SYSCFG PADCFG 548: SD0_CMD"]
    #[inline(always)]
    pub const fn padcfg_sd0_cmd(&self) -> &PADCFG_SD0_CMD {
        &self.padcfg_sd0_cmd
    }
    #[doc = "0x228 - SYS IOMUX CFG SAIF SYSCFG PADCFG 552: SD0_DATA0"]
    #[inline(always)]
    pub const fn padcfg_sd0_data0(&self) -> &PADCFG_SD0_DATA0 {
        &self.padcfg_sd0_data0
    }
    #[doc = "0x238 - SYS IOMUX CFG SAIF SYSCFG PADCFG 568: SD0_DATA1"]
    #[inline(always)]
    pub const fn padcfg_sd0_data1(&self) -> &PADCFG_SD0_DATA1 {
        &self.padcfg_sd0_data1
    }
    #[doc = "0x248 - SYS IOMUX CFG SAIF SYSCFG PADCFG 584: SD0_STRB"]
    #[inline(always)]
    pub const fn padcfg_sd0_strb(&self) -> &PADCFG_SD0_STRB {
        unsafe { &*(self as *const Self).cast::<u8>().add(584).cast() }
    }
    #[doc = "0x248 - SYS IOMUX CFG SAIF SYSCFG PADCFG 584: SD0_DATA2"]
    #[inline(always)]
    pub const fn padcfg_sd0_data2(&self) -> &PADCFG_SD0_DATA2 {
        unsafe { &*(self as *const Self).cast::<u8>().add(584).cast() }
    }
    #[doc = "0x258 - SYS IOMUX CFG SAIF SYSCFG PADCFG 600: SD0_DATA3"]
    #[inline(always)]
    pub const fn padcfg_sd0_data3(&self) -> &PADCFG_SD0_DATA3 {
        &self.padcfg_sd0_data3
    }
    #[doc = "0x268 - SYS IOMUX CFG SAIF SYSCFG PADCFG 616: SD0_DATA4"]
    #[inline(always)]
    pub const fn padcfg_sd0_data4(&self) -> &PADCFG_SD0_DATA4 {
        &self.padcfg_sd0_data4
    }
    #[doc = "0x278 - SYS IOMUX CFG SAIF SYSCFG PADCFG 632: SD0_DATA5"]
    #[inline(always)]
    pub const fn padcfg_sd0_data5(&self) -> &PADCFG_SD0_DATA5 {
        &self.padcfg_sd0_data5
    }
    #[doc = "0x284 - SYS IOMUX CFG SAIF SYSCFG PADCFG 644: QSPI_SCLK"]
    #[inline(always)]
    pub const fn padcfg_qspi_sclk(&self) -> &PADCFG_QSPI_SCLK {
        &self.padcfg_qspi_sclk
    }
    #[doc = "0x288 - SYS IOMUX CFG SAIF SYSCFG PADCFG 648: QSPI_CSN0"]
    #[inline(always)]
    pub const fn padcfg_qspi_csn0(&self) -> &PADCFG_QSPI_CSN0 {
        unsafe { &*(self as *const Self).cast::<u8>().add(648).cast() }
    }
    #[doc = "0x288 - SYS IOMUX CFG SAIF SYSCFG PADCFG 648: SD0_DATA6"]
    #[inline(always)]
    pub const fn padcfg_sd0_data6(&self) -> &PADCFG_SD0_DATA6 {
        unsafe { &*(self as *const Self).cast::<u8>().add(648).cast() }
    }
    #[doc = "0x28c - SYS IOMUX CFG SAIF SYSCFG PADCFG 652: QSPI_DATA0"]
    #[inline(always)]
    pub const fn padcfg_qspi_data0(&self) -> &PADCFG_QSPI_DATA0 {
        &self.padcfg_qspi_data0
    }
    #[doc = "0x298 - SYS IOMUX CFG SAIF SYSCFG PADCFG 664: SD0_DATA7"]
    #[inline(always)]
    pub const fn padcfg_sd0_data7(&self) -> &PADCFG_SD0_DATA7 {
        &self.padcfg_sd0_data7
    }
    #[doc = "0x29c - SYS IOMUX CFG SAIF SYSCFG 0"]
    #[inline(always)]
    pub const fn func_sel0(&self) -> &FUNC_SEL0 {
        unsafe { &*(self as *const Self).cast::<u8>().add(668).cast() }
    }
    #[doc = "0x29c - SYS IOMUX CFG SAIF SYSCFG PADCFG 668: QSPI_DATA1"]
    #[inline(always)]
    pub const fn padcfg_qspi_data1(&self) -> &PADCFG_QSPI_DATA1 {
        unsafe { &*(self as *const Self).cast::<u8>().add(668).cast() }
    }
    #[doc = "0x2a0 - SYS IOMUX CFG SAIF SYSCFG 1"]
    #[inline(always)]
    pub const fn func_sel1(&self) -> &FUNC_SEL1 {
        &self.func_sel1
    }
    #[doc = "0x2a4 - SYS IOMUX CFG SAIF SYSCFG 2"]
    #[inline(always)]
    pub const fn func_sel2(&self) -> &FUNC_SEL2 {
        &self.func_sel2
    }
    #[doc = "0x2a8 - SYS IOMUX CFG SAIF SYSCFG 3"]
    #[inline(always)]
    pub const fn func_sel3(&self) -> &FUNC_SEL3 {
        &self.func_sel3
    }
    #[doc = "0x2ac - SYS IOMUX CFG SAIF SYSCFG 4"]
    #[inline(always)]
    pub const fn func_sel4(&self) -> &FUNC_SEL4 {
        unsafe { &*(self as *const Self).cast::<u8>().add(684).cast() }
    }
    #[doc = "0x2ac - SYS IOMUX CFG SAIF SYSCFG PADCFG 684: QSPI_DATA2"]
    #[inline(always)]
    pub const fn padcfg_qspi_data2(&self) -> &PADCFG_QSPI_DATA2 {
        unsafe { &*(self as *const Self).cast::<u8>().add(684).cast() }
    }
    #[doc = "0x2b0 - SYS IOMUX CFG SAIF SYSCFG 5"]
    #[inline(always)]
    pub const fn func_sel5(&self) -> &FUNC_SEL5 {
        &self.func_sel5
    }
    #[doc = "0x2b4 - SYS IOMUX CFG SAIF SYSCFG 6"]
    #[inline(always)]
    pub const fn func_sel6(&self) -> &FUNC_SEL6 {
        &self.func_sel6
    }
    #[doc = "0x2bc - SYS IOMUX CFG SAIF SYSCFG PADCFG 700: QSPI_DATA3"]
    #[inline(always)]
    pub const fn padcfg_qspi_data3(&self) -> &PADCFG_QSPI_DATA3 {
        &self.padcfg_qspi_data3
    }
}
#[doc = "gpo_doen0 (rw) register accessor: SYS IOMUX CFG SAIF SYSCFG FMUX 0 DOEN\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`gpo_doen0::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`gpo_doen0::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@gpo_doen0`]
module"]
pub type GPO_DOEN0 = crate::Reg<gpo_doen0::GPO_DOEN0_SPEC>;
#[doc = "SYS IOMUX CFG SAIF SYSCFG FMUX 0 DOEN"]
pub mod gpo_doen0;
#[doc = "gpo_doen1 (rw) register accessor: SYS IOMUX CFG SAIF SYSCFG FMUX 1 DOEN\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`gpo_doen1::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`gpo_doen1::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@gpo_doen1`]
module"]
pub type GPO_DOEN1 = crate::Reg<gpo_doen1::GPO_DOEN1_SPEC>;
#[doc = "SYS IOMUX CFG SAIF SYSCFG FMUX 1 DOEN"]
pub mod gpo_doen1;
#[doc = "gpo_doen2 (rw) register accessor: SYS IOMUX CFG SAIF SYSCFG FMUX 2 DOEN\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`gpo_doen2::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`gpo_doen2::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@gpo_doen2`]
module"]
pub type GPO_DOEN2 = crate::Reg<gpo_doen2::GPO_DOEN2_SPEC>;
#[doc = "SYS IOMUX CFG SAIF SYSCFG FMUX 2 DOEN"]
pub mod gpo_doen2;
#[doc = "gpo_doen3 (rw) register accessor: SYS IOMUX CFG SAIF SYSCFG FMUX 3 DOEN\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`gpo_doen3::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`gpo_doen3::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@gpo_doen3`]
module"]
pub type GPO_DOEN3 = crate::Reg<gpo_doen3::GPO_DOEN3_SPEC>;
#[doc = "SYS IOMUX CFG SAIF SYSCFG FMUX 3 DOEN"]
pub mod gpo_doen3;
#[doc = "gpo_doen4 (rw) register accessor: SYS IOMUX CFG SAIF SYSCFG FMUX 4 DOEN\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`gpo_doen4::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`gpo_doen4::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@gpo_doen4`]
module"]
pub type GPO_DOEN4 = crate::Reg<gpo_doen4::GPO_DOEN4_SPEC>;
#[doc = "SYS IOMUX CFG SAIF SYSCFG FMUX 4 DOEN"]
pub mod gpo_doen4;
#[doc = "gpo_doen5 (rw) register accessor: SYS IOMUX CFG SAIF SYSCFG FMUX 5 DOEN\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`gpo_doen5::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`gpo_doen5::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@gpo_doen5`]
module"]
pub type GPO_DOEN5 = crate::Reg<gpo_doen5::GPO_DOEN5_SPEC>;
#[doc = "SYS IOMUX CFG SAIF SYSCFG FMUX 5 DOEN"]
pub mod gpo_doen5;
#[doc = "gpo_doen6 (rw) register accessor: SYS IOMUX CFG SAIF SYSCFG FMUX 6 DOEN\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`gpo_doen6::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`gpo_doen6::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@gpo_doen6`]
module"]
pub type GPO_DOEN6 = crate::Reg<gpo_doen6::GPO_DOEN6_SPEC>;
#[doc = "SYS IOMUX CFG SAIF SYSCFG FMUX 6 DOEN"]
pub mod gpo_doen6;
#[doc = "gpo_doen7 (rw) register accessor: SYS IOMUX CFG SAIF SYSCFG FMUX 7 DOEN\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`gpo_doen7::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`gpo_doen7::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@gpo_doen7`]
module"]
pub type GPO_DOEN7 = crate::Reg<gpo_doen7::GPO_DOEN7_SPEC>;
#[doc = "SYS IOMUX CFG SAIF SYSCFG FMUX 7 DOEN"]
pub mod gpo_doen7;
#[doc = "gpo_doen8 (rw) register accessor: SYS IOMUX CFG SAIF SYSCFG FMUX 8 DOEN\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`gpo_doen8::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`gpo_doen8::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@gpo_doen8`]
module"]
pub type GPO_DOEN8 = crate::Reg<gpo_doen8::GPO_DOEN8_SPEC>;
#[doc = "SYS IOMUX CFG SAIF SYSCFG FMUX 8 DOEN"]
pub mod gpo_doen8;
#[doc = "gpo_doen9 (rw) register accessor: SYS IOMUX CFG SAIF SYSCFG FMUX 9 DOEN\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`gpo_doen9::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`gpo_doen9::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@gpo_doen9`]
module"]
pub type GPO_DOEN9 = crate::Reg<gpo_doen9::GPO_DOEN9_SPEC>;
#[doc = "SYS IOMUX CFG SAIF SYSCFG FMUX 9 DOEN"]
pub mod gpo_doen9;
#[doc = "gpo_doen10 (rw) register accessor: SYS IOMUX CFG SAIF SYSCFG FMUX 10 DOEN\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`gpo_doen10::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`gpo_doen10::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@gpo_doen10`]
module"]
pub type GPO_DOEN10 = crate::Reg<gpo_doen10::GPO_DOEN10_SPEC>;
#[doc = "SYS IOMUX CFG SAIF SYSCFG FMUX 10 DOEN"]
pub mod gpo_doen10;
#[doc = "gpo_doen11 (rw) register accessor: SYS IOMUX CFG SAIF SYSCFG FMUX 11 DOEN\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`gpo_doen11::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`gpo_doen11::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@gpo_doen11`]
module"]
pub type GPO_DOEN11 = crate::Reg<gpo_doen11::GPO_DOEN11_SPEC>;
#[doc = "SYS IOMUX CFG SAIF SYSCFG FMUX 11 DOEN"]
pub mod gpo_doen11;
#[doc = "gpo_doen12 (rw) register accessor: SYS IOMUX CFG SAIF SYSCFG FMUX 12 DOEN\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`gpo_doen12::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`gpo_doen12::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@gpo_doen12`]
module"]
pub type GPO_DOEN12 = crate::Reg<gpo_doen12::GPO_DOEN12_SPEC>;
#[doc = "SYS IOMUX CFG SAIF SYSCFG FMUX 12 DOEN"]
pub mod gpo_doen12;
#[doc = "gpo_doen13 (rw) register accessor: SYS IOMUX CFG SAIF SYSCFG FMUX 13 DOEN\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`gpo_doen13::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`gpo_doen13::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@gpo_doen13`]
module"]
pub type GPO_DOEN13 = crate::Reg<gpo_doen13::GPO_DOEN13_SPEC>;
#[doc = "SYS IOMUX CFG SAIF SYSCFG FMUX 13 DOEN"]
pub mod gpo_doen13;
#[doc = "gpo_doen14 (rw) register accessor: SYS IOMUX CFG SAIF SYSCFG FMUX 14 DOEN\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`gpo_doen14::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`gpo_doen14::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@gpo_doen14`]
module"]
pub type GPO_DOEN14 = crate::Reg<gpo_doen14::GPO_DOEN14_SPEC>;
#[doc = "SYS IOMUX CFG SAIF SYSCFG FMUX 14 DOEN"]
pub mod gpo_doen14;
#[doc = "gpo_doen15 (rw) register accessor: SYS IOMUX CFG SAIF SYSCFG FMUX 15 DOEN\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`gpo_doen15::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`gpo_doen15::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@gpo_doen15`]
module"]
pub type GPO_DOEN15 = crate::Reg<gpo_doen15::GPO_DOEN15_SPEC>;
#[doc = "SYS IOMUX CFG SAIF SYSCFG FMUX 15 DOEN"]
pub mod gpo_doen15;
#[doc = "gpo_dout0_3 (rw) register accessor: SYS IOMUX CFG SAIF SYSCFG FMUX GPIO 0-3 DOUT\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`gpo_dout0_3::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`gpo_dout0_3::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@gpo_dout0_3`]
module"]
pub type GPO_DOUT0_3 = crate::Reg<gpo_dout0_3::GPO_DOUT0_3_SPEC>;
#[doc = "SYS IOMUX CFG SAIF SYSCFG FMUX GPIO 0-3 DOUT"]
pub mod gpo_dout0_3;
#[doc = "gpo_dout4_7 (rw) register accessor: SYS IOMUX CFG SAIF SYSCFG FMUX GPIO 4-7 DOUT\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`gpo_dout4_7::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`gpo_dout4_7::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@gpo_dout4_7`]
module"]
pub type GPO_DOUT4_7 = crate::Reg<gpo_dout4_7::GPO_DOUT4_7_SPEC>;
#[doc = "SYS IOMUX CFG SAIF SYSCFG FMUX GPIO 4-7 DOUT"]
pub mod gpo_dout4_7;
#[doc = "gpo_dout8_11 (rw) register accessor: SYS IOMUX CFG SAIF SYSCFG FMUX GPIO 8-11 DOUT\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`gpo_dout8_11::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`gpo_dout8_11::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@gpo_dout8_11`]
module"]
pub type GPO_DOUT8_11 = crate::Reg<gpo_dout8_11::GPO_DOUT8_11_SPEC>;
#[doc = "SYS IOMUX CFG SAIF SYSCFG FMUX GPIO 8-11 DOUT"]
pub mod gpo_dout8_11;
#[doc = "gpo_dout12_15 (rw) register accessor: SYS IOMUX CFG SAIF SYSCFG FMUX GPIO 12-15 DOUT\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`gpo_dout12_15::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`gpo_dout12_15::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@gpo_dout12_15`]
module"]
pub type GPO_DOUT12_15 = crate::Reg<gpo_dout12_15::GPO_DOUT12_15_SPEC>;
#[doc = "SYS IOMUX CFG SAIF SYSCFG FMUX GPIO 12-15 DOUT"]
pub mod gpo_dout12_15;
#[doc = "gpo_dout16_19 (rw) register accessor: SYS IOMUX CFG SAIF SYSCFG FMUX GPIO 16-19 DOUT\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`gpo_dout16_19::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`gpo_dout16_19::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@gpo_dout16_19`]
module"]
pub type GPO_DOUT16_19 = crate::Reg<gpo_dout16_19::GPO_DOUT16_19_SPEC>;
#[doc = "SYS IOMUX CFG SAIF SYSCFG FMUX GPIO 16-19 DOUT"]
pub mod gpo_dout16_19;
#[doc = "gpo_dout20_23 (rw) register accessor: SYS IOMUX CFG SAIF SYSCFG FMUX GPIO 20-23 DOUT\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`gpo_dout20_23::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`gpo_dout20_23::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@gpo_dout20_23`]
module"]
pub type GPO_DOUT20_23 = crate::Reg<gpo_dout20_23::GPO_DOUT20_23_SPEC>;
#[doc = "SYS IOMUX CFG SAIF SYSCFG FMUX GPIO 20-23 DOUT"]
pub mod gpo_dout20_23;
#[doc = "gpo_dout24_27 (rw) register accessor: SYS IOMUX CFG SAIF SYSCFG FMUX GPIO 24-27 DOUT\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`gpo_dout24_27::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`gpo_dout24_27::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@gpo_dout24_27`]
module"]
pub type GPO_DOUT24_27 = crate::Reg<gpo_dout24_27::GPO_DOUT24_27_SPEC>;
#[doc = "SYS IOMUX CFG SAIF SYSCFG FMUX GPIO 24-27 DOUT"]
pub mod gpo_dout24_27;
#[doc = "gpo_dout28_31 (rw) register accessor: SYS IOMUX CFG SAIF SYSCFG FMUX GPIO 28-31 DOUT\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`gpo_dout28_31::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`gpo_dout28_31::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@gpo_dout28_31`]
module"]
pub type GPO_DOUT28_31 = crate::Reg<gpo_dout28_31::GPO_DOUT28_31_SPEC>;
#[doc = "SYS IOMUX CFG SAIF SYSCFG FMUX GPIO 28-31 DOUT"]
pub mod gpo_dout28_31;
#[doc = "gpo_dout32_35 (rw) register accessor: SYS IOMUX CFG SAIF SYSCFG FMUX GPIO 32-35 DOUT\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`gpo_dout32_35::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`gpo_dout32_35::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@gpo_dout32_35`]
module"]
pub type GPO_DOUT32_35 = crate::Reg<gpo_dout32_35::GPO_DOUT32_35_SPEC>;
#[doc = "SYS IOMUX CFG SAIF SYSCFG FMUX GPIO 32-35 DOUT"]
pub mod gpo_dout32_35;
#[doc = "gpo_dout36_39 (rw) register accessor: SYS IOMUX CFG SAIF SYSCFG FMUX GPIO 36-39 DOUT\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`gpo_dout36_39::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`gpo_dout36_39::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@gpo_dout36_39`]
module"]
pub type GPO_DOUT36_39 = crate::Reg<gpo_dout36_39::GPO_DOUT36_39_SPEC>;
#[doc = "SYS IOMUX CFG SAIF SYSCFG FMUX GPIO 36-39 DOUT"]
pub mod gpo_dout36_39;
#[doc = "gpo_dout40_43 (rw) register accessor: SYS IOMUX CFG SAIF SYSCFG FMUX GPIO 40-43 DOUT\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`gpo_dout40_43::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`gpo_dout40_43::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@gpo_dout40_43`]
module"]
pub type GPO_DOUT40_43 = crate::Reg<gpo_dout40_43::GPO_DOUT40_43_SPEC>;
#[doc = "SYS IOMUX CFG SAIF SYSCFG FMUX GPIO 40-43 DOUT"]
pub mod gpo_dout40_43;
#[doc = "gpo_dout44_47 (rw) register accessor: SYS IOMUX CFG SAIF SYSCFG FMUX GPIO 44-47 DOUT\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`gpo_dout44_47::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`gpo_dout44_47::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@gpo_dout44_47`]
module"]
pub type GPO_DOUT44_47 = crate::Reg<gpo_dout44_47::GPO_DOUT44_47_SPEC>;
#[doc = "SYS IOMUX CFG SAIF SYSCFG FMUX GPIO 44-47 DOUT"]
pub mod gpo_dout44_47;
#[doc = "gpo_dout48_51 (rw) register accessor: SYS IOMUX CFG SAIF SYSCFG FMUX GPIO 48-51 DOUT\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`gpo_dout48_51::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`gpo_dout48_51::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@gpo_dout48_51`]
module"]
pub type GPO_DOUT48_51 = crate::Reg<gpo_dout48_51::GPO_DOUT48_51_SPEC>;
#[doc = "SYS IOMUX CFG SAIF SYSCFG FMUX GPIO 48-51 DOUT"]
pub mod gpo_dout48_51;
#[doc = "gpo_dout52_55 (rw) register accessor: SYS IOMUX CFG SAIF SYSCFG FMUX GPIO 52-55 DOUT\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`gpo_dout52_55::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`gpo_dout52_55::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@gpo_dout52_55`]
module"]
pub type GPO_DOUT52_55 = crate::Reg<gpo_dout52_55::GPO_DOUT52_55_SPEC>;
#[doc = "SYS IOMUX CFG SAIF SYSCFG FMUX GPIO 52-55 DOUT"]
pub mod gpo_dout52_55;
#[doc = "gpo_dout56_59 (rw) register accessor: SYS IOMUX CFG SAIF SYSCFG FMUX GPIO 56-59 DOUT\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`gpo_dout56_59::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`gpo_dout56_59::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@gpo_dout56_59`]
module"]
pub type GPO_DOUT56_59 = crate::Reg<gpo_dout56_59::GPO_DOUT56_59_SPEC>;
#[doc = "SYS IOMUX CFG SAIF SYSCFG FMUX GPIO 56-59 DOUT"]
pub mod gpo_dout56_59;
#[doc = "gpo_dout60_63 (rw) register accessor: SYS IOMUX CFG SAIF SYSCFG FMUX GPIO 60-63 DOUT\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`gpo_dout60_63::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`gpo_dout60_63::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@gpo_dout60_63`]
module"]
pub type GPO_DOUT60_63 = crate::Reg<gpo_dout60_63::GPO_DOUT60_63_SPEC>;
#[doc = "SYS IOMUX CFG SAIF SYSCFG FMUX GPIO 60-63 DOUT"]
pub mod gpo_dout60_63;
#[doc = "gpi0 (rw) register accessor: SYS IOMUX CFG SAIF SYSCFG FMUX GPIO GPI 0 - The register can be used to configure the selected GPIO connector number for input signals. The signal name is indicated in the \"Name\" column of the following table per StarFive naming conventions. For example, name \"u0_WAVE511_i_uart_rxsin_cfg\" indicates the corresponding input signal is \"u0_WAVE511.i_uart_rxsin\". See GPIO Input Signals (on page 107) for a complete list of the input GPIO signals.\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`gpi0::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`gpi0::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@gpi0`]
module"]
pub type GPI0 = crate::Reg<gpi0::GPI0_SPEC>;
#[doc = "SYS IOMUX CFG SAIF SYSCFG FMUX GPIO GPI 0 - The register can be used to configure the selected GPIO connector number for input signals. The signal name is indicated in the \"Name\" column of the following table per StarFive naming conventions. For example, name \"u0_WAVE511_i_uart_rxsin_cfg\" indicates the corresponding input signal is \"u0_WAVE511.i_uart_rxsin\". See GPIO Input Signals (on page 107) for a complete list of the input GPIO signals."]
pub mod gpi0;
#[doc = "gpi4 (rw) register accessor: SYS IOMUX CFG SAIF SYSCFG FMUX GPIO GPI 4 - The register can be used to configure the selected GPIO connector number for input signals. The signal name is indicated in the \"Name\" column of the following table per StarFive naming conventions. For example, name \"u0_WAVE511_i_uart_rxsin_cfg\" indicates the corresponding input signal is \"u0_WAVE511.i_uart_rxsin\". See GPIO Input Signals (on page 107) for a complete list of the input GPIO signals.\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`gpi4::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`gpi4::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@gpi4`]
module"]
pub type GPI4 = crate::Reg<gpi4::GPI4_SPEC>;
#[doc = "SYS IOMUX CFG SAIF SYSCFG FMUX GPIO GPI 4 - The register can be used to configure the selected GPIO connector number for input signals. The signal name is indicated in the \"Name\" column of the following table per StarFive naming conventions. For example, name \"u0_WAVE511_i_uart_rxsin_cfg\" indicates the corresponding input signal is \"u0_WAVE511.i_uart_rxsin\". See GPIO Input Signals (on page 107) for a complete list of the input GPIO signals."]
pub mod gpi4;
#[doc = "gpi8 (rw) register accessor: SYS IOMUX CFG SAIF SYSCFG FMUX GPIO GPI 8 - The register can be used to configure the selected GPIO connector number for input signals. The signal name is indicated in the \"Name\" column of the following table per StarFive naming conventions. For example, name \"u0_WAVE511_i_uart_rxsin_cfg\" indicates the corresponding input signal is \"u0_WAVE511.i_uart_rxsin\". See GPIO Input Signals (on page 107) for a complete list of the input GPIO signals.\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`gpi8::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`gpi8::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@gpi8`]
module"]
pub type GPI8 = crate::Reg<gpi8::GPI8_SPEC>;
#[doc = "SYS IOMUX CFG SAIF SYSCFG FMUX GPIO GPI 8 - The register can be used to configure the selected GPIO connector number for input signals. The signal name is indicated in the \"Name\" column of the following table per StarFive naming conventions. For example, name \"u0_WAVE511_i_uart_rxsin_cfg\" indicates the corresponding input signal is \"u0_WAVE511.i_uart_rxsin\". See GPIO Input Signals (on page 107) for a complete list of the input GPIO signals."]
pub mod gpi8;
#[doc = "gpi12 (rw) register accessor: SYS IOMUX CFG SAIF SYSCFG FMUX GPIO GPI 12 - The register can be used to configure the selected GPIO connector number for input signals. The signal name is indicated in the \"Name\" column of the following table per StarFive naming conventions. For example, name \"u0_WAVE511_i_uart_rxsin_cfg\" indicates the corresponding input signal is \"u0_WAVE511.i_uart_rxsin\". See GPIO Input Signals (on page 107) for a complete list of the input GPIO signals.\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`gpi12::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`gpi12::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@gpi12`]
module"]
pub type GPI12 = crate::Reg<gpi12::GPI12_SPEC>;
#[doc = "SYS IOMUX CFG SAIF SYSCFG FMUX GPIO GPI 12 - The register can be used to configure the selected GPIO connector number for input signals. The signal name is indicated in the \"Name\" column of the following table per StarFive naming conventions. For example, name \"u0_WAVE511_i_uart_rxsin_cfg\" indicates the corresponding input signal is \"u0_WAVE511.i_uart_rxsin\". See GPIO Input Signals (on page 107) for a complete list of the input GPIO signals."]
pub mod gpi12;
#[doc = "gpi16 (rw) register accessor: SYS IOMUX CFG SAIF SYSCFG FMUX GPIO GPI 16 - The register can be used to configure the selected GPIO connector number for input signals. The signal name is indicated in the \"Name\" column of the following table per StarFive naming conventions. For example, name \"u0_WAVE511_i_uart_rxsin_cfg\" indicates the corresponding input signal is \"u0_WAVE511.i_uart_rxsin\". See GPIO Input Signals (on page 107) for a complete list of the input GPIO signals.\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`gpi16::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`gpi16::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@gpi16`]
module"]
pub type GPI16 = crate::Reg<gpi16::GPI16_SPEC>;
#[doc = "SYS IOMUX CFG SAIF SYSCFG FMUX GPIO GPI 16 - The register can be used to configure the selected GPIO connector number for input signals. The signal name is indicated in the \"Name\" column of the following table per StarFive naming conventions. For example, name \"u0_WAVE511_i_uart_rxsin_cfg\" indicates the corresponding input signal is \"u0_WAVE511.i_uart_rxsin\". See GPIO Input Signals (on page 107) for a complete list of the input GPIO signals."]
pub mod gpi16;
#[doc = "gpi20 (rw) register accessor: SYS IOMUX CFG SAIF SYSCFG FMUX GPIO GPI 20 - The register can be used to configure the selected GPIO connector number for input signals. The signal name is indicated in the \"Name\" column of the following table per StarFive naming conventions. For example, name \"u0_WAVE511_i_uart_rxsin_cfg\" indicates the corresponding input signal is \"u0_WAVE511.i_uart_rxsin\". See GPIO Input Signals (on page 107) for a complete list of the input GPIO signals.\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`gpi20::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`gpi20::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@gpi20`]
module"]
pub type GPI20 = crate::Reg<gpi20::GPI20_SPEC>;
#[doc = "SYS IOMUX CFG SAIF SYSCFG FMUX GPIO GPI 20 - The register can be used to configure the selected GPIO connector number for input signals. The signal name is indicated in the \"Name\" column of the following table per StarFive naming conventions. For example, name \"u0_WAVE511_i_uart_rxsin_cfg\" indicates the corresponding input signal is \"u0_WAVE511.i_uart_rxsin\". See GPIO Input Signals (on page 107) for a complete list of the input GPIO signals."]
pub mod gpi20;
#[doc = "gpi24 (rw) register accessor: SYS IOMUX CFG SAIF SYSCFG FMUX GPIO GPI 24 - The register can be used to configure the selected GPIO connector number for input signals. The signal name is indicated in the \"Name\" column of the following table per StarFive naming conventions. For example, name \"u0_WAVE511_i_uart_rxsin_cfg\" indicates the corresponding input signal is \"u0_WAVE511.i_uart_rxsin\". See GPIO Input Signals (on page 107) for a complete list of the input GPIO signals.\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`gpi24::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`gpi24::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@gpi24`]
module"]
pub type GPI24 = crate::Reg<gpi24::GPI24_SPEC>;
#[doc = "SYS IOMUX CFG SAIF SYSCFG FMUX GPIO GPI 24 - The register can be used to configure the selected GPIO connector number for input signals. The signal name is indicated in the \"Name\" column of the following table per StarFive naming conventions. For example, name \"u0_WAVE511_i_uart_rxsin_cfg\" indicates the corresponding input signal is \"u0_WAVE511.i_uart_rxsin\". See GPIO Input Signals (on page 107) for a complete list of the input GPIO signals."]
pub mod gpi24;
#[doc = "gpi28 (rw) register accessor: SYS IOMUX CFG SAIF SYSCFG FMUX GPIO GPI 28 - The register can be used to configure the selected GPIO connector number for input signals. The signal name is indicated in the \"Name\" column of the following table per StarFive naming conventions. For example, name \"u0_WAVE511_i_uart_rxsin_cfg\" indicates the corresponding input signal is \"u0_WAVE511.i_uart_rxsin\". See GPIO Input Signals (on page 107) for a complete list of the input GPIO signals.\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`gpi28::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`gpi28::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@gpi28`]
module"]
pub type GPI28 = crate::Reg<gpi28::GPI28_SPEC>;
#[doc = "SYS IOMUX CFG SAIF SYSCFG FMUX GPIO GPI 28 - The register can be used to configure the selected GPIO connector number for input signals. The signal name is indicated in the \"Name\" column of the following table per StarFive naming conventions. For example, name \"u0_WAVE511_i_uart_rxsin_cfg\" indicates the corresponding input signal is \"u0_WAVE511.i_uart_rxsin\". See GPIO Input Signals (on page 107) for a complete list of the input GPIO signals."]
pub mod gpi28;
#[doc = "gpi32 (rw) register accessor: SYS IOMUX CFG SAIF SYSCFG FMUX GPIO GPI 32 - The register can be used to configure the selected GPIO connector number for input signals. The signal name is indicated in the \"Name\" column of the following table per StarFive naming conventions. For example, name \"u0_WAVE511_i_uart_rxsin_cfg\" indicates the corresponding input signal is \"u0_WAVE511.i_uart_rxsin\". See GPIO Input Signals (on page 107) for a complete list of the input GPIO signals.\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`gpi32::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`gpi32::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@gpi32`]
module"]
pub type GPI32 = crate::Reg<gpi32::GPI32_SPEC>;
#[doc = "SYS IOMUX CFG SAIF SYSCFG FMUX GPIO GPI 32 - The register can be used to configure the selected GPIO connector number for input signals. The signal name is indicated in the \"Name\" column of the following table per StarFive naming conventions. For example, name \"u0_WAVE511_i_uart_rxsin_cfg\" indicates the corresponding input signal is \"u0_WAVE511.i_uart_rxsin\". See GPIO Input Signals (on page 107) for a complete list of the input GPIO signals."]
pub mod gpi32;
#[doc = "gpi36 (rw) register accessor: SYS IOMUX CFG SAIF SYSCFG FMUX GPIO GPI 36 - The register can be used to configure the selected GPIO connector number for input signals. The signal name is indicated in the \"Name\" column of the following table per StarFive naming conventions. For example, name \"u0_WAVE511_i_uart_rxsin_cfg\" indicates the corresponding input signal is \"u0_WAVE511.i_uart_rxsin\". See GPIO Input Signals (on page 107) for a complete list of the input GPIO signals.\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`gpi36::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`gpi36::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@gpi36`]
module"]
pub type GPI36 = crate::Reg<gpi36::GPI36_SPEC>;
#[doc = "SYS IOMUX CFG SAIF SYSCFG FMUX GPIO GPI 36 - The register can be used to configure the selected GPIO connector number for input signals. The signal name is indicated in the \"Name\" column of the following table per StarFive naming conventions. For example, name \"u0_WAVE511_i_uart_rxsin_cfg\" indicates the corresponding input signal is \"u0_WAVE511.i_uart_rxsin\". See GPIO Input Signals (on page 107) for a complete list of the input GPIO signals."]
pub mod gpi36;
#[doc = "gpi40 (rw) register accessor: SYS IOMUX CFG SAIF SYSCFG FMUX GPIO GPI 40 - The register can be used to configure the selected GPIO connector number for input signals. The signal name is indicated in the \"Name\" column of the following table per StarFive naming conventions. For example, name \"u0_WAVE511_i_uart_rxsin_cfg\" indicates the corresponding input signal is \"u0_WAVE511.i_uart_rxsin\". See GPIO Input Signals (on page 107) for a complete list of the input GPIO signals.\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`gpi40::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`gpi40::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@gpi40`]
module"]
pub type GPI40 = crate::Reg<gpi40::GPI40_SPEC>;
#[doc = "SYS IOMUX CFG SAIF SYSCFG FMUX GPIO GPI 40 - The register can be used to configure the selected GPIO connector number for input signals. The signal name is indicated in the \"Name\" column of the following table per StarFive naming conventions. For example, name \"u0_WAVE511_i_uart_rxsin_cfg\" indicates the corresponding input signal is \"u0_WAVE511.i_uart_rxsin\". See GPIO Input Signals (on page 107) for a complete list of the input GPIO signals."]
pub mod gpi40;
#[doc = "gpi44 (rw) register accessor: SYS IOMUX CFG SAIF SYSCFG FMUX GPIO GPI 44 - The register can be used to configure the selected GPIO connector number for input signals. The signal name is indicated in the \"Name\" column of the following table per StarFive naming conventions. For example, name \"u0_WAVE511_i_uart_rxsin_cfg\" indicates the corresponding input signal is \"u0_WAVE511.i_uart_rxsin\". See GPIO Input Signals (on page 107) for a complete list of the input GPIO signals.\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`gpi44::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`gpi44::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@gpi44`]
module"]
pub type GPI44 = crate::Reg<gpi44::GPI44_SPEC>;
#[doc = "SYS IOMUX CFG SAIF SYSCFG FMUX GPIO GPI 44 - The register can be used to configure the selected GPIO connector number for input signals. The signal name is indicated in the \"Name\" column of the following table per StarFive naming conventions. For example, name \"u0_WAVE511_i_uart_rxsin_cfg\" indicates the corresponding input signal is \"u0_WAVE511.i_uart_rxsin\". See GPIO Input Signals (on page 107) for a complete list of the input GPIO signals."]
pub mod gpi44;
#[doc = "gpi48 (rw) register accessor: SYS IOMUX CFG SAIF SYSCFG FMUX GPIO GPI 48 - The register can be used to configure the selected GPIO connector number for input signals. The signal name is indicated in the \"Name\" column of the following table per StarFive naming conventions. For example, name \"u0_WAVE511_i_uart_rxsin_cfg\" indicates the corresponding input signal is \"u0_WAVE511.i_uart_rxsin\". See GPIO Input Signals (on page 107) for a complete list of the input GPIO signals.\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`gpi48::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`gpi48::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@gpi48`]
module"]
pub type GPI48 = crate::Reg<gpi48::GPI48_SPEC>;
#[doc = "SYS IOMUX CFG SAIF SYSCFG FMUX GPIO GPI 48 - The register can be used to configure the selected GPIO connector number for input signals. The signal name is indicated in the \"Name\" column of the following table per StarFive naming conventions. For example, name \"u0_WAVE511_i_uart_rxsin_cfg\" indicates the corresponding input signal is \"u0_WAVE511.i_uart_rxsin\". See GPIO Input Signals (on page 107) for a complete list of the input GPIO signals."]
pub mod gpi48;
#[doc = "gpi52 (rw) register accessor: SYS IOMUX CFG SAIF SYSCFG FMUX GPIO GPI 52 - The register can be used to configure the selected GPIO connector number for input signals. The signal name is indicated in the \"Name\" column of the following table per StarFive naming conventions. For example, name \"u0_WAVE511_i_uart_rxsin_cfg\" indicates the corresponding input signal is \"u0_WAVE511.i_uart_rxsin\". See GPIO Input Signals (on page 107) for a complete list of the input GPIO signals.\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`gpi52::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`gpi52::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@gpi52`]
module"]
pub type GPI52 = crate::Reg<gpi52::GPI52_SPEC>;
#[doc = "SYS IOMUX CFG SAIF SYSCFG FMUX GPIO GPI 52 - The register can be used to configure the selected GPIO connector number for input signals. The signal name is indicated in the \"Name\" column of the following table per StarFive naming conventions. For example, name \"u0_WAVE511_i_uart_rxsin_cfg\" indicates the corresponding input signal is \"u0_WAVE511.i_uart_rxsin\". See GPIO Input Signals (on page 107) for a complete list of the input GPIO signals."]
pub mod gpi52;
#[doc = "gpi56 (rw) register accessor: SYS IOMUX CFG SAIF SYSCFG FMUX GPIO GPI 56 - The register can be used to configure the selected GPIO connector number for input signals. The signal name is indicated in the \"Name\" column of the following table per StarFive naming conventions. For example, name \"u0_WAVE511_i_uart_rxsin_cfg\" indicates the corresponding input signal is \"u0_WAVE511.i_uart_rxsin\". See GPIO Input Signals (on page 107) for a complete list of the input GPIO signals.\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`gpi56::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`gpi56::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@gpi56`]
module"]
pub type GPI56 = crate::Reg<gpi56::GPI56_SPEC>;
#[doc = "SYS IOMUX CFG SAIF SYSCFG FMUX GPIO GPI 56 - The register can be used to configure the selected GPIO connector number for input signals. The signal name is indicated in the \"Name\" column of the following table per StarFive naming conventions. For example, name \"u0_WAVE511_i_uart_rxsin_cfg\" indicates the corresponding input signal is \"u0_WAVE511.i_uart_rxsin\". See GPIO Input Signals (on page 107) for a complete list of the input GPIO signals."]
pub mod gpi56;
#[doc = "gpi60 (rw) register accessor: SYS IOMUX CFG SAIF SYSCFG FMUX GPIO GPI 60 - The register can be used to configure the selected GPIO connector number for input signals. The signal name is indicated in the \"Name\" column of the following table per StarFive naming conventions. For example, name \"u0_WAVE511_i_uart_rxsin_cfg\" indicates the corresponding input signal is \"u0_WAVE511.i_uart_rxsin\". See GPIO Input Signals (on page 107) for a complete list of the input GPIO signals.\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`gpi60::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`gpi60::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@gpi60`]
module"]
pub type GPI60 = crate::Reg<gpi60::GPI60_SPEC>;
#[doc = "SYS IOMUX CFG SAIF SYSCFG FMUX GPIO GPI 60 - The register can be used to configure the selected GPIO connector number for input signals. The signal name is indicated in the \"Name\" column of the following table per StarFive naming conventions. For example, name \"u0_WAVE511_i_uart_rxsin_cfg\" indicates the corresponding input signal is \"u0_WAVE511.i_uart_rxsin\". See GPIO Input Signals (on page 107) for a complete list of the input GPIO signals."]
pub mod gpi60;
#[doc = "gpi64 (rw) register accessor: SYS IOMUX CFG SAIF SYSCFG FMUX GPIO GPI 64 - The register can be used to configure the selected GPIO connector number for input signals. The signal name is indicated in the \"Name\" column of the following table per StarFive naming conventions. For example, name \"u0_WAVE511_i_uart_rxsin_cfg\" indicates the corresponding input signal is \"u0_WAVE511.i_uart_rxsin\". See GPIO Input Signals (on page 107) for a complete list of the input GPIO signals.\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`gpi64::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`gpi64::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@gpi64`]
module"]
pub type GPI64 = crate::Reg<gpi64::GPI64_SPEC>;
#[doc = "SYS IOMUX CFG SAIF SYSCFG FMUX GPIO GPI 64 - The register can be used to configure the selected GPIO connector number for input signals. The signal name is indicated in the \"Name\" column of the following table per StarFive naming conventions. For example, name \"u0_WAVE511_i_uart_rxsin_cfg\" indicates the corresponding input signal is \"u0_WAVE511.i_uart_rxsin\". See GPIO Input Signals (on page 107) for a complete list of the input GPIO signals."]
pub mod gpi64;
#[doc = "gpi68 (rw) register accessor: SYS IOMUX CFG SAIF SYSCFG FMUX GPIO GPI 68 - The register can be used to configure the selected GPIO connector number for input signals. The signal name is indicated in the \"Name\" column of the following table per StarFive naming conventions. For example, name \"u0_WAVE511_i_uart_rxsin_cfg\" indicates the corresponding input signal is \"u0_WAVE511.i_uart_rxsin\". See GPIO Input Signals (on page 107) for a complete list of the input GPIO signals.\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`gpi68::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`gpi68::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@gpi68`]
module"]
pub type GPI68 = crate::Reg<gpi68::GPI68_SPEC>;
#[doc = "SYS IOMUX CFG SAIF SYSCFG FMUX GPIO GPI 68 - The register can be used to configure the selected GPIO connector number for input signals. The signal name is indicated in the \"Name\" column of the following table per StarFive naming conventions. For example, name \"u0_WAVE511_i_uart_rxsin_cfg\" indicates the corresponding input signal is \"u0_WAVE511.i_uart_rxsin\". See GPIO Input Signals (on page 107) for a complete list of the input GPIO signals."]
pub mod gpi68;
#[doc = "gpi72 (rw) register accessor: SYS IOMUX CFG SAIF SYSCFG FMUX GPIO GPI 72 - The register can be used to configure the selected GPIO connector number for input signals. The signal name is indicated in the \"Name\" column of the following table per StarFive naming conventions. For example, name \"u0_WAVE511_i_uart_rxsin_cfg\" indicates the corresponding input signal is \"u0_WAVE511.i_uart_rxsin\". See GPIO Input Signals (on page 107) for a complete list of the input GPIO signals.\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`gpi72::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`gpi72::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@gpi72`]
module"]
pub type GPI72 = crate::Reg<gpi72::GPI72_SPEC>;
#[doc = "SYS IOMUX CFG SAIF SYSCFG FMUX GPIO GPI 72 - The register can be used to configure the selected GPIO connector number for input signals. The signal name is indicated in the \"Name\" column of the following table per StarFive naming conventions. For example, name \"u0_WAVE511_i_uart_rxsin_cfg\" indicates the corresponding input signal is \"u0_WAVE511.i_uart_rxsin\". See GPIO Input Signals (on page 107) for a complete list of the input GPIO signals."]
pub mod gpi72;
#[doc = "gpi76 (rw) register accessor: SYS IOMUX CFG SAIF SYSCFG FMUX GPIO GPI 76 - The register can be used to configure the selected GPIO connector number for input signals. The signal name is indicated in the \"Name\" column of the following table per StarFive naming conventions. For example, name \"u0_WAVE511_i_uart_rxsin_cfg\" indicates the corresponding input signal is \"u0_WAVE511.i_uart_rxsin\". See GPIO Input Signals (on page 107) for a complete list of the input GPIO signals.\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`gpi76::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`gpi76::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@gpi76`]
module"]
pub type GPI76 = crate::Reg<gpi76::GPI76_SPEC>;
#[doc = "SYS IOMUX CFG SAIF SYSCFG FMUX GPIO GPI 76 - The register can be used to configure the selected GPIO connector number for input signals. The signal name is indicated in the \"Name\" column of the following table per StarFive naming conventions. For example, name \"u0_WAVE511_i_uart_rxsin_cfg\" indicates the corresponding input signal is \"u0_WAVE511.i_uart_rxsin\". See GPIO Input Signals (on page 107) for a complete list of the input GPIO signals."]
pub mod gpi76;
#[doc = "gpi80 (rw) register accessor: SYS IOMUX CFG SAIF SYSCFG FMUX GPIO GPI 80 - The register can be used to configure the selected GPIO connector number for input signals. The signal name is indicated in the \"Name\" column of the following table per StarFive naming conventions. For example, name \"u0_WAVE511_i_uart_rxsin_cfg\" indicates the corresponding input signal is \"u0_WAVE511.i_uart_rxsin\". See GPIO Input Signals (on page 107) for a complete list of the input GPIO signals.\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`gpi80::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`gpi80::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@gpi80`]
module"]
pub type GPI80 = crate::Reg<gpi80::GPI80_SPEC>;
#[doc = "SYS IOMUX CFG SAIF SYSCFG FMUX GPIO GPI 80 - The register can be used to configure the selected GPIO connector number for input signals. The signal name is indicated in the \"Name\" column of the following table per StarFive naming conventions. For example, name \"u0_WAVE511_i_uart_rxsin_cfg\" indicates the corresponding input signal is \"u0_WAVE511.i_uart_rxsin\". See GPIO Input Signals (on page 107) for a complete list of the input GPIO signals."]
pub mod gpi80;
#[doc = "gpi84 (rw) register accessor: SYS IOMUX CFG SAIF SYSCFG FMUX GPIO GPI 84 - The register can be used to configure the selected GPIO connector number for input signals. The signal name is indicated in the \"Name\" column of the following table per StarFive naming conventions. For example, name \"u0_WAVE511_i_uart_rxsin_cfg\" indicates the corresponding input signal is \"u0_WAVE511.i_uart_rxsin\". See GPIO Input Signals (on page 107) for a complete list of the input GPIO signals.\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`gpi84::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`gpi84::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@gpi84`]
module"]
pub type GPI84 = crate::Reg<gpi84::GPI84_SPEC>;
#[doc = "SYS IOMUX CFG SAIF SYSCFG FMUX GPIO GPI 84 - The register can be used to configure the selected GPIO connector number for input signals. The signal name is indicated in the \"Name\" column of the following table per StarFive naming conventions. For example, name \"u0_WAVE511_i_uart_rxsin_cfg\" indicates the corresponding input signal is \"u0_WAVE511.i_uart_rxsin\". See GPIO Input Signals (on page 107) for a complete list of the input GPIO signals."]
pub mod gpi84;
#[doc = "gpi88 (rw) register accessor: SYS IOMUX CFG SAIF SYSCFG FMUX GPIO GPI 88 - The register can be used to configure the selected GPIO connector number for input signals. The signal name is indicated in the \"Name\" column of the following table per StarFive naming conventions. For example, name \"u0_WAVE511_i_uart_rxsin_cfg\" indicates the corresponding input signal is \"u0_WAVE511.i_uart_rxsin\". See GPIO Input Signals (on page 107) for a complete list of the input GPIO signals.\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`gpi88::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`gpi88::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@gpi88`]
module"]
pub type GPI88 = crate::Reg<gpi88::GPI88_SPEC>;
#[doc = "SYS IOMUX CFG SAIF SYSCFG FMUX GPIO GPI 88 - The register can be used to configure the selected GPIO connector number for input signals. The signal name is indicated in the \"Name\" column of the following table per StarFive naming conventions. For example, name \"u0_WAVE511_i_uart_rxsin_cfg\" indicates the corresponding input signal is \"u0_WAVE511.i_uart_rxsin\". See GPIO Input Signals (on page 107) for a complete list of the input GPIO signals."]
pub mod gpi88;
#[doc = "ioirq0 (rw) register accessor: Enable GPIO IRQ function\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`ioirq0::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`ioirq0::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@ioirq0`]
module"]
pub type IOIRQ0 = crate::Reg<ioirq0::IOIRQ0_SPEC>;
#[doc = "Enable GPIO IRQ function"]
pub mod ioirq0;
#[doc = "ioirq1 (rw) register accessor: SYS IOMUX CFGSAIF SYSCFG IOIRQ 56: GPIO Interrupt Edge Trigger Selector\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`ioirq1::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`ioirq1::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@ioirq1`]
module"]
pub type IOIRQ1 = crate::Reg<ioirq1::IOIRQ1_SPEC>;
#[doc = "SYS IOMUX CFGSAIF SYSCFG IOIRQ 56: GPIO Interrupt Edge Trigger Selector"]
pub mod ioirq1;
#[doc = "ioirq2 (rw) register accessor: SYS IOMUX CFGSAIF SYSCFG IOIRQ 57: GPIO Interrupt Edge Trigger Selector\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`ioirq2::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`ioirq2::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@ioirq2`]
module"]
pub type IOIRQ2 = crate::Reg<ioirq2::IOIRQ2_SPEC>;
#[doc = "SYS IOMUX CFGSAIF SYSCFG IOIRQ 57: GPIO Interrupt Edge Trigger Selector"]
pub mod ioirq2;
#[doc = "ioirq3 (rw) register accessor: SYS IOMUX CFGSAIF SYSCFG IOIRQ 58: GPIO Interrupt Clear\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`ioirq3::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`ioirq3::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@ioirq3`]
module"]
pub type IOIRQ3 = crate::Reg<ioirq3::IOIRQ3_SPEC>;
#[doc = "SYS IOMUX CFGSAIF SYSCFG IOIRQ 58: GPIO Interrupt Clear"]
pub mod ioirq3;
#[doc = "ioirq4 (rw) register accessor: SYS IOMUX CFGSAIF SYSCFG IOIRQ 59: GPIO Interrupt Clear\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`ioirq4::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`ioirq4::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@ioirq4`]
module"]
pub type IOIRQ4 = crate::Reg<ioirq4::IOIRQ4_SPEC>;
#[doc = "SYS IOMUX CFGSAIF SYSCFG IOIRQ 59: GPIO Interrupt Clear"]
pub mod ioirq4;
#[doc = "ioirq5 (rw) register accessor: SYS IOMUX CFGSAIF SYSCFG IOIRQ 60: GPIO Interrupt Both Edge Trigger Selector\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`ioirq5::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`ioirq5::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@ioirq5`]
module"]
pub type IOIRQ5 = crate::Reg<ioirq5::IOIRQ5_SPEC>;
#[doc = "SYS IOMUX CFGSAIF SYSCFG IOIRQ 60: GPIO Interrupt Both Edge Trigger Selector"]
pub mod ioirq5;
#[doc = "ioirq6 (rw) register accessor: SYS IOMUX CFGSAIF SYSCFG IOIRQ 61: GPIO Interrupt Both Edge Trigger Selector\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`ioirq6::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`ioirq6::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@ioirq6`]
module"]
pub type IOIRQ6 = crate::Reg<ioirq6::IOIRQ6_SPEC>;
#[doc = "SYS IOMUX CFGSAIF SYSCFG IOIRQ 61: GPIO Interrupt Both Edge Trigger Selector"]
pub mod ioirq6;
#[doc = "ioirq7 (rw) register accessor: SYS IOMUX CFGSAIF SYSCFG IOIRQ 62: GPIO Interrupt Edge Value\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`ioirq7::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`ioirq7::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@ioirq7`]
module"]
pub type IOIRQ7 = crate::Reg<ioirq7::IOIRQ7_SPEC>;
#[doc = "SYS IOMUX CFGSAIF SYSCFG IOIRQ 62: GPIO Interrupt Edge Value"]
pub mod ioirq7;
#[doc = "ioirq8 (rw) register accessor: SYS IOMUX CFGSAIF SYSCFG IOIRQ 63: GPIO Interrupt Edge Value\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`ioirq8::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`ioirq8::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@ioirq8`]
module"]
pub type IOIRQ8 = crate::Reg<ioirq8::IOIRQ8_SPEC>;
#[doc = "SYS IOMUX CFGSAIF SYSCFG IOIRQ 63: GPIO Interrupt Edge Value"]
pub mod ioirq8;
#[doc = "ioirq9 (rw) register accessor: SYS IOMUX CFGSAIF SYSCFG IOIRQ 64: GPIO Interrupt Edge Mask Selector\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`ioirq9::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`ioirq9::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@ioirq9`]
module"]
pub type IOIRQ9 = crate::Reg<ioirq9::IOIRQ9_SPEC>;
#[doc = "SYS IOMUX CFGSAIF SYSCFG IOIRQ 64: GPIO Interrupt Edge Mask Selector"]
pub mod ioirq9;
#[doc = "ioirq10 (rw) register accessor: SYS IOMUX CFGSAIF SYSCFG IOIRQ 65: GPIO Interrupt Edge Mask Selector\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`ioirq10::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`ioirq10::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@ioirq10`]
module"]
pub type IOIRQ10 = crate::Reg<ioirq10::IOIRQ10_SPEC>;
#[doc = "SYS IOMUX CFGSAIF SYSCFG IOIRQ 65: GPIO Interrupt Edge Mask Selector"]
pub mod ioirq10;
#[doc = "ioirq11 (rw) register accessor: SYS IOMUX CFGSAIF SYSCFG IOIRQ 66: GPIO Register Interrupt Status\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`ioirq11::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`ioirq11::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@ioirq11`]
module"]
pub type IOIRQ11 = crate::Reg<ioirq11::IOIRQ11_SPEC>;
#[doc = "SYS IOMUX CFGSAIF SYSCFG IOIRQ 66: GPIO Register Interrupt Status"]
pub mod ioirq11;
#[doc = "ioirq12 (rw) register accessor: SYS IOMUX CFGSAIF SYSCFG IOIRQ 67: GPIO Register Interrupt Status\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`ioirq12::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`ioirq12::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@ioirq12`]
module"]
pub type IOIRQ12 = crate::Reg<ioirq12::IOIRQ12_SPEC>;
#[doc = "SYS IOMUX CFGSAIF SYSCFG IOIRQ 67: GPIO Register Interrupt Status"]
pub mod ioirq12;
#[doc = "ioirq13 (rw) register accessor: SYS IOMUX CFGSAIF SYSCFG IOIRQ 68: GPIO Masked Interrupt Status\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`ioirq13::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`ioirq13::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@ioirq13`]
module"]
pub type IOIRQ13 = crate::Reg<ioirq13::IOIRQ13_SPEC>;
#[doc = "SYS IOMUX CFGSAIF SYSCFG IOIRQ 68: GPIO Masked Interrupt Status"]
pub mod ioirq13;
#[doc = "ioirq14 (rw) register accessor: SYS IOMUX CFGSAIF SYSCFG IOIRQ 69: GPIO Masked Interrupt Status\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`ioirq14::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`ioirq14::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@ioirq14`]
module"]
pub type IOIRQ14 = crate::Reg<ioirq14::IOIRQ14_SPEC>;
#[doc = "SYS IOMUX CFGSAIF SYSCFG IOIRQ 69: GPIO Masked Interrupt Status"]
pub mod ioirq14;
#[doc = "ioirq15 (rw) register accessor: SYS IOMUX CFGSAIF SYSCFG IOIRQ 70: GPIO Synchronization Status\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`ioirq15::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`ioirq15::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@ioirq15`]
module"]
pub type IOIRQ15 = crate::Reg<ioirq15::IOIRQ15_SPEC>;
#[doc = "SYS IOMUX CFGSAIF SYSCFG IOIRQ 70: GPIO Synchronization Status"]
pub mod ioirq15;
#[doc = "ioirq16 (rw) register accessor: SYS IOMUX CFGSAIF SYSCFG IOIRQ 71: GPIO Synchronization Status\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`ioirq16::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`ioirq16::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@ioirq16`]
module"]
pub type IOIRQ16 = crate::Reg<ioirq16::IOIRQ16_SPEC>;
#[doc = "SYS IOMUX CFGSAIF SYSCFG IOIRQ 71: GPIO Synchronization Status"]
pub mod ioirq16;
#[doc = "padcfg_gpio0 (rw) register accessor: SYS IOMUX CFG SAIF SYSCFG PADCFG 288: GPIO0\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`padcfg_gpio0::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`padcfg_gpio0::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@padcfg_gpio0`]
module"]
pub type PADCFG_GPIO0 = crate::Reg<padcfg_gpio0::PADCFG_GPIO0_SPEC>;
#[doc = "SYS IOMUX CFG SAIF SYSCFG PADCFG 288: GPIO0"]
pub mod padcfg_gpio0;
#[doc = "padcfg_gpio1 (rw) register accessor: SYS IOMUX CFG SAIF SYSCFG PADCFG 292: GPIO1\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`padcfg_gpio1::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`padcfg_gpio1::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@padcfg_gpio1`]
module"]
pub type PADCFG_GPIO1 = crate::Reg<padcfg_gpio1::PADCFG_GPIO1_SPEC>;
#[doc = "SYS IOMUX CFG SAIF SYSCFG PADCFG 292: GPIO1"]
pub mod padcfg_gpio1;
#[doc = "padcfg_gpio2 (rw) register accessor: SYS IOMUX CFG SAIF SYSCFG PADCFG 296: GPIO2\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`padcfg_gpio2::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`padcfg_gpio2::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@padcfg_gpio2`]
module"]
pub type PADCFG_GPIO2 = crate::Reg<padcfg_gpio2::PADCFG_GPIO2_SPEC>;
#[doc = "SYS IOMUX CFG SAIF SYSCFG PADCFG 296: GPIO2"]
pub mod padcfg_gpio2;
#[doc = "padcfg_gpio3 (rw) register accessor: SYS IOMUX CFG SAIF SYSCFG PADCFG 300: GPIO3\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`padcfg_gpio3::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`padcfg_gpio3::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@padcfg_gpio3`]
module"]
pub type PADCFG_GPIO3 = crate::Reg<padcfg_gpio3::PADCFG_GPIO3_SPEC>;
#[doc = "SYS IOMUX CFG SAIF SYSCFG PADCFG 300: GPIO3"]
pub mod padcfg_gpio3;
#[doc = "padcfg_gpio4 (rw) register accessor: SYS IOMUX CFG SAIF SYSCFG PADCFG 304: GPIO4\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`padcfg_gpio4::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`padcfg_gpio4::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@padcfg_gpio4`]
module"]
pub type PADCFG_GPIO4 = crate::Reg<padcfg_gpio4::PADCFG_GPIO4_SPEC>;
#[doc = "SYS IOMUX CFG SAIF SYSCFG PADCFG 304: GPIO4"]
pub mod padcfg_gpio4;
#[doc = "padcfg_gpio5 (rw) register accessor: SYS IOMUX CFG SAIF SYSCFG PADCFG 308: GPIO5\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`padcfg_gpio5::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`padcfg_gpio5::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@padcfg_gpio5`]
module"]
pub type PADCFG_GPIO5 = crate::Reg<padcfg_gpio5::PADCFG_GPIO5_SPEC>;
#[doc = "SYS IOMUX CFG SAIF SYSCFG PADCFG 308: GPIO5"]
pub mod padcfg_gpio5;
#[doc = "padcfg_gpio6 (rw) register accessor: SYS IOMUX CFG SAIF SYSCFG PADCFG 312: GPIO6\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`padcfg_gpio6::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`padcfg_gpio6::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@padcfg_gpio6`]
module"]
pub type PADCFG_GPIO6 = crate::Reg<padcfg_gpio6::PADCFG_GPIO6_SPEC>;
#[doc = "SYS IOMUX CFG SAIF SYSCFG PADCFG 312: GPIO6"]
pub mod padcfg_gpio6;
#[doc = "padcfg_gpio7 (rw) register accessor: SYS IOMUX CFG SAIF SYSCFG PADCFG 316: GPIO7\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`padcfg_gpio7::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`padcfg_gpio7::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@padcfg_gpio7`]
module"]
pub type PADCFG_GPIO7 = crate::Reg<padcfg_gpio7::PADCFG_GPIO7_SPEC>;
#[doc = "SYS IOMUX CFG SAIF SYSCFG PADCFG 316: GPIO7"]
pub mod padcfg_gpio7;
#[doc = "padcfg_gpio8 (rw) register accessor: SYS IOMUX CFG SAIF SYSCFG PADCFG 320: GPIO8\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`padcfg_gpio8::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`padcfg_gpio8::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@padcfg_gpio8`]
module"]
pub type PADCFG_GPIO8 = crate::Reg<padcfg_gpio8::PADCFG_GPIO8_SPEC>;
#[doc = "SYS IOMUX CFG SAIF SYSCFG PADCFG 320: GPIO8"]
pub mod padcfg_gpio8;
#[doc = "padcfg_gpio9 (rw) register accessor: SYS IOMUX CFG SAIF SYSCFG PADCFG 324: GPIO9\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`padcfg_gpio9::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`padcfg_gpio9::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@padcfg_gpio9`]
module"]
pub type PADCFG_GPIO9 = crate::Reg<padcfg_gpio9::PADCFG_GPIO9_SPEC>;
#[doc = "SYS IOMUX CFG SAIF SYSCFG PADCFG 324: GPIO9"]
pub mod padcfg_gpio9;
#[doc = "padcfg_gpio10 (rw) register accessor: SYS IOMUX CFG SAIF SYSCFG PADCFG 328: GPIO10\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`padcfg_gpio10::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`padcfg_gpio10::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@padcfg_gpio10`]
module"]
pub type PADCFG_GPIO10 = crate::Reg<padcfg_gpio10::PADCFG_GPIO10_SPEC>;
#[doc = "SYS IOMUX CFG SAIF SYSCFG PADCFG 328: GPIO10"]
pub mod padcfg_gpio10;
#[doc = "padcfg_gpio11 (rw) register accessor: SYS IOMUX CFG SAIF SYSCFG PADCFG 332: GPIO11\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`padcfg_gpio11::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`padcfg_gpio11::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@padcfg_gpio11`]
module"]
pub type PADCFG_GPIO11 = crate::Reg<padcfg_gpio11::PADCFG_GPIO11_SPEC>;
#[doc = "SYS IOMUX CFG SAIF SYSCFG PADCFG 332: GPIO11"]
pub mod padcfg_gpio11;
#[doc = "padcfg_gpio12 (rw) register accessor: SYS IOMUX CFG SAIF SYSCFG PADCFG 336: GPIO12\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`padcfg_gpio12::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`padcfg_gpio12::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@padcfg_gpio12`]
module"]
pub type PADCFG_GPIO12 = crate::Reg<padcfg_gpio12::PADCFG_GPIO12_SPEC>;
#[doc = "SYS IOMUX CFG SAIF SYSCFG PADCFG 336: GPIO12"]
pub mod padcfg_gpio12;
#[doc = "padcfg_gpio13 (rw) register accessor: SYS IOMUX CFG SAIF SYSCFG PADCFG 340: GPIO13\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`padcfg_gpio13::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`padcfg_gpio13::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@padcfg_gpio13`]
module"]
pub type PADCFG_GPIO13 = crate::Reg<padcfg_gpio13::PADCFG_GPIO13_SPEC>;
#[doc = "SYS IOMUX CFG SAIF SYSCFG PADCFG 340: GPIO13"]
pub mod padcfg_gpio13;
#[doc = "padcfg_gpio14 (rw) register accessor: SYS IOMUX CFG SAIF SYSCFG PADCFG 344: GPIO14\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`padcfg_gpio14::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`padcfg_gpio14::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@padcfg_gpio14`]
module"]
pub type PADCFG_GPIO14 = crate::Reg<padcfg_gpio14::PADCFG_GPIO14_SPEC>;
#[doc = "SYS IOMUX CFG SAIF SYSCFG PADCFG 344: GPIO14"]
pub mod padcfg_gpio14;
#[doc = "padcfg_gpio15 (rw) register accessor: SYS IOMUX CFG SAIF SYSCFG PADCFG 348: GPIO15\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`padcfg_gpio15::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`padcfg_gpio15::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@padcfg_gpio15`]
module"]
pub type PADCFG_GPIO15 = crate::Reg<padcfg_gpio15::PADCFG_GPIO15_SPEC>;
#[doc = "SYS IOMUX CFG SAIF SYSCFG PADCFG 348: GPIO15"]
pub mod padcfg_gpio15;
#[doc = "padcfg_gpio16 (rw) register accessor: SYS IOMUX CFG SAIF SYSCFG PADCFG 352: GPIO16\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`padcfg_gpio16::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`padcfg_gpio16::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@padcfg_gpio16`]
module"]
pub type PADCFG_GPIO16 = crate::Reg<padcfg_gpio16::PADCFG_GPIO16_SPEC>;
#[doc = "SYS IOMUX CFG SAIF SYSCFG PADCFG 352: GPIO16"]
pub mod padcfg_gpio16;
#[doc = "padcfg_gpio17 (rw) register accessor: SYS IOMUX CFG SAIF SYSCFG PADCFG 356: GPIO17\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`padcfg_gpio17::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`padcfg_gpio17::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@padcfg_gpio17`]
module"]
pub type PADCFG_GPIO17 = crate::Reg<padcfg_gpio17::PADCFG_GPIO17_SPEC>;
#[doc = "SYS IOMUX CFG SAIF SYSCFG PADCFG 356: GPIO17"]
pub mod padcfg_gpio17;
#[doc = "padcfg_gpio18 (rw) register accessor: SYS IOMUX CFG SAIF SYSCFG PADCFG 360: GPIO18\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`padcfg_gpio18::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`padcfg_gpio18::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@padcfg_gpio18`]
module"]
pub type PADCFG_GPIO18 = crate::Reg<padcfg_gpio18::PADCFG_GPIO18_SPEC>;
#[doc = "SYS IOMUX CFG SAIF SYSCFG PADCFG 360: GPIO18"]
pub mod padcfg_gpio18;
#[doc = "padcfg_gpio19 (rw) register accessor: SYS IOMUX CFG SAIF SYSCFG PADCFG 364: GPIO19\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`padcfg_gpio19::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`padcfg_gpio19::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@padcfg_gpio19`]
module"]
pub type PADCFG_GPIO19 = crate::Reg<padcfg_gpio19::PADCFG_GPIO19_SPEC>;
#[doc = "SYS IOMUX CFG SAIF SYSCFG PADCFG 364: GPIO19"]
pub mod padcfg_gpio19;
#[doc = "padcfg_gpio20 (rw) register accessor: SYS IOMUX CFG SAIF SYSCFG PADCFG 368: GPIO20\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`padcfg_gpio20::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`padcfg_gpio20::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@padcfg_gpio20`]
module"]
pub type PADCFG_GPIO20 = crate::Reg<padcfg_gpio20::PADCFG_GPIO20_SPEC>;
#[doc = "SYS IOMUX CFG SAIF SYSCFG PADCFG 368: GPIO20"]
pub mod padcfg_gpio20;
#[doc = "padcfg_gpio21 (rw) register accessor: SYS IOMUX CFG SAIF SYSCFG PADCFG 372: GPIO21\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`padcfg_gpio21::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`padcfg_gpio21::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@padcfg_gpio21`]
module"]
pub type PADCFG_GPIO21 = crate::Reg<padcfg_gpio21::PADCFG_GPIO21_SPEC>;
#[doc = "SYS IOMUX CFG SAIF SYSCFG PADCFG 372: GPIO21"]
pub mod padcfg_gpio21;
#[doc = "padcfg_gpio22 (rw) register accessor: SYS IOMUX CFG SAIF SYSCFG PADCFG 376: GPIO22\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`padcfg_gpio22::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`padcfg_gpio22::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@padcfg_gpio22`]
module"]
pub type PADCFG_GPIO22 = crate::Reg<padcfg_gpio22::PADCFG_GPIO22_SPEC>;
#[doc = "SYS IOMUX CFG SAIF SYSCFG PADCFG 376: GPIO22"]
pub mod padcfg_gpio22;
#[doc = "padcfg_gpio23 (rw) register accessor: SYS IOMUX CFG SAIF SYSCFG PADCFG 380: GPIO23\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`padcfg_gpio23::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`padcfg_gpio23::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@padcfg_gpio23`]
module"]
pub type PADCFG_GPIO23 = crate::Reg<padcfg_gpio23::PADCFG_GPIO23_SPEC>;
#[doc = "SYS IOMUX CFG SAIF SYSCFG PADCFG 380: GPIO23"]
pub mod padcfg_gpio23;
#[doc = "padcfg_gpio24 (rw) register accessor: SYS IOMUX CFG SAIF SYSCFG PADCFG 384: GPIO24\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`padcfg_gpio24::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`padcfg_gpio24::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@padcfg_gpio24`]
module"]
pub type PADCFG_GPIO24 = crate::Reg<padcfg_gpio24::PADCFG_GPIO24_SPEC>;
#[doc = "SYS IOMUX CFG SAIF SYSCFG PADCFG 384: GPIO24"]
pub mod padcfg_gpio24;
#[doc = "padcfg_gpio25 (rw) register accessor: SYS IOMUX CFG SAIF SYSCFG PADCFG 388: GPIO25\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`padcfg_gpio25::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`padcfg_gpio25::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@padcfg_gpio25`]
module"]
pub type PADCFG_GPIO25 = crate::Reg<padcfg_gpio25::PADCFG_GPIO25_SPEC>;
#[doc = "SYS IOMUX CFG SAIF SYSCFG PADCFG 388: GPIO25"]
pub mod padcfg_gpio25;
#[doc = "padcfg_gpio26 (rw) register accessor: SYS IOMUX CFG SAIF SYSCFG PADCFG 392: GPIO26\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`padcfg_gpio26::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`padcfg_gpio26::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@padcfg_gpio26`]
module"]
pub type PADCFG_GPIO26 = crate::Reg<padcfg_gpio26::PADCFG_GPIO26_SPEC>;
#[doc = "SYS IOMUX CFG SAIF SYSCFG PADCFG 392: GPIO26"]
pub mod padcfg_gpio26;
#[doc = "padcfg_gpio27 (rw) register accessor: SYS IOMUX CFG SAIF SYSCFG PADCFG 396: GPIO27\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`padcfg_gpio27::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`padcfg_gpio27::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@padcfg_gpio27`]
module"]
pub type PADCFG_GPIO27 = crate::Reg<padcfg_gpio27::PADCFG_GPIO27_SPEC>;
#[doc = "SYS IOMUX CFG SAIF SYSCFG PADCFG 396: GPIO27"]
pub mod padcfg_gpio27;
#[doc = "padcfg_gpio28 (rw) register accessor: SYS IOMUX CFG SAIF SYSCFG PADCFG 400: GPIO28\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`padcfg_gpio28::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`padcfg_gpio28::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@padcfg_gpio28`]
module"]
pub type PADCFG_GPIO28 = crate::Reg<padcfg_gpio28::PADCFG_GPIO28_SPEC>;
#[doc = "SYS IOMUX CFG SAIF SYSCFG PADCFG 400: GPIO28"]
pub mod padcfg_gpio28;
#[doc = "padcfg_gpio29 (rw) register accessor: SYS IOMUX CFG SAIF SYSCFG PADCFG 404: GPIO29\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`padcfg_gpio29::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`padcfg_gpio29::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@padcfg_gpio29`]
module"]
pub type PADCFG_GPIO29 = crate::Reg<padcfg_gpio29::PADCFG_GPIO29_SPEC>;
#[doc = "SYS IOMUX CFG SAIF SYSCFG PADCFG 404: GPIO29"]
pub mod padcfg_gpio29;
#[doc = "padcfg_gpio30 (rw) register accessor: SYS IOMUX CFG SAIF SYSCFG PADCFG 408: GPIO30\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`padcfg_gpio30::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`padcfg_gpio30::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@padcfg_gpio30`]
module"]
pub type PADCFG_GPIO30 = crate::Reg<padcfg_gpio30::PADCFG_GPIO30_SPEC>;
#[doc = "SYS IOMUX CFG SAIF SYSCFG PADCFG 408: GPIO30"]
pub mod padcfg_gpio30;
#[doc = "padcfg_gpio31 (rw) register accessor: SYS IOMUX CFG SAIF SYSCFG PADCFG 412: GPIO31\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`padcfg_gpio31::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`padcfg_gpio31::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@padcfg_gpio31`]
module"]
pub type PADCFG_GPIO31 = crate::Reg<padcfg_gpio31::PADCFG_GPIO31_SPEC>;
#[doc = "SYS IOMUX CFG SAIF SYSCFG PADCFG 412: GPIO31"]
pub mod padcfg_gpio31;
#[doc = "padcfg_gpio32 (rw) register accessor: SYS IOMUX CFG SAIF SYSCFG PADCFG 416: GPIO32\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`padcfg_gpio32::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`padcfg_gpio32::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@padcfg_gpio32`]
module"]
pub type PADCFG_GPIO32 = crate::Reg<padcfg_gpio32::PADCFG_GPIO32_SPEC>;
#[doc = "SYS IOMUX CFG SAIF SYSCFG PADCFG 416: GPIO32"]
pub mod padcfg_gpio32;
#[doc = "padcfg_gpio33 (rw) register accessor: SYS IOMUX CFG SAIF SYSCFG PADCFG 420: GPIO33\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`padcfg_gpio33::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`padcfg_gpio33::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@padcfg_gpio33`]
module"]
pub type PADCFG_GPIO33 = crate::Reg<padcfg_gpio33::PADCFG_GPIO33_SPEC>;
#[doc = "SYS IOMUX CFG SAIF SYSCFG PADCFG 420: GPIO33"]
pub mod padcfg_gpio33;
#[doc = "padcfg_gpio34 (rw) register accessor: SYS IOMUX CFG SAIF SYSCFG PADCFG 424: GPIO34\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`padcfg_gpio34::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`padcfg_gpio34::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@padcfg_gpio34`]
module"]
pub type PADCFG_GPIO34 = crate::Reg<padcfg_gpio34::PADCFG_GPIO34_SPEC>;
#[doc = "SYS IOMUX CFG SAIF SYSCFG PADCFG 424: GPIO34"]
pub mod padcfg_gpio34;
#[doc = "padcfg_gpio35 (rw) register accessor: SYS IOMUX CFG SAIF SYSCFG PADCFG 428: GPIO35\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`padcfg_gpio35::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`padcfg_gpio35::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@padcfg_gpio35`]
module"]
pub type PADCFG_GPIO35 = crate::Reg<padcfg_gpio35::PADCFG_GPIO35_SPEC>;
#[doc = "SYS IOMUX CFG SAIF SYSCFG PADCFG 428: GPIO35"]
pub mod padcfg_gpio35;
#[doc = "padcfg_gpio36 (rw) register accessor: SYS IOMUX CFG SAIF SYSCFG PADCFG 432: GPIO36\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`padcfg_gpio36::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`padcfg_gpio36::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@padcfg_gpio36`]
module"]
pub type PADCFG_GPIO36 = crate::Reg<padcfg_gpio36::PADCFG_GPIO36_SPEC>;
#[doc = "SYS IOMUX CFG SAIF SYSCFG PADCFG 432: GPIO36"]
pub mod padcfg_gpio36;
#[doc = "padcfg_gpio37 (rw) register accessor: SYS IOMUX CFG SAIF SYSCFG PADCFG 436: GPIO37\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`padcfg_gpio37::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`padcfg_gpio37::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@padcfg_gpio37`]
module"]
pub type PADCFG_GPIO37 = crate::Reg<padcfg_gpio37::PADCFG_GPIO37_SPEC>;
#[doc = "SYS IOMUX CFG SAIF SYSCFG PADCFG 436: GPIO37"]
pub mod padcfg_gpio37;
#[doc = "padcfg_gpio38 (rw) register accessor: SYS IOMUX CFG SAIF SYSCFG PADCFG 440: GPIO38\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`padcfg_gpio38::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`padcfg_gpio38::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@padcfg_gpio38`]
module"]
pub type PADCFG_GPIO38 = crate::Reg<padcfg_gpio38::PADCFG_GPIO38_SPEC>;
#[doc = "SYS IOMUX CFG SAIF SYSCFG PADCFG 440: GPIO38"]
pub mod padcfg_gpio38;
#[doc = "padcfg_gpio39 (rw) register accessor: SYS IOMUX CFG SAIF SYSCFG PADCFG 444: GPIO39\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`padcfg_gpio39::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`padcfg_gpio39::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@padcfg_gpio39`]
module"]
pub type PADCFG_GPIO39 = crate::Reg<padcfg_gpio39::PADCFG_GPIO39_SPEC>;
#[doc = "SYS IOMUX CFG SAIF SYSCFG PADCFG 444: GPIO39"]
pub mod padcfg_gpio39;
#[doc = "padcfg_gpio40 (rw) register accessor: SYS IOMUX CFG SAIF SYSCFG PADCFG 448: GPIO40\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`padcfg_gpio40::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`padcfg_gpio40::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@padcfg_gpio40`]
module"]
pub type PADCFG_GPIO40 = crate::Reg<padcfg_gpio40::PADCFG_GPIO40_SPEC>;
#[doc = "SYS IOMUX CFG SAIF SYSCFG PADCFG 448: GPIO40"]
pub mod padcfg_gpio40;
#[doc = "padcfg_gpio41 (rw) register accessor: SYS IOMUX CFG SAIF SYSCFG PADCFG 452: GPIO41\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`padcfg_gpio41::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`padcfg_gpio41::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@padcfg_gpio41`]
module"]
pub type PADCFG_GPIO41 = crate::Reg<padcfg_gpio41::PADCFG_GPIO41_SPEC>;
#[doc = "SYS IOMUX CFG SAIF SYSCFG PADCFG 452: GPIO41"]
pub mod padcfg_gpio41;
#[doc = "padcfg_gpio42 (rw) register accessor: SYS IOMUX CFG SAIF SYSCFG PADCFG 456: GPIO42\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`padcfg_gpio42::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`padcfg_gpio42::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@padcfg_gpio42`]
module"]
pub type PADCFG_GPIO42 = crate::Reg<padcfg_gpio42::PADCFG_GPIO42_SPEC>;
#[doc = "SYS IOMUX CFG SAIF SYSCFG PADCFG 456: GPIO42"]
pub mod padcfg_gpio42;
#[doc = "padcfg_gpio43 (rw) register accessor: SYS IOMUX CFG SAIF SYSCFG PADCFG 460: GPIO43\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`padcfg_gpio43::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`padcfg_gpio43::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@padcfg_gpio43`]
module"]
pub type PADCFG_GPIO43 = crate::Reg<padcfg_gpio43::PADCFG_GPIO43_SPEC>;
#[doc = "SYS IOMUX CFG SAIF SYSCFG PADCFG 460: GPIO43"]
pub mod padcfg_gpio43;
#[doc = "padcfg_gpio44 (rw) register accessor: SYS IOMUX CFG SAIF SYSCFG PADCFG 464: GPIO44\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`padcfg_gpio44::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`padcfg_gpio44::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@padcfg_gpio44`]
module"]
pub type PADCFG_GPIO44 = crate::Reg<padcfg_gpio44::PADCFG_GPIO44_SPEC>;
#[doc = "SYS IOMUX CFG SAIF SYSCFG PADCFG 464: GPIO44"]
pub mod padcfg_gpio44;
#[doc = "padcfg_gpio45 (rw) register accessor: SYS IOMUX CFG SAIF SYSCFG PADCFG 468: GPIO45\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`padcfg_gpio45::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`padcfg_gpio45::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@padcfg_gpio45`]
module"]
pub type PADCFG_GPIO45 = crate::Reg<padcfg_gpio45::PADCFG_GPIO45_SPEC>;
#[doc = "SYS IOMUX CFG SAIF SYSCFG PADCFG 468: GPIO45"]
pub mod padcfg_gpio45;
#[doc = "padcfg_gpio46 (rw) register accessor: SYS IOMUX CFG SAIF SYSCFG PADCFG 472: GPIO46\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`padcfg_gpio46::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`padcfg_gpio46::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@padcfg_gpio46`]
module"]
pub type PADCFG_GPIO46 = crate::Reg<padcfg_gpio46::PADCFG_GPIO46_SPEC>;
#[doc = "SYS IOMUX CFG SAIF SYSCFG PADCFG 472: GPIO46"]
pub mod padcfg_gpio46;
#[doc = "padcfg_gpio47 (rw) register accessor: SYS IOMUX CFG SAIF SYSCFG PADCFG 476: GPIO47\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`padcfg_gpio47::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`padcfg_gpio47::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@padcfg_gpio47`]
module"]
pub type PADCFG_GPIO47 = crate::Reg<padcfg_gpio47::PADCFG_GPIO47_SPEC>;
#[doc = "SYS IOMUX CFG SAIF SYSCFG PADCFG 476: GPIO47"]
pub mod padcfg_gpio47;
#[doc = "padcfg_gpio48 (rw) register accessor: SYS IOMUX CFG SAIF SYSCFG PADCFG 480: GPIO48\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`padcfg_gpio48::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`padcfg_gpio48::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@padcfg_gpio48`]
module"]
pub type PADCFG_GPIO48 = crate::Reg<padcfg_gpio48::PADCFG_GPIO48_SPEC>;
#[doc = "SYS IOMUX CFG SAIF SYSCFG PADCFG 480: GPIO48"]
pub mod padcfg_gpio48;
#[doc = "padcfg_gpio49 (rw) register accessor: SYS IOMUX CFG SAIF SYSCFG PADCFG 484: GPIO49\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`padcfg_gpio49::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`padcfg_gpio49::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@padcfg_gpio49`]
module"]
pub type PADCFG_GPIO49 = crate::Reg<padcfg_gpio49::PADCFG_GPIO49_SPEC>;
#[doc = "SYS IOMUX CFG SAIF SYSCFG PADCFG 484: GPIO49"]
pub mod padcfg_gpio49;
#[doc = "padcfg_gpio50 (rw) register accessor: SYS IOMUX CFG SAIF SYSCFG PADCFG 488: GPIO50\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`padcfg_gpio50::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`padcfg_gpio50::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@padcfg_gpio50`]
module"]
pub type PADCFG_GPIO50 = crate::Reg<padcfg_gpio50::PADCFG_GPIO50_SPEC>;
#[doc = "SYS IOMUX CFG SAIF SYSCFG PADCFG 488: GPIO50"]
pub mod padcfg_gpio50;
#[doc = "padcfg_gpio51 (rw) register accessor: SYS IOMUX CFG SAIF SYSCFG PADCFG 492: GPIO51\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`padcfg_gpio51::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`padcfg_gpio51::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@padcfg_gpio51`]
module"]
pub type PADCFG_GPIO51 = crate::Reg<padcfg_gpio51::PADCFG_GPIO51_SPEC>;
#[doc = "SYS IOMUX CFG SAIF SYSCFG PADCFG 492: GPIO51"]
pub mod padcfg_gpio51;
#[doc = "padcfg_gpio52 (rw) register accessor: SYS IOMUX CFG SAIF SYSCFG PADCFG 496: GPIO52\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`padcfg_gpio52::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`padcfg_gpio52::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@padcfg_gpio52`]
module"]
pub type PADCFG_GPIO52 = crate::Reg<padcfg_gpio52::PADCFG_GPIO52_SPEC>;
#[doc = "SYS IOMUX CFG SAIF SYSCFG PADCFG 496: GPIO52"]
pub mod padcfg_gpio52;
#[doc = "padcfg_gpio53 (rw) register accessor: SYS IOMUX CFG SAIF SYSCFG PADCFG 500: GPIO53\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`padcfg_gpio53::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`padcfg_gpio53::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@padcfg_gpio53`]
module"]
pub type PADCFG_GPIO53 = crate::Reg<padcfg_gpio53::PADCFG_GPIO53_SPEC>;
#[doc = "SYS IOMUX CFG SAIF SYSCFG PADCFG 500: GPIO53"]
pub mod padcfg_gpio53;
#[doc = "padcfg_gpio54 (rw) register accessor: SYS IOMUX CFG SAIF SYSCFG PADCFG 504: GPIO54\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`padcfg_gpio54::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`padcfg_gpio54::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@padcfg_gpio54`]
module"]
pub type PADCFG_GPIO54 = crate::Reg<padcfg_gpio54::PADCFG_GPIO54_SPEC>;
#[doc = "SYS IOMUX CFG SAIF SYSCFG PADCFG 504: GPIO54"]
pub mod padcfg_gpio54;
#[doc = "padcfg_gpio55 (rw) register accessor: SYS IOMUX CFG SAIF SYSCFG PADCFG 508: GPIO55\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`padcfg_gpio55::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`padcfg_gpio55::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@padcfg_gpio55`]
module"]
pub type PADCFG_GPIO55 = crate::Reg<padcfg_gpio55::PADCFG_GPIO55_SPEC>;
#[doc = "SYS IOMUX CFG SAIF SYSCFG PADCFG 508: GPIO55"]
pub mod padcfg_gpio55;
#[doc = "padcfg_gpio56 (rw) register accessor: SYS IOMUX CFG SAIF SYSCFG PADCFG 512: GPIO56\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`padcfg_gpio56::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`padcfg_gpio56::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@padcfg_gpio56`]
module"]
pub type PADCFG_GPIO56 = crate::Reg<padcfg_gpio56::PADCFG_GPIO56_SPEC>;
#[doc = "SYS IOMUX CFG SAIF SYSCFG PADCFG 512: GPIO56"]
pub mod padcfg_gpio56;
#[doc = "padcfg_gpio57 (rw) register accessor: SYS IOMUX CFG SAIF SYSCFG PADCFG 516: GPIO57\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`padcfg_gpio57::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`padcfg_gpio57::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@padcfg_gpio57`]
module"]
pub type PADCFG_GPIO57 = crate::Reg<padcfg_gpio57::PADCFG_GPIO57_SPEC>;
#[doc = "SYS IOMUX CFG SAIF SYSCFG PADCFG 516: GPIO57"]
pub mod padcfg_gpio57;
#[doc = "padcfg_gpio58 (rw) register accessor: SYS IOMUX CFG SAIF SYSCFG PADCFG 520: GPIO58\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`padcfg_gpio58::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`padcfg_gpio58::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@padcfg_gpio58`]
module"]
pub type PADCFG_GPIO58 = crate::Reg<padcfg_gpio58::PADCFG_GPIO58_SPEC>;
#[doc = "SYS IOMUX CFG SAIF SYSCFG PADCFG 520: GPIO58"]
pub mod padcfg_gpio58;
#[doc = "padcfg_gpio59 (rw) register accessor: SYS IOMUX CFG SAIF SYSCFG PADCFG 524: GPIO59\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`padcfg_gpio59::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`padcfg_gpio59::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@padcfg_gpio59`]
module"]
pub type PADCFG_GPIO59 = crate::Reg<padcfg_gpio59::PADCFG_GPIO59_SPEC>;
#[doc = "SYS IOMUX CFG SAIF SYSCFG PADCFG 524: GPIO59"]
pub mod padcfg_gpio59;
#[doc = "padcfg_gpio60 (rw) register accessor: SYS IOMUX CFG SAIF SYSCFG PADCFG 528: GPIO60\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`padcfg_gpio60::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`padcfg_gpio60::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@padcfg_gpio60`]
module"]
pub type PADCFG_GPIO60 = crate::Reg<padcfg_gpio60::PADCFG_GPIO60_SPEC>;
#[doc = "SYS IOMUX CFG SAIF SYSCFG PADCFG 528: GPIO60"]
pub mod padcfg_gpio60;
#[doc = "padcfg_gpio61 (rw) register accessor: SYS IOMUX CFG SAIF SYSCFG PADCFG 532: GPIO61\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`padcfg_gpio61::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`padcfg_gpio61::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@padcfg_gpio61`]
module"]
pub type PADCFG_GPIO61 = crate::Reg<padcfg_gpio61::PADCFG_GPIO61_SPEC>;
#[doc = "SYS IOMUX CFG SAIF SYSCFG PADCFG 532: GPIO61"]
pub mod padcfg_gpio61;
#[doc = "padcfg_gpio62 (rw) register accessor: SYS IOMUX CFG SAIF SYSCFG PADCFG 536: GPIO62\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`padcfg_gpio62::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`padcfg_gpio62::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@padcfg_gpio62`]
module"]
pub type PADCFG_GPIO62 = crate::Reg<padcfg_gpio62::PADCFG_GPIO62_SPEC>;
#[doc = "SYS IOMUX CFG SAIF SYSCFG PADCFG 536: GPIO62"]
pub mod padcfg_gpio62;
#[doc = "padcfg_gpio63 (rw) register accessor: SYS IOMUX CFG SAIF SYSCFG PADCFG 540: GPIO63\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`padcfg_gpio63::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`padcfg_gpio63::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@padcfg_gpio63`]
module"]
pub type PADCFG_GPIO63 = crate::Reg<padcfg_gpio63::PADCFG_GPIO63_SPEC>;
#[doc = "SYS IOMUX CFG SAIF SYSCFG PADCFG 540: GPIO63"]
pub mod padcfg_gpio63;
#[doc = "padcfg_sd0_clk (rw) register accessor: SYS IOMUX CFG SAIF SYSCFG PADCFG 544: SD0_CLK\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`padcfg_sd0_clk::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`padcfg_sd0_clk::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@padcfg_sd0_clk`]
module"]
pub type PADCFG_SD0_CLK = crate::Reg<padcfg_sd0_clk::PADCFG_SD0_CLK_SPEC>;
#[doc = "SYS IOMUX CFG SAIF SYSCFG PADCFG 544: SD0_CLK"]
pub mod padcfg_sd0_clk;
#[doc = "padcfg_sd0_cmd (rw) register accessor: SYS IOMUX CFG SAIF SYSCFG PADCFG 548: SD0_CMD\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`padcfg_sd0_cmd::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`padcfg_sd0_cmd::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@padcfg_sd0_cmd`]
module"]
pub type PADCFG_SD0_CMD = crate::Reg<padcfg_sd0_cmd::PADCFG_SD0_CMD_SPEC>;
#[doc = "SYS IOMUX CFG SAIF SYSCFG PADCFG 548: SD0_CMD"]
pub mod padcfg_sd0_cmd;
#[doc = "padcfg_sd0_data0 (rw) register accessor: SYS IOMUX CFG SAIF SYSCFG PADCFG 552: SD0_DATA0\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`padcfg_sd0_data0::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`padcfg_sd0_data0::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@padcfg_sd0_data0`]
module"]
pub type PADCFG_SD0_DATA0 = crate::Reg<padcfg_sd0_data0::PADCFG_SD0_DATA0_SPEC>;
#[doc = "SYS IOMUX CFG SAIF SYSCFG PADCFG 552: SD0_DATA0"]
pub mod padcfg_sd0_data0;
#[doc = "padcfg_sd0_data1 (rw) register accessor: SYS IOMUX CFG SAIF SYSCFG PADCFG 568: SD0_DATA1\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`padcfg_sd0_data1::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`padcfg_sd0_data1::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@padcfg_sd0_data1`]
module"]
pub type PADCFG_SD0_DATA1 = crate::Reg<padcfg_sd0_data1::PADCFG_SD0_DATA1_SPEC>;
#[doc = "SYS IOMUX CFG SAIF SYSCFG PADCFG 568: SD0_DATA1"]
pub mod padcfg_sd0_data1;
#[doc = "padcfg_sd0_data2 (rw) register accessor: SYS IOMUX CFG SAIF SYSCFG PADCFG 584: SD0_DATA2\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`padcfg_sd0_data2::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`padcfg_sd0_data2::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@padcfg_sd0_data2`]
module"]
pub type PADCFG_SD0_DATA2 = crate::Reg<padcfg_sd0_data2::PADCFG_SD0_DATA2_SPEC>;
#[doc = "SYS IOMUX CFG SAIF SYSCFG PADCFG 584: SD0_DATA2"]
pub mod padcfg_sd0_data2;
#[doc = "padcfg_sd0_data3 (rw) register accessor: SYS IOMUX CFG SAIF SYSCFG PADCFG 600: SD0_DATA3\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`padcfg_sd0_data3::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`padcfg_sd0_data3::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@padcfg_sd0_data3`]
module"]
pub type PADCFG_SD0_DATA3 = crate::Reg<padcfg_sd0_data3::PADCFG_SD0_DATA3_SPEC>;
#[doc = "SYS IOMUX CFG SAIF SYSCFG PADCFG 600: SD0_DATA3"]
pub mod padcfg_sd0_data3;
#[doc = "padcfg_sd0_data4 (rw) register accessor: SYS IOMUX CFG SAIF SYSCFG PADCFG 616: SD0_DATA4\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`padcfg_sd0_data4::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`padcfg_sd0_data4::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@padcfg_sd0_data4`]
module"]
pub type PADCFG_SD0_DATA4 = crate::Reg<padcfg_sd0_data4::PADCFG_SD0_DATA4_SPEC>;
#[doc = "SYS IOMUX CFG SAIF SYSCFG PADCFG 616: SD0_DATA4"]
pub mod padcfg_sd0_data4;
#[doc = "padcfg_sd0_data5 (rw) register accessor: SYS IOMUX CFG SAIF SYSCFG PADCFG 632: SD0_DATA5\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`padcfg_sd0_data5::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`padcfg_sd0_data5::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@padcfg_sd0_data5`]
module"]
pub type PADCFG_SD0_DATA5 = crate::Reg<padcfg_sd0_data5::PADCFG_SD0_DATA5_SPEC>;
#[doc = "SYS IOMUX CFG SAIF SYSCFG PADCFG 632: SD0_DATA5"]
pub mod padcfg_sd0_data5;
#[doc = "padcfg_sd0_data6 (rw) register accessor: SYS IOMUX CFG SAIF SYSCFG PADCFG 648: SD0_DATA6\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`padcfg_sd0_data6::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`padcfg_sd0_data6::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@padcfg_sd0_data6`]
module"]
pub type PADCFG_SD0_DATA6 = crate::Reg<padcfg_sd0_data6::PADCFG_SD0_DATA6_SPEC>;
#[doc = "SYS IOMUX CFG SAIF SYSCFG PADCFG 648: SD0_DATA6"]
pub mod padcfg_sd0_data6;
#[doc = "padcfg_sd0_data7 (rw) register accessor: SYS IOMUX CFG SAIF SYSCFG PADCFG 664: SD0_DATA7\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`padcfg_sd0_data7::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`padcfg_sd0_data7::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@padcfg_sd0_data7`]
module"]
pub type PADCFG_SD0_DATA7 = crate::Reg<padcfg_sd0_data7::PADCFG_SD0_DATA7_SPEC>;
#[doc = "SYS IOMUX CFG SAIF SYSCFG PADCFG 664: SD0_DATA7"]
pub mod padcfg_sd0_data7;
#[doc = "padcfg_sd0_strb (rw) register accessor: SYS IOMUX CFG SAIF SYSCFG PADCFG 584: SD0_STRB\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`padcfg_sd0_strb::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`padcfg_sd0_strb::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@padcfg_sd0_strb`]
module"]
pub type PADCFG_SD0_STRB = crate::Reg<padcfg_sd0_strb::PADCFG_SD0_STRB_SPEC>;
#[doc = "SYS IOMUX CFG SAIF SYSCFG PADCFG 584: SD0_STRB"]
pub mod padcfg_sd0_strb;
#[doc = "padcfg_gmac1_mdc_syscon (rw) register accessor: GPIO GMAC1 MDC Pad Configuration\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`padcfg_gmac1_mdc_syscon::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`padcfg_gmac1_mdc_syscon::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@padcfg_gmac1_mdc_syscon`]
module"]
pub type PADCFG_GMAC1_MDC_SYSCON =
    crate::Reg<padcfg_gmac1_mdc_syscon::PADCFG_GMAC1_MDC_SYSCON_SPEC>;
#[doc = "GPIO GMAC1 MDC Pad Configuration"]
pub mod padcfg_gmac1_mdc_syscon;
#[doc = "padcfg_gmac1_mdio_syscon (rw) register accessor: GPIO GMAC1 MDIO Pad Configuration\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`padcfg_gmac1_mdio_syscon::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`padcfg_gmac1_mdio_syscon::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@padcfg_gmac1_mdio_syscon`]
module"]
pub type PADCFG_GMAC1_MDIO_SYSCON =
    crate::Reg<padcfg_gmac1_mdio_syscon::PADCFG_GMAC1_MDIO_SYSCON_SPEC>;
#[doc = "GPIO GMAC1 MDIO Pad Configuration"]
pub mod padcfg_gmac1_mdio_syscon;
#[doc = "padcfg_gmac1_rxd0_syscon (rw) register accessor: GPIO GMAC1 RXD0 Pad Configuration\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`padcfg_gmac1_rxd0_syscon::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`padcfg_gmac1_rxd0_syscon::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@padcfg_gmac1_rxd0_syscon`]
module"]
pub type PADCFG_GMAC1_RXD0_SYSCON =
    crate::Reg<padcfg_gmac1_rxd0_syscon::PADCFG_GMAC1_RXD0_SYSCON_SPEC>;
#[doc = "GPIO GMAC1 RXD0 Pad Configuration"]
pub mod padcfg_gmac1_rxd0_syscon;
#[doc = "padcfg_gmac1_rxd1_syscon (rw) register accessor: GPIO GMAC1 RXD1 Pad Configuration\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`padcfg_gmac1_rxd1_syscon::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`padcfg_gmac1_rxd1_syscon::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@padcfg_gmac1_rxd1_syscon`]
module"]
pub type PADCFG_GMAC1_RXD1_SYSCON =
    crate::Reg<padcfg_gmac1_rxd1_syscon::PADCFG_GMAC1_RXD1_SYSCON_SPEC>;
#[doc = "GPIO GMAC1 RXD1 Pad Configuration"]
pub mod padcfg_gmac1_rxd1_syscon;
#[doc = "padcfg_gmac1_rxd2_syscon (rw) register accessor: GPIO GMAC1 RXD2 Pad Configuration\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`padcfg_gmac1_rxd2_syscon::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`padcfg_gmac1_rxd2_syscon::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@padcfg_gmac1_rxd2_syscon`]
module"]
pub type PADCFG_GMAC1_RXD2_SYSCON =
    crate::Reg<padcfg_gmac1_rxd2_syscon::PADCFG_GMAC1_RXD2_SYSCON_SPEC>;
#[doc = "GPIO GMAC1 RXD2 Pad Configuration"]
pub mod padcfg_gmac1_rxd2_syscon;
#[doc = "padcfg_gmac1_rxd3_syscon (rw) register accessor: GPIO GMAC1 RXD3 Pad Configuration\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`padcfg_gmac1_rxd3_syscon::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`padcfg_gmac1_rxd3_syscon::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@padcfg_gmac1_rxd3_syscon`]
module"]
pub type PADCFG_GMAC1_RXD3_SYSCON =
    crate::Reg<padcfg_gmac1_rxd3_syscon::PADCFG_GMAC1_RXD3_SYSCON_SPEC>;
#[doc = "GPIO GMAC1 RXD3 Pad Configuration"]
pub mod padcfg_gmac1_rxd3_syscon;
#[doc = "padcfg_gmac1_rxdv_syscon (rw) register accessor: GPIO GMAC1 RXDV Pad Configuration\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`padcfg_gmac1_rxdv_syscon::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`padcfg_gmac1_rxdv_syscon::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@padcfg_gmac1_rxdv_syscon`]
module"]
pub type PADCFG_GMAC1_RXDV_SYSCON =
    crate::Reg<padcfg_gmac1_rxdv_syscon::PADCFG_GMAC1_RXDV_SYSCON_SPEC>;
#[doc = "GPIO GMAC1 RXDV Pad Configuration"]
pub mod padcfg_gmac1_rxdv_syscon;
#[doc = "padcfg_gmac1_rxc_syscon (rw) register accessor: GPIO GMAC1 RXC Pad Configuration\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`padcfg_gmac1_rxc_syscon::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`padcfg_gmac1_rxc_syscon::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@padcfg_gmac1_rxc_syscon`]
module"]
pub type PADCFG_GMAC1_RXC_SYSCON =
    crate::Reg<padcfg_gmac1_rxc_syscon::PADCFG_GMAC1_RXC_SYSCON_SPEC>;
#[doc = "GPIO GMAC1 RXC Pad Configuration"]
pub mod padcfg_gmac1_rxc_syscon;
#[doc = "padcfg_gmac1_txd0_syscon (rw) register accessor: GPIO GMAC1 TXD0 Pad Configuration\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`padcfg_gmac1_txd0_syscon::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`padcfg_gmac1_txd0_syscon::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@padcfg_gmac1_txd0_syscon`]
module"]
pub type PADCFG_GMAC1_TXD0_SYSCON =
    crate::Reg<padcfg_gmac1_txd0_syscon::PADCFG_GMAC1_TXD0_SYSCON_SPEC>;
#[doc = "GPIO GMAC1 TXD0 Pad Configuration"]
pub mod padcfg_gmac1_txd0_syscon;
#[doc = "padcfg_gmac1_txd1_syscon (rw) register accessor: GPIO GMAC1 TXD1 Pad Configuration\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`padcfg_gmac1_txd1_syscon::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`padcfg_gmac1_txd1_syscon::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@padcfg_gmac1_txd1_syscon`]
module"]
pub type PADCFG_GMAC1_TXD1_SYSCON =
    crate::Reg<padcfg_gmac1_txd1_syscon::PADCFG_GMAC1_TXD1_SYSCON_SPEC>;
#[doc = "GPIO GMAC1 TXD1 Pad Configuration"]
pub mod padcfg_gmac1_txd1_syscon;
#[doc = "padcfg_gmac1_txd2_syscon (rw) register accessor: GPIO GMAC1 TXD2 Pad Configuration\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`padcfg_gmac1_txd2_syscon::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`padcfg_gmac1_txd2_syscon::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@padcfg_gmac1_txd2_syscon`]
module"]
pub type PADCFG_GMAC1_TXD2_SYSCON =
    crate::Reg<padcfg_gmac1_txd2_syscon::PADCFG_GMAC1_TXD2_SYSCON_SPEC>;
#[doc = "GPIO GMAC1 TXD2 Pad Configuration"]
pub mod padcfg_gmac1_txd2_syscon;
#[doc = "padcfg_gmac1_txd3_syscon (rw) register accessor: GPIO GMAC1 TXD3 Pad Configuration\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`padcfg_gmac1_txd3_syscon::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`padcfg_gmac1_txd3_syscon::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@padcfg_gmac1_txd3_syscon`]
module"]
pub type PADCFG_GMAC1_TXD3_SYSCON =
    crate::Reg<padcfg_gmac1_txd3_syscon::PADCFG_GMAC1_TXD3_SYSCON_SPEC>;
#[doc = "GPIO GMAC1 TXD3 Pad Configuration"]
pub mod padcfg_gmac1_txd3_syscon;
#[doc = "padcfg_gmac1_txen_syscon (rw) register accessor: GPIO GMAC1 TXEN Pad Configuration\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`padcfg_gmac1_txen_syscon::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`padcfg_gmac1_txen_syscon::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@padcfg_gmac1_txen_syscon`]
module"]
pub type PADCFG_GMAC1_TXEN_SYSCON =
    crate::Reg<padcfg_gmac1_txen_syscon::PADCFG_GMAC1_TXEN_SYSCON_SPEC>;
#[doc = "GPIO GMAC1 TXEN Pad Configuration"]
pub mod padcfg_gmac1_txen_syscon;
#[doc = "padcfg_gmac1_txc_syscon (rw) register accessor: GPIO GMAC1 TXC Pad Configuration\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`padcfg_gmac1_txc_syscon::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`padcfg_gmac1_txc_syscon::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@padcfg_gmac1_txc_syscon`]
module"]
pub type PADCFG_GMAC1_TXC_SYSCON =
    crate::Reg<padcfg_gmac1_txc_syscon::PADCFG_GMAC1_TXC_SYSCON_SPEC>;
#[doc = "GPIO GMAC1 TXC Pad Configuration"]
pub mod padcfg_gmac1_txc_syscon;
#[doc = "padcfg_qspi_sclk (rw) register accessor: SYS IOMUX CFG SAIF SYSCFG PADCFG 644: QSPI_SCLK\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`padcfg_qspi_sclk::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`padcfg_qspi_sclk::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@padcfg_qspi_sclk`]
module"]
pub type PADCFG_QSPI_SCLK = crate::Reg<padcfg_qspi_sclk::PADCFG_QSPI_SCLK_SPEC>;
#[doc = "SYS IOMUX CFG SAIF SYSCFG PADCFG 644: QSPI_SCLK"]
pub mod padcfg_qspi_sclk;
#[doc = "padcfg_qspi_csn0 (rw) register accessor: SYS IOMUX CFG SAIF SYSCFG PADCFG 648: QSPI_CSN0\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`padcfg_qspi_csn0::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`padcfg_qspi_csn0::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@padcfg_qspi_csn0`]
module"]
pub type PADCFG_QSPI_CSN0 = crate::Reg<padcfg_qspi_csn0::PADCFG_QSPI_CSN0_SPEC>;
#[doc = "SYS IOMUX CFG SAIF SYSCFG PADCFG 648: QSPI_CSN0"]
pub mod padcfg_qspi_csn0;
#[doc = "padcfg_qspi_data0 (rw) register accessor: SYS IOMUX CFG SAIF SYSCFG PADCFG 652: QSPI_DATA0\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`padcfg_qspi_data0::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`padcfg_qspi_data0::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@padcfg_qspi_data0`]
module"]
pub type PADCFG_QSPI_DATA0 = crate::Reg<padcfg_qspi_data0::PADCFG_QSPI_DATA0_SPEC>;
#[doc = "SYS IOMUX CFG SAIF SYSCFG PADCFG 652: QSPI_DATA0"]
pub mod padcfg_qspi_data0;
#[doc = "padcfg_qspi_data1 (rw) register accessor: SYS IOMUX CFG SAIF SYSCFG PADCFG 668: QSPI_DATA1\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`padcfg_qspi_data1::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`padcfg_qspi_data1::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@padcfg_qspi_data1`]
module"]
pub type PADCFG_QSPI_DATA1 = crate::Reg<padcfg_qspi_data1::PADCFG_QSPI_DATA1_SPEC>;
#[doc = "SYS IOMUX CFG SAIF SYSCFG PADCFG 668: QSPI_DATA1"]
pub mod padcfg_qspi_data1;
#[doc = "padcfg_qspi_data2 (rw) register accessor: SYS IOMUX CFG SAIF SYSCFG PADCFG 684: QSPI_DATA2\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`padcfg_qspi_data2::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`padcfg_qspi_data2::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@padcfg_qspi_data2`]
module"]
pub type PADCFG_QSPI_DATA2 = crate::Reg<padcfg_qspi_data2::PADCFG_QSPI_DATA2_SPEC>;
#[doc = "SYS IOMUX CFG SAIF SYSCFG PADCFG 684: QSPI_DATA2"]
pub mod padcfg_qspi_data2;
#[doc = "padcfg_qspi_data3 (rw) register accessor: SYS IOMUX CFG SAIF SYSCFG PADCFG 700: QSPI_DATA3\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`padcfg_qspi_data3::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`padcfg_qspi_data3::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@padcfg_qspi_data3`]
module"]
pub type PADCFG_QSPI_DATA3 = crate::Reg<padcfg_qspi_data3::PADCFG_QSPI_DATA3_SPEC>;
#[doc = "SYS IOMUX CFG SAIF SYSCFG PADCFG 700: QSPI_DATA3"]
pub mod padcfg_qspi_data3;
#[doc = "func_sel0 (rw) register accessor: SYS IOMUX CFG SAIF SYSCFG 0\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`func_sel0::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`func_sel0::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@func_sel0`]
module"]
pub type FUNC_SEL0 = crate::Reg<func_sel0::FUNC_SEL0_SPEC>;
#[doc = "SYS IOMUX CFG SAIF SYSCFG 0"]
pub mod func_sel0;
#[doc = "func_sel1 (rw) register accessor: SYS IOMUX CFG SAIF SYSCFG 1\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`func_sel1::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`func_sel1::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@func_sel1`]
module"]
pub type FUNC_SEL1 = crate::Reg<func_sel1::FUNC_SEL1_SPEC>;
#[doc = "SYS IOMUX CFG SAIF SYSCFG 1"]
pub mod func_sel1;
#[doc = "func_sel2 (rw) register accessor: SYS IOMUX CFG SAIF SYSCFG 2\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`func_sel2::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`func_sel2::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@func_sel2`]
module"]
pub type FUNC_SEL2 = crate::Reg<func_sel2::FUNC_SEL2_SPEC>;
#[doc = "SYS IOMUX CFG SAIF SYSCFG 2"]
pub mod func_sel2;
#[doc = "func_sel3 (rw) register accessor: SYS IOMUX CFG SAIF SYSCFG 3\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`func_sel3::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`func_sel3::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@func_sel3`]
module"]
pub type FUNC_SEL3 = crate::Reg<func_sel3::FUNC_SEL3_SPEC>;
#[doc = "SYS IOMUX CFG SAIF SYSCFG 3"]
pub mod func_sel3;
#[doc = "func_sel4 (rw) register accessor: SYS IOMUX CFG SAIF SYSCFG 4\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`func_sel4::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`func_sel4::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@func_sel4`]
module"]
pub type FUNC_SEL4 = crate::Reg<func_sel4::FUNC_SEL4_SPEC>;
#[doc = "SYS IOMUX CFG SAIF SYSCFG 4"]
pub mod func_sel4;
#[doc = "func_sel5 (rw) register accessor: SYS IOMUX CFG SAIF SYSCFG 5\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`func_sel5::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`func_sel5::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@func_sel5`]
module"]
pub type FUNC_SEL5 = crate::Reg<func_sel5::FUNC_SEL5_SPEC>;
#[doc = "SYS IOMUX CFG SAIF SYSCFG 5"]
pub mod func_sel5;
#[doc = "func_sel6 (rw) register accessor: SYS IOMUX CFG SAIF SYSCFG 6\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`func_sel6::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`func_sel6::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@func_sel6`]
module"]
pub type FUNC_SEL6 = crate::Reg<func_sel6::FUNC_SEL6_SPEC>;
#[doc = "SYS IOMUX CFG SAIF SYSCFG 6"]
pub mod func_sel6;
