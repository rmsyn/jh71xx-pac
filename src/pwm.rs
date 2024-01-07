#[doc = r"Register block"]
#[repr(C)]
pub struct RegisterBlock {
    cntr: CNTR,
    hrc: HRC,
    lrc: LRC,
    ctrl: CTRL,
}
impl RegisterBlock {
    #[doc = "0x00 - PTC counter register"]
    #[inline(always)]
    pub const fn cntr(&self) -> &CNTR {
        &self.cntr
    }
    #[doc = "0x04 - PTC duty-cycle register"]
    #[inline(always)]
    pub const fn hrc(&self) -> &HRC {
        &self.hrc
    }
    #[doc = "0x08 - PTC period register"]
    #[inline(always)]
    pub const fn lrc(&self) -> &LRC {
        &self.lrc
    }
    #[doc = "0x0c - PTC control register"]
    #[inline(always)]
    pub const fn ctrl(&self) -> &CTRL {
        &self.ctrl
    }
}
#[doc = "cntr (rw) register accessor: PTC counter register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`cntr::R`].  You can [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`cntr::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@cntr`]
module"]
pub type CNTR = crate::Reg<cntr::CNTR_SPEC>;
#[doc = "PTC counter register"]
pub mod cntr;
#[doc = "hrc (rw) register accessor: PTC duty-cycle register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`hrc::R`].  You can [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`hrc::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@hrc`]
module"]
pub type HRC = crate::Reg<hrc::HRC_SPEC>;
#[doc = "PTC duty-cycle register"]
pub mod hrc;
#[doc = "lrc (rw) register accessor: PTC period register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`lrc::R`].  You can [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`lrc::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@lrc`]
module"]
pub type LRC = crate::Reg<lrc::LRC_SPEC>;
#[doc = "PTC period register"]
pub mod lrc;
#[doc = "ctrl (rw) register accessor: PTC control register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`ctrl::R`].  You can [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`ctrl::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@ctrl`]
module"]
pub type CTRL = crate::Reg<ctrl::CTRL_SPEC>;
#[doc = "PTC control register"]
pub mod ctrl;
