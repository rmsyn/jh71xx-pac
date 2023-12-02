#[doc = r"Register block"]
#[repr(C)]
pub struct RegisterBlock {
    sys_sysconsaif_syscfg0: SYS_SYSCONSAIF_SYSCFG0,
    sys_sysconsaif_syscfg4: SYS_SYSCONSAIF_SYSCFG4,
    sys_sysconsaif_syscfg8: SYS_SYSCONSAIF_SYSCFG8,
    sys_sysconsaif_syscfg12: SYS_SYSCONSAIF_SYSCFG12,
    sys_sysconsaif_syscfg16: SYS_SYSCONSAIF_SYSCFG16,
    sys_sysconsaif_syscfg20: SYS_SYSCONSAIF_SYSCFG20,
    sys_sysconsaif_syscfg24: SYS_SYSCONSAIF_SYSCFG24,
    sys_sysconsaif_syscfg28: SYS_SYSCONSAIF_SYSCFG28,
    sys_sysconsaif_syscfg32: SYS_SYSCONSAIF_SYSCFG32,
    sys_sysconsaif_syscfg36: SYS_SYSCONSAIF_SYSCFG36,
    sys_sysconsaif_syscfg40: SYS_SYSCONSAIF_SYSCFG40,
    sys_sysconsaif_syscfg44: SYS_SYSCONSAIF_SYSCFG44,
    sys_sysconsaif_syscfg48: SYS_SYSCONSAIF_SYSCFG48,
    sys_sysconsaif_syscfg52: SYS_SYSCONSAIF_SYSCFG52,
    sys_sysconsaif_syscfg56: SYS_SYSCONSAIF_SYSCFG56,
    sys_sysconsaif_syscfg60: SYS_SYSCONSAIF_SYSCFG60,
    sys_sysconsaif_syscfg64: SYS_SYSCONSAIF_SYSCFG64,
    sys_sysconsaif_syscfg68: SYS_SYSCONSAIF_SYSCFG68,
    sys_sysconsaif_syscfg72: SYS_SYSCONSAIF_SYSCFG72,
    sys_sysconsaif_syscfg76: SYS_SYSCONSAIF_SYSCFG76,
    sys_sysconsaif_syscfg80: SYS_SYSCONSAIF_SYSCFG80,
    sys_sysconsaif_syscfg84: SYS_SYSCONSAIF_SYSCFG84,
    sys_sysconsaif_syscfg88: SYS_SYSCONSAIF_SYSCFG88,
    sys_sysconsaif_syscfg92: SYS_SYSCONSAIF_SYSCFG92,
    sys_sysconsaif_syscfg96: SYS_SYSCONSAIF_SYSCFG96,
    sys_sysconsaif_syscfg100: SYS_SYSCONSAIF_SYSCFG100,
    sys_sysconsaif_syscfg104: SYS_SYSCONSAIF_SYSCFG104,
    sys_sysconsaif_syscfg108: SYS_SYSCONSAIF_SYSCFG108,
    sys_sysconsaif_syscfg112: SYS_SYSCONSAIF_SYSCFG112,
    sys_sysconsaif_syscfg116: SYS_SYSCONSAIF_SYSCFG116,
    sys_sysconsaif_syscfg120: SYS_SYSCONSAIF_SYSCFG120,
    sys_sysconsaif_syscfg124: SYS_SYSCONSAIF_SYSCFG124,
    sys_sysconsaif_syscfg128: SYS_SYSCONSAIF_SYSCFG128,
    _reserved33: [u8; 0x04],
    sys_sysconsaif_syscfg136: SYS_SYSCONSAIF_SYSCFG136,
    sys_sysconsaif_syscfg140: SYS_SYSCONSAIF_SYSCFG140,
    sys_sysconsaif_syscfg144: SYS_SYSCONSAIF_SYSCFG144,
    sys_sysconsaif_syscfg148: SYS_SYSCONSAIF_SYSCFG148,
    sys_sysconsaif_syscfg152: SYS_SYSCONSAIF_SYSCFG152,
    sys_sysconsaif_syscfg156: SYS_SYSCONSAIF_SYSCFG156,
}
impl RegisterBlock {
    #[doc = "0x00 - SYS SYSCONSAIF SYSCFG 0"]
    #[inline(always)]
    pub const fn sys_sysconsaif_syscfg0(&self) -> &SYS_SYSCONSAIF_SYSCFG0 {
        &self.sys_sysconsaif_syscfg0
    }
    #[doc = "0x04 - SYS SYSCONSAIF SYSCFG 4"]
    #[inline(always)]
    pub const fn sys_sysconsaif_syscfg4(&self) -> &SYS_SYSCONSAIF_SYSCFG4 {
        &self.sys_sysconsaif_syscfg4
    }
    #[doc = "0x08 - SYS SYSCONSAIF SYSCFG 8"]
    #[inline(always)]
    pub const fn sys_sysconsaif_syscfg8(&self) -> &SYS_SYSCONSAIF_SYSCFG8 {
        &self.sys_sysconsaif_syscfg8
    }
    #[doc = "0x0c - SYS SYSCONSAIF SYSCFG 12: Set the GPIO voltage of all the 4 GPIO groups in this register"]
    #[inline(always)]
    pub const fn sys_sysconsaif_syscfg12(&self) -> &SYS_SYSCONSAIF_SYSCFG12 {
        &self.sys_sysconsaif_syscfg12
    }
    #[doc = "0x10 - SYS SYSCONSAIF SYSCFG 16"]
    #[inline(always)]
    pub const fn sys_sysconsaif_syscfg16(&self) -> &SYS_SYSCONSAIF_SYSCFG16 {
        &self.sys_sysconsaif_syscfg16
    }
    #[doc = "0x14 - SYS SYSCONSAIF SYSCFG 20"]
    #[inline(always)]
    pub const fn sys_sysconsaif_syscfg20(&self) -> &SYS_SYSCONSAIF_SYSCFG20 {
        &self.sys_sysconsaif_syscfg20
    }
    #[doc = "0x18 - SYS SYSCONSAIF SYSCFG 24"]
    #[inline(always)]
    pub const fn sys_sysconsaif_syscfg24(&self) -> &SYS_SYSCONSAIF_SYSCFG24 {
        &self.sys_sysconsaif_syscfg24
    }
    #[doc = "0x1c - SYS SYSCONSAIF SYSCFG 28"]
    #[inline(always)]
    pub const fn sys_sysconsaif_syscfg28(&self) -> &SYS_SYSCONSAIF_SYSCFG28 {
        &self.sys_sysconsaif_syscfg28
    }
    #[doc = "0x20 - SYS SYSCONSAIF SYSCFG 32"]
    #[inline(always)]
    pub const fn sys_sysconsaif_syscfg32(&self) -> &SYS_SYSCONSAIF_SYSCFG32 {
        &self.sys_sysconsaif_syscfg32
    }
    #[doc = "0x24 - SYS SYSCONSAIF SYSCFG 36"]
    #[inline(always)]
    pub const fn sys_sysconsaif_syscfg36(&self) -> &SYS_SYSCONSAIF_SYSCFG36 {
        &self.sys_sysconsaif_syscfg36
    }
    #[doc = "0x28 - SYS SYSCONSAIF SYSCFG 40"]
    #[inline(always)]
    pub const fn sys_sysconsaif_syscfg40(&self) -> &SYS_SYSCONSAIF_SYSCFG40 {
        &self.sys_sysconsaif_syscfg40
    }
    #[doc = "0x2c - SYS SYSCONSAIF SYSCFG 44"]
    #[inline(always)]
    pub const fn sys_sysconsaif_syscfg44(&self) -> &SYS_SYSCONSAIF_SYSCFG44 {
        &self.sys_sysconsaif_syscfg44
    }
    #[doc = "0x30 - SYS SYSCONSAIF SYSCFG 48"]
    #[inline(always)]
    pub const fn sys_sysconsaif_syscfg48(&self) -> &SYS_SYSCONSAIF_SYSCFG48 {
        &self.sys_sysconsaif_syscfg48
    }
    #[doc = "0x34 - SYS SYSCONSAIF SYSCFG 52"]
    #[inline(always)]
    pub const fn sys_sysconsaif_syscfg52(&self) -> &SYS_SYSCONSAIF_SYSCFG52 {
        &self.sys_sysconsaif_syscfg52
    }
    #[doc = "0x38 - SYS SYSCONSAIF SYSCFG 56"]
    #[inline(always)]
    pub const fn sys_sysconsaif_syscfg56(&self) -> &SYS_SYSCONSAIF_SYSCFG56 {
        &self.sys_sysconsaif_syscfg56
    }
    #[doc = "0x3c - SYS SYSCONSAIF SYSCFG 60"]
    #[inline(always)]
    pub const fn sys_sysconsaif_syscfg60(&self) -> &SYS_SYSCONSAIF_SYSCFG60 {
        &self.sys_sysconsaif_syscfg60
    }
    #[doc = "0x40 - SYS SYSCONSAIF SYSCFG 64"]
    #[inline(always)]
    pub const fn sys_sysconsaif_syscfg64(&self) -> &SYS_SYSCONSAIF_SYSCFG64 {
        &self.sys_sysconsaif_syscfg64
    }
    #[doc = "0x44 - SYS SYSCONSAIF SYSCFG 68"]
    #[inline(always)]
    pub const fn sys_sysconsaif_syscfg68(&self) -> &SYS_SYSCONSAIF_SYSCFG68 {
        &self.sys_sysconsaif_syscfg68
    }
    #[doc = "0x48 - SYS SYSCONSAIF SYSCFG 72"]
    #[inline(always)]
    pub const fn sys_sysconsaif_syscfg72(&self) -> &SYS_SYSCONSAIF_SYSCFG72 {
        &self.sys_sysconsaif_syscfg72
    }
    #[doc = "0x4c - SYS SYSCONSAIF SYSCFG 76"]
    #[inline(always)]
    pub const fn sys_sysconsaif_syscfg76(&self) -> &SYS_SYSCONSAIF_SYSCFG76 {
        &self.sys_sysconsaif_syscfg76
    }
    #[doc = "0x50 - SYS SYSCONSAIF SYSCFG 80"]
    #[inline(always)]
    pub const fn sys_sysconsaif_syscfg80(&self) -> &SYS_SYSCONSAIF_SYSCFG80 {
        &self.sys_sysconsaif_syscfg80
    }
    #[doc = "0x54 - SYS SYSCONSAIF SYSCFG 84"]
    #[inline(always)]
    pub const fn sys_sysconsaif_syscfg84(&self) -> &SYS_SYSCONSAIF_SYSCFG84 {
        &self.sys_sysconsaif_syscfg84
    }
    #[doc = "0x58 - SYS SYSCONSAIF SYSCFG 88"]
    #[inline(always)]
    pub const fn sys_sysconsaif_syscfg88(&self) -> &SYS_SYSCONSAIF_SYSCFG88 {
        &self.sys_sysconsaif_syscfg88
    }
    #[doc = "0x5c - SYS SYSCONSAIF SYSCFG 92"]
    #[inline(always)]
    pub const fn sys_sysconsaif_syscfg92(&self) -> &SYS_SYSCONSAIF_SYSCFG92 {
        &self.sys_sysconsaif_syscfg92
    }
    #[doc = "0x60 - SYS SYSCONSAIF SYSCFG 96"]
    #[inline(always)]
    pub const fn sys_sysconsaif_syscfg96(&self) -> &SYS_SYSCONSAIF_SYSCFG96 {
        &self.sys_sysconsaif_syscfg96
    }
    #[doc = "0x64 - SYS SYSCONSAIF SYSCFG 100"]
    #[inline(always)]
    pub const fn sys_sysconsaif_syscfg100(&self) -> &SYS_SYSCONSAIF_SYSCFG100 {
        &self.sys_sysconsaif_syscfg100
    }
    #[doc = "0x68 - SYS SYSCONSAIF SYSCFG 104"]
    #[inline(always)]
    pub const fn sys_sysconsaif_syscfg104(&self) -> &SYS_SYSCONSAIF_SYSCFG104 {
        &self.sys_sysconsaif_syscfg104
    }
    #[doc = "0x6c - SYS SYSCONSAIF SYSCFG 108"]
    #[inline(always)]
    pub const fn sys_sysconsaif_syscfg108(&self) -> &SYS_SYSCONSAIF_SYSCFG108 {
        &self.sys_sysconsaif_syscfg108
    }
    #[doc = "0x70 - SYS SYSCONSAIF SYSCFG 112"]
    #[inline(always)]
    pub const fn sys_sysconsaif_syscfg112(&self) -> &SYS_SYSCONSAIF_SYSCFG112 {
        &self.sys_sysconsaif_syscfg112
    }
    #[doc = "0x74 - SYS SYSCONSAIF SYSCFG 116"]
    #[inline(always)]
    pub const fn sys_sysconsaif_syscfg116(&self) -> &SYS_SYSCONSAIF_SYSCFG116 {
        &self.sys_sysconsaif_syscfg116
    }
    #[doc = "0x78 - SYS SYSCONSAIF SYSCFG 120"]
    #[inline(always)]
    pub const fn sys_sysconsaif_syscfg120(&self) -> &SYS_SYSCONSAIF_SYSCFG120 {
        &self.sys_sysconsaif_syscfg120
    }
    #[doc = "0x7c - SYS SYSCONSAIF SYSCFG 124"]
    #[inline(always)]
    pub const fn sys_sysconsaif_syscfg124(&self) -> &SYS_SYSCONSAIF_SYSCFG124 {
        &self.sys_sysconsaif_syscfg124
    }
    #[doc = "0x80 - SYS SYSCONSAIF SYSCFG 128"]
    #[inline(always)]
    pub const fn sys_sysconsaif_syscfg128(&self) -> &SYS_SYSCONSAIF_SYSCFG128 {
        &self.sys_sysconsaif_syscfg128
    }
    #[doc = "0x88 - SYS SYSCONSAIF SYSCFG 136"]
    #[inline(always)]
    pub const fn sys_sysconsaif_syscfg136(&self) -> &SYS_SYSCONSAIF_SYSCFG136 {
        &self.sys_sysconsaif_syscfg136
    }
    #[doc = "0x8c - SYS SYSCONSAIF SYSCFG 140"]
    #[inline(always)]
    pub const fn sys_sysconsaif_syscfg140(&self) -> &SYS_SYSCONSAIF_SYSCFG140 {
        &self.sys_sysconsaif_syscfg140
    }
    #[doc = "0x90 - SYS SYSCONSAIF SYSCFG 144"]
    #[inline(always)]
    pub const fn sys_sysconsaif_syscfg144(&self) -> &SYS_SYSCONSAIF_SYSCFG144 {
        &self.sys_sysconsaif_syscfg144
    }
    #[doc = "0x94 - SYS SYSCONSAIF SYSCFG 148"]
    #[inline(always)]
    pub const fn sys_sysconsaif_syscfg148(&self) -> &SYS_SYSCONSAIF_SYSCFG148 {
        &self.sys_sysconsaif_syscfg148
    }
    #[doc = "0x98 - SYS SYSCONSAIF SYSCFG 152"]
    #[inline(always)]
    pub const fn sys_sysconsaif_syscfg152(&self) -> &SYS_SYSCONSAIF_SYSCFG152 {
        &self.sys_sysconsaif_syscfg152
    }
    #[doc = "0x9c - SYS SYSCONSAIF SYSCFG 156"]
    #[inline(always)]
    pub const fn sys_sysconsaif_syscfg156(&self) -> &SYS_SYSCONSAIF_SYSCFG156 {
        &self.sys_sysconsaif_syscfg156
    }
}
#[doc = "sys_sysconsaif_syscfg0 (rw) register accessor: SYS SYSCONSAIF SYSCFG 0\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`sys_sysconsaif_syscfg0::R`].  You can [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`sys_sysconsaif_syscfg0::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@sys_sysconsaif_syscfg0`]
module"]
pub type SYS_SYSCONSAIF_SYSCFG0 = crate::Reg<sys_sysconsaif_syscfg0::SYS_SYSCONSAIF_SYSCFG0_SPEC>;
#[doc = "SYS SYSCONSAIF SYSCFG 0"]
pub mod sys_sysconsaif_syscfg0;
#[doc = "sys_sysconsaif_syscfg4 (rw) register accessor: SYS SYSCONSAIF SYSCFG 4\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`sys_sysconsaif_syscfg4::R`].  You can [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`sys_sysconsaif_syscfg4::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@sys_sysconsaif_syscfg4`]
module"]
pub type SYS_SYSCONSAIF_SYSCFG4 = crate::Reg<sys_sysconsaif_syscfg4::SYS_SYSCONSAIF_SYSCFG4_SPEC>;
#[doc = "SYS SYSCONSAIF SYSCFG 4"]
pub mod sys_sysconsaif_syscfg4;
#[doc = "sys_sysconsaif_syscfg8 (rw) register accessor: SYS SYSCONSAIF SYSCFG 8\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`sys_sysconsaif_syscfg8::R`].  You can [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`sys_sysconsaif_syscfg8::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@sys_sysconsaif_syscfg8`]
module"]
pub type SYS_SYSCONSAIF_SYSCFG8 = crate::Reg<sys_sysconsaif_syscfg8::SYS_SYSCONSAIF_SYSCFG8_SPEC>;
#[doc = "SYS SYSCONSAIF SYSCFG 8"]
pub mod sys_sysconsaif_syscfg8;
#[doc = "sys_sysconsaif_syscfg12 (rw) register accessor: SYS SYSCONSAIF SYSCFG 12: Set the GPIO voltage of all the 4 GPIO groups in this register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`sys_sysconsaif_syscfg12::R`].  You can [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`sys_sysconsaif_syscfg12::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@sys_sysconsaif_syscfg12`]
module"]
pub type SYS_SYSCONSAIF_SYSCFG12 =
    crate::Reg<sys_sysconsaif_syscfg12::SYS_SYSCONSAIF_SYSCFG12_SPEC>;
#[doc = "SYS SYSCONSAIF SYSCFG 12: Set the GPIO voltage of all the 4 GPIO groups in this register"]
pub mod sys_sysconsaif_syscfg12;
#[doc = "sys_sysconsaif_syscfg16 (rw) register accessor: SYS SYSCONSAIF SYSCFG 16\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`sys_sysconsaif_syscfg16::R`].  You can [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`sys_sysconsaif_syscfg16::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@sys_sysconsaif_syscfg16`]
module"]
pub type SYS_SYSCONSAIF_SYSCFG16 =
    crate::Reg<sys_sysconsaif_syscfg16::SYS_SYSCONSAIF_SYSCFG16_SPEC>;
#[doc = "SYS SYSCONSAIF SYSCFG 16"]
pub mod sys_sysconsaif_syscfg16;
#[doc = "sys_sysconsaif_syscfg20 (rw) register accessor: SYS SYSCONSAIF SYSCFG 20\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`sys_sysconsaif_syscfg20::R`].  You can [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`sys_sysconsaif_syscfg20::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@sys_sysconsaif_syscfg20`]
module"]
pub type SYS_SYSCONSAIF_SYSCFG20 =
    crate::Reg<sys_sysconsaif_syscfg20::SYS_SYSCONSAIF_SYSCFG20_SPEC>;
#[doc = "SYS SYSCONSAIF SYSCFG 20"]
pub mod sys_sysconsaif_syscfg20;
#[doc = "sys_sysconsaif_syscfg24 (rw) register accessor: SYS SYSCONSAIF SYSCFG 24\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`sys_sysconsaif_syscfg24::R`].  You can [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`sys_sysconsaif_syscfg24::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@sys_sysconsaif_syscfg24`]
module"]
pub type SYS_SYSCONSAIF_SYSCFG24 =
    crate::Reg<sys_sysconsaif_syscfg24::SYS_SYSCONSAIF_SYSCFG24_SPEC>;
#[doc = "SYS SYSCONSAIF SYSCFG 24"]
pub mod sys_sysconsaif_syscfg24;
#[doc = "sys_sysconsaif_syscfg28 (rw) register accessor: SYS SYSCONSAIF SYSCFG 28\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`sys_sysconsaif_syscfg28::R`].  You can [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`sys_sysconsaif_syscfg28::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@sys_sysconsaif_syscfg28`]
module"]
pub type SYS_SYSCONSAIF_SYSCFG28 =
    crate::Reg<sys_sysconsaif_syscfg28::SYS_SYSCONSAIF_SYSCFG28_SPEC>;
#[doc = "SYS SYSCONSAIF SYSCFG 28"]
pub mod sys_sysconsaif_syscfg28;
#[doc = "sys_sysconsaif_syscfg32 (rw) register accessor: SYS SYSCONSAIF SYSCFG 32\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`sys_sysconsaif_syscfg32::R`].  You can [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`sys_sysconsaif_syscfg32::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@sys_sysconsaif_syscfg32`]
module"]
pub type SYS_SYSCONSAIF_SYSCFG32 =
    crate::Reg<sys_sysconsaif_syscfg32::SYS_SYSCONSAIF_SYSCFG32_SPEC>;
#[doc = "SYS SYSCONSAIF SYSCFG 32"]
pub mod sys_sysconsaif_syscfg32;
#[doc = "sys_sysconsaif_syscfg36 (rw) register accessor: SYS SYSCONSAIF SYSCFG 36\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`sys_sysconsaif_syscfg36::R`].  You can [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`sys_sysconsaif_syscfg36::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@sys_sysconsaif_syscfg36`]
module"]
pub type SYS_SYSCONSAIF_SYSCFG36 =
    crate::Reg<sys_sysconsaif_syscfg36::SYS_SYSCONSAIF_SYSCFG36_SPEC>;
#[doc = "SYS SYSCONSAIF SYSCFG 36"]
pub mod sys_sysconsaif_syscfg36;
#[doc = "sys_sysconsaif_syscfg40 (rw) register accessor: SYS SYSCONSAIF SYSCFG 40\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`sys_sysconsaif_syscfg40::R`].  You can [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`sys_sysconsaif_syscfg40::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@sys_sysconsaif_syscfg40`]
module"]
pub type SYS_SYSCONSAIF_SYSCFG40 =
    crate::Reg<sys_sysconsaif_syscfg40::SYS_SYSCONSAIF_SYSCFG40_SPEC>;
#[doc = "SYS SYSCONSAIF SYSCFG 40"]
pub mod sys_sysconsaif_syscfg40;
#[doc = "sys_sysconsaif_syscfg44 (rw) register accessor: SYS SYSCONSAIF SYSCFG 44\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`sys_sysconsaif_syscfg44::R`].  You can [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`sys_sysconsaif_syscfg44::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@sys_sysconsaif_syscfg44`]
module"]
pub type SYS_SYSCONSAIF_SYSCFG44 =
    crate::Reg<sys_sysconsaif_syscfg44::SYS_SYSCONSAIF_SYSCFG44_SPEC>;
#[doc = "SYS SYSCONSAIF SYSCFG 44"]
pub mod sys_sysconsaif_syscfg44;
#[doc = "sys_sysconsaif_syscfg48 (rw) register accessor: SYS SYSCONSAIF SYSCFG 48\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`sys_sysconsaif_syscfg48::R`].  You can [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`sys_sysconsaif_syscfg48::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@sys_sysconsaif_syscfg48`]
module"]
pub type SYS_SYSCONSAIF_SYSCFG48 =
    crate::Reg<sys_sysconsaif_syscfg48::SYS_SYSCONSAIF_SYSCFG48_SPEC>;
#[doc = "SYS SYSCONSAIF SYSCFG 48"]
pub mod sys_sysconsaif_syscfg48;
#[doc = "sys_sysconsaif_syscfg52 (rw) register accessor: SYS SYSCONSAIF SYSCFG 52\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`sys_sysconsaif_syscfg52::R`].  You can [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`sys_sysconsaif_syscfg52::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@sys_sysconsaif_syscfg52`]
module"]
pub type SYS_SYSCONSAIF_SYSCFG52 =
    crate::Reg<sys_sysconsaif_syscfg52::SYS_SYSCONSAIF_SYSCFG52_SPEC>;
#[doc = "SYS SYSCONSAIF SYSCFG 52"]
pub mod sys_sysconsaif_syscfg52;
#[doc = "sys_sysconsaif_syscfg56 (rw) register accessor: SYS SYSCONSAIF SYSCFG 56\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`sys_sysconsaif_syscfg56::R`].  You can [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`sys_sysconsaif_syscfg56::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@sys_sysconsaif_syscfg56`]
module"]
pub type SYS_SYSCONSAIF_SYSCFG56 =
    crate::Reg<sys_sysconsaif_syscfg56::SYS_SYSCONSAIF_SYSCFG56_SPEC>;
#[doc = "SYS SYSCONSAIF SYSCFG 56"]
pub mod sys_sysconsaif_syscfg56;
#[doc = "sys_sysconsaif_syscfg60 (rw) register accessor: SYS SYSCONSAIF SYSCFG 60\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`sys_sysconsaif_syscfg60::R`].  You can [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`sys_sysconsaif_syscfg60::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@sys_sysconsaif_syscfg60`]
module"]
pub type SYS_SYSCONSAIF_SYSCFG60 =
    crate::Reg<sys_sysconsaif_syscfg60::SYS_SYSCONSAIF_SYSCFG60_SPEC>;
#[doc = "SYS SYSCONSAIF SYSCFG 60"]
pub mod sys_sysconsaif_syscfg60;
#[doc = "sys_sysconsaif_syscfg64 (rw) register accessor: SYS SYSCONSAIF SYSCFG 64\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`sys_sysconsaif_syscfg64::R`].  You can [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`sys_sysconsaif_syscfg64::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@sys_sysconsaif_syscfg64`]
module"]
pub type SYS_SYSCONSAIF_SYSCFG64 =
    crate::Reg<sys_sysconsaif_syscfg64::SYS_SYSCONSAIF_SYSCFG64_SPEC>;
#[doc = "SYS SYSCONSAIF SYSCFG 64"]
pub mod sys_sysconsaif_syscfg64;
#[doc = "sys_sysconsaif_syscfg68 (rw) register accessor: SYS SYSCONSAIF SYSCFG 68\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`sys_sysconsaif_syscfg68::R`].  You can [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`sys_sysconsaif_syscfg68::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@sys_sysconsaif_syscfg68`]
module"]
pub type SYS_SYSCONSAIF_SYSCFG68 =
    crate::Reg<sys_sysconsaif_syscfg68::SYS_SYSCONSAIF_SYSCFG68_SPEC>;
#[doc = "SYS SYSCONSAIF SYSCFG 68"]
pub mod sys_sysconsaif_syscfg68;
#[doc = "sys_sysconsaif_syscfg72 (rw) register accessor: SYS SYSCONSAIF SYSCFG 72\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`sys_sysconsaif_syscfg72::R`].  You can [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`sys_sysconsaif_syscfg72::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@sys_sysconsaif_syscfg72`]
module"]
pub type SYS_SYSCONSAIF_SYSCFG72 =
    crate::Reg<sys_sysconsaif_syscfg72::SYS_SYSCONSAIF_SYSCFG72_SPEC>;
#[doc = "SYS SYSCONSAIF SYSCFG 72"]
pub mod sys_sysconsaif_syscfg72;
#[doc = "sys_sysconsaif_syscfg76 (rw) register accessor: SYS SYSCONSAIF SYSCFG 76\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`sys_sysconsaif_syscfg76::R`].  You can [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`sys_sysconsaif_syscfg76::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@sys_sysconsaif_syscfg76`]
module"]
pub type SYS_SYSCONSAIF_SYSCFG76 =
    crate::Reg<sys_sysconsaif_syscfg76::SYS_SYSCONSAIF_SYSCFG76_SPEC>;
#[doc = "SYS SYSCONSAIF SYSCFG 76"]
pub mod sys_sysconsaif_syscfg76;
#[doc = "sys_sysconsaif_syscfg80 (rw) register accessor: SYS SYSCONSAIF SYSCFG 80\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`sys_sysconsaif_syscfg80::R`].  You can [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`sys_sysconsaif_syscfg80::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@sys_sysconsaif_syscfg80`]
module"]
pub type SYS_SYSCONSAIF_SYSCFG80 =
    crate::Reg<sys_sysconsaif_syscfg80::SYS_SYSCONSAIF_SYSCFG80_SPEC>;
#[doc = "SYS SYSCONSAIF SYSCFG 80"]
pub mod sys_sysconsaif_syscfg80;
#[doc = "sys_sysconsaif_syscfg84 (rw) register accessor: SYS SYSCONSAIF SYSCFG 84\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`sys_sysconsaif_syscfg84::R`].  You can [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`sys_sysconsaif_syscfg84::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@sys_sysconsaif_syscfg84`]
module"]
pub type SYS_SYSCONSAIF_SYSCFG84 =
    crate::Reg<sys_sysconsaif_syscfg84::SYS_SYSCONSAIF_SYSCFG84_SPEC>;
#[doc = "SYS SYSCONSAIF SYSCFG 84"]
pub mod sys_sysconsaif_syscfg84;
#[doc = "sys_sysconsaif_syscfg88 (rw) register accessor: SYS SYSCONSAIF SYSCFG 88\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`sys_sysconsaif_syscfg88::R`].  You can [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`sys_sysconsaif_syscfg88::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@sys_sysconsaif_syscfg88`]
module"]
pub type SYS_SYSCONSAIF_SYSCFG88 =
    crate::Reg<sys_sysconsaif_syscfg88::SYS_SYSCONSAIF_SYSCFG88_SPEC>;
#[doc = "SYS SYSCONSAIF SYSCFG 88"]
pub mod sys_sysconsaif_syscfg88;
#[doc = "sys_sysconsaif_syscfg92 (rw) register accessor: SYS SYSCONSAIF SYSCFG 92\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`sys_sysconsaif_syscfg92::R`].  You can [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`sys_sysconsaif_syscfg92::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@sys_sysconsaif_syscfg92`]
module"]
pub type SYS_SYSCONSAIF_SYSCFG92 =
    crate::Reg<sys_sysconsaif_syscfg92::SYS_SYSCONSAIF_SYSCFG92_SPEC>;
#[doc = "SYS SYSCONSAIF SYSCFG 92"]
pub mod sys_sysconsaif_syscfg92;
#[doc = "sys_sysconsaif_syscfg96 (rw) register accessor: SYS SYSCONSAIF SYSCFG 96\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`sys_sysconsaif_syscfg96::R`].  You can [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`sys_sysconsaif_syscfg96::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@sys_sysconsaif_syscfg96`]
module"]
pub type SYS_SYSCONSAIF_SYSCFG96 =
    crate::Reg<sys_sysconsaif_syscfg96::SYS_SYSCONSAIF_SYSCFG96_SPEC>;
#[doc = "SYS SYSCONSAIF SYSCFG 96"]
pub mod sys_sysconsaif_syscfg96;
#[doc = "sys_sysconsaif_syscfg100 (rw) register accessor: SYS SYSCONSAIF SYSCFG 100\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`sys_sysconsaif_syscfg100::R`].  You can [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`sys_sysconsaif_syscfg100::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@sys_sysconsaif_syscfg100`]
module"]
pub type SYS_SYSCONSAIF_SYSCFG100 =
    crate::Reg<sys_sysconsaif_syscfg100::SYS_SYSCONSAIF_SYSCFG100_SPEC>;
#[doc = "SYS SYSCONSAIF SYSCFG 100"]
pub mod sys_sysconsaif_syscfg100;
#[doc = "sys_sysconsaif_syscfg104 (rw) register accessor: SYS SYSCONSAIF SYSCFG 104\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`sys_sysconsaif_syscfg104::R`].  You can [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`sys_sysconsaif_syscfg104::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@sys_sysconsaif_syscfg104`]
module"]
pub type SYS_SYSCONSAIF_SYSCFG104 =
    crate::Reg<sys_sysconsaif_syscfg104::SYS_SYSCONSAIF_SYSCFG104_SPEC>;
#[doc = "SYS SYSCONSAIF SYSCFG 104"]
pub mod sys_sysconsaif_syscfg104;
#[doc = "sys_sysconsaif_syscfg108 (rw) register accessor: SYS SYSCONSAIF SYSCFG 108\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`sys_sysconsaif_syscfg108::R`].  You can [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`sys_sysconsaif_syscfg108::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@sys_sysconsaif_syscfg108`]
module"]
pub type SYS_SYSCONSAIF_SYSCFG108 =
    crate::Reg<sys_sysconsaif_syscfg108::SYS_SYSCONSAIF_SYSCFG108_SPEC>;
#[doc = "SYS SYSCONSAIF SYSCFG 108"]
pub mod sys_sysconsaif_syscfg108;
#[doc = "sys_sysconsaif_syscfg112 (rw) register accessor: SYS SYSCONSAIF SYSCFG 112\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`sys_sysconsaif_syscfg112::R`].  You can [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`sys_sysconsaif_syscfg112::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@sys_sysconsaif_syscfg112`]
module"]
pub type SYS_SYSCONSAIF_SYSCFG112 =
    crate::Reg<sys_sysconsaif_syscfg112::SYS_SYSCONSAIF_SYSCFG112_SPEC>;
#[doc = "SYS SYSCONSAIF SYSCFG 112"]
pub mod sys_sysconsaif_syscfg112;
#[doc = "sys_sysconsaif_syscfg116 (rw) register accessor: SYS SYSCONSAIF SYSCFG 116\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`sys_sysconsaif_syscfg116::R`].  You can [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`sys_sysconsaif_syscfg116::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@sys_sysconsaif_syscfg116`]
module"]
pub type SYS_SYSCONSAIF_SYSCFG116 =
    crate::Reg<sys_sysconsaif_syscfg116::SYS_SYSCONSAIF_SYSCFG116_SPEC>;
#[doc = "SYS SYSCONSAIF SYSCFG 116"]
pub mod sys_sysconsaif_syscfg116;
#[doc = "sys_sysconsaif_syscfg120 (rw) register accessor: SYS SYSCONSAIF SYSCFG 120\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`sys_sysconsaif_syscfg120::R`].  You can [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`sys_sysconsaif_syscfg120::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@sys_sysconsaif_syscfg120`]
module"]
pub type SYS_SYSCONSAIF_SYSCFG120 =
    crate::Reg<sys_sysconsaif_syscfg120::SYS_SYSCONSAIF_SYSCFG120_SPEC>;
#[doc = "SYS SYSCONSAIF SYSCFG 120"]
pub mod sys_sysconsaif_syscfg120;
#[doc = "sys_sysconsaif_syscfg124 (rw) register accessor: SYS SYSCONSAIF SYSCFG 124\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`sys_sysconsaif_syscfg124::R`].  You can [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`sys_sysconsaif_syscfg124::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@sys_sysconsaif_syscfg124`]
module"]
pub type SYS_SYSCONSAIF_SYSCFG124 =
    crate::Reg<sys_sysconsaif_syscfg124::SYS_SYSCONSAIF_SYSCFG124_SPEC>;
#[doc = "SYS SYSCONSAIF SYSCFG 124"]
pub mod sys_sysconsaif_syscfg124;
#[doc = "sys_sysconsaif_syscfg128 (rw) register accessor: SYS SYSCONSAIF SYSCFG 128\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`sys_sysconsaif_syscfg128::R`].  You can [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`sys_sysconsaif_syscfg128::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@sys_sysconsaif_syscfg128`]
module"]
pub type SYS_SYSCONSAIF_SYSCFG128 =
    crate::Reg<sys_sysconsaif_syscfg128::SYS_SYSCONSAIF_SYSCFG128_SPEC>;
#[doc = "SYS SYSCONSAIF SYSCFG 128"]
pub mod sys_sysconsaif_syscfg128;
#[doc = "sys_sysconsaif_syscfg136 (rw) register accessor: SYS SYSCONSAIF SYSCFG 136\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`sys_sysconsaif_syscfg136::R`].  You can [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`sys_sysconsaif_syscfg136::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@sys_sysconsaif_syscfg136`]
module"]
pub type SYS_SYSCONSAIF_SYSCFG136 =
    crate::Reg<sys_sysconsaif_syscfg136::SYS_SYSCONSAIF_SYSCFG136_SPEC>;
#[doc = "SYS SYSCONSAIF SYSCFG 136"]
pub mod sys_sysconsaif_syscfg136;
#[doc = "sys_sysconsaif_syscfg140 (rw) register accessor: SYS SYSCONSAIF SYSCFG 140\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`sys_sysconsaif_syscfg140::R`].  You can [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`sys_sysconsaif_syscfg140::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@sys_sysconsaif_syscfg140`]
module"]
pub type SYS_SYSCONSAIF_SYSCFG140 =
    crate::Reg<sys_sysconsaif_syscfg140::SYS_SYSCONSAIF_SYSCFG140_SPEC>;
#[doc = "SYS SYSCONSAIF SYSCFG 140"]
pub mod sys_sysconsaif_syscfg140;
#[doc = "sys_sysconsaif_syscfg144 (rw) register accessor: SYS SYSCONSAIF SYSCFG 144\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`sys_sysconsaif_syscfg144::R`].  You can [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`sys_sysconsaif_syscfg144::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@sys_sysconsaif_syscfg144`]
module"]
pub type SYS_SYSCONSAIF_SYSCFG144 =
    crate::Reg<sys_sysconsaif_syscfg144::SYS_SYSCONSAIF_SYSCFG144_SPEC>;
#[doc = "SYS SYSCONSAIF SYSCFG 144"]
pub mod sys_sysconsaif_syscfg144;
#[doc = "sys_sysconsaif_syscfg148 (rw) register accessor: SYS SYSCONSAIF SYSCFG 148\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`sys_sysconsaif_syscfg148::R`].  You can [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`sys_sysconsaif_syscfg148::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@sys_sysconsaif_syscfg148`]
module"]
pub type SYS_SYSCONSAIF_SYSCFG148 =
    crate::Reg<sys_sysconsaif_syscfg148::SYS_SYSCONSAIF_SYSCFG148_SPEC>;
#[doc = "SYS SYSCONSAIF SYSCFG 148"]
pub mod sys_sysconsaif_syscfg148;
#[doc = "sys_sysconsaif_syscfg152 (rw) register accessor: SYS SYSCONSAIF SYSCFG 152\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`sys_sysconsaif_syscfg152::R`].  You can [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`sys_sysconsaif_syscfg152::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@sys_sysconsaif_syscfg152`]
module"]
pub type SYS_SYSCONSAIF_SYSCFG152 =
    crate::Reg<sys_sysconsaif_syscfg152::SYS_SYSCONSAIF_SYSCFG152_SPEC>;
#[doc = "SYS SYSCONSAIF SYSCFG 152"]
pub mod sys_sysconsaif_syscfg152;
#[doc = "sys_sysconsaif_syscfg156 (rw) register accessor: SYS SYSCONSAIF SYSCFG 156\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`sys_sysconsaif_syscfg156::R`].  You can [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`sys_sysconsaif_syscfg156::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@sys_sysconsaif_syscfg156`]
module"]
pub type SYS_SYSCONSAIF_SYSCFG156 =
    crate::Reg<sys_sysconsaif_syscfg156::SYS_SYSCONSAIF_SYSCFG156_SPEC>;
#[doc = "SYS SYSCONSAIF SYSCFG 156"]
pub mod sys_sysconsaif_syscfg156;
