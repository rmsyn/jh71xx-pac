#[doc = "Register `sys_syscfg_25` reader"]
pub type R = crate::R<SYS_SYSCFG_25_SPEC>;
#[doc = "Register `sys_syscfg_25` writer"]
pub type W = crate::W<SYS_SYSCFG_25_SPEC>;
#[doc = "Field `u0_trace_mtx_scfg_c3_in0_ctl` reader - u0_trace_mtx_scfg_c3_in0_ctl"]
pub type U0_TRACE_MTX_SCFG_C3_IN0_CTL_R = crate::FieldReader;
#[doc = "Field `u0_trace_mtx_scfg_c3_in0_ctl` writer - u0_trace_mtx_scfg_c3_in0_ctl"]
pub type U0_TRACE_MTX_SCFG_C3_IN0_CTL_W<'a, REG> = crate::FieldWriter<'a, REG, 5>;
#[doc = "Field `u0_trace_mtx_scfg_c3_in1_ctl` reader - u0_trace_mtx_scfg_c3_in1_ctl"]
pub type U0_TRACE_MTX_SCFG_C3_IN1_CTL_R = crate::FieldReader;
#[doc = "Field `u0_trace_mtx_scfg_c3_in1_ctl` writer - u0_trace_mtx_scfg_c3_in1_ctl"]
pub type U0_TRACE_MTX_SCFG_C3_IN1_CTL_W<'a, REG> = crate::FieldWriter<'a, REG, 5>;
#[doc = "Field `u0_trace_mtx_scfg_c4_in0_ctl` reader - u0_trace_mtx_scfg_c4_in0_ctl"]
pub type U0_TRACE_MTX_SCFG_C4_IN0_CTL_R = crate::FieldReader;
#[doc = "Field `u0_trace_mtx_scfg_c4_in0_ctl` writer - u0_trace_mtx_scfg_c4_in0_ctl"]
pub type U0_TRACE_MTX_SCFG_C4_IN0_CTL_W<'a, REG> = crate::FieldWriter<'a, REG, 5>;
#[doc = "Field `u0_trace_mtx_scfg_c4_in1_ctl` reader - u0_trace_mtx_scfg_c4_in1_ctl"]
pub type U0_TRACE_MTX_SCFG_C4_IN1_CTL_R = crate::FieldReader;
#[doc = "Field `u0_trace_mtx_scfg_c4_in1_ctl` writer - u0_trace_mtx_scfg_c4_in1_ctl"]
pub type U0_TRACE_MTX_SCFG_C4_IN1_CTL_W<'a, REG> = crate::FieldWriter<'a, REG, 5>;
#[doc = "Field `u0_cease_from_tile_0` reader - u0_cease_from_tile_0"]
pub type U0_CEASE_FROM_TILE_0_R = crate::BitReader;
#[doc = "Field `u0_cease_from_tile_1` reader - u0_cease_from_tile_1"]
pub type U0_CEASE_FROM_TILE_1_R = crate::BitReader;
#[doc = "Field `u0_cease_from_tile_2` reader - u0_cease_from_tile_2"]
pub type U0_CEASE_FROM_TILE_2_R = crate::BitReader;
#[doc = "Field `u0_cease_from_tile_3` reader - u0_cease_from_tile_3"]
pub type U0_CEASE_FROM_TILE_3_R = crate::BitReader;
#[doc = "Field `u0_cease_from_tile_4` reader - u0_cease_from_tile_4"]
pub type U0_CEASE_FROM_TILE_4_R = crate::BitReader;
#[doc = "Field `u0_halt_from_tile_0` reader - u0_halt_from_tile_0"]
pub type U0_HALT_FROM_TILE_0_R = crate::BitReader;
#[doc = "Field `u0_halt_from_tile_1` reader - u0_halt_from_tile_1"]
pub type U0_HALT_FROM_TILE_1_R = crate::BitReader;
#[doc = "Field `u0_halt_from_tile_2` reader - u0_halt_from_tile_2"]
pub type U0_HALT_FROM_TILE_2_R = crate::BitReader;
#[doc = "Field `u0_halt_from_tile_3` reader - u0_halt_from_tile_3"]
pub type U0_HALT_FROM_TILE_3_R = crate::BitReader;
#[doc = "Field `u0_halt_from_tile_4` reader - u0_halt_from_tile_4"]
pub type U0_HALT_FROM_TILE_4_R = crate::BitReader;
impl R {
    #[doc = "Bits 0:4 - u0_trace_mtx_scfg_c3_in0_ctl"]
    #[inline(always)]
    pub fn u0_trace_mtx_scfg_c3_in0_ctl(&self) -> U0_TRACE_MTX_SCFG_C3_IN0_CTL_R {
        U0_TRACE_MTX_SCFG_C3_IN0_CTL_R::new((self.bits & 0x1f) as u8)
    }
    #[doc = "Bits 5:9 - u0_trace_mtx_scfg_c3_in1_ctl"]
    #[inline(always)]
    pub fn u0_trace_mtx_scfg_c3_in1_ctl(&self) -> U0_TRACE_MTX_SCFG_C3_IN1_CTL_R {
        U0_TRACE_MTX_SCFG_C3_IN1_CTL_R::new(((self.bits >> 5) & 0x1f) as u8)
    }
    #[doc = "Bits 10:14 - u0_trace_mtx_scfg_c4_in0_ctl"]
    #[inline(always)]
    pub fn u0_trace_mtx_scfg_c4_in0_ctl(&self) -> U0_TRACE_MTX_SCFG_C4_IN0_CTL_R {
        U0_TRACE_MTX_SCFG_C4_IN0_CTL_R::new(((self.bits >> 10) & 0x1f) as u8)
    }
    #[doc = "Bits 15:19 - u0_trace_mtx_scfg_c4_in1_ctl"]
    #[inline(always)]
    pub fn u0_trace_mtx_scfg_c4_in1_ctl(&self) -> U0_TRACE_MTX_SCFG_C4_IN1_CTL_R {
        U0_TRACE_MTX_SCFG_C4_IN1_CTL_R::new(((self.bits >> 15) & 0x1f) as u8)
    }
    #[doc = "Bit 20 - u0_cease_from_tile_0"]
    #[inline(always)]
    pub fn u0_cease_from_tile_0(&self) -> U0_CEASE_FROM_TILE_0_R {
        U0_CEASE_FROM_TILE_0_R::new(((self.bits >> 20) & 1) != 0)
    }
    #[doc = "Bit 21 - u0_cease_from_tile_1"]
    #[inline(always)]
    pub fn u0_cease_from_tile_1(&self) -> U0_CEASE_FROM_TILE_1_R {
        U0_CEASE_FROM_TILE_1_R::new(((self.bits >> 21) & 1) != 0)
    }
    #[doc = "Bit 22 - u0_cease_from_tile_2"]
    #[inline(always)]
    pub fn u0_cease_from_tile_2(&self) -> U0_CEASE_FROM_TILE_2_R {
        U0_CEASE_FROM_TILE_2_R::new(((self.bits >> 22) & 1) != 0)
    }
    #[doc = "Bit 23 - u0_cease_from_tile_3"]
    #[inline(always)]
    pub fn u0_cease_from_tile_3(&self) -> U0_CEASE_FROM_TILE_3_R {
        U0_CEASE_FROM_TILE_3_R::new(((self.bits >> 23) & 1) != 0)
    }
    #[doc = "Bit 24 - u0_cease_from_tile_4"]
    #[inline(always)]
    pub fn u0_cease_from_tile_4(&self) -> U0_CEASE_FROM_TILE_4_R {
        U0_CEASE_FROM_TILE_4_R::new(((self.bits >> 24) & 1) != 0)
    }
    #[doc = "Bit 25 - u0_halt_from_tile_0"]
    #[inline(always)]
    pub fn u0_halt_from_tile_0(&self) -> U0_HALT_FROM_TILE_0_R {
        U0_HALT_FROM_TILE_0_R::new(((self.bits >> 25) & 1) != 0)
    }
    #[doc = "Bit 26 - u0_halt_from_tile_1"]
    #[inline(always)]
    pub fn u0_halt_from_tile_1(&self) -> U0_HALT_FROM_TILE_1_R {
        U0_HALT_FROM_TILE_1_R::new(((self.bits >> 26) & 1) != 0)
    }
    #[doc = "Bit 27 - u0_halt_from_tile_2"]
    #[inline(always)]
    pub fn u0_halt_from_tile_2(&self) -> U0_HALT_FROM_TILE_2_R {
        U0_HALT_FROM_TILE_2_R::new(((self.bits >> 27) & 1) != 0)
    }
    #[doc = "Bit 28 - u0_halt_from_tile_3"]
    #[inline(always)]
    pub fn u0_halt_from_tile_3(&self) -> U0_HALT_FROM_TILE_3_R {
        U0_HALT_FROM_TILE_3_R::new(((self.bits >> 28) & 1) != 0)
    }
    #[doc = "Bit 29 - u0_halt_from_tile_4"]
    #[inline(always)]
    pub fn u0_halt_from_tile_4(&self) -> U0_HALT_FROM_TILE_4_R {
        U0_HALT_FROM_TILE_4_R::new(((self.bits >> 29) & 1) != 0)
    }
}
impl W {
    #[doc = "Bits 0:4 - u0_trace_mtx_scfg_c3_in0_ctl"]
    #[inline(always)]
    #[must_use]
    pub fn u0_trace_mtx_scfg_c3_in0_ctl(
        &mut self,
    ) -> U0_TRACE_MTX_SCFG_C3_IN0_CTL_W<SYS_SYSCFG_25_SPEC> {
        U0_TRACE_MTX_SCFG_C3_IN0_CTL_W::new(self, 0)
    }
    #[doc = "Bits 5:9 - u0_trace_mtx_scfg_c3_in1_ctl"]
    #[inline(always)]
    #[must_use]
    pub fn u0_trace_mtx_scfg_c3_in1_ctl(
        &mut self,
    ) -> U0_TRACE_MTX_SCFG_C3_IN1_CTL_W<SYS_SYSCFG_25_SPEC> {
        U0_TRACE_MTX_SCFG_C3_IN1_CTL_W::new(self, 5)
    }
    #[doc = "Bits 10:14 - u0_trace_mtx_scfg_c4_in0_ctl"]
    #[inline(always)]
    #[must_use]
    pub fn u0_trace_mtx_scfg_c4_in0_ctl(
        &mut self,
    ) -> U0_TRACE_MTX_SCFG_C4_IN0_CTL_W<SYS_SYSCFG_25_SPEC> {
        U0_TRACE_MTX_SCFG_C4_IN0_CTL_W::new(self, 10)
    }
    #[doc = "Bits 15:19 - u0_trace_mtx_scfg_c4_in1_ctl"]
    #[inline(always)]
    #[must_use]
    pub fn u0_trace_mtx_scfg_c4_in1_ctl(
        &mut self,
    ) -> U0_TRACE_MTX_SCFG_C4_IN1_CTL_W<SYS_SYSCFG_25_SPEC> {
        U0_TRACE_MTX_SCFG_C4_IN1_CTL_W::new(self, 15)
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
#[doc = "SYS SYSCONSAIF SYSCFG 100\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`sys_syscfg_25::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`sys_syscfg_25::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct SYS_SYSCFG_25_SPEC;
impl crate::RegisterSpec for SYS_SYSCFG_25_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`sys_syscfg_25::R`](R) reader structure"]
impl crate::Readable for SYS_SYSCFG_25_SPEC {}
#[doc = "`write(|w| ..)` method takes [`sys_syscfg_25::W`](W) writer structure"]
impl crate::Writable for SYS_SYSCFG_25_SPEC {
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets sys_syscfg_25 to value 0"]
impl crate::Resettable for SYS_SYSCFG_25_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
