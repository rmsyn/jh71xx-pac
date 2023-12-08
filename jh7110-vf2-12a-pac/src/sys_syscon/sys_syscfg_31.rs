#[doc = "Register `sys_syscfg_31` reader"]
pub type R = crate::R<SYS_SYSCFG_31_SPEC>;
#[doc = "Register `sys_syscfg_31` writer"]
pub type W = crate::W<SYS_SYSCFG_31_SPEC>;
#[doc = "Field `reset_vector_3_32` reader - U0 U74MC Reset Vector 3: 32"]
pub type RESET_VECTOR_3_32_R = crate::BitReader;
#[doc = "Field `reset_vector_3_32` writer - U0 U74MC Reset Vector 3: 32"]
pub type RESET_VECTOR_3_32_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `reset_vector_3_33` reader - U0 U74MC Reset Vector 3: 33"]
pub type RESET_VECTOR_3_33_R = crate::BitReader;
#[doc = "Field `reset_vector_3_33` writer - U0 U74MC Reset Vector 3: 33"]
pub type RESET_VECTOR_3_33_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `reset_vector_3_34` reader - U0 U74MC Reset Vector 3: 34"]
pub type RESET_VECTOR_3_34_R = crate::BitReader;
#[doc = "Field `reset_vector_3_34` writer - U0 U74MC Reset Vector 3: 34"]
pub type RESET_VECTOR_3_34_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `reset_vector_3_35` reader - U0 U74MC Reset Vector 3: 35"]
pub type RESET_VECTOR_3_35_R = crate::BitReader;
#[doc = "Field `reset_vector_3_35` writer - U0 U74MC Reset Vector 3: 35"]
pub type RESET_VECTOR_3_35_W<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bit 0 - U0 U74MC Reset Vector 3: 32"]
    #[inline(always)]
    pub fn reset_vector_3_32(&self) -> RESET_VECTOR_3_32_R {
        RESET_VECTOR_3_32_R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - U0 U74MC Reset Vector 3: 33"]
    #[inline(always)]
    pub fn reset_vector_3_33(&self) -> RESET_VECTOR_3_33_R {
        RESET_VECTOR_3_33_R::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - U0 U74MC Reset Vector 3: 34"]
    #[inline(always)]
    pub fn reset_vector_3_34(&self) -> RESET_VECTOR_3_34_R {
        RESET_VECTOR_3_34_R::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3 - U0 U74MC Reset Vector 3: 35"]
    #[inline(always)]
    pub fn reset_vector_3_35(&self) -> RESET_VECTOR_3_35_R {
        RESET_VECTOR_3_35_R::new(((self.bits >> 3) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - U0 U74MC Reset Vector 3: 32"]
    #[inline(always)]
    #[must_use]
    pub fn reset_vector_3_32(&mut self) -> RESET_VECTOR_3_32_W<SYS_SYSCFG_31_SPEC> {
        RESET_VECTOR_3_32_W::new(self, 0)
    }
    #[doc = "Bit 1 - U0 U74MC Reset Vector 3: 33"]
    #[inline(always)]
    #[must_use]
    pub fn reset_vector_3_33(&mut self) -> RESET_VECTOR_3_33_W<SYS_SYSCFG_31_SPEC> {
        RESET_VECTOR_3_33_W::new(self, 1)
    }
    #[doc = "Bit 2 - U0 U74MC Reset Vector 3: 34"]
    #[inline(always)]
    #[must_use]
    pub fn reset_vector_3_34(&mut self) -> RESET_VECTOR_3_34_W<SYS_SYSCFG_31_SPEC> {
        RESET_VECTOR_3_34_W::new(self, 2)
    }
    #[doc = "Bit 3 - U0 U74MC Reset Vector 3: 35"]
    #[inline(always)]
    #[must_use]
    pub fn reset_vector_3_35(&mut self) -> RESET_VECTOR_3_35_W<SYS_SYSCFG_31_SPEC> {
        RESET_VECTOR_3_35_W::new(self, 3)
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
#[doc = "SYS SYSCONSAIF SYSCFG 31\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`sys_syscfg_31::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`sys_syscfg_31::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct SYS_SYSCFG_31_SPEC;
impl crate::RegisterSpec for SYS_SYSCFG_31_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`sys_syscfg_31::R`](R) reader structure"]
impl crate::Readable for SYS_SYSCFG_31_SPEC {}
#[doc = "`write(|w| ..)` method takes [`sys_syscfg_31::W`](W) writer structure"]
impl crate::Writable for SYS_SYSCFG_31_SPEC {
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets sys_syscfg_31 to value 0"]
impl crate::Resettable for SYS_SYSCFG_31_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
