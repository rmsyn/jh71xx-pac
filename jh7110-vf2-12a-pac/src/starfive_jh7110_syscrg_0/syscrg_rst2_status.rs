#[doc = "Register `syscrg_rst2_status` reader"]
pub type R = crate::R<SYSCRG_RST2_STATUS_SPEC>;
#[doc = "Register `syscrg_rst2_status` writer"]
pub type W = crate::W<SYSCRG_RST2_STATUS_SPEC>;
#[doc = "Field `rstn_u0_sdio_rstn_ahb` reader - 1: Assert reset, 0: De-assert reset"]
pub type RSTN_U0_SDIO_RSTN_AHB_R = crate::BitReader;
#[doc = "Field `rstn_u0_sdio_rstn_ahb` writer - 1: Assert reset, 0: De-assert reset"]
pub type RSTN_U0_SDIO_RSTN_AHB_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O>;
#[doc = "Field `rstn_u1_sdi_rstn_ahb` reader - 1: Assert reset, 0: De-assert reset"]
pub type RSTN_U1_SDI_RSTN_AHB_R = crate::BitReader;
#[doc = "Field `rstn_u1_sdi_rstn_ahb` writer - 1: Assert reset, 0: De-assert reset"]
pub type RSTN_U1_SDI_RSTN_AHB_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O>;
#[doc = "Field `rstn_u1_gmac5_axi64_aresetn_i` reader - 1: Assert reset, 0: De-assert reset"]
pub type RSTN_U1_GMAC5_AXI64_ARESETN_I_R = crate::BitReader;
#[doc = "Field `rstn_u1_gmac5_axi64_aresetn_i` writer - 1: Assert reset, 0: De-assert reset"]
pub type RSTN_U1_GMAC5_AXI64_ARESETN_I_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O>;
#[doc = "Field `rstn_u1_gmac5_axi64_hresetn_n` reader - 1: Assert reset, 0: De-assert reset"]
pub type RSTN_U1_GMAC5_AXI64_HRESETN_N_R = crate::BitReader;
#[doc = "Field `rstn_u1_gmac5_axi64_hresetn_n` writer - 1: Assert reset, 0: De-assert reset"]
pub type RSTN_U1_GMAC5_AXI64_HRESETN_N_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O>;
#[doc = "Field `rstn_u0_mailbox_presetn` reader - 1: Assert reset, 0: De-assert reset"]
pub type RSTN_U0_MAILBOX_PRESETN_R = crate::BitReader;
#[doc = "Field `rstn_u0_mailbox_presetn` writer - 1: Assert reset, 0: De-assert reset"]
pub type RSTN_U0_MAILBOX_PRESETN_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O>;
#[doc = "Field `rstn_u0_ssp_spi_rstn_apb` reader - 1: Assert reset, 0: De-assert reset"]
pub type RSTN_U0_SSP_SPI_RSTN_APB_R = crate::BitReader;
#[doc = "Field `rstn_u0_ssp_spi_rstn_apb` writer - 1: Assert reset, 0: De-assert reset"]
pub type RSTN_U0_SSP_SPI_RSTN_APB_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O>;
#[doc = "Field `rstn_u1_ssp_spi_rstn_apb` reader - 1: Assert reset, 0: De-assert reset"]
pub type RSTN_U1_SSP_SPI_RSTN_APB_R = crate::BitReader;
#[doc = "Field `rstn_u1_ssp_spi_rstn_apb` writer - 1: Assert reset, 0: De-assert reset"]
pub type RSTN_U1_SSP_SPI_RSTN_APB_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O>;
#[doc = "Field `rstn_u2_ssp_spi_rstn_apb` reader - 1: Assert reset, 0: De-assert reset"]
pub type RSTN_U2_SSP_SPI_RSTN_APB_R = crate::BitReader;
#[doc = "Field `rstn_u2_ssp_spi_rstn_apb` writer - 1: Assert reset, 0: De-assert reset"]
pub type RSTN_U2_SSP_SPI_RSTN_APB_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O>;
#[doc = "Field `rstn_u3_ssp_spi_rstn_apb` reader - 1: Assert reset, 0: De-assert reset"]
pub type RSTN_U3_SSP_SPI_RSTN_APB_R = crate::BitReader;
#[doc = "Field `rstn_u3_ssp_spi_rstn_apb` writer - 1: Assert reset, 0: De-assert reset"]
pub type RSTN_U3_SSP_SPI_RSTN_APB_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O>;
#[doc = "Field `rstn_u4_ssp_spi_rstn_apb` reader - 1: Assert reset, 0: De-assert reset"]
pub type RSTN_U4_SSP_SPI_RSTN_APB_R = crate::BitReader;
#[doc = "Field `rstn_u4_ssp_spi_rstn_apb` writer - 1: Assert reset, 0: De-assert reset"]
pub type RSTN_U4_SSP_SPI_RSTN_APB_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O>;
#[doc = "Field `rstn_u5_ssp_spi_rstn_apb` reader - 1: Assert reset, 0: De-assert reset"]
pub type RSTN_U5_SSP_SPI_RSTN_APB_R = crate::BitReader;
#[doc = "Field `rstn_u5_ssp_spi_rstn_apb` writer - 1: Assert reset, 0: De-assert reset"]
pub type RSTN_U5_SSP_SPI_RSTN_APB_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O>;
#[doc = "Field `rstn_u6_ssp_spi_rstn_apb` reader - 1: Assert reset, 0: De-assert reset"]
pub type RSTN_U6_SSP_SPI_RSTN_APB_R = crate::BitReader;
#[doc = "Field `rstn_u6_ssp_spi_rstn_apb` writer - 1: Assert reset, 0: De-assert reset"]
pub type RSTN_U6_SSP_SPI_RSTN_APB_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O>;
#[doc = "Field `rstn_u0_i2c_rstn_apb` reader - 1: Assert reset, 0: De-assert reset"]
pub type RSTN_U0_I2C_RSTN_APB_R = crate::BitReader;
#[doc = "Field `rstn_u0_i2c_rstn_apb` writer - 1: Assert reset, 0: De-assert reset"]
pub type RSTN_U0_I2C_RSTN_APB_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O>;
#[doc = "Field `rstn_u1_i2c_rstn_apb` reader - 1: Assert reset, 0: De-assert reset"]
pub type RSTN_U1_I2C_RSTN_APB_R = crate::BitReader;
#[doc = "Field `rstn_u1_i2c_rstn_apb` writer - 1: Assert reset, 0: De-assert reset"]
pub type RSTN_U1_I2C_RSTN_APB_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O>;
#[doc = "Field `rstn_u2_i2c_rstn_apb` reader - 1: Assert reset, 0: De-assert reset"]
pub type RSTN_U2_I2C_RSTN_APB_R = crate::BitReader;
#[doc = "Field `rstn_u2_i2c_rstn_apb` writer - 1: Assert reset, 0: De-assert reset"]
pub type RSTN_U2_I2C_RSTN_APB_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O>;
#[doc = "Field `rstn_u3_i2c_rstn_apb` reader - 1: Assert reset, 0: De-assert reset"]
pub type RSTN_U3_I2C_RSTN_APB_R = crate::BitReader;
#[doc = "Field `rstn_u3_i2c_rstn_apb` writer - 1: Assert reset, 0: De-assert reset"]
pub type RSTN_U3_I2C_RSTN_APB_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O>;
#[doc = "Field `rstn_u4_i2c_rstn_apb` reader - 1: Assert reset, 0: De-assert reset"]
pub type RSTN_U4_I2C_RSTN_APB_R = crate::BitReader;
#[doc = "Field `rstn_u4_i2c_rstn_apb` writer - 1: Assert reset, 0: De-assert reset"]
pub type RSTN_U4_I2C_RSTN_APB_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O>;
#[doc = "Field `rstn_u5_i2c_rstn_apb` reader - 1: Assert reset, 0: De-assert reset"]
pub type RSTN_U5_I2C_RSTN_APB_R = crate::BitReader;
#[doc = "Field `rstn_u5_i2c_rstn_apb` writer - 1: Assert reset, 0: De-assert reset"]
pub type RSTN_U5_I2C_RSTN_APB_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O>;
#[doc = "Field `rstn_u6_i2c_rstn_apb` reader - 1: Assert reset, 0: De-assert reset"]
pub type RSTN_U6_I2C_RSTN_APB_R = crate::BitReader;
#[doc = "Field `rstn_u6_i2c_rstn_apb` writer - 1: Assert reset, 0: De-assert reset"]
pub type RSTN_U6_I2C_RSTN_APB_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O>;
#[doc = "Field `rstn_u0_uart_rstn_apb` reader - 1: Assert reset, 0: De-assert reset"]
pub type RSTN_U0_UART_RSTN_APB_R = crate::BitReader;
#[doc = "Field `rstn_u0_uart_rstn_apb` writer - 1: Assert reset, 0: De-assert reset"]
pub type RSTN_U0_UART_RSTN_APB_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O>;
#[doc = "Field `rstn_u0_uart_rstn_core` reader - 1: Assert reset, 0: De-assert reset"]
pub type RSTN_U0_UART_RSTN_CORE_R = crate::BitReader;
#[doc = "Field `rstn_u0_uart_rstn_core` writer - 1: Assert reset, 0: De-assert reset"]
pub type RSTN_U0_UART_RSTN_CORE_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O>;
#[doc = "Field `rstn_u1_uart_rstn_apb` reader - 1: Assert reset, 0: De-assert reset"]
pub type RSTN_U1_UART_RSTN_APB_R = crate::BitReader;
#[doc = "Field `rstn_u1_uart_rstn_apb` writer - 1: Assert reset, 0: De-assert reset"]
pub type RSTN_U1_UART_RSTN_APB_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O>;
#[doc = "Field `rstn_u1_uart_rstn_core` reader - 1: Assert reset, 0: De-assert reset"]
pub type RSTN_U1_UART_RSTN_CORE_R = crate::BitReader;
#[doc = "Field `rstn_u1_uart_rstn_core` writer - 1: Assert reset, 0: De-assert reset"]
pub type RSTN_U1_UART_RSTN_CORE_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O>;
#[doc = "Field `rstn_u2_uart_rstn_apb` reader - 1: Assert reset, 0: De-assert reset"]
pub type RSTN_U2_UART_RSTN_APB_R = crate::BitReader;
#[doc = "Field `rstn_u2_uart_rstn_apb` writer - 1: Assert reset, 0: De-assert reset"]
pub type RSTN_U2_UART_RSTN_APB_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O>;
#[doc = "Field `rstn_u2_uart_rstn_core` reader - 1: Assert reset, 0: De-assert reset"]
pub type RSTN_U2_UART_RSTN_CORE_R = crate::BitReader;
#[doc = "Field `rstn_u2_uart_rstn_core` writer - 1: Assert reset, 0: De-assert reset"]
pub type RSTN_U2_UART_RSTN_CORE_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O>;
#[doc = "Field `rstn_u3_uart_rstn_apb` reader - 1: Assert reset, 0: De-assert reset"]
pub type RSTN_U3_UART_RSTN_APB_R = crate::BitReader;
#[doc = "Field `rstn_u3_uart_rstn_apb` writer - 1: Assert reset, 0: De-assert reset"]
pub type RSTN_U3_UART_RSTN_APB_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O>;
#[doc = "Field `rstn_u3_uart_rstn_core` reader - 1: Assert reset, 0: De-assert reset"]
pub type RSTN_U3_UART_RSTN_CORE_R = crate::BitReader;
#[doc = "Field `rstn_u3_uart_rstn_core` writer - 1: Assert reset, 0: De-assert reset"]
pub type RSTN_U3_UART_RSTN_CORE_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O>;
#[doc = "Field `rstn_u4_uart_rstn_apb` reader - 1: Assert reset, 0: De-assert reset"]
pub type RSTN_U4_UART_RSTN_APB_R = crate::BitReader;
#[doc = "Field `rstn_u4_uart_rstn_apb` writer - 1: Assert reset, 0: De-assert reset"]
pub type RSTN_U4_UART_RSTN_APB_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O>;
#[doc = "Field `rstn_u4_uart_rstn_core` reader - 1: Assert reset, 0: De-assert reset"]
pub type RSTN_U4_UART_RSTN_CORE_R = crate::BitReader;
#[doc = "Field `rstn_u4_uart_rstn_core` writer - 1: Assert reset, 0: De-assert reset"]
pub type RSTN_U4_UART_RSTN_CORE_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O>;
#[doc = "Field `rstn_u5_uart_rstn_apb` reader - 1: Assert reset, 0: De-assert reset"]
pub type RSTN_U5_UART_RSTN_APB_R = crate::BitReader;
#[doc = "Field `rstn_u5_uart_rstn_apb` writer - 1: Assert reset, 0: De-assert reset"]
pub type RSTN_U5_UART_RSTN_APB_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O>;
#[doc = "Field `rstn_u6_uart_rstn_core` reader - 1: Assert reset, 0: De-assert reset"]
pub type RSTN_U6_UART_RSTN_CORE_R = crate::BitReader;
#[doc = "Field `rstn_u6_uart_rstn_core` writer - 1: Assert reset, 0: De-assert reset"]
pub type RSTN_U6_UART_RSTN_CORE_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O>;
#[doc = "Field `rstn_u0_cdns_spdif_rstn_apb` reader - 1: Assert reset, 0: De-assert reset"]
pub type RSTN_U0_CDNS_SPDIF_RSTN_APB_R = crate::BitReader;
#[doc = "Field `rstn_u0_cdns_spdif_rstn_apb` writer - 1: Assert reset, 0: De-assert reset"]
pub type RSTN_U0_CDNS_SPDIF_RSTN_APB_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O>;
impl R {
    #[doc = "Bit 0 - 1: Assert reset, 0: De-assert reset"]
    #[inline(always)]
    pub fn rstn_u0_sdio_rstn_ahb(&self) -> RSTN_U0_SDIO_RSTN_AHB_R {
        RSTN_U0_SDIO_RSTN_AHB_R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - 1: Assert reset, 0: De-assert reset"]
    #[inline(always)]
    pub fn rstn_u1_sdi_rstn_ahb(&self) -> RSTN_U1_SDI_RSTN_AHB_R {
        RSTN_U1_SDI_RSTN_AHB_R::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - 1: Assert reset, 0: De-assert reset"]
    #[inline(always)]
    pub fn rstn_u1_gmac5_axi64_aresetn_i(&self) -> RSTN_U1_GMAC5_AXI64_ARESETN_I_R {
        RSTN_U1_GMAC5_AXI64_ARESETN_I_R::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3 - 1: Assert reset, 0: De-assert reset"]
    #[inline(always)]
    pub fn rstn_u1_gmac5_axi64_hresetn_n(&self) -> RSTN_U1_GMAC5_AXI64_HRESETN_N_R {
        RSTN_U1_GMAC5_AXI64_HRESETN_N_R::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bit 4 - 1: Assert reset, 0: De-assert reset"]
    #[inline(always)]
    pub fn rstn_u0_mailbox_presetn(&self) -> RSTN_U0_MAILBOX_PRESETN_R {
        RSTN_U0_MAILBOX_PRESETN_R::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bit 5 - 1: Assert reset, 0: De-assert reset"]
    #[inline(always)]
    pub fn rstn_u0_ssp_spi_rstn_apb(&self) -> RSTN_U0_SSP_SPI_RSTN_APB_R {
        RSTN_U0_SSP_SPI_RSTN_APB_R::new(((self.bits >> 5) & 1) != 0)
    }
    #[doc = "Bit 6 - 1: Assert reset, 0: De-assert reset"]
    #[inline(always)]
    pub fn rstn_u1_ssp_spi_rstn_apb(&self) -> RSTN_U1_SSP_SPI_RSTN_APB_R {
        RSTN_U1_SSP_SPI_RSTN_APB_R::new(((self.bits >> 6) & 1) != 0)
    }
    #[doc = "Bit 7 - 1: Assert reset, 0: De-assert reset"]
    #[inline(always)]
    pub fn rstn_u2_ssp_spi_rstn_apb(&self) -> RSTN_U2_SSP_SPI_RSTN_APB_R {
        RSTN_U2_SSP_SPI_RSTN_APB_R::new(((self.bits >> 7) & 1) != 0)
    }
    #[doc = "Bit 8 - 1: Assert reset, 0: De-assert reset"]
    #[inline(always)]
    pub fn rstn_u3_ssp_spi_rstn_apb(&self) -> RSTN_U3_SSP_SPI_RSTN_APB_R {
        RSTN_U3_SSP_SPI_RSTN_APB_R::new(((self.bits >> 8) & 1) != 0)
    }
    #[doc = "Bit 9 - 1: Assert reset, 0: De-assert reset"]
    #[inline(always)]
    pub fn rstn_u4_ssp_spi_rstn_apb(&self) -> RSTN_U4_SSP_SPI_RSTN_APB_R {
        RSTN_U4_SSP_SPI_RSTN_APB_R::new(((self.bits >> 9) & 1) != 0)
    }
    #[doc = "Bit 10 - 1: Assert reset, 0: De-assert reset"]
    #[inline(always)]
    pub fn rstn_u5_ssp_spi_rstn_apb(&self) -> RSTN_U5_SSP_SPI_RSTN_APB_R {
        RSTN_U5_SSP_SPI_RSTN_APB_R::new(((self.bits >> 10) & 1) != 0)
    }
    #[doc = "Bit 11 - 1: Assert reset, 0: De-assert reset"]
    #[inline(always)]
    pub fn rstn_u6_ssp_spi_rstn_apb(&self) -> RSTN_U6_SSP_SPI_RSTN_APB_R {
        RSTN_U6_SSP_SPI_RSTN_APB_R::new(((self.bits >> 11) & 1) != 0)
    }
    #[doc = "Bit 12 - 1: Assert reset, 0: De-assert reset"]
    #[inline(always)]
    pub fn rstn_u0_i2c_rstn_apb(&self) -> RSTN_U0_I2C_RSTN_APB_R {
        RSTN_U0_I2C_RSTN_APB_R::new(((self.bits >> 12) & 1) != 0)
    }
    #[doc = "Bit 13 - 1: Assert reset, 0: De-assert reset"]
    #[inline(always)]
    pub fn rstn_u1_i2c_rstn_apb(&self) -> RSTN_U1_I2C_RSTN_APB_R {
        RSTN_U1_I2C_RSTN_APB_R::new(((self.bits >> 13) & 1) != 0)
    }
    #[doc = "Bit 14 - 1: Assert reset, 0: De-assert reset"]
    #[inline(always)]
    pub fn rstn_u2_i2c_rstn_apb(&self) -> RSTN_U2_I2C_RSTN_APB_R {
        RSTN_U2_I2C_RSTN_APB_R::new(((self.bits >> 14) & 1) != 0)
    }
    #[doc = "Bit 15 - 1: Assert reset, 0: De-assert reset"]
    #[inline(always)]
    pub fn rstn_u3_i2c_rstn_apb(&self) -> RSTN_U3_I2C_RSTN_APB_R {
        RSTN_U3_I2C_RSTN_APB_R::new(((self.bits >> 15) & 1) != 0)
    }
    #[doc = "Bit 16 - 1: Assert reset, 0: De-assert reset"]
    #[inline(always)]
    pub fn rstn_u4_i2c_rstn_apb(&self) -> RSTN_U4_I2C_RSTN_APB_R {
        RSTN_U4_I2C_RSTN_APB_R::new(((self.bits >> 16) & 1) != 0)
    }
    #[doc = "Bit 17 - 1: Assert reset, 0: De-assert reset"]
    #[inline(always)]
    pub fn rstn_u5_i2c_rstn_apb(&self) -> RSTN_U5_I2C_RSTN_APB_R {
        RSTN_U5_I2C_RSTN_APB_R::new(((self.bits >> 17) & 1) != 0)
    }
    #[doc = "Bit 18 - 1: Assert reset, 0: De-assert reset"]
    #[inline(always)]
    pub fn rstn_u6_i2c_rstn_apb(&self) -> RSTN_U6_I2C_RSTN_APB_R {
        RSTN_U6_I2C_RSTN_APB_R::new(((self.bits >> 18) & 1) != 0)
    }
    #[doc = "Bit 19 - 1: Assert reset, 0: De-assert reset"]
    #[inline(always)]
    pub fn rstn_u0_uart_rstn_apb(&self) -> RSTN_U0_UART_RSTN_APB_R {
        RSTN_U0_UART_RSTN_APB_R::new(((self.bits >> 19) & 1) != 0)
    }
    #[doc = "Bit 20 - 1: Assert reset, 0: De-assert reset"]
    #[inline(always)]
    pub fn rstn_u0_uart_rstn_core(&self) -> RSTN_U0_UART_RSTN_CORE_R {
        RSTN_U0_UART_RSTN_CORE_R::new(((self.bits >> 20) & 1) != 0)
    }
    #[doc = "Bit 21 - 1: Assert reset, 0: De-assert reset"]
    #[inline(always)]
    pub fn rstn_u1_uart_rstn_apb(&self) -> RSTN_U1_UART_RSTN_APB_R {
        RSTN_U1_UART_RSTN_APB_R::new(((self.bits >> 21) & 1) != 0)
    }
    #[doc = "Bit 22 - 1: Assert reset, 0: De-assert reset"]
    #[inline(always)]
    pub fn rstn_u1_uart_rstn_core(&self) -> RSTN_U1_UART_RSTN_CORE_R {
        RSTN_U1_UART_RSTN_CORE_R::new(((self.bits >> 22) & 1) != 0)
    }
    #[doc = "Bit 23 - 1: Assert reset, 0: De-assert reset"]
    #[inline(always)]
    pub fn rstn_u2_uart_rstn_apb(&self) -> RSTN_U2_UART_RSTN_APB_R {
        RSTN_U2_UART_RSTN_APB_R::new(((self.bits >> 23) & 1) != 0)
    }
    #[doc = "Bit 24 - 1: Assert reset, 0: De-assert reset"]
    #[inline(always)]
    pub fn rstn_u2_uart_rstn_core(&self) -> RSTN_U2_UART_RSTN_CORE_R {
        RSTN_U2_UART_RSTN_CORE_R::new(((self.bits >> 24) & 1) != 0)
    }
    #[doc = "Bit 25 - 1: Assert reset, 0: De-assert reset"]
    #[inline(always)]
    pub fn rstn_u3_uart_rstn_apb(&self) -> RSTN_U3_UART_RSTN_APB_R {
        RSTN_U3_UART_RSTN_APB_R::new(((self.bits >> 25) & 1) != 0)
    }
    #[doc = "Bit 26 - 1: Assert reset, 0: De-assert reset"]
    #[inline(always)]
    pub fn rstn_u3_uart_rstn_core(&self) -> RSTN_U3_UART_RSTN_CORE_R {
        RSTN_U3_UART_RSTN_CORE_R::new(((self.bits >> 26) & 1) != 0)
    }
    #[doc = "Bit 27 - 1: Assert reset, 0: De-assert reset"]
    #[inline(always)]
    pub fn rstn_u4_uart_rstn_apb(&self) -> RSTN_U4_UART_RSTN_APB_R {
        RSTN_U4_UART_RSTN_APB_R::new(((self.bits >> 27) & 1) != 0)
    }
    #[doc = "Bit 28 - 1: Assert reset, 0: De-assert reset"]
    #[inline(always)]
    pub fn rstn_u4_uart_rstn_core(&self) -> RSTN_U4_UART_RSTN_CORE_R {
        RSTN_U4_UART_RSTN_CORE_R::new(((self.bits >> 28) & 1) != 0)
    }
    #[doc = "Bit 29 - 1: Assert reset, 0: De-assert reset"]
    #[inline(always)]
    pub fn rstn_u5_uart_rstn_apb(&self) -> RSTN_U5_UART_RSTN_APB_R {
        RSTN_U5_UART_RSTN_APB_R::new(((self.bits >> 29) & 1) != 0)
    }
    #[doc = "Bit 30 - 1: Assert reset, 0: De-assert reset"]
    #[inline(always)]
    pub fn rstn_u6_uart_rstn_core(&self) -> RSTN_U6_UART_RSTN_CORE_R {
        RSTN_U6_UART_RSTN_CORE_R::new(((self.bits >> 30) & 1) != 0)
    }
    #[doc = "Bit 31 - 1: Assert reset, 0: De-assert reset"]
    #[inline(always)]
    pub fn rstn_u0_cdns_spdif_rstn_apb(&self) -> RSTN_U0_CDNS_SPDIF_RSTN_APB_R {
        RSTN_U0_CDNS_SPDIF_RSTN_APB_R::new(((self.bits >> 31) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - 1: Assert reset, 0: De-assert reset"]
    #[inline(always)]
    #[must_use]
    pub fn rstn_u0_sdio_rstn_ahb(&mut self) -> RSTN_U0_SDIO_RSTN_AHB_W<SYSCRG_RST2_STATUS_SPEC, 0> {
        RSTN_U0_SDIO_RSTN_AHB_W::new(self)
    }
    #[doc = "Bit 1 - 1: Assert reset, 0: De-assert reset"]
    #[inline(always)]
    #[must_use]
    pub fn rstn_u1_sdi_rstn_ahb(&mut self) -> RSTN_U1_SDI_RSTN_AHB_W<SYSCRG_RST2_STATUS_SPEC, 1> {
        RSTN_U1_SDI_RSTN_AHB_W::new(self)
    }
    #[doc = "Bit 2 - 1: Assert reset, 0: De-assert reset"]
    #[inline(always)]
    #[must_use]
    pub fn rstn_u1_gmac5_axi64_aresetn_i(
        &mut self,
    ) -> RSTN_U1_GMAC5_AXI64_ARESETN_I_W<SYSCRG_RST2_STATUS_SPEC, 2> {
        RSTN_U1_GMAC5_AXI64_ARESETN_I_W::new(self)
    }
    #[doc = "Bit 3 - 1: Assert reset, 0: De-assert reset"]
    #[inline(always)]
    #[must_use]
    pub fn rstn_u1_gmac5_axi64_hresetn_n(
        &mut self,
    ) -> RSTN_U1_GMAC5_AXI64_HRESETN_N_W<SYSCRG_RST2_STATUS_SPEC, 3> {
        RSTN_U1_GMAC5_AXI64_HRESETN_N_W::new(self)
    }
    #[doc = "Bit 4 - 1: Assert reset, 0: De-assert reset"]
    #[inline(always)]
    #[must_use]
    pub fn rstn_u0_mailbox_presetn(
        &mut self,
    ) -> RSTN_U0_MAILBOX_PRESETN_W<SYSCRG_RST2_STATUS_SPEC, 4> {
        RSTN_U0_MAILBOX_PRESETN_W::new(self)
    }
    #[doc = "Bit 5 - 1: Assert reset, 0: De-assert reset"]
    #[inline(always)]
    #[must_use]
    pub fn rstn_u0_ssp_spi_rstn_apb(
        &mut self,
    ) -> RSTN_U0_SSP_SPI_RSTN_APB_W<SYSCRG_RST2_STATUS_SPEC, 5> {
        RSTN_U0_SSP_SPI_RSTN_APB_W::new(self)
    }
    #[doc = "Bit 6 - 1: Assert reset, 0: De-assert reset"]
    #[inline(always)]
    #[must_use]
    pub fn rstn_u1_ssp_spi_rstn_apb(
        &mut self,
    ) -> RSTN_U1_SSP_SPI_RSTN_APB_W<SYSCRG_RST2_STATUS_SPEC, 6> {
        RSTN_U1_SSP_SPI_RSTN_APB_W::new(self)
    }
    #[doc = "Bit 7 - 1: Assert reset, 0: De-assert reset"]
    #[inline(always)]
    #[must_use]
    pub fn rstn_u2_ssp_spi_rstn_apb(
        &mut self,
    ) -> RSTN_U2_SSP_SPI_RSTN_APB_W<SYSCRG_RST2_STATUS_SPEC, 7> {
        RSTN_U2_SSP_SPI_RSTN_APB_W::new(self)
    }
    #[doc = "Bit 8 - 1: Assert reset, 0: De-assert reset"]
    #[inline(always)]
    #[must_use]
    pub fn rstn_u3_ssp_spi_rstn_apb(
        &mut self,
    ) -> RSTN_U3_SSP_SPI_RSTN_APB_W<SYSCRG_RST2_STATUS_SPEC, 8> {
        RSTN_U3_SSP_SPI_RSTN_APB_W::new(self)
    }
    #[doc = "Bit 9 - 1: Assert reset, 0: De-assert reset"]
    #[inline(always)]
    #[must_use]
    pub fn rstn_u4_ssp_spi_rstn_apb(
        &mut self,
    ) -> RSTN_U4_SSP_SPI_RSTN_APB_W<SYSCRG_RST2_STATUS_SPEC, 9> {
        RSTN_U4_SSP_SPI_RSTN_APB_W::new(self)
    }
    #[doc = "Bit 10 - 1: Assert reset, 0: De-assert reset"]
    #[inline(always)]
    #[must_use]
    pub fn rstn_u5_ssp_spi_rstn_apb(
        &mut self,
    ) -> RSTN_U5_SSP_SPI_RSTN_APB_W<SYSCRG_RST2_STATUS_SPEC, 10> {
        RSTN_U5_SSP_SPI_RSTN_APB_W::new(self)
    }
    #[doc = "Bit 11 - 1: Assert reset, 0: De-assert reset"]
    #[inline(always)]
    #[must_use]
    pub fn rstn_u6_ssp_spi_rstn_apb(
        &mut self,
    ) -> RSTN_U6_SSP_SPI_RSTN_APB_W<SYSCRG_RST2_STATUS_SPEC, 11> {
        RSTN_U6_SSP_SPI_RSTN_APB_W::new(self)
    }
    #[doc = "Bit 12 - 1: Assert reset, 0: De-assert reset"]
    #[inline(always)]
    #[must_use]
    pub fn rstn_u0_i2c_rstn_apb(&mut self) -> RSTN_U0_I2C_RSTN_APB_W<SYSCRG_RST2_STATUS_SPEC, 12> {
        RSTN_U0_I2C_RSTN_APB_W::new(self)
    }
    #[doc = "Bit 13 - 1: Assert reset, 0: De-assert reset"]
    #[inline(always)]
    #[must_use]
    pub fn rstn_u1_i2c_rstn_apb(&mut self) -> RSTN_U1_I2C_RSTN_APB_W<SYSCRG_RST2_STATUS_SPEC, 13> {
        RSTN_U1_I2C_RSTN_APB_W::new(self)
    }
    #[doc = "Bit 14 - 1: Assert reset, 0: De-assert reset"]
    #[inline(always)]
    #[must_use]
    pub fn rstn_u2_i2c_rstn_apb(&mut self) -> RSTN_U2_I2C_RSTN_APB_W<SYSCRG_RST2_STATUS_SPEC, 14> {
        RSTN_U2_I2C_RSTN_APB_W::new(self)
    }
    #[doc = "Bit 15 - 1: Assert reset, 0: De-assert reset"]
    #[inline(always)]
    #[must_use]
    pub fn rstn_u3_i2c_rstn_apb(&mut self) -> RSTN_U3_I2C_RSTN_APB_W<SYSCRG_RST2_STATUS_SPEC, 15> {
        RSTN_U3_I2C_RSTN_APB_W::new(self)
    }
    #[doc = "Bit 16 - 1: Assert reset, 0: De-assert reset"]
    #[inline(always)]
    #[must_use]
    pub fn rstn_u4_i2c_rstn_apb(&mut self) -> RSTN_U4_I2C_RSTN_APB_W<SYSCRG_RST2_STATUS_SPEC, 16> {
        RSTN_U4_I2C_RSTN_APB_W::new(self)
    }
    #[doc = "Bit 17 - 1: Assert reset, 0: De-assert reset"]
    #[inline(always)]
    #[must_use]
    pub fn rstn_u5_i2c_rstn_apb(&mut self) -> RSTN_U5_I2C_RSTN_APB_W<SYSCRG_RST2_STATUS_SPEC, 17> {
        RSTN_U5_I2C_RSTN_APB_W::new(self)
    }
    #[doc = "Bit 18 - 1: Assert reset, 0: De-assert reset"]
    #[inline(always)]
    #[must_use]
    pub fn rstn_u6_i2c_rstn_apb(&mut self) -> RSTN_U6_I2C_RSTN_APB_W<SYSCRG_RST2_STATUS_SPEC, 18> {
        RSTN_U6_I2C_RSTN_APB_W::new(self)
    }
    #[doc = "Bit 19 - 1: Assert reset, 0: De-assert reset"]
    #[inline(always)]
    #[must_use]
    pub fn rstn_u0_uart_rstn_apb(
        &mut self,
    ) -> RSTN_U0_UART_RSTN_APB_W<SYSCRG_RST2_STATUS_SPEC, 19> {
        RSTN_U0_UART_RSTN_APB_W::new(self)
    }
    #[doc = "Bit 20 - 1: Assert reset, 0: De-assert reset"]
    #[inline(always)]
    #[must_use]
    pub fn rstn_u0_uart_rstn_core(
        &mut self,
    ) -> RSTN_U0_UART_RSTN_CORE_W<SYSCRG_RST2_STATUS_SPEC, 20> {
        RSTN_U0_UART_RSTN_CORE_W::new(self)
    }
    #[doc = "Bit 21 - 1: Assert reset, 0: De-assert reset"]
    #[inline(always)]
    #[must_use]
    pub fn rstn_u1_uart_rstn_apb(
        &mut self,
    ) -> RSTN_U1_UART_RSTN_APB_W<SYSCRG_RST2_STATUS_SPEC, 21> {
        RSTN_U1_UART_RSTN_APB_W::new(self)
    }
    #[doc = "Bit 22 - 1: Assert reset, 0: De-assert reset"]
    #[inline(always)]
    #[must_use]
    pub fn rstn_u1_uart_rstn_core(
        &mut self,
    ) -> RSTN_U1_UART_RSTN_CORE_W<SYSCRG_RST2_STATUS_SPEC, 22> {
        RSTN_U1_UART_RSTN_CORE_W::new(self)
    }
    #[doc = "Bit 23 - 1: Assert reset, 0: De-assert reset"]
    #[inline(always)]
    #[must_use]
    pub fn rstn_u2_uart_rstn_apb(
        &mut self,
    ) -> RSTN_U2_UART_RSTN_APB_W<SYSCRG_RST2_STATUS_SPEC, 23> {
        RSTN_U2_UART_RSTN_APB_W::new(self)
    }
    #[doc = "Bit 24 - 1: Assert reset, 0: De-assert reset"]
    #[inline(always)]
    #[must_use]
    pub fn rstn_u2_uart_rstn_core(
        &mut self,
    ) -> RSTN_U2_UART_RSTN_CORE_W<SYSCRG_RST2_STATUS_SPEC, 24> {
        RSTN_U2_UART_RSTN_CORE_W::new(self)
    }
    #[doc = "Bit 25 - 1: Assert reset, 0: De-assert reset"]
    #[inline(always)]
    #[must_use]
    pub fn rstn_u3_uart_rstn_apb(
        &mut self,
    ) -> RSTN_U3_UART_RSTN_APB_W<SYSCRG_RST2_STATUS_SPEC, 25> {
        RSTN_U3_UART_RSTN_APB_W::new(self)
    }
    #[doc = "Bit 26 - 1: Assert reset, 0: De-assert reset"]
    #[inline(always)]
    #[must_use]
    pub fn rstn_u3_uart_rstn_core(
        &mut self,
    ) -> RSTN_U3_UART_RSTN_CORE_W<SYSCRG_RST2_STATUS_SPEC, 26> {
        RSTN_U3_UART_RSTN_CORE_W::new(self)
    }
    #[doc = "Bit 27 - 1: Assert reset, 0: De-assert reset"]
    #[inline(always)]
    #[must_use]
    pub fn rstn_u4_uart_rstn_apb(
        &mut self,
    ) -> RSTN_U4_UART_RSTN_APB_W<SYSCRG_RST2_STATUS_SPEC, 27> {
        RSTN_U4_UART_RSTN_APB_W::new(self)
    }
    #[doc = "Bit 28 - 1: Assert reset, 0: De-assert reset"]
    #[inline(always)]
    #[must_use]
    pub fn rstn_u4_uart_rstn_core(
        &mut self,
    ) -> RSTN_U4_UART_RSTN_CORE_W<SYSCRG_RST2_STATUS_SPEC, 28> {
        RSTN_U4_UART_RSTN_CORE_W::new(self)
    }
    #[doc = "Bit 29 - 1: Assert reset, 0: De-assert reset"]
    #[inline(always)]
    #[must_use]
    pub fn rstn_u5_uart_rstn_apb(
        &mut self,
    ) -> RSTN_U5_UART_RSTN_APB_W<SYSCRG_RST2_STATUS_SPEC, 29> {
        RSTN_U5_UART_RSTN_APB_W::new(self)
    }
    #[doc = "Bit 30 - 1: Assert reset, 0: De-assert reset"]
    #[inline(always)]
    #[must_use]
    pub fn rstn_u6_uart_rstn_core(
        &mut self,
    ) -> RSTN_U6_UART_RSTN_CORE_W<SYSCRG_RST2_STATUS_SPEC, 30> {
        RSTN_U6_UART_RSTN_CORE_W::new(self)
    }
    #[doc = "Bit 31 - 1: Assert reset, 0: De-assert reset"]
    #[inline(always)]
    #[must_use]
    pub fn rstn_u0_cdns_spdif_rstn_apb(
        &mut self,
    ) -> RSTN_U0_CDNS_SPDIF_RSTN_APB_W<SYSCRG_RST2_STATUS_SPEC, 31> {
        RSTN_U0_CDNS_SPDIF_RSTN_APB_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.bits = bits;
        self
    }
}
#[doc = "SYSCRG RESET Status 2\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`syscrg_rst2_status::R`](R).  You can [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`syscrg_rst2_status::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct SYSCRG_RST2_STATUS_SPEC;
impl crate::RegisterSpec for SYSCRG_RST2_STATUS_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`syscrg_rst2_status::R`](R) reader structure"]
impl crate::Readable for SYSCRG_RST2_STATUS_SPEC {}
#[doc = "`write(|w| ..)` method takes [`syscrg_rst2_status::W`](W) writer structure"]
impl crate::Writable for SYSCRG_RST2_STATUS_SPEC {
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
