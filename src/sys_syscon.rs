#[doc = r"Register block"]
#[repr(C)]
pub struct RegisterBlock {
    sys_syscfg_0: SYS_SYSCFG_0,
    sys_syscfg_1: SYS_SYSCFG_1,
    sys_syscfg_2: SYS_SYSCFG_2,
    sys_syscfg_3: SYS_SYSCFG_3,
    sys_syscfg_4: SYS_SYSCFG_4,
    sys_syscfg_5: SYS_SYSCFG_5,
    sys_syscfg_6: SYS_SYSCFG_6,
    sys_syscfg_7: SYS_SYSCFG_7,
    sys_syscfg_8: SYS_SYSCFG_8,
    sys_syscfg_9: SYS_SYSCFG_9,
    sys_syscfg_10: SYS_SYSCFG_10,
    sys_syscfg_11: SYS_SYSCFG_11,
    sys_syscfg_12: SYS_SYSCFG_12,
    sys_syscfg_13: SYS_SYSCFG_13,
    sys_syscfg_14: SYS_SYSCFG_14,
    sys_syscfg_15: SYS_SYSCFG_15,
    sys_syscfg_16: SYS_SYSCFG_16,
    sys_syscfg_17: SYS_SYSCFG_17,
    sys_syscfg_18: SYS_SYSCFG_18,
    sys_syscfg_19: SYS_SYSCFG_19,
    sys_syscfg_20: SYS_SYSCFG_20,
    sys_syscfg_21: SYS_SYSCFG_21,
    sys_syscfg_22: SYS_SYSCFG_22,
    sys_syscfg_23: SYS_SYSCFG_23,
    sys_syscfg_24: SYS_SYSCFG_24,
    sys_syscfg_25: SYS_SYSCFG_25,
    sys_syscfg_26: SYS_SYSCFG_26,
    sys_syscfg_27: SYS_SYSCFG_27,
    sys_syscfg_28: SYS_SYSCFG_28,
    sys_syscfg_29: SYS_SYSCFG_29,
    sys_syscfg_30: SYS_SYSCFG_30,
    sys_syscfg_31: SYS_SYSCFG_31,
    sys_syscfg_32: SYS_SYSCFG_32,
    sys_syscfg_33: SYS_SYSCFG_33,
    sys_syscfg_34: SYS_SYSCFG_34,
    sys_syscfg_35: SYS_SYSCFG_35,
    sys_syscfg_36: SYS_SYSCFG_36,
    sys_syscfg_37: SYS_SYSCFG_37,
    sys_syscfg_38: SYS_SYSCFG_38,
    sys_syscfg_39: SYS_SYSCFG_39,
}
impl RegisterBlock {
    #[doc = "0x00 - SYS SYSCONSAIF SYSCFG 0"]
    #[inline(always)]
    pub const fn sys_syscfg_0(&self) -> &SYS_SYSCFG_0 {
        &self.sys_syscfg_0
    }
    #[doc = "0x04 - SYS SYSCONSAIF SYSCFG 4"]
    #[inline(always)]
    pub const fn sys_syscfg_1(&self) -> &SYS_SYSCFG_1 {
        &self.sys_syscfg_1
    }
    #[doc = "0x08 - SYS SYSCONSAIF SYSCFG 8"]
    #[inline(always)]
    pub const fn sys_syscfg_2(&self) -> &SYS_SYSCFG_2 {
        &self.sys_syscfg_2
    }
    #[doc = "0x0c - SYS SYSCONSAIF SYSCFG 12: Set the GPIO voltage of all the 4 GPIO groups in this register"]
    #[inline(always)]
    pub const fn sys_syscfg_3(&self) -> &SYS_SYSCFG_3 {
        &self.sys_syscfg_3
    }
    #[doc = "0x10 - SYS SYSCONSAIF SYSCFG 16"]
    #[inline(always)]
    pub const fn sys_syscfg_4(&self) -> &SYS_SYSCFG_4 {
        &self.sys_syscfg_4
    }
    #[doc = "0x14 - SYS SYSCONSAIF SYSCFG 20"]
    #[inline(always)]
    pub const fn sys_syscfg_5(&self) -> &SYS_SYSCFG_5 {
        &self.sys_syscfg_5
    }
    #[doc = "0x18 - SYS SYSCONSAIF SYSCFG 24"]
    #[inline(always)]
    pub const fn sys_syscfg_6(&self) -> &SYS_SYSCFG_6 {
        &self.sys_syscfg_6
    }
    #[doc = "0x1c - SYS SYSCONSAIF SYSCFG 28"]
    #[inline(always)]
    pub const fn sys_syscfg_7(&self) -> &SYS_SYSCFG_7 {
        &self.sys_syscfg_7
    }
    #[doc = "0x20 - SYS SYSCONSAIF SYSCFG 32"]
    #[inline(always)]
    pub const fn sys_syscfg_8(&self) -> &SYS_SYSCFG_8 {
        &self.sys_syscfg_8
    }
    #[doc = "0x24 - SYS SYSCONSAIF SYSCFG 36"]
    #[inline(always)]
    pub const fn sys_syscfg_9(&self) -> &SYS_SYSCFG_9 {
        &self.sys_syscfg_9
    }
    #[doc = "0x28 - SYS SYSCONSAIF SYSCFG 40"]
    #[inline(always)]
    pub const fn sys_syscfg_10(&self) -> &SYS_SYSCFG_10 {
        &self.sys_syscfg_10
    }
    #[doc = "0x2c - SYS SYSCONSAIF SYSCFG 44"]
    #[inline(always)]
    pub const fn sys_syscfg_11(&self) -> &SYS_SYSCFG_11 {
        &self.sys_syscfg_11
    }
    #[doc = "0x30 - SYS SYSCONSAIF SYSCFG 48"]
    #[inline(always)]
    pub const fn sys_syscfg_12(&self) -> &SYS_SYSCFG_12 {
        &self.sys_syscfg_12
    }
    #[doc = "0x34 - SYS SYSCONSAIF SYSCFG 52"]
    #[inline(always)]
    pub const fn sys_syscfg_13(&self) -> &SYS_SYSCFG_13 {
        &self.sys_syscfg_13
    }
    #[doc = "0x38 - SYS SYSCONSAIF SYSCFG 56"]
    #[inline(always)]
    pub const fn sys_syscfg_14(&self) -> &SYS_SYSCFG_14 {
        &self.sys_syscfg_14
    }
    #[doc = "0x3c - SYS SYSCONSAIF SYSCFG 60"]
    #[inline(always)]
    pub const fn sys_syscfg_15(&self) -> &SYS_SYSCFG_15 {
        &self.sys_syscfg_15
    }
    #[doc = "0x40 - SYS SYSCONSAIF SYSCFG 64"]
    #[inline(always)]
    pub const fn sys_syscfg_16(&self) -> &SYS_SYSCFG_16 {
        &self.sys_syscfg_16
    }
    #[doc = "0x44 - SYS SYSCONSAIF SYSCFG 68"]
    #[inline(always)]
    pub const fn sys_syscfg_17(&self) -> &SYS_SYSCFG_17 {
        &self.sys_syscfg_17
    }
    #[doc = "0x48 - SYS SYSCONSAIF SYSCFG 72"]
    #[inline(always)]
    pub const fn sys_syscfg_18(&self) -> &SYS_SYSCFG_18 {
        &self.sys_syscfg_18
    }
    #[doc = "0x4c - SYS SYSCONSAIF SYSCFG 76"]
    #[inline(always)]
    pub const fn sys_syscfg_19(&self) -> &SYS_SYSCFG_19 {
        &self.sys_syscfg_19
    }
    #[doc = "0x50 - SYS SYSCONSAIF SYSCFG 80"]
    #[inline(always)]
    pub const fn sys_syscfg_20(&self) -> &SYS_SYSCFG_20 {
        &self.sys_syscfg_20
    }
    #[doc = "0x54 - SYS SYSCONSAIF SYSCFG 84"]
    #[inline(always)]
    pub const fn sys_syscfg_21(&self) -> &SYS_SYSCFG_21 {
        &self.sys_syscfg_21
    }
    #[doc = "0x58 - SYS SYSCONSAIF SYSCFG 88"]
    #[inline(always)]
    pub const fn sys_syscfg_22(&self) -> &SYS_SYSCFG_22 {
        &self.sys_syscfg_22
    }
    #[doc = "0x5c - SYS SYSCONSAIF SYSCFG 92"]
    #[inline(always)]
    pub const fn sys_syscfg_23(&self) -> &SYS_SYSCFG_23 {
        &self.sys_syscfg_23
    }
    #[doc = "0x60 - SYS SYSCONSAIF SYSCFG 96"]
    #[inline(always)]
    pub const fn sys_syscfg_24(&self) -> &SYS_SYSCFG_24 {
        &self.sys_syscfg_24
    }
    #[doc = "0x64 - SYS SYSCONSAIF SYSCFG 100"]
    #[inline(always)]
    pub const fn sys_syscfg_25(&self) -> &SYS_SYSCFG_25 {
        &self.sys_syscfg_25
    }
    #[doc = "0x68 - SYS SYSCONSAIF SYSCFG 26"]
    #[inline(always)]
    pub const fn sys_syscfg_26(&self) -> &SYS_SYSCFG_26 {
        &self.sys_syscfg_26
    }
    #[doc = "0x6c - SYS SYSCONSAIF SYSCFG 27"]
    #[inline(always)]
    pub const fn sys_syscfg_27(&self) -> &SYS_SYSCFG_27 {
        &self.sys_syscfg_27
    }
    #[doc = "0x70 - SYS SYSCONSAIF SYSCFG 28"]
    #[inline(always)]
    pub const fn sys_syscfg_28(&self) -> &SYS_SYSCFG_28 {
        &self.sys_syscfg_28
    }
    #[doc = "0x74 - SYS SYSCONSAIF SYSCFG 29"]
    #[inline(always)]
    pub const fn sys_syscfg_29(&self) -> &SYS_SYSCFG_29 {
        &self.sys_syscfg_29
    }
    #[doc = "0x78 - SYS SYSCONSAIF SYSCFG 30"]
    #[inline(always)]
    pub const fn sys_syscfg_30(&self) -> &SYS_SYSCFG_30 {
        &self.sys_syscfg_30
    }
    #[doc = "0x7c - SYS SYSCONSAIF SYSCFG 31"]
    #[inline(always)]
    pub const fn sys_syscfg_31(&self) -> &SYS_SYSCFG_31 {
        &self.sys_syscfg_31
    }
    #[doc = "0x80 - SYS SYSCONSAIF SYSCFG 32"]
    #[inline(always)]
    pub const fn sys_syscfg_32(&self) -> &SYS_SYSCFG_32 {
        &self.sys_syscfg_32
    }
    #[doc = "0x84 - SYS SYSCONSAIF SYSCFG 132"]
    #[inline(always)]
    pub const fn sys_syscfg_33(&self) -> &SYS_SYSCFG_33 {
        &self.sys_syscfg_33
    }
    #[doc = "0x88 - SYS SYSCONSAIF SYSCFG 136"]
    #[inline(always)]
    pub const fn sys_syscfg_34(&self) -> &SYS_SYSCFG_34 {
        &self.sys_syscfg_34
    }
    #[doc = "0x8c - SYS SYSCONSAIF SYSCFG 140"]
    #[inline(always)]
    pub const fn sys_syscfg_35(&self) -> &SYS_SYSCFG_35 {
        &self.sys_syscfg_35
    }
    #[doc = "0x90 - SYS SYSCONSAIF SYSCFG 144"]
    #[inline(always)]
    pub const fn sys_syscfg_36(&self) -> &SYS_SYSCFG_36 {
        &self.sys_syscfg_36
    }
    #[doc = "0x94 - SYS SYSCONSAIF SYSCFG 148"]
    #[inline(always)]
    pub const fn sys_syscfg_37(&self) -> &SYS_SYSCFG_37 {
        &self.sys_syscfg_37
    }
    #[doc = "0x98 - SYS SYSCONSAIF SYSCFG 152"]
    #[inline(always)]
    pub const fn sys_syscfg_38(&self) -> &SYS_SYSCFG_38 {
        &self.sys_syscfg_38
    }
    #[doc = "0x9c - SYS SYSCONSAIF SYSCFG 156"]
    #[inline(always)]
    pub const fn sys_syscfg_39(&self) -> &SYS_SYSCFG_39 {
        &self.sys_syscfg_39
    }
}
#[doc = "sys_syscfg_0 (rw) register accessor: SYS SYSCONSAIF SYSCFG 0\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`sys_syscfg_0::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`sys_syscfg_0::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@sys_syscfg_0`]
module"]
pub type SYS_SYSCFG_0 = crate::Reg<sys_syscfg_0::SYS_SYSCFG_0_SPEC>;
#[doc = "SYS SYSCONSAIF SYSCFG 0"]
pub mod sys_syscfg_0;
#[doc = "sys_syscfg_1 (rw) register accessor: SYS SYSCONSAIF SYSCFG 4\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`sys_syscfg_1::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`sys_syscfg_1::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@sys_syscfg_1`]
module"]
pub type SYS_SYSCFG_1 = crate::Reg<sys_syscfg_1::SYS_SYSCFG_1_SPEC>;
#[doc = "SYS SYSCONSAIF SYSCFG 4"]
pub mod sys_syscfg_1;
#[doc = "sys_syscfg_2 (rw) register accessor: SYS SYSCONSAIF SYSCFG 8\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`sys_syscfg_2::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`sys_syscfg_2::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@sys_syscfg_2`]
module"]
pub type SYS_SYSCFG_2 = crate::Reg<sys_syscfg_2::SYS_SYSCFG_2_SPEC>;
#[doc = "SYS SYSCONSAIF SYSCFG 8"]
pub mod sys_syscfg_2;
#[doc = "sys_syscfg_3 (rw) register accessor: SYS SYSCONSAIF SYSCFG 12: Set the GPIO voltage of all the 4 GPIO groups in this register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`sys_syscfg_3::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`sys_syscfg_3::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@sys_syscfg_3`]
module"]
pub type SYS_SYSCFG_3 = crate::Reg<sys_syscfg_3::SYS_SYSCFG_3_SPEC>;
#[doc = "SYS SYSCONSAIF SYSCFG 12: Set the GPIO voltage of all the 4 GPIO groups in this register"]
pub mod sys_syscfg_3;
#[doc = "sys_syscfg_4 (rw) register accessor: SYS SYSCONSAIF SYSCFG 16\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`sys_syscfg_4::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`sys_syscfg_4::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@sys_syscfg_4`]
module"]
pub type SYS_SYSCFG_4 = crate::Reg<sys_syscfg_4::SYS_SYSCFG_4_SPEC>;
#[doc = "SYS SYSCONSAIF SYSCFG 16"]
pub mod sys_syscfg_4;
#[doc = "sys_syscfg_5 (rw) register accessor: SYS SYSCONSAIF SYSCFG 20\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`sys_syscfg_5::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`sys_syscfg_5::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@sys_syscfg_5`]
module"]
pub type SYS_SYSCFG_5 = crate::Reg<sys_syscfg_5::SYS_SYSCFG_5_SPEC>;
#[doc = "SYS SYSCONSAIF SYSCFG 20"]
pub mod sys_syscfg_5;
#[doc = "sys_syscfg_6 (rw) register accessor: SYS SYSCONSAIF SYSCFG 24\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`sys_syscfg_6::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`sys_syscfg_6::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@sys_syscfg_6`]
module"]
pub type SYS_SYSCFG_6 = crate::Reg<sys_syscfg_6::SYS_SYSCFG_6_SPEC>;
#[doc = "SYS SYSCONSAIF SYSCFG 24"]
pub mod sys_syscfg_6;
#[doc = "sys_syscfg_7 (rw) register accessor: SYS SYSCONSAIF SYSCFG 28\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`sys_syscfg_7::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`sys_syscfg_7::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@sys_syscfg_7`]
module"]
pub type SYS_SYSCFG_7 = crate::Reg<sys_syscfg_7::SYS_SYSCFG_7_SPEC>;
#[doc = "SYS SYSCONSAIF SYSCFG 28"]
pub mod sys_syscfg_7;
#[doc = "sys_syscfg_8 (rw) register accessor: SYS SYSCONSAIF SYSCFG 32\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`sys_syscfg_8::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`sys_syscfg_8::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@sys_syscfg_8`]
module"]
pub type SYS_SYSCFG_8 = crate::Reg<sys_syscfg_8::SYS_SYSCFG_8_SPEC>;
#[doc = "SYS SYSCONSAIF SYSCFG 32"]
pub mod sys_syscfg_8;
#[doc = "sys_syscfg_9 (rw) register accessor: SYS SYSCONSAIF SYSCFG 36\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`sys_syscfg_9::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`sys_syscfg_9::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@sys_syscfg_9`]
module"]
pub type SYS_SYSCFG_9 = crate::Reg<sys_syscfg_9::SYS_SYSCFG_9_SPEC>;
#[doc = "SYS SYSCONSAIF SYSCFG 36"]
pub mod sys_syscfg_9;
#[doc = "sys_syscfg_10 (rw) register accessor: SYS SYSCONSAIF SYSCFG 40\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`sys_syscfg_10::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`sys_syscfg_10::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@sys_syscfg_10`]
module"]
pub type SYS_SYSCFG_10 = crate::Reg<sys_syscfg_10::SYS_SYSCFG_10_SPEC>;
#[doc = "SYS SYSCONSAIF SYSCFG 40"]
pub mod sys_syscfg_10;
#[doc = "sys_syscfg_11 (rw) register accessor: SYS SYSCONSAIF SYSCFG 44\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`sys_syscfg_11::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`sys_syscfg_11::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@sys_syscfg_11`]
module"]
pub type SYS_SYSCFG_11 = crate::Reg<sys_syscfg_11::SYS_SYSCFG_11_SPEC>;
#[doc = "SYS SYSCONSAIF SYSCFG 44"]
pub mod sys_syscfg_11;
#[doc = "sys_syscfg_12 (rw) register accessor: SYS SYSCONSAIF SYSCFG 48\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`sys_syscfg_12::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`sys_syscfg_12::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@sys_syscfg_12`]
module"]
pub type SYS_SYSCFG_12 = crate::Reg<sys_syscfg_12::SYS_SYSCFG_12_SPEC>;
#[doc = "SYS SYSCONSAIF SYSCFG 48"]
pub mod sys_syscfg_12;
#[doc = "sys_syscfg_13 (rw) register accessor: SYS SYSCONSAIF SYSCFG 52\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`sys_syscfg_13::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`sys_syscfg_13::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@sys_syscfg_13`]
module"]
pub type SYS_SYSCFG_13 = crate::Reg<sys_syscfg_13::SYS_SYSCFG_13_SPEC>;
#[doc = "SYS SYSCONSAIF SYSCFG 52"]
pub mod sys_syscfg_13;
#[doc = "sys_syscfg_14 (rw) register accessor: SYS SYSCONSAIF SYSCFG 56\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`sys_syscfg_14::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`sys_syscfg_14::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@sys_syscfg_14`]
module"]
pub type SYS_SYSCFG_14 = crate::Reg<sys_syscfg_14::SYS_SYSCFG_14_SPEC>;
#[doc = "SYS SYSCONSAIF SYSCFG 56"]
pub mod sys_syscfg_14;
#[doc = "sys_syscfg_15 (rw) register accessor: SYS SYSCONSAIF SYSCFG 60\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`sys_syscfg_15::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`sys_syscfg_15::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@sys_syscfg_15`]
module"]
pub type SYS_SYSCFG_15 = crate::Reg<sys_syscfg_15::SYS_SYSCFG_15_SPEC>;
#[doc = "SYS SYSCONSAIF SYSCFG 60"]
pub mod sys_syscfg_15;
#[doc = "sys_syscfg_16 (rw) register accessor: SYS SYSCONSAIF SYSCFG 64\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`sys_syscfg_16::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`sys_syscfg_16::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@sys_syscfg_16`]
module"]
pub type SYS_SYSCFG_16 = crate::Reg<sys_syscfg_16::SYS_SYSCFG_16_SPEC>;
#[doc = "SYS SYSCONSAIF SYSCFG 64"]
pub mod sys_syscfg_16;
#[doc = "sys_syscfg_17 (rw) register accessor: SYS SYSCONSAIF SYSCFG 68\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`sys_syscfg_17::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`sys_syscfg_17::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@sys_syscfg_17`]
module"]
pub type SYS_SYSCFG_17 = crate::Reg<sys_syscfg_17::SYS_SYSCFG_17_SPEC>;
#[doc = "SYS SYSCONSAIF SYSCFG 68"]
pub mod sys_syscfg_17;
#[doc = "sys_syscfg_18 (rw) register accessor: SYS SYSCONSAIF SYSCFG 72\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`sys_syscfg_18::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`sys_syscfg_18::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@sys_syscfg_18`]
module"]
pub type SYS_SYSCFG_18 = crate::Reg<sys_syscfg_18::SYS_SYSCFG_18_SPEC>;
#[doc = "SYS SYSCONSAIF SYSCFG 72"]
pub mod sys_syscfg_18;
#[doc = "sys_syscfg_19 (rw) register accessor: SYS SYSCONSAIF SYSCFG 76\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`sys_syscfg_19::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`sys_syscfg_19::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@sys_syscfg_19`]
module"]
pub type SYS_SYSCFG_19 = crate::Reg<sys_syscfg_19::SYS_SYSCFG_19_SPEC>;
#[doc = "SYS SYSCONSAIF SYSCFG 76"]
pub mod sys_syscfg_19;
#[doc = "sys_syscfg_20 (rw) register accessor: SYS SYSCONSAIF SYSCFG 80\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`sys_syscfg_20::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`sys_syscfg_20::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@sys_syscfg_20`]
module"]
pub type SYS_SYSCFG_20 = crate::Reg<sys_syscfg_20::SYS_SYSCFG_20_SPEC>;
#[doc = "SYS SYSCONSAIF SYSCFG 80"]
pub mod sys_syscfg_20;
#[doc = "sys_syscfg_21 (rw) register accessor: SYS SYSCONSAIF SYSCFG 84\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`sys_syscfg_21::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`sys_syscfg_21::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@sys_syscfg_21`]
module"]
pub type SYS_SYSCFG_21 = crate::Reg<sys_syscfg_21::SYS_SYSCFG_21_SPEC>;
#[doc = "SYS SYSCONSAIF SYSCFG 84"]
pub mod sys_syscfg_21;
#[doc = "sys_syscfg_22 (rw) register accessor: SYS SYSCONSAIF SYSCFG 88\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`sys_syscfg_22::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`sys_syscfg_22::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@sys_syscfg_22`]
module"]
pub type SYS_SYSCFG_22 = crate::Reg<sys_syscfg_22::SYS_SYSCFG_22_SPEC>;
#[doc = "SYS SYSCONSAIF SYSCFG 88"]
pub mod sys_syscfg_22;
#[doc = "sys_syscfg_23 (rw) register accessor: SYS SYSCONSAIF SYSCFG 92\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`sys_syscfg_23::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`sys_syscfg_23::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@sys_syscfg_23`]
module"]
pub type SYS_SYSCFG_23 = crate::Reg<sys_syscfg_23::SYS_SYSCFG_23_SPEC>;
#[doc = "SYS SYSCONSAIF SYSCFG 92"]
pub mod sys_syscfg_23;
#[doc = "sys_syscfg_24 (rw) register accessor: SYS SYSCONSAIF SYSCFG 96\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`sys_syscfg_24::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`sys_syscfg_24::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@sys_syscfg_24`]
module"]
pub type SYS_SYSCFG_24 = crate::Reg<sys_syscfg_24::SYS_SYSCFG_24_SPEC>;
#[doc = "SYS SYSCONSAIF SYSCFG 96"]
pub mod sys_syscfg_24;
#[doc = "sys_syscfg_25 (rw) register accessor: SYS SYSCONSAIF SYSCFG 100\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`sys_syscfg_25::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`sys_syscfg_25::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@sys_syscfg_25`]
module"]
pub type SYS_SYSCFG_25 = crate::Reg<sys_syscfg_25::SYS_SYSCFG_25_SPEC>;
#[doc = "SYS SYSCONSAIF SYSCFG 100"]
pub mod sys_syscfg_25;
#[doc = "sys_syscfg_26 (rw) register accessor: SYS SYSCONSAIF SYSCFG 26\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`sys_syscfg_26::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`sys_syscfg_26::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@sys_syscfg_26`]
module"]
pub type SYS_SYSCFG_26 = crate::Reg<sys_syscfg_26::SYS_SYSCFG_26_SPEC>;
#[doc = "SYS SYSCONSAIF SYSCFG 26"]
pub mod sys_syscfg_26;
#[doc = "sys_syscfg_27 (rw) register accessor: SYS SYSCONSAIF SYSCFG 27\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`sys_syscfg_27::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`sys_syscfg_27::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@sys_syscfg_27`]
module"]
pub type SYS_SYSCFG_27 = crate::Reg<sys_syscfg_27::SYS_SYSCFG_27_SPEC>;
#[doc = "SYS SYSCONSAIF SYSCFG 27"]
pub mod sys_syscfg_27;
#[doc = "sys_syscfg_28 (rw) register accessor: SYS SYSCONSAIF SYSCFG 28\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`sys_syscfg_28::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`sys_syscfg_28::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@sys_syscfg_28`]
module"]
pub type SYS_SYSCFG_28 = crate::Reg<sys_syscfg_28::SYS_SYSCFG_28_SPEC>;
#[doc = "SYS SYSCONSAIF SYSCFG 28"]
pub mod sys_syscfg_28;
#[doc = "sys_syscfg_29 (rw) register accessor: SYS SYSCONSAIF SYSCFG 29\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`sys_syscfg_29::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`sys_syscfg_29::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@sys_syscfg_29`]
module"]
pub type SYS_SYSCFG_29 = crate::Reg<sys_syscfg_29::SYS_SYSCFG_29_SPEC>;
#[doc = "SYS SYSCONSAIF SYSCFG 29"]
pub mod sys_syscfg_29;
#[doc = "sys_syscfg_30 (rw) register accessor: SYS SYSCONSAIF SYSCFG 30\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`sys_syscfg_30::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`sys_syscfg_30::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@sys_syscfg_30`]
module"]
pub type SYS_SYSCFG_30 = crate::Reg<sys_syscfg_30::SYS_SYSCFG_30_SPEC>;
#[doc = "SYS SYSCONSAIF SYSCFG 30"]
pub mod sys_syscfg_30;
#[doc = "sys_syscfg_31 (rw) register accessor: SYS SYSCONSAIF SYSCFG 31\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`sys_syscfg_31::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`sys_syscfg_31::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@sys_syscfg_31`]
module"]
pub type SYS_SYSCFG_31 = crate::Reg<sys_syscfg_31::SYS_SYSCFG_31_SPEC>;
#[doc = "SYS SYSCONSAIF SYSCFG 31"]
pub mod sys_syscfg_31;
#[doc = "sys_syscfg_32 (rw) register accessor: SYS SYSCONSAIF SYSCFG 32\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`sys_syscfg_32::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`sys_syscfg_32::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@sys_syscfg_32`]
module"]
pub type SYS_SYSCFG_32 = crate::Reg<sys_syscfg_32::SYS_SYSCFG_32_SPEC>;
#[doc = "SYS SYSCONSAIF SYSCFG 32"]
pub mod sys_syscfg_32;
#[doc = "sys_syscfg_33 (rw) register accessor: SYS SYSCONSAIF SYSCFG 132\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`sys_syscfg_33::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`sys_syscfg_33::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@sys_syscfg_33`]
module"]
pub type SYS_SYSCFG_33 = crate::Reg<sys_syscfg_33::SYS_SYSCFG_33_SPEC>;
#[doc = "SYS SYSCONSAIF SYSCFG 132"]
pub mod sys_syscfg_33;
#[doc = "sys_syscfg_34 (rw) register accessor: SYS SYSCONSAIF SYSCFG 136\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`sys_syscfg_34::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`sys_syscfg_34::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@sys_syscfg_34`]
module"]
pub type SYS_SYSCFG_34 = crate::Reg<sys_syscfg_34::SYS_SYSCFG_34_SPEC>;
#[doc = "SYS SYSCONSAIF SYSCFG 136"]
pub mod sys_syscfg_34;
#[doc = "sys_syscfg_35 (rw) register accessor: SYS SYSCONSAIF SYSCFG 140\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`sys_syscfg_35::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`sys_syscfg_35::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@sys_syscfg_35`]
module"]
pub type SYS_SYSCFG_35 = crate::Reg<sys_syscfg_35::SYS_SYSCFG_35_SPEC>;
#[doc = "SYS SYSCONSAIF SYSCFG 140"]
pub mod sys_syscfg_35;
#[doc = "sys_syscfg_36 (rw) register accessor: SYS SYSCONSAIF SYSCFG 144\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`sys_syscfg_36::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`sys_syscfg_36::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@sys_syscfg_36`]
module"]
pub type SYS_SYSCFG_36 = crate::Reg<sys_syscfg_36::SYS_SYSCFG_36_SPEC>;
#[doc = "SYS SYSCONSAIF SYSCFG 144"]
pub mod sys_syscfg_36;
#[doc = "sys_syscfg_37 (rw) register accessor: SYS SYSCONSAIF SYSCFG 148\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`sys_syscfg_37::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`sys_syscfg_37::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@sys_syscfg_37`]
module"]
pub type SYS_SYSCFG_37 = crate::Reg<sys_syscfg_37::SYS_SYSCFG_37_SPEC>;
#[doc = "SYS SYSCONSAIF SYSCFG 148"]
pub mod sys_syscfg_37;
#[doc = "sys_syscfg_38 (rw) register accessor: SYS SYSCONSAIF SYSCFG 152\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`sys_syscfg_38::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`sys_syscfg_38::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@sys_syscfg_38`]
module"]
pub type SYS_SYSCFG_38 = crate::Reg<sys_syscfg_38::SYS_SYSCFG_38_SPEC>;
#[doc = "SYS SYSCONSAIF SYSCFG 152"]
pub mod sys_syscfg_38;
#[doc = "sys_syscfg_39 (rw) register accessor: SYS SYSCONSAIF SYSCFG 156\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`sys_syscfg_39::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`sys_syscfg_39::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@sys_syscfg_39`]
module"]
pub type SYS_SYSCFG_39 = crate::Reg<sys_syscfg_39::SYS_SYSCFG_39_SPEC>;
#[doc = "SYS SYSCONSAIF SYSCFG 156"]
pub mod sys_syscfg_39;
