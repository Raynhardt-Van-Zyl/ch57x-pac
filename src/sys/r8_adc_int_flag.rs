#[doc = "Register `R8_ADC_INT_FLAG` reader"]
pub type R = crate::R<R8AdcIntFlagSpec>;
#[doc = "Field `RB_ADC_IF_EOC` reader - RO, ADC conversion interrupt flag: 0=free or converting, 1=end of conversion, interrupt action, write R8_ADC_CONVERT to clear flag"]
pub type RbAdcIfEocR = crate::BitReader;
impl R {
    #[doc = "Bit 7 - RO, ADC conversion interrupt flag: 0=free or converting, 1=end of conversion, interrupt action, write R8_ADC_CONVERT to clear flag"]
    #[inline(always)]
    pub fn rb_adc_if_eoc(&self) -> RbAdcIfEocR {
        RbAdcIfEocR::new(((self.bits >> 7) & 1) != 0)
    }
}
#[doc = "RO, ADC interrupt flag register\n\nYou can [`read`](crate::Reg::read) this register and get [`r8_adc_int_flag::R`](R). See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct R8AdcIntFlagSpec;
impl crate::RegisterSpec for R8AdcIntFlagSpec {
    type Ux = u8;
}
#[doc = "`read()` method returns [`r8_adc_int_flag::R`](R) reader structure"]
impl crate::Readable for R8AdcIntFlagSpec {}
#[doc = "`reset()` method sets R8_ADC_INT_FLAG to value 0"]
impl crate::Resettable for R8AdcIntFlagSpec {}
