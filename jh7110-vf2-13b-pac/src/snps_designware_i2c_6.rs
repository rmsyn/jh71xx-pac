#[doc = r"Register block"]
#[repr(C)]
pub struct RegisterBlock {
    #[doc = "0x00 - DesignWare I2C CON"]
    pub con: CON,
    #[doc = "0x04 - DesignWare I2C TAR"]
    pub tar: TAR,
    #[doc = "0x08 - DesignWare I2C SAR"]
    pub sar: SAR,
    _reserved3: [u8; 0x04],
    #[doc = "0x10 - DesignWare I2C Data Command"]
    pub data_cmd: DATA_CMD,
    #[doc = "0x14 - DesignWare I2C SS SCL HCNT"]
    pub ss_scl_hcnt: SS_SCL_HCNT,
    #[doc = "0x18 - DesignWare I2C SS SCL LCNT"]
    pub ss_scl_lcnt: SS_SCL_LCNT,
    #[doc = "0x1c - DesignWare I2C FS SCL HCNT"]
    pub fs_scl_hcnt: FS_SCL_HCNT,
    #[doc = "0x20 - DesignWare I2C FS SCL LCNT"]
    pub fs_scl_lcnt: FS_SCL_LCNT,
    #[doc = "0x24 - DesignWare I2C HS SCL HCNT"]
    pub hs_scl_hcnt: HS_SCL_HCNT,
    #[doc = "0x28 - DesignWare I2C HS SCL LCNT"]
    pub hs_scl_lcnt: HS_SCL_LCNT,
    #[doc = "0x2c - DesignWare I2C Interrupt Status"]
    pub intr_stat: INTR_STAT,
    #[doc = "0x30 - DesignWare I2C Interrupt Mask"]
    pub intr_mask: INTR_MASK,
    #[doc = "0x34 - DesignWare I2C Raw Interrupt Status"]
    pub raw_intr_stat: RAW_INTR_STAT,
    #[doc = "0x38 - DesignWare I2C RX TL"]
    pub rx_tl: RX_TL,
    #[doc = "0x3c - DesignWare I2C TX TL"]
    pub tx_tl: TX_TL,
    #[doc = "0x40 - DesignWare I2C Clear Interrrupt"]
    pub clr_intr: CLR_INTR,
    #[doc = "0x44 - DesignWare I2C Clear RX Underrun"]
    pub clr_rx_under: CLR_RX_UNDER,
    #[doc = "0x48 - DesignWare I2C Clear RX Overrun"]
    pub clr_rx_over: CLR_RX_OVER,
    #[doc = "0x4c - DesignWare I2C Clear TX Overrun"]
    pub clr_tx_over: CLR_TX_OVER,
    #[doc = "0x50 - DesignWare I2C Clear Read Request"]
    pub clr_rd_req: CLR_RD_REQ,
    #[doc = "0x54 - DesignWare I2C Clear TX Abort"]
    pub clr_tx_abrt: CLR_TX_ABRT,
    #[doc = "0x58 - DesignWare I2C Clear RX Done"]
    pub clr_rx_done: CLR_RX_DONE,
    #[doc = "0x5c - DesignWare I2C Clear Activity"]
    pub clr_activity: CLR_ACTIVITY,
    #[doc = "0x60 - DesignWare I2C Clear Stop DET"]
    pub clr_stop_det: CLR_STOP_DET,
    #[doc = "0x64 - DesignWare I2C Clear Start DET"]
    pub clr_start_det: CLR_START_DET,
    #[doc = "0x68 - DesignWare I2C Clear General Call"]
    pub clr_gen_call: CLR_GEN_CALL,
    #[doc = "0x6c - DesignWare I2C Enable"]
    pub enable: ENABLE,
    #[doc = "0x70 - DesignWare I2C Status"]
    pub status: STATUS,
    #[doc = "0x74 - DesignWare I2C TX Failure"]
    pub txflr: TXFLR,
    #[doc = "0x78 - DesignWare I2C RX Failure"]
    pub rxflr: RXFLR,
    #[doc = "0x7c - DesignWare I2C SDA Hold"]
    pub sda_hold: SDA_HOLD,
    #[doc = "0x80 - DesignWare I2C TX Abort Source"]
    pub tx_abrt_source: TX_ABRT_SOURCE,
    _reserved32: [u8; 0x18],
    #[doc = "0x9c - DesignWare I2C Enable Status"]
    pub enable_status: ENABLE_STATUS,
    _reserved33: [u8; 0x08],
    #[doc = "0xa8 - DesignWare I2C Clear Restart DET"]
    pub clr_restart_det: CLR_RESTART_DET,
    _reserved34: [u8; 0x48],
    #[doc = "0xf4 - DesignWare I2C Compatibility Parameter 1"]
    pub comp_param_1: COMP_PARAM_1,
    #[doc = "0xf8 - DesignWare I2C Compatibility Version"]
    pub comp_version: COMP_VERSION,
    #[doc = "0xfc - DesignWare I2C Compatibility Type"]
    pub comp_type: COMP_TYPE,
}
#[doc = "con (rw) register accessor: DesignWare I2C CON\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`con::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`con::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`con`]
module"]
pub type CON = crate::Reg<con::CON_SPEC>;
#[doc = "DesignWare I2C CON"]
pub mod con;
#[doc = "tar (rw) register accessor: DesignWare I2C TAR\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`tar::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`tar::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`tar`]
module"]
pub type TAR = crate::Reg<tar::TAR_SPEC>;
#[doc = "DesignWare I2C TAR"]
pub mod tar;
#[doc = "sar (rw) register accessor: DesignWare I2C SAR\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`sar::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`sar::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`sar`]
module"]
pub type SAR = crate::Reg<sar::SAR_SPEC>;
#[doc = "DesignWare I2C SAR"]
pub mod sar;
#[doc = "data_cmd (rw) register accessor: DesignWare I2C Data Command\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`data_cmd::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`data_cmd::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`data_cmd`]
module"]
pub type DATA_CMD = crate::Reg<data_cmd::DATA_CMD_SPEC>;
#[doc = "DesignWare I2C Data Command"]
pub mod data_cmd;
#[doc = "ss_scl_hcnt (rw) register accessor: DesignWare I2C SS SCL HCNT\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`ss_scl_hcnt::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`ss_scl_hcnt::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`ss_scl_hcnt`]
module"]
pub type SS_SCL_HCNT = crate::Reg<ss_scl_hcnt::SS_SCL_HCNT_SPEC>;
#[doc = "DesignWare I2C SS SCL HCNT"]
pub mod ss_scl_hcnt;
#[doc = "ss_scl_lcnt (rw) register accessor: DesignWare I2C SS SCL LCNT\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`ss_scl_lcnt::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`ss_scl_lcnt::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`ss_scl_lcnt`]
module"]
pub type SS_SCL_LCNT = crate::Reg<ss_scl_lcnt::SS_SCL_LCNT_SPEC>;
#[doc = "DesignWare I2C SS SCL LCNT"]
pub mod ss_scl_lcnt;
#[doc = "fs_scl_hcnt (rw) register accessor: DesignWare I2C FS SCL HCNT\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`fs_scl_hcnt::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`fs_scl_hcnt::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`fs_scl_hcnt`]
module"]
pub type FS_SCL_HCNT = crate::Reg<fs_scl_hcnt::FS_SCL_HCNT_SPEC>;
#[doc = "DesignWare I2C FS SCL HCNT"]
pub mod fs_scl_hcnt;
#[doc = "fs_scl_lcnt (rw) register accessor: DesignWare I2C FS SCL LCNT\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`fs_scl_lcnt::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`fs_scl_lcnt::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`fs_scl_lcnt`]
module"]
pub type FS_SCL_LCNT = crate::Reg<fs_scl_lcnt::FS_SCL_LCNT_SPEC>;
#[doc = "DesignWare I2C FS SCL LCNT"]
pub mod fs_scl_lcnt;
#[doc = "hs_scl_hcnt (rw) register accessor: DesignWare I2C HS SCL HCNT\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`hs_scl_hcnt::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`hs_scl_hcnt::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`hs_scl_hcnt`]
module"]
pub type HS_SCL_HCNT = crate::Reg<hs_scl_hcnt::HS_SCL_HCNT_SPEC>;
#[doc = "DesignWare I2C HS SCL HCNT"]
pub mod hs_scl_hcnt;
#[doc = "hs_scl_lcnt (rw) register accessor: DesignWare I2C HS SCL LCNT\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`hs_scl_lcnt::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`hs_scl_lcnt::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`hs_scl_lcnt`]
module"]
pub type HS_SCL_LCNT = crate::Reg<hs_scl_lcnt::HS_SCL_LCNT_SPEC>;
#[doc = "DesignWare I2C HS SCL LCNT"]
pub mod hs_scl_lcnt;
#[doc = "intr_stat (rw) register accessor: DesignWare I2C Interrupt Status\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`intr_stat::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`intr_stat::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`intr_stat`]
module"]
pub type INTR_STAT = crate::Reg<intr_stat::INTR_STAT_SPEC>;
#[doc = "DesignWare I2C Interrupt Status"]
pub mod intr_stat;
#[doc = "intr_mask (rw) register accessor: DesignWare I2C Interrupt Mask\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`intr_mask::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`intr_mask::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`intr_mask`]
module"]
pub type INTR_MASK = crate::Reg<intr_mask::INTR_MASK_SPEC>;
#[doc = "DesignWare I2C Interrupt Mask"]
pub mod intr_mask;
#[doc = "raw_intr_stat (rw) register accessor: DesignWare I2C Raw Interrupt Status\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`raw_intr_stat::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`raw_intr_stat::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`raw_intr_stat`]
module"]
pub type RAW_INTR_STAT = crate::Reg<raw_intr_stat::RAW_INTR_STAT_SPEC>;
#[doc = "DesignWare I2C Raw Interrupt Status"]
pub mod raw_intr_stat;
#[doc = "rx_tl (rw) register accessor: DesignWare I2C RX TL\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`rx_tl::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`rx_tl::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`rx_tl`]
module"]
pub type RX_TL = crate::Reg<rx_tl::RX_TL_SPEC>;
#[doc = "DesignWare I2C RX TL"]
pub mod rx_tl;
#[doc = "tx_tl (rw) register accessor: DesignWare I2C TX TL\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`tx_tl::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`tx_tl::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`tx_tl`]
module"]
pub type TX_TL = crate::Reg<tx_tl::TX_TL_SPEC>;
#[doc = "DesignWare I2C TX TL"]
pub mod tx_tl;
#[doc = "clr_intr (rw) register accessor: DesignWare I2C Clear Interrrupt\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`clr_intr::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`clr_intr::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`clr_intr`]
module"]
pub type CLR_INTR = crate::Reg<clr_intr::CLR_INTR_SPEC>;
#[doc = "DesignWare I2C Clear Interrrupt"]
pub mod clr_intr;
#[doc = "clr_rx_under (rw) register accessor: DesignWare I2C Clear RX Underrun\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`clr_rx_under::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`clr_rx_under::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`clr_rx_under`]
module"]
pub type CLR_RX_UNDER = crate::Reg<clr_rx_under::CLR_RX_UNDER_SPEC>;
#[doc = "DesignWare I2C Clear RX Underrun"]
pub mod clr_rx_under;
#[doc = "clr_rx_over (rw) register accessor: DesignWare I2C Clear RX Overrun\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`clr_rx_over::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`clr_rx_over::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`clr_rx_over`]
module"]
pub type CLR_RX_OVER = crate::Reg<clr_rx_over::CLR_RX_OVER_SPEC>;
#[doc = "DesignWare I2C Clear RX Overrun"]
pub mod clr_rx_over;
#[doc = "clr_tx_over (rw) register accessor: DesignWare I2C Clear TX Overrun\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`clr_tx_over::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`clr_tx_over::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`clr_tx_over`]
module"]
pub type CLR_TX_OVER = crate::Reg<clr_tx_over::CLR_TX_OVER_SPEC>;
#[doc = "DesignWare I2C Clear TX Overrun"]
pub mod clr_tx_over;
#[doc = "clr_rd_req (rw) register accessor: DesignWare I2C Clear Read Request\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`clr_rd_req::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`clr_rd_req::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`clr_rd_req`]
module"]
pub type CLR_RD_REQ = crate::Reg<clr_rd_req::CLR_RD_REQ_SPEC>;
#[doc = "DesignWare I2C Clear Read Request"]
pub mod clr_rd_req;
#[doc = "clr_tx_abrt (rw) register accessor: DesignWare I2C Clear TX Abort\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`clr_tx_abrt::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`clr_tx_abrt::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`clr_tx_abrt`]
module"]
pub type CLR_TX_ABRT = crate::Reg<clr_tx_abrt::CLR_TX_ABRT_SPEC>;
#[doc = "DesignWare I2C Clear TX Abort"]
pub mod clr_tx_abrt;
#[doc = "clr_rx_done (rw) register accessor: DesignWare I2C Clear RX Done\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`clr_rx_done::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`clr_rx_done::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`clr_rx_done`]
module"]
pub type CLR_RX_DONE = crate::Reg<clr_rx_done::CLR_RX_DONE_SPEC>;
#[doc = "DesignWare I2C Clear RX Done"]
pub mod clr_rx_done;
#[doc = "clr_activity (rw) register accessor: DesignWare I2C Clear Activity\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`clr_activity::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`clr_activity::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`clr_activity`]
module"]
pub type CLR_ACTIVITY = crate::Reg<clr_activity::CLR_ACTIVITY_SPEC>;
#[doc = "DesignWare I2C Clear Activity"]
pub mod clr_activity;
#[doc = "clr_stop_det (rw) register accessor: DesignWare I2C Clear Stop DET\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`clr_stop_det::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`clr_stop_det::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`clr_stop_det`]
module"]
pub type CLR_STOP_DET = crate::Reg<clr_stop_det::CLR_STOP_DET_SPEC>;
#[doc = "DesignWare I2C Clear Stop DET"]
pub mod clr_stop_det;
#[doc = "clr_start_det (rw) register accessor: DesignWare I2C Clear Start DET\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`clr_start_det::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`clr_start_det::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`clr_start_det`]
module"]
pub type CLR_START_DET = crate::Reg<clr_start_det::CLR_START_DET_SPEC>;
#[doc = "DesignWare I2C Clear Start DET"]
pub mod clr_start_det;
#[doc = "clr_gen_call (rw) register accessor: DesignWare I2C Clear General Call\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`clr_gen_call::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`clr_gen_call::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`clr_gen_call`]
module"]
pub type CLR_GEN_CALL = crate::Reg<clr_gen_call::CLR_GEN_CALL_SPEC>;
#[doc = "DesignWare I2C Clear General Call"]
pub mod clr_gen_call;
#[doc = "enable (rw) register accessor: DesignWare I2C Enable\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`enable::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`enable::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`enable`]
module"]
pub type ENABLE = crate::Reg<enable::ENABLE_SPEC>;
#[doc = "DesignWare I2C Enable"]
pub mod enable;
#[doc = "status (rw) register accessor: DesignWare I2C Status\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`status::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`status::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`status`]
module"]
pub type STATUS = crate::Reg<status::STATUS_SPEC>;
#[doc = "DesignWare I2C Status"]
pub mod status;
#[doc = "txflr (rw) register accessor: DesignWare I2C TX Failure\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`txflr::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`txflr::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`txflr`]
module"]
pub type TXFLR = crate::Reg<txflr::TXFLR_SPEC>;
#[doc = "DesignWare I2C TX Failure"]
pub mod txflr;
#[doc = "rxflr (rw) register accessor: DesignWare I2C RX Failure\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`rxflr::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`rxflr::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`rxflr`]
module"]
pub type RXFLR = crate::Reg<rxflr::RXFLR_SPEC>;
#[doc = "DesignWare I2C RX Failure"]
pub mod rxflr;
#[doc = "sda_hold (rw) register accessor: DesignWare I2C SDA Hold\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`sda_hold::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`sda_hold::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`sda_hold`]
module"]
pub type SDA_HOLD = crate::Reg<sda_hold::SDA_HOLD_SPEC>;
#[doc = "DesignWare I2C SDA Hold"]
pub mod sda_hold;
#[doc = "tx_abrt_source (rw) register accessor: DesignWare I2C TX Abort Source\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`tx_abrt_source::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`tx_abrt_source::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`tx_abrt_source`]
module"]
pub type TX_ABRT_SOURCE = crate::Reg<tx_abrt_source::TX_ABRT_SOURCE_SPEC>;
#[doc = "DesignWare I2C TX Abort Source"]
pub mod tx_abrt_source;
#[doc = "enable_status (rw) register accessor: DesignWare I2C Enable Status\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`enable_status::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`enable_status::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`enable_status`]
module"]
pub type ENABLE_STATUS = crate::Reg<enable_status::ENABLE_STATUS_SPEC>;
#[doc = "DesignWare I2C Enable Status"]
pub mod enable_status;
#[doc = "clr_restart_det (rw) register accessor: DesignWare I2C Clear Restart DET\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`clr_restart_det::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`clr_restart_det::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`clr_restart_det`]
module"]
pub type CLR_RESTART_DET = crate::Reg<clr_restart_det::CLR_RESTART_DET_SPEC>;
#[doc = "DesignWare I2C Clear Restart DET"]
pub mod clr_restart_det;
#[doc = "comp_param_1 (rw) register accessor: DesignWare I2C Compatibility Parameter 1\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`comp_param_1::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`comp_param_1::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`comp_param_1`]
module"]
pub type COMP_PARAM_1 = crate::Reg<comp_param_1::COMP_PARAM_1_SPEC>;
#[doc = "DesignWare I2C Compatibility Parameter 1"]
pub mod comp_param_1;
#[doc = "comp_version (rw) register accessor: DesignWare I2C Compatibility Version\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`comp_version::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`comp_version::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`comp_version`]
module"]
pub type COMP_VERSION = crate::Reg<comp_version::COMP_VERSION_SPEC>;
#[doc = "DesignWare I2C Compatibility Version"]
pub mod comp_version;
#[doc = "comp_type (rw) register accessor: DesignWare I2C Compatibility Type\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`comp_type::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`comp_type::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`comp_type`]
module"]
pub type COMP_TYPE = crate::Reg<comp_type::COMP_TYPE_SPEC>;
#[doc = "DesignWare I2C Compatibility Type"]
pub mod comp_type;
