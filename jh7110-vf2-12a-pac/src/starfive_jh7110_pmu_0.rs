#[doc = r"Register block"]
#[repr(C)]
pub struct RegisterBlock {
    _reserved0: [u8; 0x04],
    #[doc = "0x04 - Hardware Event Turn-On Mask"]
    pub hard_event_turn_on_mask: HARD_EVENT_TURN_ON_MASK,
    _reserved1: [u8; 0x04],
    #[doc = "0x0c - Software Turn-On Power Mode"]
    pub soft_turn_on_power_mode: SOFT_TURN_ON_POWER_MODE,
    #[doc = "0x10 - Software Turn-Off Power Mode"]
    pub soft_turn_off_power_mode: SOFT_TURN_OFF_POWER_MODE,
    #[doc = "0x14 - Threshold Sequence Timeout"]
    pub timeout_seq_thd: TIMEOUT_SEQ_THD,
    #[doc = "0x18 - Powerdomain Cascade 0"]
    pub pdc0: PDC0,
    #[doc = "0x1c - Powerdomain Cascade 1"]
    pub pdc1: PDC1,
    #[doc = "0x20 - Powerdomain Cascade 2"]
    pub pdc2: PDC2,
    _reserved7: [u8; 0x20],
    #[doc = "0x44 - Software Encouragement"]
    pub sw_encourage: SW_ENCOURAGE,
    #[doc = "0x48 - TIMER Interrupt Mask"]
    pub tim: TIM,
    #[doc = "0x4c - P-channel Bypass"]
    pub pch_bypass: PCH_BYPASS,
    #[doc = "0x50 - P-channel PSTATE"]
    pub pch_pstate: PCH_PSTATE,
    #[doc = "0x54 - P-channel Timeout Threshold"]
    pub pch_timeout: PCH_TIMEOUT,
    #[doc = "0x58 - LP Cell Control Timeout Threshold"]
    pub lp_timeout: LP_TIMEOUT,
    #[doc = "0x5c - Hardware Turn-On Power Mode"]
    pub hard_turn_on_power_mode: HARD_TURN_ON_POWER_MODE,
    _reserved14: [u8; 0x20],
    #[doc = "0x80 - Current Power Mode"]
    pub current_power_mode: CURRENT_POWER_MODE,
    #[doc = "0x84 - Current Sequence State"]
    pub current_seq_state: CURRENT_SEQ_STATE,
    #[doc = "0x88 - PMU Event Status"]
    pub event_status: EVENT_STATUS,
    #[doc = "0x8c - PMU Interrupt Status"]
    pub int_status: INT_STATUS,
    #[doc = "0x90 - Hardware Event Record"]
    pub hw_event_crd: HW_EVENT_CRD,
    #[doc = "0x94 - Hardware Event Type Record"]
    pub encourage_type_crd: ENCOURAGE_TYPE_CRD,
    #[doc = "0x98 - P-channel PACTIVE Status"]
    pub pch_active: PCH_ACTIVE,
}
#[doc = "hard_event_turn_on_mask (rw) register accessor: Hardware Event Turn-On Mask\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`hard_event_turn_on_mask::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`hard_event_turn_on_mask::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`hard_event_turn_on_mask`]
module"]
pub type HARD_EVENT_TURN_ON_MASK =
    crate::Reg<hard_event_turn_on_mask::HARD_EVENT_TURN_ON_MASK_SPEC>;
#[doc = "Hardware Event Turn-On Mask"]
pub mod hard_event_turn_on_mask;
#[doc = "soft_turn_on_power_mode (rw) register accessor: Software Turn-On Power Mode\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`soft_turn_on_power_mode::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`soft_turn_on_power_mode::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`soft_turn_on_power_mode`]
module"]
pub type SOFT_TURN_ON_POWER_MODE =
    crate::Reg<soft_turn_on_power_mode::SOFT_TURN_ON_POWER_MODE_SPEC>;
#[doc = "Software Turn-On Power Mode"]
pub mod soft_turn_on_power_mode;
#[doc = "soft_turn_off_power_mode (rw) register accessor: Software Turn-Off Power Mode\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`soft_turn_off_power_mode::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`soft_turn_off_power_mode::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`soft_turn_off_power_mode`]
module"]
pub type SOFT_TURN_OFF_POWER_MODE =
    crate::Reg<soft_turn_off_power_mode::SOFT_TURN_OFF_POWER_MODE_SPEC>;
#[doc = "Software Turn-Off Power Mode"]
pub mod soft_turn_off_power_mode;
#[doc = "timeout_seq_thd (rw) register accessor: Threshold Sequence Timeout\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`timeout_seq_thd::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`timeout_seq_thd::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`timeout_seq_thd`]
module"]
pub type TIMEOUT_SEQ_THD = crate::Reg<timeout_seq_thd::TIMEOUT_SEQ_THD_SPEC>;
#[doc = "Threshold Sequence Timeout"]
pub mod timeout_seq_thd;
#[doc = "pdc0 (rw) register accessor: Powerdomain Cascade 0\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`pdc0::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`pdc0::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`pdc0`]
module"]
pub type PDC0 = crate::Reg<pdc0::PDC0_SPEC>;
#[doc = "Powerdomain Cascade 0"]
pub mod pdc0;
#[doc = "pdc1 (rw) register accessor: Powerdomain Cascade 1\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`pdc1::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`pdc1::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`pdc1`]
module"]
pub type PDC1 = crate::Reg<pdc1::PDC1_SPEC>;
#[doc = "Powerdomain Cascade 1"]
pub mod pdc1;
#[doc = "pdc2 (rw) register accessor: Powerdomain Cascade 2\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`pdc2::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`pdc2::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`pdc2`]
module"]
pub type PDC2 = crate::Reg<pdc2::PDC2_SPEC>;
#[doc = "Powerdomain Cascade 2"]
pub mod pdc2;
#[doc = "sw_encourage (rw) register accessor: Software Encouragement\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`sw_encourage::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`sw_encourage::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`sw_encourage`]
module"]
pub type SW_ENCOURAGE = crate::Reg<sw_encourage::SW_ENCOURAGE_SPEC>;
#[doc = "Software Encouragement"]
pub mod sw_encourage;
#[doc = "tim (rw) register accessor: TIMER Interrupt Mask\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`tim::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`tim::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`tim`]
module"]
pub type TIM = crate::Reg<tim::TIM_SPEC>;
#[doc = "TIMER Interrupt Mask"]
pub mod tim;
#[doc = "pch_bypass (rw) register accessor: P-channel Bypass\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`pch_bypass::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`pch_bypass::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`pch_bypass`]
module"]
pub type PCH_BYPASS = crate::Reg<pch_bypass::PCH_BYPASS_SPEC>;
#[doc = "P-channel Bypass"]
pub mod pch_bypass;
#[doc = "pch_pstate (rw) register accessor: P-channel PSTATE\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`pch_pstate::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`pch_pstate::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`pch_pstate`]
module"]
pub type PCH_PSTATE = crate::Reg<pch_pstate::PCH_PSTATE_SPEC>;
#[doc = "P-channel PSTATE"]
pub mod pch_pstate;
#[doc = "pch_timeout (rw) register accessor: P-channel Timeout Threshold\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`pch_timeout::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`pch_timeout::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`pch_timeout`]
module"]
pub type PCH_TIMEOUT = crate::Reg<pch_timeout::PCH_TIMEOUT_SPEC>;
#[doc = "P-channel Timeout Threshold"]
pub mod pch_timeout;
#[doc = "lp_timeout (rw) register accessor: LP Cell Control Timeout Threshold\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`lp_timeout::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`lp_timeout::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`lp_timeout`]
module"]
pub type LP_TIMEOUT = crate::Reg<lp_timeout::LP_TIMEOUT_SPEC>;
#[doc = "LP Cell Control Timeout Threshold"]
pub mod lp_timeout;
#[doc = "hard_turn_on_power_mode (rw) register accessor: Hardware Turn-On Power Mode\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`hard_turn_on_power_mode::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`hard_turn_on_power_mode::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`hard_turn_on_power_mode`]
module"]
pub type HARD_TURN_ON_POWER_MODE =
    crate::Reg<hard_turn_on_power_mode::HARD_TURN_ON_POWER_MODE_SPEC>;
#[doc = "Hardware Turn-On Power Mode"]
pub mod hard_turn_on_power_mode;
#[doc = "current_power_mode (rw) register accessor: Current Power Mode\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`current_power_mode::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`current_power_mode::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`current_power_mode`]
module"]
pub type CURRENT_POWER_MODE = crate::Reg<current_power_mode::CURRENT_POWER_MODE_SPEC>;
#[doc = "Current Power Mode"]
pub mod current_power_mode;
#[doc = "current_seq_state (rw) register accessor: Current Sequence State\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`current_seq_state::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`current_seq_state::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`current_seq_state`]
module"]
pub type CURRENT_SEQ_STATE = crate::Reg<current_seq_state::CURRENT_SEQ_STATE_SPEC>;
#[doc = "Current Sequence State"]
pub mod current_seq_state;
#[doc = "event_status (rw) register accessor: PMU Event Status\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`event_status::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`event_status::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`event_status`]
module"]
pub type EVENT_STATUS = crate::Reg<event_status::EVENT_STATUS_SPEC>;
#[doc = "PMU Event Status"]
pub mod event_status;
#[doc = "int_status (rw) register accessor: PMU Interrupt Status\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`int_status::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`int_status::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`int_status`]
module"]
pub type INT_STATUS = crate::Reg<int_status::INT_STATUS_SPEC>;
#[doc = "PMU Interrupt Status"]
pub mod int_status;
#[doc = "hw_event_crd (rw) register accessor: Hardware Event Record\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`hw_event_crd::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`hw_event_crd::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`hw_event_crd`]
module"]
pub type HW_EVENT_CRD = crate::Reg<hw_event_crd::HW_EVENT_CRD_SPEC>;
#[doc = "Hardware Event Record"]
pub mod hw_event_crd;
#[doc = "encourage_type_crd (rw) register accessor: Hardware Event Type Record\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`encourage_type_crd::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`encourage_type_crd::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`encourage_type_crd`]
module"]
pub type ENCOURAGE_TYPE_CRD = crate::Reg<encourage_type_crd::ENCOURAGE_TYPE_CRD_SPEC>;
#[doc = "Hardware Event Type Record"]
pub mod encourage_type_crd;
#[doc = "pch_active (rw) register accessor: P-channel PACTIVE Status\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`pch_active::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`pch_active::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`pch_active`]
module"]
pub type PCH_ACTIVE = crate::Reg<pch_active::PCH_ACTIVE_SPEC>;
#[doc = "P-channel PACTIVE Status"]
pub mod pch_active;
