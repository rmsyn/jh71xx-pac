#[doc = r"Register block"]
#[repr(C)]
pub struct RegisterBlock {
    ssp_cr0: SSP_CR0,
    _reserved1: [u8; 0x02],
    ssp_cr1: SSP_CR1,
    _reserved2: [u8; 0x02],
    ssp_dr: SSP_DR,
    _reserved3: [u8; 0x02],
    ssp_sr: SSP_SR,
    _reserved4: [u8; 0x02],
    ssp_cpsr: SSP_CPSR,
    _reserved5: [u8; 0x02],
    ssp_imsc: SSP_IMSC,
    _reserved6: [u8; 0x02],
    ssp_ris: SSP_RIS,
    _reserved7: [u8; 0x02],
    ssp_mis: SSP_MIS,
    _reserved8: [u8; 0x02],
    ssp_icr: SSP_ICR,
    _reserved9: [u8; 0x02],
    ssp_dmacr: SSP_DMACR,
    _reserved10: [u8; 0x0fba],
    ssp_periph_id0: SSP_PERIPH_ID0,
    _reserved11: [u8; 0x02],
    ssp_periph_id1: SSP_PERIPH_ID1,
    _reserved12: [u8; 0x02],
    ssp_periph_id2: SSP_PERIPH_ID2,
    _reserved13: [u8; 0x02],
    ssp_periph_id3: SSP_PERIPH_ID3,
    _reserved14: [u8; 0x02],
    ssp_pcell_id0: SSP_PCELL_ID0,
    _reserved15: [u8; 0x02],
    ssp_pcell_id1: SSP_PCELL_ID1,
    _reserved16: [u8; 0x02],
    ssp_pcell_id2: SSP_PCELL_ID2,
    _reserved17: [u8; 0x02],
    ssp_pcell_id3: SSP_PCELL_ID3,
}
impl RegisterBlock {
    #[doc = "0x00 - SSPCR0 is control register 0 and contains five bit fields that control various functions within the PrimeCell SSP."]
    #[inline(always)]
    pub const fn ssp_cr0(&self) -> &SSP_CR0 {
        &self.ssp_cr0
    }
    #[doc = "0x04 - SSPCR1 is the control register 1 and contains four different bit fields, that control various functions within the PrimeCell SSP."]
    #[inline(always)]
    pub const fn ssp_cr1(&self) -> &SSP_CR1 {
        &self.ssp_cr1
    }
    #[doc = "0x08 - SSPDR is the data register and is 16-bits wide. When SSPDR is read, the entry in the receive FIFO, pointed to by the current FIFO read pointer, is accessed. As data values are removed by the PrimeCell SSP receive logic from the incoming data frame, they are placed into the entry in the receive FIFO, pointed to by the current FIFO write pointer. When SSPDR is written to, the entry in the transmit FIFO, pointed to by the write pointer, is written to. Data values are removed from the transmit FIFO one value at a time by the transmit logic. It is loaded into the transmit serial shifter, then serially shifted out onto the SSPTXD pin at the programmed bit rate. When a data size of less than 16 bits is selected, the user must right-justify data written to the transmit FIFO. The transmit logic ignores the unused bits. Received data less than 16 bits is automatically right-justified in the receive buffer."]
    #[inline(always)]
    pub const fn ssp_dr(&self) -> &SSP_DR {
        &self.ssp_dr
    }
    #[doc = "0x0c - SSPSR is a RO status register that contains bits that indicate the FIFO fill status and the PrimeCell SSP busy status."]
    #[inline(always)]
    pub const fn ssp_sr(&self) -> &SSP_SR {
        &self.ssp_sr
    }
    #[doc = "0x10 - SSPCPSR is the clock prescale register and specifies the division factor by which the input SSPCLK must be internally divided before further use. The value programmed into this register must be an even number between \\[2:254\\]. The least significant bit of the programmed number is hard-coded to zero. If an odd number is written to this register, data read back from this register has the least significant bit as zero."]
    #[inline(always)]
    pub const fn ssp_cpsr(&self) -> &SSP_CPSR {
        &self.ssp_cpsr
    }
    #[doc = "0x14 - The SSPIMSC register is the interrupt mask set or clear register. It is a RW register. On a read this register gives the current value of the mask on the relevant interrupt. A write of 1 to the particular bit sets the mask, enabling the interrupt to be read. A write of 0 clears the corresponding mask. All the bits are cleared to 0 when reset."]
    #[inline(always)]
    pub const fn ssp_imsc(&self) -> &SSP_IMSC {
        &self.ssp_imsc
    }
    #[doc = "0x18 - The SSPRIS register is the raw interrupt status register. It is a RO register. On a read this register gives the current raw status value of the corresponding interrupt prior to masking. A write has no effect."]
    #[inline(always)]
    pub const fn ssp_ris(&self) -> &SSP_RIS {
        &self.ssp_ris
    }
    #[doc = "0x1c - The SSPMIS register is the masked interrupt status register. It is a RO register. On a read this register gives the current masked status value of the corresponding interrupt. A write has no effect."]
    #[inline(always)]
    pub const fn ssp_mis(&self) -> &SSP_MIS {
        &self.ssp_mis
    }
    #[doc = "0x20 - The SSPICR register is the interrupt clear register and is write-only. On a write of 1, the corresponding interrupt is cleared. A write of 0 has no effect."]
    #[inline(always)]
    pub const fn ssp_icr(&self) -> &SSP_ICR {
        &self.ssp_icr
    }
    #[doc = "0x24 - The SSPDMACR register is the DMA control register. It is a RW register. All the bits are cleared to 0 on reset."]
    #[inline(always)]
    pub const fn ssp_dmacr(&self) -> &SSP_DMACR {
        &self.ssp_dmacr
    }
    #[doc = "0xfe0 - The SSPPeriphID0 register is hard-coded and the fields within the register determine reset value. The SSPPeriphID0-3 registers are four 8-bit registers, that span address locations 0xFE0 to 0xFEC. The registers can conceptually be treated as a single 32-bit register."]
    #[inline(always)]
    pub const fn ssp_periph_id0(&self) -> &SSP_PERIPH_ID0 {
        &self.ssp_periph_id0
    }
    #[doc = "0xfe4 - The SSPPeriphID1 register is hard-coded and the fields within the register determine reset value. The SSPPeriphID0-3 registers are four 8-bit registers, that span address locations 0xFE0 to 0xFEC. The registers can conceptually be treated as a single 32-bit register."]
    #[inline(always)]
    pub const fn ssp_periph_id1(&self) -> &SSP_PERIPH_ID1 {
        &self.ssp_periph_id1
    }
    #[doc = "0xfe8 - The SSPPeriphID2 register is hard-coded and the fields within the register determine reset value. The SSPPeriphID0-3 registers are four 8-bit registers, that span address locations 0xFE0 to 0xFEC. The registers can conceptually be treated as a single 32-bit register."]
    #[inline(always)]
    pub const fn ssp_periph_id2(&self) -> &SSP_PERIPH_ID2 {
        &self.ssp_periph_id2
    }
    #[doc = "0xfec - The SSPPeriphID3 register is hard-coded and the fields within the register determine reset value. The SSPPeriphID0-3 registers are four 8-bit registers, that span address locations 0xFE0 to 0xFEC. The registers can conceptually be treated as a single 32-bit register."]
    #[inline(always)]
    pub const fn ssp_periph_id3(&self) -> &SSP_PERIPH_ID3 {
        &self.ssp_periph_id3
    }
    #[doc = "0xff0 - The SSPPCellID0-3 registers are four 8-bit wide registers, that span address locations 0xFF0-0xFFC. The registers can conceptually be treated as a 32-bit register. The register is used as a standard cross-peripheral identification system. The SSPPCellID register is set to 0xB105F00D."]
    #[inline(always)]
    pub const fn ssp_pcell_id0(&self) -> &SSP_PCELL_ID0 {
        &self.ssp_pcell_id0
    }
    #[doc = "0xff4 - The SSPPCellID0-3 registers are four 8-bit wide registers, that span address locations 0xFF0-0xFFC. The registers can conceptually be treated as a 32-bit register. The register is used as a standard cross-peripheral identification system. The SSPPCellID register is set to 0xB105F00D."]
    #[inline(always)]
    pub const fn ssp_pcell_id1(&self) -> &SSP_PCELL_ID1 {
        &self.ssp_pcell_id1
    }
    #[doc = "0xff8 - The SSPPCellID0-3 registers are four 8-bit wide registers, that span address locations 0xFF0-0xFFC. The registers can conceptually be treated as a 32-bit register. The register is used as a standard cross-peripheral identification system. The SSPPCellID register is set to 0xB105F00D."]
    #[inline(always)]
    pub const fn ssp_pcell_id2(&self) -> &SSP_PCELL_ID2 {
        &self.ssp_pcell_id2
    }
    #[doc = "0xffc - The SSPPCellID0-3 registers are four 8-bit wide registers, that span address locations 0xFF0-0xFFC. The registers can conceptually be treated as a 32-bit register. The register is used as a standard cross-peripheral identification system. The SSPPCellID register is set to 0xB105F00D."]
    #[inline(always)]
    pub const fn ssp_pcell_id3(&self) -> &SSP_PCELL_ID3 {
        &self.ssp_pcell_id3
    }
}
#[doc = "ssp_cr0 (rw) register accessor: SSPCR0 is control register 0 and contains five bit fields that control various functions within the PrimeCell SSP.\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`ssp_cr0::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`ssp_cr0::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@ssp_cr0`]
module"]
pub type SSP_CR0 = crate::Reg<ssp_cr0::SSP_CR0_SPEC>;
#[doc = "SSPCR0 is control register 0 and contains five bit fields that control various functions within the PrimeCell SSP."]
pub mod ssp_cr0;
#[doc = "ssp_cr1 (rw) register accessor: SSPCR1 is the control register 1 and contains four different bit fields, that control various functions within the PrimeCell SSP.\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`ssp_cr1::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`ssp_cr1::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@ssp_cr1`]
module"]
pub type SSP_CR1 = crate::Reg<ssp_cr1::SSP_CR1_SPEC>;
#[doc = "SSPCR1 is the control register 1 and contains four different bit fields, that control various functions within the PrimeCell SSP."]
pub mod ssp_cr1;
#[doc = "ssp_dr (rw) register accessor: SSPDR is the data register and is 16-bits wide. When SSPDR is read, the entry in the receive FIFO, pointed to by the current FIFO read pointer, is accessed. As data values are removed by the PrimeCell SSP receive logic from the incoming data frame, they are placed into the entry in the receive FIFO, pointed to by the current FIFO write pointer. When SSPDR is written to, the entry in the transmit FIFO, pointed to by the write pointer, is written to. Data values are removed from the transmit FIFO one value at a time by the transmit logic. It is loaded into the transmit serial shifter, then serially shifted out onto the SSPTXD pin at the programmed bit rate. When a data size of less than 16 bits is selected, the user must right-justify data written to the transmit FIFO. The transmit logic ignores the unused bits. Received data less than 16 bits is automatically right-justified in the receive buffer.\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`ssp_dr::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`ssp_dr::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@ssp_dr`]
module"]
pub type SSP_DR = crate::Reg<ssp_dr::SSP_DR_SPEC>;
#[doc = "SSPDR is the data register and is 16-bits wide. When SSPDR is read, the entry in the receive FIFO, pointed to by the current FIFO read pointer, is accessed. As data values are removed by the PrimeCell SSP receive logic from the incoming data frame, they are placed into the entry in the receive FIFO, pointed to by the current FIFO write pointer. When SSPDR is written to, the entry in the transmit FIFO, pointed to by the write pointer, is written to. Data values are removed from the transmit FIFO one value at a time by the transmit logic. It is loaded into the transmit serial shifter, then serially shifted out onto the SSPTXD pin at the programmed bit rate. When a data size of less than 16 bits is selected, the user must right-justify data written to the transmit FIFO. The transmit logic ignores the unused bits. Received data less than 16 bits is automatically right-justified in the receive buffer."]
pub mod ssp_dr;
#[doc = "ssp_sr (rw) register accessor: SSPSR is a RO status register that contains bits that indicate the FIFO fill status and the PrimeCell SSP busy status.\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`ssp_sr::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`ssp_sr::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@ssp_sr`]
module"]
pub type SSP_SR = crate::Reg<ssp_sr::SSP_SR_SPEC>;
#[doc = "SSPSR is a RO status register that contains bits that indicate the FIFO fill status and the PrimeCell SSP busy status."]
pub mod ssp_sr;
#[doc = "ssp_cpsr (rw) register accessor: SSPCPSR is the clock prescale register and specifies the division factor by which the input SSPCLK must be internally divided before further use. The value programmed into this register must be an even number between \\[2:254\\]. The least significant bit of the programmed number is hard-coded to zero. If an odd number is written to this register, data read back from this register has the least significant bit as zero.\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`ssp_cpsr::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`ssp_cpsr::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@ssp_cpsr`]
module"]
pub type SSP_CPSR = crate::Reg<ssp_cpsr::SSP_CPSR_SPEC>;
#[doc = "SSPCPSR is the clock prescale register and specifies the division factor by which the input SSPCLK must be internally divided before further use. The value programmed into this register must be an even number between \\[2:254\\]. The least significant bit of the programmed number is hard-coded to zero. If an odd number is written to this register, data read back from this register has the least significant bit as zero."]
pub mod ssp_cpsr;
#[doc = "ssp_imsc (rw) register accessor: The SSPIMSC register is the interrupt mask set or clear register. It is a RW register. On a read this register gives the current value of the mask on the relevant interrupt. A write of 1 to the particular bit sets the mask, enabling the interrupt to be read. A write of 0 clears the corresponding mask. All the bits are cleared to 0 when reset.\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`ssp_imsc::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`ssp_imsc::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@ssp_imsc`]
module"]
pub type SSP_IMSC = crate::Reg<ssp_imsc::SSP_IMSC_SPEC>;
#[doc = "The SSPIMSC register is the interrupt mask set or clear register. It is a RW register. On a read this register gives the current value of the mask on the relevant interrupt. A write of 1 to the particular bit sets the mask, enabling the interrupt to be read. A write of 0 clears the corresponding mask. All the bits are cleared to 0 when reset."]
pub mod ssp_imsc;
#[doc = "ssp_ris (rw) register accessor: The SSPRIS register is the raw interrupt status register. It is a RO register. On a read this register gives the current raw status value of the corresponding interrupt prior to masking. A write has no effect.\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`ssp_ris::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`ssp_ris::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@ssp_ris`]
module"]
pub type SSP_RIS = crate::Reg<ssp_ris::SSP_RIS_SPEC>;
#[doc = "The SSPRIS register is the raw interrupt status register. It is a RO register. On a read this register gives the current raw status value of the corresponding interrupt prior to masking. A write has no effect."]
pub mod ssp_ris;
#[doc = "ssp_mis (rw) register accessor: The SSPMIS register is the masked interrupt status register. It is a RO register. On a read this register gives the current masked status value of the corresponding interrupt. A write has no effect.\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`ssp_mis::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`ssp_mis::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@ssp_mis`]
module"]
pub type SSP_MIS = crate::Reg<ssp_mis::SSP_MIS_SPEC>;
#[doc = "The SSPMIS register is the masked interrupt status register. It is a RO register. On a read this register gives the current masked status value of the corresponding interrupt. A write has no effect."]
pub mod ssp_mis;
#[doc = "ssp_icr (rw) register accessor: The SSPICR register is the interrupt clear register and is write-only. On a write of 1, the corresponding interrupt is cleared. A write of 0 has no effect.\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`ssp_icr::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`ssp_icr::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@ssp_icr`]
module"]
pub type SSP_ICR = crate::Reg<ssp_icr::SSP_ICR_SPEC>;
#[doc = "The SSPICR register is the interrupt clear register and is write-only. On a write of 1, the corresponding interrupt is cleared. A write of 0 has no effect."]
pub mod ssp_icr;
#[doc = "ssp_dmacr (rw) register accessor: The SSPDMACR register is the DMA control register. It is a RW register. All the bits are cleared to 0 on reset.\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`ssp_dmacr::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`ssp_dmacr::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@ssp_dmacr`]
module"]
pub type SSP_DMACR = crate::Reg<ssp_dmacr::SSP_DMACR_SPEC>;
#[doc = "The SSPDMACR register is the DMA control register. It is a RW register. All the bits are cleared to 0 on reset."]
pub mod ssp_dmacr;
#[doc = "ssp_periph_id0 (rw) register accessor: The SSPPeriphID0 register is hard-coded and the fields within the register determine reset value. The SSPPeriphID0-3 registers are four 8-bit registers, that span address locations 0xFE0 to 0xFEC. The registers can conceptually be treated as a single 32-bit register.\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`ssp_periph_id0::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`ssp_periph_id0::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@ssp_periph_id0`]
module"]
pub type SSP_PERIPH_ID0 = crate::Reg<ssp_periph_id0::SSP_PERIPH_ID0_SPEC>;
#[doc = "The SSPPeriphID0 register is hard-coded and the fields within the register determine reset value. The SSPPeriphID0-3 registers are four 8-bit registers, that span address locations 0xFE0 to 0xFEC. The registers can conceptually be treated as a single 32-bit register."]
pub mod ssp_periph_id0;
#[doc = "ssp_periph_id1 (rw) register accessor: The SSPPeriphID1 register is hard-coded and the fields within the register determine reset value. The SSPPeriphID0-3 registers are four 8-bit registers, that span address locations 0xFE0 to 0xFEC. The registers can conceptually be treated as a single 32-bit register.\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`ssp_periph_id1::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`ssp_periph_id1::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@ssp_periph_id1`]
module"]
pub type SSP_PERIPH_ID1 = crate::Reg<ssp_periph_id1::SSP_PERIPH_ID1_SPEC>;
#[doc = "The SSPPeriphID1 register is hard-coded and the fields within the register determine reset value. The SSPPeriphID0-3 registers are four 8-bit registers, that span address locations 0xFE0 to 0xFEC. The registers can conceptually be treated as a single 32-bit register."]
pub mod ssp_periph_id1;
#[doc = "ssp_periph_id2 (rw) register accessor: The SSPPeriphID2 register is hard-coded and the fields within the register determine reset value. The SSPPeriphID0-3 registers are four 8-bit registers, that span address locations 0xFE0 to 0xFEC. The registers can conceptually be treated as a single 32-bit register.\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`ssp_periph_id2::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`ssp_periph_id2::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@ssp_periph_id2`]
module"]
pub type SSP_PERIPH_ID2 = crate::Reg<ssp_periph_id2::SSP_PERIPH_ID2_SPEC>;
#[doc = "The SSPPeriphID2 register is hard-coded and the fields within the register determine reset value. The SSPPeriphID0-3 registers are four 8-bit registers, that span address locations 0xFE0 to 0xFEC. The registers can conceptually be treated as a single 32-bit register."]
pub mod ssp_periph_id2;
#[doc = "ssp_periph_id3 (rw) register accessor: The SSPPeriphID3 register is hard-coded and the fields within the register determine reset value. The SSPPeriphID0-3 registers are four 8-bit registers, that span address locations 0xFE0 to 0xFEC. The registers can conceptually be treated as a single 32-bit register.\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`ssp_periph_id3::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`ssp_periph_id3::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@ssp_periph_id3`]
module"]
pub type SSP_PERIPH_ID3 = crate::Reg<ssp_periph_id3::SSP_PERIPH_ID3_SPEC>;
#[doc = "The SSPPeriphID3 register is hard-coded and the fields within the register determine reset value. The SSPPeriphID0-3 registers are four 8-bit registers, that span address locations 0xFE0 to 0xFEC. The registers can conceptually be treated as a single 32-bit register."]
pub mod ssp_periph_id3;
#[doc = "ssp_pcell_id0 (rw) register accessor: The SSPPCellID0-3 registers are four 8-bit wide registers, that span address locations 0xFF0-0xFFC. The registers can conceptually be treated as a 32-bit register. The register is used as a standard cross-peripheral identification system. The SSPPCellID register is set to 0xB105F00D.\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`ssp_pcell_id0::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`ssp_pcell_id0::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@ssp_pcell_id0`]
module"]
pub type SSP_PCELL_ID0 = crate::Reg<ssp_pcell_id0::SSP_PCELL_ID0_SPEC>;
#[doc = "The SSPPCellID0-3 registers are four 8-bit wide registers, that span address locations 0xFF0-0xFFC. The registers can conceptually be treated as a 32-bit register. The register is used as a standard cross-peripheral identification system. The SSPPCellID register is set to 0xB105F00D."]
pub mod ssp_pcell_id0;
#[doc = "ssp_pcell_id1 (rw) register accessor: The SSPPCellID0-3 registers are four 8-bit wide registers, that span address locations 0xFF0-0xFFC. The registers can conceptually be treated as a 32-bit register. The register is used as a standard cross-peripheral identification system. The SSPPCellID register is set to 0xB105F00D.\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`ssp_pcell_id1::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`ssp_pcell_id1::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@ssp_pcell_id1`]
module"]
pub type SSP_PCELL_ID1 = crate::Reg<ssp_pcell_id1::SSP_PCELL_ID1_SPEC>;
#[doc = "The SSPPCellID0-3 registers are four 8-bit wide registers, that span address locations 0xFF0-0xFFC. The registers can conceptually be treated as a 32-bit register. The register is used as a standard cross-peripheral identification system. The SSPPCellID register is set to 0xB105F00D."]
pub mod ssp_pcell_id1;
#[doc = "ssp_pcell_id2 (rw) register accessor: The SSPPCellID0-3 registers are four 8-bit wide registers, that span address locations 0xFF0-0xFFC. The registers can conceptually be treated as a 32-bit register. The register is used as a standard cross-peripheral identification system. The SSPPCellID register is set to 0xB105F00D.\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`ssp_pcell_id2::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`ssp_pcell_id2::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@ssp_pcell_id2`]
module"]
pub type SSP_PCELL_ID2 = crate::Reg<ssp_pcell_id2::SSP_PCELL_ID2_SPEC>;
#[doc = "The SSPPCellID0-3 registers are four 8-bit wide registers, that span address locations 0xFF0-0xFFC. The registers can conceptually be treated as a 32-bit register. The register is used as a standard cross-peripheral identification system. The SSPPCellID register is set to 0xB105F00D."]
pub mod ssp_pcell_id2;
#[doc = "ssp_pcell_id3 (rw) register accessor: The SSPPCellID0-3 registers are four 8-bit wide registers, that span address locations 0xFF0-0xFFC. The registers can conceptually be treated as a 32-bit register. The register is used as a standard cross-peripheral identification system. The SSPPCellID register is set to 0xB105F00D.\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`ssp_pcell_id3::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`ssp_pcell_id3::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@ssp_pcell_id3`]
module"]
pub type SSP_PCELL_ID3 = crate::Reg<ssp_pcell_id3::SSP_PCELL_ID3_SPEC>;
#[doc = "The SSPPCellID0-3 registers are four 8-bit wide registers, that span address locations 0xFF0-0xFFC. The registers can conceptually be treated as a 32-bit register. The register is used as a standard cross-peripheral identification system. The SSPPCellID register is set to 0xB105F00D."]
pub mod ssp_pcell_id3;
