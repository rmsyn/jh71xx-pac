#[doc = "Register `syscrg_rst0_status` reader"]
pub type R = crate::R<SYSCRG_RST0_STATUS_SPEC>;
#[doc = "Register `syscrg_rst0_status` writer"]
pub type W = crate::W<SYSCRG_RST0_STATUS_SPEC>;
#[doc = "Field `rstn_u0_jtag2apb_presetn` reader - 1: Assert reset, 0: De-assert reset"]
pub type RSTN_U0_JTAG2APB_PRESETN_R = crate::BitReader;
#[doc = "Field `rstn_u0_jtag2apb_presetn` writer - 1: Assert reset, 0: De-assert reset"]
pub type RSTN_U0_JTAG2APB_PRESETN_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `rstn_u0_sys_syscon_presetn` reader - 1: Assert reset, 0: De-assert reset"]
pub type RSTN_U0_SYS_SYSCON_PRESETN_R = crate::BitReader;
#[doc = "Field `rstn_u0_sys_syscon_presetn` writer - 1: Assert reset, 0: De-assert reset"]
pub type RSTN_U0_SYS_SYSCON_PRESETN_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `rstn_u0_sys_iomux_presetn` reader - 1: Assert reset, 0: De-assert reset"]
pub type RSTN_U0_SYS_IOMUX_PRESETN_R = crate::BitReader;
#[doc = "Field `rstn_u0_sys_iomux_presetn` writer - 1: Assert reset, 0: De-assert reset"]
pub type RSTN_U0_SYS_IOMUX_PRESETN_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `rst_u0_u7mc_sft7110_rst_bus` reader - 1: Assert reset, 0: De-assert reset"]
pub type RST_U0_U7MC_SFT7110_RST_BUS_R = crate::BitReader;
#[doc = "Field `rst_u0_u7mc_sft7110_rst_bus` writer - 1: Assert reset, 0: De-assert reset"]
pub type RST_U0_U7MC_SFT7110_RST_BUS_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `rst_u0_u7mc_sft7110_debug_reset` reader - 1: Assert reset, 0: De-assert reset"]
pub type RST_U0_U7MC_SFT7110_DEBUG_RESET_R = crate::BitReader;
#[doc = "Field `rst_u0_u7mc_sft7110_debug_reset` writer - 1: Assert reset, 0: De-assert reset"]
pub type RST_U0_U7MC_SFT7110_DEBUG_RESET_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `rst_u0_u7mc_sft7110_rst_core0` reader - 1: Assert reset, 0: De-assert reset"]
pub type RST_U0_U7MC_SFT7110_RST_CORE0_R = crate::BitReader;
#[doc = "Field `rst_u0_u7mc_sft7110_rst_core0` writer - 1: Assert reset, 0: De-assert reset"]
pub type RST_U0_U7MC_SFT7110_RST_CORE0_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `rst_u0_u7mc_sft7110_rst_core1` reader - 1: Assert reset, 0: De-assert reset"]
pub type RST_U0_U7MC_SFT7110_RST_CORE1_R = crate::BitReader;
#[doc = "Field `rst_u0_u7mc_sft7110_rst_core1` writer - 1: Assert reset, 0: De-assert reset"]
pub type RST_U0_U7MC_SFT7110_RST_CORE1_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `rst_u0_u7mc_sft7110_rst_core2` reader - 1: Assert reset, 0: De-assert reset"]
pub type RST_U0_U7MC_SFT7110_RST_CORE2_R = crate::BitReader;
#[doc = "Field `rst_u0_u7mc_sft7110_rst_core2` writer - 1: Assert reset, 0: De-assert reset"]
pub type RST_U0_U7MC_SFT7110_RST_CORE2_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `rst_u0_u7mc_sft7110_rst_core3` reader - 1: Assert reset, 0: De-assert reset"]
pub type RST_U0_U7MC_SFT7110_RST_CORE3_R = crate::BitReader;
#[doc = "Field `rst_u0_u7mc_sft7110_rst_core3` writer - 1: Assert reset, 0: De-assert reset"]
pub type RST_U0_U7MC_SFT7110_RST_CORE3_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `rst_u0_u7mc_sft7110_rst_core4` reader - 1: Assert reset, 0: De-assert reset"]
pub type RST_U0_U7MC_SFT7110_RST_CORE4_R = crate::BitReader;
#[doc = "Field `rst_u0_u7mc_sft7110_rst_core4` writer - 1: Assert reset, 0: De-assert reset"]
pub type RST_U0_U7MC_SFT7110_RST_CORE4_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `rst_u0_u7mc_sft7110_rst_core0_st` reader - 1: Assert reset, 0: De-assert reset"]
pub type RST_U0_U7MC_SFT7110_RST_CORE0_ST_R = crate::BitReader;
#[doc = "Field `rst_u0_u7mc_sft7110_rst_core0_st` writer - 1: Assert reset, 0: De-assert reset"]
pub type RST_U0_U7MC_SFT7110_RST_CORE0_ST_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `rst_u0_u7mc_sft7110_rst_core1_st` reader - 1: Assert reset, 0: De-assert reset"]
pub type RST_U0_U7MC_SFT7110_RST_CORE1_ST_R = crate::BitReader;
#[doc = "Field `rst_u0_u7mc_sft7110_rst_core1_st` writer - 1: Assert reset, 0: De-assert reset"]
pub type RST_U0_U7MC_SFT7110_RST_CORE1_ST_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `rst_u0_u7mc_sft7110_rst_core2_st` reader - 1: Assert reset, 0: De-assert reset"]
pub type RST_U0_U7MC_SFT7110_RST_CORE2_ST_R = crate::BitReader;
#[doc = "Field `rst_u0_u7mc_sft7110_rst_core2_st` writer - 1: Assert reset, 0: De-assert reset"]
pub type RST_U0_U7MC_SFT7110_RST_CORE2_ST_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `rst_u0_u7mc_sft7110_rst_core3_st` reader - 1: Assert reset, 0: De-assert reset"]
pub type RST_U0_U7MC_SFT7110_RST_CORE3_ST_R = crate::BitReader;
#[doc = "Field `rst_u0_u7mc_sft7110_rst_core3_st` writer - 1: Assert reset, 0: De-assert reset"]
pub type RST_U0_U7MC_SFT7110_RST_CORE3_ST_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `rst_u0_u7mc_sft7110_rst_core4_st` reader - 1: Assert reset, 0: De-assert reset"]
pub type RST_U0_U7MC_SFT7110_RST_CORE4_ST_R = crate::BitReader;
#[doc = "Field `rst_u0_u7mc_sft7110_rst_core4_st` writer - 1: Assert reset, 0: De-assert reset"]
pub type RST_U0_U7MC_SFT7110_RST_CORE4_ST_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `rst_u0_u7mc_sft7110_trace_rst0` reader - 1: Assert reset, 0: De-assert reset"]
pub type RST_U0_U7MC_SFT7110_TRACE_RST0_R = crate::BitReader;
#[doc = "Field `rst_u0_u7mc_sft7110_trace_rst0` writer - 1: Assert reset, 0: De-assert reset"]
pub type RST_U0_U7MC_SFT7110_TRACE_RST0_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `rst_u0_u7mc_sft7110_trace_rst1` reader - 1: Assert reset, 0: De-assert reset"]
pub type RST_U0_U7MC_SFT7110_TRACE_RST1_R = crate::BitReader;
#[doc = "Field `rst_u0_u7mc_sft7110_trace_rst1` writer - 1: Assert reset, 0: De-assert reset"]
pub type RST_U0_U7MC_SFT7110_TRACE_RST1_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `rst_u0_u7mc_sft7110_trace_rst2` reader - 1: Assert reset, 0: De-assert reset"]
pub type RST_U0_U7MC_SFT7110_TRACE_RST2_R = crate::BitReader;
#[doc = "Field `rst_u0_u7mc_sft7110_trace_rst2` writer - 1: Assert reset, 0: De-assert reset"]
pub type RST_U0_U7MC_SFT7110_TRACE_RST2_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `rst_u0_u7mc_sft7110_trace_rst3` reader - 1: Assert reset, 0: De-assert reset"]
pub type RST_U0_U7MC_SFT7110_TRACE_RST3_R = crate::BitReader;
#[doc = "Field `rst_u0_u7mc_sft7110_trace_rst3` writer - 1: Assert reset, 0: De-assert reset"]
pub type RST_U0_U7MC_SFT7110_TRACE_RST3_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `rst_u0_u7mc_sft7110_trace_rst4` reader - 1: Assert reset, 0: De-assert reset"]
pub type RST_U0_U7MC_SFT7110_TRACE_RST4_R = crate::BitReader;
#[doc = "Field `rst_u0_u7mc_sft7110_trace_rst4` writer - 1: Assert reset, 0: De-assert reset"]
pub type RST_U0_U7MC_SFT7110_TRACE_RST4_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `rst_u0_u7mc_sft7110_trace_com_rst` reader - 1: Assert reset, 0: De-assert reset"]
pub type RST_U0_U7MC_SFT7110_TRACE_COM_RST_R = crate::BitReader;
#[doc = "Field `rst_u0_u7mc_sft7110_trace_com_rst` writer - 1: Assert reset, 0: De-assert reset"]
pub type RST_U0_U7MC_SFT7110_TRACE_COM_RST_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `rst_u0_img_gpu_rstn_apb` reader - 1: Assert reset, 0: De-assert reset"]
pub type RST_U0_IMG_GPU_RSTN_APB_R = crate::BitReader;
#[doc = "Field `rst_u0_img_gpu_rstn_apb` writer - 1: Assert reset, 0: De-assert reset"]
pub type RST_U0_IMG_GPU_RSTN_APB_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `rst_u0_img_gpu_rstn_doma` reader - 1: Assert reset, 0: De-assert reset"]
pub type RST_U0_IMG_GPU_RSTN_DOMA_R = crate::BitReader;
#[doc = "Field `rst_u0_img_gpu_rstn_doma` writer - 1: Assert reset, 0: De-assert reset"]
pub type RST_U0_IMG_GPU_RSTN_DOMA_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `rst_u0_u7mc_sft7110_noc_bus_reset_apb_bus_n` reader - 1: Assert reset, 0: De-assert reset"]
pub type RST_U0_U7MC_SFT7110_NOC_BUS_RESET_APB_BUS_N_R = crate::BitReader;
#[doc = "Field `rst_u0_u7mc_sft7110_noc_bus_reset_apb_bus_n` writer - 1: Assert reset, 0: De-assert reset"]
pub type RST_U0_U7MC_SFT7110_NOC_BUS_RESET_APB_BUS_N_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `rst_u0_u7mc_sft7110_noc_bus_reset_axicfg0_axi_n` reader - 1: Assert reset, 0: De-assert reset"]
pub type RST_U0_U7MC_SFT7110_NOC_BUS_RESET_AXICFG0_AXI_N_R = crate::BitReader;
#[doc = "Field `rst_u0_u7mc_sft7110_noc_bus_reset_axicfg0_axi_n` writer - 1: Assert reset, 0: De-assert reset"]
pub type RST_U0_U7MC_SFT7110_NOC_BUS_RESET_AXICFG0_AXI_N_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `rst_u0_u7mc_sft7110_noc_bus_reset_cpu_axi_n` reader - 1: Assert reset, 0: De-assert reset"]
pub type RST_U0_U7MC_SFT7110_NOC_BUS_RESET_CPU_AXI_N_R = crate::BitReader;
#[doc = "Field `rst_u0_u7mc_sft7110_noc_bus_reset_cpu_axi_n` writer - 1: Assert reset, 0: De-assert reset"]
pub type RST_U0_U7MC_SFT7110_NOC_BUS_RESET_CPU_AXI_N_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `rst_u0_u7mc_sft7110_noc_bus_reset_disp_axi_n` reader - 1: Assert reset, 0: De-assert reset"]
pub type RST_U0_U7MC_SFT7110_NOC_BUS_RESET_DISP_AXI_N_R = crate::BitReader;
#[doc = "Field `rst_u0_u7mc_sft7110_noc_bus_reset_disp_axi_n` writer - 1: Assert reset, 0: De-assert reset"]
pub type RST_U0_U7MC_SFT7110_NOC_BUS_RESET_DISP_AXI_N_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `rst_u0_u7mc_sft7110_noc_bus_reset_gpu_axi_n` reader - 1: Assert reset, 0: De-assert reset"]
pub type RST_U0_U7MC_SFT7110_NOC_BUS_RESET_GPU_AXI_N_R = crate::BitReader;
#[doc = "Field `rst_u0_u7mc_sft7110_noc_bus_reset_gpu_axi_n` writer - 1: Assert reset, 0: De-assert reset"]
pub type RST_U0_U7MC_SFT7110_NOC_BUS_RESET_GPU_AXI_N_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `rst_u0_u7mc_sft7110_noc_bus_reset_isp_axi_n` reader - 1: Assert reset, 0: De-assert reset"]
pub type RST_U0_U7MC_SFT7110_NOC_BUS_RESET_ISP_AXI_N_R = crate::BitReader;
#[doc = "Field `rst_u0_u7mc_sft7110_noc_bus_reset_isp_axi_n` writer - 1: Assert reset, 0: De-assert reset"]
pub type RST_U0_U7MC_SFT7110_NOC_BUS_RESET_ISP_AXI_N_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `rst_u0_u7mc_sft7110_noc_bus_reset_ddrc_n` reader - 1: Assert reset, 0: De-assert reset"]
pub type RST_U0_U7MC_SFT7110_NOC_BUS_RESET_DDRC_N_R = crate::BitReader;
#[doc = "Field `rst_u0_u7mc_sft7110_noc_bus_reset_ddrc_n` writer - 1: Assert reset, 0: De-assert reset"]
pub type RST_U0_U7MC_SFT7110_NOC_BUS_RESET_DDRC_N_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `rst_u0_u7mc_sft7110_noc_bus_reset_stg_axi_n` reader - 1: Assert reset, 0: De-assert reset"]
pub type RST_U0_U7MC_SFT7110_NOC_BUS_RESET_STG_AXI_N_R = crate::BitReader;
#[doc = "Field `rst_u0_u7mc_sft7110_noc_bus_reset_stg_axi_n` writer - 1: Assert reset, 0: De-assert reset"]
pub type RST_U0_U7MC_SFT7110_NOC_BUS_RESET_STG_AXI_N_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `rst_u0_u7mc_sft7110_noc_bus_reset_vdec_axi_n` reader - 1: Assert reset, 0: De-assert reset"]
pub type RST_U0_U7MC_SFT7110_NOC_BUS_RESET_VDEC_AXI_N_R = crate::BitReader;
#[doc = "Field `rst_u0_u7mc_sft7110_noc_bus_reset_vdec_axi_n` writer - 1: Assert reset, 0: De-assert reset"]
pub type RST_U0_U7MC_SFT7110_NOC_BUS_RESET_VDEC_AXI_N_W<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bit 0 - 1: Assert reset, 0: De-assert reset"]
    #[inline(always)]
    pub fn rstn_u0_jtag2apb_presetn(&self) -> RSTN_U0_JTAG2APB_PRESETN_R {
        RSTN_U0_JTAG2APB_PRESETN_R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - 1: Assert reset, 0: De-assert reset"]
    #[inline(always)]
    pub fn rstn_u0_sys_syscon_presetn(&self) -> RSTN_U0_SYS_SYSCON_PRESETN_R {
        RSTN_U0_SYS_SYSCON_PRESETN_R::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - 1: Assert reset, 0: De-assert reset"]
    #[inline(always)]
    pub fn rstn_u0_sys_iomux_presetn(&self) -> RSTN_U0_SYS_IOMUX_PRESETN_R {
        RSTN_U0_SYS_IOMUX_PRESETN_R::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3 - 1: Assert reset, 0: De-assert reset"]
    #[inline(always)]
    pub fn rst_u0_u7mc_sft7110_rst_bus(&self) -> RST_U0_U7MC_SFT7110_RST_BUS_R {
        RST_U0_U7MC_SFT7110_RST_BUS_R::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bit 4 - 1: Assert reset, 0: De-assert reset"]
    #[inline(always)]
    pub fn rst_u0_u7mc_sft7110_debug_reset(&self) -> RST_U0_U7MC_SFT7110_DEBUG_RESET_R {
        RST_U0_U7MC_SFT7110_DEBUG_RESET_R::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bit 5 - 1: Assert reset, 0: De-assert reset"]
    #[inline(always)]
    pub fn rst_u0_u7mc_sft7110_rst_core0(&self) -> RST_U0_U7MC_SFT7110_RST_CORE0_R {
        RST_U0_U7MC_SFT7110_RST_CORE0_R::new(((self.bits >> 5) & 1) != 0)
    }
    #[doc = "Bit 6 - 1: Assert reset, 0: De-assert reset"]
    #[inline(always)]
    pub fn rst_u0_u7mc_sft7110_rst_core1(&self) -> RST_U0_U7MC_SFT7110_RST_CORE1_R {
        RST_U0_U7MC_SFT7110_RST_CORE1_R::new(((self.bits >> 6) & 1) != 0)
    }
    #[doc = "Bit 7 - 1: Assert reset, 0: De-assert reset"]
    #[inline(always)]
    pub fn rst_u0_u7mc_sft7110_rst_core2(&self) -> RST_U0_U7MC_SFT7110_RST_CORE2_R {
        RST_U0_U7MC_SFT7110_RST_CORE2_R::new(((self.bits >> 7) & 1) != 0)
    }
    #[doc = "Bit 8 - 1: Assert reset, 0: De-assert reset"]
    #[inline(always)]
    pub fn rst_u0_u7mc_sft7110_rst_core3(&self) -> RST_U0_U7MC_SFT7110_RST_CORE3_R {
        RST_U0_U7MC_SFT7110_RST_CORE3_R::new(((self.bits >> 8) & 1) != 0)
    }
    #[doc = "Bit 9 - 1: Assert reset, 0: De-assert reset"]
    #[inline(always)]
    pub fn rst_u0_u7mc_sft7110_rst_core4(&self) -> RST_U0_U7MC_SFT7110_RST_CORE4_R {
        RST_U0_U7MC_SFT7110_RST_CORE4_R::new(((self.bits >> 9) & 1) != 0)
    }
    #[doc = "Bit 10 - 1: Assert reset, 0: De-assert reset"]
    #[inline(always)]
    pub fn rst_u0_u7mc_sft7110_rst_core0_st(&self) -> RST_U0_U7MC_SFT7110_RST_CORE0_ST_R {
        RST_U0_U7MC_SFT7110_RST_CORE0_ST_R::new(((self.bits >> 10) & 1) != 0)
    }
    #[doc = "Bit 11 - 1: Assert reset, 0: De-assert reset"]
    #[inline(always)]
    pub fn rst_u0_u7mc_sft7110_rst_core1_st(&self) -> RST_U0_U7MC_SFT7110_RST_CORE1_ST_R {
        RST_U0_U7MC_SFT7110_RST_CORE1_ST_R::new(((self.bits >> 11) & 1) != 0)
    }
    #[doc = "Bit 12 - 1: Assert reset, 0: De-assert reset"]
    #[inline(always)]
    pub fn rst_u0_u7mc_sft7110_rst_core2_st(&self) -> RST_U0_U7MC_SFT7110_RST_CORE2_ST_R {
        RST_U0_U7MC_SFT7110_RST_CORE2_ST_R::new(((self.bits >> 12) & 1) != 0)
    }
    #[doc = "Bit 13 - 1: Assert reset, 0: De-assert reset"]
    #[inline(always)]
    pub fn rst_u0_u7mc_sft7110_rst_core3_st(&self) -> RST_U0_U7MC_SFT7110_RST_CORE3_ST_R {
        RST_U0_U7MC_SFT7110_RST_CORE3_ST_R::new(((self.bits >> 13) & 1) != 0)
    }
    #[doc = "Bit 14 - 1: Assert reset, 0: De-assert reset"]
    #[inline(always)]
    pub fn rst_u0_u7mc_sft7110_rst_core4_st(&self) -> RST_U0_U7MC_SFT7110_RST_CORE4_ST_R {
        RST_U0_U7MC_SFT7110_RST_CORE4_ST_R::new(((self.bits >> 14) & 1) != 0)
    }
    #[doc = "Bit 15 - 1: Assert reset, 0: De-assert reset"]
    #[inline(always)]
    pub fn rst_u0_u7mc_sft7110_trace_rst0(&self) -> RST_U0_U7MC_SFT7110_TRACE_RST0_R {
        RST_U0_U7MC_SFT7110_TRACE_RST0_R::new(((self.bits >> 15) & 1) != 0)
    }
    #[doc = "Bit 16 - 1: Assert reset, 0: De-assert reset"]
    #[inline(always)]
    pub fn rst_u0_u7mc_sft7110_trace_rst1(&self) -> RST_U0_U7MC_SFT7110_TRACE_RST1_R {
        RST_U0_U7MC_SFT7110_TRACE_RST1_R::new(((self.bits >> 16) & 1) != 0)
    }
    #[doc = "Bit 17 - 1: Assert reset, 0: De-assert reset"]
    #[inline(always)]
    pub fn rst_u0_u7mc_sft7110_trace_rst2(&self) -> RST_U0_U7MC_SFT7110_TRACE_RST2_R {
        RST_U0_U7MC_SFT7110_TRACE_RST2_R::new(((self.bits >> 17) & 1) != 0)
    }
    #[doc = "Bit 18 - 1: Assert reset, 0: De-assert reset"]
    #[inline(always)]
    pub fn rst_u0_u7mc_sft7110_trace_rst3(&self) -> RST_U0_U7MC_SFT7110_TRACE_RST3_R {
        RST_U0_U7MC_SFT7110_TRACE_RST3_R::new(((self.bits >> 18) & 1) != 0)
    }
    #[doc = "Bit 19 - 1: Assert reset, 0: De-assert reset"]
    #[inline(always)]
    pub fn rst_u0_u7mc_sft7110_trace_rst4(&self) -> RST_U0_U7MC_SFT7110_TRACE_RST4_R {
        RST_U0_U7MC_SFT7110_TRACE_RST4_R::new(((self.bits >> 19) & 1) != 0)
    }
    #[doc = "Bit 20 - 1: Assert reset, 0: De-assert reset"]
    #[inline(always)]
    pub fn rst_u0_u7mc_sft7110_trace_com_rst(&self) -> RST_U0_U7MC_SFT7110_TRACE_COM_RST_R {
        RST_U0_U7MC_SFT7110_TRACE_COM_RST_R::new(((self.bits >> 20) & 1) != 0)
    }
    #[doc = "Bit 21 - 1: Assert reset, 0: De-assert reset"]
    #[inline(always)]
    pub fn rst_u0_img_gpu_rstn_apb(&self) -> RST_U0_IMG_GPU_RSTN_APB_R {
        RST_U0_IMG_GPU_RSTN_APB_R::new(((self.bits >> 21) & 1) != 0)
    }
    #[doc = "Bit 22 - 1: Assert reset, 0: De-assert reset"]
    #[inline(always)]
    pub fn rst_u0_img_gpu_rstn_doma(&self) -> RST_U0_IMG_GPU_RSTN_DOMA_R {
        RST_U0_IMG_GPU_RSTN_DOMA_R::new(((self.bits >> 22) & 1) != 0)
    }
    #[doc = "Bit 23 - 1: Assert reset, 0: De-assert reset"]
    #[inline(always)]
    pub fn rst_u0_u7mc_sft7110_noc_bus_reset_apb_bus_n(
        &self,
    ) -> RST_U0_U7MC_SFT7110_NOC_BUS_RESET_APB_BUS_N_R {
        RST_U0_U7MC_SFT7110_NOC_BUS_RESET_APB_BUS_N_R::new(((self.bits >> 23) & 1) != 0)
    }
    #[doc = "Bit 24 - 1: Assert reset, 0: De-assert reset"]
    #[inline(always)]
    pub fn rst_u0_u7mc_sft7110_noc_bus_reset_axicfg0_axi_n(
        &self,
    ) -> RST_U0_U7MC_SFT7110_NOC_BUS_RESET_AXICFG0_AXI_N_R {
        RST_U0_U7MC_SFT7110_NOC_BUS_RESET_AXICFG0_AXI_N_R::new(((self.bits >> 24) & 1) != 0)
    }
    #[doc = "Bit 25 - 1: Assert reset, 0: De-assert reset"]
    #[inline(always)]
    pub fn rst_u0_u7mc_sft7110_noc_bus_reset_cpu_axi_n(
        &self,
    ) -> RST_U0_U7MC_SFT7110_NOC_BUS_RESET_CPU_AXI_N_R {
        RST_U0_U7MC_SFT7110_NOC_BUS_RESET_CPU_AXI_N_R::new(((self.bits >> 25) & 1) != 0)
    }
    #[doc = "Bit 26 - 1: Assert reset, 0: De-assert reset"]
    #[inline(always)]
    pub fn rst_u0_u7mc_sft7110_noc_bus_reset_disp_axi_n(
        &self,
    ) -> RST_U0_U7MC_SFT7110_NOC_BUS_RESET_DISP_AXI_N_R {
        RST_U0_U7MC_SFT7110_NOC_BUS_RESET_DISP_AXI_N_R::new(((self.bits >> 26) & 1) != 0)
    }
    #[doc = "Bit 27 - 1: Assert reset, 0: De-assert reset"]
    #[inline(always)]
    pub fn rst_u0_u7mc_sft7110_noc_bus_reset_gpu_axi_n(
        &self,
    ) -> RST_U0_U7MC_SFT7110_NOC_BUS_RESET_GPU_AXI_N_R {
        RST_U0_U7MC_SFT7110_NOC_BUS_RESET_GPU_AXI_N_R::new(((self.bits >> 27) & 1) != 0)
    }
    #[doc = "Bit 28 - 1: Assert reset, 0: De-assert reset"]
    #[inline(always)]
    pub fn rst_u0_u7mc_sft7110_noc_bus_reset_isp_axi_n(
        &self,
    ) -> RST_U0_U7MC_SFT7110_NOC_BUS_RESET_ISP_AXI_N_R {
        RST_U0_U7MC_SFT7110_NOC_BUS_RESET_ISP_AXI_N_R::new(((self.bits >> 28) & 1) != 0)
    }
    #[doc = "Bit 29 - 1: Assert reset, 0: De-assert reset"]
    #[inline(always)]
    pub fn rst_u0_u7mc_sft7110_noc_bus_reset_ddrc_n(
        &self,
    ) -> RST_U0_U7MC_SFT7110_NOC_BUS_RESET_DDRC_N_R {
        RST_U0_U7MC_SFT7110_NOC_BUS_RESET_DDRC_N_R::new(((self.bits >> 29) & 1) != 0)
    }
    #[doc = "Bit 30 - 1: Assert reset, 0: De-assert reset"]
    #[inline(always)]
    pub fn rst_u0_u7mc_sft7110_noc_bus_reset_stg_axi_n(
        &self,
    ) -> RST_U0_U7MC_SFT7110_NOC_BUS_RESET_STG_AXI_N_R {
        RST_U0_U7MC_SFT7110_NOC_BUS_RESET_STG_AXI_N_R::new(((self.bits >> 30) & 1) != 0)
    }
    #[doc = "Bit 31 - 1: Assert reset, 0: De-assert reset"]
    #[inline(always)]
    pub fn rst_u0_u7mc_sft7110_noc_bus_reset_vdec_axi_n(
        &self,
    ) -> RST_U0_U7MC_SFT7110_NOC_BUS_RESET_VDEC_AXI_N_R {
        RST_U0_U7MC_SFT7110_NOC_BUS_RESET_VDEC_AXI_N_R::new(((self.bits >> 31) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - 1: Assert reset, 0: De-assert reset"]
    #[inline(always)]
    #[must_use]
    pub fn rstn_u0_jtag2apb_presetn(
        &mut self,
    ) -> RSTN_U0_JTAG2APB_PRESETN_W<SYSCRG_RST0_STATUS_SPEC> {
        RSTN_U0_JTAG2APB_PRESETN_W::new(self, 0)
    }
    #[doc = "Bit 1 - 1: Assert reset, 0: De-assert reset"]
    #[inline(always)]
    #[must_use]
    pub fn rstn_u0_sys_syscon_presetn(
        &mut self,
    ) -> RSTN_U0_SYS_SYSCON_PRESETN_W<SYSCRG_RST0_STATUS_SPEC> {
        RSTN_U0_SYS_SYSCON_PRESETN_W::new(self, 1)
    }
    #[doc = "Bit 2 - 1: Assert reset, 0: De-assert reset"]
    #[inline(always)]
    #[must_use]
    pub fn rstn_u0_sys_iomux_presetn(
        &mut self,
    ) -> RSTN_U0_SYS_IOMUX_PRESETN_W<SYSCRG_RST0_STATUS_SPEC> {
        RSTN_U0_SYS_IOMUX_PRESETN_W::new(self, 2)
    }
    #[doc = "Bit 3 - 1: Assert reset, 0: De-assert reset"]
    #[inline(always)]
    #[must_use]
    pub fn rst_u0_u7mc_sft7110_rst_bus(
        &mut self,
    ) -> RST_U0_U7MC_SFT7110_RST_BUS_W<SYSCRG_RST0_STATUS_SPEC> {
        RST_U0_U7MC_SFT7110_RST_BUS_W::new(self, 3)
    }
    #[doc = "Bit 4 - 1: Assert reset, 0: De-assert reset"]
    #[inline(always)]
    #[must_use]
    pub fn rst_u0_u7mc_sft7110_debug_reset(
        &mut self,
    ) -> RST_U0_U7MC_SFT7110_DEBUG_RESET_W<SYSCRG_RST0_STATUS_SPEC> {
        RST_U0_U7MC_SFT7110_DEBUG_RESET_W::new(self, 4)
    }
    #[doc = "Bit 5 - 1: Assert reset, 0: De-assert reset"]
    #[inline(always)]
    #[must_use]
    pub fn rst_u0_u7mc_sft7110_rst_core0(
        &mut self,
    ) -> RST_U0_U7MC_SFT7110_RST_CORE0_W<SYSCRG_RST0_STATUS_SPEC> {
        RST_U0_U7MC_SFT7110_RST_CORE0_W::new(self, 5)
    }
    #[doc = "Bit 6 - 1: Assert reset, 0: De-assert reset"]
    #[inline(always)]
    #[must_use]
    pub fn rst_u0_u7mc_sft7110_rst_core1(
        &mut self,
    ) -> RST_U0_U7MC_SFT7110_RST_CORE1_W<SYSCRG_RST0_STATUS_SPEC> {
        RST_U0_U7MC_SFT7110_RST_CORE1_W::new(self, 6)
    }
    #[doc = "Bit 7 - 1: Assert reset, 0: De-assert reset"]
    #[inline(always)]
    #[must_use]
    pub fn rst_u0_u7mc_sft7110_rst_core2(
        &mut self,
    ) -> RST_U0_U7MC_SFT7110_RST_CORE2_W<SYSCRG_RST0_STATUS_SPEC> {
        RST_U0_U7MC_SFT7110_RST_CORE2_W::new(self, 7)
    }
    #[doc = "Bit 8 - 1: Assert reset, 0: De-assert reset"]
    #[inline(always)]
    #[must_use]
    pub fn rst_u0_u7mc_sft7110_rst_core3(
        &mut self,
    ) -> RST_U0_U7MC_SFT7110_RST_CORE3_W<SYSCRG_RST0_STATUS_SPEC> {
        RST_U0_U7MC_SFT7110_RST_CORE3_W::new(self, 8)
    }
    #[doc = "Bit 9 - 1: Assert reset, 0: De-assert reset"]
    #[inline(always)]
    #[must_use]
    pub fn rst_u0_u7mc_sft7110_rst_core4(
        &mut self,
    ) -> RST_U0_U7MC_SFT7110_RST_CORE4_W<SYSCRG_RST0_STATUS_SPEC> {
        RST_U0_U7MC_SFT7110_RST_CORE4_W::new(self, 9)
    }
    #[doc = "Bit 10 - 1: Assert reset, 0: De-assert reset"]
    #[inline(always)]
    #[must_use]
    pub fn rst_u0_u7mc_sft7110_rst_core0_st(
        &mut self,
    ) -> RST_U0_U7MC_SFT7110_RST_CORE0_ST_W<SYSCRG_RST0_STATUS_SPEC> {
        RST_U0_U7MC_SFT7110_RST_CORE0_ST_W::new(self, 10)
    }
    #[doc = "Bit 11 - 1: Assert reset, 0: De-assert reset"]
    #[inline(always)]
    #[must_use]
    pub fn rst_u0_u7mc_sft7110_rst_core1_st(
        &mut self,
    ) -> RST_U0_U7MC_SFT7110_RST_CORE1_ST_W<SYSCRG_RST0_STATUS_SPEC> {
        RST_U0_U7MC_SFT7110_RST_CORE1_ST_W::new(self, 11)
    }
    #[doc = "Bit 12 - 1: Assert reset, 0: De-assert reset"]
    #[inline(always)]
    #[must_use]
    pub fn rst_u0_u7mc_sft7110_rst_core2_st(
        &mut self,
    ) -> RST_U0_U7MC_SFT7110_RST_CORE2_ST_W<SYSCRG_RST0_STATUS_SPEC> {
        RST_U0_U7MC_SFT7110_RST_CORE2_ST_W::new(self, 12)
    }
    #[doc = "Bit 13 - 1: Assert reset, 0: De-assert reset"]
    #[inline(always)]
    #[must_use]
    pub fn rst_u0_u7mc_sft7110_rst_core3_st(
        &mut self,
    ) -> RST_U0_U7MC_SFT7110_RST_CORE3_ST_W<SYSCRG_RST0_STATUS_SPEC> {
        RST_U0_U7MC_SFT7110_RST_CORE3_ST_W::new(self, 13)
    }
    #[doc = "Bit 14 - 1: Assert reset, 0: De-assert reset"]
    #[inline(always)]
    #[must_use]
    pub fn rst_u0_u7mc_sft7110_rst_core4_st(
        &mut self,
    ) -> RST_U0_U7MC_SFT7110_RST_CORE4_ST_W<SYSCRG_RST0_STATUS_SPEC> {
        RST_U0_U7MC_SFT7110_RST_CORE4_ST_W::new(self, 14)
    }
    #[doc = "Bit 15 - 1: Assert reset, 0: De-assert reset"]
    #[inline(always)]
    #[must_use]
    pub fn rst_u0_u7mc_sft7110_trace_rst0(
        &mut self,
    ) -> RST_U0_U7MC_SFT7110_TRACE_RST0_W<SYSCRG_RST0_STATUS_SPEC> {
        RST_U0_U7MC_SFT7110_TRACE_RST0_W::new(self, 15)
    }
    #[doc = "Bit 16 - 1: Assert reset, 0: De-assert reset"]
    #[inline(always)]
    #[must_use]
    pub fn rst_u0_u7mc_sft7110_trace_rst1(
        &mut self,
    ) -> RST_U0_U7MC_SFT7110_TRACE_RST1_W<SYSCRG_RST0_STATUS_SPEC> {
        RST_U0_U7MC_SFT7110_TRACE_RST1_W::new(self, 16)
    }
    #[doc = "Bit 17 - 1: Assert reset, 0: De-assert reset"]
    #[inline(always)]
    #[must_use]
    pub fn rst_u0_u7mc_sft7110_trace_rst2(
        &mut self,
    ) -> RST_U0_U7MC_SFT7110_TRACE_RST2_W<SYSCRG_RST0_STATUS_SPEC> {
        RST_U0_U7MC_SFT7110_TRACE_RST2_W::new(self, 17)
    }
    #[doc = "Bit 18 - 1: Assert reset, 0: De-assert reset"]
    #[inline(always)]
    #[must_use]
    pub fn rst_u0_u7mc_sft7110_trace_rst3(
        &mut self,
    ) -> RST_U0_U7MC_SFT7110_TRACE_RST3_W<SYSCRG_RST0_STATUS_SPEC> {
        RST_U0_U7MC_SFT7110_TRACE_RST3_W::new(self, 18)
    }
    #[doc = "Bit 19 - 1: Assert reset, 0: De-assert reset"]
    #[inline(always)]
    #[must_use]
    pub fn rst_u0_u7mc_sft7110_trace_rst4(
        &mut self,
    ) -> RST_U0_U7MC_SFT7110_TRACE_RST4_W<SYSCRG_RST0_STATUS_SPEC> {
        RST_U0_U7MC_SFT7110_TRACE_RST4_W::new(self, 19)
    }
    #[doc = "Bit 20 - 1: Assert reset, 0: De-assert reset"]
    #[inline(always)]
    #[must_use]
    pub fn rst_u0_u7mc_sft7110_trace_com_rst(
        &mut self,
    ) -> RST_U0_U7MC_SFT7110_TRACE_COM_RST_W<SYSCRG_RST0_STATUS_SPEC> {
        RST_U0_U7MC_SFT7110_TRACE_COM_RST_W::new(self, 20)
    }
    #[doc = "Bit 21 - 1: Assert reset, 0: De-assert reset"]
    #[inline(always)]
    #[must_use]
    pub fn rst_u0_img_gpu_rstn_apb(
        &mut self,
    ) -> RST_U0_IMG_GPU_RSTN_APB_W<SYSCRG_RST0_STATUS_SPEC> {
        RST_U0_IMG_GPU_RSTN_APB_W::new(self, 21)
    }
    #[doc = "Bit 22 - 1: Assert reset, 0: De-assert reset"]
    #[inline(always)]
    #[must_use]
    pub fn rst_u0_img_gpu_rstn_doma(
        &mut self,
    ) -> RST_U0_IMG_GPU_RSTN_DOMA_W<SYSCRG_RST0_STATUS_SPEC> {
        RST_U0_IMG_GPU_RSTN_DOMA_W::new(self, 22)
    }
    #[doc = "Bit 23 - 1: Assert reset, 0: De-assert reset"]
    #[inline(always)]
    #[must_use]
    pub fn rst_u0_u7mc_sft7110_noc_bus_reset_apb_bus_n(
        &mut self,
    ) -> RST_U0_U7MC_SFT7110_NOC_BUS_RESET_APB_BUS_N_W<SYSCRG_RST0_STATUS_SPEC> {
        RST_U0_U7MC_SFT7110_NOC_BUS_RESET_APB_BUS_N_W::new(self, 23)
    }
    #[doc = "Bit 24 - 1: Assert reset, 0: De-assert reset"]
    #[inline(always)]
    #[must_use]
    pub fn rst_u0_u7mc_sft7110_noc_bus_reset_axicfg0_axi_n(
        &mut self,
    ) -> RST_U0_U7MC_SFT7110_NOC_BUS_RESET_AXICFG0_AXI_N_W<SYSCRG_RST0_STATUS_SPEC> {
        RST_U0_U7MC_SFT7110_NOC_BUS_RESET_AXICFG0_AXI_N_W::new(self, 24)
    }
    #[doc = "Bit 25 - 1: Assert reset, 0: De-assert reset"]
    #[inline(always)]
    #[must_use]
    pub fn rst_u0_u7mc_sft7110_noc_bus_reset_cpu_axi_n(
        &mut self,
    ) -> RST_U0_U7MC_SFT7110_NOC_BUS_RESET_CPU_AXI_N_W<SYSCRG_RST0_STATUS_SPEC> {
        RST_U0_U7MC_SFT7110_NOC_BUS_RESET_CPU_AXI_N_W::new(self, 25)
    }
    #[doc = "Bit 26 - 1: Assert reset, 0: De-assert reset"]
    #[inline(always)]
    #[must_use]
    pub fn rst_u0_u7mc_sft7110_noc_bus_reset_disp_axi_n(
        &mut self,
    ) -> RST_U0_U7MC_SFT7110_NOC_BUS_RESET_DISP_AXI_N_W<SYSCRG_RST0_STATUS_SPEC> {
        RST_U0_U7MC_SFT7110_NOC_BUS_RESET_DISP_AXI_N_W::new(self, 26)
    }
    #[doc = "Bit 27 - 1: Assert reset, 0: De-assert reset"]
    #[inline(always)]
    #[must_use]
    pub fn rst_u0_u7mc_sft7110_noc_bus_reset_gpu_axi_n(
        &mut self,
    ) -> RST_U0_U7MC_SFT7110_NOC_BUS_RESET_GPU_AXI_N_W<SYSCRG_RST0_STATUS_SPEC> {
        RST_U0_U7MC_SFT7110_NOC_BUS_RESET_GPU_AXI_N_W::new(self, 27)
    }
    #[doc = "Bit 28 - 1: Assert reset, 0: De-assert reset"]
    #[inline(always)]
    #[must_use]
    pub fn rst_u0_u7mc_sft7110_noc_bus_reset_isp_axi_n(
        &mut self,
    ) -> RST_U0_U7MC_SFT7110_NOC_BUS_RESET_ISP_AXI_N_W<SYSCRG_RST0_STATUS_SPEC> {
        RST_U0_U7MC_SFT7110_NOC_BUS_RESET_ISP_AXI_N_W::new(self, 28)
    }
    #[doc = "Bit 29 - 1: Assert reset, 0: De-assert reset"]
    #[inline(always)]
    #[must_use]
    pub fn rst_u0_u7mc_sft7110_noc_bus_reset_ddrc_n(
        &mut self,
    ) -> RST_U0_U7MC_SFT7110_NOC_BUS_RESET_DDRC_N_W<SYSCRG_RST0_STATUS_SPEC> {
        RST_U0_U7MC_SFT7110_NOC_BUS_RESET_DDRC_N_W::new(self, 29)
    }
    #[doc = "Bit 30 - 1: Assert reset, 0: De-assert reset"]
    #[inline(always)]
    #[must_use]
    pub fn rst_u0_u7mc_sft7110_noc_bus_reset_stg_axi_n(
        &mut self,
    ) -> RST_U0_U7MC_SFT7110_NOC_BUS_RESET_STG_AXI_N_W<SYSCRG_RST0_STATUS_SPEC> {
        RST_U0_U7MC_SFT7110_NOC_BUS_RESET_STG_AXI_N_W::new(self, 30)
    }
    #[doc = "Bit 31 - 1: Assert reset, 0: De-assert reset"]
    #[inline(always)]
    #[must_use]
    pub fn rst_u0_u7mc_sft7110_noc_bus_reset_vdec_axi_n(
        &mut self,
    ) -> RST_U0_U7MC_SFT7110_NOC_BUS_RESET_VDEC_AXI_N_W<SYSCRG_RST0_STATUS_SPEC> {
        RST_U0_U7MC_SFT7110_NOC_BUS_RESET_VDEC_AXI_N_W::new(self, 31)
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
#[doc = "SYSCRG RESET Status 0\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`syscrg_rst0_status::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`syscrg_rst0_status::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct SYSCRG_RST0_STATUS_SPEC;
impl crate::RegisterSpec for SYSCRG_RST0_STATUS_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`syscrg_rst0_status::R`](R) reader structure"]
impl crate::Readable for SYSCRG_RST0_STATUS_SPEC {}
#[doc = "`write(|w| ..)` method takes [`syscrg_rst0_status::W`](W) writer structure"]
impl crate::Writable for SYSCRG_RST0_STATUS_SPEC {
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets syscrg_rst0_status to value 0"]
impl crate::Resettable for SYSCRG_RST0_STATUS_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
