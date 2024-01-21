#[doc = r"Register block"]
#[repr(C)]
pub struct RegisterBlock {
    csr: [CSR; 512],
    base: [BASE; 512],
    ac_base: [AC_BASE; 2048],
}
impl RegisterBlock {
    #[doc = "0x00..0x800 - DDR Memory Control PHY CSR"]
    #[inline(always)]
    pub const fn csr(&self, n: usize) -> &CSR {
        &self.csr[n]
    }
    #[doc = "Iterator for array of:"]
    #[doc = "0x00..0x800 - DDR Memory Control PHY CSR"]
    #[inline(always)]
    pub fn csr_iter(&self) -> impl Iterator<Item = &CSR> {
        self.csr.iter()
    }
    #[doc = "0x800..0x1000 - DDR Memory Control PHY Base register"]
    #[inline(always)]
    pub const fn base(&self, n: usize) -> &BASE {
        &self.base[n]
    }
    #[doc = "Iterator for array of:"]
    #[doc = "0x800..0x1000 - DDR Memory Control PHY Base register"]
    #[inline(always)]
    pub fn base_iter(&self) -> impl Iterator<Item = &BASE> {
        self.base.iter()
    }
    #[doc = "0x1000..0x3000 - DDR Memory Control PHY AC Base register"]
    #[inline(always)]
    pub const fn ac_base(&self, n: usize) -> &AC_BASE {
        &self.ac_base[n]
    }
    #[doc = "Iterator for array of:"]
    #[doc = "0x1000..0x3000 - DDR Memory Control PHY AC Base register"]
    #[inline(always)]
    pub fn ac_base_iter(&self) -> impl Iterator<Item = &AC_BASE> {
        self.ac_base.iter()
    }
}
#[doc = "csr (rw) register accessor: DDR Memory Control PHY CSR\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`csr::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`csr::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@csr`]
module"]
pub type CSR = crate::Reg<csr::CSR_SPEC>;
#[doc = "DDR Memory Control PHY CSR"]
pub mod csr;
#[doc = "base (rw) register accessor: DDR Memory Control PHY Base register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`base::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`base::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@base`]
module"]
pub type BASE = crate::Reg<base::BASE_SPEC>;
#[doc = "DDR Memory Control PHY Base register"]
pub mod base;
#[doc = "ac_base (rw) register accessor: DDR Memory Control PHY AC Base register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`ac_base::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`ac_base::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@ac_base`]
module"]
pub type AC_BASE = crate::Reg<ac_base::AC_BASE_SPEC>;
#[doc = "DDR Memory Control PHY AC Base register"]
pub mod ac_base;
