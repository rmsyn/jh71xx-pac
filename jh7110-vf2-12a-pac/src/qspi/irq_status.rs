#[doc = "Register `irq_status` reader"]
pub type R = crate::R<IRQ_STATUS_SPEC>;
#[doc = "Register `irq_status` writer"]
pub type W = crate::W<IRQ_STATUS_SPEC>;
#[doc = "Field `mode_err` reader - Mode error interrupt"]
pub type MODE_ERR_R = crate::BitReader;
#[doc = "Field `mode_err` writer - Mode error interrupt"]
pub type MODE_ERR_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `underflow` reader - Buffer underflow interrupt"]
pub type UNDERFLOW_R = crate::BitReader;
#[doc = "Field `underflow` writer - Buffer underflow interrupt"]
pub type UNDERFLOW_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `ind_comp` reader - Indirect computation interrupt"]
pub type IND_COMP_R = crate::BitReader;
#[doc = "Field `ind_comp` writer - Indirect computation interrupt"]
pub type IND_COMP_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `ind_rd_reject` reader - Indirect read rejection interrupt"]
pub type IND_RD_REJECT_R = crate::BitReader;
#[doc = "Field `ind_rd_reject` writer - Indirect read rejection interrupt"]
pub type IND_RD_REJECT_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `wr_protected_err` reader - Write protected error interrupt"]
pub type WR_PROTECTED_ERR_R = crate::BitReader;
#[doc = "Field `wr_protected_err` writer - Write protected error interrupt"]
pub type WR_PROTECTED_ERR_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `illegal_ahb_err` reader - Illegal AHB clock error interrupt"]
pub type ILLEGAL_AHB_ERR_R = crate::BitReader;
#[doc = "Field `illegal_ahb_err` writer - Illegal AHB clock error interrupt"]
pub type ILLEGAL_AHB_ERR_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `watermark` reader - Watermark interrupt"]
pub type WATERMARK_R = crate::BitReader;
#[doc = "Field `watermark` writer - Watermark interrupt"]
pub type WATERMARK_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `ind_sram_full` reader - Indirect SRAM full interrupt"]
pub type IND_SRAM_FULL_R = crate::BitReader;
#[doc = "Field `ind_sram_full` writer - Indirect SRAM full interrupt"]
pub type IND_SRAM_FULL_W<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bit 0 - Mode error interrupt"]
    #[inline(always)]
    pub fn mode_err(&self) -> MODE_ERR_R {
        MODE_ERR_R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - Buffer underflow interrupt"]
    #[inline(always)]
    pub fn underflow(&self) -> UNDERFLOW_R {
        UNDERFLOW_R::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - Indirect computation interrupt"]
    #[inline(always)]
    pub fn ind_comp(&self) -> IND_COMP_R {
        IND_COMP_R::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3 - Indirect read rejection interrupt"]
    #[inline(always)]
    pub fn ind_rd_reject(&self) -> IND_RD_REJECT_R {
        IND_RD_REJECT_R::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bit 4 - Write protected error interrupt"]
    #[inline(always)]
    pub fn wr_protected_err(&self) -> WR_PROTECTED_ERR_R {
        WR_PROTECTED_ERR_R::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bit 5 - Illegal AHB clock error interrupt"]
    #[inline(always)]
    pub fn illegal_ahb_err(&self) -> ILLEGAL_AHB_ERR_R {
        ILLEGAL_AHB_ERR_R::new(((self.bits >> 5) & 1) != 0)
    }
    #[doc = "Bit 6 - Watermark interrupt"]
    #[inline(always)]
    pub fn watermark(&self) -> WATERMARK_R {
        WATERMARK_R::new(((self.bits >> 6) & 1) != 0)
    }
    #[doc = "Bit 12 - Indirect SRAM full interrupt"]
    #[inline(always)]
    pub fn ind_sram_full(&self) -> IND_SRAM_FULL_R {
        IND_SRAM_FULL_R::new(((self.bits >> 12) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - Mode error interrupt"]
    #[inline(always)]
    #[must_use]
    pub fn mode_err(&mut self) -> MODE_ERR_W<IRQ_STATUS_SPEC> {
        MODE_ERR_W::new(self, 0)
    }
    #[doc = "Bit 1 - Buffer underflow interrupt"]
    #[inline(always)]
    #[must_use]
    pub fn underflow(&mut self) -> UNDERFLOW_W<IRQ_STATUS_SPEC> {
        UNDERFLOW_W::new(self, 1)
    }
    #[doc = "Bit 2 - Indirect computation interrupt"]
    #[inline(always)]
    #[must_use]
    pub fn ind_comp(&mut self) -> IND_COMP_W<IRQ_STATUS_SPEC> {
        IND_COMP_W::new(self, 2)
    }
    #[doc = "Bit 3 - Indirect read rejection interrupt"]
    #[inline(always)]
    #[must_use]
    pub fn ind_rd_reject(&mut self) -> IND_RD_REJECT_W<IRQ_STATUS_SPEC> {
        IND_RD_REJECT_W::new(self, 3)
    }
    #[doc = "Bit 4 - Write protected error interrupt"]
    #[inline(always)]
    #[must_use]
    pub fn wr_protected_err(&mut self) -> WR_PROTECTED_ERR_W<IRQ_STATUS_SPEC> {
        WR_PROTECTED_ERR_W::new(self, 4)
    }
    #[doc = "Bit 5 - Illegal AHB clock error interrupt"]
    #[inline(always)]
    #[must_use]
    pub fn illegal_ahb_err(&mut self) -> ILLEGAL_AHB_ERR_W<IRQ_STATUS_SPEC> {
        ILLEGAL_AHB_ERR_W::new(self, 5)
    }
    #[doc = "Bit 6 - Watermark interrupt"]
    #[inline(always)]
    #[must_use]
    pub fn watermark(&mut self) -> WATERMARK_W<IRQ_STATUS_SPEC> {
        WATERMARK_W::new(self, 6)
    }
    #[doc = "Bit 12 - Indirect SRAM full interrupt"]
    #[inline(always)]
    #[must_use]
    pub fn ind_sram_full(&mut self) -> IND_SRAM_FULL_W<IRQ_STATUS_SPEC> {
        IND_SRAM_FULL_W::new(self, 12)
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
#[doc = "Cadence QSPI IRQ Status\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`irq_status::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`irq_status::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct IRQ_STATUS_SPEC;
impl crate::RegisterSpec for IRQ_STATUS_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`irq_status::R`](R) reader structure"]
impl crate::Readable for IRQ_STATUS_SPEC {}
#[doc = "`write(|w| ..)` method takes [`irq_status::W`](W) writer structure"]
impl crate::Writable for IRQ_STATUS_SPEC {
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets irq_status to value 0x0001_ffff"]
impl crate::Resettable for IRQ_STATUS_SPEC {
    const RESET_VALUE: Self::Ux = 0x0001_ffff;
}
