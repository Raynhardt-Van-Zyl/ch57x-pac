#[doc = "Register `R8_OSC_CAL_CTRL` reader"]
pub type R = crate::R<R8OscCalCtrlSpec>;
#[doc = "Register `R8_OSC_CAL_CTRL` writer"]
pub type W = crate::W<R8OscCalCtrlSpec>;
#[doc = "Field `RB_OSC_CNT_EN` reader - RWA, calibration counter enable"]
pub type RbOscCntEnR = crate::BitReader;
#[doc = "Field `RB_OSC_CNT_EN` writer - RWA, calibration counter enable"]
pub type RbOscCntEnW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `RB_OSC_CNT_HALT` reader - RO, calibration counter halt status: 0=counting, 1=halt for reading count value"]
pub type RbOscCntHaltR = crate::BitReader;
impl R {
    #[doc = "Bit 0 - RWA, calibration counter enable"]
    #[inline(always)]
    pub fn rb_osc_cnt_en(&self) -> RbOscCntEnR {
        RbOscCntEnR::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - RO, calibration counter halt status: 0=counting, 1=halt for reading count value"]
    #[inline(always)]
    pub fn rb_osc_cnt_halt(&self) -> RbOscCntHaltR {
        RbOscCntHaltR::new(((self.bits >> 1) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - RWA, calibration counter enable"]
    #[inline(always)]
    pub fn rb_osc_cnt_en(&mut self) -> RbOscCntEnW<'_, R8OscCalCtrlSpec> {
        RbOscCntEnW::new(self, 0)
    }
}
#[doc = "RWA, oscillator frequency calibration control, SAM\n\nYou can [`read`](crate::Reg::read) this register and get [`r8_osc_cal_ctrl::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`r8_osc_cal_ctrl::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct R8OscCalCtrlSpec;
impl crate::RegisterSpec for R8OscCalCtrlSpec {
    type Ux = u8;
}
#[doc = "`read()` method returns [`r8_osc_cal_ctrl::R`](R) reader structure"]
impl crate::Readable for R8OscCalCtrlSpec {}
#[doc = "`write(|w| ..)` method takes [`r8_osc_cal_ctrl::W`](W) writer structure"]
impl crate::Writable for R8OscCalCtrlSpec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets R8_OSC_CAL_CTRL to value 0x02"]
impl crate::Resettable for R8OscCalCtrlSpec {
    const RESET_VALUE: u8 = 0x02;
}
