#[doc = "Register `config` reader"]
pub type R = crate::R<CONFIG_SPEC>;
#[doc = "Register `config` writer"]
pub type W = crate::W<CONFIG_SPEC>;
#[doc = "Field `enable` reader - Enable the QSPI controller"]
pub type ENABLE_R = crate::BitReader;
#[doc = "Field `enable` writer - Enable the QSPI controller"]
pub type ENABLE_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O>;
#[doc = "Field `enb_dir_acc_ctrl` reader - Enable direct access controller"]
pub type ENB_DIR_ACC_CTRL_R = crate::BitReader;
#[doc = "Field `enb_dir_acc_ctrl` writer - Enable direct access controller"]
pub type ENB_DIR_ACC_CTRL_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O>;
#[doc = "Field `decode` reader - Enable the QSPI decoder"]
pub type DECODE_R = crate::BitReader;
#[doc = "Field `decode` writer - Enable the QSPI decoder"]
pub type DECODE_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O>;
#[doc = "Field `chipselect` reader - Chip select - CS0: 0b1110, CS1: 0b1101, CS2: 0b1011, CS3: 0b0111"]
pub type CHIPSELECT_R = crate::FieldReader;
#[doc = "Field `chipselect` writer - Chip select - CS0: 0b1110, CS1: 0b1101, CS2: 0b1011, CS3: 0b0111"]
pub type CHIPSELECT_W<'a, REG, const O: u8> = crate::FieldWriter<'a, REG, 4, O>;
#[doc = "Field `dma` reader - Enable Direct Memory Access"]
pub type DMA_R = crate::BitReader;
#[doc = "Field `dma` writer - Enable Direct Memory Access"]
pub type DMA_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O>;
#[doc = "Field `baud` reader - Set the QSPI BAUD rate divisor"]
pub type BAUD_R = crate::FieldReader;
#[doc = "Field `baud` writer - Set the QSPI BAUD rate divisor"]
pub type BAUD_W<'a, REG, const O: u8> = crate::FieldWriter<'a, REG, 4, O>;
#[doc = "Field `dtr_proto` reader - Enable DTR Protocol"]
pub type DTR_PROTO_R = crate::BitReader;
#[doc = "Field `dtr_proto` writer - Enable DTR Protocol"]
pub type DTR_PROTO_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O>;
#[doc = "Field `dual_opcode` reader - Enable Dual Opcode Mode"]
pub type DUAL_OPCODE_R = crate::BitReader;
#[doc = "Field `dual_opcode` writer - Enable Dual Opcode Mode"]
pub type DUAL_OPCODE_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O>;
#[doc = "Field `idle` reader - Set Idle"]
pub type IDLE_R = crate::BitReader;
#[doc = "Field `idle` writer - Set Idle"]
pub type IDLE_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O>;
impl R {
    #[doc = "Bit 0 - Enable the QSPI controller"]
    #[inline(always)]
    pub fn enable(&self) -> ENABLE_R {
        ENABLE_R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 7 - Enable direct access controller"]
    #[inline(always)]
    pub fn enb_dir_acc_ctrl(&self) -> ENB_DIR_ACC_CTRL_R {
        ENB_DIR_ACC_CTRL_R::new(((self.bits >> 7) & 1) != 0)
    }
    #[doc = "Bit 9 - Enable the QSPI decoder"]
    #[inline(always)]
    pub fn decode(&self) -> DECODE_R {
        DECODE_R::new(((self.bits >> 9) & 1) != 0)
    }
    #[doc = "Bits 10:13 - Chip select - CS0: 0b1110, CS1: 0b1101, CS2: 0b1011, CS3: 0b0111"]
    #[inline(always)]
    pub fn chipselect(&self) -> CHIPSELECT_R {
        CHIPSELECT_R::new(((self.bits >> 10) & 0x0f) as u8)
    }
    #[doc = "Bit 15 - Enable Direct Memory Access"]
    #[inline(always)]
    pub fn dma(&self) -> DMA_R {
        DMA_R::new(((self.bits >> 15) & 1) != 0)
    }
    #[doc = "Bits 19:22 - Set the QSPI BAUD rate divisor"]
    #[inline(always)]
    pub fn baud(&self) -> BAUD_R {
        BAUD_R::new(((self.bits >> 19) & 0x0f) as u8)
    }
    #[doc = "Bit 24 - Enable DTR Protocol"]
    #[inline(always)]
    pub fn dtr_proto(&self) -> DTR_PROTO_R {
        DTR_PROTO_R::new(((self.bits >> 24) & 1) != 0)
    }
    #[doc = "Bit 30 - Enable Dual Opcode Mode"]
    #[inline(always)]
    pub fn dual_opcode(&self) -> DUAL_OPCODE_R {
        DUAL_OPCODE_R::new(((self.bits >> 30) & 1) != 0)
    }
    #[doc = "Bit 31 - Set Idle"]
    #[inline(always)]
    pub fn idle(&self) -> IDLE_R {
        IDLE_R::new(((self.bits >> 31) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - Enable the QSPI controller"]
    #[inline(always)]
    #[must_use]
    pub fn enable(&mut self) -> ENABLE_W<CONFIG_SPEC, 0> {
        ENABLE_W::new(self)
    }
    #[doc = "Bit 7 - Enable direct access controller"]
    #[inline(always)]
    #[must_use]
    pub fn enb_dir_acc_ctrl(&mut self) -> ENB_DIR_ACC_CTRL_W<CONFIG_SPEC, 7> {
        ENB_DIR_ACC_CTRL_W::new(self)
    }
    #[doc = "Bit 9 - Enable the QSPI decoder"]
    #[inline(always)]
    #[must_use]
    pub fn decode(&mut self) -> DECODE_W<CONFIG_SPEC, 9> {
        DECODE_W::new(self)
    }
    #[doc = "Bits 10:13 - Chip select - CS0: 0b1110, CS1: 0b1101, CS2: 0b1011, CS3: 0b0111"]
    #[inline(always)]
    #[must_use]
    pub fn chipselect(&mut self) -> CHIPSELECT_W<CONFIG_SPEC, 10> {
        CHIPSELECT_W::new(self)
    }
    #[doc = "Bit 15 - Enable Direct Memory Access"]
    #[inline(always)]
    #[must_use]
    pub fn dma(&mut self) -> DMA_W<CONFIG_SPEC, 15> {
        DMA_W::new(self)
    }
    #[doc = "Bits 19:22 - Set the QSPI BAUD rate divisor"]
    #[inline(always)]
    #[must_use]
    pub fn baud(&mut self) -> BAUD_W<CONFIG_SPEC, 19> {
        BAUD_W::new(self)
    }
    #[doc = "Bit 24 - Enable DTR Protocol"]
    #[inline(always)]
    #[must_use]
    pub fn dtr_proto(&mut self) -> DTR_PROTO_W<CONFIG_SPEC, 24> {
        DTR_PROTO_W::new(self)
    }
    #[doc = "Bit 30 - Enable Dual Opcode Mode"]
    #[inline(always)]
    #[must_use]
    pub fn dual_opcode(&mut self) -> DUAL_OPCODE_W<CONFIG_SPEC, 30> {
        DUAL_OPCODE_W::new(self)
    }
    #[doc = "Bit 31 - Set Idle"]
    #[inline(always)]
    #[must_use]
    pub fn idle(&mut self) -> IDLE_W<CONFIG_SPEC, 31> {
        IDLE_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.bits = bits;
        self
    }
}
#[doc = "Cadence QSPI Configuration\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`config::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`config::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct CONFIG_SPEC;
impl crate::RegisterSpec for CONFIG_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`config::R`](R) reader structure"]
impl crate::Readable for CONFIG_SPEC {}
#[doc = "`write(|w| ..)` method takes [`config::W`](W) writer structure"]
impl crate::Writable for CONFIG_SPEC {
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets config to value 0"]
impl crate::Resettable for CONFIG_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
