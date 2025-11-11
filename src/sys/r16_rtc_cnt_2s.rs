#[doc = "Register `R16_RTC_CNT_2S` reader"]
pub type R = crate::R<R16RtcCnt2sSpec>;
#[doc = "Field `R16_RTC_CNT_2S` reader - RO, RTC count based 2 second"]
pub type R16RtcCnt2sR = crate::FieldReader<u16>;
impl R {
    #[doc = "Bits 0:15 - RO, RTC count based 2 second"]
    #[inline(always)]
    pub fn r16_rtc_cnt_2s(&self) -> R16RtcCnt2sR {
        R16RtcCnt2sR::new(self.bits)
    }
}
#[doc = "RO, RTC count based 2 second\n\nYou can [`read`](crate::Reg::read) this register and get [`r16_rtc_cnt_2s::R`](R). See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct R16RtcCnt2sSpec;
impl crate::RegisterSpec for R16RtcCnt2sSpec {
    type Ux = u16;
}
#[doc = "`read()` method returns [`r16_rtc_cnt_2s::R`](R) reader structure"]
impl crate::Readable for R16RtcCnt2sSpec {}
#[doc = "`reset()` method sets R16_RTC_CNT_2S to value 0"]
impl crate::Resettable for R16RtcCnt2sSpec {}
