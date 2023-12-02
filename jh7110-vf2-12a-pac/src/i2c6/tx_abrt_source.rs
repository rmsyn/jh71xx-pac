#[doc = "Register `tx_abrt_source` reader"]
pub type R = crate::R<TX_ABRT_SOURCE_SPEC>;
#[doc = "Register `tx_abrt_source` writer"]
pub type W = crate::W<TX_ABRT_SOURCE_SPEC>;
#[doc = "Field `b7_addr_noack` reader - b7_addr_noack"]
pub type B7_ADDR_NOACK_R = crate::BitReader;
#[doc = "Field `b10_addr1_noack` reader - b10_addr1_noack"]
pub type B10_ADDR1_NOACK_R = crate::BitReader;
#[doc = "Field `b10_addr2_noack` reader - b10_addr2_noack"]
pub type B10_ADDR2_NOACK_R = crate::BitReader;
#[doc = "Field `txdata_noack` reader - txdata_noack"]
pub type TXDATA_NOACK_R = crate::BitReader;
#[doc = "Field `gcall_noack` reader - gcall_noack"]
pub type GCALL_NOACK_R = crate::BitReader;
#[doc = "Field `gcall_read` reader - gcall_read"]
pub type GCALL_READ_R = crate::BitReader;
#[doc = "Field `sbyte_ackdet` reader - sbyte_ackdet"]
pub type SBYTE_ACKDET_R = crate::BitReader;
#[doc = "Field `sbyte_norstrt` reader - sbyte_norstrt"]
pub type SBYTE_NORSTRT_R = crate::BitReader;
#[doc = "Field `b10_rd_norstrt` reader - b10_rd_norstrt"]
pub type B10_RD_NORSTRT_R = crate::BitReader;
#[doc = "Field `master_dis` reader - master_dis"]
pub type MASTER_DIS_R = crate::BitReader;
#[doc = "Field `arb_lost` reader - arb_lost"]
pub type ARB_LOST_R = crate::BitReader;
#[doc = "Field `slave_flush_txfifo` reader - slave_flush_txfifo"]
pub type SLAVE_FLUSH_TXFIFO_R = crate::BitReader;
#[doc = "Field `slave_arblost` reader - slave_arblost"]
pub type SLAVE_ARBLOST_R = crate::BitReader;
#[doc = "Field `slave_rd_intx` reader - slave_rd_intx"]
pub type SLAVE_RD_INTX_R = crate::BitReader;
impl R {
    #[doc = "Bit 0 - b7_addr_noack"]
    #[inline(always)]
    pub fn b7_addr_noack(&self) -> B7_ADDR_NOACK_R {
        B7_ADDR_NOACK_R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - b10_addr1_noack"]
    #[inline(always)]
    pub fn b10_addr1_noack(&self) -> B10_ADDR1_NOACK_R {
        B10_ADDR1_NOACK_R::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - b10_addr2_noack"]
    #[inline(always)]
    pub fn b10_addr2_noack(&self) -> B10_ADDR2_NOACK_R {
        B10_ADDR2_NOACK_R::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3 - txdata_noack"]
    #[inline(always)]
    pub fn txdata_noack(&self) -> TXDATA_NOACK_R {
        TXDATA_NOACK_R::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bit 4 - gcall_noack"]
    #[inline(always)]
    pub fn gcall_noack(&self) -> GCALL_NOACK_R {
        GCALL_NOACK_R::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bit 5 - gcall_read"]
    #[inline(always)]
    pub fn gcall_read(&self) -> GCALL_READ_R {
        GCALL_READ_R::new(((self.bits >> 5) & 1) != 0)
    }
    #[doc = "Bit 7 - sbyte_ackdet"]
    #[inline(always)]
    pub fn sbyte_ackdet(&self) -> SBYTE_ACKDET_R {
        SBYTE_ACKDET_R::new(((self.bits >> 7) & 1) != 0)
    }
    #[doc = "Bit 9 - sbyte_norstrt"]
    #[inline(always)]
    pub fn sbyte_norstrt(&self) -> SBYTE_NORSTRT_R {
        SBYTE_NORSTRT_R::new(((self.bits >> 9) & 1) != 0)
    }
    #[doc = "Bit 10 - b10_rd_norstrt"]
    #[inline(always)]
    pub fn b10_rd_norstrt(&self) -> B10_RD_NORSTRT_R {
        B10_RD_NORSTRT_R::new(((self.bits >> 10) & 1) != 0)
    }
    #[doc = "Bit 11 - master_dis"]
    #[inline(always)]
    pub fn master_dis(&self) -> MASTER_DIS_R {
        MASTER_DIS_R::new(((self.bits >> 11) & 1) != 0)
    }
    #[doc = "Bit 12 - arb_lost"]
    #[inline(always)]
    pub fn arb_lost(&self) -> ARB_LOST_R {
        ARB_LOST_R::new(((self.bits >> 12) & 1) != 0)
    }
    #[doc = "Bit 13 - slave_flush_txfifo"]
    #[inline(always)]
    pub fn slave_flush_txfifo(&self) -> SLAVE_FLUSH_TXFIFO_R {
        SLAVE_FLUSH_TXFIFO_R::new(((self.bits >> 13) & 1) != 0)
    }
    #[doc = "Bit 14 - slave_arblost"]
    #[inline(always)]
    pub fn slave_arblost(&self) -> SLAVE_ARBLOST_R {
        SLAVE_ARBLOST_R::new(((self.bits >> 14) & 1) != 0)
    }
    #[doc = "Bit 15 - slave_rd_intx"]
    #[inline(always)]
    pub fn slave_rd_intx(&self) -> SLAVE_RD_INTX_R {
        SLAVE_RD_INTX_R::new(((self.bits >> 15) & 1) != 0)
    }
}
impl W {
    #[doc = r" Writes raw bits to the register."]
    #[doc = r""]
    #[doc = r" # Safety"]
    #[doc = r""]
    #[doc = r" Passing incorrect value can cause undefined behaviour. See reference manual"]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.bits = bits;
        self
    }
}
#[doc = "DesignWare I2C TX Abort Source\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`tx_abrt_source::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`tx_abrt_source::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct TX_ABRT_SOURCE_SPEC;
impl crate::RegisterSpec for TX_ABRT_SOURCE_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`tx_abrt_source::R`](R) reader structure"]
impl crate::Readable for TX_ABRT_SOURCE_SPEC {}
#[doc = "`write(|w| ..)` method takes [`tx_abrt_source::W`](W) writer structure"]
impl crate::Writable for TX_ABRT_SOURCE_SPEC {
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets tx_abrt_source to value 0"]
impl crate::Resettable for TX_ABRT_SOURCE_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
