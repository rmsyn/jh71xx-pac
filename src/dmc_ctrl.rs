#[doc = r"Register block"]
#[repr(C)]
pub struct RegisterBlock {
    csr: [CSR; 1024],
    sec: [SEC; 2048],
}
impl RegisterBlock {
    #[doc = "0x00..0x1000 - DDR Memory Control CSR register"]
    #[inline(always)]
    pub const fn csr(&self, n: usize) -> &CSR {
        &self.csr[n]
    }
    #[doc = "Iterator for array of:"]
    #[doc = "0x00..0x1000 - DDR Memory Control CSR register"]
    #[inline(always)]
    pub fn csr_iter(&self) -> impl Iterator<Item = &CSR> {
        self.csr.iter()
    }
    #[doc = "0x1000..0x3000 - DDR Memory Control SEC register"]
    #[inline(always)]
    pub const fn sec(&self, n: usize) -> &SEC {
        &self.sec[n]
    }
    #[doc = "Iterator for array of:"]
    #[doc = "0x1000..0x3000 - DDR Memory Control SEC register"]
    #[inline(always)]
    pub fn sec_iter(&self) -> impl Iterator<Item = &SEC> {
        self.sec.iter()
    }
}
#[doc = "csr (rw) register accessor: DDR Memory Control CSR register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`csr::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`csr::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@csr`]
module"]
pub type CSR = crate::Reg<csr::CSR_SPEC>;
#[doc = "DDR Memory Control CSR register"]
pub mod csr;
#[doc = "sec (rw) register accessor: DDR Memory Control SEC register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`sec::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`sec::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@sec`]
module"]
pub type SEC = crate::Reg<sec::SEC_SPEC>;
#[doc = "DDR Memory Control SEC register"]
pub mod sec;
