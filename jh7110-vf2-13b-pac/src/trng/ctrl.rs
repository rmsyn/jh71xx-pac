#[doc = "Register `ctrl` reader"]
pub type R = crate::R<CTRL_SPEC>;
#[doc = "Register `ctrl` writer"]
pub type W = crate::W<CTRL_SPEC>;
#[doc = "Field `exec_nop` reader - Execute a NOP instruction"]
pub type EXEC_NOP_R = crate::BitReader;
#[doc = "Field `exec_nop` writer - Execute a NOP instruction"]
pub type EXEC_NOP_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `gene_randnum` reader - Generate a random number"]
pub type GENE_RANDNUM_R = crate::BitReader;
#[doc = "Field `gene_randnum` writer - Generate a random number"]
pub type GENE_RANDNUM_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `exec_randreseed` reader - Reseed the TRNG from noise sources"]
pub type EXEC_RANDRESEED_R = crate::BitReader;
#[doc = "Field `exec_randreseed` writer - Reseed the TRNG from noise sources"]
pub type EXEC_RANDRESEED_W<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bit 0 - Execute a NOP instruction"]
    #[inline(always)]
    pub fn exec_nop(&self) -> EXEC_NOP_R {
        EXEC_NOP_R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - Generate a random number"]
    #[inline(always)]
    pub fn gene_randnum(&self) -> GENE_RANDNUM_R {
        GENE_RANDNUM_R::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - Reseed the TRNG from noise sources"]
    #[inline(always)]
    pub fn exec_randreseed(&self) -> EXEC_RANDRESEED_R {
        EXEC_RANDRESEED_R::new(((self.bits >> 2) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - Execute a NOP instruction"]
    #[inline(always)]
    #[must_use]
    pub fn exec_nop(&mut self) -> EXEC_NOP_W<CTRL_SPEC> {
        EXEC_NOP_W::new(self, 0)
    }
    #[doc = "Bit 1 - Generate a random number"]
    #[inline(always)]
    #[must_use]
    pub fn gene_randnum(&mut self) -> GENE_RANDNUM_W<CTRL_SPEC> {
        GENE_RANDNUM_W::new(self, 1)
    }
    #[doc = "Bit 2 - Reseed the TRNG from noise sources"]
    #[inline(always)]
    #[must_use]
    pub fn exec_randreseed(&mut self) -> EXEC_RANDRESEED_W<CTRL_SPEC> {
        EXEC_RANDRESEED_W::new(self, 2)
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
#[doc = "TRNG CTRL Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`ctrl::R`](R).  You can [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`ctrl::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct CTRL_SPEC;
impl crate::RegisterSpec for CTRL_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`ctrl::R`](R) reader structure"]
impl crate::Readable for CTRL_SPEC {}
#[doc = "`write(|w| ..)` method takes [`ctrl::W`](W) writer structure"]
impl crate::Writable for CTRL_SPEC {
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
