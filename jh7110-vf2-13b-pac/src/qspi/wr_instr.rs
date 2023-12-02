#[doc = "Register `wr_instr` reader"]
pub type R = crate::R<WR_INSTR_SPEC>;
#[doc = "Register `wr_instr` writer"]
pub type W = crate::W<WR_INSTR_SPEC>;
#[doc = "Field `opcode` reader - Instruction Opcode"]
pub type OPCODE_R = crate::FieldReader;
#[doc = "Field `opcode` writer - Instruction Opcode"]
pub type OPCODE_W<'a, REG> = crate::FieldWriter<'a, REG, 8>;
#[doc = "Field `type_addr` reader - Type of Address"]
pub type TYPE_ADDR_R = crate::FieldReader;
#[doc = "Field `type_addr` writer - Type of Address"]
pub type TYPE_ADDR_W<'a, REG> = crate::FieldWriter<'a, REG, 2>;
#[doc = "Field `type_data` reader - type_data"]
pub type TYPE_DATA_R = crate::FieldReader;
#[doc = "Field `type_data` writer - type_data"]
pub type TYPE_DATA_W<'a, REG> = crate::FieldWriter<'a, REG, 2>;
impl R {
    #[doc = "Bits 0:7 - Instruction Opcode"]
    #[inline(always)]
    pub fn opcode(&self) -> OPCODE_R {
        OPCODE_R::new((self.bits & 0xff) as u8)
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
}
impl W {
    #[doc = "Bits 0:7 - Instruction Opcode"]
    #[inline(always)]
    #[must_use]
    pub fn opcode(&mut self) -> OPCODE_W<WR_INSTR_SPEC> {
        OPCODE_W::new(self, 0)
    }
    #[doc = "Bits 12:13 - Type of Address"]
    #[inline(always)]
    #[must_use]
    pub fn type_addr(&mut self) -> TYPE_ADDR_W<WR_INSTR_SPEC> {
        TYPE_ADDR_W::new(self, 12)
    }
    #[doc = "Bits 16:17 - type_data"]
    #[inline(always)]
    #[must_use]
    pub fn type_data(&mut self) -> TYPE_DATA_W<WR_INSTR_SPEC> {
        TYPE_DATA_W::new(self, 16)
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
#[doc = "Cadence QSPI Write Instruction\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`wr_instr::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`wr_instr::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct WR_INSTR_SPEC;
impl crate::RegisterSpec for WR_INSTR_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`wr_instr::R`](R) reader structure"]
impl crate::Readable for WR_INSTR_SPEC {}
#[doc = "`write(|w| ..)` method takes [`wr_instr::W`](W) writer structure"]
impl crate::Writable for WR_INSTR_SPEC {
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets wr_instr to value 0"]
impl crate::Resettable for WR_INSTR_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
