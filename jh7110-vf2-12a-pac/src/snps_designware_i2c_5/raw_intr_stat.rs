#[doc = "Register `raw_intr_stat` reader"]
pub type R = crate::R<RAW_INTR_STAT_SPEC>;
#[doc = "Register `raw_intr_stat` writer"]
pub type W = crate::W<RAW_INTR_STAT_SPEC>;
#[doc = "Field `rx_under` reader - RX FIFO Underrun"]
pub type RX_UNDER_R = crate::BitReader;
#[doc = "Field `rx_over` reader - RX FIFO Overrun"]
pub type RX_OVER_R = crate::BitReader;
#[doc = "Field `rx_full` reader - RX FIFO Full"]
pub type RX_FULL_R = crate::BitReader;
#[doc = "Field `tx_over` reader - TX FIFO Overrun"]
pub type TX_OVER_R = crate::BitReader;
#[doc = "Field `tx_empty` reader - TX FIFO Empty"]
pub type TX_EMPTY_R = crate::BitReader;
#[doc = "Field `rd_req` reader - Read Request"]
pub type RD_REQ_R = crate::BitReader;
#[doc = "Field `tx_abrt` reader - TX Abort"]
pub type TX_ABRT_R = crate::BitReader;
#[doc = "Field `rx_done` reader - RX Done"]
pub type RX_DONE_R = crate::BitReader;
#[doc = "Field `activity` reader - Activity"]
pub type ACTIVITY_R = crate::BitReader;
#[doc = "Field `stop_det` reader - Stop DET"]
pub type STOP_DET_R = crate::BitReader;
#[doc = "Field `start_det` reader - Start DET"]
pub type START_DET_R = crate::BitReader;
#[doc = "Field `gen_call` reader - General Call"]
pub type GEN_CALL_R = crate::BitReader;
#[doc = "Field `restart_det` reader - Restart DET"]
pub type RESTART_DET_R = crate::BitReader;
#[doc = "Field `mst_on_hold` reader - Master on Hold"]
pub type MST_ON_HOLD_R = crate::BitReader;
impl R {
    #[doc = "Bit 0 - RX FIFO Underrun"]
    #[inline(always)]
    pub fn rx_under(&self) -> RX_UNDER_R {
        RX_UNDER_R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - RX FIFO Overrun"]
    #[inline(always)]
    pub fn rx_over(&self) -> RX_OVER_R {
        RX_OVER_R::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - RX FIFO Full"]
    #[inline(always)]
    pub fn rx_full(&self) -> RX_FULL_R {
        RX_FULL_R::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3 - TX FIFO Overrun"]
    #[inline(always)]
    pub fn tx_over(&self) -> TX_OVER_R {
        TX_OVER_R::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bit 4 - TX FIFO Empty"]
    #[inline(always)]
    pub fn tx_empty(&self) -> TX_EMPTY_R {
        TX_EMPTY_R::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bit 5 - Read Request"]
    #[inline(always)]
    pub fn rd_req(&self) -> RD_REQ_R {
        RD_REQ_R::new(((self.bits >> 5) & 1) != 0)
    }
    #[doc = "Bit 6 - TX Abort"]
    #[inline(always)]
    pub fn tx_abrt(&self) -> TX_ABRT_R {
        TX_ABRT_R::new(((self.bits >> 6) & 1) != 0)
    }
    #[doc = "Bit 7 - RX Done"]
    #[inline(always)]
    pub fn rx_done(&self) -> RX_DONE_R {
        RX_DONE_R::new(((self.bits >> 7) & 1) != 0)
    }
    #[doc = "Bit 8 - Activity"]
    #[inline(always)]
    pub fn activity(&self) -> ACTIVITY_R {
        ACTIVITY_R::new(((self.bits >> 8) & 1) != 0)
    }
    #[doc = "Bit 9 - Stop DET"]
    #[inline(always)]
    pub fn stop_det(&self) -> STOP_DET_R {
        STOP_DET_R::new(((self.bits >> 9) & 1) != 0)
    }
    #[doc = "Bit 10 - Start DET"]
    #[inline(always)]
    pub fn start_det(&self) -> START_DET_R {
        START_DET_R::new(((self.bits >> 10) & 1) != 0)
    }
    #[doc = "Bit 11 - General Call"]
    #[inline(always)]
    pub fn gen_call(&self) -> GEN_CALL_R {
        GEN_CALL_R::new(((self.bits >> 11) & 1) != 0)
    }
    #[doc = "Bit 12 - Restart DET"]
    #[inline(always)]
    pub fn restart_det(&self) -> RESTART_DET_R {
        RESTART_DET_R::new(((self.bits >> 12) & 1) != 0)
    }
    #[doc = "Bit 13 - Master on Hold"]
    #[inline(always)]
    pub fn mst_on_hold(&self) -> MST_ON_HOLD_R {
        MST_ON_HOLD_R::new(((self.bits >> 13) & 1) != 0)
    }
}
impl W {
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.bits = bits;
        self
    }
}
#[doc = "DesignWare I2C Raw Interrupt Status\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`raw_intr_stat::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`raw_intr_stat::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct RAW_INTR_STAT_SPEC;
impl crate::RegisterSpec for RAW_INTR_STAT_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`raw_intr_stat::R`](R) reader structure"]
impl crate::Readable for RAW_INTR_STAT_SPEC {}
#[doc = "`write(|w| ..)` method takes [`raw_intr_stat::W`](W) writer structure"]
impl crate::Writable for RAW_INTR_STAT_SPEC {
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets raw_intr_stat to value 0"]
impl crate::Resettable for RAW_INTR_STAT_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
