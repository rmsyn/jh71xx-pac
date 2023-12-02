#[doc = r"Register block"]
#[repr(C)]
pub struct RegisterBlock {
    msip_0: MSIP_0,
    msip_1: MSIP_1,
    msip_2: MSIP_2,
    msip_3: MSIP_3,
    msip_4: MSIP_4,
    _reserved5: [u8; 0x3fec],
    mtimecmp_0: MTIMECMP_0,
    mtimecmp_1: MTIMECMP_1,
    mtimecmp_2: MTIMECMP_2,
    mtimecmp_3: MTIMECMP_3,
    mtimecmp_4: MTIMECMP_4,
    _reserved10: [u8; 0x7fd0],
    mtime: MTIME,
}
impl RegisterBlock {
    #[doc = "0x00 - MSIP Register for hart 0"]
    #[inline(always)]
    pub const fn msip_0(&self) -> &MSIP_0 {
        &self.msip_0
    }
    #[doc = "0x04 - MSIP Register for hart 1"]
    #[inline(always)]
    pub const fn msip_1(&self) -> &MSIP_1 {
        &self.msip_1
    }
    #[doc = "0x08 - MSIP Register for hart 2"]
    #[inline(always)]
    pub const fn msip_2(&self) -> &MSIP_2 {
        &self.msip_2
    }
    #[doc = "0x0c - MSIP Register for hart 3"]
    #[inline(always)]
    pub const fn msip_3(&self) -> &MSIP_3 {
        &self.msip_3
    }
    #[doc = "0x10 - MSIP Register for hart 4"]
    #[inline(always)]
    pub const fn msip_4(&self) -> &MSIP_4 {
        &self.msip_4
    }
    #[doc = "0x4000..0x4008 - MTIMECMP Register for hart 0"]
    #[inline(always)]
    pub const fn mtimecmp_0(&self) -> &MTIMECMP_0 {
        &self.mtimecmp_0
    }
    #[doc = "0x4008..0x4010 - MTIMECMP Register for hart 1"]
    #[inline(always)]
    pub const fn mtimecmp_1(&self) -> &MTIMECMP_1 {
        &self.mtimecmp_1
    }
    #[doc = "0x4010..0x4018 - MTIMECMP Register for hart 2"]
    #[inline(always)]
    pub const fn mtimecmp_2(&self) -> &MTIMECMP_2 {
        &self.mtimecmp_2
    }
    #[doc = "0x4018..0x4020 - MTIMECMP Register for hart 3"]
    #[inline(always)]
    pub const fn mtimecmp_3(&self) -> &MTIMECMP_3 {
        &self.mtimecmp_3
    }
    #[doc = "0x4020..0x4028 - MTIMECMP Register for hart 4"]
    #[inline(always)]
    pub const fn mtimecmp_4(&self) -> &MTIMECMP_4 {
        &self.mtimecmp_4
    }
    #[doc = "0xbff8..0xc000 - MTIME Register"]
    #[inline(always)]
    pub const fn mtime(&self) -> &MTIME {
        &self.mtime
    }
}
#[doc = "msip_0 (rw) register accessor: MSIP Register for hart 0\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`msip_0::R`].  You can [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`msip_0::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@msip_0`]
module"]
pub type MSIP_0 = crate::Reg<msip_0::MSIP_0_SPEC>;
#[doc = "MSIP Register for hart 0"]
pub mod msip_0;
#[doc = "msip_1 (rw) register accessor: MSIP Register for hart 1\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`msip_1::R`].  You can [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`msip_1::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@msip_1`]
module"]
pub type MSIP_1 = crate::Reg<msip_1::MSIP_1_SPEC>;
#[doc = "MSIP Register for hart 1"]
pub mod msip_1;
#[doc = "msip_2 (rw) register accessor: MSIP Register for hart 2\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`msip_2::R`].  You can [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`msip_2::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@msip_2`]
module"]
pub type MSIP_2 = crate::Reg<msip_2::MSIP_2_SPEC>;
#[doc = "MSIP Register for hart 2"]
pub mod msip_2;
#[doc = "msip_3 (rw) register accessor: MSIP Register for hart 3\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`msip_3::R`].  You can [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`msip_3::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@msip_3`]
module"]
pub type MSIP_3 = crate::Reg<msip_3::MSIP_3_SPEC>;
#[doc = "MSIP Register for hart 3"]
pub mod msip_3;
#[doc = "msip_4 (rw) register accessor: MSIP Register for hart 4\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`msip_4::R`].  You can [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`msip_4::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@msip_4`]
module"]
pub type MSIP_4 = crate::Reg<msip_4::MSIP_4_SPEC>;
#[doc = "MSIP Register for hart 4"]
pub mod msip_4;
#[doc = "mtimecmp_0 (rw) register accessor: MTIMECMP Register for hart 0\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`mtimecmp_0::R`].  You can [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`mtimecmp_0::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@mtimecmp_0`]
module"]
pub type MTIMECMP_0 = crate::Reg<mtimecmp_0::MTIMECMP_0_SPEC>;
#[doc = "MTIMECMP Register for hart 0"]
pub mod mtimecmp_0;
#[doc = "mtimecmp_1 (rw) register accessor: MTIMECMP Register for hart 1\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`mtimecmp_1::R`].  You can [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`mtimecmp_1::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@mtimecmp_1`]
module"]
pub type MTIMECMP_1 = crate::Reg<mtimecmp_1::MTIMECMP_1_SPEC>;
#[doc = "MTIMECMP Register for hart 1"]
pub mod mtimecmp_1;
#[doc = "mtimecmp_2 (rw) register accessor: MTIMECMP Register for hart 2\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`mtimecmp_2::R`].  You can [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`mtimecmp_2::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@mtimecmp_2`]
module"]
pub type MTIMECMP_2 = crate::Reg<mtimecmp_2::MTIMECMP_2_SPEC>;
#[doc = "MTIMECMP Register for hart 2"]
pub mod mtimecmp_2;
#[doc = "mtimecmp_3 (rw) register accessor: MTIMECMP Register for hart 3\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`mtimecmp_3::R`].  You can [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`mtimecmp_3::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@mtimecmp_3`]
module"]
pub type MTIMECMP_3 = crate::Reg<mtimecmp_3::MTIMECMP_3_SPEC>;
#[doc = "MTIMECMP Register for hart 3"]
pub mod mtimecmp_3;
#[doc = "mtimecmp_4 (rw) register accessor: MTIMECMP Register for hart 4\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`mtimecmp_4::R`].  You can [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`mtimecmp_4::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@mtimecmp_4`]
module"]
pub type MTIMECMP_4 = crate::Reg<mtimecmp_4::MTIMECMP_4_SPEC>;
#[doc = "MTIMECMP Register for hart 4"]
pub mod mtimecmp_4;
#[doc = "mtime (rw) register accessor: MTIME Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`mtime::R`].  You can [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`mtime::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@mtime`]
module"]
pub type MTIME = crate::Reg<mtime::MTIME_SPEC>;
#[doc = "MTIME Register"]
pub mod mtime;
