#[doc = "Register `data_cmd` reader"]
pub type R = crate::R<DATA_CMD_SPEC>;
#[doc = "Register `data_cmd` writer"]
pub type W = crate::W<DATA_CMD_SPEC>;
#[doc = "Field `dat` reader - Data Command Data Byte"]
pub type DAT_R = crate::FieldReader;
#[doc = "Field `dat` writer - Data Command Data Byte"]
pub type DAT_W<'a, REG, const O: u8> = crate::FieldWriter<'a, REG, 8, O>;
#[doc = "Field `read` reader - Data Command READ Bit - 0: Write, 1: Read"]
pub type READ_R = crate::BitReader;
#[doc = "Field `read` writer - Data Command READ Bit - 0: Write, 1: Read"]
pub type READ_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O>;
#[doc = "Field `stop` reader - Data Command STOP Bit - 0: Non-terminal DATA command byte, 1: Terminal DATA command byte"]
pub type STOP_R = crate::BitReader;
#[doc = "Field `stop` writer - Data Command STOP Bit - 0: Non-terminal DATA command byte, 1: Terminal DATA command byte"]
pub type STOP_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O>;
#[doc = "Field `restart` reader - Data Command RESTART Bit - 0: Do not restart the transfer, 1: Restart the transfer"]
pub type RESTART_R = crate::BitReader;
#[doc = "Field `restart` writer - Data Command RESTART Bit - 0: Do not restart the transfer, 1: Restart the transfer"]
pub type RESTART_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O>;
#[doc = "Field `first_data_byte` reader - Data Command First Data Byte - 0: False, 1: True"]
pub type FIRST_DATA_BYTE_R = crate::BitReader;
#[doc = "Field `first_data_byte` writer - Data Command First Data Byte - 0: False, 1: True"]
pub type FIRST_DATA_BYTE_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O>;
impl R {
    #[doc = "Bits 0:7 - Data Command Data Byte"]
    #[inline(always)]
    pub fn dat(&self) -> DAT_R {
        DAT_R::new((self.bits & 0xff) as u8)
    }
    #[doc = "Bit 8 - Data Command READ Bit - 0: Write, 1: Read"]
    #[inline(always)]
    pub fn read(&self) -> READ_R {
        READ_R::new(((self.bits >> 8) & 1) != 0)
    }
    #[doc = "Bit 9 - Data Command STOP Bit - 0: Non-terminal DATA command byte, 1: Terminal DATA command byte"]
    #[inline(always)]
    pub fn stop(&self) -> STOP_R {
        STOP_R::new(((self.bits >> 9) & 1) != 0)
    }
    #[doc = "Bit 10 - Data Command RESTART Bit - 0: Do not restart the transfer, 1: Restart the transfer"]
    #[inline(always)]
    pub fn restart(&self) -> RESTART_R {
        RESTART_R::new(((self.bits >> 10) & 1) != 0)
    }
    #[doc = "Bit 11 - Data Command First Data Byte - 0: False, 1: True"]
    #[inline(always)]
    pub fn first_data_byte(&self) -> FIRST_DATA_BYTE_R {
        FIRST_DATA_BYTE_R::new(((self.bits >> 11) & 1) != 0)
    }
}
impl W {
    #[doc = "Bits 0:7 - Data Command Data Byte"]
    #[inline(always)]
    #[must_use]
    pub fn dat(&mut self) -> DAT_W<DATA_CMD_SPEC, 0> {
        DAT_W::new(self)
    }
    #[doc = "Bit 8 - Data Command READ Bit - 0: Write, 1: Read"]
    #[inline(always)]
    #[must_use]
    pub fn read(&mut self) -> READ_W<DATA_CMD_SPEC, 8> {
        READ_W::new(self)
    }
    #[doc = "Bit 9 - Data Command STOP Bit - 0: Non-terminal DATA command byte, 1: Terminal DATA command byte"]
    #[inline(always)]
    #[must_use]
    pub fn stop(&mut self) -> STOP_W<DATA_CMD_SPEC, 9> {
        STOP_W::new(self)
    }
    #[doc = "Bit 10 - Data Command RESTART Bit - 0: Do not restart the transfer, 1: Restart the transfer"]
    #[inline(always)]
    #[must_use]
    pub fn restart(&mut self) -> RESTART_W<DATA_CMD_SPEC, 10> {
        RESTART_W::new(self)
    }
    #[doc = "Bit 11 - Data Command First Data Byte - 0: False, 1: True"]
    #[inline(always)]
    #[must_use]
    pub fn first_data_byte(&mut self) -> FIRST_DATA_BYTE_W<DATA_CMD_SPEC, 11> {
        FIRST_DATA_BYTE_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.bits = bits;
        self
    }
}
#[doc = "DesignWare I2C Data Command\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`data_cmd::R`](R).  You can [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`data_cmd::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct DATA_CMD_SPEC;
impl crate::RegisterSpec for DATA_CMD_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`data_cmd::R`](R) reader structure"]
impl crate::Readable for DATA_CMD_SPEC {}
#[doc = "`write(|w| ..)` method takes [`data_cmd::W`](W) writer structure"]
impl crate::Writable for DATA_CMD_SPEC {
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
