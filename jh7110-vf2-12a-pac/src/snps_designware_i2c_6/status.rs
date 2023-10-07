#[doc = "Register `status` reader"]
pub type R = crate::R<STATUS_SPEC>;
#[doc = "Register `status` writer"]
pub type W = crate::W<STATUS_SPEC>;
#[doc = "Field `activity` reader - activity"]
pub type ACTIVITY_R = crate::BitReader;
#[doc = "Field `tfe` reader - tfe"]
pub type TFE_R = crate::BitReader;
#[doc = "Field `rfne` reader - rfne"]
pub type RFNE_R = crate::BitReader;
#[doc = "Field `master_activity` reader - master_activity"]
pub type MASTER_ACTIVITY_R = crate::BitReader;
#[doc = "Field `slave_activity` reader - slave_activity"]
pub type SLAVE_ACTIVITY_R = crate::BitReader;
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
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.bits = bits;
        self
    }
}
#[doc = "DesignWare I2C Status\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`status::R`](R).  You can [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`status::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct STATUS_SPEC;
impl crate::RegisterSpec for STATUS_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`status::R`](R) reader structure"]
impl crate::Readable for STATUS_SPEC {}
#[doc = "`write(|w| ..)` method takes [`status::W`](W) writer structure"]
impl crate::Writable for STATUS_SPEC {
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
