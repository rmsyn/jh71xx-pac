#[doc = r"Register block"]
#[repr(C)]
pub struct RegisterBlock {
    aon_syscfg_0: AON_SYSCFG_0,
    aon_syscfg_1: AON_SYSCFG_1,
    aon_syscfg_2: AON_SYSCFG_2,
    aon_syscfg_3: AON_SYSCFG_3,
    aon_syscfg_4: AON_SYSCFG_4,
    aon_syscfg_5: AON_SYSCFG_5,
    aon_syscfg_6: AON_SYSCFG_6,
    aon_syscfg_7: AON_SYSCFG_7,
    aon_syscfg_8: AON_SYSCFG_8,
    aon_syscfg_9: AON_SYSCFG_9,
    aon_syscfg_10: AON_SYSCFG_10,
}
impl RegisterBlock {
    #[doc = "0x00 - AON SYSCONSAIF SYSCFG 0"]
    #[inline(always)]
    pub const fn aon_syscfg_0(&self) -> &AON_SYSCFG_0 {
        &self.aon_syscfg_0
    }
    #[doc = "0x04 - AON SYSCONSAIF SYSCFG 4"]
    #[inline(always)]
    pub const fn aon_syscfg_1(&self) -> &AON_SYSCFG_1 {
        &self.aon_syscfg_1
    }
    #[doc = "0x08 - AON SYSCONSAIF SYSCFG 8"]
    #[inline(always)]
    pub const fn aon_syscfg_2(&self) -> &AON_SYSCFG_2 {
        &self.aon_syscfg_2
    }
    #[doc = "0x0c - AON SYSCONSAIF SYSCFG 12"]
    #[inline(always)]
    pub const fn aon_syscfg_3(&self) -> &AON_SYSCFG_3 {
        &self.aon_syscfg_3
    }
    #[doc = "0x10 - AON SYSCONSAIF SYSCFG 16"]
    #[inline(always)]
    pub const fn aon_syscfg_4(&self) -> &AON_SYSCFG_4 {
        &self.aon_syscfg_4
    }
    #[doc = "0x14 - AON SYSCONSAIF SYSCFG 20"]
    #[inline(always)]
    pub const fn aon_syscfg_5(&self) -> &AON_SYSCFG_5 {
        &self.aon_syscfg_5
    }
    #[doc = "0x18 - AON SYSCONSAIF SYSCFG 24"]
    #[inline(always)]
    pub const fn aon_syscfg_6(&self) -> &AON_SYSCFG_6 {
        &self.aon_syscfg_6
    }
    #[doc = "0x1c - AON SYSCONSAIF SYSCFG 28"]
    #[inline(always)]
    pub const fn aon_syscfg_7(&self) -> &AON_SYSCFG_7 {
        &self.aon_syscfg_7
    }
    #[doc = "0x20 - AON SYSCONSAIF SYSCFG 32"]
    #[inline(always)]
    pub const fn aon_syscfg_8(&self) -> &AON_SYSCFG_8 {
        &self.aon_syscfg_8
    }
    #[doc = "0x24 - AON SYSCONSAIF SYSCFG 36"]
    #[inline(always)]
    pub const fn aon_syscfg_9(&self) -> &AON_SYSCFG_9 {
        &self.aon_syscfg_9
    }
    #[doc = "0x28 - AON SYSCONSAIF SYSCFG 40"]
    #[inline(always)]
    pub const fn aon_syscfg_10(&self) -> &AON_SYSCFG_10 {
        &self.aon_syscfg_10
    }
}
#[doc = "aon_syscfg_0 (rw) register accessor: AON SYSCONSAIF SYSCFG 0\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`aon_syscfg_0::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`aon_syscfg_0::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@aon_syscfg_0`]
module"]
pub type AON_SYSCFG_0 = crate::Reg<aon_syscfg_0::AON_SYSCFG_0_SPEC>;
#[doc = "AON SYSCONSAIF SYSCFG 0"]
pub mod aon_syscfg_0;
#[doc = "aon_syscfg_1 (rw) register accessor: AON SYSCONSAIF SYSCFG 4\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`aon_syscfg_1::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`aon_syscfg_1::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@aon_syscfg_1`]
module"]
pub type AON_SYSCFG_1 = crate::Reg<aon_syscfg_1::AON_SYSCFG_1_SPEC>;
#[doc = "AON SYSCONSAIF SYSCFG 4"]
pub mod aon_syscfg_1;
#[doc = "aon_syscfg_2 (rw) register accessor: AON SYSCONSAIF SYSCFG 8\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`aon_syscfg_2::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`aon_syscfg_2::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@aon_syscfg_2`]
module"]
pub type AON_SYSCFG_2 = crate::Reg<aon_syscfg_2::AON_SYSCFG_2_SPEC>;
#[doc = "AON SYSCONSAIF SYSCFG 8"]
pub mod aon_syscfg_2;
#[doc = "aon_syscfg_3 (rw) register accessor: AON SYSCONSAIF SYSCFG 12\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`aon_syscfg_3::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`aon_syscfg_3::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@aon_syscfg_3`]
module"]
pub type AON_SYSCFG_3 = crate::Reg<aon_syscfg_3::AON_SYSCFG_3_SPEC>;
#[doc = "AON SYSCONSAIF SYSCFG 12"]
pub mod aon_syscfg_3;
#[doc = "aon_syscfg_4 (rw) register accessor: AON SYSCONSAIF SYSCFG 16\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`aon_syscfg_4::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`aon_syscfg_4::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@aon_syscfg_4`]
module"]
pub type AON_SYSCFG_4 = crate::Reg<aon_syscfg_4::AON_SYSCFG_4_SPEC>;
#[doc = "AON SYSCONSAIF SYSCFG 16"]
pub mod aon_syscfg_4;
#[doc = "aon_syscfg_5 (rw) register accessor: AON SYSCONSAIF SYSCFG 20\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`aon_syscfg_5::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`aon_syscfg_5::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@aon_syscfg_5`]
module"]
pub type AON_SYSCFG_5 = crate::Reg<aon_syscfg_5::AON_SYSCFG_5_SPEC>;
#[doc = "AON SYSCONSAIF SYSCFG 20"]
pub mod aon_syscfg_5;
#[doc = "aon_syscfg_6 (rw) register accessor: AON SYSCONSAIF SYSCFG 24\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`aon_syscfg_6::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`aon_syscfg_6::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@aon_syscfg_6`]
module"]
pub type AON_SYSCFG_6 = crate::Reg<aon_syscfg_6::AON_SYSCFG_6_SPEC>;
#[doc = "AON SYSCONSAIF SYSCFG 24"]
pub mod aon_syscfg_6;
#[doc = "aon_syscfg_7 (rw) register accessor: AON SYSCONSAIF SYSCFG 28\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`aon_syscfg_7::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`aon_syscfg_7::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@aon_syscfg_7`]
module"]
pub type AON_SYSCFG_7 = crate::Reg<aon_syscfg_7::AON_SYSCFG_7_SPEC>;
#[doc = "AON SYSCONSAIF SYSCFG 28"]
pub mod aon_syscfg_7;
#[doc = "aon_syscfg_8 (rw) register accessor: AON SYSCONSAIF SYSCFG 32\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`aon_syscfg_8::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`aon_syscfg_8::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@aon_syscfg_8`]
module"]
pub type AON_SYSCFG_8 = crate::Reg<aon_syscfg_8::AON_SYSCFG_8_SPEC>;
#[doc = "AON SYSCONSAIF SYSCFG 32"]
pub mod aon_syscfg_8;
#[doc = "aon_syscfg_9 (rw) register accessor: AON SYSCONSAIF SYSCFG 36\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`aon_syscfg_9::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`aon_syscfg_9::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@aon_syscfg_9`]
module"]
pub type AON_SYSCFG_9 = crate::Reg<aon_syscfg_9::AON_SYSCFG_9_SPEC>;
#[doc = "AON SYSCONSAIF SYSCFG 36"]
pub mod aon_syscfg_9;
#[doc = "aon_syscfg_10 (rw) register accessor: AON SYSCONSAIF SYSCFG 40\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`aon_syscfg_10::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`aon_syscfg_10::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@aon_syscfg_10`]
module"]
pub type AON_SYSCFG_10 = crate::Reg<aon_syscfg_10::AON_SYSCFG_10_SPEC>;
#[doc = "AON SYSCONSAIF SYSCFG 40"]
pub mod aon_syscfg_10;
