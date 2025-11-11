#[doc = "Register `R16_ADC_DATA` reader"]
pub type R = crate::R<R16AdcDataSpec>;
#[doc = "Field `RB_ADC_DATA` reader - RO, ADC conversion data"]
pub type RbAdcDataR = crate::FieldReader<u16>;
impl R {
    #[doc = "Bits 0:11 - RO, ADC conversion data"]
    #[inline(always)]
    pub fn rb_adc_data(&self) -> RbAdcDataR {
        RbAdcDataR::new(self.bits & 0x0fff)
    }
}
#[doc = "RO, ADC data\n\nYou can [`read`](crate::Reg::read) this register and get [`r16_adc_data::R`](R). See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct R16AdcDataSpec;
impl crate::RegisterSpec for R16AdcDataSpec {
    type Ux = u16;
}
#[doc = "`read()` method returns [`r16_adc_data::R`](R) reader structure"]
impl crate::Readable for R16AdcDataSpec {}
#[doc = "`reset()` method sets R16_ADC_DATA to value 0"]
impl crate::Resettable for R16AdcDataSpec {}
