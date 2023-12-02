#[doc = "Register `sram_partition` reader"]
pub type R = crate::R<SRAM_PARTITION_SPEC>;
#[doc = "Register `sram_partition` writer"]
pub type W = crate::W<SRAM_PARTITION_SPEC>;
#[doc = "Field `size` reader - Partition size in bytes"]
pub type SIZE_R = crate::FieldReader<u32>;
#[doc = "Field `size` writer - Partition size in bytes"]
pub type SIZE_W<'a, REG> = crate::FieldWriter<'a, REG, 32, u32>;
impl R {
    #[doc = "Bits 0:31 - Partition size in bytes"]
    #[inline(always)]
    pub fn size(&self) -> SIZE_R {
        SIZE_R::new(self.bits)
    }
}
impl W {
    #[doc = "Bits 0:31 - Partition size in bytes"]
    #[inline(always)]
    #[must_use]
    pub fn size(&mut self) -> SIZE_W<SRAM_PARTITION_SPEC> {
        SIZE_W::new(self, 0)
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
#[doc = "Cadence QSPI SRAM Partition Size\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`sram_partition::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`sram_partition::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct SRAM_PARTITION_SPEC;
impl crate::RegisterSpec for SRAM_PARTITION_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`sram_partition::R`](R) reader structure"]
impl crate::Readable for SRAM_PARTITION_SPEC {}
#[doc = "`write(|w| ..)` method takes [`sram_partition::W`](W) writer structure"]
impl crate::Writable for SRAM_PARTITION_SPEC {
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets sram_partition to value 0"]
impl crate::Resettable for SRAM_PARTITION_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
