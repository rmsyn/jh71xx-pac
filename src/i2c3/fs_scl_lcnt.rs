#[doc = "Register `fs_scl_lcnt` reader"]
pub type R = crate::R<FS_SCL_LCNT_SPEC>;
#[doc = "Register `fs_scl_lcnt` writer"]
pub type W = crate::W<FS_SCL_LCNT_SPEC>;
#[doc = "Field `fs_scl_lcnt` reader - fs_scl_lcnt"]
pub type FS_SCL_LCNT_R = crate::FieldReader<u32>;
#[doc = "Field `fs_scl_lcnt` writer - fs_scl_lcnt"]
pub type FS_SCL_LCNT_W<'a, REG> = crate::FieldWriter<'a, REG, 32, u32>;
impl R {
    #[doc = "Bits 0:31 - fs_scl_lcnt"]
    #[inline(always)]
    pub fn fs_scl_lcnt(&self) -> FS_SCL_LCNT_R {
        FS_SCL_LCNT_R::new(self.bits)
    }
}
impl W {
    #[doc = "Bits 0:31 - fs_scl_lcnt"]
    #[inline(always)]
    #[must_use]
    pub fn fs_scl_lcnt(&mut self) -> FS_SCL_LCNT_W<FS_SCL_LCNT_SPEC> {
        FS_SCL_LCNT_W::new(self, 0)
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
#[doc = "DesignWare I2C FS SCL LCNT\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`fs_scl_lcnt::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`fs_scl_lcnt::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct FS_SCL_LCNT_SPEC;
impl crate::RegisterSpec for FS_SCL_LCNT_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`fs_scl_lcnt::R`](R) reader structure"]
impl crate::Readable for FS_SCL_LCNT_SPEC {}
#[doc = "`write(|w| ..)` method takes [`fs_scl_lcnt::W`](W) writer structure"]
impl crate::Writable for FS_SCL_LCNT_SPEC {
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets fs_scl_lcnt to value 0"]
impl crate::Resettable for FS_SCL_LCNT_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
