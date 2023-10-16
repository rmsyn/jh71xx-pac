#[doc = "Register `tar` reader"]
pub type R = crate::R<TAR_SPEC>;
#[doc = "Register `tar` writer"]
pub type W = crate::W<TAR_SPEC>;
#[doc = "Field `master_10bit` reader - If I2C_DYNAMIC_TAR_UPDATE is set, the 10-bit addressing mode has to be enabled via bit 12 of TAR register. We set it always as I2C_DYNAMIC_TAR_UPDATE can't be detected from registers."]
pub type MASTER_10BIT_R = crate::BitReader;
#[doc = "Field `master_10bit` writer - If I2C_DYNAMIC_TAR_UPDATE is set, the 10-bit addressing mode has to be enabled via bit 12 of TAR register. We set it always as I2C_DYNAMIC_TAR_UPDATE can't be detected from registers."]
pub type MASTER_10BIT_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O>;
impl R {
    #[doc = "Bit 12 - If I2C_DYNAMIC_TAR_UPDATE is set, the 10-bit addressing mode has to be enabled via bit 12 of TAR register. We set it always as I2C_DYNAMIC_TAR_UPDATE can't be detected from registers."]
    #[inline(always)]
    pub fn master_10bit(&self) -> MASTER_10BIT_R {
        MASTER_10BIT_R::new(((self.bits >> 12) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 12 - If I2C_DYNAMIC_TAR_UPDATE is set, the 10-bit addressing mode has to be enabled via bit 12 of TAR register. We set it always as I2C_DYNAMIC_TAR_UPDATE can't be detected from registers."]
    #[inline(always)]
    #[must_use]
    pub fn master_10bit(&mut self) -> MASTER_10BIT_W<TAR_SPEC, 12> {
        MASTER_10BIT_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.bits = bits;
        self
    }
}
#[doc = "DesignWare I2C TAR\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`tar::R`](R).  You can [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`tar::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct TAR_SPEC;
impl crate::RegisterSpec for TAR_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`tar::R`](R) reader structure"]
impl crate::Readable for TAR_SPEC {}
#[doc = "`write(|w| ..)` method takes [`tar::W`](W) writer structure"]
impl crate::Writable for TAR_SPEC {
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
