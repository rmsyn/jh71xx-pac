#[doc = "Register `syscrg_rst_status_2` reader"]
pub type R = crate::R<SYSCRG_RST_STATUS_2_SPEC>;
#[doc = "Register `syscrg_rst_status_2` writer"]
pub type W = crate::W<SYSCRG_RST_STATUS_2_SPEC>;
#[doc = "Field `u0_sdio_ahb` reader - 1: Assert reset, 0: De-assert reset"]
pub type U0_SDIO_AHB_R = crate::BitReader;
#[doc = "Field `u0_sdio_ahb` writer - 1: Assert reset, 0: De-assert reset"]
pub type U0_SDIO_AHB_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `u1_sdi_ahb` reader - 1: Assert reset, 0: De-assert reset"]
pub type U1_SDI_AHB_R = crate::BitReader;
#[doc = "Field `u1_sdi_ahb` writer - 1: Assert reset, 0: De-assert reset"]
pub type U1_SDI_AHB_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `u1_gmac5_axi64` reader - 1: Assert reset, 0: De-assert reset"]
pub type U1_GMAC5_AXI64_R = crate::BitReader;
#[doc = "Field `u1_gmac5_axi64` writer - 1: Assert reset, 0: De-assert reset"]
pub type U1_GMAC5_AXI64_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `u1_gmac5_axi64_hresetn` reader - 1: Assert reset, 0: De-assert reset"]
pub type U1_GMAC5_AXI64_HRESETN_R = crate::BitReader;
#[doc = "Field `u1_gmac5_axi64_hresetn` writer - 1: Assert reset, 0: De-assert reset"]
pub type U1_GMAC5_AXI64_HRESETN_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `u0_mailbox_presetn` reader - 1: Assert reset, 0: De-assert reset"]
pub type U0_MAILBOX_PRESETN_R = crate::BitReader;
#[doc = "Field `u0_mailbox_presetn` writer - 1: Assert reset, 0: De-assert reset"]
pub type U0_MAILBOX_PRESETN_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `u0_spi_apb` reader - 1: Assert reset, 0: De-assert reset"]
pub type U0_SPI_APB_R = crate::BitReader;
#[doc = "Field `u0_spi_apb` writer - 1: Assert reset, 0: De-assert reset"]
pub type U0_SPI_APB_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `u1_spi_apb` reader - 1: Assert reset, 0: De-assert reset"]
pub type U1_SPI_APB_R = crate::BitReader;
#[doc = "Field `u1_spi_apb` writer - 1: Assert reset, 0: De-assert reset"]
pub type U1_SPI_APB_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `u2_spi_apb` reader - 1: Assert reset, 0: De-assert reset"]
pub type U2_SPI_APB_R = crate::BitReader;
#[doc = "Field `u2_spi_apb` writer - 1: Assert reset, 0: De-assert reset"]
pub type U2_SPI_APB_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `u3_spi_apb` reader - 1: Assert reset, 0: De-assert reset"]
pub type U3_SPI_APB_R = crate::BitReader;
#[doc = "Field `u3_spi_apb` writer - 1: Assert reset, 0: De-assert reset"]
pub type U3_SPI_APB_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `u4_spi_apb` reader - 1: Assert reset, 0: De-assert reset"]
pub type U4_SPI_APB_R = crate::BitReader;
#[doc = "Field `u4_spi_apb` writer - 1: Assert reset, 0: De-assert reset"]
pub type U4_SPI_APB_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `u5_spi_apb` reader - 1: Assert reset, 0: De-assert reset"]
pub type U5_SPI_APB_R = crate::BitReader;
#[doc = "Field `u5_spi_apb` writer - 1: Assert reset, 0: De-assert reset"]
pub type U5_SPI_APB_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `u6_spi_apb` reader - 1: Assert reset, 0: De-assert reset"]
pub type U6_SPI_APB_R = crate::BitReader;
#[doc = "Field `u6_spi_apb` writer - 1: Assert reset, 0: De-assert reset"]
pub type U6_SPI_APB_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `u0_i2c_apb` reader - 1: Assert reset, 0: De-assert reset"]
pub type U0_I2C_APB_R = crate::BitReader;
#[doc = "Field `u0_i2c_apb` writer - 1: Assert reset, 0: De-assert reset"]
pub type U0_I2C_APB_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `u1_i2c_apb` reader - 1: Assert reset, 0: De-assert reset"]
pub type U1_I2C_APB_R = crate::BitReader;
#[doc = "Field `u1_i2c_apb` writer - 1: Assert reset, 0: De-assert reset"]
pub type U1_I2C_APB_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `u2_i2c_apb` reader - 1: Assert reset, 0: De-assert reset"]
pub type U2_I2C_APB_R = crate::BitReader;
#[doc = "Field `u2_i2c_apb` writer - 1: Assert reset, 0: De-assert reset"]
pub type U2_I2C_APB_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `u3_i2c_apb` reader - 1: Assert reset, 0: De-assert reset"]
pub type U3_I2C_APB_R = crate::BitReader;
#[doc = "Field `u3_i2c_apb` writer - 1: Assert reset, 0: De-assert reset"]
pub type U3_I2C_APB_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `u4_i2c_apb` reader - 1: Assert reset, 0: De-assert reset"]
pub type U4_I2C_APB_R = crate::BitReader;
#[doc = "Field `u4_i2c_apb` writer - 1: Assert reset, 0: De-assert reset"]
pub type U4_I2C_APB_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `u5_i2c_apb` reader - 1: Assert reset, 0: De-assert reset"]
pub type U5_I2C_APB_R = crate::BitReader;
#[doc = "Field `u5_i2c_apb` writer - 1: Assert reset, 0: De-assert reset"]
pub type U5_I2C_APB_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `u6_i2c_apb` reader - 1: Assert reset, 0: De-assert reset"]
pub type U6_I2C_APB_R = crate::BitReader;
#[doc = "Field `u6_i2c_apb` writer - 1: Assert reset, 0: De-assert reset"]
pub type U6_I2C_APB_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `u0_uart_apb` reader - 1: Assert reset, 0: De-assert reset"]
pub type U0_UART_APB_R = crate::BitReader;
#[doc = "Field `u0_uart_apb` writer - 1: Assert reset, 0: De-assert reset"]
pub type U0_UART_APB_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `u0_uart_core` reader - 1: Assert reset, 0: De-assert reset"]
pub type U0_UART_CORE_R = crate::BitReader;
#[doc = "Field `u0_uart_core` writer - 1: Assert reset, 0: De-assert reset"]
pub type U0_UART_CORE_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `u1_uart_apb` reader - 1: Assert reset, 0: De-assert reset"]
pub type U1_UART_APB_R = crate::BitReader;
#[doc = "Field `u1_uart_apb` writer - 1: Assert reset, 0: De-assert reset"]
pub type U1_UART_APB_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `u1_uart_core` reader - 1: Assert reset, 0: De-assert reset"]
pub type U1_UART_CORE_R = crate::BitReader;
#[doc = "Field `u1_uart_core` writer - 1: Assert reset, 0: De-assert reset"]
pub type U1_UART_CORE_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `u2_uart_apb` reader - 1: Assert reset, 0: De-assert reset"]
pub type U2_UART_APB_R = crate::BitReader;
#[doc = "Field `u2_uart_apb` writer - 1: Assert reset, 0: De-assert reset"]
pub type U2_UART_APB_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `u2_uart_core` reader - 1: Assert reset, 0: De-assert reset"]
pub type U2_UART_CORE_R = crate::BitReader;
#[doc = "Field `u2_uart_core` writer - 1: Assert reset, 0: De-assert reset"]
pub type U2_UART_CORE_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `u3_uart_apb` reader - 1: Assert reset, 0: De-assert reset"]
pub type U3_UART_APB_R = crate::BitReader;
#[doc = "Field `u3_uart_apb` writer - 1: Assert reset, 0: De-assert reset"]
pub type U3_UART_APB_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `u3_uart_core` reader - 1: Assert reset, 0: De-assert reset"]
pub type U3_UART_CORE_R = crate::BitReader;
#[doc = "Field `u3_uart_core` writer - 1: Assert reset, 0: De-assert reset"]
pub type U3_UART_CORE_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `u4_uart_apb` reader - 1: Assert reset, 0: De-assert reset"]
pub type U4_UART_APB_R = crate::BitReader;
#[doc = "Field `u4_uart_apb` writer - 1: Assert reset, 0: De-assert reset"]
pub type U4_UART_APB_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `u4_uart_core` reader - 1: Assert reset, 0: De-assert reset"]
pub type U4_UART_CORE_R = crate::BitReader;
#[doc = "Field `u4_uart_core` writer - 1: Assert reset, 0: De-assert reset"]
pub type U4_UART_CORE_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `u5_uart_apb` reader - 1: Assert reset, 0: De-assert reset"]
pub type U5_UART_APB_R = crate::BitReader;
#[doc = "Field `u5_uart_apb` writer - 1: Assert reset, 0: De-assert reset"]
pub type U5_UART_APB_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `u6_uart_core` reader - 1: Assert reset, 0: De-assert reset"]
pub type U6_UART_CORE_R = crate::BitReader;
#[doc = "Field `u6_uart_core` writer - 1: Assert reset, 0: De-assert reset"]
pub type U6_UART_CORE_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `u0_spdif_apb` reader - 1: Assert reset, 0: De-assert reset"]
pub type U0_SPDIF_APB_R = crate::BitReader;
#[doc = "Field `u0_spdif_apb` writer - 1: Assert reset, 0: De-assert reset"]
pub type U0_SPDIF_APB_W<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bit 0 - 1: Assert reset, 0: De-assert reset"]
    #[inline(always)]
    pub fn u0_sdio_ahb(&self) -> U0_SDIO_AHB_R {
        U0_SDIO_AHB_R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - 1: Assert reset, 0: De-assert reset"]
    #[inline(always)]
    pub fn u1_sdi_ahb(&self) -> U1_SDI_AHB_R {
        U1_SDI_AHB_R::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - 1: Assert reset, 0: De-assert reset"]
    #[inline(always)]
    pub fn u1_gmac5_axi64(&self) -> U1_GMAC5_AXI64_R {
        U1_GMAC5_AXI64_R::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3 - 1: Assert reset, 0: De-assert reset"]
    #[inline(always)]
    pub fn u1_gmac5_axi64_hresetn(&self) -> U1_GMAC5_AXI64_HRESETN_R {
        U1_GMAC5_AXI64_HRESETN_R::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bit 4 - 1: Assert reset, 0: De-assert reset"]
    #[inline(always)]
    pub fn u0_mailbox_presetn(&self) -> U0_MAILBOX_PRESETN_R {
        U0_MAILBOX_PRESETN_R::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bit 5 - 1: Assert reset, 0: De-assert reset"]
    #[inline(always)]
    pub fn u0_spi_apb(&self) -> U0_SPI_APB_R {
        U0_SPI_APB_R::new(((self.bits >> 5) & 1) != 0)
    }
    #[doc = "Bit 6 - 1: Assert reset, 0: De-assert reset"]
    #[inline(always)]
    pub fn u1_spi_apb(&self) -> U1_SPI_APB_R {
        U1_SPI_APB_R::new(((self.bits >> 6) & 1) != 0)
    }
    #[doc = "Bit 7 - 1: Assert reset, 0: De-assert reset"]
    #[inline(always)]
    pub fn u2_spi_apb(&self) -> U2_SPI_APB_R {
        U2_SPI_APB_R::new(((self.bits >> 7) & 1) != 0)
    }
    #[doc = "Bit 8 - 1: Assert reset, 0: De-assert reset"]
    #[inline(always)]
    pub fn u3_spi_apb(&self) -> U3_SPI_APB_R {
        U3_SPI_APB_R::new(((self.bits >> 8) & 1) != 0)
    }
    #[doc = "Bit 9 - 1: Assert reset, 0: De-assert reset"]
    #[inline(always)]
    pub fn u4_spi_apb(&self) -> U4_SPI_APB_R {
        U4_SPI_APB_R::new(((self.bits >> 9) & 1) != 0)
    }
    #[doc = "Bit 10 - 1: Assert reset, 0: De-assert reset"]
    #[inline(always)]
    pub fn u5_spi_apb(&self) -> U5_SPI_APB_R {
        U5_SPI_APB_R::new(((self.bits >> 10) & 1) != 0)
    }
    #[doc = "Bit 11 - 1: Assert reset, 0: De-assert reset"]
    #[inline(always)]
    pub fn u6_spi_apb(&self) -> U6_SPI_APB_R {
        U6_SPI_APB_R::new(((self.bits >> 11) & 1) != 0)
    }
    #[doc = "Bit 12 - 1: Assert reset, 0: De-assert reset"]
    #[inline(always)]
    pub fn u0_i2c_apb(&self) -> U0_I2C_APB_R {
        U0_I2C_APB_R::new(((self.bits >> 12) & 1) != 0)
    }
    #[doc = "Bit 13 - 1: Assert reset, 0: De-assert reset"]
    #[inline(always)]
    pub fn u1_i2c_apb(&self) -> U1_I2C_APB_R {
        U1_I2C_APB_R::new(((self.bits >> 13) & 1) != 0)
    }
    #[doc = "Bit 14 - 1: Assert reset, 0: De-assert reset"]
    #[inline(always)]
    pub fn u2_i2c_apb(&self) -> U2_I2C_APB_R {
        U2_I2C_APB_R::new(((self.bits >> 14) & 1) != 0)
    }
    #[doc = "Bit 15 - 1: Assert reset, 0: De-assert reset"]
    #[inline(always)]
    pub fn u3_i2c_apb(&self) -> U3_I2C_APB_R {
        U3_I2C_APB_R::new(((self.bits >> 15) & 1) != 0)
    }
    #[doc = "Bit 16 - 1: Assert reset, 0: De-assert reset"]
    #[inline(always)]
    pub fn u4_i2c_apb(&self) -> U4_I2C_APB_R {
        U4_I2C_APB_R::new(((self.bits >> 16) & 1) != 0)
    }
    #[doc = "Bit 17 - 1: Assert reset, 0: De-assert reset"]
    #[inline(always)]
    pub fn u5_i2c_apb(&self) -> U5_I2C_APB_R {
        U5_I2C_APB_R::new(((self.bits >> 17) & 1) != 0)
    }
    #[doc = "Bit 18 - 1: Assert reset, 0: De-assert reset"]
    #[inline(always)]
    pub fn u6_i2c_apb(&self) -> U6_I2C_APB_R {
        U6_I2C_APB_R::new(((self.bits >> 18) & 1) != 0)
    }
    #[doc = "Bit 19 - 1: Assert reset, 0: De-assert reset"]
    #[inline(always)]
    pub fn u0_uart_apb(&self) -> U0_UART_APB_R {
        U0_UART_APB_R::new(((self.bits >> 19) & 1) != 0)
    }
    #[doc = "Bit 20 - 1: Assert reset, 0: De-assert reset"]
    #[inline(always)]
    pub fn u0_uart_core(&self) -> U0_UART_CORE_R {
        U0_UART_CORE_R::new(((self.bits >> 20) & 1) != 0)
    }
    #[doc = "Bit 21 - 1: Assert reset, 0: De-assert reset"]
    #[inline(always)]
    pub fn u1_uart_apb(&self) -> U1_UART_APB_R {
        U1_UART_APB_R::new(((self.bits >> 21) & 1) != 0)
    }
    #[doc = "Bit 22 - 1: Assert reset, 0: De-assert reset"]
    #[inline(always)]
    pub fn u1_uart_core(&self) -> U1_UART_CORE_R {
        U1_UART_CORE_R::new(((self.bits >> 22) & 1) != 0)
    }
    #[doc = "Bit 23 - 1: Assert reset, 0: De-assert reset"]
    #[inline(always)]
    pub fn u2_uart_apb(&self) -> U2_UART_APB_R {
        U2_UART_APB_R::new(((self.bits >> 23) & 1) != 0)
    }
    #[doc = "Bit 24 - 1: Assert reset, 0: De-assert reset"]
    #[inline(always)]
    pub fn u2_uart_core(&self) -> U2_UART_CORE_R {
        U2_UART_CORE_R::new(((self.bits >> 24) & 1) != 0)
    }
    #[doc = "Bit 25 - 1: Assert reset, 0: De-assert reset"]
    #[inline(always)]
    pub fn u3_uart_apb(&self) -> U3_UART_APB_R {
        U3_UART_APB_R::new(((self.bits >> 25) & 1) != 0)
    }
    #[doc = "Bit 26 - 1: Assert reset, 0: De-assert reset"]
    #[inline(always)]
    pub fn u3_uart_core(&self) -> U3_UART_CORE_R {
        U3_UART_CORE_R::new(((self.bits >> 26) & 1) != 0)
    }
    #[doc = "Bit 27 - 1: Assert reset, 0: De-assert reset"]
    #[inline(always)]
    pub fn u4_uart_apb(&self) -> U4_UART_APB_R {
        U4_UART_APB_R::new(((self.bits >> 27) & 1) != 0)
    }
    #[doc = "Bit 28 - 1: Assert reset, 0: De-assert reset"]
    #[inline(always)]
    pub fn u4_uart_core(&self) -> U4_UART_CORE_R {
        U4_UART_CORE_R::new(((self.bits >> 28) & 1) != 0)
    }
    #[doc = "Bit 29 - 1: Assert reset, 0: De-assert reset"]
    #[inline(always)]
    pub fn u5_uart_apb(&self) -> U5_UART_APB_R {
        U5_UART_APB_R::new(((self.bits >> 29) & 1) != 0)
    }
    #[doc = "Bit 30 - 1: Assert reset, 0: De-assert reset"]
    #[inline(always)]
    pub fn u6_uart_core(&self) -> U6_UART_CORE_R {
        U6_UART_CORE_R::new(((self.bits >> 30) & 1) != 0)
    }
    #[doc = "Bit 31 - 1: Assert reset, 0: De-assert reset"]
    #[inline(always)]
    pub fn u0_spdif_apb(&self) -> U0_SPDIF_APB_R {
        U0_SPDIF_APB_R::new(((self.bits >> 31) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - 1: Assert reset, 0: De-assert reset"]
    #[inline(always)]
    #[must_use]
    pub fn u0_sdio_ahb(&mut self) -> U0_SDIO_AHB_W<SYSCRG_RST_STATUS_2_SPEC> {
        U0_SDIO_AHB_W::new(self, 0)
    }
    #[doc = "Bit 1 - 1: Assert reset, 0: De-assert reset"]
    #[inline(always)]
    #[must_use]
    pub fn u1_sdi_ahb(&mut self) -> U1_SDI_AHB_W<SYSCRG_RST_STATUS_2_SPEC> {
        U1_SDI_AHB_W::new(self, 1)
    }
    #[doc = "Bit 2 - 1: Assert reset, 0: De-assert reset"]
    #[inline(always)]
    #[must_use]
    pub fn u1_gmac5_axi64(&mut self) -> U1_GMAC5_AXI64_W<SYSCRG_RST_STATUS_2_SPEC> {
        U1_GMAC5_AXI64_W::new(self, 2)
    }
    #[doc = "Bit 3 - 1: Assert reset, 0: De-assert reset"]
    #[inline(always)]
    #[must_use]
    pub fn u1_gmac5_axi64_hresetn(&mut self) -> U1_GMAC5_AXI64_HRESETN_W<SYSCRG_RST_STATUS_2_SPEC> {
        U1_GMAC5_AXI64_HRESETN_W::new(self, 3)
    }
    #[doc = "Bit 4 - 1: Assert reset, 0: De-assert reset"]
    #[inline(always)]
    #[must_use]
    pub fn u0_mailbox_presetn(&mut self) -> U0_MAILBOX_PRESETN_W<SYSCRG_RST_STATUS_2_SPEC> {
        U0_MAILBOX_PRESETN_W::new(self, 4)
    }
    #[doc = "Bit 5 - 1: Assert reset, 0: De-assert reset"]
    #[inline(always)]
    #[must_use]
    pub fn u0_spi_apb(&mut self) -> U0_SPI_APB_W<SYSCRG_RST_STATUS_2_SPEC> {
        U0_SPI_APB_W::new(self, 5)
    }
    #[doc = "Bit 6 - 1: Assert reset, 0: De-assert reset"]
    #[inline(always)]
    #[must_use]
    pub fn u1_spi_apb(&mut self) -> U1_SPI_APB_W<SYSCRG_RST_STATUS_2_SPEC> {
        U1_SPI_APB_W::new(self, 6)
    }
    #[doc = "Bit 7 - 1: Assert reset, 0: De-assert reset"]
    #[inline(always)]
    #[must_use]
    pub fn u2_spi_apb(&mut self) -> U2_SPI_APB_W<SYSCRG_RST_STATUS_2_SPEC> {
        U2_SPI_APB_W::new(self, 7)
    }
    #[doc = "Bit 8 - 1: Assert reset, 0: De-assert reset"]
    #[inline(always)]
    #[must_use]
    pub fn u3_spi_apb(&mut self) -> U3_SPI_APB_W<SYSCRG_RST_STATUS_2_SPEC> {
        U3_SPI_APB_W::new(self, 8)
    }
    #[doc = "Bit 9 - 1: Assert reset, 0: De-assert reset"]
    #[inline(always)]
    #[must_use]
    pub fn u4_spi_apb(&mut self) -> U4_SPI_APB_W<SYSCRG_RST_STATUS_2_SPEC> {
        U4_SPI_APB_W::new(self, 9)
    }
    #[doc = "Bit 10 - 1: Assert reset, 0: De-assert reset"]
    #[inline(always)]
    #[must_use]
    pub fn u5_spi_apb(&mut self) -> U5_SPI_APB_W<SYSCRG_RST_STATUS_2_SPEC> {
        U5_SPI_APB_W::new(self, 10)
    }
    #[doc = "Bit 11 - 1: Assert reset, 0: De-assert reset"]
    #[inline(always)]
    #[must_use]
    pub fn u6_spi_apb(&mut self) -> U6_SPI_APB_W<SYSCRG_RST_STATUS_2_SPEC> {
        U6_SPI_APB_W::new(self, 11)
    }
    #[doc = "Bit 12 - 1: Assert reset, 0: De-assert reset"]
    #[inline(always)]
    #[must_use]
    pub fn u0_i2c_apb(&mut self) -> U0_I2C_APB_W<SYSCRG_RST_STATUS_2_SPEC> {
        U0_I2C_APB_W::new(self, 12)
    }
    #[doc = "Bit 13 - 1: Assert reset, 0: De-assert reset"]
    #[inline(always)]
    #[must_use]
    pub fn u1_i2c_apb(&mut self) -> U1_I2C_APB_W<SYSCRG_RST_STATUS_2_SPEC> {
        U1_I2C_APB_W::new(self, 13)
    }
    #[doc = "Bit 14 - 1: Assert reset, 0: De-assert reset"]
    #[inline(always)]
    #[must_use]
    pub fn u2_i2c_apb(&mut self) -> U2_I2C_APB_W<SYSCRG_RST_STATUS_2_SPEC> {
        U2_I2C_APB_W::new(self, 14)
    }
    #[doc = "Bit 15 - 1: Assert reset, 0: De-assert reset"]
    #[inline(always)]
    #[must_use]
    pub fn u3_i2c_apb(&mut self) -> U3_I2C_APB_W<SYSCRG_RST_STATUS_2_SPEC> {
        U3_I2C_APB_W::new(self, 15)
    }
    #[doc = "Bit 16 - 1: Assert reset, 0: De-assert reset"]
    #[inline(always)]
    #[must_use]
    pub fn u4_i2c_apb(&mut self) -> U4_I2C_APB_W<SYSCRG_RST_STATUS_2_SPEC> {
        U4_I2C_APB_W::new(self, 16)
    }
    #[doc = "Bit 17 - 1: Assert reset, 0: De-assert reset"]
    #[inline(always)]
    #[must_use]
    pub fn u5_i2c_apb(&mut self) -> U5_I2C_APB_W<SYSCRG_RST_STATUS_2_SPEC> {
        U5_I2C_APB_W::new(self, 17)
    }
    #[doc = "Bit 18 - 1: Assert reset, 0: De-assert reset"]
    #[inline(always)]
    #[must_use]
    pub fn u6_i2c_apb(&mut self) -> U6_I2C_APB_W<SYSCRG_RST_STATUS_2_SPEC> {
        U6_I2C_APB_W::new(self, 18)
    }
    #[doc = "Bit 19 - 1: Assert reset, 0: De-assert reset"]
    #[inline(always)]
    #[must_use]
    pub fn u0_uart_apb(&mut self) -> U0_UART_APB_W<SYSCRG_RST_STATUS_2_SPEC> {
        U0_UART_APB_W::new(self, 19)
    }
    #[doc = "Bit 20 - 1: Assert reset, 0: De-assert reset"]
    #[inline(always)]
    #[must_use]
    pub fn u0_uart_core(&mut self) -> U0_UART_CORE_W<SYSCRG_RST_STATUS_2_SPEC> {
        U0_UART_CORE_W::new(self, 20)
    }
    #[doc = "Bit 21 - 1: Assert reset, 0: De-assert reset"]
    #[inline(always)]
    #[must_use]
    pub fn u1_uart_apb(&mut self) -> U1_UART_APB_W<SYSCRG_RST_STATUS_2_SPEC> {
        U1_UART_APB_W::new(self, 21)
    }
    #[doc = "Bit 22 - 1: Assert reset, 0: De-assert reset"]
    #[inline(always)]
    #[must_use]
    pub fn u1_uart_core(&mut self) -> U1_UART_CORE_W<SYSCRG_RST_STATUS_2_SPEC> {
        U1_UART_CORE_W::new(self, 22)
    }
    #[doc = "Bit 23 - 1: Assert reset, 0: De-assert reset"]
    #[inline(always)]
    #[must_use]
    pub fn u2_uart_apb(&mut self) -> U2_UART_APB_W<SYSCRG_RST_STATUS_2_SPEC> {
        U2_UART_APB_W::new(self, 23)
    }
    #[doc = "Bit 24 - 1: Assert reset, 0: De-assert reset"]
    #[inline(always)]
    #[must_use]
    pub fn u2_uart_core(&mut self) -> U2_UART_CORE_W<SYSCRG_RST_STATUS_2_SPEC> {
        U2_UART_CORE_W::new(self, 24)
    }
    #[doc = "Bit 25 - 1: Assert reset, 0: De-assert reset"]
    #[inline(always)]
    #[must_use]
    pub fn u3_uart_apb(&mut self) -> U3_UART_APB_W<SYSCRG_RST_STATUS_2_SPEC> {
        U3_UART_APB_W::new(self, 25)
    }
    #[doc = "Bit 26 - 1: Assert reset, 0: De-assert reset"]
    #[inline(always)]
    #[must_use]
    pub fn u3_uart_core(&mut self) -> U3_UART_CORE_W<SYSCRG_RST_STATUS_2_SPEC> {
        U3_UART_CORE_W::new(self, 26)
    }
    #[doc = "Bit 27 - 1: Assert reset, 0: De-assert reset"]
    #[inline(always)]
    #[must_use]
    pub fn u4_uart_apb(&mut self) -> U4_UART_APB_W<SYSCRG_RST_STATUS_2_SPEC> {
        U4_UART_APB_W::new(self, 27)
    }
    #[doc = "Bit 28 - 1: Assert reset, 0: De-assert reset"]
    #[inline(always)]
    #[must_use]
    pub fn u4_uart_core(&mut self) -> U4_UART_CORE_W<SYSCRG_RST_STATUS_2_SPEC> {
        U4_UART_CORE_W::new(self, 28)
    }
    #[doc = "Bit 29 - 1: Assert reset, 0: De-assert reset"]
    #[inline(always)]
    #[must_use]
    pub fn u5_uart_apb(&mut self) -> U5_UART_APB_W<SYSCRG_RST_STATUS_2_SPEC> {
        U5_UART_APB_W::new(self, 29)
    }
    #[doc = "Bit 30 - 1: Assert reset, 0: De-assert reset"]
    #[inline(always)]
    #[must_use]
    pub fn u6_uart_core(&mut self) -> U6_UART_CORE_W<SYSCRG_RST_STATUS_2_SPEC> {
        U6_UART_CORE_W::new(self, 30)
    }
    #[doc = "Bit 31 - 1: Assert reset, 0: De-assert reset"]
    #[inline(always)]
    #[must_use]
    pub fn u0_spdif_apb(&mut self) -> U0_SPDIF_APB_W<SYSCRG_RST_STATUS_2_SPEC> {
        U0_SPDIF_APB_W::new(self, 31)
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
#[doc = "SYSCRG RESET Status 2\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`syscrg_rst_status_2::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`syscrg_rst_status_2::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct SYSCRG_RST_STATUS_2_SPEC;
impl crate::RegisterSpec for SYSCRG_RST_STATUS_2_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`syscrg_rst_status_2::R`](R) reader structure"]
impl crate::Readable for SYSCRG_RST_STATUS_2_SPEC {}
#[doc = "`write(|w| ..)` method takes [`syscrg_rst_status_2::W`](W) writer structure"]
impl crate::Writable for SYSCRG_RST_STATUS_2_SPEC {
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets syscrg_rst_status_2 to value 0"]
impl crate::Resettable for SYSCRG_RST_STATUS_2_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
