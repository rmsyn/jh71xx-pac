#[doc = "Register `aon_iomux_cfgsaif_syscfg84` reader"]
pub type R = crate::R<AON_IOMUX_CFGSAIF_SYSCFG84_SPEC>;
#[doc = "Register `aon_iomux_cfgsaif_syscfg84` writer"]
pub type W = crate::W<AON_IOMUX_CFGSAIF_SYSCFG84_SPEC>;
#[doc = "Field `padcfg_pad_osc_ds` reader - Output Drive Strength (DS): * 00: The rated drive strength is 2 mA. * 01: The rated drive strength is 4 mA. * 10: The rated drive strength is 8 mA. * 11: The rated drive strength is 12 mA."]
pub type PADCFG_PAD_OSC_DS_R = crate::FieldReader;
#[doc = "Field `padcfg_pad_osc_ds` writer - Output Drive Strength (DS): * 00: The rated drive strength is 2 mA. * 01: The rated drive strength is 4 mA. * 10: The rated drive strength is 8 mA. * 11: The rated drive strength is 12 mA."]
pub type PADCFG_PAD_OSC_DS_W<'a, REG, const O: u8> = crate::FieldWriter<'a, REG, 2, O>;
impl R {
    #[doc = "Bits 0:1 - Output Drive Strength (DS): * 00: The rated drive strength is 2 mA. * 01: The rated drive strength is 4 mA. * 10: The rated drive strength is 8 mA. * 11: The rated drive strength is 12 mA."]
    #[inline(always)]
    pub fn padcfg_pad_osc_ds(&self) -> PADCFG_PAD_OSC_DS_R {
        PADCFG_PAD_OSC_DS_R::new((self.bits & 3) as u8)
    }
}
impl W {
    #[doc = "Bits 0:1 - Output Drive Strength (DS): * 00: The rated drive strength is 2 mA. * 01: The rated drive strength is 4 mA. * 10: The rated drive strength is 8 mA. * 11: The rated drive strength is 12 mA."]
    #[inline(always)]
    #[must_use]
    pub fn padcfg_pad_osc_ds(&mut self) -> PADCFG_PAD_OSC_DS_W<AON_IOMUX_CFGSAIF_SYSCFG84_SPEC, 0> {
        PADCFG_PAD_OSC_DS_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.bits = bits;
        self
    }
}
#[doc = "AON IOMUX CFG SAIF SYSCFG 84\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`aon_iomux_cfgsaif_syscfg84::R`](R).  You can [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`aon_iomux_cfgsaif_syscfg84::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct AON_IOMUX_CFGSAIF_SYSCFG84_SPEC;
impl crate::RegisterSpec for AON_IOMUX_CFGSAIF_SYSCFG84_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`aon_iomux_cfgsaif_syscfg84::R`](R) reader structure"]
impl crate::Readable for AON_IOMUX_CFGSAIF_SYSCFG84_SPEC {}
#[doc = "`write(|w| ..)` method takes [`aon_iomux_cfgsaif_syscfg84::W`](W) writer structure"]
impl crate::Writable for AON_IOMUX_CFGSAIF_SYSCFG84_SPEC {
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
