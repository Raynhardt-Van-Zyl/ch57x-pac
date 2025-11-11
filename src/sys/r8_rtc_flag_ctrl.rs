#[doc = "Register `R8_RTC_FLAG_CTRL` reader"]
pub type R = crate::R<R8RtcFlagCtrlSpec>;
#[doc = "Register `R8_RTC_FLAG_CTRL` writer"]
pub type W = crate::W<R8RtcFlagCtrlSpec>;
#[doc = "Field `RB_RTC_TMR_CLR` reader - RW, set 1 to clear RTC timer action flag, auto clear"]
pub type RbRtcTmrClrR = crate::BitReader;
#[doc = "Field `RB_RTC_TMR_CLR` writer - RW, set 1 to clear RTC timer action flag, auto clear"]
pub type RbRtcTmrClrW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `RB_RTC_TRIG_CLR` reader - RW, set 1 to clear RTC trigger action flag, auto clear"]
pub type RbRtcTrigClrR = crate::BitReader;
#[doc = "Field `RB_RTC_TRIG_CLR` writer - RW, set 1 to clear RTC trigger action flag, auto clear"]
pub type RbRtcTrigClrW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `RB_RTC_TMR_FLAG` reader - RO, RTC timer action flag"]
pub type RbRtcTmrFlagR = crate::BitReader;
#[doc = "Field `RB_RTC_TRIG_FLAG` reader - RO, RTC trigger action flag"]
pub type RbRtcTrigFlagR = crate::BitReader;
impl R {
    #[doc = "Bit 4 - RW, set 1 to clear RTC timer action flag, auto clear"]
    #[inline(always)]
    pub fn rb_rtc_tmr_clr(&self) -> RbRtcTmrClrR {
        RbRtcTmrClrR::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bit 5 - RW, set 1 to clear RTC trigger action flag, auto clear"]
    #[inline(always)]
    pub fn rb_rtc_trig_clr(&self) -> RbRtcTrigClrR {
        RbRtcTrigClrR::new(((self.bits >> 5) & 1) != 0)
    }
    #[doc = "Bit 6 - RO, RTC timer action flag"]
    #[inline(always)]
    pub fn rb_rtc_tmr_flag(&self) -> RbRtcTmrFlagR {
        RbRtcTmrFlagR::new(((self.bits >> 6) & 1) != 0)
    }
    #[doc = "Bit 7 - RO, RTC trigger action flag"]
    #[inline(always)]
    pub fn rb_rtc_trig_flag(&self) -> RbRtcTrigFlagR {
        RbRtcTrigFlagR::new(((self.bits >> 7) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 4 - RW, set 1 to clear RTC timer action flag, auto clear"]
    #[inline(always)]
    pub fn rb_rtc_tmr_clr(&mut self) -> RbRtcTmrClrW<'_, R8RtcFlagCtrlSpec> {
        RbRtcTmrClrW::new(self, 4)
    }
    #[doc = "Bit 5 - RW, set 1 to clear RTC trigger action flag, auto clear"]
    #[inline(always)]
    pub fn rb_rtc_trig_clr(&mut self) -> RbRtcTrigClrW<'_, R8RtcFlagCtrlSpec> {
        RbRtcTrigClrW::new(self, 5)
    }
}
#[doc = "RW, RTC flag and clear control\n\nYou can [`read`](crate::Reg::read) this register and get [`r8_rtc_flag_ctrl::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`r8_rtc_flag_ctrl::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct R8RtcFlagCtrlSpec;
impl crate::RegisterSpec for R8RtcFlagCtrlSpec {
    type Ux = u8;
}
#[doc = "`read()` method returns [`r8_rtc_flag_ctrl::R`](R) reader structure"]
impl crate::Readable for R8RtcFlagCtrlSpec {}
#[doc = "`write(|w| ..)` method takes [`r8_rtc_flag_ctrl::W`](W) writer structure"]
impl crate::Writable for R8RtcFlagCtrlSpec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets R8_RTC_FLAG_CTRL to value 0x30"]
impl crate::Resettable for R8RtcFlagCtrlSpec {
    const RESET_VALUE: u8 = 0x30;
}
