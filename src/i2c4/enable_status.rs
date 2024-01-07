#[doc = "Register `enable_status` reader"]
pub type R = crate::R<ENABLE_STATUS_SPEC>;
#[doc = "Register `enable_status` writer"]
pub type W = crate::W<ENABLE_STATUS_SPEC>;
#[doc = "Field `activity` reader - activity"]
pub type ACTIVITY_R = crate::BitReader;
#[doc = "Field `activity` writer - activity"]
pub type ACTIVITY_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `tfe` reader - tfe"]
pub type TFE_R = crate::BitReader;
#[doc = "Field `tfe` writer - tfe"]
pub type TFE_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `rfne` reader - rfne"]
pub type RFNE_R = crate::BitReader;
#[doc = "Field `rfne` writer - rfne"]
pub type RFNE_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `master_activity` reader - master_activity"]
pub type MASTER_ACTIVITY_R = crate::BitReader;
#[doc = "Field `master_activity` writer - master_activity"]
pub type MASTER_ACTIVITY_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `slave_activity` reader - slave_activity"]
pub type SLAVE_ACTIVITY_R = crate::BitReader;
#[doc = "Field `slave_activity` writer - slave_activity"]
pub type SLAVE_ACTIVITY_W<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bit 0 - activity"]
    #[inline(always)]
    pub fn activity(&self) -> ACTIVITY_R {
        ACTIVITY_R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 2 - tfe"]
    #[inline(always)]
    pub fn tfe(&self) -> TFE_R {
        TFE_R::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3 - rfne"]
    #[inline(always)]
    pub fn rfne(&self) -> RFNE_R {
        RFNE_R::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bit 5 - master_activity"]
    #[inline(always)]
    pub fn master_activity(&self) -> MASTER_ACTIVITY_R {
        MASTER_ACTIVITY_R::new(((self.bits >> 5) & 1) != 0)
    }
    #[doc = "Bit 6 - slave_activity"]
    #[inline(always)]
    pub fn slave_activity(&self) -> SLAVE_ACTIVITY_R {
        SLAVE_ACTIVITY_R::new(((self.bits >> 6) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - activity"]
    #[inline(always)]
    #[must_use]
    pub fn activity(&mut self) -> ACTIVITY_W<ENABLE_STATUS_SPEC> {
        ACTIVITY_W::new(self, 0)
    }
    #[doc = "Bit 2 - tfe"]
    #[inline(always)]
    #[must_use]
    pub fn tfe(&mut self) -> TFE_W<ENABLE_STATUS_SPEC> {
        TFE_W::new(self, 2)
    }
    #[doc = "Bit 3 - rfne"]
    #[inline(always)]
    #[must_use]
    pub fn rfne(&mut self) -> RFNE_W<ENABLE_STATUS_SPEC> {
        RFNE_W::new(self, 3)
    }
    #[doc = "Bit 5 - master_activity"]
    #[inline(always)]
    #[must_use]
    pub fn master_activity(&mut self) -> MASTER_ACTIVITY_W<ENABLE_STATUS_SPEC> {
        MASTER_ACTIVITY_W::new(self, 5)
    }
    #[doc = "Bit 6 - slave_activity"]
    #[inline(always)]
    #[must_use]
    pub fn slave_activity(&mut self) -> SLAVE_ACTIVITY_W<ENABLE_STATUS_SPEC> {
        SLAVE_ACTIVITY_W::new(self, 6)
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
#[doc = "DesignWare I2C Enable Status\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`enable_status::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`enable_status::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct ENABLE_STATUS_SPEC;
impl crate::RegisterSpec for ENABLE_STATUS_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`enable_status::R`](R) reader structure"]
impl crate::Readable for ENABLE_STATUS_SPEC {}
#[doc = "`write(|w| ..)` method takes [`enable_status::W`](W) writer structure"]
impl crate::Writable for ENABLE_STATUS_SPEC {
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets enable_status to value 0"]
impl crate::Resettable for ENABLE_STATUS_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
