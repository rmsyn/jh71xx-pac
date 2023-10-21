#[doc = "Register `stg_sysconsaif_syscfg8` reader"]
pub type R = crate::R<STG_SYSCONSAIF_SYSCFG8_SPEC>;
#[doc = "Register `stg_sysconsaif_syscfg8` writer"]
pub type W = crate::W<STG_SYSCONSAIF_SYSCFG8_SPEC>;
#[doc = "Field `u0_cdn_usb_rx_dp` reader - u0_cdn_usb_rx_dp"]
pub type U0_CDN_USB_RX_DP_R = crate::BitReader;
#[doc = "Field `u0_cdn_usb_rx_rcv` reader - u0_cdn_usb_rx_rcv"]
pub type U0_CDN_USB_RX_RCV_R = crate::BitReader;
#[doc = "Field `u0_cdn_usb_self_test` reader - For software bist_test"]
pub type U0_CDN_USB_SELF_TEST_R = crate::BitReader;
#[doc = "Field `u0_cdn_usb_self_test` writer - For software bist_test"]
pub type U0_CDN_USB_SELF_TEST_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O>;
#[doc = "Field `u0_cdn_usb_sessend` reader - u0_cdn_usb_sessend"]
pub type U0_CDN_USB_SESSEND_R = crate::BitReader;
#[doc = "Field `u0_cdn_usb_sessvalid` reader - u0_cdn_usb_sessvalid"]
pub type U0_CDN_USB_SESSVALID_R = crate::BitReader;
#[doc = "Field `u0_cdn_usb_sof` reader - u0_cdn_usb_sof"]
pub type U0_CDN_USB_SOF_R = crate::BitReader;
#[doc = "Field `u0_cdn_usb_test_bist` reader - For software bist_test"]
pub type U0_CDN_USB_TEST_BIST_R = crate::BitReader;
#[doc = "Field `u0_cdn_usb_usbdev_main_power_off_ack` reader - u0_cdn_usb_usbdev_main_power_off_ack"]
pub type U0_CDN_USB_USBDEV_MAIN_POWER_OFF_ACK_R = crate::BitReader;
#[doc = "Field `u0_cdn_usb_usbdev_main_power_off_ready` reader - u0_cdn_usb_usbdev_main_power_off_ready"]
pub type U0_CDN_USB_USBDEV_MAIN_POWER_OFF_READY_R = crate::BitReader;
#[doc = "Field `u0_cdn_usb_usbdev_main_power_off_req` reader - u0_cdn_usb_usbdev_main_power_off_req"]
pub type U0_CDN_USB_USBDEV_MAIN_POWER_OFF_REQ_R = crate::BitReader;
#[doc = "Field `u0_cdn_usb_usbdev_main_power_off_req` writer - u0_cdn_usb_usbdev_main_power_off_req"]
pub type U0_CDN_USB_USBDEV_MAIN_POWER_OFF_REQ_W<'a, REG, const O: u8> =
    crate::BitWriter<'a, REG, O>;
#[doc = "Field `u0_cdn_usb_usbdev_main_power_on_ready` reader - u0_cdn_usb_usbdev_main_power_on_ready"]
pub type U0_CDN_USB_USBDEV_MAIN_POWER_ON_READY_R = crate::BitReader;
#[doc = "Field `u0_cdn_usb_usbdev_main_power_on_req` reader - u0_cdn_usb_usbdev_main_power_on_req"]
pub type U0_CDN_USB_USBDEV_MAIN_POWER_ON_REQ_R = crate::BitReader;
#[doc = "Field `u0_cdn_usb_usbdev_main_power_on_valid` reader - u0_cdn_usb_usbdev_main_power_on_valid"]
pub type U0_CDN_USB_USBDEV_MAIN_POWER_ON_VALID_R = crate::BitReader;
#[doc = "Field `u0_cdn_usb_usbdev_main_power_on_valid` writer - u0_cdn_usb_usbdev_main_power_on_valid"]
pub type U0_CDN_USB_USBDEV_MAIN_POWER_ON_VALID_W<'a, REG, const O: u8> =
    crate::BitWriter<'a, REG, O>;
#[doc = "Field `u0_cdn_usb_usbdev_power_off_ack` reader - u0_cdn_usb_usbdev_power_off_ack"]
pub type U0_CDN_USB_USBDEV_POWER_OFF_ACK_R = crate::BitReader;
#[doc = "Field `u0_cdn_usb_usbdev_power_off_ready` reader - u0_cdn_usb_usbdev_power_off_ready"]
pub type U0_CDN_USB_USBDEV_POWER_OFF_READY_R = crate::BitReader;
#[doc = "Field `u0_cdn_usb_usbdev_power_off_req` reader - u0_cdn_usb_usbdev_power_off_req"]
pub type U0_CDN_USB_USBDEV_POWER_OFF_REQ_R = crate::BitReader;
#[doc = "Field `u0_cdn_usb_usbdev_power_off_req` writer - u0_cdn_usb_usbdev_power_off_req"]
pub type U0_CDN_USB_USBDEV_POWER_OFF_REQ_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O>;
#[doc = "Field `u0_cdn_usb_usbdev_power_on_ready` reader - u0_cdn_usb_usbdev_power_on_ready"]
pub type U0_CDN_USB_USBDEV_POWER_ON_READY_R = crate::BitReader;
#[doc = "Field `u0_cdn_usb_usbdev_power_on_req` reader - u0_cdn_usb_usbdev_power_on_req"]
pub type U0_CDN_USB_USBDEV_POWER_ON_REQ_R = crate::BitReader;
#[doc = "Field `u0_cdn_usb_usbdev_power_on_valid` reader - u0_cdn_usb_usbdev_power_on_valid"]
pub type U0_CDN_USB_USBDEV_POWER_ON_VALID_R = crate::BitReader;
#[doc = "Field `u0_cdn_usb_usbdev_power_on_valid` writer - u0_cdn_usb_usbdev_power_on_valid"]
pub type U0_CDN_USB_USBDEV_POWER_ON_VALID_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O>;
#[doc = "Field `u0_cdn_usb_utmi_dmpulldown_sit` reader - u0_cdn_usb_utmi_dmpulldown_sit"]
pub type U0_CDN_USB_UTMI_DMPULLDOWN_SIT_R = crate::BitReader;
#[doc = "Field `u0_cdn_usb_utmi_dmpulldown_sit` writer - u0_cdn_usb_utmi_dmpulldown_sit"]
pub type U0_CDN_USB_UTMI_DMPULLDOWN_SIT_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O>;
#[doc = "Field `u0_cdn_usb_utmi_dppulldown_sit` reader - u0_cdn_usb_utmi_dppulldown_sit"]
pub type U0_CDN_USB_UTMI_DPPULLDOWN_SIT_R = crate::BitReader;
#[doc = "Field `u0_cdn_usb_utmi_dppulldown_sit` writer - u0_cdn_usb_utmi_dppulldown_sit"]
pub type U0_CDN_USB_UTMI_DPPULLDOWN_SIT_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O>;
#[doc = "Field `u0_cdn_usb_utmi_fslsserialmode_sit` reader - u0_cdn_usb_utmi_fslsserialmode_sit"]
pub type U0_CDN_USB_UTMI_FSLSSERIALMODE_SIT_R = crate::BitReader;
#[doc = "Field `u0_cdn_usb_utmi_fslsserialmode_sit` writer - u0_cdn_usb_utmi_fslsserialmode_sit"]
pub type U0_CDN_USB_UTMI_FSLSSERIALMODE_SIT_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O>;
#[doc = "Field `u0_cdn_usb_utmi_hostdisconnect_sit` reader - u0_cdn_usb_utmi_hostdisconnect_sit"]
pub type U0_CDN_USB_UTMI_HOSTDISCONNECT_SIT_R = crate::BitReader;
#[doc = "Field `u0_cdn_usb_utmi_iddig_sit` reader - u0_cdn_usb_utmi_iddig_sit"]
pub type U0_CDN_USB_UTMI_IDDIG_SIT_R = crate::BitReader;
#[doc = "Field `u0_cdn_usb_utmi_idpullup_sit` reader - u0_cdn_usb_utmi_idpullup_sit"]
pub type U0_CDN_USB_UTMI_IDPULLUP_SIT_R = crate::BitReader;
#[doc = "Field `u0_cdn_usb_utmi_idpullup_sit` writer - u0_cdn_usb_utmi_idpullup_sit"]
pub type U0_CDN_USB_UTMI_IDPULLUP_SIT_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O>;
#[doc = "Field `u0_cdn_usb_utmi_linestate_sit` reader - u0_cdn_usb_utmi_linestate_sit"]
pub type U0_CDN_USB_UTMI_LINESTATE_SIT_R = crate::FieldReader;
#[doc = "Field `u0_cdn_usb_utmi_opmode_sit` reader - u0_cdn_usb_utmi_opmode_sit"]
pub type U0_CDN_USB_UTMI_OPMODE_SIT_R = crate::FieldReader;
#[doc = "Field `u0_cdn_usb_utmi_opmode_sit` writer - u0_cdn_usb_utmi_opmode_sit"]
pub type U0_CDN_USB_UTMI_OPMODE_SIT_W<'a, REG, const O: u8> = crate::FieldWriter<'a, REG, 2, O>;
#[doc = "Field `u0_cdn_usb_utmi_rxactive_sit` reader - u0_cdn_usb_utmi_rxactive_sit"]
pub type U0_CDN_USB_UTMI_RXACTIVE_SIT_R = crate::BitReader;
#[doc = "Field `u0_cdn_usb_utmi_rxerror_sit` reader - u0_cdn_usb_utmi_rxerror_sit"]
pub type U0_CDN_USB_UTMI_RXERROR_SIT_R = crate::BitReader;
#[doc = "Field `u0_cdn_usb_utmi_rxvalid_sit` reader - u0_cdn_usb_utmi_rxvalid_sit"]
pub type U0_CDN_USB_UTMI_RXVALID_SIT_R = crate::BitReader;
impl R {
    #[doc = "Bit 0 - u0_cdn_usb_rx_dp"]
    #[inline(always)]
    pub fn u0_cdn_usb_rx_dp(&self) -> U0_CDN_USB_RX_DP_R {
        U0_CDN_USB_RX_DP_R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - u0_cdn_usb_rx_rcv"]
    #[inline(always)]
    pub fn u0_cdn_usb_rx_rcv(&self) -> U0_CDN_USB_RX_RCV_R {
        U0_CDN_USB_RX_RCV_R::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - For software bist_test"]
    #[inline(always)]
    pub fn u0_cdn_usb_self_test(&self) -> U0_CDN_USB_SELF_TEST_R {
        U0_CDN_USB_SELF_TEST_R::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3 - u0_cdn_usb_sessend"]
    #[inline(always)]
    pub fn u0_cdn_usb_sessend(&self) -> U0_CDN_USB_SESSEND_R {
        U0_CDN_USB_SESSEND_R::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bit 4 - u0_cdn_usb_sessvalid"]
    #[inline(always)]
    pub fn u0_cdn_usb_sessvalid(&self) -> U0_CDN_USB_SESSVALID_R {
        U0_CDN_USB_SESSVALID_R::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bit 5 - u0_cdn_usb_sof"]
    #[inline(always)]
    pub fn u0_cdn_usb_sof(&self) -> U0_CDN_USB_SOF_R {
        U0_CDN_USB_SOF_R::new(((self.bits >> 5) & 1) != 0)
    }
    #[doc = "Bit 6 - For software bist_test"]
    #[inline(always)]
    pub fn u0_cdn_usb_test_bist(&self) -> U0_CDN_USB_TEST_BIST_R {
        U0_CDN_USB_TEST_BIST_R::new(((self.bits >> 6) & 1) != 0)
    }
    #[doc = "Bit 7 - u0_cdn_usb_usbdev_main_power_off_ack"]
    #[inline(always)]
    pub fn u0_cdn_usb_usbdev_main_power_off_ack(&self) -> U0_CDN_USB_USBDEV_MAIN_POWER_OFF_ACK_R {
        U0_CDN_USB_USBDEV_MAIN_POWER_OFF_ACK_R::new(((self.bits >> 7) & 1) != 0)
    }
    #[doc = "Bit 8 - u0_cdn_usb_usbdev_main_power_off_ready"]
    #[inline(always)]
    pub fn u0_cdn_usb_usbdev_main_power_off_ready(
        &self,
    ) -> U0_CDN_USB_USBDEV_MAIN_POWER_OFF_READY_R {
        U0_CDN_USB_USBDEV_MAIN_POWER_OFF_READY_R::new(((self.bits >> 8) & 1) != 0)
    }
    #[doc = "Bit 9 - u0_cdn_usb_usbdev_main_power_off_req"]
    #[inline(always)]
    pub fn u0_cdn_usb_usbdev_main_power_off_req(&self) -> U0_CDN_USB_USBDEV_MAIN_POWER_OFF_REQ_R {
        U0_CDN_USB_USBDEV_MAIN_POWER_OFF_REQ_R::new(((self.bits >> 9) & 1) != 0)
    }
    #[doc = "Bit 10 - u0_cdn_usb_usbdev_main_power_on_ready"]
    #[inline(always)]
    pub fn u0_cdn_usb_usbdev_main_power_on_ready(&self) -> U0_CDN_USB_USBDEV_MAIN_POWER_ON_READY_R {
        U0_CDN_USB_USBDEV_MAIN_POWER_ON_READY_R::new(((self.bits >> 10) & 1) != 0)
    }
    #[doc = "Bit 11 - u0_cdn_usb_usbdev_main_power_on_req"]
    #[inline(always)]
    pub fn u0_cdn_usb_usbdev_main_power_on_req(&self) -> U0_CDN_USB_USBDEV_MAIN_POWER_ON_REQ_R {
        U0_CDN_USB_USBDEV_MAIN_POWER_ON_REQ_R::new(((self.bits >> 11) & 1) != 0)
    }
    #[doc = "Bit 12 - u0_cdn_usb_usbdev_main_power_on_valid"]
    #[inline(always)]
    pub fn u0_cdn_usb_usbdev_main_power_on_valid(&self) -> U0_CDN_USB_USBDEV_MAIN_POWER_ON_VALID_R {
        U0_CDN_USB_USBDEV_MAIN_POWER_ON_VALID_R::new(((self.bits >> 12) & 1) != 0)
    }
    #[doc = "Bit 13 - u0_cdn_usb_usbdev_power_off_ack"]
    #[inline(always)]
    pub fn u0_cdn_usb_usbdev_power_off_ack(&self) -> U0_CDN_USB_USBDEV_POWER_OFF_ACK_R {
        U0_CDN_USB_USBDEV_POWER_OFF_ACK_R::new(((self.bits >> 13) & 1) != 0)
    }
    #[doc = "Bit 14 - u0_cdn_usb_usbdev_power_off_ready"]
    #[inline(always)]
    pub fn u0_cdn_usb_usbdev_power_off_ready(&self) -> U0_CDN_USB_USBDEV_POWER_OFF_READY_R {
        U0_CDN_USB_USBDEV_POWER_OFF_READY_R::new(((self.bits >> 14) & 1) != 0)
    }
    #[doc = "Bit 15 - u0_cdn_usb_usbdev_power_off_req"]
    #[inline(always)]
    pub fn u0_cdn_usb_usbdev_power_off_req(&self) -> U0_CDN_USB_USBDEV_POWER_OFF_REQ_R {
        U0_CDN_USB_USBDEV_POWER_OFF_REQ_R::new(((self.bits >> 15) & 1) != 0)
    }
    #[doc = "Bit 16 - u0_cdn_usb_usbdev_power_on_ready"]
    #[inline(always)]
    pub fn u0_cdn_usb_usbdev_power_on_ready(&self) -> U0_CDN_USB_USBDEV_POWER_ON_READY_R {
        U0_CDN_USB_USBDEV_POWER_ON_READY_R::new(((self.bits >> 16) & 1) != 0)
    }
    #[doc = "Bit 17 - u0_cdn_usb_usbdev_power_on_req"]
    #[inline(always)]
    pub fn u0_cdn_usb_usbdev_power_on_req(&self) -> U0_CDN_USB_USBDEV_POWER_ON_REQ_R {
        U0_CDN_USB_USBDEV_POWER_ON_REQ_R::new(((self.bits >> 17) & 1) != 0)
    }
    #[doc = "Bit 18 - u0_cdn_usb_usbdev_power_on_valid"]
    #[inline(always)]
    pub fn u0_cdn_usb_usbdev_power_on_valid(&self) -> U0_CDN_USB_USBDEV_POWER_ON_VALID_R {
        U0_CDN_USB_USBDEV_POWER_ON_VALID_R::new(((self.bits >> 18) & 1) != 0)
    }
    #[doc = "Bit 19 - u0_cdn_usb_utmi_dmpulldown_sit"]
    #[inline(always)]
    pub fn u0_cdn_usb_utmi_dmpulldown_sit(&self) -> U0_CDN_USB_UTMI_DMPULLDOWN_SIT_R {
        U0_CDN_USB_UTMI_DMPULLDOWN_SIT_R::new(((self.bits >> 19) & 1) != 0)
    }
    #[doc = "Bit 20 - u0_cdn_usb_utmi_dppulldown_sit"]
    #[inline(always)]
    pub fn u0_cdn_usb_utmi_dppulldown_sit(&self) -> U0_CDN_USB_UTMI_DPPULLDOWN_SIT_R {
        U0_CDN_USB_UTMI_DPPULLDOWN_SIT_R::new(((self.bits >> 20) & 1) != 0)
    }
    #[doc = "Bit 21 - u0_cdn_usb_utmi_fslsserialmode_sit"]
    #[inline(always)]
    pub fn u0_cdn_usb_utmi_fslsserialmode_sit(&self) -> U0_CDN_USB_UTMI_FSLSSERIALMODE_SIT_R {
        U0_CDN_USB_UTMI_FSLSSERIALMODE_SIT_R::new(((self.bits >> 21) & 1) != 0)
    }
    #[doc = "Bit 22 - u0_cdn_usb_utmi_hostdisconnect_sit"]
    #[inline(always)]
    pub fn u0_cdn_usb_utmi_hostdisconnect_sit(&self) -> U0_CDN_USB_UTMI_HOSTDISCONNECT_SIT_R {
        U0_CDN_USB_UTMI_HOSTDISCONNECT_SIT_R::new(((self.bits >> 22) & 1) != 0)
    }
    #[doc = "Bit 23 - u0_cdn_usb_utmi_iddig_sit"]
    #[inline(always)]
    pub fn u0_cdn_usb_utmi_iddig_sit(&self) -> U0_CDN_USB_UTMI_IDDIG_SIT_R {
        U0_CDN_USB_UTMI_IDDIG_SIT_R::new(((self.bits >> 23) & 1) != 0)
    }
    #[doc = "Bit 24 - u0_cdn_usb_utmi_idpullup_sit"]
    #[inline(always)]
    pub fn u0_cdn_usb_utmi_idpullup_sit(&self) -> U0_CDN_USB_UTMI_IDPULLUP_SIT_R {
        U0_CDN_USB_UTMI_IDPULLUP_SIT_R::new(((self.bits >> 24) & 1) != 0)
    }
    #[doc = "Bits 25:26 - u0_cdn_usb_utmi_linestate_sit"]
    #[inline(always)]
    pub fn u0_cdn_usb_utmi_linestate_sit(&self) -> U0_CDN_USB_UTMI_LINESTATE_SIT_R {
        U0_CDN_USB_UTMI_LINESTATE_SIT_R::new(((self.bits >> 25) & 3) as u8)
    }
    #[doc = "Bits 27:28 - u0_cdn_usb_utmi_opmode_sit"]
    #[inline(always)]
    pub fn u0_cdn_usb_utmi_opmode_sit(&self) -> U0_CDN_USB_UTMI_OPMODE_SIT_R {
        U0_CDN_USB_UTMI_OPMODE_SIT_R::new(((self.bits >> 27) & 3) as u8)
    }
    #[doc = "Bit 29 - u0_cdn_usb_utmi_rxactive_sit"]
    #[inline(always)]
    pub fn u0_cdn_usb_utmi_rxactive_sit(&self) -> U0_CDN_USB_UTMI_RXACTIVE_SIT_R {
        U0_CDN_USB_UTMI_RXACTIVE_SIT_R::new(((self.bits >> 29) & 1) != 0)
    }
    #[doc = "Bit 30 - u0_cdn_usb_utmi_rxerror_sit"]
    #[inline(always)]
    pub fn u0_cdn_usb_utmi_rxerror_sit(&self) -> U0_CDN_USB_UTMI_RXERROR_SIT_R {
        U0_CDN_USB_UTMI_RXERROR_SIT_R::new(((self.bits >> 30) & 1) != 0)
    }
    #[doc = "Bit 31 - u0_cdn_usb_utmi_rxvalid_sit"]
    #[inline(always)]
    pub fn u0_cdn_usb_utmi_rxvalid_sit(&self) -> U0_CDN_USB_UTMI_RXVALID_SIT_R {
        U0_CDN_USB_UTMI_RXVALID_SIT_R::new(((self.bits >> 31) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 2 - For software bist_test"]
    #[inline(always)]
    #[must_use]
    pub fn u0_cdn_usb_self_test(
        &mut self,
    ) -> U0_CDN_USB_SELF_TEST_W<STG_SYSCONSAIF_SYSCFG8_SPEC, 2> {
        U0_CDN_USB_SELF_TEST_W::new(self)
    }
    #[doc = "Bit 9 - u0_cdn_usb_usbdev_main_power_off_req"]
    #[inline(always)]
    #[must_use]
    pub fn u0_cdn_usb_usbdev_main_power_off_req(
        &mut self,
    ) -> U0_CDN_USB_USBDEV_MAIN_POWER_OFF_REQ_W<STG_SYSCONSAIF_SYSCFG8_SPEC, 9> {
        U0_CDN_USB_USBDEV_MAIN_POWER_OFF_REQ_W::new(self)
    }
    #[doc = "Bit 12 - u0_cdn_usb_usbdev_main_power_on_valid"]
    #[inline(always)]
    #[must_use]
    pub fn u0_cdn_usb_usbdev_main_power_on_valid(
        &mut self,
    ) -> U0_CDN_USB_USBDEV_MAIN_POWER_ON_VALID_W<STG_SYSCONSAIF_SYSCFG8_SPEC, 12> {
        U0_CDN_USB_USBDEV_MAIN_POWER_ON_VALID_W::new(self)
    }
    #[doc = "Bit 15 - u0_cdn_usb_usbdev_power_off_req"]
    #[inline(always)]
    #[must_use]
    pub fn u0_cdn_usb_usbdev_power_off_req(
        &mut self,
    ) -> U0_CDN_USB_USBDEV_POWER_OFF_REQ_W<STG_SYSCONSAIF_SYSCFG8_SPEC, 15> {
        U0_CDN_USB_USBDEV_POWER_OFF_REQ_W::new(self)
    }
    #[doc = "Bit 18 - u0_cdn_usb_usbdev_power_on_valid"]
    #[inline(always)]
    #[must_use]
    pub fn u0_cdn_usb_usbdev_power_on_valid(
        &mut self,
    ) -> U0_CDN_USB_USBDEV_POWER_ON_VALID_W<STG_SYSCONSAIF_SYSCFG8_SPEC, 18> {
        U0_CDN_USB_USBDEV_POWER_ON_VALID_W::new(self)
    }
    #[doc = "Bit 19 - u0_cdn_usb_utmi_dmpulldown_sit"]
    #[inline(always)]
    #[must_use]
    pub fn u0_cdn_usb_utmi_dmpulldown_sit(
        &mut self,
    ) -> U0_CDN_USB_UTMI_DMPULLDOWN_SIT_W<STG_SYSCONSAIF_SYSCFG8_SPEC, 19> {
        U0_CDN_USB_UTMI_DMPULLDOWN_SIT_W::new(self)
    }
    #[doc = "Bit 20 - u0_cdn_usb_utmi_dppulldown_sit"]
    #[inline(always)]
    #[must_use]
    pub fn u0_cdn_usb_utmi_dppulldown_sit(
        &mut self,
    ) -> U0_CDN_USB_UTMI_DPPULLDOWN_SIT_W<STG_SYSCONSAIF_SYSCFG8_SPEC, 20> {
        U0_CDN_USB_UTMI_DPPULLDOWN_SIT_W::new(self)
    }
    #[doc = "Bit 21 - u0_cdn_usb_utmi_fslsserialmode_sit"]
    #[inline(always)]
    #[must_use]
    pub fn u0_cdn_usb_utmi_fslsserialmode_sit(
        &mut self,
    ) -> U0_CDN_USB_UTMI_FSLSSERIALMODE_SIT_W<STG_SYSCONSAIF_SYSCFG8_SPEC, 21> {
        U0_CDN_USB_UTMI_FSLSSERIALMODE_SIT_W::new(self)
    }
    #[doc = "Bit 24 - u0_cdn_usb_utmi_idpullup_sit"]
    #[inline(always)]
    #[must_use]
    pub fn u0_cdn_usb_utmi_idpullup_sit(
        &mut self,
    ) -> U0_CDN_USB_UTMI_IDPULLUP_SIT_W<STG_SYSCONSAIF_SYSCFG8_SPEC, 24> {
        U0_CDN_USB_UTMI_IDPULLUP_SIT_W::new(self)
    }
    #[doc = "Bits 27:28 - u0_cdn_usb_utmi_opmode_sit"]
    #[inline(always)]
    #[must_use]
    pub fn u0_cdn_usb_utmi_opmode_sit(
        &mut self,
    ) -> U0_CDN_USB_UTMI_OPMODE_SIT_W<STG_SYSCONSAIF_SYSCFG8_SPEC, 27> {
        U0_CDN_USB_UTMI_OPMODE_SIT_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.bits = bits;
        self
    }
}
#[doc = "STG SYSCONSAIF SYSCFG 8\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`stg_sysconsaif_syscfg8::R`](R).  You can [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`stg_sysconsaif_syscfg8::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct STG_SYSCONSAIF_SYSCFG8_SPEC;
impl crate::RegisterSpec for STG_SYSCONSAIF_SYSCFG8_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`stg_sysconsaif_syscfg8::R`](R) reader structure"]
impl crate::Readable for STG_SYSCONSAIF_SYSCFG8_SPEC {}
#[doc = "`write(|w| ..)` method takes [`stg_sysconsaif_syscfg8::W`](W) writer structure"]
impl crate::Writable for STG_SYSCONSAIF_SYSCFG8_SPEC {
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
