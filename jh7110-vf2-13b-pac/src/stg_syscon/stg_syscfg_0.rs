#[doc = "Register `stg_syscfg_0` reader"]
pub type R = crate::R<STG_SYSCFG_0_SPEC>;
#[doc = "Register `stg_syscfg_0` writer"]
pub type W = crate::W<STG_SYSCFG_0_SPEC>;
#[doc = "Field `scfg_hprot_sd_0` reader - scfg_hprot_sd_0"]
pub type SCFG_HPROT_SD_0_R = crate::FieldReader;
#[doc = "Field `scfg_hprot_sd_0` writer - scfg_hprot_sd_0"]
pub type SCFG_HPROT_SD_0_W<'a, REG> = crate::FieldWriter<'a, REG, 4>;
#[doc = "Field `scfg_hprot_sd_1` reader - scfg_hprot_sd_1"]
pub type SCFG_HPROT_SD_1_R = crate::FieldReader;
#[doc = "Field `scfg_hprot_sd_1` writer - scfg_hprot_sd_1"]
pub type SCFG_HPROT_SD_1_W<'a, REG> = crate::FieldWriter<'a, REG, 4>;
#[doc = "Field `u0_cdn_usb_adp_en` reader - u0_cdn_usb_adp_en"]
pub type U0_CDN_USB_ADP_EN_R = crate::BitReader;
#[doc = "Field `u0_cdn_usb_adp_probe_ana` reader - u0_cdn_usb_adp_probe_ana"]
pub type U0_CDN_USB_ADP_PROBE_ANA_R = crate::BitReader;
#[doc = "Field `u0_cdn_usb_adp_probe_ana` writer - u0_cdn_usb_adp_probe_ana"]
pub type U0_CDN_USB_ADP_PROBE_ANA_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `u0_cdn_usb_adp_probe_en` reader - u0_cdn_usb_adp_probe_en"]
pub type U0_CDN_USB_ADP_PROBE_EN_R = crate::BitReader;
#[doc = "Field `u0_cdn_usb_adp_sense_ana` reader - u0_cdn_usb_adp_sense_ana"]
pub type U0_CDN_USB_ADP_SENSE_ANA_R = crate::BitReader;
#[doc = "Field `u0_cdn_usb_adp_sense_ana` writer - u0_cdn_usb_adp_sense_ana"]
pub type U0_CDN_USB_ADP_SENSE_ANA_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `u0_cdn_usb_adp_sense_en` reader - u0_cdn_usb_adp_sense_en"]
pub type U0_CDN_USB_ADP_SENSE_EN_R = crate::BitReader;
#[doc = "Field `u0_cdn_usb_adp_sink_current_en` reader - u0_cdn_usb_adp_sink_current_en"]
pub type U0_CDN_USB_ADP_SINK_CURRENT_EN_R = crate::BitReader;
#[doc = "Field `u0_cdn_usb_adp_source_current_en` reader - u0_cdn_usb_adp_source_current_en"]
pub type U0_CDN_USB_ADP_SOURCE_CURRENT_EN_R = crate::BitReader;
#[doc = "Field `u0_cdn_usb_bc_en` reader - u0_cdn_usb_bc_en"]
pub type U0_CDN_USB_BC_EN_R = crate::BitReader;
#[doc = "Field `u0_cdn_usb_chrg_vbus` reader - u0_cdn_usb_chrg_vbus"]
pub type U0_CDN_USB_CHRG_VBUS_R = crate::BitReader;
#[doc = "Field `u0_cdn_usb_chrg_vbus` writer - u0_cdn_usb_chrg_vbus"]
pub type U0_CDN_USB_CHRG_VBUS_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `u0_cdn_usb_dcd_comp_sts` reader - u0_cdn_usb_dcd_comp_sts"]
pub type U0_CDN_USB_DCD_COMP_STS_R = crate::BitReader;
#[doc = "Field `u0_cdn_usb_dcd_comp_sts` writer - u0_cdn_usb_dcd_comp_sts"]
pub type U0_CDN_USB_DCD_COMP_STS_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `u0_cdn_usb_dischrg_vbus` reader - u0_cdn_usb_dischrg_vbus"]
pub type U0_CDN_USB_DISCHRG_VBUS_R = crate::BitReader;
#[doc = "Field `u0_cdn_usb_dischrg_vbus` writer - u0_cdn_usb_dischrg_vbus"]
pub type U0_CDN_USB_DISCHRG_VBUS_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `u0_cdn_usb_dm_vdat_ref_comp_en` reader - u0_cdn_usb_dm_vdat_ref_comp_en"]
pub type U0_CDN_USB_DM_VDAT_REF_COMP_EN_R = crate::BitReader;
#[doc = "Field `u0_cdn_usb_dm_vdat_ref_comp_sts` reader - u0_cdn_usb_dm_vdat_ref_comp_sts"]
pub type U0_CDN_USB_DM_VDAT_REF_COMP_STS_R = crate::BitReader;
#[doc = "Field `u0_cdn_usb_dm_vdat_ref_comp_sts` writer - u0_cdn_usb_dm_vdat_ref_comp_sts"]
pub type U0_CDN_USB_DM_VDAT_REF_COMP_STS_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `u0_cdn_usb_dm_vlgc_comp_en` reader - u0_cdn_usb_dm_vlgc_comp_en"]
pub type U0_CDN_USB_DM_VLGC_COMP_EN_R = crate::BitReader;
#[doc = "Field `u0_cdn_usb_dm_vlgc_comp_sts` reader - u0_cdn_usb_dm_vlgc_comp_sts"]
pub type U0_CDN_USB_DM_VLGC_COMP_STS_R = crate::BitReader;
#[doc = "Field `u0_cdn_usb_dm_vlgc_comp_sts` writer - u0_cdn_usb_dm_vlgc_comp_sts"]
pub type U0_CDN_USB_DM_VLGC_COMP_STS_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `u0_cdn_usb_dp_vdat_ref_comp_en` reader - u0_cdn_usb_dp_vdat_ref_comp_en"]
pub type U0_CDN_USB_DP_VDAT_REF_COMP_EN_R = crate::BitReader;
#[doc = "Field `u0_cdn_usb_dp_vdat_ref_comp_sts` reader - u0_cdn_usb_dp_vdat_ref_comp_sts"]
pub type U0_CDN_USB_DP_VDAT_REF_COMP_STS_R = crate::BitReader;
#[doc = "Field `u0_cdn_usb_dp_vdat_ref_comp_sts` writer - u0_cdn_usb_dp_vdat_ref_comp_sts"]
pub type U0_CDN_USB_DP_VDAT_REF_COMP_STS_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `u0_cdn_usb_host_system_err` reader - u0_cdn_usb_host_system_err"]
pub type U0_CDN_USB_HOST_SYSTEM_ERR_R = crate::BitReader;
#[doc = "Field `u0_cdn_usb_host_system_err` writer - u0_cdn_usb_host_system_err"]
pub type U0_CDN_USB_HOST_SYSTEM_ERR_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `u0_cdn_usb_hsystem_err_ext` reader - u0_cdn_usb_hsystem_err_ext"]
pub type U0_CDN_USB_HSYSTEM_ERR_EXT_R = crate::BitReader;
#[doc = "Field `u0_cdn_usb_idm_sink_en` reader - u0_cdn_usb_idm_sink_en"]
pub type U0_CDN_USB_IDM_SINK_EN_R = crate::BitReader;
#[doc = "Field `u0_cdn_usb_idp_sink_en` reader - u0_cdn_usb_idp_sink_en"]
pub type U0_CDN_USB_IDP_SINK_EN_R = crate::BitReader;
#[doc = "Field `u0_cdn_usb_idp_src_en` reader - u0_cdn_usb_idp_src_en"]
pub type U0_CDN_USB_IDP_SRC_EN_R = crate::BitReader;
impl R {
    #[doc = "Bits 0:3 - scfg_hprot_sd_0"]
    #[inline(always)]
    pub fn scfg_hprot_sd_0(&self) -> SCFG_HPROT_SD_0_R {
        SCFG_HPROT_SD_0_R::new((self.bits & 0x0f) as u8)
    }
    #[doc = "Bits 4:7 - scfg_hprot_sd_1"]
    #[inline(always)]
    pub fn scfg_hprot_sd_1(&self) -> SCFG_HPROT_SD_1_R {
        SCFG_HPROT_SD_1_R::new(((self.bits >> 4) & 0x0f) as u8)
    }
    #[doc = "Bit 8 - u0_cdn_usb_adp_en"]
    #[inline(always)]
    pub fn u0_cdn_usb_adp_en(&self) -> U0_CDN_USB_ADP_EN_R {
        U0_CDN_USB_ADP_EN_R::new(((self.bits >> 8) & 1) != 0)
    }
    #[doc = "Bit 9 - u0_cdn_usb_adp_probe_ana"]
    #[inline(always)]
    pub fn u0_cdn_usb_adp_probe_ana(&self) -> U0_CDN_USB_ADP_PROBE_ANA_R {
        U0_CDN_USB_ADP_PROBE_ANA_R::new(((self.bits >> 9) & 1) != 0)
    }
    #[doc = "Bit 10 - u0_cdn_usb_adp_probe_en"]
    #[inline(always)]
    pub fn u0_cdn_usb_adp_probe_en(&self) -> U0_CDN_USB_ADP_PROBE_EN_R {
        U0_CDN_USB_ADP_PROBE_EN_R::new(((self.bits >> 10) & 1) != 0)
    }
    #[doc = "Bit 11 - u0_cdn_usb_adp_sense_ana"]
    #[inline(always)]
    pub fn u0_cdn_usb_adp_sense_ana(&self) -> U0_CDN_USB_ADP_SENSE_ANA_R {
        U0_CDN_USB_ADP_SENSE_ANA_R::new(((self.bits >> 11) & 1) != 0)
    }
    #[doc = "Bit 12 - u0_cdn_usb_adp_sense_en"]
    #[inline(always)]
    pub fn u0_cdn_usb_adp_sense_en(&self) -> U0_CDN_USB_ADP_SENSE_EN_R {
        U0_CDN_USB_ADP_SENSE_EN_R::new(((self.bits >> 12) & 1) != 0)
    }
    #[doc = "Bit 13 - u0_cdn_usb_adp_sink_current_en"]
    #[inline(always)]
    pub fn u0_cdn_usb_adp_sink_current_en(&self) -> U0_CDN_USB_ADP_SINK_CURRENT_EN_R {
        U0_CDN_USB_ADP_SINK_CURRENT_EN_R::new(((self.bits >> 13) & 1) != 0)
    }
    #[doc = "Bit 14 - u0_cdn_usb_adp_source_current_en"]
    #[inline(always)]
    pub fn u0_cdn_usb_adp_source_current_en(&self) -> U0_CDN_USB_ADP_SOURCE_CURRENT_EN_R {
        U0_CDN_USB_ADP_SOURCE_CURRENT_EN_R::new(((self.bits >> 14) & 1) != 0)
    }
    #[doc = "Bit 15 - u0_cdn_usb_bc_en"]
    #[inline(always)]
    pub fn u0_cdn_usb_bc_en(&self) -> U0_CDN_USB_BC_EN_R {
        U0_CDN_USB_BC_EN_R::new(((self.bits >> 15) & 1) != 0)
    }
    #[doc = "Bit 16 - u0_cdn_usb_chrg_vbus"]
    #[inline(always)]
    pub fn u0_cdn_usb_chrg_vbus(&self) -> U0_CDN_USB_CHRG_VBUS_R {
        U0_CDN_USB_CHRG_VBUS_R::new(((self.bits >> 16) & 1) != 0)
    }
    #[doc = "Bit 17 - u0_cdn_usb_dcd_comp_sts"]
    #[inline(always)]
    pub fn u0_cdn_usb_dcd_comp_sts(&self) -> U0_CDN_USB_DCD_COMP_STS_R {
        U0_CDN_USB_DCD_COMP_STS_R::new(((self.bits >> 17) & 1) != 0)
    }
    #[doc = "Bit 18 - u0_cdn_usb_dischrg_vbus"]
    #[inline(always)]
    pub fn u0_cdn_usb_dischrg_vbus(&self) -> U0_CDN_USB_DISCHRG_VBUS_R {
        U0_CDN_USB_DISCHRG_VBUS_R::new(((self.bits >> 18) & 1) != 0)
    }
    #[doc = "Bit 19 - u0_cdn_usb_dm_vdat_ref_comp_en"]
    #[inline(always)]
    pub fn u0_cdn_usb_dm_vdat_ref_comp_en(&self) -> U0_CDN_USB_DM_VDAT_REF_COMP_EN_R {
        U0_CDN_USB_DM_VDAT_REF_COMP_EN_R::new(((self.bits >> 19) & 1) != 0)
    }
    #[doc = "Bit 20 - u0_cdn_usb_dm_vdat_ref_comp_sts"]
    #[inline(always)]
    pub fn u0_cdn_usb_dm_vdat_ref_comp_sts(&self) -> U0_CDN_USB_DM_VDAT_REF_COMP_STS_R {
        U0_CDN_USB_DM_VDAT_REF_COMP_STS_R::new(((self.bits >> 20) & 1) != 0)
    }
    #[doc = "Bit 21 - u0_cdn_usb_dm_vlgc_comp_en"]
    #[inline(always)]
    pub fn u0_cdn_usb_dm_vlgc_comp_en(&self) -> U0_CDN_USB_DM_VLGC_COMP_EN_R {
        U0_CDN_USB_DM_VLGC_COMP_EN_R::new(((self.bits >> 21) & 1) != 0)
    }
    #[doc = "Bit 22 - u0_cdn_usb_dm_vlgc_comp_sts"]
    #[inline(always)]
    pub fn u0_cdn_usb_dm_vlgc_comp_sts(&self) -> U0_CDN_USB_DM_VLGC_COMP_STS_R {
        U0_CDN_USB_DM_VLGC_COMP_STS_R::new(((self.bits >> 22) & 1) != 0)
    }
    #[doc = "Bit 23 - u0_cdn_usb_dp_vdat_ref_comp_en"]
    #[inline(always)]
    pub fn u0_cdn_usb_dp_vdat_ref_comp_en(&self) -> U0_CDN_USB_DP_VDAT_REF_COMP_EN_R {
        U0_CDN_USB_DP_VDAT_REF_COMP_EN_R::new(((self.bits >> 23) & 1) != 0)
    }
    #[doc = "Bit 24 - u0_cdn_usb_dp_vdat_ref_comp_sts"]
    #[inline(always)]
    pub fn u0_cdn_usb_dp_vdat_ref_comp_sts(&self) -> U0_CDN_USB_DP_VDAT_REF_COMP_STS_R {
        U0_CDN_USB_DP_VDAT_REF_COMP_STS_R::new(((self.bits >> 24) & 1) != 0)
    }
    #[doc = "Bit 25 - u0_cdn_usb_host_system_err"]
    #[inline(always)]
    pub fn u0_cdn_usb_host_system_err(&self) -> U0_CDN_USB_HOST_SYSTEM_ERR_R {
        U0_CDN_USB_HOST_SYSTEM_ERR_R::new(((self.bits >> 25) & 1) != 0)
    }
    #[doc = "Bit 26 - u0_cdn_usb_hsystem_err_ext"]
    #[inline(always)]
    pub fn u0_cdn_usb_hsystem_err_ext(&self) -> U0_CDN_USB_HSYSTEM_ERR_EXT_R {
        U0_CDN_USB_HSYSTEM_ERR_EXT_R::new(((self.bits >> 26) & 1) != 0)
    }
    #[doc = "Bit 27 - u0_cdn_usb_idm_sink_en"]
    #[inline(always)]
    pub fn u0_cdn_usb_idm_sink_en(&self) -> U0_CDN_USB_IDM_SINK_EN_R {
        U0_CDN_USB_IDM_SINK_EN_R::new(((self.bits >> 27) & 1) != 0)
    }
    #[doc = "Bit 28 - u0_cdn_usb_idp_sink_en"]
    #[inline(always)]
    pub fn u0_cdn_usb_idp_sink_en(&self) -> U0_CDN_USB_IDP_SINK_EN_R {
        U0_CDN_USB_IDP_SINK_EN_R::new(((self.bits >> 28) & 1) != 0)
    }
    #[doc = "Bit 29 - u0_cdn_usb_idp_src_en"]
    #[inline(always)]
    pub fn u0_cdn_usb_idp_src_en(&self) -> U0_CDN_USB_IDP_SRC_EN_R {
        U0_CDN_USB_IDP_SRC_EN_R::new(((self.bits >> 29) & 1) != 0)
    }
}
impl W {
    #[doc = "Bits 0:3 - scfg_hprot_sd_0"]
    #[inline(always)]
    #[must_use]
    pub fn scfg_hprot_sd_0(&mut self) -> SCFG_HPROT_SD_0_W<STG_SYSCFG_0_SPEC> {
        SCFG_HPROT_SD_0_W::new(self, 0)
    }
    #[doc = "Bits 4:7 - scfg_hprot_sd_1"]
    #[inline(always)]
    #[must_use]
    pub fn scfg_hprot_sd_1(&mut self) -> SCFG_HPROT_SD_1_W<STG_SYSCFG_0_SPEC> {
        SCFG_HPROT_SD_1_W::new(self, 4)
    }
    #[doc = "Bit 9 - u0_cdn_usb_adp_probe_ana"]
    #[inline(always)]
    #[must_use]
    pub fn u0_cdn_usb_adp_probe_ana(&mut self) -> U0_CDN_USB_ADP_PROBE_ANA_W<STG_SYSCFG_0_SPEC> {
        U0_CDN_USB_ADP_PROBE_ANA_W::new(self, 9)
    }
    #[doc = "Bit 11 - u0_cdn_usb_adp_sense_ana"]
    #[inline(always)]
    #[must_use]
    pub fn u0_cdn_usb_adp_sense_ana(&mut self) -> U0_CDN_USB_ADP_SENSE_ANA_W<STG_SYSCFG_0_SPEC> {
        U0_CDN_USB_ADP_SENSE_ANA_W::new(self, 11)
    }
    #[doc = "Bit 16 - u0_cdn_usb_chrg_vbus"]
    #[inline(always)]
    #[must_use]
    pub fn u0_cdn_usb_chrg_vbus(&mut self) -> U0_CDN_USB_CHRG_VBUS_W<STG_SYSCFG_0_SPEC> {
        U0_CDN_USB_CHRG_VBUS_W::new(self, 16)
    }
    #[doc = "Bit 17 - u0_cdn_usb_dcd_comp_sts"]
    #[inline(always)]
    #[must_use]
    pub fn u0_cdn_usb_dcd_comp_sts(&mut self) -> U0_CDN_USB_DCD_COMP_STS_W<STG_SYSCFG_0_SPEC> {
        U0_CDN_USB_DCD_COMP_STS_W::new(self, 17)
    }
    #[doc = "Bit 18 - u0_cdn_usb_dischrg_vbus"]
    #[inline(always)]
    #[must_use]
    pub fn u0_cdn_usb_dischrg_vbus(&mut self) -> U0_CDN_USB_DISCHRG_VBUS_W<STG_SYSCFG_0_SPEC> {
        U0_CDN_USB_DISCHRG_VBUS_W::new(self, 18)
    }
    #[doc = "Bit 20 - u0_cdn_usb_dm_vdat_ref_comp_sts"]
    #[inline(always)]
    #[must_use]
    pub fn u0_cdn_usb_dm_vdat_ref_comp_sts(
        &mut self,
    ) -> U0_CDN_USB_DM_VDAT_REF_COMP_STS_W<STG_SYSCFG_0_SPEC> {
        U0_CDN_USB_DM_VDAT_REF_COMP_STS_W::new(self, 20)
    }
    #[doc = "Bit 22 - u0_cdn_usb_dm_vlgc_comp_sts"]
    #[inline(always)]
    #[must_use]
    pub fn u0_cdn_usb_dm_vlgc_comp_sts(
        &mut self,
    ) -> U0_CDN_USB_DM_VLGC_COMP_STS_W<STG_SYSCFG_0_SPEC> {
        U0_CDN_USB_DM_VLGC_COMP_STS_W::new(self, 22)
    }
    #[doc = "Bit 24 - u0_cdn_usb_dp_vdat_ref_comp_sts"]
    #[inline(always)]
    #[must_use]
    pub fn u0_cdn_usb_dp_vdat_ref_comp_sts(
        &mut self,
    ) -> U0_CDN_USB_DP_VDAT_REF_COMP_STS_W<STG_SYSCFG_0_SPEC> {
        U0_CDN_USB_DP_VDAT_REF_COMP_STS_W::new(self, 24)
    }
    #[doc = "Bit 25 - u0_cdn_usb_host_system_err"]
    #[inline(always)]
    #[must_use]
    pub fn u0_cdn_usb_host_system_err(
        &mut self,
    ) -> U0_CDN_USB_HOST_SYSTEM_ERR_W<STG_SYSCFG_0_SPEC> {
        U0_CDN_USB_HOST_SYSTEM_ERR_W::new(self, 25)
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
#[doc = "STG SYCONSAIF SYSCFG 0\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`stg_syscfg_0::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`stg_syscfg_0::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct STG_SYSCFG_0_SPEC;
impl crate::RegisterSpec for STG_SYSCFG_0_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`stg_syscfg_0::R`](R) reader structure"]
impl crate::Readable for STG_SYSCFG_0_SPEC {}
#[doc = "`write(|w| ..)` method takes [`stg_syscfg_0::W`](W) writer structure"]
impl crate::Writable for STG_SYSCFG_0_SPEC {
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets stg_syscfg_0 to value 0"]
impl crate::Resettable for STG_SYSCFG_0_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
