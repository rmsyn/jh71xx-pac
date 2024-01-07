#[doc = r"Register block"]
#[repr(C)]
pub struct RegisterBlock {
    _reserved_0_dll: [u8; 0x04],
    _reserved_1_dlh: [u8; 0x04],
    _reserved_2_fcr: [u8; 0x04],
    lcr: LCR,
    mcr: MCR,
    lsr: LSR,
    msr: MSR,
    scr: SCR,
    lpdll: LPDLL,
    lpdlh: LPDLH,
    _reserved10: [u8; 0x08],
    _reserved_10_srbr0: [u8; 0x04],
    _reserved_11_srbr1: [u8; 0x04],
    _reserved_12_srbr2: [u8; 0x04],
    _reserved_13_srbr3: [u8; 0x04],
    _reserved_14_srbr4: [u8; 0x04],
    _reserved_15_srbr5: [u8; 0x04],
    _reserved_16_srbr6: [u8; 0x04],
    _reserved_17_srbr7: [u8; 0x04],
    _reserved_18_srbr8: [u8; 0x04],
    _reserved_19_srbr9: [u8; 0x04],
    _reserved_20_srbr10: [u8; 0x04],
    _reserved_21_srbr11: [u8; 0x04],
    _reserved_22_srbr12: [u8; 0x04],
    _reserved_23_srbr13: [u8; 0x04],
    _reserved_24_srbr14: [u8; 0x04],
    _reserved_25_srbr15: [u8; 0x04],
    far: FAR,
    tfr: TFR,
    rfw: RFW,
    usr: USR,
    tfl: TFL,
    rfl: RFL,
    srr: SRR,
    srts: SRTS,
    sbcr: SBCR,
    sdmam: SDMAM,
    sfe: SFE,
    srt: SRT,
    stet: STET,
    htx: HTX,
    dmasa: DMASA,
    _reserved41: [u8; 0x48],
    cpr: CPR,
    _reserved_42_ctr: [u8; 0x04],
}
impl RegisterBlock {
    #[doc = "0x00 - Divisor Latch Low"]
    #[inline(always)]
    pub const fn dll(&self) -> &DLL {
        unsafe { &*(self as *const Self).cast::<u8>().add(0).cast() }
    }
    #[doc = "0x00 - Transmit Holding Register"]
    #[inline(always)]
    pub const fn thr(&self) -> &THR {
        unsafe { &*(self as *const Self).cast::<u8>().add(0).cast() }
    }
    #[doc = "0x00 - Receive Buffer Register"]
    #[inline(always)]
    pub const fn rbr(&self) -> &RBR {
        unsafe { &*(self as *const Self).cast::<u8>().add(0).cast() }
    }
    #[doc = "0x04 - Interrupt Enable Register"]
    #[inline(always)]
    pub const fn ier(&self) -> &IER {
        unsafe { &*(self as *const Self).cast::<u8>().add(4).cast() }
    }
    #[doc = "0x04 - Divisor Latch High"]
    #[inline(always)]
    pub const fn dlh(&self) -> &DLH {
        unsafe { &*(self as *const Self).cast::<u8>().add(4).cast() }
    }
    #[doc = "0x08 - FIFO Control Register"]
    #[inline(always)]
    pub const fn fcr(&self) -> &FCR {
        unsafe { &*(self as *const Self).cast::<u8>().add(8).cast() }
    }
    #[doc = "0x08 - Interrupt Identity Register"]
    #[inline(always)]
    pub const fn iir(&self) -> &IIR {
        unsafe { &*(self as *const Self).cast::<u8>().add(8).cast() }
    }
    #[doc = "0x0c - Line Control Register"]
    #[inline(always)]
    pub const fn lcr(&self) -> &LCR {
        &self.lcr
    }
    #[doc = "0x10 - Modem Control Register"]
    #[inline(always)]
    pub const fn mcr(&self) -> &MCR {
        &self.mcr
    }
    #[doc = "0x14 - Line Status Register"]
    #[inline(always)]
    pub const fn lsr(&self) -> &LSR {
        &self.lsr
    }
    #[doc = "0x18 - Line Status Register"]
    #[inline(always)]
    pub const fn msr(&self) -> &MSR {
        &self.msr
    }
    #[doc = "0x1c - Scratch Pad Register"]
    #[inline(always)]
    pub const fn scr(&self) -> &SCR {
        &self.scr
    }
    #[doc = "0x20 - Low Power Divisor Latch Low Register: This register is only valid when the DW_apb_uart is configured to have SIR low-power reception capabilities implemented (SIR_LP_RX = Yes). If SIR low-power reception capabilities are not implemented, this register does not exist and reading from thsi register address returns zero."]
    #[inline(always)]
    pub const fn lpdll(&self) -> &LPDLL {
        &self.lpdll
    }
    #[doc = "0x24 - Low Power Divisor Latch High Register: This register is only valid when the DW_apb_uart is configured to have SIR low-power reception capabilities implemented (SIR_LP_RX = Yes). If SIR low-power reception capabilities are not implemented, this register does not exist and reading from thsi register address returns zero."]
    #[inline(always)]
    pub const fn lpdlh(&self) -> &LPDLH {
        &self.lpdlh
    }
    #[doc = "0x30 - Shadow Transmit Holding Register 0: This register is only valid when the DW_apb_uart is configured to have additional shadow registers implemented (SHADOW == YES). If shadow registers are not implemented, this register does not exist and reading from this register address returns zero."]
    #[inline(always)]
    pub const fn sthr0(&self) -> &STHR0 {
        unsafe { &*(self as *const Self).cast::<u8>().add(48).cast() }
    }
    #[doc = "0x30 - Shadow Receive Buffer Register 0: This register is only valid when the DW_apb_uart is configured to have additional shadow registers implemented (SHADOW == YES). If shadow registers are not implemented, this register does not exist and reading from this register address returns zero."]
    #[inline(always)]
    pub const fn srbr0(&self) -> &SRBR0 {
        unsafe { &*(self as *const Self).cast::<u8>().add(48).cast() }
    }
    #[doc = "0x34 - Shadow Transmit Holding Register 1: This register is only valid when the DW_apb_uart is configured to have additional shadow registers implemented (SHADOW == YES). If shadow registers are not implemented, this register does not exist and reading from this register address returns zero."]
    #[inline(always)]
    pub const fn sthr1(&self) -> &STHR1 {
        unsafe { &*(self as *const Self).cast::<u8>().add(52).cast() }
    }
    #[doc = "0x34 - Shadow Receive Buffer Register 1: This register is only valid when the DW_apb_uart is configured to have additional shadow registers implemented (SHADOW == YES). If shadow registers are not implemented, this register does not exist and reading from this register address returns zero."]
    #[inline(always)]
    pub const fn srbr1(&self) -> &SRBR1 {
        unsafe { &*(self as *const Self).cast::<u8>().add(52).cast() }
    }
    #[doc = "0x38 - Shadow Transmit Holding Register 2: This register is only valid when the DW_apb_uart is configured to have additional shadow registers implemented (SHADOW == YES). If shadow registers are not implemented, this register does not exist and reading from this register address returns zero."]
    #[inline(always)]
    pub const fn sthr2(&self) -> &STHR2 {
        unsafe { &*(self as *const Self).cast::<u8>().add(56).cast() }
    }
    #[doc = "0x38 - Shadow Receive Buffer Register 2: This register is only valid when the DW_apb_uart is configured to have additional shadow registers implemented (SHADOW == YES). If shadow registers are not implemented, this register does not exist and reading from this register address returns zero."]
    #[inline(always)]
    pub const fn srbr2(&self) -> &SRBR2 {
        unsafe { &*(self as *const Self).cast::<u8>().add(56).cast() }
    }
    #[doc = "0x3c - Shadow Transmit Holding Register 3: This register is only valid when the DW_apb_uart is configured to have additional shadow registers implemented (SHADOW == YES). If shadow registers are not implemented, this register does not exist and reading from this register address returns zero."]
    #[inline(always)]
    pub const fn sthr3(&self) -> &STHR3 {
        unsafe { &*(self as *const Self).cast::<u8>().add(60).cast() }
    }
    #[doc = "0x3c - Shadow Receive Buffer Register 3: This register is only valid when the DW_apb_uart is configured to have additional shadow registers implemented (SHADOW == YES). If shadow registers are not implemented, this register does not exist and reading from this register address returns zero."]
    #[inline(always)]
    pub const fn srbr3(&self) -> &SRBR3 {
        unsafe { &*(self as *const Self).cast::<u8>().add(60).cast() }
    }
    #[doc = "0x40 - Shadow Transmit Holding Register 4: This register is only valid when the DW_apb_uart is configured to have additional shadow registers implemented (SHADOW == YES). If shadow registers are not implemented, this register does not exist and reading from this register address returns zero."]
    #[inline(always)]
    pub const fn sthr4(&self) -> &STHR4 {
        unsafe { &*(self as *const Self).cast::<u8>().add(64).cast() }
    }
    #[doc = "0x40 - Shadow Receive Buffer Register 4: This register is only valid when the DW_apb_uart is configured to have additional shadow registers implemented (SHADOW == YES). If shadow registers are not implemented, this register does not exist and reading from this register address returns zero."]
    #[inline(always)]
    pub const fn srbr4(&self) -> &SRBR4 {
        unsafe { &*(self as *const Self).cast::<u8>().add(64).cast() }
    }
    #[doc = "0x44 - Shadow Transmit Holding Register 5: This register is only valid when the DW_apb_uart is configured to have additional shadow registers implemented (SHADOW == YES). If shadow registers are not implemented, this register does not exist and reading from this register address returns zero."]
    #[inline(always)]
    pub const fn sthr5(&self) -> &STHR5 {
        unsafe { &*(self as *const Self).cast::<u8>().add(68).cast() }
    }
    #[doc = "0x44 - Shadow Receive Buffer Register 5: This register is only valid when the DW_apb_uart is configured to have additional shadow registers implemented (SHADOW == YES). If shadow registers are not implemented, this register does not exist and reading from this register address returns zero."]
    #[inline(always)]
    pub const fn srbr5(&self) -> &SRBR5 {
        unsafe { &*(self as *const Self).cast::<u8>().add(68).cast() }
    }
    #[doc = "0x48 - Shadow Transmit Holding Register 6: This register is only valid when the DW_apb_uart is configured to have additional shadow registers implemented (SHADOW == YES). If shadow registers are not implemented, this register does not exist and reading from this register address returns zero."]
    #[inline(always)]
    pub const fn sthr6(&self) -> &STHR6 {
        unsafe { &*(self as *const Self).cast::<u8>().add(72).cast() }
    }
    #[doc = "0x48 - Shadow Receive Buffer Register 6: This register is only valid when the DW_apb_uart is configured to have additional shadow registers implemented (SHADOW == YES). If shadow registers are not implemented, this register does not exist and reading from this register address returns zero."]
    #[inline(always)]
    pub const fn srbr6(&self) -> &SRBR6 {
        unsafe { &*(self as *const Self).cast::<u8>().add(72).cast() }
    }
    #[doc = "0x4c - Shadow Transmit Holding Register 7: This register is only valid when the DW_apb_uart is configured to have additional shadow registers implemented (SHADOW == YES). If shadow registers are not implemented, this register does not exist and reading from this register address returns zero."]
    #[inline(always)]
    pub const fn sthr7(&self) -> &STHR7 {
        unsafe { &*(self as *const Self).cast::<u8>().add(76).cast() }
    }
    #[doc = "0x4c - Shadow Receive Buffer Register 7: This register is only valid when the DW_apb_uart is configured to have additional shadow registers implemented (SHADOW == YES). If shadow registers are not implemented, this register does not exist and reading from this register address returns zero."]
    #[inline(always)]
    pub const fn srbr7(&self) -> &SRBR7 {
        unsafe { &*(self as *const Self).cast::<u8>().add(76).cast() }
    }
    #[doc = "0x50 - Shadow Transmit Holding Register 8: This register is only valid when the DW_apb_uart is configured to have additional shadow registers implemented (SHADOW == YES). If shadow registers are not implemented, this register does not exist and reading from this register address returns zero."]
    #[inline(always)]
    pub const fn sthr8(&self) -> &STHR8 {
        unsafe { &*(self as *const Self).cast::<u8>().add(80).cast() }
    }
    #[doc = "0x50 - Shadow Receive Buffer Register 8: This register is only valid when the DW_apb_uart is configured to have additional shadow registers implemented (SHADOW == YES). If shadow registers are not implemented, this register does not exist and reading from this register address returns zero."]
    #[inline(always)]
    pub const fn srbr8(&self) -> &SRBR8 {
        unsafe { &*(self as *const Self).cast::<u8>().add(80).cast() }
    }
    #[doc = "0x54 - Shadow Transmit Holding Register 9: This register is only valid when the DW_apb_uart is configured to have additional shadow registers implemented (SHADOW == YES). If shadow registers are not implemented, this register does not exist and reading from this register address returns zero."]
    #[inline(always)]
    pub const fn sthr9(&self) -> &STHR9 {
        unsafe { &*(self as *const Self).cast::<u8>().add(84).cast() }
    }
    #[doc = "0x54 - Shadow Receive Buffer Register 9: This register is only valid when the DW_apb_uart is configured to have additional shadow registers implemented (SHADOW == YES). If shadow registers are not implemented, this register does not exist and reading from this register address returns zero."]
    #[inline(always)]
    pub const fn srbr9(&self) -> &SRBR9 {
        unsafe { &*(self as *const Self).cast::<u8>().add(84).cast() }
    }
    #[doc = "0x58 - Shadow Transmit Holding Register 10: This register is only valid when the DW_apb_uart is configured to have additional shadow registers implemented (SHADOW == YES). If shadow registers are not implemented, this register does not exist and reading from this register address returns zero."]
    #[inline(always)]
    pub const fn sthr10(&self) -> &STHR10 {
        unsafe { &*(self as *const Self).cast::<u8>().add(88).cast() }
    }
    #[doc = "0x58 - Shadow Receive Buffer Register 10: This register is only valid when the DW_apb_uart is configured to have additional shadow registers implemented (SHADOW == YES). If shadow registers are not implemented, this register does not exist and reading from this register address returns zero."]
    #[inline(always)]
    pub const fn srbr10(&self) -> &SRBR10 {
        unsafe { &*(self as *const Self).cast::<u8>().add(88).cast() }
    }
    #[doc = "0x5c - Shadow Transmit Holding Register 11: This register is only valid when the DW_apb_uart is configured to have additional shadow registers implemented (SHADOW == YES). If shadow registers are not implemented, this register does not exist and reading from this register address returns zero."]
    #[inline(always)]
    pub const fn sthr11(&self) -> &STHR11 {
        unsafe { &*(self as *const Self).cast::<u8>().add(92).cast() }
    }
    #[doc = "0x5c - Shadow Receive Buffer Register 11: This register is only valid when the DW_apb_uart is configured to have additional shadow registers implemented (SHADOW == YES). If shadow registers are not implemented, this register does not exist and reading from this register address returns zero."]
    #[inline(always)]
    pub const fn srbr11(&self) -> &SRBR11 {
        unsafe { &*(self as *const Self).cast::<u8>().add(92).cast() }
    }
    #[doc = "0x60 - Shadow Transmit Holding Register 12: This register is only valid when the DW_apb_uart is configured to have additional shadow registers implemented (SHADOW == YES). If shadow registers are not implemented, this register does not exist and reading from this register address returns zero."]
    #[inline(always)]
    pub const fn sthr12(&self) -> &STHR12 {
        unsafe { &*(self as *const Self).cast::<u8>().add(96).cast() }
    }
    #[doc = "0x60 - Shadow Receive Buffer Register 12: This register is only valid when the DW_apb_uart is configured to have additional shadow registers implemented (SHADOW == YES). If shadow registers are not implemented, this register does not exist and reading from this register address returns zero."]
    #[inline(always)]
    pub const fn srbr12(&self) -> &SRBR12 {
        unsafe { &*(self as *const Self).cast::<u8>().add(96).cast() }
    }
    #[doc = "0x64 - Shadow Transmit Holding Register 13: This register is only valid when the DW_apb_uart is configured to have additional shadow registers implemented (SHADOW == YES). If shadow registers are not implemented, this register does not exist and reading from this register address returns zero."]
    #[inline(always)]
    pub const fn sthr13(&self) -> &STHR13 {
        unsafe { &*(self as *const Self).cast::<u8>().add(100).cast() }
    }
    #[doc = "0x64 - Shadow Receive Buffer Register 13: This register is only valid when the DW_apb_uart is configured to have additional shadow registers implemented (SHADOW == YES). If shadow registers are not implemented, this register does not exist and reading from this register address returns zero."]
    #[inline(always)]
    pub const fn srbr13(&self) -> &SRBR13 {
        unsafe { &*(self as *const Self).cast::<u8>().add(100).cast() }
    }
    #[doc = "0x68 - Shadow Transmit Holding Register 14: This register is only valid when the DW_apb_uart is configured to have additional shadow registers implemented (SHADOW == YES). If shadow registers are not implemented, this register does not exist and reading from this register address returns zero."]
    #[inline(always)]
    pub const fn sthr14(&self) -> &STHR14 {
        unsafe { &*(self as *const Self).cast::<u8>().add(104).cast() }
    }
    #[doc = "0x68 - Shadow Receive Buffer Register 14: This register is only valid when the DW_apb_uart is configured to have additional shadow registers implemented (SHADOW == YES). If shadow registers are not implemented, this register does not exist and reading from this register address returns zero."]
    #[inline(always)]
    pub const fn srbr14(&self) -> &SRBR14 {
        unsafe { &*(self as *const Self).cast::<u8>().add(104).cast() }
    }
    #[doc = "0x6c - Shadow Transmit Holding Register 15: This register is only valid when the DW_apb_uart is configured to have additional shadow registers implemented (SHADOW == YES). If shadow registers are not implemented, this register does not exist and reading from this register address returns zero."]
    #[inline(always)]
    pub const fn sthr15(&self) -> &STHR15 {
        unsafe { &*(self as *const Self).cast::<u8>().add(108).cast() }
    }
    #[doc = "0x6c - Shadow Receive Buffer Register 15: This register is only valid when the DW_apb_uart is configured to have additional shadow registers implemented (SHADOW == YES). If shadow registers are not implemented, this register does not exist and reading from this register address returns zero."]
    #[inline(always)]
    pub const fn srbr15(&self) -> &SRBR15 {
        unsafe { &*(self as *const Self).cast::<u8>().add(108).cast() }
    }
    #[doc = "0x70 - FIFO Access Register"]
    #[inline(always)]
    pub const fn far(&self) -> &FAR {
        &self.far
    }
    #[doc = "0x74 - Transmit FIFO Read"]
    #[inline(always)]
    pub const fn tfr(&self) -> &TFR {
        &self.tfr
    }
    #[doc = "0x78 - Receive FIFO Write"]
    #[inline(always)]
    pub const fn rfw(&self) -> &RFW {
        &self.rfw
    }
    #[doc = "0x7c - UART Status Register"]
    #[inline(always)]
    pub const fn usr(&self) -> &USR {
        &self.usr
    }
    #[doc = "0x80 - Transmit FIFO Level: This register is only valid when the DW_apb_uart is configured to have additional FIFO status registers implemented (FIFO_STAT == YES). If status registers are not implemented, this register does not exist and reading from this register address returns zero."]
    #[inline(always)]
    pub const fn tfl(&self) -> &TFL {
        &self.tfl
    }
    #[doc = "0x84 - Receive FIFO Level: This register is only valid when the DW_apb_uart is configured to have additional FIFO status registers implemented (FIFO_STAT == YES). If status registers are not implemented, this register does not exist and reading from this register address returns zero."]
    #[inline(always)]
    pub const fn rfl(&self) -> &RFL {
        &self.rfl
    }
    #[doc = "0x88 - Software Reset Register: This register is only valid when the DW_apb_uart is configured to have additional shadow registers implemented (SHADOW == YES). If shadow registers are not implemented, this register does not exist and reading from this register address returns zero."]
    #[inline(always)]
    pub const fn srr(&self) -> &SRR {
        &self.srr
    }
    #[doc = "0x8c - Shadow Request to Send: This register is only valid when the DW_apb_uart is configured to have additional shadow registers implemented (SHADOW == YES). If shadow registers are not implemented, this register does not exist and reading from this register address returns zero."]
    #[inline(always)]
    pub const fn srts(&self) -> &SRTS {
        &self.srts
    }
    #[doc = "0x90 - Shadow Break Control Register: This register is only valid when the DW_apb_uart is configured to have additional shadow registers implemented (SHADOW == YES). If shadow registers are not implemented, this register does not exist and reading from this register address returns zero."]
    #[inline(always)]
    pub const fn sbcr(&self) -> &SBCR {
        &self.sbcr
    }
    #[doc = "0x94 - Shadow DMA Mode: This register is only valid when the DW_apb_uart is configured to have additional FIFO registers implemented (FIFO_MODE != None) and additional shadow registers implemented (SHADOW == YES). If these registers are not implemented, this register does not exist and reading from this register address returns zero."]
    #[inline(always)]
    pub const fn sdmam(&self) -> &SDMAM {
        &self.sdmam
    }
    #[doc = "0x98 - Shadow FIFO Enable: This register is only valid when the DW_apb_uart is configured to have additional FIFO registers implemented (FIFO_MODE != None) and additional shadow registers implemented (SHADOW == YES). If these registers are not implemented, this register does not exist and reading from this register address returns zero."]
    #[inline(always)]
    pub const fn sfe(&self) -> &SFE {
        &self.sfe
    }
    #[doc = "0x9c - Shadow RCVR Trigger: This register is only valid when the DW_apb_uart is configured to have additional FIFO registers implemented (FIFO_MODE != None) and additional shadow registers implemented (SHADOW == YES). If these registers are not implemented, this register does not exist and reading from this register address returns zero."]
    #[inline(always)]
    pub const fn srt(&self) -> &SRT {
        &self.srt
    }
    #[doc = "0xa0 - Shadow TX Empty Trigger: This register is only valid when the DW_apb_uart is configured to have FIFOs implemented (FIFO_MODE != NONE) and THRE interrupt support implemented (THRE_MODE_USER == Enabled) and additional shadow registers implemented (SHADOW == YES). If FIFOs are not implemented or THRE interrupt support is not implemented or shadow registers are not implemented, this register does not exist and reading from this register address returns zero."]
    #[inline(always)]
    pub const fn stet(&self) -> &STET {
        &self.stet
    }
    #[doc = "0xa4 - Halt TX"]
    #[inline(always)]
    pub const fn htx(&self) -> &HTX {
        &self.htx
    }
    #[doc = "0xa8 - DMA Software Acknowledge"]
    #[inline(always)]
    pub const fn dmasa(&self) -> &DMASA {
        &self.dmasa
    }
    #[doc = "0xf4 - Component Parameter Register: This register is only valid when the DW_apb_uart is configured to have the Component Parameter register implemented (UART_ADD_ENCODED_PARAMS == YES). If the Component Parameter register is not implemented, this register does not exist and reading from this register address returns zero."]
    #[inline(always)]
    pub const fn cpr(&self) -> &CPR {
        &self.cpr
    }
    #[doc = "0xf8 - Component Type Register: This register is only valid when the DW_apb_uart is configured to have additional features implemented (ADDITIONAL_FEATURES == YES). If additional features are not implemented, this register does not exist and reading from this register address returns zero."]
    #[inline(always)]
    pub const fn ctr(&self) -> &CTR {
        unsafe { &*(self as *const Self).cast::<u8>().add(248).cast() }
    }
    #[doc = "0xf8 - UART Component Version: This register is only valid when the DW_apb_uart is configured to have additional features implemented (ADDITIONAL_FEATURES == YES). If additional features are not implemented, this register does not exist and reading from this register address returns zero."]
    #[inline(always)]
    pub const fn ucv(&self) -> &UCV {
        unsafe { &*(self as *const Self).cast::<u8>().add(248).cast() }
    }
}
#[doc = "rbr (rw) register accessor: Receive Buffer Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`rbr::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`rbr::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@rbr`]
module"]
pub type RBR = crate::Reg<rbr::RBR_SPEC>;
#[doc = "Receive Buffer Register"]
pub mod rbr;
#[doc = "thr (rw) register accessor: Transmit Holding Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`thr::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`thr::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@thr`]
module"]
pub type THR = crate::Reg<thr::THR_SPEC>;
#[doc = "Transmit Holding Register"]
pub mod thr;
#[doc = "dll (rw) register accessor: Divisor Latch Low\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`dll::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`dll::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@dll`]
module"]
pub type DLL = crate::Reg<dll::DLL_SPEC>;
#[doc = "Divisor Latch Low"]
pub mod dll;
#[doc = "dlh (rw) register accessor: Divisor Latch High\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`dlh::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`dlh::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@dlh`]
module"]
pub type DLH = crate::Reg<dlh::DLH_SPEC>;
#[doc = "Divisor Latch High"]
pub mod dlh;
#[doc = "ier (rw) register accessor: Interrupt Enable Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`ier::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`ier::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@ier`]
module"]
pub type IER = crate::Reg<ier::IER_SPEC>;
#[doc = "Interrupt Enable Register"]
pub mod ier;
#[doc = "iir (rw) register accessor: Interrupt Identity Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`iir::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`iir::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@iir`]
module"]
pub type IIR = crate::Reg<iir::IIR_SPEC>;
#[doc = "Interrupt Identity Register"]
pub mod iir;
#[doc = "fcr (rw) register accessor: FIFO Control Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`fcr::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`fcr::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@fcr`]
module"]
pub type FCR = crate::Reg<fcr::FCR_SPEC>;
#[doc = "FIFO Control Register"]
pub mod fcr;
#[doc = "lcr (rw) register accessor: Line Control Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`lcr::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`lcr::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@lcr`]
module"]
pub type LCR = crate::Reg<lcr::LCR_SPEC>;
#[doc = "Line Control Register"]
pub mod lcr;
#[doc = "mcr (rw) register accessor: Modem Control Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`mcr::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`mcr::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@mcr`]
module"]
pub type MCR = crate::Reg<mcr::MCR_SPEC>;
#[doc = "Modem Control Register"]
pub mod mcr;
#[doc = "lsr (rw) register accessor: Line Status Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`lsr::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`lsr::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@lsr`]
module"]
pub type LSR = crate::Reg<lsr::LSR_SPEC>;
#[doc = "Line Status Register"]
pub mod lsr;
#[doc = "msr (rw) register accessor: Line Status Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`msr::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`msr::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@msr`]
module"]
pub type MSR = crate::Reg<msr::MSR_SPEC>;
#[doc = "Line Status Register"]
pub mod msr;
#[doc = "scr (rw) register accessor: Scratch Pad Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`scr::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`scr::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@scr`]
module"]
pub type SCR = crate::Reg<scr::SCR_SPEC>;
#[doc = "Scratch Pad Register"]
pub mod scr;
#[doc = "lpdll (rw) register accessor: Low Power Divisor Latch Low Register: This register is only valid when the DW_apb_uart is configured to have SIR low-power reception capabilities implemented (SIR_LP_RX = Yes). If SIR low-power reception capabilities are not implemented, this register does not exist and reading from thsi register address returns zero.\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`lpdll::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`lpdll::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@lpdll`]
module"]
pub type LPDLL = crate::Reg<lpdll::LPDLL_SPEC>;
#[doc = "Low Power Divisor Latch Low Register: This register is only valid when the DW_apb_uart is configured to have SIR low-power reception capabilities implemented (SIR_LP_RX = Yes). If SIR low-power reception capabilities are not implemented, this register does not exist and reading from thsi register address returns zero."]
pub mod lpdll;
#[doc = "lpdlh (rw) register accessor: Low Power Divisor Latch High Register: This register is only valid when the DW_apb_uart is configured to have SIR low-power reception capabilities implemented (SIR_LP_RX = Yes). If SIR low-power reception capabilities are not implemented, this register does not exist and reading from thsi register address returns zero.\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`lpdlh::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`lpdlh::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@lpdlh`]
module"]
pub type LPDLH = crate::Reg<lpdlh::LPDLH_SPEC>;
#[doc = "Low Power Divisor Latch High Register: This register is only valid when the DW_apb_uart is configured to have SIR low-power reception capabilities implemented (SIR_LP_RX = Yes). If SIR low-power reception capabilities are not implemented, this register does not exist and reading from thsi register address returns zero."]
pub mod lpdlh;
#[doc = "srbr0 (rw) register accessor: Shadow Receive Buffer Register 0: This register is only valid when the DW_apb_uart is configured to have additional shadow registers implemented (SHADOW == YES). If shadow registers are not implemented, this register does not exist and reading from this register address returns zero.\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`srbr0::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`srbr0::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@srbr0`]
module"]
pub type SRBR0 = crate::Reg<srbr0::SRBR0_SPEC>;
#[doc = "Shadow Receive Buffer Register 0: This register is only valid when the DW_apb_uart is configured to have additional shadow registers implemented (SHADOW == YES). If shadow registers are not implemented, this register does not exist and reading from this register address returns zero."]
pub mod srbr0;
#[doc = "sthr0 (rw) register accessor: Shadow Transmit Holding Register 0: This register is only valid when the DW_apb_uart is configured to have additional shadow registers implemented (SHADOW == YES). If shadow registers are not implemented, this register does not exist and reading from this register address returns zero.\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`sthr0::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`sthr0::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@sthr0`]
module"]
pub type STHR0 = crate::Reg<sthr0::STHR0_SPEC>;
#[doc = "Shadow Transmit Holding Register 0: This register is only valid when the DW_apb_uart is configured to have additional shadow registers implemented (SHADOW == YES). If shadow registers are not implemented, this register does not exist and reading from this register address returns zero."]
pub mod sthr0;
#[doc = "srbr1 (rw) register accessor: Shadow Receive Buffer Register 1: This register is only valid when the DW_apb_uart is configured to have additional shadow registers implemented (SHADOW == YES). If shadow registers are not implemented, this register does not exist and reading from this register address returns zero.\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`srbr1::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`srbr1::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@srbr1`]
module"]
pub type SRBR1 = crate::Reg<srbr1::SRBR1_SPEC>;
#[doc = "Shadow Receive Buffer Register 1: This register is only valid when the DW_apb_uart is configured to have additional shadow registers implemented (SHADOW == YES). If shadow registers are not implemented, this register does not exist and reading from this register address returns zero."]
pub mod srbr1;
#[doc = "sthr1 (rw) register accessor: Shadow Transmit Holding Register 1: This register is only valid when the DW_apb_uart is configured to have additional shadow registers implemented (SHADOW == YES). If shadow registers are not implemented, this register does not exist and reading from this register address returns zero.\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`sthr1::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`sthr1::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@sthr1`]
module"]
pub type STHR1 = crate::Reg<sthr1::STHR1_SPEC>;
#[doc = "Shadow Transmit Holding Register 1: This register is only valid when the DW_apb_uart is configured to have additional shadow registers implemented (SHADOW == YES). If shadow registers are not implemented, this register does not exist and reading from this register address returns zero."]
pub mod sthr1;
#[doc = "srbr2 (rw) register accessor: Shadow Receive Buffer Register 2: This register is only valid when the DW_apb_uart is configured to have additional shadow registers implemented (SHADOW == YES). If shadow registers are not implemented, this register does not exist and reading from this register address returns zero.\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`srbr2::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`srbr2::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@srbr2`]
module"]
pub type SRBR2 = crate::Reg<srbr2::SRBR2_SPEC>;
#[doc = "Shadow Receive Buffer Register 2: This register is only valid when the DW_apb_uart is configured to have additional shadow registers implemented (SHADOW == YES). If shadow registers are not implemented, this register does not exist and reading from this register address returns zero."]
pub mod srbr2;
#[doc = "sthr2 (rw) register accessor: Shadow Transmit Holding Register 2: This register is only valid when the DW_apb_uart is configured to have additional shadow registers implemented (SHADOW == YES). If shadow registers are not implemented, this register does not exist and reading from this register address returns zero.\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`sthr2::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`sthr2::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@sthr2`]
module"]
pub type STHR2 = crate::Reg<sthr2::STHR2_SPEC>;
#[doc = "Shadow Transmit Holding Register 2: This register is only valid when the DW_apb_uart is configured to have additional shadow registers implemented (SHADOW == YES). If shadow registers are not implemented, this register does not exist and reading from this register address returns zero."]
pub mod sthr2;
#[doc = "srbr3 (rw) register accessor: Shadow Receive Buffer Register 3: This register is only valid when the DW_apb_uart is configured to have additional shadow registers implemented (SHADOW == YES). If shadow registers are not implemented, this register does not exist and reading from this register address returns zero.\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`srbr3::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`srbr3::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@srbr3`]
module"]
pub type SRBR3 = crate::Reg<srbr3::SRBR3_SPEC>;
#[doc = "Shadow Receive Buffer Register 3: This register is only valid when the DW_apb_uart is configured to have additional shadow registers implemented (SHADOW == YES). If shadow registers are not implemented, this register does not exist and reading from this register address returns zero."]
pub mod srbr3;
#[doc = "sthr3 (rw) register accessor: Shadow Transmit Holding Register 3: This register is only valid when the DW_apb_uart is configured to have additional shadow registers implemented (SHADOW == YES). If shadow registers are not implemented, this register does not exist and reading from this register address returns zero.\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`sthr3::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`sthr3::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@sthr3`]
module"]
pub type STHR3 = crate::Reg<sthr3::STHR3_SPEC>;
#[doc = "Shadow Transmit Holding Register 3: This register is only valid when the DW_apb_uart is configured to have additional shadow registers implemented (SHADOW == YES). If shadow registers are not implemented, this register does not exist and reading from this register address returns zero."]
pub mod sthr3;
#[doc = "srbr4 (rw) register accessor: Shadow Receive Buffer Register 4: This register is only valid when the DW_apb_uart is configured to have additional shadow registers implemented (SHADOW == YES). If shadow registers are not implemented, this register does not exist and reading from this register address returns zero.\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`srbr4::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`srbr4::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@srbr4`]
module"]
pub type SRBR4 = crate::Reg<srbr4::SRBR4_SPEC>;
#[doc = "Shadow Receive Buffer Register 4: This register is only valid when the DW_apb_uart is configured to have additional shadow registers implemented (SHADOW == YES). If shadow registers are not implemented, this register does not exist and reading from this register address returns zero."]
pub mod srbr4;
#[doc = "sthr4 (rw) register accessor: Shadow Transmit Holding Register 4: This register is only valid when the DW_apb_uart is configured to have additional shadow registers implemented (SHADOW == YES). If shadow registers are not implemented, this register does not exist and reading from this register address returns zero.\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`sthr4::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`sthr4::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@sthr4`]
module"]
pub type STHR4 = crate::Reg<sthr4::STHR4_SPEC>;
#[doc = "Shadow Transmit Holding Register 4: This register is only valid when the DW_apb_uart is configured to have additional shadow registers implemented (SHADOW == YES). If shadow registers are not implemented, this register does not exist and reading from this register address returns zero."]
pub mod sthr4;
#[doc = "srbr5 (rw) register accessor: Shadow Receive Buffer Register 5: This register is only valid when the DW_apb_uart is configured to have additional shadow registers implemented (SHADOW == YES). If shadow registers are not implemented, this register does not exist and reading from this register address returns zero.\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`srbr5::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`srbr5::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@srbr5`]
module"]
pub type SRBR5 = crate::Reg<srbr5::SRBR5_SPEC>;
#[doc = "Shadow Receive Buffer Register 5: This register is only valid when the DW_apb_uart is configured to have additional shadow registers implemented (SHADOW == YES). If shadow registers are not implemented, this register does not exist and reading from this register address returns zero."]
pub mod srbr5;
#[doc = "sthr5 (rw) register accessor: Shadow Transmit Holding Register 5: This register is only valid when the DW_apb_uart is configured to have additional shadow registers implemented (SHADOW == YES). If shadow registers are not implemented, this register does not exist and reading from this register address returns zero.\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`sthr5::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`sthr5::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@sthr5`]
module"]
pub type STHR5 = crate::Reg<sthr5::STHR5_SPEC>;
#[doc = "Shadow Transmit Holding Register 5: This register is only valid when the DW_apb_uart is configured to have additional shadow registers implemented (SHADOW == YES). If shadow registers are not implemented, this register does not exist and reading from this register address returns zero."]
pub mod sthr5;
#[doc = "srbr6 (rw) register accessor: Shadow Receive Buffer Register 6: This register is only valid when the DW_apb_uart is configured to have additional shadow registers implemented (SHADOW == YES). If shadow registers are not implemented, this register does not exist and reading from this register address returns zero.\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`srbr6::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`srbr6::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@srbr6`]
module"]
pub type SRBR6 = crate::Reg<srbr6::SRBR6_SPEC>;
#[doc = "Shadow Receive Buffer Register 6: This register is only valid when the DW_apb_uart is configured to have additional shadow registers implemented (SHADOW == YES). If shadow registers are not implemented, this register does not exist and reading from this register address returns zero."]
pub mod srbr6;
#[doc = "sthr6 (rw) register accessor: Shadow Transmit Holding Register 6: This register is only valid when the DW_apb_uart is configured to have additional shadow registers implemented (SHADOW == YES). If shadow registers are not implemented, this register does not exist and reading from this register address returns zero.\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`sthr6::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`sthr6::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@sthr6`]
module"]
pub type STHR6 = crate::Reg<sthr6::STHR6_SPEC>;
#[doc = "Shadow Transmit Holding Register 6: This register is only valid when the DW_apb_uart is configured to have additional shadow registers implemented (SHADOW == YES). If shadow registers are not implemented, this register does not exist and reading from this register address returns zero."]
pub mod sthr6;
#[doc = "srbr7 (rw) register accessor: Shadow Receive Buffer Register 7: This register is only valid when the DW_apb_uart is configured to have additional shadow registers implemented (SHADOW == YES). If shadow registers are not implemented, this register does not exist and reading from this register address returns zero.\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`srbr7::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`srbr7::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@srbr7`]
module"]
pub type SRBR7 = crate::Reg<srbr7::SRBR7_SPEC>;
#[doc = "Shadow Receive Buffer Register 7: This register is only valid when the DW_apb_uart is configured to have additional shadow registers implemented (SHADOW == YES). If shadow registers are not implemented, this register does not exist and reading from this register address returns zero."]
pub mod srbr7;
#[doc = "sthr7 (rw) register accessor: Shadow Transmit Holding Register 7: This register is only valid when the DW_apb_uart is configured to have additional shadow registers implemented (SHADOW == YES). If shadow registers are not implemented, this register does not exist and reading from this register address returns zero.\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`sthr7::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`sthr7::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@sthr7`]
module"]
pub type STHR7 = crate::Reg<sthr7::STHR7_SPEC>;
#[doc = "Shadow Transmit Holding Register 7: This register is only valid when the DW_apb_uart is configured to have additional shadow registers implemented (SHADOW == YES). If shadow registers are not implemented, this register does not exist and reading from this register address returns zero."]
pub mod sthr7;
#[doc = "srbr8 (rw) register accessor: Shadow Receive Buffer Register 8: This register is only valid when the DW_apb_uart is configured to have additional shadow registers implemented (SHADOW == YES). If shadow registers are not implemented, this register does not exist and reading from this register address returns zero.\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`srbr8::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`srbr8::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@srbr8`]
module"]
pub type SRBR8 = crate::Reg<srbr8::SRBR8_SPEC>;
#[doc = "Shadow Receive Buffer Register 8: This register is only valid when the DW_apb_uart is configured to have additional shadow registers implemented (SHADOW == YES). If shadow registers are not implemented, this register does not exist and reading from this register address returns zero."]
pub mod srbr8;
#[doc = "sthr8 (rw) register accessor: Shadow Transmit Holding Register 8: This register is only valid when the DW_apb_uart is configured to have additional shadow registers implemented (SHADOW == YES). If shadow registers are not implemented, this register does not exist and reading from this register address returns zero.\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`sthr8::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`sthr8::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@sthr8`]
module"]
pub type STHR8 = crate::Reg<sthr8::STHR8_SPEC>;
#[doc = "Shadow Transmit Holding Register 8: This register is only valid when the DW_apb_uart is configured to have additional shadow registers implemented (SHADOW == YES). If shadow registers are not implemented, this register does not exist and reading from this register address returns zero."]
pub mod sthr8;
#[doc = "srbr9 (rw) register accessor: Shadow Receive Buffer Register 9: This register is only valid when the DW_apb_uart is configured to have additional shadow registers implemented (SHADOW == YES). If shadow registers are not implemented, this register does not exist and reading from this register address returns zero.\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`srbr9::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`srbr9::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@srbr9`]
module"]
pub type SRBR9 = crate::Reg<srbr9::SRBR9_SPEC>;
#[doc = "Shadow Receive Buffer Register 9: This register is only valid when the DW_apb_uart is configured to have additional shadow registers implemented (SHADOW == YES). If shadow registers are not implemented, this register does not exist and reading from this register address returns zero."]
pub mod srbr9;
#[doc = "sthr9 (rw) register accessor: Shadow Transmit Holding Register 9: This register is only valid when the DW_apb_uart is configured to have additional shadow registers implemented (SHADOW == YES). If shadow registers are not implemented, this register does not exist and reading from this register address returns zero.\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`sthr9::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`sthr9::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@sthr9`]
module"]
pub type STHR9 = crate::Reg<sthr9::STHR9_SPEC>;
#[doc = "Shadow Transmit Holding Register 9: This register is only valid when the DW_apb_uart is configured to have additional shadow registers implemented (SHADOW == YES). If shadow registers are not implemented, this register does not exist and reading from this register address returns zero."]
pub mod sthr9;
#[doc = "srbr10 (rw) register accessor: Shadow Receive Buffer Register 10: This register is only valid when the DW_apb_uart is configured to have additional shadow registers implemented (SHADOW == YES). If shadow registers are not implemented, this register does not exist and reading from this register address returns zero.\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`srbr10::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`srbr10::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@srbr10`]
module"]
pub type SRBR10 = crate::Reg<srbr10::SRBR10_SPEC>;
#[doc = "Shadow Receive Buffer Register 10: This register is only valid when the DW_apb_uart is configured to have additional shadow registers implemented (SHADOW == YES). If shadow registers are not implemented, this register does not exist and reading from this register address returns zero."]
pub mod srbr10;
#[doc = "sthr10 (rw) register accessor: Shadow Transmit Holding Register 10: This register is only valid when the DW_apb_uart is configured to have additional shadow registers implemented (SHADOW == YES). If shadow registers are not implemented, this register does not exist and reading from this register address returns zero.\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`sthr10::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`sthr10::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@sthr10`]
module"]
pub type STHR10 = crate::Reg<sthr10::STHR10_SPEC>;
#[doc = "Shadow Transmit Holding Register 10: This register is only valid when the DW_apb_uart is configured to have additional shadow registers implemented (SHADOW == YES). If shadow registers are not implemented, this register does not exist and reading from this register address returns zero."]
pub mod sthr10;
#[doc = "srbr11 (rw) register accessor: Shadow Receive Buffer Register 11: This register is only valid when the DW_apb_uart is configured to have additional shadow registers implemented (SHADOW == YES). If shadow registers are not implemented, this register does not exist and reading from this register address returns zero.\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`srbr11::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`srbr11::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@srbr11`]
module"]
pub type SRBR11 = crate::Reg<srbr11::SRBR11_SPEC>;
#[doc = "Shadow Receive Buffer Register 11: This register is only valid when the DW_apb_uart is configured to have additional shadow registers implemented (SHADOW == YES). If shadow registers are not implemented, this register does not exist and reading from this register address returns zero."]
pub mod srbr11;
#[doc = "sthr11 (rw) register accessor: Shadow Transmit Holding Register 11: This register is only valid when the DW_apb_uart is configured to have additional shadow registers implemented (SHADOW == YES). If shadow registers are not implemented, this register does not exist and reading from this register address returns zero.\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`sthr11::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`sthr11::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@sthr11`]
module"]
pub type STHR11 = crate::Reg<sthr11::STHR11_SPEC>;
#[doc = "Shadow Transmit Holding Register 11: This register is only valid when the DW_apb_uart is configured to have additional shadow registers implemented (SHADOW == YES). If shadow registers are not implemented, this register does not exist and reading from this register address returns zero."]
pub mod sthr11;
#[doc = "srbr12 (rw) register accessor: Shadow Receive Buffer Register 12: This register is only valid when the DW_apb_uart is configured to have additional shadow registers implemented (SHADOW == YES). If shadow registers are not implemented, this register does not exist and reading from this register address returns zero.\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`srbr12::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`srbr12::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@srbr12`]
module"]
pub type SRBR12 = crate::Reg<srbr12::SRBR12_SPEC>;
#[doc = "Shadow Receive Buffer Register 12: This register is only valid when the DW_apb_uart is configured to have additional shadow registers implemented (SHADOW == YES). If shadow registers are not implemented, this register does not exist and reading from this register address returns zero."]
pub mod srbr12;
#[doc = "sthr12 (rw) register accessor: Shadow Transmit Holding Register 12: This register is only valid when the DW_apb_uart is configured to have additional shadow registers implemented (SHADOW == YES). If shadow registers are not implemented, this register does not exist and reading from this register address returns zero.\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`sthr12::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`sthr12::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@sthr12`]
module"]
pub type STHR12 = crate::Reg<sthr12::STHR12_SPEC>;
#[doc = "Shadow Transmit Holding Register 12: This register is only valid when the DW_apb_uart is configured to have additional shadow registers implemented (SHADOW == YES). If shadow registers are not implemented, this register does not exist and reading from this register address returns zero."]
pub mod sthr12;
#[doc = "srbr13 (rw) register accessor: Shadow Receive Buffer Register 13: This register is only valid when the DW_apb_uart is configured to have additional shadow registers implemented (SHADOW == YES). If shadow registers are not implemented, this register does not exist and reading from this register address returns zero.\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`srbr13::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`srbr13::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@srbr13`]
module"]
pub type SRBR13 = crate::Reg<srbr13::SRBR13_SPEC>;
#[doc = "Shadow Receive Buffer Register 13: This register is only valid when the DW_apb_uart is configured to have additional shadow registers implemented (SHADOW == YES). If shadow registers are not implemented, this register does not exist and reading from this register address returns zero."]
pub mod srbr13;
#[doc = "sthr13 (rw) register accessor: Shadow Transmit Holding Register 13: This register is only valid when the DW_apb_uart is configured to have additional shadow registers implemented (SHADOW == YES). If shadow registers are not implemented, this register does not exist and reading from this register address returns zero.\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`sthr13::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`sthr13::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@sthr13`]
module"]
pub type STHR13 = crate::Reg<sthr13::STHR13_SPEC>;
#[doc = "Shadow Transmit Holding Register 13: This register is only valid when the DW_apb_uart is configured to have additional shadow registers implemented (SHADOW == YES). If shadow registers are not implemented, this register does not exist and reading from this register address returns zero."]
pub mod sthr13;
#[doc = "srbr14 (rw) register accessor: Shadow Receive Buffer Register 14: This register is only valid when the DW_apb_uart is configured to have additional shadow registers implemented (SHADOW == YES). If shadow registers are not implemented, this register does not exist and reading from this register address returns zero.\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`srbr14::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`srbr14::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@srbr14`]
module"]
pub type SRBR14 = crate::Reg<srbr14::SRBR14_SPEC>;
#[doc = "Shadow Receive Buffer Register 14: This register is only valid when the DW_apb_uart is configured to have additional shadow registers implemented (SHADOW == YES). If shadow registers are not implemented, this register does not exist and reading from this register address returns zero."]
pub mod srbr14;
#[doc = "sthr14 (rw) register accessor: Shadow Transmit Holding Register 14: This register is only valid when the DW_apb_uart is configured to have additional shadow registers implemented (SHADOW == YES). If shadow registers are not implemented, this register does not exist and reading from this register address returns zero.\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`sthr14::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`sthr14::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@sthr14`]
module"]
pub type STHR14 = crate::Reg<sthr14::STHR14_SPEC>;
#[doc = "Shadow Transmit Holding Register 14: This register is only valid when the DW_apb_uart is configured to have additional shadow registers implemented (SHADOW == YES). If shadow registers are not implemented, this register does not exist and reading from this register address returns zero."]
pub mod sthr14;
#[doc = "srbr15 (rw) register accessor: Shadow Receive Buffer Register 15: This register is only valid when the DW_apb_uart is configured to have additional shadow registers implemented (SHADOW == YES). If shadow registers are not implemented, this register does not exist and reading from this register address returns zero.\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`srbr15::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`srbr15::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@srbr15`]
module"]
pub type SRBR15 = crate::Reg<srbr15::SRBR15_SPEC>;
#[doc = "Shadow Receive Buffer Register 15: This register is only valid when the DW_apb_uart is configured to have additional shadow registers implemented (SHADOW == YES). If shadow registers are not implemented, this register does not exist and reading from this register address returns zero."]
pub mod srbr15;
#[doc = "sthr15 (rw) register accessor: Shadow Transmit Holding Register 15: This register is only valid when the DW_apb_uart is configured to have additional shadow registers implemented (SHADOW == YES). If shadow registers are not implemented, this register does not exist and reading from this register address returns zero.\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`sthr15::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`sthr15::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@sthr15`]
module"]
pub type STHR15 = crate::Reg<sthr15::STHR15_SPEC>;
#[doc = "Shadow Transmit Holding Register 15: This register is only valid when the DW_apb_uart is configured to have additional shadow registers implemented (SHADOW == YES). If shadow registers are not implemented, this register does not exist and reading from this register address returns zero."]
pub mod sthr15;
#[doc = "far (rw) register accessor: FIFO Access Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`far::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`far::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@far`]
module"]
pub type FAR = crate::Reg<far::FAR_SPEC>;
#[doc = "FIFO Access Register"]
pub mod far;
#[doc = "tfr (rw) register accessor: Transmit FIFO Read\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`tfr::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`tfr::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@tfr`]
module"]
pub type TFR = crate::Reg<tfr::TFR_SPEC>;
#[doc = "Transmit FIFO Read"]
pub mod tfr;
#[doc = "rfw (rw) register accessor: Receive FIFO Write\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`rfw::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`rfw::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@rfw`]
module"]
pub type RFW = crate::Reg<rfw::RFW_SPEC>;
#[doc = "Receive FIFO Write"]
pub mod rfw;
#[doc = "usr (rw) register accessor: UART Status Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`usr::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`usr::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@usr`]
module"]
pub type USR = crate::Reg<usr::USR_SPEC>;
#[doc = "UART Status Register"]
pub mod usr;
#[doc = "tfl (rw) register accessor: Transmit FIFO Level: This register is only valid when the DW_apb_uart is configured to have additional FIFO status registers implemented (FIFO_STAT == YES). If status registers are not implemented, this register does not exist and reading from this register address returns zero.\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`tfl::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`tfl::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@tfl`]
module"]
pub type TFL = crate::Reg<tfl::TFL_SPEC>;
#[doc = "Transmit FIFO Level: This register is only valid when the DW_apb_uart is configured to have additional FIFO status registers implemented (FIFO_STAT == YES). If status registers are not implemented, this register does not exist and reading from this register address returns zero."]
pub mod tfl;
#[doc = "rfl (rw) register accessor: Receive FIFO Level: This register is only valid when the DW_apb_uart is configured to have additional FIFO status registers implemented (FIFO_STAT == YES). If status registers are not implemented, this register does not exist and reading from this register address returns zero.\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`rfl::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`rfl::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@rfl`]
module"]
pub type RFL = crate::Reg<rfl::RFL_SPEC>;
#[doc = "Receive FIFO Level: This register is only valid when the DW_apb_uart is configured to have additional FIFO status registers implemented (FIFO_STAT == YES). If status registers are not implemented, this register does not exist and reading from this register address returns zero."]
pub mod rfl;
#[doc = "srr (rw) register accessor: Software Reset Register: This register is only valid when the DW_apb_uart is configured to have additional shadow registers implemented (SHADOW == YES). If shadow registers are not implemented, this register does not exist and reading from this register address returns zero.\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`srr::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`srr::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@srr`]
module"]
pub type SRR = crate::Reg<srr::SRR_SPEC>;
#[doc = "Software Reset Register: This register is only valid when the DW_apb_uart is configured to have additional shadow registers implemented (SHADOW == YES). If shadow registers are not implemented, this register does not exist and reading from this register address returns zero."]
pub mod srr;
#[doc = "srts (rw) register accessor: Shadow Request to Send: This register is only valid when the DW_apb_uart is configured to have additional shadow registers implemented (SHADOW == YES). If shadow registers are not implemented, this register does not exist and reading from this register address returns zero.\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`srts::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`srts::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@srts`]
module"]
pub type SRTS = crate::Reg<srts::SRTS_SPEC>;
#[doc = "Shadow Request to Send: This register is only valid when the DW_apb_uart is configured to have additional shadow registers implemented (SHADOW == YES). If shadow registers are not implemented, this register does not exist and reading from this register address returns zero."]
pub mod srts;
#[doc = "sbcr (rw) register accessor: Shadow Break Control Register: This register is only valid when the DW_apb_uart is configured to have additional shadow registers implemented (SHADOW == YES). If shadow registers are not implemented, this register does not exist and reading from this register address returns zero.\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`sbcr::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`sbcr::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@sbcr`]
module"]
pub type SBCR = crate::Reg<sbcr::SBCR_SPEC>;
#[doc = "Shadow Break Control Register: This register is only valid when the DW_apb_uart is configured to have additional shadow registers implemented (SHADOW == YES). If shadow registers are not implemented, this register does not exist and reading from this register address returns zero."]
pub mod sbcr;
#[doc = "sdmam (rw) register accessor: Shadow DMA Mode: This register is only valid when the DW_apb_uart is configured to have additional FIFO registers implemented (FIFO_MODE != None) and additional shadow registers implemented (SHADOW == YES). If these registers are not implemented, this register does not exist and reading from this register address returns zero.\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`sdmam::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`sdmam::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@sdmam`]
module"]
pub type SDMAM = crate::Reg<sdmam::SDMAM_SPEC>;
#[doc = "Shadow DMA Mode: This register is only valid when the DW_apb_uart is configured to have additional FIFO registers implemented (FIFO_MODE != None) and additional shadow registers implemented (SHADOW == YES). If these registers are not implemented, this register does not exist and reading from this register address returns zero."]
pub mod sdmam;
#[doc = "sfe (rw) register accessor: Shadow FIFO Enable: This register is only valid when the DW_apb_uart is configured to have additional FIFO registers implemented (FIFO_MODE != None) and additional shadow registers implemented (SHADOW == YES). If these registers are not implemented, this register does not exist and reading from this register address returns zero.\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`sfe::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`sfe::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@sfe`]
module"]
pub type SFE = crate::Reg<sfe::SFE_SPEC>;
#[doc = "Shadow FIFO Enable: This register is only valid when the DW_apb_uart is configured to have additional FIFO registers implemented (FIFO_MODE != None) and additional shadow registers implemented (SHADOW == YES). If these registers are not implemented, this register does not exist and reading from this register address returns zero."]
pub mod sfe;
#[doc = "srt (rw) register accessor: Shadow RCVR Trigger: This register is only valid when the DW_apb_uart is configured to have additional FIFO registers implemented (FIFO_MODE != None) and additional shadow registers implemented (SHADOW == YES). If these registers are not implemented, this register does not exist and reading from this register address returns zero.\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`srt::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`srt::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@srt`]
module"]
pub type SRT = crate::Reg<srt::SRT_SPEC>;
#[doc = "Shadow RCVR Trigger: This register is only valid when the DW_apb_uart is configured to have additional FIFO registers implemented (FIFO_MODE != None) and additional shadow registers implemented (SHADOW == YES). If these registers are not implemented, this register does not exist and reading from this register address returns zero."]
pub mod srt;
#[doc = "stet (rw) register accessor: Shadow TX Empty Trigger: This register is only valid when the DW_apb_uart is configured to have FIFOs implemented (FIFO_MODE != NONE) and THRE interrupt support implemented (THRE_MODE_USER == Enabled) and additional shadow registers implemented (SHADOW == YES). If FIFOs are not implemented or THRE interrupt support is not implemented or shadow registers are not implemented, this register does not exist and reading from this register address returns zero.\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`stet::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`stet::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@stet`]
module"]
pub type STET = crate::Reg<stet::STET_SPEC>;
#[doc = "Shadow TX Empty Trigger: This register is only valid when the DW_apb_uart is configured to have FIFOs implemented (FIFO_MODE != NONE) and THRE interrupt support implemented (THRE_MODE_USER == Enabled) and additional shadow registers implemented (SHADOW == YES). If FIFOs are not implemented or THRE interrupt support is not implemented or shadow registers are not implemented, this register does not exist and reading from this register address returns zero."]
pub mod stet;
#[doc = "htx (rw) register accessor: Halt TX\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`htx::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`htx::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@htx`]
module"]
pub type HTX = crate::Reg<htx::HTX_SPEC>;
#[doc = "Halt TX"]
pub mod htx;
#[doc = "dmasa (rw) register accessor: DMA Software Acknowledge\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`dmasa::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`dmasa::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@dmasa`]
module"]
pub type DMASA = crate::Reg<dmasa::DMASA_SPEC>;
#[doc = "DMA Software Acknowledge"]
pub mod dmasa;
#[doc = "cpr (rw) register accessor: Component Parameter Register: This register is only valid when the DW_apb_uart is configured to have the Component Parameter register implemented (UART_ADD_ENCODED_PARAMS == YES). If the Component Parameter register is not implemented, this register does not exist and reading from this register address returns zero.\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`cpr::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`cpr::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@cpr`]
module"]
pub type CPR = crate::Reg<cpr::CPR_SPEC>;
#[doc = "Component Parameter Register: This register is only valid when the DW_apb_uart is configured to have the Component Parameter register implemented (UART_ADD_ENCODED_PARAMS == YES). If the Component Parameter register is not implemented, this register does not exist and reading from this register address returns zero."]
pub mod cpr;
#[doc = "ucv (rw) register accessor: UART Component Version: This register is only valid when the DW_apb_uart is configured to have additional features implemented (ADDITIONAL_FEATURES == YES). If additional features are not implemented, this register does not exist and reading from this register address returns zero.\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`ucv::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`ucv::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@ucv`]
module"]
pub type UCV = crate::Reg<ucv::UCV_SPEC>;
#[doc = "UART Component Version: This register is only valid when the DW_apb_uart is configured to have additional features implemented (ADDITIONAL_FEATURES == YES). If additional features are not implemented, this register does not exist and reading from this register address returns zero."]
pub mod ucv;
#[doc = "ctr (rw) register accessor: Component Type Register: This register is only valid when the DW_apb_uart is configured to have additional features implemented (ADDITIONAL_FEATURES == YES). If additional features are not implemented, this register does not exist and reading from this register address returns zero.\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`ctr::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`ctr::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@ctr`]
module"]
pub type CTR = crate::Reg<ctr::CTR_SPEC>;
#[doc = "Component Type Register: This register is only valid when the DW_apb_uart is configured to have additional features implemented (ADDITIONAL_FEATURES == YES). If additional features are not implemented, this register does not exist and reading from this register address returns zero."]
pub mod ctr;
