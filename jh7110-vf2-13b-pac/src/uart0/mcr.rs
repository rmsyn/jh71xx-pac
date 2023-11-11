#[doc = "Register `mcr` reader"]
pub type R = crate::R<MCR_SPEC>;
#[doc = "Register `mcr` writer"]
pub type W = crate::W<MCR_SPEC>;
#[doc = "Field `dtr` reader - Data Terminal Ready. This is used to directly control the Data Terminal Ready (dtr_n) output. The value written to this location is inverted and driven out on dtr_n, that is: 0 = dtr_n de-asserted (logic 1) 1 = dtr_n asserted (logic 0) The Data Terminal Ready output is used to inform the modem or data set that the UART is ready to establish communications. Note that in Loopback mode (MCR\\[4\\]
set to one), the dtr_n output is held inactive high while the value of this location is internally looped back to an input."]
pub type DTR_R = crate::BitReader;
#[doc = "Field `dtr` writer - Data Terminal Ready. This is used to directly control the Data Terminal Ready (dtr_n) output. The value written to this location is inverted and driven out on dtr_n, that is: 0 = dtr_n de-asserted (logic 1) 1 = dtr_n asserted (logic 0) The Data Terminal Ready output is used to inform the modem or data set that the UART is ready to establish communications. Note that in Loopback mode (MCR\\[4\\]
set to one), the dtr_n output is held inactive high while the value of this location is internally looped back to an input."]
pub type DTR_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O>;
#[doc = "Field `rts` reader - Request to Send. This is used to directly control the Request to Send (rts_n) output. The Request To Send (rts_n) output is used to inform the modem or data set that the UART is ready to exchange data. When Auto RTS Flow Control is not enabled (MCR\\[5\\]
set to zero), the rts_n signal is set low by programming MCR\\[1\\]
(RTS) to a high.In Auto Flow Control, AFCE_MODE == Enabled and active (MCR\\[5\\]
set to one) and FIFOs enable (FCR\\[0\\]
set to one), the rts_n output is controlled in the same way, but is also gated with the receiver FIFO threshold trigger (rts_n is inactive high when above the threshold). The rts_n signal is de-asserted when MCR\\[1\\]
is set low. Note that in Loopback mode (MCR\\[4\\]
set to one), the rts_n output is held inactive high while the value of this location is internally looped back to an input."]
pub type RTS_R = crate::BitReader;
#[doc = "Field `rts` writer - Request to Send. This is used to directly control the Request to Send (rts_n) output. The Request To Send (rts_n) output is used to inform the modem or data set that the UART is ready to exchange data. When Auto RTS Flow Control is not enabled (MCR\\[5\\]
set to zero), the rts_n signal is set low by programming MCR\\[1\\]
(RTS) to a high.In Auto Flow Control, AFCE_MODE == Enabled and active (MCR\\[5\\]
set to one) and FIFOs enable (FCR\\[0\\]
set to one), the rts_n output is controlled in the same way, but is also gated with the receiver FIFO threshold trigger (rts_n is inactive high when above the threshold). The rts_n signal is de-asserted when MCR\\[1\\]
is set low. Note that in Loopback mode (MCR\\[4\\]
set to one), the rts_n output is held inactive high while the value of this location is internally looped back to an input."]
pub type RTS_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O>;
#[doc = "Field `out1` reader - OUT1. This is used to directly control the user-designated Output1 (out1_n) output. The value written to this location is inverted and driven out on out1_n, that is: 0 = out1_n de-asserted (logic 1) 1 = out1_n asserted (logic 0) Note that in Loopback mode (MCR\\[4\\]
set to one), the out1_n output is held inactive high while the value of this location is internally looped back to an input."]
pub type OUT1_R = crate::BitReader;
#[doc = "Field `out1` writer - OUT1. This is used to directly control the user-designated Output1 (out1_n) output. The value written to this location is inverted and driven out on out1_n, that is: 0 = out1_n de-asserted (logic 1) 1 = out1_n asserted (logic 0) Note that in Loopback mode (MCR\\[4\\]
set to one), the out1_n output is held inactive high while the value of this location is internally looped back to an input."]
pub type OUT1_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O>;
#[doc = "Field `out2` reader - OUT2. This is used to directly control the user-designated Output2 (out2_n) output. The value written to this location is inverted and driven out on out2_n, that is: 0 = out2_n de-asserted (logic 1) 1 = out2_n asserted (logic 0) Note that in Loopback mode (MCR\\[4\\]
set to one), the out2_n output is held inactive high while the value of this location is internally looped back to an input."]
pub type OUT2_R = crate::BitReader;
#[doc = "Field `out2` writer - OUT2. This is used to directly control the user-designated Output2 (out2_n) output. The value written to this location is inverted and driven out on out2_n, that is: 0 = out2_n de-asserted (logic 1) 1 = out2_n asserted (logic 0) Note that in Loopback mode (MCR\\[4\\]
set to one), the out2_n output is held inactive high while the value of this location is internally looped back to an input."]
pub type OUT2_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O>;
#[doc = "Field `lb` reader - LoopBack Bit. This is used to put the UART into a diagnostic mode for test purposes. If operating in UART mode (SIR_MODE != Enabled or not active, MCR\\[6\\]
set to zero), data on the sout line is held high, while serial data output is looped back to the sin line, internally. In this mode all the interrupts are fully functional. Also, in loopback mode, the modem control inputs (dsr_n, cts_n, ri_n, dcd_n) are disconnected and the modem control outputs (dtr_n, rts_n, out1_n, out2_n) are looped back to the inputs, internally. If operating in infrared mode (SIR_MODE == Enabled AND active, MCR\\[6\\]
set to one), data on the sir_out_n line is held low, while serial data output is inverted and looped back to the sir_in line"]
pub type LB_R = crate::BitReader;
#[doc = "Field `lb` writer - LoopBack Bit. This is used to put the UART into a diagnostic mode for test purposes. If operating in UART mode (SIR_MODE != Enabled or not active, MCR\\[6\\]
set to zero), data on the sout line is held high, while serial data output is looped back to the sin line, internally. In this mode all the interrupts are fully functional. Also, in loopback mode, the modem control inputs (dsr_n, cts_n, ri_n, dcd_n) are disconnected and the modem control outputs (dtr_n, rts_n, out1_n, out2_n) are looped back to the inputs, internally. If operating in infrared mode (SIR_MODE == Enabled AND active, MCR\\[6\\]
set to one), data on the sir_out_n line is held low, while serial data output is inverted and looped back to the sir_in line"]
pub type LB_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O>;
#[doc = "Field `afce` reader - Auto Flow Control Enable. Writeable only when AFCE_MODE == Enabled, always readable. When FIFOs are enabled and the Auto Flow Control Enable (AFCE) bit is set, Auto Flow Control features are enabled as described in “Auto Flow Control” on page 51. 0 = Auto Flow Control Mode disabled 1 = Auto Flow Control Mode enabled"]
pub type AFCE_R = crate::BitReader;
#[doc = "Field `afce` writer - Auto Flow Control Enable. Writeable only when AFCE_MODE == Enabled, always readable. When FIFOs are enabled and the Auto Flow Control Enable (AFCE) bit is set, Auto Flow Control features are enabled as described in “Auto Flow Control” on page 51. 0 = Auto Flow Control Mode disabled 1 = Auto Flow Control Mode enabled"]
pub type AFCE_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O>;
#[doc = "Field `sire` reader - SIR Mode Enable. Writeable only when SIR_MODE == Enabled, always readable. This is used to enable/disable the IrDA SIR Mode features as described in “IrDA 1.0 SIR Protocol” on page 47. 0 = IrDA SIR Mode disabled 1 = IrDA SIR Mode enabled"]
pub type SIRE_R = crate::BitReader;
#[doc = "Field `sire` writer - SIR Mode Enable. Writeable only when SIR_MODE == Enabled, always readable. This is used to enable/disable the IrDA SIR Mode features as described in “IrDA 1.0 SIR Protocol” on page 47. 0 = IrDA SIR Mode disabled 1 = IrDA SIR Mode enabled"]
pub type SIRE_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O>;
impl R {
    #[doc = "Bit 0 - Data Terminal Ready. This is used to directly control the Data Terminal Ready (dtr_n) output. The value written to this location is inverted and driven out on dtr_n, that is: 0 = dtr_n de-asserted (logic 1) 1 = dtr_n asserted (logic 0) The Data Terminal Ready output is used to inform the modem or data set that the UART is ready to establish communications. Note that in Loopback mode (MCR\\[4\\]
set to one), the dtr_n output is held inactive high while the value of this location is internally looped back to an input."]
    #[inline(always)]
    pub fn dtr(&self) -> DTR_R {
        DTR_R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - Request to Send. This is used to directly control the Request to Send (rts_n) output. The Request To Send (rts_n) output is used to inform the modem or data set that the UART is ready to exchange data. When Auto RTS Flow Control is not enabled (MCR\\[5\\]
set to zero), the rts_n signal is set low by programming MCR\\[1\\]
(RTS) to a high.In Auto Flow Control, AFCE_MODE == Enabled and active (MCR\\[5\\]
set to one) and FIFOs enable (FCR\\[0\\]
set to one), the rts_n output is controlled in the same way, but is also gated with the receiver FIFO threshold trigger (rts_n is inactive high when above the threshold). The rts_n signal is de-asserted when MCR\\[1\\]
is set low. Note that in Loopback mode (MCR\\[4\\]
set to one), the rts_n output is held inactive high while the value of this location is internally looped back to an input."]
    #[inline(always)]
    pub fn rts(&self) -> RTS_R {
        RTS_R::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - OUT1. This is used to directly control the user-designated Output1 (out1_n) output. The value written to this location is inverted and driven out on out1_n, that is: 0 = out1_n de-asserted (logic 1) 1 = out1_n asserted (logic 0) Note that in Loopback mode (MCR\\[4\\]
set to one), the out1_n output is held inactive high while the value of this location is internally looped back to an input."]
    #[inline(always)]
    pub fn out1(&self) -> OUT1_R {
        OUT1_R::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3 - OUT2. This is used to directly control the user-designated Output2 (out2_n) output. The value written to this location is inverted and driven out on out2_n, that is: 0 = out2_n de-asserted (logic 1) 1 = out2_n asserted (logic 0) Note that in Loopback mode (MCR\\[4\\]
set to one), the out2_n output is held inactive high while the value of this location is internally looped back to an input."]
    #[inline(always)]
    pub fn out2(&self) -> OUT2_R {
        OUT2_R::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bit 4 - LoopBack Bit. This is used to put the UART into a diagnostic mode for test purposes. If operating in UART mode (SIR_MODE != Enabled or not active, MCR\\[6\\]
set to zero), data on the sout line is held high, while serial data output is looped back to the sin line, internally. In this mode all the interrupts are fully functional. Also, in loopback mode, the modem control inputs (dsr_n, cts_n, ri_n, dcd_n) are disconnected and the modem control outputs (dtr_n, rts_n, out1_n, out2_n) are looped back to the inputs, internally. If operating in infrared mode (SIR_MODE == Enabled AND active, MCR\\[6\\]
set to one), data on the sir_out_n line is held low, while serial data output is inverted and looped back to the sir_in line"]
    #[inline(always)]
    pub fn lb(&self) -> LB_R {
        LB_R::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bit 5 - Auto Flow Control Enable. Writeable only when AFCE_MODE == Enabled, always readable. When FIFOs are enabled and the Auto Flow Control Enable (AFCE) bit is set, Auto Flow Control features are enabled as described in “Auto Flow Control” on page 51. 0 = Auto Flow Control Mode disabled 1 = Auto Flow Control Mode enabled"]
    #[inline(always)]
    pub fn afce(&self) -> AFCE_R {
        AFCE_R::new(((self.bits >> 5) & 1) != 0)
    }
    #[doc = "Bit 6 - SIR Mode Enable. Writeable only when SIR_MODE == Enabled, always readable. This is used to enable/disable the IrDA SIR Mode features as described in “IrDA 1.0 SIR Protocol” on page 47. 0 = IrDA SIR Mode disabled 1 = IrDA SIR Mode enabled"]
    #[inline(always)]
    pub fn sire(&self) -> SIRE_R {
        SIRE_R::new(((self.bits >> 6) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - Data Terminal Ready. This is used to directly control the Data Terminal Ready (dtr_n) output. The value written to this location is inverted and driven out on dtr_n, that is: 0 = dtr_n de-asserted (logic 1) 1 = dtr_n asserted (logic 0) The Data Terminal Ready output is used to inform the modem or data set that the UART is ready to establish communications. Note that in Loopback mode (MCR\\[4\\]
set to one), the dtr_n output is held inactive high while the value of this location is internally looped back to an input."]
    #[inline(always)]
    #[must_use]
    pub fn dtr(&mut self) -> DTR_W<MCR_SPEC, 0> {
        DTR_W::new(self)
    }
    #[doc = "Bit 1 - Request to Send. This is used to directly control the Request to Send (rts_n) output. The Request To Send (rts_n) output is used to inform the modem or data set that the UART is ready to exchange data. When Auto RTS Flow Control is not enabled (MCR\\[5\\]
set to zero), the rts_n signal is set low by programming MCR\\[1\\]
(RTS) to a high.In Auto Flow Control, AFCE_MODE == Enabled and active (MCR\\[5\\]
set to one) and FIFOs enable (FCR\\[0\\]
set to one), the rts_n output is controlled in the same way, but is also gated with the receiver FIFO threshold trigger (rts_n is inactive high when above the threshold). The rts_n signal is de-asserted when MCR\\[1\\]
is set low. Note that in Loopback mode (MCR\\[4\\]
set to one), the rts_n output is held inactive high while the value of this location is internally looped back to an input."]
    #[inline(always)]
    #[must_use]
    pub fn rts(&mut self) -> RTS_W<MCR_SPEC, 1> {
        RTS_W::new(self)
    }
    #[doc = "Bit 2 - OUT1. This is used to directly control the user-designated Output1 (out1_n) output. The value written to this location is inverted and driven out on out1_n, that is: 0 = out1_n de-asserted (logic 1) 1 = out1_n asserted (logic 0) Note that in Loopback mode (MCR\\[4\\]
set to one), the out1_n output is held inactive high while the value of this location is internally looped back to an input."]
    #[inline(always)]
    #[must_use]
    pub fn out1(&mut self) -> OUT1_W<MCR_SPEC, 2> {
        OUT1_W::new(self)
    }
    #[doc = "Bit 3 - OUT2. This is used to directly control the user-designated Output2 (out2_n) output. The value written to this location is inverted and driven out on out2_n, that is: 0 = out2_n de-asserted (logic 1) 1 = out2_n asserted (logic 0) Note that in Loopback mode (MCR\\[4\\]
set to one), the out2_n output is held inactive high while the value of this location is internally looped back to an input."]
    #[inline(always)]
    #[must_use]
    pub fn out2(&mut self) -> OUT2_W<MCR_SPEC, 3> {
        OUT2_W::new(self)
    }
    #[doc = "Bit 4 - LoopBack Bit. This is used to put the UART into a diagnostic mode for test purposes. If operating in UART mode (SIR_MODE != Enabled or not active, MCR\\[6\\]
set to zero), data on the sout line is held high, while serial data output is looped back to the sin line, internally. In this mode all the interrupts are fully functional. Also, in loopback mode, the modem control inputs (dsr_n, cts_n, ri_n, dcd_n) are disconnected and the modem control outputs (dtr_n, rts_n, out1_n, out2_n) are looped back to the inputs, internally. If operating in infrared mode (SIR_MODE == Enabled AND active, MCR\\[6\\]
set to one), data on the sir_out_n line is held low, while serial data output is inverted and looped back to the sir_in line"]
    #[inline(always)]
    #[must_use]
    pub fn lb(&mut self) -> LB_W<MCR_SPEC, 4> {
        LB_W::new(self)
    }
    #[doc = "Bit 5 - Auto Flow Control Enable. Writeable only when AFCE_MODE == Enabled, always readable. When FIFOs are enabled and the Auto Flow Control Enable (AFCE) bit is set, Auto Flow Control features are enabled as described in “Auto Flow Control” on page 51. 0 = Auto Flow Control Mode disabled 1 = Auto Flow Control Mode enabled"]
    #[inline(always)]
    #[must_use]
    pub fn afce(&mut self) -> AFCE_W<MCR_SPEC, 5> {
        AFCE_W::new(self)
    }
    #[doc = "Bit 6 - SIR Mode Enable. Writeable only when SIR_MODE == Enabled, always readable. This is used to enable/disable the IrDA SIR Mode features as described in “IrDA 1.0 SIR Protocol” on page 47. 0 = IrDA SIR Mode disabled 1 = IrDA SIR Mode enabled"]
    #[inline(always)]
    #[must_use]
    pub fn sire(&mut self) -> SIRE_W<MCR_SPEC, 6> {
        SIRE_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.bits = bits;
        self
    }
}
#[doc = "Modem Control Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`mcr::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`mcr::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct MCR_SPEC;
impl crate::RegisterSpec for MCR_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`mcr::R`](R) reader structure"]
impl crate::Readable for MCR_SPEC {}
#[doc = "`write(|w| ..)` method takes [`mcr::W`](W) writer structure"]
impl crate::Writable for MCR_SPEC {
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets mcr to value 0"]
impl crate::Resettable for MCR_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
