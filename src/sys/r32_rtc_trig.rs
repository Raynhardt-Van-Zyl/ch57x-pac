#[doc = "Register `R32_RTC_TRIG` reader"]
pub type R = crate::R<R32RtcTrigSpec>;
#[doc = "Register `R32_RTC_TRIG` writer"]
pub type W = crate::W<R32RtcTrigSpec>;
#[doc = "Field `R32_RTC_TRIG` reader - RWA, RTC trigger value"]
pub type R32RtcTrigR = crate::FieldReader<u32>;
#[doc = "Field `R32_RTC_TRIG` writer - RWA, RTC trigger value"]
pub type R32RtcTrigW<'a, REG> = crate::FieldWriter<'a, REG, 32, u32>;
impl R {
    #[doc = "Bits 0:31 - RWA, RTC trigger value"]
    #[inline(always)]
    pub fn r32_rtc_trig(&self) -> R32RtcTrigR {
        R32RtcTrigR::new(self.bits)
    }
}
impl W {
    #[doc = "Bits 0:31 - RWA, RTC trigger value"]
    #[inline(always)]
    pub fn r32_rtc_trig(&mut self) -> R32RtcTrigW<'_, R32RtcTrigSpec> {
        R32RtcTrigW::new(self, 0)
    }
}
#[doc = "RWA, RTC trigger value, SAM\n\nYou can [`read`](crate::Reg::read) this register and get [`r32_rtc_trig::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`r32_rtc_trig::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct R32RtcTrigSpec;
impl crate::RegisterSpec for R32RtcTrigSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`r32_rtc_trig::R`](R) reader structure"]
impl crate::Readable for R32RtcTrigSpec {}
#[doc = "`write(|w| ..)` method takes [`r32_rtc_trig::W`](W) writer structure"]
impl crate::Writable for R32RtcTrigSpec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets R32_RTC_TRIG to value 0"]
impl crate::Resettable for R32RtcTrigSpec {}
