#[doc = "Register `R16_RTC_CNT_32K` reader"]
pub type R = crate::R<R16RtcCnt32kSpec>;
#[doc = "Field `R16_RTC_CNT_32K` reader - RWA,RTC count based 32KHz"]
pub type R16RtcCnt32kR = crate::FieldReader<u16>;
impl R {
    #[doc = "Bits 0:15 - RWA,RTC count based 32KHz"]
    #[inline(always)]
    pub fn r16_rtc_cnt_32k(&self) -> R16RtcCnt32kR {
        R16RtcCnt32kR::new(self.bits)
    }
}
#[doc = "RO, RTC count based 32KHz\n\nYou can [`read`](crate::Reg::read) this register and get [`r16_rtc_cnt_32k::R`](R). See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct R16RtcCnt32kSpec;
impl crate::RegisterSpec for R16RtcCnt32kSpec {
    type Ux = u16;
}
#[doc = "`read()` method returns [`r16_rtc_cnt_32k::R`](R) reader structure"]
impl crate::Readable for R16RtcCnt32kSpec {}
#[doc = "`reset()` method sets R16_RTC_CNT_32K to value 0"]
impl crate::Resettable for R16RtcCnt32kSpec {}
