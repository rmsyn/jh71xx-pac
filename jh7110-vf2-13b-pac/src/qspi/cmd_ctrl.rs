#[doc = "Register `cmd_ctrl` reader"]
pub type R = crate::R<CMD_CTRL_SPEC>;
#[doc = "Register `cmd_ctrl` writer"]
pub type W = crate::W<CMD_CTRL_SPEC>;
#[doc = "Field `execute` reader - Execute-in-Place (XIP)"]
pub type EXECUTE_R = crate::BitReader;
#[doc = "Field `execute` writer - Execute-in-Place (XIP)"]
pub type EXECUTE_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O>;
#[doc = "Field `in_progress` reader - Command in progress"]
pub type IN_PROGRESS_R = crate::BitReader;
#[doc = "Field `in_progress` writer - Command in progress"]
pub type IN_PROGRESS_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O>;
#[doc = "Field `dummy` reader - Dummy command"]
pub type DUMMY_R = crate::FieldReader;
#[doc = "Field `dummy` writer - Dummy command"]
pub type DUMMY_W<'a, REG, const O: u8> = crate::FieldWriter<'a, REG, 5, O>;
#[doc = "Field `wr_bytes` reader - Write bytes"]
pub type WR_BYTES_R = crate::FieldReader;
#[doc = "Field `wr_bytes` writer - Write bytes"]
pub type WR_BYTES_W<'a, REG, const O: u8> = crate::FieldWriter<'a, REG, 3, O>;
#[doc = "Field `wr_en` reader - Write enable"]
pub type WR_EN_R = crate::BitReader;
#[doc = "Field `wr_en` writer - Write enable"]
pub type WR_EN_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O>;
#[doc = "Field `add_bytes` reader - Add command bytes"]
pub type ADD_BYTES_R = crate::FieldReader;
#[doc = "Field `add_bytes` writer - Add command bytes"]
pub type ADD_BYTES_W<'a, REG, const O: u8> = crate::FieldWriter<'a, REG, 2, O>;
#[doc = "Field `addr_en` reader - Address enable"]
pub type ADDR_EN_R = crate::BitReader;
#[doc = "Field `addr_en` writer - Address enable"]
pub type ADDR_EN_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O>;
#[doc = "Field `rd_bytes` reader - Read bytes"]
pub type RD_BYTES_R = crate::FieldReader;
#[doc = "Field `rd_bytes` writer - Read bytes"]
pub type RD_BYTES_W<'a, REG, const O: u8> = crate::FieldWriter<'a, REG, 3, O>;
#[doc = "Field `rd_en` reader - Read enable"]
pub type RD_EN_R = crate::BitReader;
#[doc = "Field `rd_en` writer - Read enable"]
pub type RD_EN_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O>;
#[doc = "Field `opcode` reader - Command opcode"]
pub type OPCODE_R = crate::FieldReader;
#[doc = "Field `opcode` writer - Command opcode"]
pub type OPCODE_W<'a, REG, const O: u8> = crate::FieldWriter<'a, REG, 8, O>;
impl R {
    #[doc = "Bit 0 - Execute-in-Place (XIP)"]
    #[inline(always)]
    pub fn execute(&self) -> EXECUTE_R {
        EXECUTE_R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - Command in progress"]
    #[inline(always)]
    pub fn in_progress(&self) -> IN_PROGRESS_R {
        IN_PROGRESS_R::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bits 7:11 - Dummy command"]
    #[inline(always)]
    pub fn dummy(&self) -> DUMMY_R {
        DUMMY_R::new(((self.bits >> 7) & 0x1f) as u8)
    }
    #[doc = "Bits 12:14 - Write bytes"]
    #[inline(always)]
    pub fn wr_bytes(&self) -> WR_BYTES_R {
        WR_BYTES_R::new(((self.bits >> 12) & 7) as u8)
    }
    #[doc = "Bit 15 - Write enable"]
    #[inline(always)]
    pub fn wr_en(&self) -> WR_EN_R {
        WR_EN_R::new(((self.bits >> 15) & 1) != 0)
    }
    #[doc = "Bits 16:17 - Add command bytes"]
    #[inline(always)]
    pub fn add_bytes(&self) -> ADD_BYTES_R {
        ADD_BYTES_R::new(((self.bits >> 16) & 3) as u8)
    }
    #[doc = "Bit 19 - Address enable"]
    #[inline(always)]
    pub fn addr_en(&self) -> ADDR_EN_R {
        ADDR_EN_R::new(((self.bits >> 19) & 1) != 0)
    }
    #[doc = "Bits 20:22 - Read bytes"]
    #[inline(always)]
    pub fn rd_bytes(&self) -> RD_BYTES_R {
        RD_BYTES_R::new(((self.bits >> 20) & 7) as u8)
    }
    #[doc = "Bit 23 - Read enable"]
    #[inline(always)]
    pub fn rd_en(&self) -> RD_EN_R {
        RD_EN_R::new(((self.bits >> 23) & 1) != 0)
    }
    #[doc = "Bits 24:31 - Command opcode"]
    #[inline(always)]
    pub fn opcode(&self) -> OPCODE_R {
        OPCODE_R::new(((self.bits >> 24) & 0xff) as u8)
    }
}
impl W {
    #[doc = "Bit 0 - Execute-in-Place (XIP)"]
    #[inline(always)]
    #[must_use]
    pub fn execute(&mut self) -> EXECUTE_W<CMD_CTRL_SPEC, 0> {
        EXECUTE_W::new(self)
    }
    #[doc = "Bit 1 - Command in progress"]
    #[inline(always)]
    #[must_use]
    pub fn in_progress(&mut self) -> IN_PROGRESS_W<CMD_CTRL_SPEC, 1> {
        IN_PROGRESS_W::new(self)
    }
    #[doc = "Bits 7:11 - Dummy command"]
    #[inline(always)]
    #[must_use]
    pub fn dummy(&mut self) -> DUMMY_W<CMD_CTRL_SPEC, 7> {
        DUMMY_W::new(self)
    }
    #[doc = "Bits 12:14 - Write bytes"]
    #[inline(always)]
    #[must_use]
    pub fn wr_bytes(&mut self) -> WR_BYTES_W<CMD_CTRL_SPEC, 12> {
        WR_BYTES_W::new(self)
    }
    #[doc = "Bit 15 - Write enable"]
    #[inline(always)]
    #[must_use]
    pub fn wr_en(&mut self) -> WR_EN_W<CMD_CTRL_SPEC, 15> {
        WR_EN_W::new(self)
    }
    #[doc = "Bits 16:17 - Add command bytes"]
    #[inline(always)]
    #[must_use]
    pub fn add_bytes(&mut self) -> ADD_BYTES_W<CMD_CTRL_SPEC, 16> {
        ADD_BYTES_W::new(self)
    }
    #[doc = "Bit 19 - Address enable"]
    #[inline(always)]
    #[must_use]
    pub fn addr_en(&mut self) -> ADDR_EN_W<CMD_CTRL_SPEC, 19> {
        ADDR_EN_W::new(self)
    }
    #[doc = "Bits 20:22 - Read bytes"]
    #[inline(always)]
    #[must_use]
    pub fn rd_bytes(&mut self) -> RD_BYTES_W<CMD_CTRL_SPEC, 20> {
        RD_BYTES_W::new(self)
    }
    #[doc = "Bit 23 - Read enable"]
    #[inline(always)]
    #[must_use]
    pub fn rd_en(&mut self) -> RD_EN_W<CMD_CTRL_SPEC, 23> {
        RD_EN_W::new(self)
    }
    #[doc = "Bits 24:31 - Command opcode"]
    #[inline(always)]
    #[must_use]
    pub fn opcode(&mut self) -> OPCODE_W<CMD_CTRL_SPEC, 24> {
        OPCODE_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.bits = bits;
        self
    }
}
#[doc = "Cadence QSPI Command Control\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`cmd_ctrl::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`cmd_ctrl::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct CMD_CTRL_SPEC;
impl crate::RegisterSpec for CMD_CTRL_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`cmd_ctrl::R`](R) reader structure"]
impl crate::Readable for CMD_CTRL_SPEC {}
#[doc = "`write(|w| ..)` method takes [`cmd_ctrl::W`](W) writer structure"]
impl crate::Writable for CMD_CTRL_SPEC {
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets cmd_ctrl to value 0"]
impl crate::Resettable for CMD_CTRL_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
