#[doc = r"Register block"]
#[repr(C)]
pub struct RegisterBlock {
    ctrl: CTRL,
    stat: STAT,
    mode: MODE,
    smode: SMODE,
    ie: IE,
    istat: ISTAT,
    _reserved6: [u8; 0x08],
    rand_0: RAND_0,
    rand_1: RAND_1,
    rand_2: RAND_2,
    rand_3: RAND_3,
    rand_4: RAND_4,
    rand_5: RAND_5,
    rand_6: RAND_6,
    rand_7: RAND_7,
    _reserved14: [u8; 0x20],
    auto_rqsts: AUTO_RQSTS,
    auto_age: AUTO_AGE,
}
impl RegisterBlock {
    #[doc = "0x00 - TRNG CTRL Register"]
    #[inline(always)]
    pub const fn ctrl(&self) -> &CTRL {
        &self.ctrl
    }
    #[doc = "0x04 - TRNG STAT Register"]
    #[inline(always)]
    pub const fn stat(&self) -> &STAT {
        &self.stat
    }
    #[doc = "0x08 - TRNG MODE Register"]
    #[inline(always)]
    pub const fn mode(&self) -> &MODE {
        &self.mode
    }
    #[doc = "0x0c - TRNG SMODE Register"]
    #[inline(always)]
    pub const fn smode(&self) -> &SMODE {
        &self.smode
    }
    #[doc = "0x10 - TRNG Interrupt Enable Register"]
    #[inline(always)]
    pub const fn ie(&self) -> &IE {
        &self.ie
    }
    #[doc = "0x14 - TRNG Interrupt Status Register"]
    #[inline(always)]
    pub const fn istat(&self) -> &ISTAT {
        &self.istat
    }
    #[doc = "0x20 - TRNG RAND 0 Status Register"]
    #[inline(always)]
    pub const fn rand_0(&self) -> &RAND_0 {
        &self.rand_0
    }
    #[doc = "0x24 - TRNG RAND 1 Status Register"]
    #[inline(always)]
    pub const fn rand_1(&self) -> &RAND_1 {
        &self.rand_1
    }
    #[doc = "0x28 - TRNG RAND 2 Status Register"]
    #[inline(always)]
    pub const fn rand_2(&self) -> &RAND_2 {
        &self.rand_2
    }
    #[doc = "0x2c - TRNG RAND 3 Status Register"]
    #[inline(always)]
    pub const fn rand_3(&self) -> &RAND_3 {
        &self.rand_3
    }
    #[doc = "0x30 - TRNG RAND 4 Status Register"]
    #[inline(always)]
    pub const fn rand_4(&self) -> &RAND_4 {
        &self.rand_4
    }
    #[doc = "0x34 - TRNG RAND 5 Status Register"]
    #[inline(always)]
    pub const fn rand_5(&self) -> &RAND_5 {
        &self.rand_5
    }
    #[doc = "0x38 - TRNG RAND 6 Status Register"]
    #[inline(always)]
    pub const fn rand_6(&self) -> &RAND_6 {
        &self.rand_6
    }
    #[doc = "0x3c - TRNG RAND 7 Status Register"]
    #[inline(always)]
    pub const fn rand_7(&self) -> &RAND_7 {
        &self.rand_7
    }
    #[doc = "0x60 - Auto-reseeding after random number requests by host reaches specified counter: 0 - disable counter, other - reload value for internal counter"]
    #[inline(always)]
    pub const fn auto_rqsts(&self) -> &AUTO_RQSTS {
        &self.auto_rqsts
    }
    #[doc = "0x64 - Auto-reseeding after specified timer countdowns to 0: 0 - disable timer, other - reload value for internal timer"]
    #[inline(always)]
    pub const fn auto_age(&self) -> &AUTO_AGE {
        &self.auto_age
    }
}
#[doc = "ctrl (rw) register accessor: TRNG CTRL Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`ctrl::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`ctrl::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@ctrl`]
module"]
pub type CTRL = crate::Reg<ctrl::CTRL_SPEC>;
#[doc = "TRNG CTRL Register"]
pub mod ctrl;
#[doc = "stat (rw) register accessor: TRNG STAT Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`stat::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`stat::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@stat`]
module"]
pub type STAT = crate::Reg<stat::STAT_SPEC>;
#[doc = "TRNG STAT Register"]
pub mod stat;
#[doc = "mode (rw) register accessor: TRNG MODE Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`mode::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`mode::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@mode`]
module"]
pub type MODE = crate::Reg<mode::MODE_SPEC>;
#[doc = "TRNG MODE Register"]
pub mod mode;
#[doc = "smode (rw) register accessor: TRNG SMODE Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`smode::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`smode::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@smode`]
module"]
pub type SMODE = crate::Reg<smode::SMODE_SPEC>;
#[doc = "TRNG SMODE Register"]
pub mod smode;
#[doc = "ie (rw) register accessor: TRNG Interrupt Enable Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`ie::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`ie::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@ie`]
module"]
pub type IE = crate::Reg<ie::IE_SPEC>;
#[doc = "TRNG Interrupt Enable Register"]
pub mod ie;
#[doc = "istat (rw) register accessor: TRNG Interrupt Status Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`istat::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`istat::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@istat`]
module"]
pub type ISTAT = crate::Reg<istat::ISTAT_SPEC>;
#[doc = "TRNG Interrupt Status Register"]
pub mod istat;
#[doc = "rand_0 (rw) register accessor: TRNG RAND 0 Status Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`rand_0::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`rand_0::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@rand_0`]
module"]
pub type RAND_0 = crate::Reg<rand_0::RAND_0_SPEC>;
#[doc = "TRNG RAND 0 Status Register"]
pub mod rand_0;
#[doc = "rand_1 (rw) register accessor: TRNG RAND 1 Status Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`rand_1::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`rand_1::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@rand_1`]
module"]
pub type RAND_1 = crate::Reg<rand_1::RAND_1_SPEC>;
#[doc = "TRNG RAND 1 Status Register"]
pub mod rand_1;
#[doc = "rand_2 (rw) register accessor: TRNG RAND 2 Status Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`rand_2::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`rand_2::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@rand_2`]
module"]
pub type RAND_2 = crate::Reg<rand_2::RAND_2_SPEC>;
#[doc = "TRNG RAND 2 Status Register"]
pub mod rand_2;
#[doc = "rand_3 (rw) register accessor: TRNG RAND 3 Status Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`rand_3::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`rand_3::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@rand_3`]
module"]
pub type RAND_3 = crate::Reg<rand_3::RAND_3_SPEC>;
#[doc = "TRNG RAND 3 Status Register"]
pub mod rand_3;
#[doc = "rand_4 (rw) register accessor: TRNG RAND 4 Status Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`rand_4::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`rand_4::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@rand_4`]
module"]
pub type RAND_4 = crate::Reg<rand_4::RAND_4_SPEC>;
#[doc = "TRNG RAND 4 Status Register"]
pub mod rand_4;
#[doc = "rand_5 (rw) register accessor: TRNG RAND 5 Status Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`rand_5::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`rand_5::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@rand_5`]
module"]
pub type RAND_5 = crate::Reg<rand_5::RAND_5_SPEC>;
#[doc = "TRNG RAND 5 Status Register"]
pub mod rand_5;
#[doc = "rand_6 (rw) register accessor: TRNG RAND 6 Status Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`rand_6::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`rand_6::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@rand_6`]
module"]
pub type RAND_6 = crate::Reg<rand_6::RAND_6_SPEC>;
#[doc = "TRNG RAND 6 Status Register"]
pub mod rand_6;
#[doc = "rand_7 (rw) register accessor: TRNG RAND 7 Status Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`rand_7::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`rand_7::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@rand_7`]
module"]
pub type RAND_7 = crate::Reg<rand_7::RAND_7_SPEC>;
#[doc = "TRNG RAND 7 Status Register"]
pub mod rand_7;
#[doc = "auto_rqsts (rw) register accessor: Auto-reseeding after random number requests by host reaches specified counter: 0 - disable counter, other - reload value for internal counter\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`auto_rqsts::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`auto_rqsts::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@auto_rqsts`]
module"]
pub type AUTO_RQSTS = crate::Reg<auto_rqsts::AUTO_RQSTS_SPEC>;
#[doc = "Auto-reseeding after random number requests by host reaches specified counter: 0 - disable counter, other - reload value for internal counter"]
pub mod auto_rqsts;
#[doc = "auto_age (rw) register accessor: Auto-reseeding after specified timer countdowns to 0: 0 - disable timer, other - reload value for internal timer\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`auto_age::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`auto_age::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@auto_age`]
module"]
pub type AUTO_AGE = crate::Reg<auto_age::AUTO_AGE_SPEC>;
#[doc = "Auto-reseeding after specified timer countdowns to 0: 0 - disable timer, other - reload value for internal timer"]
pub mod auto_age;
