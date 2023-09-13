#[doc = r"Register block"]
#[repr(C)]
pub struct RegisterBlock {
    #[doc = "0x00 - MSIP Register for hart 0"]
    pub msip_0: MSIP_0,
    #[doc = "0x04 - MSIP Register for hart 1"]
    pub msip_1: MSIP_1,
    #[doc = "0x08 - MSIP Register for hart 2"]
    pub msip_2: MSIP_2,
    #[doc = "0x0c - MSIP Register for hart 3"]
    pub msip_3: MSIP_3,
    #[doc = "0x10 - MSIP Register for hart 4"]
    pub msip_4: MSIP_4,
    _reserved5: [u8; 0x3fec],
    #[doc = "0x4000..0x4008 - MTIMECMP Register for hart 0"]
    pub mtimecmp_0: MTIMECMP_0,
    #[doc = "0x4008..0x4010 - MTIMECMP Register for hart 1"]
    pub mtimecmp_1: MTIMECMP_1,
    #[doc = "0x4010..0x4018 - MTIMECMP Register for hart 2"]
    pub mtimecmp_2: MTIMECMP_2,
    #[doc = "0x4018..0x4020 - MTIMECMP Register for hart 3"]
    pub mtimecmp_3: MTIMECMP_3,
    #[doc = "0x4020..0x4028 - MTIMECMP Register for hart 4"]
    pub mtimecmp_4: MTIMECMP_4,
    _reserved10: [u8; 0x7fd0],
    #[doc = "0xbff8..0xc000 - MTIME Register"]
    pub mtime: MTIME,
}
#[doc = "msip_0 (rw) register accessor: MSIP Register for hart 0\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`msip_0::R`].  You can [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`msip_0::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`msip_0`]
module"]
pub type MSIP_0 = crate::Reg<msip_0::MSIP_0_SPEC>;
#[doc = "MSIP Register for hart 0"]
pub mod msip_0;
#[doc = "msip_1 (rw) register accessor: MSIP Register for hart 1\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`msip_1::R`].  You can [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`msip_1::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`msip_1`]
module"]
pub type MSIP_1 = crate::Reg<msip_1::MSIP_1_SPEC>;
#[doc = "MSIP Register for hart 1"]
pub mod msip_1;
#[doc = "msip_2 (rw) register accessor: MSIP Register for hart 2\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`msip_2::R`].  You can [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`msip_2::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`msip_2`]
module"]
pub type MSIP_2 = crate::Reg<msip_2::MSIP_2_SPEC>;
#[doc = "MSIP Register for hart 2"]
pub mod msip_2;
#[doc = "msip_3 (rw) register accessor: MSIP Register for hart 3\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`msip_3::R`].  You can [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`msip_3::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`msip_3`]
module"]
pub type MSIP_3 = crate::Reg<msip_3::MSIP_3_SPEC>;
#[doc = "MSIP Register for hart 3"]
pub mod msip_3;
#[doc = "msip_4 (rw) register accessor: MSIP Register for hart 4\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`msip_4::R`].  You can [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`msip_4::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`msip_4`]
module"]
pub type MSIP_4 = crate::Reg<msip_4::MSIP_4_SPEC>;
#[doc = "MSIP Register for hart 4"]
pub mod msip_4;
#[doc = "mtimecmp_0 (rw) register accessor: MTIMECMP Register for hart 0\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`mtimecmp_0::R`].  You can [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`mtimecmp_0::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mtimecmp_0`]
module"]
pub type MTIMECMP_0 = crate::Reg<mtimecmp_0::MTIMECMP_0_SPEC>;
#[doc = "MTIMECMP Register for hart 0"]
pub mod mtimecmp_0;
#[doc = "mtimecmp_1 (rw) register accessor: MTIMECMP Register for hart 1\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`mtimecmp_1::R`].  You can [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`mtimecmp_1::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mtimecmp_1`]
module"]
pub type MTIMECMP_1 = crate::Reg<mtimecmp_1::MTIMECMP_1_SPEC>;
#[doc = "MTIMECMP Register for hart 1"]
pub mod mtimecmp_1;
#[doc = "mtimecmp_2 (rw) register accessor: MTIMECMP Register for hart 2\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`mtimecmp_2::R`].  You can [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`mtimecmp_2::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mtimecmp_2`]
module"]
pub type MTIMECMP_2 = crate::Reg<mtimecmp_2::MTIMECMP_2_SPEC>;
#[doc = "MTIMECMP Register for hart 2"]
pub mod mtimecmp_2;
#[doc = "mtimecmp_3 (rw) register accessor: MTIMECMP Register for hart 3\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`mtimecmp_3::R`].  You can [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`mtimecmp_3::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mtimecmp_3`]
module"]
pub type MTIMECMP_3 = crate::Reg<mtimecmp_3::MTIMECMP_3_SPEC>;
#[doc = "MTIMECMP Register for hart 3"]
pub mod mtimecmp_3;
#[doc = "mtimecmp_4 (rw) register accessor: MTIMECMP Register for hart 4\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`mtimecmp_4::R`].  You can [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`mtimecmp_4::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mtimecmp_4`]
module"]
pub type MTIMECMP_4 = crate::Reg<mtimecmp_4::MTIMECMP_4_SPEC>;
#[doc = "MTIMECMP Register for hart 4"]
pub mod mtimecmp_4;
#[doc = "mtime (rw) register accessor: MTIME Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`mtime::R`].  You can [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`mtime::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mtime`]
module"]
pub type MTIME = crate::Reg<mtime::MTIME_SPEC>;
#[doc = "MTIME Register"]
pub mod mtime;
