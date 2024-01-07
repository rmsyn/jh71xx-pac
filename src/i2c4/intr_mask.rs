#[doc = "Register `intr_mask` reader"]
pub type R = crate::R<INTR_MASK_SPEC>;
#[doc = "Register `intr_mask` writer"]
pub type W = crate::W<INTR_MASK_SPEC>;
#[doc = "Field `rx_under` reader - RX FIFO Underrun"]
pub type RX_UNDER_R = crate::BitReader;
#[doc = "Field `rx_under` writer - RX FIFO Underrun"]
pub type RX_UNDER_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `rx_over` reader - RX FIFO Overrun"]
pub type RX_OVER_R = crate::BitReader;
#[doc = "Field `rx_over` writer - RX FIFO Overrun"]
pub type RX_OVER_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `rx_full` reader - RX FIFO Full"]
pub type RX_FULL_R = crate::BitReader;
#[doc = "Field `rx_full` writer - RX FIFO Full"]
pub type RX_FULL_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `tx_over` reader - TX FIFO Overrun"]
pub type TX_OVER_R = crate::BitReader;
#[doc = "Field `tx_over` writer - TX FIFO Overrun"]
pub type TX_OVER_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `tx_empty` reader - TX FIFO Empty"]
pub type TX_EMPTY_R = crate::BitReader;
#[doc = "Field `tx_empty` writer - TX FIFO Empty"]
pub type TX_EMPTY_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `rd_req` reader - Read Request"]
pub type RD_REQ_R = crate::BitReader;
#[doc = "Field `rd_req` writer - Read Request"]
pub type RD_REQ_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `tx_abrt` reader - TX Abort"]
pub type TX_ABRT_R = crate::BitReader;
#[doc = "Field `tx_abrt` writer - TX Abort"]
pub type TX_ABRT_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `rx_done` reader - RX Done"]
pub type RX_DONE_R = crate::BitReader;
#[doc = "Field `rx_done` writer - RX Done"]
pub type RX_DONE_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `activity` reader - Activity"]
pub type ACTIVITY_R = crate::BitReader;
#[doc = "Field `activity` writer - Activity"]
pub type ACTIVITY_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `stop_det` reader - Stop DET"]
pub type STOP_DET_R = crate::BitReader;
#[doc = "Field `stop_det` writer - Stop DET"]
pub type STOP_DET_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `start_det` reader - Start DET"]
pub type START_DET_R = crate::BitReader;
#[doc = "Field `start_det` writer - Start DET"]
pub type START_DET_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `gen_call` reader - General Call"]
pub type GEN_CALL_R = crate::BitReader;
#[doc = "Field `gen_call` writer - General Call"]
pub type GEN_CALL_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `restart_det` reader - Restart DET"]
pub type RESTART_DET_R = crate::BitReader;
#[doc = "Field `restart_det` writer - Restart DET"]
pub type RESTART_DET_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `mst_on_hold` reader - Master on Hold"]
pub type MST_ON_HOLD_R = crate::BitReader;
#[doc = "Field `mst_on_hold` writer - Master on Hold"]
pub type MST_ON_HOLD_W<'a, REG> = crate::BitWriter<'a, REG>;
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
    #[doc = "Bit 0 - RX FIFO Underrun"]
    #[inline(always)]
    #[must_use]
    pub fn rx_under(&mut self) -> RX_UNDER_W<INTR_MASK_SPEC> {
        RX_UNDER_W::new(self, 0)
    }
    #[doc = "Bit 1 - RX FIFO Overrun"]
    #[inline(always)]
    #[must_use]
    pub fn rx_over(&mut self) -> RX_OVER_W<INTR_MASK_SPEC> {
        RX_OVER_W::new(self, 1)
    }
    #[doc = "Bit 2 - RX FIFO Full"]
    #[inline(always)]
    #[must_use]
    pub fn rx_full(&mut self) -> RX_FULL_W<INTR_MASK_SPEC> {
        RX_FULL_W::new(self, 2)
    }
    #[doc = "Bit 3 - TX FIFO Overrun"]
    #[inline(always)]
    #[must_use]
    pub fn tx_over(&mut self) -> TX_OVER_W<INTR_MASK_SPEC> {
        TX_OVER_W::new(self, 3)
    }
    #[doc = "Bit 4 - TX FIFO Empty"]
    #[inline(always)]
    #[must_use]
    pub fn tx_empty(&mut self) -> TX_EMPTY_W<INTR_MASK_SPEC> {
        TX_EMPTY_W::new(self, 4)
    }
    #[doc = "Bit 5 - Read Request"]
    #[inline(always)]
    #[must_use]
    pub fn rd_req(&mut self) -> RD_REQ_W<INTR_MASK_SPEC> {
        RD_REQ_W::new(self, 5)
    }
    #[doc = "Bit 6 - TX Abort"]
    #[inline(always)]
    #[must_use]
    pub fn tx_abrt(&mut self) -> TX_ABRT_W<INTR_MASK_SPEC> {
        TX_ABRT_W::new(self, 6)
    }
    #[doc = "Bit 7 - RX Done"]
    #[inline(always)]
    #[must_use]
    pub fn rx_done(&mut self) -> RX_DONE_W<INTR_MASK_SPEC> {
        RX_DONE_W::new(self, 7)
    }
    #[doc = "Bit 8 - Activity"]
    #[inline(always)]
    #[must_use]
    pub fn activity(&mut self) -> ACTIVITY_W<INTR_MASK_SPEC> {
        ACTIVITY_W::new(self, 8)
    }
    #[doc = "Bit 9 - Stop DET"]
    #[inline(always)]
    #[must_use]
    pub fn stop_det(&mut self) -> STOP_DET_W<INTR_MASK_SPEC> {
        STOP_DET_W::new(self, 9)
    }
    #[doc = "Bit 10 - Start DET"]
    #[inline(always)]
    #[must_use]
    pub fn start_det(&mut self) -> START_DET_W<INTR_MASK_SPEC> {
        START_DET_W::new(self, 10)
    }
    #[doc = "Bit 11 - General Call"]
    #[inline(always)]
    #[must_use]
    pub fn gen_call(&mut self) -> GEN_CALL_W<INTR_MASK_SPEC> {
        GEN_CALL_W::new(self, 11)
    }
    #[doc = "Bit 12 - Restart DET"]
    #[inline(always)]
    #[must_use]
    pub fn restart_det(&mut self) -> RESTART_DET_W<INTR_MASK_SPEC> {
        RESTART_DET_W::new(self, 12)
    }
    #[doc = "Bit 13 - Master on Hold"]
    #[inline(always)]
    #[must_use]
    pub fn mst_on_hold(&mut self) -> MST_ON_HOLD_W<INTR_MASK_SPEC> {
        MST_ON_HOLD_W::new(self, 13)
    }
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
#[doc = "DesignWare I2C Interrupt Mask\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`intr_mask::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`intr_mask::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct INTR_MASK_SPEC;
impl crate::RegisterSpec for INTR_MASK_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`intr_mask::R`](R) reader structure"]
impl crate::Readable for INTR_MASK_SPEC {}
#[doc = "`write(|w| ..)` method takes [`intr_mask::W`](W) writer structure"]
impl crate::Writable for INTR_MASK_SPEC {
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets intr_mask to value 0"]
impl crate::Resettable for INTR_MASK_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
