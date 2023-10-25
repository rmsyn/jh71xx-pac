#[doc = "Register `rd_instr` reader"]
pub type R = crate::R<RD_INSTR_SPEC>;
#[doc = "Register `rd_instr` writer"]
pub type W = crate::W<RD_INSTR_SPEC>;
#[doc = "Field `opcode` reader - Instruction Opcode"]
pub type OPCODE_R = crate::FieldReader;
#[doc = "Field `opcode` writer - Instruction Opcode"]
pub type OPCODE_W<'a, REG, const O: u8> = crate::FieldWriter<'a, REG, 8, O>;
#[doc = "Field `type_instr` reader - Type of Instruction"]
pub type TYPE_INSTR_R = crate::FieldReader;
#[doc = "Field `type_instr` writer - Type of Instruction"]
pub type TYPE_INSTR_W<'a, REG, const O: u8> = crate::FieldWriter<'a, REG, 2, O>;
#[doc = "Field `type_addr` reader - Type of Address"]
pub type TYPE_ADDR_R = crate::FieldReader;
#[doc = "Field `type_addr` writer - Type of Address"]
pub type TYPE_ADDR_W<'a, REG, const O: u8> = crate::FieldWriter<'a, REG, 2, O>;
#[doc = "Field `type_data` reader - type_data"]
pub type TYPE_DATA_R = crate::FieldReader;
#[doc = "Field `type_data` writer - type_data"]
pub type TYPE_DATA_W<'a, REG, const O: u8> = crate::FieldWriter<'a, REG, 2, O>;
#[doc = "Field `mode_en` reader - Mode"]
pub type MODE_EN_R = crate::BitReader;
#[doc = "Field `mode_en` writer - Mode"]
pub type MODE_EN_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O>;
#[doc = "Field `dummy` reader - Send dummy signal to stall the device"]
pub type DUMMY_R = crate::FieldReader;
#[doc = "Field `dummy` writer - Send dummy signal to stall the device"]
pub type DUMMY_W<'a, REG, const O: u8> = crate::FieldWriter<'a, REG, 5, O>;
impl R {
    #[doc = "Bits 0:7 - Instruction Opcode"]
    #[inline(always)]
    pub fn opcode(&self) -> OPCODE_R {
        OPCODE_R::new((self.bits & 0xff) as u8)
    }
    #[doc = "Bits 8:9 - Type of Instruction"]
    #[inline(always)]
    pub fn type_instr(&self) -> TYPE_INSTR_R {
        TYPE_INSTR_R::new(((self.bits >> 8) & 3) as u8)
    }
    #[doc = "Bits 12:13 - Type of Address"]
    #[inline(always)]
    pub fn type_addr(&self) -> TYPE_ADDR_R {
        TYPE_ADDR_R::new(((self.bits >> 12) & 3) as u8)
    }
    #[doc = "Bits 16:17 - type_data"]
    #[inline(always)]
    pub fn type_data(&self) -> TYPE_DATA_R {
        TYPE_DATA_R::new(((self.bits >> 16) & 3) as u8)
    }
    #[doc = "Bit 20 - Mode"]
    #[inline(always)]
    pub fn mode_en(&self) -> MODE_EN_R {
        MODE_EN_R::new(((self.bits >> 20) & 1) != 0)
    }
    #[doc = "Bits 24:28 - Send dummy signal to stall the device"]
    #[inline(always)]
    pub fn dummy(&self) -> DUMMY_R {
        DUMMY_R::new(((self.bits >> 24) & 0x1f) as u8)
    }
}
impl W {
    #[doc = "Bits 0:7 - Instruction Opcode"]
    #[inline(always)]
    #[must_use]
    pub fn opcode(&mut self) -> OPCODE_W<RD_INSTR_SPEC, 0> {
        OPCODE_W::new(self)
    }
    #[doc = "Bits 8:9 - Type of Instruction"]
    #[inline(always)]
    #[must_use]
    pub fn type_instr(&mut self) -> TYPE_INSTR_W<RD_INSTR_SPEC, 8> {
        TYPE_INSTR_W::new(self)
    }
    #[doc = "Bits 12:13 - Type of Address"]
    #[inline(always)]
    #[must_use]
    pub fn type_addr(&mut self) -> TYPE_ADDR_W<RD_INSTR_SPEC, 12> {
        TYPE_ADDR_W::new(self)
    }
    #[doc = "Bits 16:17 - type_data"]
    #[inline(always)]
    #[must_use]
    pub fn type_data(&mut self) -> TYPE_DATA_W<RD_INSTR_SPEC, 16> {
        TYPE_DATA_W::new(self)
    }
    #[doc = "Bit 20 - Mode"]
    #[inline(always)]
    #[must_use]
    pub fn mode_en(&mut self) -> MODE_EN_W<RD_INSTR_SPEC, 20> {
        MODE_EN_W::new(self)
    }
    #[doc = "Bits 24:28 - Send dummy signal to stall the device"]
    #[inline(always)]
    #[must_use]
    pub fn dummy(&mut self) -> DUMMY_W<RD_INSTR_SPEC, 24> {
        DUMMY_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.bits = bits;
        self
    }
}
#[doc = "Cadence QSPI Read Instruction\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`rd_instr::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`rd_instr::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct RD_INSTR_SPEC;
impl crate::RegisterSpec for RD_INSTR_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`rd_instr::R`](R) reader structure"]
impl crate::Readable for RD_INSTR_SPEC {}
#[doc = "`write(|w| ..)` method takes [`rd_instr::W`](W) writer structure"]
impl crate::Writable for RD_INSTR_SPEC {
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets rd_instr to value 0"]
impl crate::Resettable for RD_INSTR_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
