#[doc = r"Register block"]
#[repr(C)]
pub struct RegisterBlock {
    #[doc = "0x00 - TRNG CTRL Register"]
    pub ctrl: CTRL,
    #[doc = "0x04 - TRNG STAT Register"]
    pub stat: STAT,
    #[doc = "0x08 - TRNG MODE Register"]
    pub mode: MODE,
    #[doc = "0x0c - TRNG SMODE Register"]
    pub smode: SMODE,
    #[doc = "0x10 - TRNG Interrupt Enable Register"]
    pub ie: IE,
    #[doc = "0x14 - TRNG Interrupt Status Register"]
    pub istat: ISTAT,
    _reserved6: [u8; 0x08],
    #[doc = "0x20 - TRNG RAND 0 Status Register"]
    pub rand0: RAND0,
    #[doc = "0x24 - TRNG RAND 1 Status Register"]
    pub rand1: RAND1,
    #[doc = "0x28 - TRNG RAND 2 Status Register"]
    pub rand2: RAND2,
    #[doc = "0x2c - TRNG RAND 3 Status Register"]
    pub rand3: RAND3,
    #[doc = "0x30 - TRNG RAND 4 Status Register"]
    pub rand4: RAND4,
    #[doc = "0x34 - TRNG RAND 5 Status Register"]
    pub rand5: RAND5,
    #[doc = "0x38 - TRNG RAND 6 Status Register"]
    pub rand6: RAND6,
    #[doc = "0x3c - TRNG RAND 7 Status Register"]
    pub rand7: RAND7,
    _reserved14: [u8; 0x20],
    #[doc = "0x60 - Auto-reseeding after random number requests by host reaches specified counter: 0 - disable counter, other - reload value for internal counter"]
    pub auto_rqsts: AUTO_RQSTS,
    #[doc = "0x64 - Auto-reseeding after specified timer countdowns to 0: 0 - disable timer, other - reload value for internal timer"]
    pub auto_age: AUTO_AGE,
}
#[doc = "ctrl (rw) register accessor: TRNG CTRL Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`ctrl::R`].  You can [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`ctrl::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`ctrl`]
module"]
pub type CTRL = crate::Reg<ctrl::CTRL_SPEC>;
#[doc = "TRNG CTRL Register"]
pub mod ctrl;
#[doc = "stat (rw) register accessor: TRNG STAT Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`stat::R`].  You can [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`stat::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`stat`]
module"]
pub type STAT = crate::Reg<stat::STAT_SPEC>;
#[doc = "TRNG STAT Register"]
pub mod stat;
#[doc = "mode (rw) register accessor: TRNG MODE Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`mode::R`].  You can [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`mode::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mode`]
module"]
pub type MODE = crate::Reg<mode::MODE_SPEC>;
#[doc = "TRNG MODE Register"]
pub mod mode;
#[doc = "smode (rw) register accessor: TRNG SMODE Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`smode::R`].  You can [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`smode::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`smode`]
module"]
pub type SMODE = crate::Reg<smode::SMODE_SPEC>;
#[doc = "TRNG SMODE Register"]
pub mod smode;
#[doc = "ie (rw) register accessor: TRNG Interrupt Enable Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`ie::R`].  You can [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`ie::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`ie`]
module"]
pub type IE = crate::Reg<ie::IE_SPEC>;
#[doc = "TRNG Interrupt Enable Register"]
pub mod ie;
#[doc = "istat (rw) register accessor: TRNG Interrupt Status Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`istat::R`].  You can [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`istat::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`istat`]
module"]
pub type ISTAT = crate::Reg<istat::ISTAT_SPEC>;
#[doc = "TRNG Interrupt Status Register"]
pub mod istat;
#[doc = "rand0 (rw) register accessor: TRNG RAND 0 Status Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`rand0::R`].  You can [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`rand0::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`rand0`]
module"]
pub type RAND0 = crate::Reg<rand0::RAND0_SPEC>;
#[doc = "TRNG RAND 0 Status Register"]
pub mod rand0;
#[doc = "rand1 (rw) register accessor: TRNG RAND 1 Status Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`rand1::R`].  You can [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`rand1::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`rand1`]
module"]
pub type RAND1 = crate::Reg<rand1::RAND1_SPEC>;
#[doc = "TRNG RAND 1 Status Register"]
pub mod rand1;
#[doc = "rand2 (rw) register accessor: TRNG RAND 2 Status Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`rand2::R`].  You can [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`rand2::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`rand2`]
module"]
pub type RAND2 = crate::Reg<rand2::RAND2_SPEC>;
#[doc = "TRNG RAND 2 Status Register"]
pub mod rand2;
#[doc = "rand3 (rw) register accessor: TRNG RAND 3 Status Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`rand3::R`].  You can [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`rand3::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`rand3`]
module"]
pub type RAND3 = crate::Reg<rand3::RAND3_SPEC>;
#[doc = "TRNG RAND 3 Status Register"]
pub mod rand3;
#[doc = "rand4 (rw) register accessor: TRNG RAND 4 Status Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`rand4::R`].  You can [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`rand4::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`rand4`]
module"]
pub type RAND4 = crate::Reg<rand4::RAND4_SPEC>;
#[doc = "TRNG RAND 4 Status Register"]
pub mod rand4;
#[doc = "rand5 (rw) register accessor: TRNG RAND 5 Status Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`rand5::R`].  You can [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`rand5::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`rand5`]
module"]
pub type RAND5 = crate::Reg<rand5::RAND5_SPEC>;
#[doc = "TRNG RAND 5 Status Register"]
pub mod rand5;
#[doc = "rand6 (rw) register accessor: TRNG RAND 6 Status Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`rand6::R`].  You can [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`rand6::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`rand6`]
module"]
pub type RAND6 = crate::Reg<rand6::RAND6_SPEC>;
#[doc = "TRNG RAND 6 Status Register"]
pub mod rand6;
#[doc = "rand7 (rw) register accessor: TRNG RAND 7 Status Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`rand7::R`].  You can [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`rand7::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`rand7`]
module"]
pub type RAND7 = crate::Reg<rand7::RAND7_SPEC>;
#[doc = "TRNG RAND 7 Status Register"]
pub mod rand7;
#[doc = "auto_rqsts (rw) register accessor: Auto-reseeding after random number requests by host reaches specified counter: 0 - disable counter, other - reload value for internal counter\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`auto_rqsts::R`].  You can [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`auto_rqsts::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`auto_rqsts`]
module"]
pub type AUTO_RQSTS = crate::Reg<auto_rqsts::AUTO_RQSTS_SPEC>;
#[doc = "Auto-reseeding after random number requests by host reaches specified counter: 0 - disable counter, other - reload value for internal counter"]
pub mod auto_rqsts;
#[doc = "auto_age (rw) register accessor: Auto-reseeding after specified timer countdowns to 0: 0 - disable timer, other - reload value for internal timer\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`auto_age::R`].  You can [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`auto_age::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`auto_age`]
module"]
pub type AUTO_AGE = crate::Reg<auto_age::AUTO_AGE_SPEC>;
#[doc = "Auto-reseeding after specified timer countdowns to 0: 0 - disable timer, other - reload value for internal timer"]
pub mod auto_age;
