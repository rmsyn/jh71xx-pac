#[doc = "Register `wr_completion_ctrl` reader"]
pub type R = crate::R<WR_COMPLETION_CTRL_SPEC>;
#[doc = "Register `wr_completion_ctrl` writer"]
pub type W = crate::W<WR_COMPLETION_CTRL_SPEC>;
#[doc = "Field `disable_auto_poll` reader - SPI NAND flashes require the address of the status register to be passed in the Read SR command. Also, some SPI NOR flashes like the Cypress Semper flash expect a 4-byte dummy address in the Read SR command in DTR mode. But this controller does not support address phase in the Read SR command when doing auto-HW polling. So, disable write completion polling on the controller's side. spi-nand and spi-nor will take care of polling the status register."]
pub type DISABLE_AUTO_POLL_R = crate::BitReader;
#[doc = "Field `disable_auto_poll` writer - SPI NAND flashes require the address of the status register to be passed in the Read SR command. Also, some SPI NOR flashes like the Cypress Semper flash expect a 4-byte dummy address in the Read SR command in DTR mode. But this controller does not support address phase in the Read SR command when doing auto-HW polling. So, disable write completion polling on the controller's side. spi-nand and spi-nor will take care of polling the status register."]
pub type DISABLE_AUTO_POLL_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O>;
impl R {
    #[doc = "Bit 14 - SPI NAND flashes require the address of the status register to be passed in the Read SR command. Also, some SPI NOR flashes like the Cypress Semper flash expect a 4-byte dummy address in the Read SR command in DTR mode. But this controller does not support address phase in the Read SR command when doing auto-HW polling. So, disable write completion polling on the controller's side. spi-nand and spi-nor will take care of polling the status register."]
    #[inline(always)]
    pub fn disable_auto_poll(&self) -> DISABLE_AUTO_POLL_R {
        DISABLE_AUTO_POLL_R::new(((self.bits >> 14) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 14 - SPI NAND flashes require the address of the status register to be passed in the Read SR command. Also, some SPI NOR flashes like the Cypress Semper flash expect a 4-byte dummy address in the Read SR command in DTR mode. But this controller does not support address phase in the Read SR command when doing auto-HW polling. So, disable write completion polling on the controller's side. spi-nand and spi-nor will take care of polling the status register."]
    #[inline(always)]
    #[must_use]
    pub fn disable_auto_poll(&mut self) -> DISABLE_AUTO_POLL_W<WR_COMPLETION_CTRL_SPEC, 14> {
        DISABLE_AUTO_POLL_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.bits = bits;
        self
    }
}
#[doc = "Cadence QSPI Write Completion Control\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`wr_completion_ctrl::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`wr_completion_ctrl::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct WR_COMPLETION_CTRL_SPEC;
impl crate::RegisterSpec for WR_COMPLETION_CTRL_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`wr_completion_ctrl::R`](R) reader structure"]
impl crate::Readable for WR_COMPLETION_CTRL_SPEC {}
#[doc = "`write(|w| ..)` method takes [`wr_completion_ctrl::W`](W) writer structure"]
impl crate::Writable for WR_COMPLETION_CTRL_SPEC {
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets wr_completion_ctrl to value 0"]
impl crate::Resettable for WR_COMPLETION_CTRL_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
