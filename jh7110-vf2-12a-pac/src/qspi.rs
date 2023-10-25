#[doc = r"Register block"]
#[repr(C)]
pub struct RegisterBlock {
    #[doc = "0x00 - Cadence QSPI Configuration"]
    pub config: CONFIG,
    #[doc = "0x04 - Cadence QSPI Read Instruction"]
    pub rd_instr: RD_INSTR,
    #[doc = "0x08 - Cadence QSPI Write Instruction"]
    pub wr_instr: WR_INSTR,
    #[doc = "0x0c - Cadence QSPI Delay"]
    pub delay: DELAY,
    #[doc = "0x10 - Cadence QSPI Read Capture"]
    pub read_capture: READ_CAPTURE,
    #[doc = "0x14 - Cadence QSPI Register Size"]
    pub reg_size: REG_SIZE,
    #[doc = "0x18 - Cadence QSPI SRAM Partition Size"]
    pub sram_partition: SRAM_PARTITION,
    #[doc = "0x1c - Cadence QSPI Indirect Trigger Address"]
    pub indirect_trigger: INDIRECT_TRIGGER,
    #[doc = "0x20 - Cadence QSPI Direct Memory Access"]
    pub dma: DMA,
    #[doc = "0x24 - Cadence QSPI Remap Address"]
    pub remap: REMAP,
    #[doc = "0x28 - Cadence QSPI Mode Bit(s)"]
    pub mode_bit: MODE_BIT,
    #[doc = "0x2c - Cadence QSPI SDRAM Level"]
    pub sdram_level: SDRAM_LEVEL,
    _reserved12: [u8; 0x08],
    #[doc = "0x38 - Cadence QSPI Write Completion Control"]
    pub wr_completion_ctrl: WR_COMPLETION_CTRL,
    _reserved13: [u8; 0x04],
    #[doc = "0x40 - Cadence QSPI IRQ Status"]
    pub irq_status: IRQ_STATUS,
    #[doc = "0x44 - Cadence QSPI IRQ Mask"]
    pub irq_mask: IRQ_MASK,
    _reserved15: [u8; 0x18],
    #[doc = "0x60 - Cadence QSPI Indirect Read"]
    pub indirect_rd: INDIRECT_RD,
    #[doc = "0x64 - Cadence QSPI Indirect Read Watermark"]
    pub indirect_rd_watermark: INDIRECT_RD_WATERMARK,
    #[doc = "0x68 - Cadence QSPI Indirect Read Start Address"]
    pub indirect_rd_start_addr: INDIRECT_RD_START_ADDR,
    #[doc = "0x6c - Cadence QSPI Indirect Read Bytes"]
    pub indirect_rd_bytes: INDIRECT_RD_BYTES,
    #[doc = "0x70 - Cadence QSPI Indirect Write"]
    pub indirect_wr: INDIRECT_WR,
    #[doc = "0x74 - Cadence QSPI Indirect Write Watermark"]
    pub indirect_wr_watermark: INDIRECT_WR_WATERMARK,
    #[doc = "0x78 - Cadence QSPI Indirect Write Start Address"]
    pub indirect_wr_start_addr: INDIRECT_WR_START_ADDR,
    #[doc = "0x7c - Cadence QSPI Indirect Write Bytes"]
    pub indirect_wr_bytes: INDIRECT_WR_BYTES,
    _reserved23: [u8; 0x10],
    #[doc = "0x90 - Cadence QSPI Command Control"]
    pub cmd_ctrl: CMD_CTRL,
    #[doc = "0x94 - Cadence QSPI Command Address"]
    pub cmd_address: CMD_ADDRESS,
    _reserved25: [u8; 0x08],
    #[doc = "0xa0 - Cadence QSPI Command Read at Lower"]
    pub cmd_read_at_lower: CMD_READ_AT_LOWER,
    #[doc = "0xa4 - Cadence QSPI Command Read at Upper"]
    pub cmd_read_at_upper: CMD_READ_AT_UPPER,
    #[doc = "0xa8 - Cadence QSPI Command Write at Lower"]
    pub cmd_write_at_lower: CMD_WRITE_AT_LOWER,
    #[doc = "0xac - Cadence QSPI Command Write at Upper"]
    pub cmd_write_at_upper: CMD_WRITE_AT_UPPER,
    #[doc = "0xb0 - Cadence QSPI Polling Status"]
    pub polling_status: POLLING_STATUS,
    _reserved30: [u8; 0x2c],
    #[doc = "0xe0 - Cadence QSPI Extension Lower"]
    pub ext_lower: EXT_LOWER,
}
#[doc = "config (rw) register accessor: Cadence QSPI Configuration\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`config::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`config::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`config`]
module"]
pub type CONFIG = crate::Reg<config::CONFIG_SPEC>;
#[doc = "Cadence QSPI Configuration"]
pub mod config;
#[doc = "rd_instr (rw) register accessor: Cadence QSPI Read Instruction\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`rd_instr::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`rd_instr::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`rd_instr`]
module"]
pub type RD_INSTR = crate::Reg<rd_instr::RD_INSTR_SPEC>;
#[doc = "Cadence QSPI Read Instruction"]
pub mod rd_instr;
#[doc = "wr_instr (rw) register accessor: Cadence QSPI Write Instruction\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`wr_instr::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`wr_instr::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`wr_instr`]
module"]
pub type WR_INSTR = crate::Reg<wr_instr::WR_INSTR_SPEC>;
#[doc = "Cadence QSPI Write Instruction"]
pub mod wr_instr;
#[doc = "delay (rw) register accessor: Cadence QSPI Delay\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`delay::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`delay::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`delay`]
module"]
pub type DELAY = crate::Reg<delay::DELAY_SPEC>;
#[doc = "Cadence QSPI Delay"]
pub mod delay;
#[doc = "read_capture (rw) register accessor: Cadence QSPI Read Capture\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`read_capture::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`read_capture::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`read_capture`]
module"]
pub type READ_CAPTURE = crate::Reg<read_capture::READ_CAPTURE_SPEC>;
#[doc = "Cadence QSPI Read Capture"]
pub mod read_capture;
#[doc = "reg_size (rw) register accessor: Cadence QSPI Register Size\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`reg_size::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`reg_size::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`reg_size`]
module"]
pub type REG_SIZE = crate::Reg<reg_size::REG_SIZE_SPEC>;
#[doc = "Cadence QSPI Register Size"]
pub mod reg_size;
#[doc = "sram_partition (rw) register accessor: Cadence QSPI SRAM Partition Size\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`sram_partition::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`sram_partition::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`sram_partition`]
module"]
pub type SRAM_PARTITION = crate::Reg<sram_partition::SRAM_PARTITION_SPEC>;
#[doc = "Cadence QSPI SRAM Partition Size"]
pub mod sram_partition;
#[doc = "indirect_trigger (rw) register accessor: Cadence QSPI Indirect Trigger Address\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`indirect_trigger::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`indirect_trigger::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`indirect_trigger`]
module"]
pub type INDIRECT_TRIGGER = crate::Reg<indirect_trigger::INDIRECT_TRIGGER_SPEC>;
#[doc = "Cadence QSPI Indirect Trigger Address"]
pub mod indirect_trigger;
#[doc = "dma (rw) register accessor: Cadence QSPI Direct Memory Access\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`dma::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`dma::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`dma`]
module"]
pub type DMA = crate::Reg<dma::DMA_SPEC>;
#[doc = "Cadence QSPI Direct Memory Access"]
pub mod dma;
#[doc = "remap (rw) register accessor: Cadence QSPI Remap Address\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`remap::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`remap::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`remap`]
module"]
pub type REMAP = crate::Reg<remap::REMAP_SPEC>;
#[doc = "Cadence QSPI Remap Address"]
pub mod remap;
#[doc = "mode_bit (rw) register accessor: Cadence QSPI Mode Bit(s)\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`mode_bit::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`mode_bit::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mode_bit`]
module"]
pub type MODE_BIT = crate::Reg<mode_bit::MODE_BIT_SPEC>;
#[doc = "Cadence QSPI Mode Bit(s)"]
pub mod mode_bit;
#[doc = "sdram_level (rw) register accessor: Cadence QSPI SDRAM Level\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`sdram_level::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`sdram_level::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`sdram_level`]
module"]
pub type SDRAM_LEVEL = crate::Reg<sdram_level::SDRAM_LEVEL_SPEC>;
#[doc = "Cadence QSPI SDRAM Level"]
pub mod sdram_level;
#[doc = "wr_completion_ctrl (rw) register accessor: Cadence QSPI Write Completion Control\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`wr_completion_ctrl::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`wr_completion_ctrl::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`wr_completion_ctrl`]
module"]
pub type WR_COMPLETION_CTRL = crate::Reg<wr_completion_ctrl::WR_COMPLETION_CTRL_SPEC>;
#[doc = "Cadence QSPI Write Completion Control"]
pub mod wr_completion_ctrl;
#[doc = "irq_status (rw) register accessor: Cadence QSPI IRQ Status\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`irq_status::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`irq_status::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`irq_status`]
module"]
pub type IRQ_STATUS = crate::Reg<irq_status::IRQ_STATUS_SPEC>;
#[doc = "Cadence QSPI IRQ Status"]
pub mod irq_status;
#[doc = "irq_mask (rw) register accessor: Cadence QSPI IRQ Mask\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`irq_mask::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`irq_mask::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`irq_mask`]
module"]
pub type IRQ_MASK = crate::Reg<irq_mask::IRQ_MASK_SPEC>;
#[doc = "Cadence QSPI IRQ Mask"]
pub mod irq_mask;
#[doc = "indirect_rd (rw) register accessor: Cadence QSPI Indirect Read\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`indirect_rd::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`indirect_rd::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`indirect_rd`]
module"]
pub type INDIRECT_RD = crate::Reg<indirect_rd::INDIRECT_RD_SPEC>;
#[doc = "Cadence QSPI Indirect Read"]
pub mod indirect_rd;
#[doc = "indirect_rd_watermark (rw) register accessor: Cadence QSPI Indirect Read Watermark\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`indirect_rd_watermark::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`indirect_rd_watermark::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`indirect_rd_watermark`]
module"]
pub type INDIRECT_RD_WATERMARK = crate::Reg<indirect_rd_watermark::INDIRECT_RD_WATERMARK_SPEC>;
#[doc = "Cadence QSPI Indirect Read Watermark"]
pub mod indirect_rd_watermark;
#[doc = "indirect_rd_start_addr (rw) register accessor: Cadence QSPI Indirect Read Start Address\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`indirect_rd_start_addr::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`indirect_rd_start_addr::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`indirect_rd_start_addr`]
module"]
pub type INDIRECT_RD_START_ADDR = crate::Reg<indirect_rd_start_addr::INDIRECT_RD_START_ADDR_SPEC>;
#[doc = "Cadence QSPI Indirect Read Start Address"]
pub mod indirect_rd_start_addr;
#[doc = "indirect_rd_bytes (rw) register accessor: Cadence QSPI Indirect Read Bytes\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`indirect_rd_bytes::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`indirect_rd_bytes::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`indirect_rd_bytes`]
module"]
pub type INDIRECT_RD_BYTES = crate::Reg<indirect_rd_bytes::INDIRECT_RD_BYTES_SPEC>;
#[doc = "Cadence QSPI Indirect Read Bytes"]
pub mod indirect_rd_bytes;
#[doc = "indirect_wr (rw) register accessor: Cadence QSPI Indirect Write\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`indirect_wr::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`indirect_wr::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`indirect_wr`]
module"]
pub type INDIRECT_WR = crate::Reg<indirect_wr::INDIRECT_WR_SPEC>;
#[doc = "Cadence QSPI Indirect Write"]
pub mod indirect_wr;
#[doc = "indirect_wr_watermark (rw) register accessor: Cadence QSPI Indirect Write Watermark\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`indirect_wr_watermark::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`indirect_wr_watermark::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`indirect_wr_watermark`]
module"]
pub type INDIRECT_WR_WATERMARK = crate::Reg<indirect_wr_watermark::INDIRECT_WR_WATERMARK_SPEC>;
#[doc = "Cadence QSPI Indirect Write Watermark"]
pub mod indirect_wr_watermark;
#[doc = "indirect_wr_start_addr (rw) register accessor: Cadence QSPI Indirect Write Start Address\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`indirect_wr_start_addr::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`indirect_wr_start_addr::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`indirect_wr_start_addr`]
module"]
pub type INDIRECT_WR_START_ADDR = crate::Reg<indirect_wr_start_addr::INDIRECT_WR_START_ADDR_SPEC>;
#[doc = "Cadence QSPI Indirect Write Start Address"]
pub mod indirect_wr_start_addr;
#[doc = "indirect_wr_bytes (rw) register accessor: Cadence QSPI Indirect Write Bytes\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`indirect_wr_bytes::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`indirect_wr_bytes::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`indirect_wr_bytes`]
module"]
pub type INDIRECT_WR_BYTES = crate::Reg<indirect_wr_bytes::INDIRECT_WR_BYTES_SPEC>;
#[doc = "Cadence QSPI Indirect Write Bytes"]
pub mod indirect_wr_bytes;
#[doc = "cmd_ctrl (rw) register accessor: Cadence QSPI Command Control\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`cmd_ctrl::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`cmd_ctrl::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`cmd_ctrl`]
module"]
pub type CMD_CTRL = crate::Reg<cmd_ctrl::CMD_CTRL_SPEC>;
#[doc = "Cadence QSPI Command Control"]
pub mod cmd_ctrl;
#[doc = "cmd_address (rw) register accessor: Cadence QSPI Command Address\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`cmd_address::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`cmd_address::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`cmd_address`]
module"]
pub type CMD_ADDRESS = crate::Reg<cmd_address::CMD_ADDRESS_SPEC>;
#[doc = "Cadence QSPI Command Address"]
pub mod cmd_address;
#[doc = "cmd_read_at_lower (rw) register accessor: Cadence QSPI Command Read at Lower\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`cmd_read_at_lower::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`cmd_read_at_lower::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`cmd_read_at_lower`]
module"]
pub type CMD_READ_AT_LOWER = crate::Reg<cmd_read_at_lower::CMD_READ_AT_LOWER_SPEC>;
#[doc = "Cadence QSPI Command Read at Lower"]
pub mod cmd_read_at_lower;
#[doc = "cmd_read_at_upper (rw) register accessor: Cadence QSPI Command Read at Upper\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`cmd_read_at_upper::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`cmd_read_at_upper::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`cmd_read_at_upper`]
module"]
pub type CMD_READ_AT_UPPER = crate::Reg<cmd_read_at_upper::CMD_READ_AT_UPPER_SPEC>;
#[doc = "Cadence QSPI Command Read at Upper"]
pub mod cmd_read_at_upper;
#[doc = "cmd_write_at_lower (rw) register accessor: Cadence QSPI Command Write at Lower\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`cmd_write_at_lower::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`cmd_write_at_lower::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`cmd_write_at_lower`]
module"]
pub type CMD_WRITE_AT_LOWER = crate::Reg<cmd_write_at_lower::CMD_WRITE_AT_LOWER_SPEC>;
#[doc = "Cadence QSPI Command Write at Lower"]
pub mod cmd_write_at_lower;
#[doc = "cmd_write_at_upper (rw) register accessor: Cadence QSPI Command Write at Upper\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`cmd_write_at_upper::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`cmd_write_at_upper::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`cmd_write_at_upper`]
module"]
pub type CMD_WRITE_AT_UPPER = crate::Reg<cmd_write_at_upper::CMD_WRITE_AT_UPPER_SPEC>;
#[doc = "Cadence QSPI Command Write at Upper"]
pub mod cmd_write_at_upper;
#[doc = "polling_status (rw) register accessor: Cadence QSPI Polling Status\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`polling_status::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`polling_status::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`polling_status`]
module"]
pub type POLLING_STATUS = crate::Reg<polling_status::POLLING_STATUS_SPEC>;
#[doc = "Cadence QSPI Polling Status"]
pub mod polling_status;
#[doc = "ext_lower (rw) register accessor: Cadence QSPI Extension Lower\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`ext_lower::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`ext_lower::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`ext_lower`]
module"]
pub type EXT_LOWER = crate::Reg<ext_lower::EXT_LOWER_SPEC>;
#[doc = "Cadence QSPI Extension Lower"]
pub mod ext_lower;
