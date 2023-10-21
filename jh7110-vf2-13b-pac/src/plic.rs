#[doc = r"Register block"]
#[repr(C)]
pub struct RegisterBlock {
    _reserved0: [u8; 0x04],
    #[doc = "0x04 - PRIORITY Register for interrupt id 1"]
    pub priority_1: PRIORITY_1,
    #[doc = "0x08 - PRIORITY Register for interrupt id 2"]
    pub priority_2: PRIORITY_2,
    #[doc = "0x0c - PRIORITY Register for interrupt id 3"]
    pub priority_3: PRIORITY_3,
    #[doc = "0x10 - PRIORITY Register for interrupt id 4"]
    pub priority_4: PRIORITY_4,
    #[doc = "0x14 - PRIORITY Register for interrupt id 5"]
    pub priority_5: PRIORITY_5,
    #[doc = "0x18 - PRIORITY Register for interrupt id 6"]
    pub priority_6: PRIORITY_6,
    #[doc = "0x1c - PRIORITY Register for interrupt id 7"]
    pub priority_7: PRIORITY_7,
    #[doc = "0x20 - PRIORITY Register for interrupt id 8"]
    pub priority_8: PRIORITY_8,
    #[doc = "0x24 - PRIORITY Register for interrupt id 9"]
    pub priority_9: PRIORITY_9,
    #[doc = "0x28 - PRIORITY Register for interrupt id 10"]
    pub priority_10: PRIORITY_10,
    #[doc = "0x2c - PRIORITY Register for interrupt id 11"]
    pub priority_11: PRIORITY_11,
    #[doc = "0x30 - PRIORITY Register for interrupt id 12"]
    pub priority_12: PRIORITY_12,
    #[doc = "0x34 - PRIORITY Register for interrupt id 13"]
    pub priority_13: PRIORITY_13,
    #[doc = "0x38 - PRIORITY Register for interrupt id 14"]
    pub priority_14: PRIORITY_14,
    #[doc = "0x3c - PRIORITY Register for interrupt id 15"]
    pub priority_15: PRIORITY_15,
    #[doc = "0x40 - PRIORITY Register for interrupt id 16"]
    pub priority_16: PRIORITY_16,
    #[doc = "0x44 - PRIORITY Register for interrupt id 17"]
    pub priority_17: PRIORITY_17,
    #[doc = "0x48 - PRIORITY Register for interrupt id 18"]
    pub priority_18: PRIORITY_18,
    #[doc = "0x4c - PRIORITY Register for interrupt id 19"]
    pub priority_19: PRIORITY_19,
    #[doc = "0x50 - PRIORITY Register for interrupt id 20"]
    pub priority_20: PRIORITY_20,
    #[doc = "0x54 - PRIORITY Register for interrupt id 21"]
    pub priority_21: PRIORITY_21,
    #[doc = "0x58 - PRIORITY Register for interrupt id 22"]
    pub priority_22: PRIORITY_22,
    #[doc = "0x5c - PRIORITY Register for interrupt id 23"]
    pub priority_23: PRIORITY_23,
    #[doc = "0x60 - PRIORITY Register for interrupt id 24"]
    pub priority_24: PRIORITY_24,
    #[doc = "0x64 - PRIORITY Register for interrupt id 25"]
    pub priority_25: PRIORITY_25,
    #[doc = "0x68 - PRIORITY Register for interrupt id 26"]
    pub priority_26: PRIORITY_26,
    #[doc = "0x6c - PRIORITY Register for interrupt id 27"]
    pub priority_27: PRIORITY_27,
    #[doc = "0x70 - PRIORITY Register for interrupt id 28"]
    pub priority_28: PRIORITY_28,
    #[doc = "0x74 - PRIORITY Register for interrupt id 29"]
    pub priority_29: PRIORITY_29,
    #[doc = "0x78 - PRIORITY Register for interrupt id 30"]
    pub priority_30: PRIORITY_30,
    #[doc = "0x7c - PRIORITY Register for interrupt id 31"]
    pub priority_31: PRIORITY_31,
    #[doc = "0x80 - PRIORITY Register for interrupt id 32"]
    pub priority_32: PRIORITY_32,
    #[doc = "0x84 - PRIORITY Register for interrupt id 33"]
    pub priority_33: PRIORITY_33,
    #[doc = "0x88 - PRIORITY Register for interrupt id 34"]
    pub priority_34: PRIORITY_34,
    #[doc = "0x8c - PRIORITY Register for interrupt id 35"]
    pub priority_35: PRIORITY_35,
    #[doc = "0x90 - PRIORITY Register for interrupt id 36"]
    pub priority_36: PRIORITY_36,
    #[doc = "0x94 - PRIORITY Register for interrupt id 37"]
    pub priority_37: PRIORITY_37,
    #[doc = "0x98 - PRIORITY Register for interrupt id 38"]
    pub priority_38: PRIORITY_38,
    #[doc = "0x9c - PRIORITY Register for interrupt id 39"]
    pub priority_39: PRIORITY_39,
    #[doc = "0xa0 - PRIORITY Register for interrupt id 40"]
    pub priority_40: PRIORITY_40,
    #[doc = "0xa4 - PRIORITY Register for interrupt id 41"]
    pub priority_41: PRIORITY_41,
    #[doc = "0xa8 - PRIORITY Register for interrupt id 42"]
    pub priority_42: PRIORITY_42,
    #[doc = "0xac - PRIORITY Register for interrupt id 43"]
    pub priority_43: PRIORITY_43,
    #[doc = "0xb0 - PRIORITY Register for interrupt id 44"]
    pub priority_44: PRIORITY_44,
    #[doc = "0xb4 - PRIORITY Register for interrupt id 45"]
    pub priority_45: PRIORITY_45,
    #[doc = "0xb8 - PRIORITY Register for interrupt id 46"]
    pub priority_46: PRIORITY_46,
    #[doc = "0xbc - PRIORITY Register for interrupt id 47"]
    pub priority_47: PRIORITY_47,
    #[doc = "0xc0 - PRIORITY Register for interrupt id 48"]
    pub priority_48: PRIORITY_48,
    #[doc = "0xc4 - PRIORITY Register for interrupt id 49"]
    pub priority_49: PRIORITY_49,
    #[doc = "0xc8 - PRIORITY Register for interrupt id 50"]
    pub priority_50: PRIORITY_50,
    #[doc = "0xcc - PRIORITY Register for interrupt id 51"]
    pub priority_51: PRIORITY_51,
    #[doc = "0xd0 - PRIORITY Register for interrupt id 52"]
    pub priority_52: PRIORITY_52,
    #[doc = "0xd4 - PRIORITY Register for interrupt id 53"]
    pub priority_53: PRIORITY_53,
    #[doc = "0xd8 - PRIORITY Register for interrupt id 54"]
    pub priority_54: PRIORITY_54,
    #[doc = "0xdc - PRIORITY Register for interrupt id 55"]
    pub priority_55: PRIORITY_55,
    #[doc = "0xe0 - PRIORITY Register for interrupt id 56"]
    pub priority_56: PRIORITY_56,
    #[doc = "0xe4 - PRIORITY Register for interrupt id 57"]
    pub priority_57: PRIORITY_57,
    #[doc = "0xe8 - PRIORITY Register for interrupt id 58"]
    pub priority_58: PRIORITY_58,
    #[doc = "0xec - PRIORITY Register for interrupt id 59"]
    pub priority_59: PRIORITY_59,
    #[doc = "0xf0 - PRIORITY Register for interrupt id 60"]
    pub priority_60: PRIORITY_60,
    #[doc = "0xf4 - PRIORITY Register for interrupt id 61"]
    pub priority_61: PRIORITY_61,
    #[doc = "0xf8 - PRIORITY Register for interrupt id 62"]
    pub priority_62: PRIORITY_62,
    #[doc = "0xfc - PRIORITY Register for interrupt id 63"]
    pub priority_63: PRIORITY_63,
    #[doc = "0x100 - PRIORITY Register for interrupt id 64"]
    pub priority_64: PRIORITY_64,
    #[doc = "0x104 - PRIORITY Register for interrupt id 65"]
    pub priority_65: PRIORITY_65,
    #[doc = "0x108 - PRIORITY Register for interrupt id 66"]
    pub priority_66: PRIORITY_66,
    #[doc = "0x10c - PRIORITY Register for interrupt id 67"]
    pub priority_67: PRIORITY_67,
    #[doc = "0x110 - PRIORITY Register for interrupt id 68"]
    pub priority_68: PRIORITY_68,
    #[doc = "0x114 - PRIORITY Register for interrupt id 69"]
    pub priority_69: PRIORITY_69,
    #[doc = "0x118 - PRIORITY Register for interrupt id 70"]
    pub priority_70: PRIORITY_70,
    #[doc = "0x11c - PRIORITY Register for interrupt id 71"]
    pub priority_71: PRIORITY_71,
    #[doc = "0x120 - PRIORITY Register for interrupt id 72"]
    pub priority_72: PRIORITY_72,
    #[doc = "0x124 - PRIORITY Register for interrupt id 73"]
    pub priority_73: PRIORITY_73,
    #[doc = "0x128 - PRIORITY Register for interrupt id 74"]
    pub priority_74: PRIORITY_74,
    #[doc = "0x12c - PRIORITY Register for interrupt id 75"]
    pub priority_75: PRIORITY_75,
    #[doc = "0x130 - PRIORITY Register for interrupt id 76"]
    pub priority_76: PRIORITY_76,
    #[doc = "0x134 - PRIORITY Register for interrupt id 77"]
    pub priority_77: PRIORITY_77,
    #[doc = "0x138 - PRIORITY Register for interrupt id 78"]
    pub priority_78: PRIORITY_78,
    #[doc = "0x13c - PRIORITY Register for interrupt id 79"]
    pub priority_79: PRIORITY_79,
    #[doc = "0x140 - PRIORITY Register for interrupt id 80"]
    pub priority_80: PRIORITY_80,
    #[doc = "0x144 - PRIORITY Register for interrupt id 81"]
    pub priority_81: PRIORITY_81,
    #[doc = "0x148 - PRIORITY Register for interrupt id 82"]
    pub priority_82: PRIORITY_82,
    #[doc = "0x14c - PRIORITY Register for interrupt id 83"]
    pub priority_83: PRIORITY_83,
    #[doc = "0x150 - PRIORITY Register for interrupt id 84"]
    pub priority_84: PRIORITY_84,
    #[doc = "0x154 - PRIORITY Register for interrupt id 85"]
    pub priority_85: PRIORITY_85,
    #[doc = "0x158 - PRIORITY Register for interrupt id 86"]
    pub priority_86: PRIORITY_86,
    #[doc = "0x15c - PRIORITY Register for interrupt id 87"]
    pub priority_87: PRIORITY_87,
    #[doc = "0x160 - PRIORITY Register for interrupt id 88"]
    pub priority_88: PRIORITY_88,
    #[doc = "0x164 - PRIORITY Register for interrupt id 89"]
    pub priority_89: PRIORITY_89,
    #[doc = "0x168 - PRIORITY Register for interrupt id 90"]
    pub priority_90: PRIORITY_90,
    #[doc = "0x16c - PRIORITY Register for interrupt id 91"]
    pub priority_91: PRIORITY_91,
    #[doc = "0x170 - PRIORITY Register for interrupt id 92"]
    pub priority_92: PRIORITY_92,
    #[doc = "0x174 - PRIORITY Register for interrupt id 93"]
    pub priority_93: PRIORITY_93,
    #[doc = "0x178 - PRIORITY Register for interrupt id 94"]
    pub priority_94: PRIORITY_94,
    #[doc = "0x17c - PRIORITY Register for interrupt id 95"]
    pub priority_95: PRIORITY_95,
    #[doc = "0x180 - PRIORITY Register for interrupt id 96"]
    pub priority_96: PRIORITY_96,
    #[doc = "0x184 - PRIORITY Register for interrupt id 97"]
    pub priority_97: PRIORITY_97,
    #[doc = "0x188 - PRIORITY Register for interrupt id 98"]
    pub priority_98: PRIORITY_98,
    #[doc = "0x18c - PRIORITY Register for interrupt id 99"]
    pub priority_99: PRIORITY_99,
    #[doc = "0x190 - PRIORITY Register for interrupt id 100"]
    pub priority_100: PRIORITY_100,
    #[doc = "0x194 - PRIORITY Register for interrupt id 101"]
    pub priority_101: PRIORITY_101,
    #[doc = "0x198 - PRIORITY Register for interrupt id 102"]
    pub priority_102: PRIORITY_102,
    #[doc = "0x19c - PRIORITY Register for interrupt id 103"]
    pub priority_103: PRIORITY_103,
    #[doc = "0x1a0 - PRIORITY Register for interrupt id 104"]
    pub priority_104: PRIORITY_104,
    #[doc = "0x1a4 - PRIORITY Register for interrupt id 105"]
    pub priority_105: PRIORITY_105,
    #[doc = "0x1a8 - PRIORITY Register for interrupt id 106"]
    pub priority_106: PRIORITY_106,
    #[doc = "0x1ac - PRIORITY Register for interrupt id 107"]
    pub priority_107: PRIORITY_107,
    #[doc = "0x1b0 - PRIORITY Register for interrupt id 108"]
    pub priority_108: PRIORITY_108,
    #[doc = "0x1b4 - PRIORITY Register for interrupt id 109"]
    pub priority_109: PRIORITY_109,
    #[doc = "0x1b8 - PRIORITY Register for interrupt id 110"]
    pub priority_110: PRIORITY_110,
    #[doc = "0x1bc - PRIORITY Register for interrupt id 111"]
    pub priority_111: PRIORITY_111,
    #[doc = "0x1c0 - PRIORITY Register for interrupt id 112"]
    pub priority_112: PRIORITY_112,
    #[doc = "0x1c4 - PRIORITY Register for interrupt id 113"]
    pub priority_113: PRIORITY_113,
    #[doc = "0x1c8 - PRIORITY Register for interrupt id 114"]
    pub priority_114: PRIORITY_114,
    #[doc = "0x1cc - PRIORITY Register for interrupt id 115"]
    pub priority_115: PRIORITY_115,
    #[doc = "0x1d0 - PRIORITY Register for interrupt id 116"]
    pub priority_116: PRIORITY_116,
    #[doc = "0x1d4 - PRIORITY Register for interrupt id 117"]
    pub priority_117: PRIORITY_117,
    #[doc = "0x1d8 - PRIORITY Register for interrupt id 118"]
    pub priority_118: PRIORITY_118,
    #[doc = "0x1dc - PRIORITY Register for interrupt id 119"]
    pub priority_119: PRIORITY_119,
    #[doc = "0x1e0 - PRIORITY Register for interrupt id 120"]
    pub priority_120: PRIORITY_120,
    #[doc = "0x1e4 - PRIORITY Register for interrupt id 121"]
    pub priority_121: PRIORITY_121,
    #[doc = "0x1e8 - PRIORITY Register for interrupt id 122"]
    pub priority_122: PRIORITY_122,
    #[doc = "0x1ec - PRIORITY Register for interrupt id 123"]
    pub priority_123: PRIORITY_123,
    #[doc = "0x1f0 - PRIORITY Register for interrupt id 124"]
    pub priority_124: PRIORITY_124,
    #[doc = "0x1f4 - PRIORITY Register for interrupt id 125"]
    pub priority_125: PRIORITY_125,
    #[doc = "0x1f8 - PRIORITY Register for interrupt id 126"]
    pub priority_126: PRIORITY_126,
    #[doc = "0x1fc - PRIORITY Register for interrupt id 127"]
    pub priority_127: PRIORITY_127,
    #[doc = "0x200 - PRIORITY Register for interrupt id 128"]
    pub priority_128: PRIORITY_128,
    #[doc = "0x204 - PRIORITY Register for interrupt id 129"]
    pub priority_129: PRIORITY_129,
    #[doc = "0x208 - PRIORITY Register for interrupt id 130"]
    pub priority_130: PRIORITY_130,
    #[doc = "0x20c - PRIORITY Register for interrupt id 131"]
    pub priority_131: PRIORITY_131,
    #[doc = "0x210 - PRIORITY Register for interrupt id 132"]
    pub priority_132: PRIORITY_132,
    #[doc = "0x214 - PRIORITY Register for interrupt id 133"]
    pub priority_133: PRIORITY_133,
    #[doc = "0x218 - PRIORITY Register for interrupt id 134"]
    pub priority_134: PRIORITY_134,
    #[doc = "0x21c - PRIORITY Register for interrupt id 135"]
    pub priority_135: PRIORITY_135,
    #[doc = "0x220 - PRIORITY Register for interrupt id 136"]
    pub priority_136: PRIORITY_136,
    _reserved136: [u8; 0x0ddc],
    #[doc = "0x1000 - PENDING Register for interrupt ids 31 to 0"]
    pub pending_0: PENDING_0,
    #[doc = "0x1004 - PENDING Register for interrupt ids 63 to 32"]
    pub pending_1: PENDING_1,
    #[doc = "0x1008 - PENDING Register for interrupt ids 95 to 64"]
    pub pending_2: PENDING_2,
    #[doc = "0x100c - PENDING Register for interrupt ids 127 to 96"]
    pub pending_3: PENDING_3,
    #[doc = "0x1010 - PENDING Register for interrupt ids 136 to 128"]
    pub pending_4: PENDING_4,
    _reserved141: [u8; 0x0fec],
    #[doc = "0x2000 - ENABLE Register for interrupt ids 31 to 0 for hart 0"]
    pub enable_0_0: ENABLE_0_0,
    #[doc = "0x2004 - ENABLE Register for interrupt ids 63 to 32 for hart 0"]
    pub enable_1_0: ENABLE_1_0,
    #[doc = "0x2008 - ENABLE Register for interrupt ids 95 to 64 for hart 0"]
    pub enable_2_0: ENABLE_2_0,
    #[doc = "0x200c - ENABLE Register for interrupt ids 127 to 96 for hart 0"]
    pub enable_3_0: ENABLE_3_0,
    #[doc = "0x2010 - ENABLE Register for interrupt ids 136 to 128 for hart 0"]
    pub enable_4_0: ENABLE_4_0,
    _reserved146: [u8; 0x6c],
    #[doc = "0x2080 - ENABLE Register for interrupt ids 31 to 0 for hart 1"]
    pub enable_0_1: ENABLE_0_1,
    #[doc = "0x2084 - ENABLE Register for interrupt ids 63 to 32 for hart 1"]
    pub enable_1_1: ENABLE_1_1,
    #[doc = "0x2088 - ENABLE Register for interrupt ids 95 to 64 for hart 1"]
    pub enable_2_1: ENABLE_2_1,
    #[doc = "0x208c - ENABLE Register for interrupt ids 127 to 96 for hart 1"]
    pub enable_3_1: ENABLE_3_1,
    #[doc = "0x2090 - ENABLE Register for interrupt ids 136 to 128 for hart 1"]
    pub enable_4_1: ENABLE_4_1,
    _reserved151: [u8; 0x6c],
    #[doc = "0x2100 - ENABLE Register for interrupt ids 31 to 0 for hart 2"]
    pub enable_0_2: ENABLE_0_2,
    #[doc = "0x2104 - ENABLE Register for interrupt ids 63 to 32 for hart 2"]
    pub enable_1_2: ENABLE_1_2,
    #[doc = "0x2108 - ENABLE Register for interrupt ids 95 to 64 for hart 2"]
    pub enable_2_2: ENABLE_2_2,
    #[doc = "0x210c - ENABLE Register for interrupt ids 127 to 96 for hart 2"]
    pub enable_3_2: ENABLE_3_2,
    #[doc = "0x2110 - ENABLE Register for interrupt ids 136 to 128 for hart 2"]
    pub enable_4_2: ENABLE_4_2,
    _reserved156: [u8; 0x6c],
    #[doc = "0x2180 - ENABLE Register for interrupt ids 31 to 0 for hart 3"]
    pub enable_0_3: ENABLE_0_3,
    #[doc = "0x2184 - ENABLE Register for interrupt ids 63 to 32 for hart 3"]
    pub enable_1_3: ENABLE_1_3,
    #[doc = "0x2188 - ENABLE Register for interrupt ids 95 to 64 for hart 3"]
    pub enable_2_3: ENABLE_2_3,
    #[doc = "0x218c - ENABLE Register for interrupt ids 127 to 96 for hart 3"]
    pub enable_3_3: ENABLE_3_3,
    #[doc = "0x2190 - ENABLE Register for interrupt ids 136 to 128 for hart 3"]
    pub enable_4_3: ENABLE_4_3,
    _reserved161: [u8; 0x6c],
    #[doc = "0x2200 - ENABLE Register for interrupt ids 31 to 0 for hart 4"]
    pub enable_0_4: ENABLE_0_4,
    #[doc = "0x2204 - ENABLE Register for interrupt ids 63 to 32 for hart 4"]
    pub enable_1_4: ENABLE_1_4,
    #[doc = "0x2208 - ENABLE Register for interrupt ids 95 to 64 for hart 4"]
    pub enable_2_4: ENABLE_2_4,
    #[doc = "0x220c - ENABLE Register for interrupt ids 127 to 96 for hart 4"]
    pub enable_3_4: ENABLE_3_4,
    #[doc = "0x2210 - ENABLE Register for interrupt ids 136 to 128 for hart 4"]
    pub enable_4_4: ENABLE_4_4,
    _reserved166: [u8; 0x001f_ddec],
    #[doc = "0x200000 - PRIORITY THRESHOLD Register for hart 0"]
    pub threshold_0: THRESHOLD_0,
    #[doc = "0x200004 - CLAIM and COMPLETE Register for hart 0"]
    pub claimplete_0: CLAIMPLETE_0,
    _reserved168: [u8; 0x0ff8],
    #[doc = "0x201000 - PRIORITY THRESHOLD Register for hart 1"]
    pub threshold_1: THRESHOLD_1,
    #[doc = "0x201004 - CLAIM and COMPLETE Register for hart 1"]
    pub claimplete_1: CLAIMPLETE_1,
    _reserved170: [u8; 0x0ff8],
    #[doc = "0x202000 - PRIORITY THRESHOLD Register for hart 2"]
    pub threshold_2: THRESHOLD_2,
    #[doc = "0x202004 - CLAIM and COMPLETE Register for hart 2"]
    pub claimplete_2: CLAIMPLETE_2,
    _reserved172: [u8; 0x0ff8],
    #[doc = "0x203000 - PRIORITY THRESHOLD Register for hart 3"]
    pub threshold_3: THRESHOLD_3,
    #[doc = "0x203004 - CLAIM and COMPLETE Register for hart 3"]
    pub claimplete_3: CLAIMPLETE_3,
    _reserved174: [u8; 0x0ff8],
    #[doc = "0x204000 - PRIORITY THRESHOLD Register for hart 4"]
    pub threshold_4: THRESHOLD_4,
    #[doc = "0x204004 - CLAIM and COMPLETE Register for hart 4"]
    pub claimplete_4: CLAIMPLETE_4,
}
#[doc = "priority_1 (rw) register accessor: PRIORITY Register for interrupt id 1\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`priority_1::R`].  You can [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`priority_1::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`priority_1`]
module"]
pub type PRIORITY_1 = crate::Reg<priority_1::PRIORITY_1_SPEC>;
#[doc = "PRIORITY Register for interrupt id 1"]
pub mod priority_1;
#[doc = "priority_2 (rw) register accessor: PRIORITY Register for interrupt id 2\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`priority_2::R`].  You can [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`priority_2::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`priority_2`]
module"]
pub type PRIORITY_2 = crate::Reg<priority_2::PRIORITY_2_SPEC>;
#[doc = "PRIORITY Register for interrupt id 2"]
pub mod priority_2;
#[doc = "priority_3 (rw) register accessor: PRIORITY Register for interrupt id 3\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`priority_3::R`].  You can [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`priority_3::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`priority_3`]
module"]
pub type PRIORITY_3 = crate::Reg<priority_3::PRIORITY_3_SPEC>;
#[doc = "PRIORITY Register for interrupt id 3"]
pub mod priority_3;
#[doc = "priority_4 (rw) register accessor: PRIORITY Register for interrupt id 4\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`priority_4::R`].  You can [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`priority_4::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`priority_4`]
module"]
pub type PRIORITY_4 = crate::Reg<priority_4::PRIORITY_4_SPEC>;
#[doc = "PRIORITY Register for interrupt id 4"]
pub mod priority_4;
#[doc = "priority_5 (rw) register accessor: PRIORITY Register for interrupt id 5\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`priority_5::R`].  You can [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`priority_5::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`priority_5`]
module"]
pub type PRIORITY_5 = crate::Reg<priority_5::PRIORITY_5_SPEC>;
#[doc = "PRIORITY Register for interrupt id 5"]
pub mod priority_5;
#[doc = "priority_6 (rw) register accessor: PRIORITY Register for interrupt id 6\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`priority_6::R`].  You can [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`priority_6::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`priority_6`]
module"]
pub type PRIORITY_6 = crate::Reg<priority_6::PRIORITY_6_SPEC>;
#[doc = "PRIORITY Register for interrupt id 6"]
pub mod priority_6;
#[doc = "priority_7 (rw) register accessor: PRIORITY Register for interrupt id 7\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`priority_7::R`].  You can [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`priority_7::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`priority_7`]
module"]
pub type PRIORITY_7 = crate::Reg<priority_7::PRIORITY_7_SPEC>;
#[doc = "PRIORITY Register for interrupt id 7"]
pub mod priority_7;
#[doc = "priority_8 (rw) register accessor: PRIORITY Register for interrupt id 8\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`priority_8::R`].  You can [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`priority_8::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`priority_8`]
module"]
pub type PRIORITY_8 = crate::Reg<priority_8::PRIORITY_8_SPEC>;
#[doc = "PRIORITY Register for interrupt id 8"]
pub mod priority_8;
#[doc = "priority_9 (rw) register accessor: PRIORITY Register for interrupt id 9\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`priority_9::R`].  You can [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`priority_9::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`priority_9`]
module"]
pub type PRIORITY_9 = crate::Reg<priority_9::PRIORITY_9_SPEC>;
#[doc = "PRIORITY Register for interrupt id 9"]
pub mod priority_9;
#[doc = "priority_10 (rw) register accessor: PRIORITY Register for interrupt id 10\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`priority_10::R`].  You can [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`priority_10::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`priority_10`]
module"]
pub type PRIORITY_10 = crate::Reg<priority_10::PRIORITY_10_SPEC>;
#[doc = "PRIORITY Register for interrupt id 10"]
pub mod priority_10;
#[doc = "priority_11 (rw) register accessor: PRIORITY Register for interrupt id 11\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`priority_11::R`].  You can [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`priority_11::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`priority_11`]
module"]
pub type PRIORITY_11 = crate::Reg<priority_11::PRIORITY_11_SPEC>;
#[doc = "PRIORITY Register for interrupt id 11"]
pub mod priority_11;
#[doc = "priority_12 (rw) register accessor: PRIORITY Register for interrupt id 12\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`priority_12::R`].  You can [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`priority_12::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`priority_12`]
module"]
pub type PRIORITY_12 = crate::Reg<priority_12::PRIORITY_12_SPEC>;
#[doc = "PRIORITY Register for interrupt id 12"]
pub mod priority_12;
#[doc = "priority_13 (rw) register accessor: PRIORITY Register for interrupt id 13\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`priority_13::R`].  You can [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`priority_13::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`priority_13`]
module"]
pub type PRIORITY_13 = crate::Reg<priority_13::PRIORITY_13_SPEC>;
#[doc = "PRIORITY Register for interrupt id 13"]
pub mod priority_13;
#[doc = "priority_14 (rw) register accessor: PRIORITY Register for interrupt id 14\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`priority_14::R`].  You can [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`priority_14::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`priority_14`]
module"]
pub type PRIORITY_14 = crate::Reg<priority_14::PRIORITY_14_SPEC>;
#[doc = "PRIORITY Register for interrupt id 14"]
pub mod priority_14;
#[doc = "priority_15 (rw) register accessor: PRIORITY Register for interrupt id 15\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`priority_15::R`].  You can [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`priority_15::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`priority_15`]
module"]
pub type PRIORITY_15 = crate::Reg<priority_15::PRIORITY_15_SPEC>;
#[doc = "PRIORITY Register for interrupt id 15"]
pub mod priority_15;
#[doc = "priority_16 (rw) register accessor: PRIORITY Register for interrupt id 16\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`priority_16::R`].  You can [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`priority_16::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`priority_16`]
module"]
pub type PRIORITY_16 = crate::Reg<priority_16::PRIORITY_16_SPEC>;
#[doc = "PRIORITY Register for interrupt id 16"]
pub mod priority_16;
#[doc = "priority_17 (rw) register accessor: PRIORITY Register for interrupt id 17\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`priority_17::R`].  You can [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`priority_17::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`priority_17`]
module"]
pub type PRIORITY_17 = crate::Reg<priority_17::PRIORITY_17_SPEC>;
#[doc = "PRIORITY Register for interrupt id 17"]
pub mod priority_17;
#[doc = "priority_18 (rw) register accessor: PRIORITY Register for interrupt id 18\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`priority_18::R`].  You can [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`priority_18::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`priority_18`]
module"]
pub type PRIORITY_18 = crate::Reg<priority_18::PRIORITY_18_SPEC>;
#[doc = "PRIORITY Register for interrupt id 18"]
pub mod priority_18;
#[doc = "priority_19 (rw) register accessor: PRIORITY Register for interrupt id 19\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`priority_19::R`].  You can [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`priority_19::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`priority_19`]
module"]
pub type PRIORITY_19 = crate::Reg<priority_19::PRIORITY_19_SPEC>;
#[doc = "PRIORITY Register for interrupt id 19"]
pub mod priority_19;
#[doc = "priority_20 (rw) register accessor: PRIORITY Register for interrupt id 20\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`priority_20::R`].  You can [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`priority_20::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`priority_20`]
module"]
pub type PRIORITY_20 = crate::Reg<priority_20::PRIORITY_20_SPEC>;
#[doc = "PRIORITY Register for interrupt id 20"]
pub mod priority_20;
#[doc = "priority_21 (rw) register accessor: PRIORITY Register for interrupt id 21\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`priority_21::R`].  You can [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`priority_21::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`priority_21`]
module"]
pub type PRIORITY_21 = crate::Reg<priority_21::PRIORITY_21_SPEC>;
#[doc = "PRIORITY Register for interrupt id 21"]
pub mod priority_21;
#[doc = "priority_22 (rw) register accessor: PRIORITY Register for interrupt id 22\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`priority_22::R`].  You can [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`priority_22::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`priority_22`]
module"]
pub type PRIORITY_22 = crate::Reg<priority_22::PRIORITY_22_SPEC>;
#[doc = "PRIORITY Register for interrupt id 22"]
pub mod priority_22;
#[doc = "priority_23 (rw) register accessor: PRIORITY Register for interrupt id 23\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`priority_23::R`].  You can [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`priority_23::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`priority_23`]
module"]
pub type PRIORITY_23 = crate::Reg<priority_23::PRIORITY_23_SPEC>;
#[doc = "PRIORITY Register for interrupt id 23"]
pub mod priority_23;
#[doc = "priority_24 (rw) register accessor: PRIORITY Register for interrupt id 24\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`priority_24::R`].  You can [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`priority_24::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`priority_24`]
module"]
pub type PRIORITY_24 = crate::Reg<priority_24::PRIORITY_24_SPEC>;
#[doc = "PRIORITY Register for interrupt id 24"]
pub mod priority_24;
#[doc = "priority_25 (rw) register accessor: PRIORITY Register for interrupt id 25\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`priority_25::R`].  You can [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`priority_25::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`priority_25`]
module"]
pub type PRIORITY_25 = crate::Reg<priority_25::PRIORITY_25_SPEC>;
#[doc = "PRIORITY Register for interrupt id 25"]
pub mod priority_25;
#[doc = "priority_26 (rw) register accessor: PRIORITY Register for interrupt id 26\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`priority_26::R`].  You can [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`priority_26::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`priority_26`]
module"]
pub type PRIORITY_26 = crate::Reg<priority_26::PRIORITY_26_SPEC>;
#[doc = "PRIORITY Register for interrupt id 26"]
pub mod priority_26;
#[doc = "priority_27 (rw) register accessor: PRIORITY Register for interrupt id 27\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`priority_27::R`].  You can [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`priority_27::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`priority_27`]
module"]
pub type PRIORITY_27 = crate::Reg<priority_27::PRIORITY_27_SPEC>;
#[doc = "PRIORITY Register for interrupt id 27"]
pub mod priority_27;
#[doc = "priority_28 (rw) register accessor: PRIORITY Register for interrupt id 28\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`priority_28::R`].  You can [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`priority_28::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`priority_28`]
module"]
pub type PRIORITY_28 = crate::Reg<priority_28::PRIORITY_28_SPEC>;
#[doc = "PRIORITY Register for interrupt id 28"]
pub mod priority_28;
#[doc = "priority_29 (rw) register accessor: PRIORITY Register for interrupt id 29\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`priority_29::R`].  You can [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`priority_29::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`priority_29`]
module"]
pub type PRIORITY_29 = crate::Reg<priority_29::PRIORITY_29_SPEC>;
#[doc = "PRIORITY Register for interrupt id 29"]
pub mod priority_29;
#[doc = "priority_30 (rw) register accessor: PRIORITY Register for interrupt id 30\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`priority_30::R`].  You can [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`priority_30::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`priority_30`]
module"]
pub type PRIORITY_30 = crate::Reg<priority_30::PRIORITY_30_SPEC>;
#[doc = "PRIORITY Register for interrupt id 30"]
pub mod priority_30;
#[doc = "priority_31 (rw) register accessor: PRIORITY Register for interrupt id 31\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`priority_31::R`].  You can [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`priority_31::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`priority_31`]
module"]
pub type PRIORITY_31 = crate::Reg<priority_31::PRIORITY_31_SPEC>;
#[doc = "PRIORITY Register for interrupt id 31"]
pub mod priority_31;
#[doc = "priority_32 (rw) register accessor: PRIORITY Register for interrupt id 32\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`priority_32::R`].  You can [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`priority_32::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`priority_32`]
module"]
pub type PRIORITY_32 = crate::Reg<priority_32::PRIORITY_32_SPEC>;
#[doc = "PRIORITY Register for interrupt id 32"]
pub mod priority_32;
#[doc = "priority_33 (rw) register accessor: PRIORITY Register for interrupt id 33\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`priority_33::R`].  You can [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`priority_33::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`priority_33`]
module"]
pub type PRIORITY_33 = crate::Reg<priority_33::PRIORITY_33_SPEC>;
#[doc = "PRIORITY Register for interrupt id 33"]
pub mod priority_33;
#[doc = "priority_34 (rw) register accessor: PRIORITY Register for interrupt id 34\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`priority_34::R`].  You can [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`priority_34::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`priority_34`]
module"]
pub type PRIORITY_34 = crate::Reg<priority_34::PRIORITY_34_SPEC>;
#[doc = "PRIORITY Register for interrupt id 34"]
pub mod priority_34;
#[doc = "priority_35 (rw) register accessor: PRIORITY Register for interrupt id 35\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`priority_35::R`].  You can [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`priority_35::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`priority_35`]
module"]
pub type PRIORITY_35 = crate::Reg<priority_35::PRIORITY_35_SPEC>;
#[doc = "PRIORITY Register for interrupt id 35"]
pub mod priority_35;
#[doc = "priority_36 (rw) register accessor: PRIORITY Register for interrupt id 36\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`priority_36::R`].  You can [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`priority_36::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`priority_36`]
module"]
pub type PRIORITY_36 = crate::Reg<priority_36::PRIORITY_36_SPEC>;
#[doc = "PRIORITY Register for interrupt id 36"]
pub mod priority_36;
#[doc = "priority_37 (rw) register accessor: PRIORITY Register for interrupt id 37\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`priority_37::R`].  You can [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`priority_37::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`priority_37`]
module"]
pub type PRIORITY_37 = crate::Reg<priority_37::PRIORITY_37_SPEC>;
#[doc = "PRIORITY Register for interrupt id 37"]
pub mod priority_37;
#[doc = "priority_38 (rw) register accessor: PRIORITY Register for interrupt id 38\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`priority_38::R`].  You can [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`priority_38::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`priority_38`]
module"]
pub type PRIORITY_38 = crate::Reg<priority_38::PRIORITY_38_SPEC>;
#[doc = "PRIORITY Register for interrupt id 38"]
pub mod priority_38;
#[doc = "priority_39 (rw) register accessor: PRIORITY Register for interrupt id 39\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`priority_39::R`].  You can [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`priority_39::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`priority_39`]
module"]
pub type PRIORITY_39 = crate::Reg<priority_39::PRIORITY_39_SPEC>;
#[doc = "PRIORITY Register for interrupt id 39"]
pub mod priority_39;
#[doc = "priority_40 (rw) register accessor: PRIORITY Register for interrupt id 40\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`priority_40::R`].  You can [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`priority_40::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`priority_40`]
module"]
pub type PRIORITY_40 = crate::Reg<priority_40::PRIORITY_40_SPEC>;
#[doc = "PRIORITY Register for interrupt id 40"]
pub mod priority_40;
#[doc = "priority_41 (rw) register accessor: PRIORITY Register for interrupt id 41\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`priority_41::R`].  You can [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`priority_41::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`priority_41`]
module"]
pub type PRIORITY_41 = crate::Reg<priority_41::PRIORITY_41_SPEC>;
#[doc = "PRIORITY Register for interrupt id 41"]
pub mod priority_41;
#[doc = "priority_42 (rw) register accessor: PRIORITY Register for interrupt id 42\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`priority_42::R`].  You can [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`priority_42::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`priority_42`]
module"]
pub type PRIORITY_42 = crate::Reg<priority_42::PRIORITY_42_SPEC>;
#[doc = "PRIORITY Register for interrupt id 42"]
pub mod priority_42;
#[doc = "priority_43 (rw) register accessor: PRIORITY Register for interrupt id 43\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`priority_43::R`].  You can [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`priority_43::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`priority_43`]
module"]
pub type PRIORITY_43 = crate::Reg<priority_43::PRIORITY_43_SPEC>;
#[doc = "PRIORITY Register for interrupt id 43"]
pub mod priority_43;
#[doc = "priority_44 (rw) register accessor: PRIORITY Register for interrupt id 44\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`priority_44::R`].  You can [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`priority_44::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`priority_44`]
module"]
pub type PRIORITY_44 = crate::Reg<priority_44::PRIORITY_44_SPEC>;
#[doc = "PRIORITY Register for interrupt id 44"]
pub mod priority_44;
#[doc = "priority_45 (rw) register accessor: PRIORITY Register for interrupt id 45\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`priority_45::R`].  You can [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`priority_45::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`priority_45`]
module"]
pub type PRIORITY_45 = crate::Reg<priority_45::PRIORITY_45_SPEC>;
#[doc = "PRIORITY Register for interrupt id 45"]
pub mod priority_45;
#[doc = "priority_46 (rw) register accessor: PRIORITY Register for interrupt id 46\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`priority_46::R`].  You can [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`priority_46::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`priority_46`]
module"]
pub type PRIORITY_46 = crate::Reg<priority_46::PRIORITY_46_SPEC>;
#[doc = "PRIORITY Register for interrupt id 46"]
pub mod priority_46;
#[doc = "priority_47 (rw) register accessor: PRIORITY Register for interrupt id 47\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`priority_47::R`].  You can [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`priority_47::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`priority_47`]
module"]
pub type PRIORITY_47 = crate::Reg<priority_47::PRIORITY_47_SPEC>;
#[doc = "PRIORITY Register for interrupt id 47"]
pub mod priority_47;
#[doc = "priority_48 (rw) register accessor: PRIORITY Register for interrupt id 48\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`priority_48::R`].  You can [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`priority_48::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`priority_48`]
module"]
pub type PRIORITY_48 = crate::Reg<priority_48::PRIORITY_48_SPEC>;
#[doc = "PRIORITY Register for interrupt id 48"]
pub mod priority_48;
#[doc = "priority_49 (rw) register accessor: PRIORITY Register for interrupt id 49\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`priority_49::R`].  You can [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`priority_49::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`priority_49`]
module"]
pub type PRIORITY_49 = crate::Reg<priority_49::PRIORITY_49_SPEC>;
#[doc = "PRIORITY Register for interrupt id 49"]
pub mod priority_49;
#[doc = "priority_50 (rw) register accessor: PRIORITY Register for interrupt id 50\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`priority_50::R`].  You can [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`priority_50::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`priority_50`]
module"]
pub type PRIORITY_50 = crate::Reg<priority_50::PRIORITY_50_SPEC>;
#[doc = "PRIORITY Register for interrupt id 50"]
pub mod priority_50;
#[doc = "priority_51 (rw) register accessor: PRIORITY Register for interrupt id 51\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`priority_51::R`].  You can [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`priority_51::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`priority_51`]
module"]
pub type PRIORITY_51 = crate::Reg<priority_51::PRIORITY_51_SPEC>;
#[doc = "PRIORITY Register for interrupt id 51"]
pub mod priority_51;
#[doc = "priority_52 (rw) register accessor: PRIORITY Register for interrupt id 52\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`priority_52::R`].  You can [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`priority_52::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`priority_52`]
module"]
pub type PRIORITY_52 = crate::Reg<priority_52::PRIORITY_52_SPEC>;
#[doc = "PRIORITY Register for interrupt id 52"]
pub mod priority_52;
#[doc = "priority_53 (rw) register accessor: PRIORITY Register for interrupt id 53\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`priority_53::R`].  You can [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`priority_53::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`priority_53`]
module"]
pub type PRIORITY_53 = crate::Reg<priority_53::PRIORITY_53_SPEC>;
#[doc = "PRIORITY Register for interrupt id 53"]
pub mod priority_53;
#[doc = "priority_54 (rw) register accessor: PRIORITY Register for interrupt id 54\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`priority_54::R`].  You can [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`priority_54::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`priority_54`]
module"]
pub type PRIORITY_54 = crate::Reg<priority_54::PRIORITY_54_SPEC>;
#[doc = "PRIORITY Register for interrupt id 54"]
pub mod priority_54;
#[doc = "priority_55 (rw) register accessor: PRIORITY Register for interrupt id 55\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`priority_55::R`].  You can [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`priority_55::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`priority_55`]
module"]
pub type PRIORITY_55 = crate::Reg<priority_55::PRIORITY_55_SPEC>;
#[doc = "PRIORITY Register for interrupt id 55"]
pub mod priority_55;
#[doc = "priority_56 (rw) register accessor: PRIORITY Register for interrupt id 56\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`priority_56::R`].  You can [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`priority_56::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`priority_56`]
module"]
pub type PRIORITY_56 = crate::Reg<priority_56::PRIORITY_56_SPEC>;
#[doc = "PRIORITY Register for interrupt id 56"]
pub mod priority_56;
#[doc = "priority_57 (rw) register accessor: PRIORITY Register for interrupt id 57\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`priority_57::R`].  You can [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`priority_57::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`priority_57`]
module"]
pub type PRIORITY_57 = crate::Reg<priority_57::PRIORITY_57_SPEC>;
#[doc = "PRIORITY Register for interrupt id 57"]
pub mod priority_57;
#[doc = "priority_58 (rw) register accessor: PRIORITY Register for interrupt id 58\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`priority_58::R`].  You can [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`priority_58::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`priority_58`]
module"]
pub type PRIORITY_58 = crate::Reg<priority_58::PRIORITY_58_SPEC>;
#[doc = "PRIORITY Register for interrupt id 58"]
pub mod priority_58;
#[doc = "priority_59 (rw) register accessor: PRIORITY Register for interrupt id 59\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`priority_59::R`].  You can [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`priority_59::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`priority_59`]
module"]
pub type PRIORITY_59 = crate::Reg<priority_59::PRIORITY_59_SPEC>;
#[doc = "PRIORITY Register for interrupt id 59"]
pub mod priority_59;
#[doc = "priority_60 (rw) register accessor: PRIORITY Register for interrupt id 60\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`priority_60::R`].  You can [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`priority_60::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`priority_60`]
module"]
pub type PRIORITY_60 = crate::Reg<priority_60::PRIORITY_60_SPEC>;
#[doc = "PRIORITY Register for interrupt id 60"]
pub mod priority_60;
#[doc = "priority_61 (rw) register accessor: PRIORITY Register for interrupt id 61\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`priority_61::R`].  You can [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`priority_61::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`priority_61`]
module"]
pub type PRIORITY_61 = crate::Reg<priority_61::PRIORITY_61_SPEC>;
#[doc = "PRIORITY Register for interrupt id 61"]
pub mod priority_61;
#[doc = "priority_62 (rw) register accessor: PRIORITY Register for interrupt id 62\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`priority_62::R`].  You can [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`priority_62::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`priority_62`]
module"]
pub type PRIORITY_62 = crate::Reg<priority_62::PRIORITY_62_SPEC>;
#[doc = "PRIORITY Register for interrupt id 62"]
pub mod priority_62;
#[doc = "priority_63 (rw) register accessor: PRIORITY Register for interrupt id 63\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`priority_63::R`].  You can [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`priority_63::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`priority_63`]
module"]
pub type PRIORITY_63 = crate::Reg<priority_63::PRIORITY_63_SPEC>;
#[doc = "PRIORITY Register for interrupt id 63"]
pub mod priority_63;
#[doc = "priority_64 (rw) register accessor: PRIORITY Register for interrupt id 64\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`priority_64::R`].  You can [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`priority_64::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`priority_64`]
module"]
pub type PRIORITY_64 = crate::Reg<priority_64::PRIORITY_64_SPEC>;
#[doc = "PRIORITY Register for interrupt id 64"]
pub mod priority_64;
#[doc = "priority_65 (rw) register accessor: PRIORITY Register for interrupt id 65\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`priority_65::R`].  You can [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`priority_65::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`priority_65`]
module"]
pub type PRIORITY_65 = crate::Reg<priority_65::PRIORITY_65_SPEC>;
#[doc = "PRIORITY Register for interrupt id 65"]
pub mod priority_65;
#[doc = "priority_66 (rw) register accessor: PRIORITY Register for interrupt id 66\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`priority_66::R`].  You can [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`priority_66::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`priority_66`]
module"]
pub type PRIORITY_66 = crate::Reg<priority_66::PRIORITY_66_SPEC>;
#[doc = "PRIORITY Register for interrupt id 66"]
pub mod priority_66;
#[doc = "priority_67 (rw) register accessor: PRIORITY Register for interrupt id 67\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`priority_67::R`].  You can [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`priority_67::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`priority_67`]
module"]
pub type PRIORITY_67 = crate::Reg<priority_67::PRIORITY_67_SPEC>;
#[doc = "PRIORITY Register for interrupt id 67"]
pub mod priority_67;
#[doc = "priority_68 (rw) register accessor: PRIORITY Register for interrupt id 68\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`priority_68::R`].  You can [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`priority_68::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`priority_68`]
module"]
pub type PRIORITY_68 = crate::Reg<priority_68::PRIORITY_68_SPEC>;
#[doc = "PRIORITY Register for interrupt id 68"]
pub mod priority_68;
#[doc = "priority_69 (rw) register accessor: PRIORITY Register for interrupt id 69\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`priority_69::R`].  You can [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`priority_69::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`priority_69`]
module"]
pub type PRIORITY_69 = crate::Reg<priority_69::PRIORITY_69_SPEC>;
#[doc = "PRIORITY Register for interrupt id 69"]
pub mod priority_69;
#[doc = "priority_70 (rw) register accessor: PRIORITY Register for interrupt id 70\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`priority_70::R`].  You can [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`priority_70::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`priority_70`]
module"]
pub type PRIORITY_70 = crate::Reg<priority_70::PRIORITY_70_SPEC>;
#[doc = "PRIORITY Register for interrupt id 70"]
pub mod priority_70;
#[doc = "priority_71 (rw) register accessor: PRIORITY Register for interrupt id 71\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`priority_71::R`].  You can [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`priority_71::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`priority_71`]
module"]
pub type PRIORITY_71 = crate::Reg<priority_71::PRIORITY_71_SPEC>;
#[doc = "PRIORITY Register for interrupt id 71"]
pub mod priority_71;
#[doc = "priority_72 (rw) register accessor: PRIORITY Register for interrupt id 72\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`priority_72::R`].  You can [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`priority_72::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`priority_72`]
module"]
pub type PRIORITY_72 = crate::Reg<priority_72::PRIORITY_72_SPEC>;
#[doc = "PRIORITY Register for interrupt id 72"]
pub mod priority_72;
#[doc = "priority_73 (rw) register accessor: PRIORITY Register for interrupt id 73\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`priority_73::R`].  You can [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`priority_73::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`priority_73`]
module"]
pub type PRIORITY_73 = crate::Reg<priority_73::PRIORITY_73_SPEC>;
#[doc = "PRIORITY Register for interrupt id 73"]
pub mod priority_73;
#[doc = "priority_74 (rw) register accessor: PRIORITY Register for interrupt id 74\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`priority_74::R`].  You can [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`priority_74::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`priority_74`]
module"]
pub type PRIORITY_74 = crate::Reg<priority_74::PRIORITY_74_SPEC>;
#[doc = "PRIORITY Register for interrupt id 74"]
pub mod priority_74;
#[doc = "priority_75 (rw) register accessor: PRIORITY Register for interrupt id 75\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`priority_75::R`].  You can [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`priority_75::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`priority_75`]
module"]
pub type PRIORITY_75 = crate::Reg<priority_75::PRIORITY_75_SPEC>;
#[doc = "PRIORITY Register for interrupt id 75"]
pub mod priority_75;
#[doc = "priority_76 (rw) register accessor: PRIORITY Register for interrupt id 76\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`priority_76::R`].  You can [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`priority_76::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`priority_76`]
module"]
pub type PRIORITY_76 = crate::Reg<priority_76::PRIORITY_76_SPEC>;
#[doc = "PRIORITY Register for interrupt id 76"]
pub mod priority_76;
#[doc = "priority_77 (rw) register accessor: PRIORITY Register for interrupt id 77\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`priority_77::R`].  You can [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`priority_77::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`priority_77`]
module"]
pub type PRIORITY_77 = crate::Reg<priority_77::PRIORITY_77_SPEC>;
#[doc = "PRIORITY Register for interrupt id 77"]
pub mod priority_77;
#[doc = "priority_78 (rw) register accessor: PRIORITY Register for interrupt id 78\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`priority_78::R`].  You can [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`priority_78::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`priority_78`]
module"]
pub type PRIORITY_78 = crate::Reg<priority_78::PRIORITY_78_SPEC>;
#[doc = "PRIORITY Register for interrupt id 78"]
pub mod priority_78;
#[doc = "priority_79 (rw) register accessor: PRIORITY Register for interrupt id 79\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`priority_79::R`].  You can [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`priority_79::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`priority_79`]
module"]
pub type PRIORITY_79 = crate::Reg<priority_79::PRIORITY_79_SPEC>;
#[doc = "PRIORITY Register for interrupt id 79"]
pub mod priority_79;
#[doc = "priority_80 (rw) register accessor: PRIORITY Register for interrupt id 80\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`priority_80::R`].  You can [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`priority_80::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`priority_80`]
module"]
pub type PRIORITY_80 = crate::Reg<priority_80::PRIORITY_80_SPEC>;
#[doc = "PRIORITY Register for interrupt id 80"]
pub mod priority_80;
#[doc = "priority_81 (rw) register accessor: PRIORITY Register for interrupt id 81\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`priority_81::R`].  You can [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`priority_81::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`priority_81`]
module"]
pub type PRIORITY_81 = crate::Reg<priority_81::PRIORITY_81_SPEC>;
#[doc = "PRIORITY Register for interrupt id 81"]
pub mod priority_81;
#[doc = "priority_82 (rw) register accessor: PRIORITY Register for interrupt id 82\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`priority_82::R`].  You can [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`priority_82::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`priority_82`]
module"]
pub type PRIORITY_82 = crate::Reg<priority_82::PRIORITY_82_SPEC>;
#[doc = "PRIORITY Register for interrupt id 82"]
pub mod priority_82;
#[doc = "priority_83 (rw) register accessor: PRIORITY Register for interrupt id 83\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`priority_83::R`].  You can [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`priority_83::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`priority_83`]
module"]
pub type PRIORITY_83 = crate::Reg<priority_83::PRIORITY_83_SPEC>;
#[doc = "PRIORITY Register for interrupt id 83"]
pub mod priority_83;
#[doc = "priority_84 (rw) register accessor: PRIORITY Register for interrupt id 84\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`priority_84::R`].  You can [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`priority_84::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`priority_84`]
module"]
pub type PRIORITY_84 = crate::Reg<priority_84::PRIORITY_84_SPEC>;
#[doc = "PRIORITY Register for interrupt id 84"]
pub mod priority_84;
#[doc = "priority_85 (rw) register accessor: PRIORITY Register for interrupt id 85\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`priority_85::R`].  You can [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`priority_85::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`priority_85`]
module"]
pub type PRIORITY_85 = crate::Reg<priority_85::PRIORITY_85_SPEC>;
#[doc = "PRIORITY Register for interrupt id 85"]
pub mod priority_85;
#[doc = "priority_86 (rw) register accessor: PRIORITY Register for interrupt id 86\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`priority_86::R`].  You can [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`priority_86::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`priority_86`]
module"]
pub type PRIORITY_86 = crate::Reg<priority_86::PRIORITY_86_SPEC>;
#[doc = "PRIORITY Register for interrupt id 86"]
pub mod priority_86;
#[doc = "priority_87 (rw) register accessor: PRIORITY Register for interrupt id 87\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`priority_87::R`].  You can [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`priority_87::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`priority_87`]
module"]
pub type PRIORITY_87 = crate::Reg<priority_87::PRIORITY_87_SPEC>;
#[doc = "PRIORITY Register for interrupt id 87"]
pub mod priority_87;
#[doc = "priority_88 (rw) register accessor: PRIORITY Register for interrupt id 88\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`priority_88::R`].  You can [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`priority_88::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`priority_88`]
module"]
pub type PRIORITY_88 = crate::Reg<priority_88::PRIORITY_88_SPEC>;
#[doc = "PRIORITY Register for interrupt id 88"]
pub mod priority_88;
#[doc = "priority_89 (rw) register accessor: PRIORITY Register for interrupt id 89\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`priority_89::R`].  You can [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`priority_89::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`priority_89`]
module"]
pub type PRIORITY_89 = crate::Reg<priority_89::PRIORITY_89_SPEC>;
#[doc = "PRIORITY Register for interrupt id 89"]
pub mod priority_89;
#[doc = "priority_90 (rw) register accessor: PRIORITY Register for interrupt id 90\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`priority_90::R`].  You can [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`priority_90::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`priority_90`]
module"]
pub type PRIORITY_90 = crate::Reg<priority_90::PRIORITY_90_SPEC>;
#[doc = "PRIORITY Register for interrupt id 90"]
pub mod priority_90;
#[doc = "priority_91 (rw) register accessor: PRIORITY Register for interrupt id 91\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`priority_91::R`].  You can [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`priority_91::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`priority_91`]
module"]
pub type PRIORITY_91 = crate::Reg<priority_91::PRIORITY_91_SPEC>;
#[doc = "PRIORITY Register for interrupt id 91"]
pub mod priority_91;
#[doc = "priority_92 (rw) register accessor: PRIORITY Register for interrupt id 92\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`priority_92::R`].  You can [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`priority_92::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`priority_92`]
module"]
pub type PRIORITY_92 = crate::Reg<priority_92::PRIORITY_92_SPEC>;
#[doc = "PRIORITY Register for interrupt id 92"]
pub mod priority_92;
#[doc = "priority_93 (rw) register accessor: PRIORITY Register for interrupt id 93\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`priority_93::R`].  You can [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`priority_93::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`priority_93`]
module"]
pub type PRIORITY_93 = crate::Reg<priority_93::PRIORITY_93_SPEC>;
#[doc = "PRIORITY Register for interrupt id 93"]
pub mod priority_93;
#[doc = "priority_94 (rw) register accessor: PRIORITY Register for interrupt id 94\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`priority_94::R`].  You can [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`priority_94::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`priority_94`]
module"]
pub type PRIORITY_94 = crate::Reg<priority_94::PRIORITY_94_SPEC>;
#[doc = "PRIORITY Register for interrupt id 94"]
pub mod priority_94;
#[doc = "priority_95 (rw) register accessor: PRIORITY Register for interrupt id 95\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`priority_95::R`].  You can [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`priority_95::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`priority_95`]
module"]
pub type PRIORITY_95 = crate::Reg<priority_95::PRIORITY_95_SPEC>;
#[doc = "PRIORITY Register for interrupt id 95"]
pub mod priority_95;
#[doc = "priority_96 (rw) register accessor: PRIORITY Register for interrupt id 96\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`priority_96::R`].  You can [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`priority_96::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`priority_96`]
module"]
pub type PRIORITY_96 = crate::Reg<priority_96::PRIORITY_96_SPEC>;
#[doc = "PRIORITY Register for interrupt id 96"]
pub mod priority_96;
#[doc = "priority_97 (rw) register accessor: PRIORITY Register for interrupt id 97\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`priority_97::R`].  You can [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`priority_97::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`priority_97`]
module"]
pub type PRIORITY_97 = crate::Reg<priority_97::PRIORITY_97_SPEC>;
#[doc = "PRIORITY Register for interrupt id 97"]
pub mod priority_97;
#[doc = "priority_98 (rw) register accessor: PRIORITY Register for interrupt id 98\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`priority_98::R`].  You can [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`priority_98::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`priority_98`]
module"]
pub type PRIORITY_98 = crate::Reg<priority_98::PRIORITY_98_SPEC>;
#[doc = "PRIORITY Register for interrupt id 98"]
pub mod priority_98;
#[doc = "priority_99 (rw) register accessor: PRIORITY Register for interrupt id 99\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`priority_99::R`].  You can [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`priority_99::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`priority_99`]
module"]
pub type PRIORITY_99 = crate::Reg<priority_99::PRIORITY_99_SPEC>;
#[doc = "PRIORITY Register for interrupt id 99"]
pub mod priority_99;
#[doc = "priority_100 (rw) register accessor: PRIORITY Register for interrupt id 100\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`priority_100::R`].  You can [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`priority_100::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`priority_100`]
module"]
pub type PRIORITY_100 = crate::Reg<priority_100::PRIORITY_100_SPEC>;
#[doc = "PRIORITY Register for interrupt id 100"]
pub mod priority_100;
#[doc = "priority_101 (rw) register accessor: PRIORITY Register for interrupt id 101\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`priority_101::R`].  You can [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`priority_101::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`priority_101`]
module"]
pub type PRIORITY_101 = crate::Reg<priority_101::PRIORITY_101_SPEC>;
#[doc = "PRIORITY Register for interrupt id 101"]
pub mod priority_101;
#[doc = "priority_102 (rw) register accessor: PRIORITY Register for interrupt id 102\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`priority_102::R`].  You can [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`priority_102::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`priority_102`]
module"]
pub type PRIORITY_102 = crate::Reg<priority_102::PRIORITY_102_SPEC>;
#[doc = "PRIORITY Register for interrupt id 102"]
pub mod priority_102;
#[doc = "priority_103 (rw) register accessor: PRIORITY Register for interrupt id 103\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`priority_103::R`].  You can [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`priority_103::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`priority_103`]
module"]
pub type PRIORITY_103 = crate::Reg<priority_103::PRIORITY_103_SPEC>;
#[doc = "PRIORITY Register for interrupt id 103"]
pub mod priority_103;
#[doc = "priority_104 (rw) register accessor: PRIORITY Register for interrupt id 104\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`priority_104::R`].  You can [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`priority_104::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`priority_104`]
module"]
pub type PRIORITY_104 = crate::Reg<priority_104::PRIORITY_104_SPEC>;
#[doc = "PRIORITY Register for interrupt id 104"]
pub mod priority_104;
#[doc = "priority_105 (rw) register accessor: PRIORITY Register for interrupt id 105\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`priority_105::R`].  You can [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`priority_105::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`priority_105`]
module"]
pub type PRIORITY_105 = crate::Reg<priority_105::PRIORITY_105_SPEC>;
#[doc = "PRIORITY Register for interrupt id 105"]
pub mod priority_105;
#[doc = "priority_106 (rw) register accessor: PRIORITY Register for interrupt id 106\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`priority_106::R`].  You can [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`priority_106::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`priority_106`]
module"]
pub type PRIORITY_106 = crate::Reg<priority_106::PRIORITY_106_SPEC>;
#[doc = "PRIORITY Register for interrupt id 106"]
pub mod priority_106;
#[doc = "priority_107 (rw) register accessor: PRIORITY Register for interrupt id 107\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`priority_107::R`].  You can [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`priority_107::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`priority_107`]
module"]
pub type PRIORITY_107 = crate::Reg<priority_107::PRIORITY_107_SPEC>;
#[doc = "PRIORITY Register for interrupt id 107"]
pub mod priority_107;
#[doc = "priority_108 (rw) register accessor: PRIORITY Register for interrupt id 108\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`priority_108::R`].  You can [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`priority_108::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`priority_108`]
module"]
pub type PRIORITY_108 = crate::Reg<priority_108::PRIORITY_108_SPEC>;
#[doc = "PRIORITY Register for interrupt id 108"]
pub mod priority_108;
#[doc = "priority_109 (rw) register accessor: PRIORITY Register for interrupt id 109\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`priority_109::R`].  You can [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`priority_109::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`priority_109`]
module"]
pub type PRIORITY_109 = crate::Reg<priority_109::PRIORITY_109_SPEC>;
#[doc = "PRIORITY Register for interrupt id 109"]
pub mod priority_109;
#[doc = "priority_110 (rw) register accessor: PRIORITY Register for interrupt id 110\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`priority_110::R`].  You can [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`priority_110::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`priority_110`]
module"]
pub type PRIORITY_110 = crate::Reg<priority_110::PRIORITY_110_SPEC>;
#[doc = "PRIORITY Register for interrupt id 110"]
pub mod priority_110;
#[doc = "priority_111 (rw) register accessor: PRIORITY Register for interrupt id 111\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`priority_111::R`].  You can [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`priority_111::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`priority_111`]
module"]
pub type PRIORITY_111 = crate::Reg<priority_111::PRIORITY_111_SPEC>;
#[doc = "PRIORITY Register for interrupt id 111"]
pub mod priority_111;
#[doc = "priority_112 (rw) register accessor: PRIORITY Register for interrupt id 112\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`priority_112::R`].  You can [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`priority_112::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`priority_112`]
module"]
pub type PRIORITY_112 = crate::Reg<priority_112::PRIORITY_112_SPEC>;
#[doc = "PRIORITY Register for interrupt id 112"]
pub mod priority_112;
#[doc = "priority_113 (rw) register accessor: PRIORITY Register for interrupt id 113\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`priority_113::R`].  You can [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`priority_113::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`priority_113`]
module"]
pub type PRIORITY_113 = crate::Reg<priority_113::PRIORITY_113_SPEC>;
#[doc = "PRIORITY Register for interrupt id 113"]
pub mod priority_113;
#[doc = "priority_114 (rw) register accessor: PRIORITY Register for interrupt id 114\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`priority_114::R`].  You can [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`priority_114::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`priority_114`]
module"]
pub type PRIORITY_114 = crate::Reg<priority_114::PRIORITY_114_SPEC>;
#[doc = "PRIORITY Register for interrupt id 114"]
pub mod priority_114;
#[doc = "priority_115 (rw) register accessor: PRIORITY Register for interrupt id 115\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`priority_115::R`].  You can [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`priority_115::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`priority_115`]
module"]
pub type PRIORITY_115 = crate::Reg<priority_115::PRIORITY_115_SPEC>;
#[doc = "PRIORITY Register for interrupt id 115"]
pub mod priority_115;
#[doc = "priority_116 (rw) register accessor: PRIORITY Register for interrupt id 116\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`priority_116::R`].  You can [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`priority_116::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`priority_116`]
module"]
pub type PRIORITY_116 = crate::Reg<priority_116::PRIORITY_116_SPEC>;
#[doc = "PRIORITY Register for interrupt id 116"]
pub mod priority_116;
#[doc = "priority_117 (rw) register accessor: PRIORITY Register for interrupt id 117\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`priority_117::R`].  You can [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`priority_117::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`priority_117`]
module"]
pub type PRIORITY_117 = crate::Reg<priority_117::PRIORITY_117_SPEC>;
#[doc = "PRIORITY Register for interrupt id 117"]
pub mod priority_117;
#[doc = "priority_118 (rw) register accessor: PRIORITY Register for interrupt id 118\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`priority_118::R`].  You can [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`priority_118::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`priority_118`]
module"]
pub type PRIORITY_118 = crate::Reg<priority_118::PRIORITY_118_SPEC>;
#[doc = "PRIORITY Register for interrupt id 118"]
pub mod priority_118;
#[doc = "priority_119 (rw) register accessor: PRIORITY Register for interrupt id 119\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`priority_119::R`].  You can [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`priority_119::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`priority_119`]
module"]
pub type PRIORITY_119 = crate::Reg<priority_119::PRIORITY_119_SPEC>;
#[doc = "PRIORITY Register for interrupt id 119"]
pub mod priority_119;
#[doc = "priority_120 (rw) register accessor: PRIORITY Register for interrupt id 120\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`priority_120::R`].  You can [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`priority_120::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`priority_120`]
module"]
pub type PRIORITY_120 = crate::Reg<priority_120::PRIORITY_120_SPEC>;
#[doc = "PRIORITY Register for interrupt id 120"]
pub mod priority_120;
#[doc = "priority_121 (rw) register accessor: PRIORITY Register for interrupt id 121\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`priority_121::R`].  You can [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`priority_121::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`priority_121`]
module"]
pub type PRIORITY_121 = crate::Reg<priority_121::PRIORITY_121_SPEC>;
#[doc = "PRIORITY Register for interrupt id 121"]
pub mod priority_121;
#[doc = "priority_122 (rw) register accessor: PRIORITY Register for interrupt id 122\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`priority_122::R`].  You can [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`priority_122::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`priority_122`]
module"]
pub type PRIORITY_122 = crate::Reg<priority_122::PRIORITY_122_SPEC>;
#[doc = "PRIORITY Register for interrupt id 122"]
pub mod priority_122;
#[doc = "priority_123 (rw) register accessor: PRIORITY Register for interrupt id 123\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`priority_123::R`].  You can [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`priority_123::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`priority_123`]
module"]
pub type PRIORITY_123 = crate::Reg<priority_123::PRIORITY_123_SPEC>;
#[doc = "PRIORITY Register for interrupt id 123"]
pub mod priority_123;
#[doc = "priority_124 (rw) register accessor: PRIORITY Register for interrupt id 124\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`priority_124::R`].  You can [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`priority_124::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`priority_124`]
module"]
pub type PRIORITY_124 = crate::Reg<priority_124::PRIORITY_124_SPEC>;
#[doc = "PRIORITY Register for interrupt id 124"]
pub mod priority_124;
#[doc = "priority_125 (rw) register accessor: PRIORITY Register for interrupt id 125\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`priority_125::R`].  You can [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`priority_125::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`priority_125`]
module"]
pub type PRIORITY_125 = crate::Reg<priority_125::PRIORITY_125_SPEC>;
#[doc = "PRIORITY Register for interrupt id 125"]
pub mod priority_125;
#[doc = "priority_126 (rw) register accessor: PRIORITY Register for interrupt id 126\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`priority_126::R`].  You can [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`priority_126::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`priority_126`]
module"]
pub type PRIORITY_126 = crate::Reg<priority_126::PRIORITY_126_SPEC>;
#[doc = "PRIORITY Register for interrupt id 126"]
pub mod priority_126;
#[doc = "priority_127 (rw) register accessor: PRIORITY Register for interrupt id 127\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`priority_127::R`].  You can [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`priority_127::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`priority_127`]
module"]
pub type PRIORITY_127 = crate::Reg<priority_127::PRIORITY_127_SPEC>;
#[doc = "PRIORITY Register for interrupt id 127"]
pub mod priority_127;
#[doc = "priority_128 (rw) register accessor: PRIORITY Register for interrupt id 128\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`priority_128::R`].  You can [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`priority_128::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`priority_128`]
module"]
pub type PRIORITY_128 = crate::Reg<priority_128::PRIORITY_128_SPEC>;
#[doc = "PRIORITY Register for interrupt id 128"]
pub mod priority_128;
#[doc = "priority_129 (rw) register accessor: PRIORITY Register for interrupt id 129\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`priority_129::R`].  You can [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`priority_129::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`priority_129`]
module"]
pub type PRIORITY_129 = crate::Reg<priority_129::PRIORITY_129_SPEC>;
#[doc = "PRIORITY Register for interrupt id 129"]
pub mod priority_129;
#[doc = "priority_130 (rw) register accessor: PRIORITY Register for interrupt id 130\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`priority_130::R`].  You can [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`priority_130::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`priority_130`]
module"]
pub type PRIORITY_130 = crate::Reg<priority_130::PRIORITY_130_SPEC>;
#[doc = "PRIORITY Register for interrupt id 130"]
pub mod priority_130;
#[doc = "priority_131 (rw) register accessor: PRIORITY Register for interrupt id 131\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`priority_131::R`].  You can [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`priority_131::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`priority_131`]
module"]
pub type PRIORITY_131 = crate::Reg<priority_131::PRIORITY_131_SPEC>;
#[doc = "PRIORITY Register for interrupt id 131"]
pub mod priority_131;
#[doc = "priority_132 (rw) register accessor: PRIORITY Register for interrupt id 132\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`priority_132::R`].  You can [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`priority_132::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`priority_132`]
module"]
pub type PRIORITY_132 = crate::Reg<priority_132::PRIORITY_132_SPEC>;
#[doc = "PRIORITY Register for interrupt id 132"]
pub mod priority_132;
#[doc = "priority_133 (rw) register accessor: PRIORITY Register for interrupt id 133\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`priority_133::R`].  You can [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`priority_133::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`priority_133`]
module"]
pub type PRIORITY_133 = crate::Reg<priority_133::PRIORITY_133_SPEC>;
#[doc = "PRIORITY Register for interrupt id 133"]
pub mod priority_133;
#[doc = "priority_134 (rw) register accessor: PRIORITY Register for interrupt id 134\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`priority_134::R`].  You can [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`priority_134::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`priority_134`]
module"]
pub type PRIORITY_134 = crate::Reg<priority_134::PRIORITY_134_SPEC>;
#[doc = "PRIORITY Register for interrupt id 134"]
pub mod priority_134;
#[doc = "priority_135 (rw) register accessor: PRIORITY Register for interrupt id 135\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`priority_135::R`].  You can [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`priority_135::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`priority_135`]
module"]
pub type PRIORITY_135 = crate::Reg<priority_135::PRIORITY_135_SPEC>;
#[doc = "PRIORITY Register for interrupt id 135"]
pub mod priority_135;
#[doc = "priority_136 (rw) register accessor: PRIORITY Register for interrupt id 136\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`priority_136::R`].  You can [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`priority_136::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`priority_136`]
module"]
pub type PRIORITY_136 = crate::Reg<priority_136::PRIORITY_136_SPEC>;
#[doc = "PRIORITY Register for interrupt id 136"]
pub mod priority_136;
#[doc = "pending_0 (rw) register accessor: PENDING Register for interrupt ids 31 to 0\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`pending_0::R`].  You can [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`pending_0::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`pending_0`]
module"]
pub type PENDING_0 = crate::Reg<pending_0::PENDING_0_SPEC>;
#[doc = "PENDING Register for interrupt ids 31 to 0"]
pub mod pending_0;
#[doc = "pending_1 (rw) register accessor: PENDING Register for interrupt ids 63 to 32\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`pending_1::R`].  You can [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`pending_1::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`pending_1`]
module"]
pub type PENDING_1 = crate::Reg<pending_1::PENDING_1_SPEC>;
#[doc = "PENDING Register for interrupt ids 63 to 32"]
pub mod pending_1;
#[doc = "pending_2 (rw) register accessor: PENDING Register for interrupt ids 95 to 64\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`pending_2::R`].  You can [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`pending_2::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`pending_2`]
module"]
pub type PENDING_2 = crate::Reg<pending_2::PENDING_2_SPEC>;
#[doc = "PENDING Register for interrupt ids 95 to 64"]
pub mod pending_2;
#[doc = "pending_3 (rw) register accessor: PENDING Register for interrupt ids 127 to 96\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`pending_3::R`].  You can [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`pending_3::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`pending_3`]
module"]
pub type PENDING_3 = crate::Reg<pending_3::PENDING_3_SPEC>;
#[doc = "PENDING Register for interrupt ids 127 to 96"]
pub mod pending_3;
#[doc = "pending_4 (rw) register accessor: PENDING Register for interrupt ids 136 to 128\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`pending_4::R`].  You can [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`pending_4::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`pending_4`]
module"]
pub type PENDING_4 = crate::Reg<pending_4::PENDING_4_SPEC>;
#[doc = "PENDING Register for interrupt ids 136 to 128"]
pub mod pending_4;
#[doc = "enable_0_0 (rw) register accessor: ENABLE Register for interrupt ids 31 to 0 for hart 0\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`enable_0_0::R`].  You can [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`enable_0_0::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`enable_0_0`]
module"]
pub type ENABLE_0_0 = crate::Reg<enable_0_0::ENABLE_0_0_SPEC>;
#[doc = "ENABLE Register for interrupt ids 31 to 0 for hart 0"]
pub mod enable_0_0;
#[doc = "enable_1_0 (rw) register accessor: ENABLE Register for interrupt ids 63 to 32 for hart 0\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`enable_1_0::R`].  You can [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`enable_1_0::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`enable_1_0`]
module"]
pub type ENABLE_1_0 = crate::Reg<enable_1_0::ENABLE_1_0_SPEC>;
#[doc = "ENABLE Register for interrupt ids 63 to 32 for hart 0"]
pub mod enable_1_0;
#[doc = "enable_2_0 (rw) register accessor: ENABLE Register for interrupt ids 95 to 64 for hart 0\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`enable_2_0::R`].  You can [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`enable_2_0::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`enable_2_0`]
module"]
pub type ENABLE_2_0 = crate::Reg<enable_2_0::ENABLE_2_0_SPEC>;
#[doc = "ENABLE Register for interrupt ids 95 to 64 for hart 0"]
pub mod enable_2_0;
#[doc = "enable_3_0 (rw) register accessor: ENABLE Register for interrupt ids 127 to 96 for hart 0\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`enable_3_0::R`].  You can [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`enable_3_0::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`enable_3_0`]
module"]
pub type ENABLE_3_0 = crate::Reg<enable_3_0::ENABLE_3_0_SPEC>;
#[doc = "ENABLE Register for interrupt ids 127 to 96 for hart 0"]
pub mod enable_3_0;
#[doc = "enable_4_0 (rw) register accessor: ENABLE Register for interrupt ids 136 to 128 for hart 0\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`enable_4_0::R`].  You can [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`enable_4_0::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`enable_4_0`]
module"]
pub type ENABLE_4_0 = crate::Reg<enable_4_0::ENABLE_4_0_SPEC>;
#[doc = "ENABLE Register for interrupt ids 136 to 128 for hart 0"]
pub mod enable_4_0;
#[doc = "enable_0_1 (rw) register accessor: ENABLE Register for interrupt ids 31 to 0 for hart 1\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`enable_0_1::R`].  You can [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`enable_0_1::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`enable_0_1`]
module"]
pub type ENABLE_0_1 = crate::Reg<enable_0_1::ENABLE_0_1_SPEC>;
#[doc = "ENABLE Register for interrupt ids 31 to 0 for hart 1"]
pub mod enable_0_1;
#[doc = "enable_1_1 (rw) register accessor: ENABLE Register for interrupt ids 63 to 32 for hart 1\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`enable_1_1::R`].  You can [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`enable_1_1::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`enable_1_1`]
module"]
pub type ENABLE_1_1 = crate::Reg<enable_1_1::ENABLE_1_1_SPEC>;
#[doc = "ENABLE Register for interrupt ids 63 to 32 for hart 1"]
pub mod enable_1_1;
#[doc = "enable_2_1 (rw) register accessor: ENABLE Register for interrupt ids 95 to 64 for hart 1\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`enable_2_1::R`].  You can [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`enable_2_1::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`enable_2_1`]
module"]
pub type ENABLE_2_1 = crate::Reg<enable_2_1::ENABLE_2_1_SPEC>;
#[doc = "ENABLE Register for interrupt ids 95 to 64 for hart 1"]
pub mod enable_2_1;
#[doc = "enable_3_1 (rw) register accessor: ENABLE Register for interrupt ids 127 to 96 for hart 1\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`enable_3_1::R`].  You can [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`enable_3_1::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`enable_3_1`]
module"]
pub type ENABLE_3_1 = crate::Reg<enable_3_1::ENABLE_3_1_SPEC>;
#[doc = "ENABLE Register for interrupt ids 127 to 96 for hart 1"]
pub mod enable_3_1;
#[doc = "enable_4_1 (rw) register accessor: ENABLE Register for interrupt ids 136 to 128 for hart 1\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`enable_4_1::R`].  You can [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`enable_4_1::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`enable_4_1`]
module"]
pub type ENABLE_4_1 = crate::Reg<enable_4_1::ENABLE_4_1_SPEC>;
#[doc = "ENABLE Register for interrupt ids 136 to 128 for hart 1"]
pub mod enable_4_1;
#[doc = "enable_0_2 (rw) register accessor: ENABLE Register for interrupt ids 31 to 0 for hart 2\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`enable_0_2::R`].  You can [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`enable_0_2::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`enable_0_2`]
module"]
pub type ENABLE_0_2 = crate::Reg<enable_0_2::ENABLE_0_2_SPEC>;
#[doc = "ENABLE Register for interrupt ids 31 to 0 for hart 2"]
pub mod enable_0_2;
#[doc = "enable_1_2 (rw) register accessor: ENABLE Register for interrupt ids 63 to 32 for hart 2\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`enable_1_2::R`].  You can [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`enable_1_2::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`enable_1_2`]
module"]
pub type ENABLE_1_2 = crate::Reg<enable_1_2::ENABLE_1_2_SPEC>;
#[doc = "ENABLE Register for interrupt ids 63 to 32 for hart 2"]
pub mod enable_1_2;
#[doc = "enable_2_2 (rw) register accessor: ENABLE Register for interrupt ids 95 to 64 for hart 2\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`enable_2_2::R`].  You can [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`enable_2_2::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`enable_2_2`]
module"]
pub type ENABLE_2_2 = crate::Reg<enable_2_2::ENABLE_2_2_SPEC>;
#[doc = "ENABLE Register for interrupt ids 95 to 64 for hart 2"]
pub mod enable_2_2;
#[doc = "enable_3_2 (rw) register accessor: ENABLE Register for interrupt ids 127 to 96 for hart 2\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`enable_3_2::R`].  You can [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`enable_3_2::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`enable_3_2`]
module"]
pub type ENABLE_3_2 = crate::Reg<enable_3_2::ENABLE_3_2_SPEC>;
#[doc = "ENABLE Register for interrupt ids 127 to 96 for hart 2"]
pub mod enable_3_2;
#[doc = "enable_4_2 (rw) register accessor: ENABLE Register for interrupt ids 136 to 128 for hart 2\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`enable_4_2::R`].  You can [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`enable_4_2::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`enable_4_2`]
module"]
pub type ENABLE_4_2 = crate::Reg<enable_4_2::ENABLE_4_2_SPEC>;
#[doc = "ENABLE Register for interrupt ids 136 to 128 for hart 2"]
pub mod enable_4_2;
#[doc = "enable_0_3 (rw) register accessor: ENABLE Register for interrupt ids 31 to 0 for hart 3\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`enable_0_3::R`].  You can [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`enable_0_3::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`enable_0_3`]
module"]
pub type ENABLE_0_3 = crate::Reg<enable_0_3::ENABLE_0_3_SPEC>;
#[doc = "ENABLE Register for interrupt ids 31 to 0 for hart 3"]
pub mod enable_0_3;
#[doc = "enable_1_3 (rw) register accessor: ENABLE Register for interrupt ids 63 to 32 for hart 3\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`enable_1_3::R`].  You can [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`enable_1_3::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`enable_1_3`]
module"]
pub type ENABLE_1_3 = crate::Reg<enable_1_3::ENABLE_1_3_SPEC>;
#[doc = "ENABLE Register for interrupt ids 63 to 32 for hart 3"]
pub mod enable_1_3;
#[doc = "enable_2_3 (rw) register accessor: ENABLE Register for interrupt ids 95 to 64 for hart 3\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`enable_2_3::R`].  You can [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`enable_2_3::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`enable_2_3`]
module"]
pub type ENABLE_2_3 = crate::Reg<enable_2_3::ENABLE_2_3_SPEC>;
#[doc = "ENABLE Register for interrupt ids 95 to 64 for hart 3"]
pub mod enable_2_3;
#[doc = "enable_3_3 (rw) register accessor: ENABLE Register for interrupt ids 127 to 96 for hart 3\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`enable_3_3::R`].  You can [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`enable_3_3::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`enable_3_3`]
module"]
pub type ENABLE_3_3 = crate::Reg<enable_3_3::ENABLE_3_3_SPEC>;
#[doc = "ENABLE Register for interrupt ids 127 to 96 for hart 3"]
pub mod enable_3_3;
#[doc = "enable_4_3 (rw) register accessor: ENABLE Register for interrupt ids 136 to 128 for hart 3\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`enable_4_3::R`].  You can [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`enable_4_3::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`enable_4_3`]
module"]
pub type ENABLE_4_3 = crate::Reg<enable_4_3::ENABLE_4_3_SPEC>;
#[doc = "ENABLE Register for interrupt ids 136 to 128 for hart 3"]
pub mod enable_4_3;
#[doc = "enable_0_4 (rw) register accessor: ENABLE Register for interrupt ids 31 to 0 for hart 4\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`enable_0_4::R`].  You can [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`enable_0_4::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`enable_0_4`]
module"]
pub type ENABLE_0_4 = crate::Reg<enable_0_4::ENABLE_0_4_SPEC>;
#[doc = "ENABLE Register for interrupt ids 31 to 0 for hart 4"]
pub mod enable_0_4;
#[doc = "enable_1_4 (rw) register accessor: ENABLE Register for interrupt ids 63 to 32 for hart 4\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`enable_1_4::R`].  You can [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`enable_1_4::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`enable_1_4`]
module"]
pub type ENABLE_1_4 = crate::Reg<enable_1_4::ENABLE_1_4_SPEC>;
#[doc = "ENABLE Register for interrupt ids 63 to 32 for hart 4"]
pub mod enable_1_4;
#[doc = "enable_2_4 (rw) register accessor: ENABLE Register for interrupt ids 95 to 64 for hart 4\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`enable_2_4::R`].  You can [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`enable_2_4::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`enable_2_4`]
module"]
pub type ENABLE_2_4 = crate::Reg<enable_2_4::ENABLE_2_4_SPEC>;
#[doc = "ENABLE Register for interrupt ids 95 to 64 for hart 4"]
pub mod enable_2_4;
#[doc = "enable_3_4 (rw) register accessor: ENABLE Register for interrupt ids 127 to 96 for hart 4\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`enable_3_4::R`].  You can [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`enable_3_4::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`enable_3_4`]
module"]
pub type ENABLE_3_4 = crate::Reg<enable_3_4::ENABLE_3_4_SPEC>;
#[doc = "ENABLE Register for interrupt ids 127 to 96 for hart 4"]
pub mod enable_3_4;
#[doc = "enable_4_4 (rw) register accessor: ENABLE Register for interrupt ids 136 to 128 for hart 4\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`enable_4_4::R`].  You can [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`enable_4_4::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`enable_4_4`]
module"]
pub type ENABLE_4_4 = crate::Reg<enable_4_4::ENABLE_4_4_SPEC>;
#[doc = "ENABLE Register for interrupt ids 136 to 128 for hart 4"]
pub mod enable_4_4;
#[doc = "threshold_0 (rw) register accessor: PRIORITY THRESHOLD Register for hart 0\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`threshold_0::R`].  You can [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`threshold_0::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`threshold_0`]
module"]
pub type THRESHOLD_0 = crate::Reg<threshold_0::THRESHOLD_0_SPEC>;
#[doc = "PRIORITY THRESHOLD Register for hart 0"]
pub mod threshold_0;
#[doc = "claimplete_0 (rw) register accessor: CLAIM and COMPLETE Register for hart 0\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`claimplete_0::R`].  You can [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`claimplete_0::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`claimplete_0`]
module"]
pub type CLAIMPLETE_0 = crate::Reg<claimplete_0::CLAIMPLETE_0_SPEC>;
#[doc = "CLAIM and COMPLETE Register for hart 0"]
pub mod claimplete_0;
#[doc = "threshold_1 (rw) register accessor: PRIORITY THRESHOLD Register for hart 1\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`threshold_1::R`].  You can [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`threshold_1::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`threshold_1`]
module"]
pub type THRESHOLD_1 = crate::Reg<threshold_1::THRESHOLD_1_SPEC>;
#[doc = "PRIORITY THRESHOLD Register for hart 1"]
pub mod threshold_1;
#[doc = "claimplete_1 (rw) register accessor: CLAIM and COMPLETE Register for hart 1\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`claimplete_1::R`].  You can [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`claimplete_1::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`claimplete_1`]
module"]
pub type CLAIMPLETE_1 = crate::Reg<claimplete_1::CLAIMPLETE_1_SPEC>;
#[doc = "CLAIM and COMPLETE Register for hart 1"]
pub mod claimplete_1;
#[doc = "threshold_2 (rw) register accessor: PRIORITY THRESHOLD Register for hart 2\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`threshold_2::R`].  You can [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`threshold_2::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`threshold_2`]
module"]
pub type THRESHOLD_2 = crate::Reg<threshold_2::THRESHOLD_2_SPEC>;
#[doc = "PRIORITY THRESHOLD Register for hart 2"]
pub mod threshold_2;
#[doc = "claimplete_2 (rw) register accessor: CLAIM and COMPLETE Register for hart 2\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`claimplete_2::R`].  You can [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`claimplete_2::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`claimplete_2`]
module"]
pub type CLAIMPLETE_2 = crate::Reg<claimplete_2::CLAIMPLETE_2_SPEC>;
#[doc = "CLAIM and COMPLETE Register for hart 2"]
pub mod claimplete_2;
#[doc = "threshold_3 (rw) register accessor: PRIORITY THRESHOLD Register for hart 3\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`threshold_3::R`].  You can [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`threshold_3::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`threshold_3`]
module"]
pub type THRESHOLD_3 = crate::Reg<threshold_3::THRESHOLD_3_SPEC>;
#[doc = "PRIORITY THRESHOLD Register for hart 3"]
pub mod threshold_3;
#[doc = "claimplete_3 (rw) register accessor: CLAIM and COMPLETE Register for hart 3\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`claimplete_3::R`].  You can [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`claimplete_3::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`claimplete_3`]
module"]
pub type CLAIMPLETE_3 = crate::Reg<claimplete_3::CLAIMPLETE_3_SPEC>;
#[doc = "CLAIM and COMPLETE Register for hart 3"]
pub mod claimplete_3;
#[doc = "threshold_4 (rw) register accessor: PRIORITY THRESHOLD Register for hart 4\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`threshold_4::R`].  You can [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`threshold_4::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`threshold_4`]
module"]
pub type THRESHOLD_4 = crate::Reg<threshold_4::THRESHOLD_4_SPEC>;
#[doc = "PRIORITY THRESHOLD Register for hart 4"]
pub mod threshold_4;
#[doc = "claimplete_4 (rw) register accessor: CLAIM and COMPLETE Register for hart 4\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`claimplete_4::R`].  You can [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`claimplete_4::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`claimplete_4`]
module"]
pub type CLAIMPLETE_4 = crate::Reg<claimplete_4::CLAIMPLETE_4_SPEC>;
#[doc = "CLAIM and COMPLETE Register for hart 4"]
pub mod claimplete_4;
