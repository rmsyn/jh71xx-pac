#[doc = "Register `con` reader"]
pub type R = crate::R<CON_SPEC>;
#[doc = "Register `con` writer"]
pub type W = crate::W<CON_SPEC>;
#[doc = "Field `master` reader - I2C Master Connection - 0: Slave, 1: Master"]
pub type MASTER_R = crate::BitReader;
#[doc = "Field `master` writer - I2C Master Connection - 0: Slave, 1: Master"]
pub type MASTER_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O>;
#[doc = "Field `speed` reader - I2C Speed - 01: Standard, 10: Fast, 11: High"]
pub type SPEED_R = crate::FieldReader;
#[doc = "Field `speed` writer - I2C Speed - 01: Standard, 10: Fast, 11: High"]
pub type SPEED_W<'a, REG, const O: u8> = crate::FieldWriter<'a, REG, 2, O>;
#[doc = "Field `slave_10bitaddr` reader - I2C Slave 10-bit Address - 0: False, 1: True"]
pub type SLAVE_10BITADDR_R = crate::BitReader;
#[doc = "Field `slave_10bitaddr` writer - I2C Slave 10-bit Address - 0: False, 1: True"]
pub type SLAVE_10BITADDR_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O>;
#[doc = "Field `master_10bitaddr` reader - I2C Master 10-bit Address - 0: False, 1: True"]
pub type MASTER_10BITADDR_R = crate::BitReader;
#[doc = "Field `master_10bitaddr` writer - I2C Master 10-bit Address - 0: False, 1: True"]
pub type MASTER_10BITADDR_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O>;
#[doc = "Field `restart_en` reader - I2C Restart Enable - 0: False, 1: True"]
pub type RESTART_EN_R = crate::BitReader;
#[doc = "Field `restart_en` writer - I2C Restart Enable - 0: False, 1: True"]
pub type RESTART_EN_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O>;
#[doc = "Field `slave_disable` reader - I2C Slave Disable - 0: False, 1: True"]
pub type SLAVE_DISABLE_R = crate::BitReader;
#[doc = "Field `slave_disable` writer - I2C Slave Disable - 0: False, 1: True"]
pub type SLAVE_DISABLE_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O>;
#[doc = "Field `stop_det_ifaddressed` reader - I2C Stop DET If Addressed - 0: False, 1: True"]
pub type STOP_DET_IFADDRESSED_R = crate::BitReader;
#[doc = "Field `stop_det_ifaddressed` writer - I2C Stop DET If Addressed - 0: False, 1: True"]
pub type STOP_DET_IFADDRESSED_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O>;
#[doc = "Field `tx_empty_ctrl` reader - I2C TX Empty Control - 0: False, 1: True"]
pub type TX_EMPTY_CTRL_R = crate::BitReader;
#[doc = "Field `tx_empty_ctrl` writer - I2C TX Empty Control - 0: False, 1: True"]
pub type TX_EMPTY_CTRL_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O>;
#[doc = "Field `rx_fifo_full_hld_ctrl` reader - I2C RX FIFO Full Hold Control - 0: False, 1: True"]
pub type RX_FIFO_FULL_HLD_CTRL_R = crate::BitReader;
#[doc = "Field `rx_fifo_full_hld_ctrl` writer - I2C RX FIFO Full Hold Control - 0: False, 1: True"]
pub type RX_FIFO_FULL_HLD_CTRL_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O>;
#[doc = "Field `bus_clear_ctrl` reader - I2C Bus Clear Control - 0: False, 1: True"]
pub type BUS_CLEAR_CTRL_R = crate::BitReader;
#[doc = "Field `bus_clear_ctrl` writer - I2C Bus Clear Control - 0: False, 1: True"]
pub type BUS_CLEAR_CTRL_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O>;
impl R {
    #[doc = "Bit 0 - I2C Master Connection - 0: Slave, 1: Master"]
    #[inline(always)]
    pub fn master(&self) -> MASTER_R {
        MASTER_R::new((self.bits & 1) != 0)
    }
    #[doc = "Bits 1:2 - I2C Speed - 01: Standard, 10: Fast, 11: High"]
    #[inline(always)]
    pub fn speed(&self) -> SPEED_R {
        SPEED_R::new(((self.bits >> 1) & 3) as u8)
    }
    #[doc = "Bit 3 - I2C Slave 10-bit Address - 0: False, 1: True"]
    #[inline(always)]
    pub fn slave_10bitaddr(&self) -> SLAVE_10BITADDR_R {
        SLAVE_10BITADDR_R::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bit 4 - I2C Master 10-bit Address - 0: False, 1: True"]
    #[inline(always)]
    pub fn master_10bitaddr(&self) -> MASTER_10BITADDR_R {
        MASTER_10BITADDR_R::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bit 5 - I2C Restart Enable - 0: False, 1: True"]
    #[inline(always)]
    pub fn restart_en(&self) -> RESTART_EN_R {
        RESTART_EN_R::new(((self.bits >> 5) & 1) != 0)
    }
    #[doc = "Bit 6 - I2C Slave Disable - 0: False, 1: True"]
    #[inline(always)]
    pub fn slave_disable(&self) -> SLAVE_DISABLE_R {
        SLAVE_DISABLE_R::new(((self.bits >> 6) & 1) != 0)
    }
    #[doc = "Bit 7 - I2C Stop DET If Addressed - 0: False, 1: True"]
    #[inline(always)]
    pub fn stop_det_ifaddressed(&self) -> STOP_DET_IFADDRESSED_R {
        STOP_DET_IFADDRESSED_R::new(((self.bits >> 7) & 1) != 0)
    }
    #[doc = "Bit 8 - I2C TX Empty Control - 0: False, 1: True"]
    #[inline(always)]
    pub fn tx_empty_ctrl(&self) -> TX_EMPTY_CTRL_R {
        TX_EMPTY_CTRL_R::new(((self.bits >> 8) & 1) != 0)
    }
    #[doc = "Bit 9 - I2C RX FIFO Full Hold Control - 0: False, 1: True"]
    #[inline(always)]
    pub fn rx_fifo_full_hld_ctrl(&self) -> RX_FIFO_FULL_HLD_CTRL_R {
        RX_FIFO_FULL_HLD_CTRL_R::new(((self.bits >> 9) & 1) != 0)
    }
    #[doc = "Bit 11 - I2C Bus Clear Control - 0: False, 1: True"]
    #[inline(always)]
    pub fn bus_clear_ctrl(&self) -> BUS_CLEAR_CTRL_R {
        BUS_CLEAR_CTRL_R::new(((self.bits >> 11) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - I2C Master Connection - 0: Slave, 1: Master"]
    #[inline(always)]
    #[must_use]
    pub fn master(&mut self) -> MASTER_W<CON_SPEC, 0> {
        MASTER_W::new(self)
    }
    #[doc = "Bits 1:2 - I2C Speed - 01: Standard, 10: Fast, 11: High"]
    #[inline(always)]
    #[must_use]
    pub fn speed(&mut self) -> SPEED_W<CON_SPEC, 1> {
        SPEED_W::new(self)
    }
    #[doc = "Bit 3 - I2C Slave 10-bit Address - 0: False, 1: True"]
    #[inline(always)]
    #[must_use]
    pub fn slave_10bitaddr(&mut self) -> SLAVE_10BITADDR_W<CON_SPEC, 3> {
        SLAVE_10BITADDR_W::new(self)
    }
    #[doc = "Bit 4 - I2C Master 10-bit Address - 0: False, 1: True"]
    #[inline(always)]
    #[must_use]
    pub fn master_10bitaddr(&mut self) -> MASTER_10BITADDR_W<CON_SPEC, 4> {
        MASTER_10BITADDR_W::new(self)
    }
    #[doc = "Bit 5 - I2C Restart Enable - 0: False, 1: True"]
    #[inline(always)]
    #[must_use]
    pub fn restart_en(&mut self) -> RESTART_EN_W<CON_SPEC, 5> {
        RESTART_EN_W::new(self)
    }
    #[doc = "Bit 6 - I2C Slave Disable - 0: False, 1: True"]
    #[inline(always)]
    #[must_use]
    pub fn slave_disable(&mut self) -> SLAVE_DISABLE_W<CON_SPEC, 6> {
        SLAVE_DISABLE_W::new(self)
    }
    #[doc = "Bit 7 - I2C Stop DET If Addressed - 0: False, 1: True"]
    #[inline(always)]
    #[must_use]
    pub fn stop_det_ifaddressed(&mut self) -> STOP_DET_IFADDRESSED_W<CON_SPEC, 7> {
        STOP_DET_IFADDRESSED_W::new(self)
    }
    #[doc = "Bit 8 - I2C TX Empty Control - 0: False, 1: True"]
    #[inline(always)]
    #[must_use]
    pub fn tx_empty_ctrl(&mut self) -> TX_EMPTY_CTRL_W<CON_SPEC, 8> {
        TX_EMPTY_CTRL_W::new(self)
    }
    #[doc = "Bit 9 - I2C RX FIFO Full Hold Control - 0: False, 1: True"]
    #[inline(always)]
    #[must_use]
    pub fn rx_fifo_full_hld_ctrl(&mut self) -> RX_FIFO_FULL_HLD_CTRL_W<CON_SPEC, 9> {
        RX_FIFO_FULL_HLD_CTRL_W::new(self)
    }
    #[doc = "Bit 11 - I2C Bus Clear Control - 0: False, 1: True"]
    #[inline(always)]
    #[must_use]
    pub fn bus_clear_ctrl(&mut self) -> BUS_CLEAR_CTRL_W<CON_SPEC, 11> {
        BUS_CLEAR_CTRL_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.bits = bits;
        self
    }
}
#[doc = "DesignWare I2C CON\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`con::R`](R).  You can [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`con::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct CON_SPEC;
impl crate::RegisterSpec for CON_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`con::R`](R) reader structure"]
impl crate::Readable for CON_SPEC {}
#[doc = "`write(|w| ..)` method takes [`con::W`](W) writer structure"]
impl crate::Writable for CON_SPEC {
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
