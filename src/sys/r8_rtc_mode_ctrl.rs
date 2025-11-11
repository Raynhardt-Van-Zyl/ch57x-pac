#[doc = "Register `R8_RTC_MODE_CTRL` reader"]
pub type R = crate::R<R8RtcModeCtrlSpec>;
#[doc = "Register `R8_RTC_MODE_CTRL` writer"]
pub type W = crate::W<R8RtcModeCtrlSpec>;
#[doc = "Field `RB_RTC_TMR_MODE` reader - RWA, RTC timer mode: 000=0.125S, 001=0.25S, 010=0.5S, 011=1S, 100=2S, 101=4S, 110=8S, 111=16S"]
pub type RbRtcTmrModeR = crate::FieldReader;
#[doc = "Field `RB_RTC_TMR_MODE` writer - RWA, RTC timer mode: 000=0.125S, 001=0.25S, 010=0.5S, 011=1S, 100=2S, 101=4S, 110=8S, 111=16S"]
pub type RbRtcTmrModeW<'a, REG> = crate::FieldWriter<'a, REG, 3>;
#[doc = "Field `RB_RTC_IGNORE_B0` reader - RWA, force ignore bit0 for trigger mode: 0=compare bit0, 1=ignore bit0"]
pub type RbRtcIgnoreB0R = crate::BitReader;
#[doc = "Field `RB_RTC_IGNORE_B0` writer - RWA, force ignore bit0 for trigger mode: 0=compare bit0, 1=ignore bit0"]
pub type RbRtcIgnoreB0W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `RB_RTC_TMR_EN` reader - RWA, RTC timer mode enable"]
pub type RbRtcTmrEnR = crate::BitReader;
#[doc = "Field `RB_RTC_TMR_EN` writer - RWA, RTC timer mode enable"]
pub type RbRtcTmrEnW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `RB_RTC_TRIG_EN` reader - RWA, RTC trigger mode enable"]
pub type RbRtcTrigEnR = crate::BitReader;
#[doc = "Field `RB_RTC_TRIG_EN` writer - RWA, RTC trigger mode enable"]
pub type RbRtcTrigEnW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `RB_RTC_LOAD_LO` reader - RWA, set 1 to load RTC count low word R32_RTC_CNT_32K, auto clear after loaded"]
pub type RbRtcLoadLoR = crate::BitReader;
#[doc = "Field `RB_RTC_LOAD_LO` writer - RWA, set 1 to load RTC count low word R32_RTC_CNT_32K, auto clear after loaded"]
pub type RbRtcLoadLoW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `RB_RTC_LOAD_HI` reader - RWA, set 1 to load RTC count high word R32_RTC_CNT_DAY, auto clear after loaded"]
pub type RbRtcLoadHiR = crate::BitReader;
#[doc = "Field `RB_RTC_LOAD_HI` writer - RWA, set 1 to load RTC count high word R32_RTC_CNT_DAY, auto clear after loaded"]
pub type RbRtcLoadHiW<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bits 0:2 - RWA, RTC timer mode: 000=0.125S, 001=0.25S, 010=0.5S, 011=1S, 100=2S, 101=4S, 110=8S, 111=16S"]
    #[inline(always)]
    pub fn rb_rtc_tmr_mode(&self) -> RbRtcTmrModeR {
        RbRtcTmrModeR::new(self.bits & 7)
    }
    #[doc = "Bit 3 - RWA, force ignore bit0 for trigger mode: 0=compare bit0, 1=ignore bit0"]
    #[inline(always)]
    pub fn rb_rtc_ignore_b0(&self) -> RbRtcIgnoreB0R {
        RbRtcIgnoreB0R::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bit 4 - RWA, RTC timer mode enable"]
    #[inline(always)]
    pub fn rb_rtc_tmr_en(&self) -> RbRtcTmrEnR {
        RbRtcTmrEnR::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bit 5 - RWA, RTC trigger mode enable"]
    #[inline(always)]
    pub fn rb_rtc_trig_en(&self) -> RbRtcTrigEnR {
        RbRtcTrigEnR::new(((self.bits >> 5) & 1) != 0)
    }
    #[doc = "Bit 6 - RWA, set 1 to load RTC count low word R32_RTC_CNT_32K, auto clear after loaded"]
    #[inline(always)]
    pub fn rb_rtc_load_lo(&self) -> RbRtcLoadLoR {
        RbRtcLoadLoR::new(((self.bits >> 6) & 1) != 0)
    }
    #[doc = "Bit 7 - RWA, set 1 to load RTC count high word R32_RTC_CNT_DAY, auto clear after loaded"]
    #[inline(always)]
    pub fn rb_rtc_load_hi(&self) -> RbRtcLoadHiR {
        RbRtcLoadHiR::new(((self.bits >> 7) & 1) != 0)
    }
}
impl W {
    #[doc = "Bits 0:2 - RWA, RTC timer mode: 000=0.125S, 001=0.25S, 010=0.5S, 011=1S, 100=2S, 101=4S, 110=8S, 111=16S"]
    #[inline(always)]
    pub fn rb_rtc_tmr_mode(&mut self) -> RbRtcTmrModeW<'_, R8RtcModeCtrlSpec> {
        RbRtcTmrModeW::new(self, 0)
    }
    #[doc = "Bit 3 - RWA, force ignore bit0 for trigger mode: 0=compare bit0, 1=ignore bit0"]
    #[inline(always)]
    pub fn rb_rtc_ignore_b0(&mut self) -> RbRtcIgnoreB0W<'_, R8RtcModeCtrlSpec> {
        RbRtcIgnoreB0W::new(self, 3)
    }
    #[doc = "Bit 4 - RWA, RTC timer mode enable"]
    #[inline(always)]
    pub fn rb_rtc_tmr_en(&mut self) -> RbRtcTmrEnW<'_, R8RtcModeCtrlSpec> {
        RbRtcTmrEnW::new(self, 4)
    }
    #[doc = "Bit 5 - RWA, RTC trigger mode enable"]
    #[inline(always)]
    pub fn rb_rtc_trig_en(&mut self) -> RbRtcTrigEnW<'_, R8RtcModeCtrlSpec> {
        RbRtcTrigEnW::new(self, 5)
    }
    #[doc = "Bit 6 - RWA, set 1 to load RTC count low word R32_RTC_CNT_32K, auto clear after loaded"]
    #[inline(always)]
    pub fn rb_rtc_load_lo(&mut self) -> RbRtcLoadLoW<'_, R8RtcModeCtrlSpec> {
        RbRtcLoadLoW::new(self, 6)
    }
    #[doc = "Bit 7 - RWA, set 1 to load RTC count high word R32_RTC_CNT_DAY, auto clear after loaded"]
    #[inline(always)]
    pub fn rb_rtc_load_hi(&mut self) -> RbRtcLoadHiW<'_, R8RtcModeCtrlSpec> {
        RbRtcLoadHiW::new(self, 7)
    }
}
#[doc = "RWA, RTC mode control, SAM\n\nYou can [`read`](crate::Reg::read) this register and get [`r8_rtc_mode_ctrl::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`r8_rtc_mode_ctrl::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct R8RtcModeCtrlSpec;
impl crate::RegisterSpec for R8RtcModeCtrlSpec {
    type Ux = u8;
}
#[doc = "`read()` method returns [`r8_rtc_mode_ctrl::R`](R) reader structure"]
impl crate::Readable for R8RtcModeCtrlSpec {}
#[doc = "`write(|w| ..)` method takes [`r8_rtc_mode_ctrl::W`](W) writer structure"]
impl crate::Writable for R8RtcModeCtrlSpec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets R8_RTC_MODE_CTRL to value 0x02"]
impl crate::Resettable for R8RtcModeCtrlSpec {
    const RESET_VALUE: u8 = 0x02;
}
