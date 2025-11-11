#[doc = "Register `R32_RTC_CNT_DAY` reader"]
pub type R = crate::R<R32RtcCntDaySpec>;
#[doc = "Field `R32_RTC_CNT_DAY` reader - RWA,RTC count based one day"]
pub type R32RtcCntDayR = crate::FieldReader<u16>;
impl R {
    #[doc = "Bits 0:13 - RWA,RTC count based one day"]
    #[inline(always)]
    pub fn r32_rtc_cnt_day(&self) -> R32RtcCntDayR {
        R32RtcCntDayR::new((self.bits & 0x3fff) as u16)
    }
}
#[doc = "RO, RTC count based one day, only low 14 bit\n\nYou can [`read`](crate::Reg::read) this register and get [`r32_rtc_cnt_day::R`](R). See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct R32RtcCntDaySpec;
impl crate::RegisterSpec for R32RtcCntDaySpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`r32_rtc_cnt_day::R`](R) reader structure"]
impl crate::Readable for R32RtcCntDaySpec {}
#[doc = "`reset()` method sets R32_RTC_CNT_DAY to value 0"]
impl crate::Resettable for R32RtcCntDaySpec {}
