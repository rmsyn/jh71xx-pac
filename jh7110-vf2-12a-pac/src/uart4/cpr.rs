#[doc = "Register `cpr` reader"]
pub type R = crate::R<CPR_SPEC>;
#[doc = "Register `cpr` writer"]
pub type W = crate::W<CPR_SPEC>;
#[doc = "Field `apb_data_width` reader - 00 = 8 bits 01 = 16 bits 10 = 32 bits 11 = reserved"]
pub type APB_DATA_WIDTH_R = crate::FieldReader;
#[doc = "Field `afce_mode` reader - 0 = false 1 = true"]
pub type AFCE_MODE_R = crate::BitReader;
#[doc = "Field `thre_mode` reader - 0 = false 1 = true"]
pub type THRE_MODE_R = crate::BitReader;
#[doc = "Field `sir_mode` reader - 0 = false 1 = true"]
pub type SIR_MODE_R = crate::BitReader;
#[doc = "Field `sir_lp_mode` reader - 0 = false 1 = true"]
pub type SIR_LP_MODE_R = crate::BitReader;
#[doc = "Field `additional_feat` reader - 0 = false 1 = true"]
pub type ADDITIONAL_FEAT_R = crate::BitReader;
#[doc = "Field `fifo_access` reader - 0 = false 1 = true"]
pub type FIFO_ACCESS_R = crate::BitReader;
#[doc = "Field `fifo_stat` reader - 0 = false 1 = true"]
pub type FIFO_STAT_R = crate::BitReader;
#[doc = "Field `shadow` reader - 0 = false 1 = true"]
pub type SHADOW_R = crate::BitReader;
#[doc = "Field `uart_add_encoded_params` reader - 0 = false 1 = true"]
pub type UART_ADD_ENCODED_PARAMS_R = crate::BitReader;
#[doc = "Field `dma_extra` reader - 0 = false 1 = true"]
pub type DMA_EXTRA_R = crate::BitReader;
#[doc = "Field `fifo_mode` reader - 0x00 = 0 0x01 = 16 0x02 = 32 to 0x80 = 2048 0x81 - 0xff = reserved"]
pub type FIFO_MODE_R = crate::FieldReader;
impl R {
    #[doc = "Bits 0:1 - 00 = 8 bits 01 = 16 bits 10 = 32 bits 11 = reserved"]
    #[inline(always)]
    pub fn apb_data_width(&self) -> APB_DATA_WIDTH_R {
        APB_DATA_WIDTH_R::new((self.bits & 3) as u8)
    }
    #[doc = "Bit 4 - 0 = false 1 = true"]
    #[inline(always)]
    pub fn afce_mode(&self) -> AFCE_MODE_R {
        AFCE_MODE_R::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bit 5 - 0 = false 1 = true"]
    #[inline(always)]
    pub fn thre_mode(&self) -> THRE_MODE_R {
        THRE_MODE_R::new(((self.bits >> 5) & 1) != 0)
    }
    #[doc = "Bit 6 - 0 = false 1 = true"]
    #[inline(always)]
    pub fn sir_mode(&self) -> SIR_MODE_R {
        SIR_MODE_R::new(((self.bits >> 6) & 1) != 0)
    }
    #[doc = "Bit 7 - 0 = false 1 = true"]
    #[inline(always)]
    pub fn sir_lp_mode(&self) -> SIR_LP_MODE_R {
        SIR_LP_MODE_R::new(((self.bits >> 7) & 1) != 0)
    }
    #[doc = "Bit 8 - 0 = false 1 = true"]
    #[inline(always)]
    pub fn additional_feat(&self) -> ADDITIONAL_FEAT_R {
        ADDITIONAL_FEAT_R::new(((self.bits >> 8) & 1) != 0)
    }
    #[doc = "Bit 9 - 0 = false 1 = true"]
    #[inline(always)]
    pub fn fifo_access(&self) -> FIFO_ACCESS_R {
        FIFO_ACCESS_R::new(((self.bits >> 9) & 1) != 0)
    }
    #[doc = "Bit 10 - 0 = false 1 = true"]
    #[inline(always)]
    pub fn fifo_stat(&self) -> FIFO_STAT_R {
        FIFO_STAT_R::new(((self.bits >> 10) & 1) != 0)
    }
    #[doc = "Bit 11 - 0 = false 1 = true"]
    #[inline(always)]
    pub fn shadow(&self) -> SHADOW_R {
        SHADOW_R::new(((self.bits >> 11) & 1) != 0)
    }
    #[doc = "Bit 12 - 0 = false 1 = true"]
    #[inline(always)]
    pub fn uart_add_encoded_params(&self) -> UART_ADD_ENCODED_PARAMS_R {
        UART_ADD_ENCODED_PARAMS_R::new(((self.bits >> 12) & 1) != 0)
    }
    #[doc = "Bit 13 - 0 = false 1 = true"]
    #[inline(always)]
    pub fn dma_extra(&self) -> DMA_EXTRA_R {
        DMA_EXTRA_R::new(((self.bits >> 13) & 1) != 0)
    }
    #[doc = "Bits 16:23 - 0x00 = 0 0x01 = 16 0x02 = 32 to 0x80 = 2048 0x81 - 0xff = reserved"]
    #[inline(always)]
    pub fn fifo_mode(&self) -> FIFO_MODE_R {
        FIFO_MODE_R::new(((self.bits >> 16) & 0xff) as u8)
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
#[doc = "Component Parameter Register: This register is only valid when the DW_apb_uart is configured to have the Component Parameter register implemented (UART_ADD_ENCODED_PARAMS == YES). If the Component Parameter register is not implemented, this register does not exist and reading from this register address returns zero.\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`cpr::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`cpr::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct CPR_SPEC;
impl crate::RegisterSpec for CPR_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`cpr::R`](R) reader structure"]
impl crate::Readable for CPR_SPEC {}
#[doc = "`write(|w| ..)` method takes [`cpr::W`](W) writer structure"]
impl crate::Writable for CPR_SPEC {
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets cpr to value 0"]
impl crate::Resettable for CPR_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
