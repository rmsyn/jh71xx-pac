#[doc = "Register `soft_rst_addr_sel_0` reader"]
pub type R = crate::R<SOFT_RST_ADDR_SEL_0_SPEC>;
#[doc = "Register `soft_rst_addr_sel_0` writer"]
pub type W = crate::W<SOFT_RST_ADDR_SEL_0_SPEC>;
#[doc = "Field `u0_jtag2apb_presetn` reader - 1: Assert reset, 0: De-assert reset"]
pub type U0_JTAG2APB_PRESETN_R = crate::BitReader;
#[doc = "Field `u0_jtag2apb_presetn` writer - 1: Assert reset, 0: De-assert reset"]
pub type U0_JTAG2APB_PRESETN_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `u0_sys_syscon_presetn` reader - 1: Assert reset, 0: De-assert reset"]
pub type U0_SYS_SYSCON_PRESETN_R = crate::BitReader;
#[doc = "Field `u0_sys_syscon_presetn` writer - 1: Assert reset, 0: De-assert reset"]
pub type U0_SYS_SYSCON_PRESETN_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `u0_sys_iomux_presetn` reader - 1: Assert reset, 0: De-assert reset"]
pub type U0_SYS_IOMUX_PRESETN_R = crate::BitReader;
#[doc = "Field `u0_sys_iomux_presetn` writer - 1: Assert reset, 0: De-assert reset"]
pub type U0_SYS_IOMUX_PRESETN_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `u0_bus` reader - 1: Assert reset, 0: De-assert reset"]
pub type U0_BUS_R = crate::BitReader;
#[doc = "Field `u0_bus` writer - 1: Assert reset, 0: De-assert reset"]
pub type U0_BUS_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `u0_debug` reader - 1: Assert reset, 0: De-assert reset"]
pub type U0_DEBUG_R = crate::BitReader;
#[doc = "Field `u0_debug` writer - 1: Assert reset, 0: De-assert reset"]
pub type U0_DEBUG_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `u0_core_0` reader - 1: Assert reset, 0: De-assert reset"]
pub type U0_CORE_0_R = crate::BitReader;
#[doc = "Field `u0_core_0` writer - 1: Assert reset, 0: De-assert reset"]
pub type U0_CORE_0_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `u0_core_1` reader - 1: Assert reset, 0: De-assert reset"]
pub type U0_CORE_1_R = crate::BitReader;
#[doc = "Field `u0_core_1` writer - 1: Assert reset, 0: De-assert reset"]
pub type U0_CORE_1_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `u0_core_2` reader - 1: Assert reset, 0: De-assert reset"]
pub type U0_CORE_2_R = crate::BitReader;
#[doc = "Field `u0_core_2` writer - 1: Assert reset, 0: De-assert reset"]
pub type U0_CORE_2_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `u0_core3` reader - 1: Assert reset, 0: De-assert reset"]
pub type U0_CORE3_R = crate::BitReader;
#[doc = "Field `u0_core3` writer - 1: Assert reset, 0: De-assert reset"]
pub type U0_CORE3_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `u0_core4` reader - 1: Assert reset, 0: De-assert reset"]
pub type U0_CORE4_R = crate::BitReader;
#[doc = "Field `u0_core4` writer - 1: Assert reset, 0: De-assert reset"]
pub type U0_CORE4_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `u0_core_st_0` reader - 1: Assert reset, 0: De-assert reset"]
pub type U0_CORE_ST_0_R = crate::BitReader;
#[doc = "Field `u0_core_st_0` writer - 1: Assert reset, 0: De-assert reset"]
pub type U0_CORE_ST_0_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `u0_core_st_1` reader - 1: Assert reset, 0: De-assert reset"]
pub type U0_CORE_ST_1_R = crate::BitReader;
#[doc = "Field `u0_core_st_1` writer - 1: Assert reset, 0: De-assert reset"]
pub type U0_CORE_ST_1_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `u0_core_st_2` reader - 1: Assert reset, 0: De-assert reset"]
pub type U0_CORE_ST_2_R = crate::BitReader;
#[doc = "Field `u0_core_st_2` writer - 1: Assert reset, 0: De-assert reset"]
pub type U0_CORE_ST_2_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `u0_core_st_3` reader - 1: Assert reset, 0: De-assert reset"]
pub type U0_CORE_ST_3_R = crate::BitReader;
#[doc = "Field `u0_core_st_3` writer - 1: Assert reset, 0: De-assert reset"]
pub type U0_CORE_ST_3_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `u0_core_st_4` reader - 1: Assert reset, 0: De-assert reset"]
pub type U0_CORE_ST_4_R = crate::BitReader;
#[doc = "Field `u0_core_st_4` writer - 1: Assert reset, 0: De-assert reset"]
pub type U0_CORE_ST_4_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `u0_trace_0` reader - 1: Assert reset, 0: De-assert reset"]
pub type U0_TRACE_0_R = crate::BitReader;
#[doc = "Field `u0_trace_0` writer - 1: Assert reset, 0: De-assert reset"]
pub type U0_TRACE_0_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `u0_trace_1` reader - 1: Assert reset, 0: De-assert reset"]
pub type U0_TRACE_1_R = crate::BitReader;
#[doc = "Field `u0_trace_1` writer - 1: Assert reset, 0: De-assert reset"]
pub type U0_TRACE_1_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `u0_trace_2` reader - 1: Assert reset, 0: De-assert reset"]
pub type U0_TRACE_2_R = crate::BitReader;
#[doc = "Field `u0_trace_2` writer - 1: Assert reset, 0: De-assert reset"]
pub type U0_TRACE_2_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `u0_trace_3` reader - 1: Assert reset, 0: De-assert reset"]
pub type U0_TRACE_3_R = crate::BitReader;
#[doc = "Field `u0_trace_3` writer - 1: Assert reset, 0: De-assert reset"]
pub type U0_TRACE_3_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `u0_trace_4` reader - 1: Assert reset, 0: De-assert reset"]
pub type U0_TRACE_4_R = crate::BitReader;
#[doc = "Field `u0_trace_4` writer - 1: Assert reset, 0: De-assert reset"]
pub type U0_TRACE_4_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `u0_trace_com` reader - 1: Assert reset, 0: De-assert reset"]
pub type U0_TRACE_COM_R = crate::BitReader;
#[doc = "Field `u0_trace_com` writer - 1: Assert reset, 0: De-assert reset"]
pub type U0_TRACE_COM_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `u0_img_gpu_apb` reader - 1: Assert reset, 0: De-assert reset"]
pub type U0_IMG_GPU_APB_R = crate::BitReader;
#[doc = "Field `u0_img_gpu_apb` writer - 1: Assert reset, 0: De-assert reset"]
pub type U0_IMG_GPU_APB_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `u0_img_gpu_doma` reader - 1: Assert reset, 0: De-assert reset"]
pub type U0_IMG_GPU_DOMA_R = crate::BitReader;
#[doc = "Field `u0_img_gpu_doma` writer - 1: Assert reset, 0: De-assert reset"]
pub type U0_IMG_GPU_DOMA_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `u0_noc_bus_apb` reader - 1: Assert reset, 0: De-assert reset"]
pub type U0_NOC_BUS_APB_R = crate::BitReader;
#[doc = "Field `u0_noc_bus_apb` writer - 1: Assert reset, 0: De-assert reset"]
pub type U0_NOC_BUS_APB_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `u0_noc_bus_axicfg0` reader - 1: Assert reset, 0: De-assert reset"]
pub type U0_NOC_BUS_AXICFG0_R = crate::BitReader;
#[doc = "Field `u0_noc_bus_axicfg0` writer - 1: Assert reset, 0: De-assert reset"]
pub type U0_NOC_BUS_AXICFG0_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `u0_noc_bus_cpu_axi` reader - 1: Assert reset, 0: De-assert reset"]
pub type U0_NOC_BUS_CPU_AXI_R = crate::BitReader;
#[doc = "Field `u0_noc_bus_cpu_axi` writer - 1: Assert reset, 0: De-assert reset"]
pub type U0_NOC_BUS_CPU_AXI_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `u0_noc_bus_disp_axi` reader - 1: Assert reset, 0: De-assert reset"]
pub type U0_NOC_BUS_DISP_AXI_R = crate::BitReader;
#[doc = "Field `u0_noc_bus_disp_axi` writer - 1: Assert reset, 0: De-assert reset"]
pub type U0_NOC_BUS_DISP_AXI_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `u0_noc_bus_gpu_axi` reader - 1: Assert reset, 0: De-assert reset"]
pub type U0_NOC_BUS_GPU_AXI_R = crate::BitReader;
#[doc = "Field `u0_noc_bus_gpu_axi` writer - 1: Assert reset, 0: De-assert reset"]
pub type U0_NOC_BUS_GPU_AXI_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `u0_noc_bus_isp_axi` reader - 1: Assert reset, 0: De-assert reset"]
pub type U0_NOC_BUS_ISP_AXI_R = crate::BitReader;
#[doc = "Field `u0_noc_bus_isp_axi` writer - 1: Assert reset, 0: De-assert reset"]
pub type U0_NOC_BUS_ISP_AXI_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `u0_noc_bus_ddrc` reader - 1: Assert reset, 0: De-assert reset"]
pub type U0_NOC_BUS_DDRC_R = crate::BitReader;
#[doc = "Field `u0_noc_bus_ddrc` writer - 1: Assert reset, 0: De-assert reset"]
pub type U0_NOC_BUS_DDRC_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `u0_noc_bus_stg_axi` reader - 1: Assert reset, 0: De-assert reset"]
pub type U0_NOC_BUS_STG_AXI_R = crate::BitReader;
#[doc = "Field `u0_noc_bus_stg_axi` writer - 1: Assert reset, 0: De-assert reset"]
pub type U0_NOC_BUS_STG_AXI_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `u0_noc_bus_vdec_axi` reader - 1: Assert reset, 0: De-assert reset"]
pub type U0_NOC_BUS_VDEC_AXI_R = crate::BitReader;
#[doc = "Field `u0_noc_bus_vdec_axi` writer - 1: Assert reset, 0: De-assert reset"]
pub type U0_NOC_BUS_VDEC_AXI_W<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bit 0 - 1: Assert reset, 0: De-assert reset"]
    #[inline(always)]
    pub fn u0_jtag2apb_presetn(&self) -> U0_JTAG2APB_PRESETN_R {
        U0_JTAG2APB_PRESETN_R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - 1: Assert reset, 0: De-assert reset"]
    #[inline(always)]
    pub fn u0_sys_syscon_presetn(&self) -> U0_SYS_SYSCON_PRESETN_R {
        U0_SYS_SYSCON_PRESETN_R::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - 1: Assert reset, 0: De-assert reset"]
    #[inline(always)]
    pub fn u0_sys_iomux_presetn(&self) -> U0_SYS_IOMUX_PRESETN_R {
        U0_SYS_IOMUX_PRESETN_R::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3 - 1: Assert reset, 0: De-assert reset"]
    #[inline(always)]
    pub fn u0_bus(&self) -> U0_BUS_R {
        U0_BUS_R::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bit 4 - 1: Assert reset, 0: De-assert reset"]
    #[inline(always)]
    pub fn u0_debug(&self) -> U0_DEBUG_R {
        U0_DEBUG_R::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bit 5 - 1: Assert reset, 0: De-assert reset"]
    #[inline(always)]
    pub fn u0_core_0(&self) -> U0_CORE_0_R {
        U0_CORE_0_R::new(((self.bits >> 5) & 1) != 0)
    }
    #[doc = "Bit 6 - 1: Assert reset, 0: De-assert reset"]
    #[inline(always)]
    pub fn u0_core_1(&self) -> U0_CORE_1_R {
        U0_CORE_1_R::new(((self.bits >> 6) & 1) != 0)
    }
    #[doc = "Bit 7 - 1: Assert reset, 0: De-assert reset"]
    #[inline(always)]
    pub fn u0_core_2(&self) -> U0_CORE_2_R {
        U0_CORE_2_R::new(((self.bits >> 7) & 1) != 0)
    }
    #[doc = "Bit 8 - 1: Assert reset, 0: De-assert reset"]
    #[inline(always)]
    pub fn u0_core3(&self) -> U0_CORE3_R {
        U0_CORE3_R::new(((self.bits >> 8) & 1) != 0)
    }
    #[doc = "Bit 9 - 1: Assert reset, 0: De-assert reset"]
    #[inline(always)]
    pub fn u0_core4(&self) -> U0_CORE4_R {
        U0_CORE4_R::new(((self.bits >> 9) & 1) != 0)
    }
    #[doc = "Bit 10 - 1: Assert reset, 0: De-assert reset"]
    #[inline(always)]
    pub fn u0_core_st_0(&self) -> U0_CORE_ST_0_R {
        U0_CORE_ST_0_R::new(((self.bits >> 10) & 1) != 0)
    }
    #[doc = "Bit 11 - 1: Assert reset, 0: De-assert reset"]
    #[inline(always)]
    pub fn u0_core_st_1(&self) -> U0_CORE_ST_1_R {
        U0_CORE_ST_1_R::new(((self.bits >> 11) & 1) != 0)
    }
    #[doc = "Bit 12 - 1: Assert reset, 0: De-assert reset"]
    #[inline(always)]
    pub fn u0_core_st_2(&self) -> U0_CORE_ST_2_R {
        U0_CORE_ST_2_R::new(((self.bits >> 12) & 1) != 0)
    }
    #[doc = "Bit 13 - 1: Assert reset, 0: De-assert reset"]
    #[inline(always)]
    pub fn u0_core_st_3(&self) -> U0_CORE_ST_3_R {
        U0_CORE_ST_3_R::new(((self.bits >> 13) & 1) != 0)
    }
    #[doc = "Bit 14 - 1: Assert reset, 0: De-assert reset"]
    #[inline(always)]
    pub fn u0_core_st_4(&self) -> U0_CORE_ST_4_R {
        U0_CORE_ST_4_R::new(((self.bits >> 14) & 1) != 0)
    }
    #[doc = "Bit 15 - 1: Assert reset, 0: De-assert reset"]
    #[inline(always)]
    pub fn u0_trace_0(&self) -> U0_TRACE_0_R {
        U0_TRACE_0_R::new(((self.bits >> 15) & 1) != 0)
    }
    #[doc = "Bit 16 - 1: Assert reset, 0: De-assert reset"]
    #[inline(always)]
    pub fn u0_trace_1(&self) -> U0_TRACE_1_R {
        U0_TRACE_1_R::new(((self.bits >> 16) & 1) != 0)
    }
    #[doc = "Bit 17 - 1: Assert reset, 0: De-assert reset"]
    #[inline(always)]
    pub fn u0_trace_2(&self) -> U0_TRACE_2_R {
        U0_TRACE_2_R::new(((self.bits >> 17) & 1) != 0)
    }
    #[doc = "Bit 18 - 1: Assert reset, 0: De-assert reset"]
    #[inline(always)]
    pub fn u0_trace_3(&self) -> U0_TRACE_3_R {
        U0_TRACE_3_R::new(((self.bits >> 18) & 1) != 0)
    }
    #[doc = "Bit 19 - 1: Assert reset, 0: De-assert reset"]
    #[inline(always)]
    pub fn u0_trace_4(&self) -> U0_TRACE_4_R {
        U0_TRACE_4_R::new(((self.bits >> 19) & 1) != 0)
    }
    #[doc = "Bit 20 - 1: Assert reset, 0: De-assert reset"]
    #[inline(always)]
    pub fn u0_trace_com(&self) -> U0_TRACE_COM_R {
        U0_TRACE_COM_R::new(((self.bits >> 20) & 1) != 0)
    }
    #[doc = "Bit 21 - 1: Assert reset, 0: De-assert reset"]
    #[inline(always)]
    pub fn u0_img_gpu_apb(&self) -> U0_IMG_GPU_APB_R {
        U0_IMG_GPU_APB_R::new(((self.bits >> 21) & 1) != 0)
    }
    #[doc = "Bit 22 - 1: Assert reset, 0: De-assert reset"]
    #[inline(always)]
    pub fn u0_img_gpu_doma(&self) -> U0_IMG_GPU_DOMA_R {
        U0_IMG_GPU_DOMA_R::new(((self.bits >> 22) & 1) != 0)
    }
    #[doc = "Bit 23 - 1: Assert reset, 0: De-assert reset"]
    #[inline(always)]
    pub fn u0_noc_bus_apb(&self) -> U0_NOC_BUS_APB_R {
        U0_NOC_BUS_APB_R::new(((self.bits >> 23) & 1) != 0)
    }
    #[doc = "Bit 24 - 1: Assert reset, 0: De-assert reset"]
    #[inline(always)]
    pub fn u0_noc_bus_axicfg0(&self) -> U0_NOC_BUS_AXICFG0_R {
        U0_NOC_BUS_AXICFG0_R::new(((self.bits >> 24) & 1) != 0)
    }
    #[doc = "Bit 25 - 1: Assert reset, 0: De-assert reset"]
    #[inline(always)]
    pub fn u0_noc_bus_cpu_axi(&self) -> U0_NOC_BUS_CPU_AXI_R {
        U0_NOC_BUS_CPU_AXI_R::new(((self.bits >> 25) & 1) != 0)
    }
    #[doc = "Bit 26 - 1: Assert reset, 0: De-assert reset"]
    #[inline(always)]
    pub fn u0_noc_bus_disp_axi(&self) -> U0_NOC_BUS_DISP_AXI_R {
        U0_NOC_BUS_DISP_AXI_R::new(((self.bits >> 26) & 1) != 0)
    }
    #[doc = "Bit 27 - 1: Assert reset, 0: De-assert reset"]
    #[inline(always)]
    pub fn u0_noc_bus_gpu_axi(&self) -> U0_NOC_BUS_GPU_AXI_R {
        U0_NOC_BUS_GPU_AXI_R::new(((self.bits >> 27) & 1) != 0)
    }
    #[doc = "Bit 28 - 1: Assert reset, 0: De-assert reset"]
    #[inline(always)]
    pub fn u0_noc_bus_isp_axi(&self) -> U0_NOC_BUS_ISP_AXI_R {
        U0_NOC_BUS_ISP_AXI_R::new(((self.bits >> 28) & 1) != 0)
    }
    #[doc = "Bit 29 - 1: Assert reset, 0: De-assert reset"]
    #[inline(always)]
    pub fn u0_noc_bus_ddrc(&self) -> U0_NOC_BUS_DDRC_R {
        U0_NOC_BUS_DDRC_R::new(((self.bits >> 29) & 1) != 0)
    }
    #[doc = "Bit 30 - 1: Assert reset, 0: De-assert reset"]
    #[inline(always)]
    pub fn u0_noc_bus_stg_axi(&self) -> U0_NOC_BUS_STG_AXI_R {
        U0_NOC_BUS_STG_AXI_R::new(((self.bits >> 30) & 1) != 0)
    }
    #[doc = "Bit 31 - 1: Assert reset, 0: De-assert reset"]
    #[inline(always)]
    pub fn u0_noc_bus_vdec_axi(&self) -> U0_NOC_BUS_VDEC_AXI_R {
        U0_NOC_BUS_VDEC_AXI_R::new(((self.bits >> 31) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - 1: Assert reset, 0: De-assert reset"]
    #[inline(always)]
    #[must_use]
    pub fn u0_jtag2apb_presetn(&mut self) -> U0_JTAG2APB_PRESETN_W<SOFT_RST_ADDR_SEL_0_SPEC> {
        U0_JTAG2APB_PRESETN_W::new(self, 0)
    }
    #[doc = "Bit 1 - 1: Assert reset, 0: De-assert reset"]
    #[inline(always)]
    #[must_use]
    pub fn u0_sys_syscon_presetn(&mut self) -> U0_SYS_SYSCON_PRESETN_W<SOFT_RST_ADDR_SEL_0_SPEC> {
        U0_SYS_SYSCON_PRESETN_W::new(self, 1)
    }
    #[doc = "Bit 2 - 1: Assert reset, 0: De-assert reset"]
    #[inline(always)]
    #[must_use]
    pub fn u0_sys_iomux_presetn(&mut self) -> U0_SYS_IOMUX_PRESETN_W<SOFT_RST_ADDR_SEL_0_SPEC> {
        U0_SYS_IOMUX_PRESETN_W::new(self, 2)
    }
    #[doc = "Bit 3 - 1: Assert reset, 0: De-assert reset"]
    #[inline(always)]
    #[must_use]
    pub fn u0_bus(&mut self) -> U0_BUS_W<SOFT_RST_ADDR_SEL_0_SPEC> {
        U0_BUS_W::new(self, 3)
    }
    #[doc = "Bit 4 - 1: Assert reset, 0: De-assert reset"]
    #[inline(always)]
    #[must_use]
    pub fn u0_debug(&mut self) -> U0_DEBUG_W<SOFT_RST_ADDR_SEL_0_SPEC> {
        U0_DEBUG_W::new(self, 4)
    }
    #[doc = "Bit 5 - 1: Assert reset, 0: De-assert reset"]
    #[inline(always)]
    #[must_use]
    pub fn u0_core_0(&mut self) -> U0_CORE_0_W<SOFT_RST_ADDR_SEL_0_SPEC> {
        U0_CORE_0_W::new(self, 5)
    }
    #[doc = "Bit 6 - 1: Assert reset, 0: De-assert reset"]
    #[inline(always)]
    #[must_use]
    pub fn u0_core_1(&mut self) -> U0_CORE_1_W<SOFT_RST_ADDR_SEL_0_SPEC> {
        U0_CORE_1_W::new(self, 6)
    }
    #[doc = "Bit 7 - 1: Assert reset, 0: De-assert reset"]
    #[inline(always)]
    #[must_use]
    pub fn u0_core_2(&mut self) -> U0_CORE_2_W<SOFT_RST_ADDR_SEL_0_SPEC> {
        U0_CORE_2_W::new(self, 7)
    }
    #[doc = "Bit 8 - 1: Assert reset, 0: De-assert reset"]
    #[inline(always)]
    #[must_use]
    pub fn u0_core3(&mut self) -> U0_CORE3_W<SOFT_RST_ADDR_SEL_0_SPEC> {
        U0_CORE3_W::new(self, 8)
    }
    #[doc = "Bit 9 - 1: Assert reset, 0: De-assert reset"]
    #[inline(always)]
    #[must_use]
    pub fn u0_core4(&mut self) -> U0_CORE4_W<SOFT_RST_ADDR_SEL_0_SPEC> {
        U0_CORE4_W::new(self, 9)
    }
    #[doc = "Bit 10 - 1: Assert reset, 0: De-assert reset"]
    #[inline(always)]
    #[must_use]
    pub fn u0_core_st_0(&mut self) -> U0_CORE_ST_0_W<SOFT_RST_ADDR_SEL_0_SPEC> {
        U0_CORE_ST_0_W::new(self, 10)
    }
    #[doc = "Bit 11 - 1: Assert reset, 0: De-assert reset"]
    #[inline(always)]
    #[must_use]
    pub fn u0_core_st_1(&mut self) -> U0_CORE_ST_1_W<SOFT_RST_ADDR_SEL_0_SPEC> {
        U0_CORE_ST_1_W::new(self, 11)
    }
    #[doc = "Bit 12 - 1: Assert reset, 0: De-assert reset"]
    #[inline(always)]
    #[must_use]
    pub fn u0_core_st_2(&mut self) -> U0_CORE_ST_2_W<SOFT_RST_ADDR_SEL_0_SPEC> {
        U0_CORE_ST_2_W::new(self, 12)
    }
    #[doc = "Bit 13 - 1: Assert reset, 0: De-assert reset"]
    #[inline(always)]
    #[must_use]
    pub fn u0_core_st_3(&mut self) -> U0_CORE_ST_3_W<SOFT_RST_ADDR_SEL_0_SPEC> {
        U0_CORE_ST_3_W::new(self, 13)
    }
    #[doc = "Bit 14 - 1: Assert reset, 0: De-assert reset"]
    #[inline(always)]
    #[must_use]
    pub fn u0_core_st_4(&mut self) -> U0_CORE_ST_4_W<SOFT_RST_ADDR_SEL_0_SPEC> {
        U0_CORE_ST_4_W::new(self, 14)
    }
    #[doc = "Bit 15 - 1: Assert reset, 0: De-assert reset"]
    #[inline(always)]
    #[must_use]
    pub fn u0_trace_0(&mut self) -> U0_TRACE_0_W<SOFT_RST_ADDR_SEL_0_SPEC> {
        U0_TRACE_0_W::new(self, 15)
    }
    #[doc = "Bit 16 - 1: Assert reset, 0: De-assert reset"]
    #[inline(always)]
    #[must_use]
    pub fn u0_trace_1(&mut self) -> U0_TRACE_1_W<SOFT_RST_ADDR_SEL_0_SPEC> {
        U0_TRACE_1_W::new(self, 16)
    }
    #[doc = "Bit 17 - 1: Assert reset, 0: De-assert reset"]
    #[inline(always)]
    #[must_use]
    pub fn u0_trace_2(&mut self) -> U0_TRACE_2_W<SOFT_RST_ADDR_SEL_0_SPEC> {
        U0_TRACE_2_W::new(self, 17)
    }
    #[doc = "Bit 18 - 1: Assert reset, 0: De-assert reset"]
    #[inline(always)]
    #[must_use]
    pub fn u0_trace_3(&mut self) -> U0_TRACE_3_W<SOFT_RST_ADDR_SEL_0_SPEC> {
        U0_TRACE_3_W::new(self, 18)
    }
    #[doc = "Bit 19 - 1: Assert reset, 0: De-assert reset"]
    #[inline(always)]
    #[must_use]
    pub fn u0_trace_4(&mut self) -> U0_TRACE_4_W<SOFT_RST_ADDR_SEL_0_SPEC> {
        U0_TRACE_4_W::new(self, 19)
    }
    #[doc = "Bit 20 - 1: Assert reset, 0: De-assert reset"]
    #[inline(always)]
    #[must_use]
    pub fn u0_trace_com(&mut self) -> U0_TRACE_COM_W<SOFT_RST_ADDR_SEL_0_SPEC> {
        U0_TRACE_COM_W::new(self, 20)
    }
    #[doc = "Bit 21 - 1: Assert reset, 0: De-assert reset"]
    #[inline(always)]
    #[must_use]
    pub fn u0_img_gpu_apb(&mut self) -> U0_IMG_GPU_APB_W<SOFT_RST_ADDR_SEL_0_SPEC> {
        U0_IMG_GPU_APB_W::new(self, 21)
    }
    #[doc = "Bit 22 - 1: Assert reset, 0: De-assert reset"]
    #[inline(always)]
    #[must_use]
    pub fn u0_img_gpu_doma(&mut self) -> U0_IMG_GPU_DOMA_W<SOFT_RST_ADDR_SEL_0_SPEC> {
        U0_IMG_GPU_DOMA_W::new(self, 22)
    }
    #[doc = "Bit 23 - 1: Assert reset, 0: De-assert reset"]
    #[inline(always)]
    #[must_use]
    pub fn u0_noc_bus_apb(&mut self) -> U0_NOC_BUS_APB_W<SOFT_RST_ADDR_SEL_0_SPEC> {
        U0_NOC_BUS_APB_W::new(self, 23)
    }
    #[doc = "Bit 24 - 1: Assert reset, 0: De-assert reset"]
    #[inline(always)]
    #[must_use]
    pub fn u0_noc_bus_axicfg0(&mut self) -> U0_NOC_BUS_AXICFG0_W<SOFT_RST_ADDR_SEL_0_SPEC> {
        U0_NOC_BUS_AXICFG0_W::new(self, 24)
    }
    #[doc = "Bit 25 - 1: Assert reset, 0: De-assert reset"]
    #[inline(always)]
    #[must_use]
    pub fn u0_noc_bus_cpu_axi(&mut self) -> U0_NOC_BUS_CPU_AXI_W<SOFT_RST_ADDR_SEL_0_SPEC> {
        U0_NOC_BUS_CPU_AXI_W::new(self, 25)
    }
    #[doc = "Bit 26 - 1: Assert reset, 0: De-assert reset"]
    #[inline(always)]
    #[must_use]
    pub fn u0_noc_bus_disp_axi(&mut self) -> U0_NOC_BUS_DISP_AXI_W<SOFT_RST_ADDR_SEL_0_SPEC> {
        U0_NOC_BUS_DISP_AXI_W::new(self, 26)
    }
    #[doc = "Bit 27 - 1: Assert reset, 0: De-assert reset"]
    #[inline(always)]
    #[must_use]
    pub fn u0_noc_bus_gpu_axi(&mut self) -> U0_NOC_BUS_GPU_AXI_W<SOFT_RST_ADDR_SEL_0_SPEC> {
        U0_NOC_BUS_GPU_AXI_W::new(self, 27)
    }
    #[doc = "Bit 28 - 1: Assert reset, 0: De-assert reset"]
    #[inline(always)]
    #[must_use]
    pub fn u0_noc_bus_isp_axi(&mut self) -> U0_NOC_BUS_ISP_AXI_W<SOFT_RST_ADDR_SEL_0_SPEC> {
        U0_NOC_BUS_ISP_AXI_W::new(self, 28)
    }
    #[doc = "Bit 29 - 1: Assert reset, 0: De-assert reset"]
    #[inline(always)]
    #[must_use]
    pub fn u0_noc_bus_ddrc(&mut self) -> U0_NOC_BUS_DDRC_W<SOFT_RST_ADDR_SEL_0_SPEC> {
        U0_NOC_BUS_DDRC_W::new(self, 29)
    }
    #[doc = "Bit 30 - 1: Assert reset, 0: De-assert reset"]
    #[inline(always)]
    #[must_use]
    pub fn u0_noc_bus_stg_axi(&mut self) -> U0_NOC_BUS_STG_AXI_W<SOFT_RST_ADDR_SEL_0_SPEC> {
        U0_NOC_BUS_STG_AXI_W::new(self, 30)
    }
    #[doc = "Bit 31 - 1: Assert reset, 0: De-assert reset"]
    #[inline(always)]
    #[must_use]
    pub fn u0_noc_bus_vdec_axi(&mut self) -> U0_NOC_BUS_VDEC_AXI_W<SOFT_RST_ADDR_SEL_0_SPEC> {
        U0_NOC_BUS_VDEC_AXI_W::new(self, 31)
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
#[doc = "Software RESET 0 Address Selector\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`soft_rst_addr_sel_0::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`soft_rst_addr_sel_0::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct SOFT_RST_ADDR_SEL_0_SPEC;
impl crate::RegisterSpec for SOFT_RST_ADDR_SEL_0_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`soft_rst_addr_sel_0::R`](R) reader structure"]
impl crate::Readable for SOFT_RST_ADDR_SEL_0_SPEC {}
#[doc = "`write(|w| ..)` method takes [`soft_rst_addr_sel_0::W`](W) writer structure"]
impl crate::Writable for SOFT_RST_ADDR_SEL_0_SPEC {
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets soft_rst_addr_sel_0 to value 0"]
impl crate::Resettable for SOFT_RST_ADDR_SEL_0_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
