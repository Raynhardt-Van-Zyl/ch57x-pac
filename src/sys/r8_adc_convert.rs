#[doc = "Register `R8_ADC_CONVERT` reader"]
pub type R = crate::R<R8AdcConvertSpec>;
#[doc = "Register `R8_ADC_CONVERT` writer"]
pub type W = crate::W<R8AdcConvertSpec>;
#[doc = "Field `RB_ADC_START` reader - RW, ADC convert start control: 0=stop ADC convert, 1=start an ADC convert, auto clear"]
pub type RbAdcStartR = crate::BitReader;
#[doc = "Field `RB_ADC_START` writer - RW, ADC convert start control: 0=stop ADC convert, 1=start an ADC convert, auto clear"]
pub type RbAdcStartW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `RB_ADC_EOC_X` reader - RO, end of ADC conversion flag"]
pub type RbAdcEocXR = crate::BitReader;
impl R {
    #[doc = "Bit 0 - RW, ADC convert start control: 0=stop ADC convert, 1=start an ADC convert, auto clear"]
    #[inline(always)]
    pub fn rb_adc_start(&self) -> RbAdcStartR {
        RbAdcStartR::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 7 - RO, end of ADC conversion flag"]
    #[inline(always)]
    pub fn rb_adc_eoc_x(&self) -> RbAdcEocXR {
        RbAdcEocXR::new(((self.bits >> 7) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - RW, ADC convert start control: 0=stop ADC convert, 1=start an ADC convert, auto clear"]
    #[inline(always)]
    pub fn rb_adc_start(&mut self) -> RbAdcStartW<'_, R8AdcConvertSpec> {
        RbAdcStartW::new(self, 0)
    }
}
#[doc = "RW, ADC convert control\n\nYou can [`read`](crate::Reg::read) this register and get [`r8_adc_convert::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`r8_adc_convert::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct R8AdcConvertSpec;
impl crate::RegisterSpec for R8AdcConvertSpec {
    type Ux = u8;
}
#[doc = "`read()` method returns [`r8_adc_convert::R`](R) reader structure"]
impl crate::Readable for R8AdcConvertSpec {}
#[doc = "`write(|w| ..)` method takes [`r8_adc_convert::W`](W) writer structure"]
impl crate::Writable for R8AdcConvertSpec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets R8_ADC_CONVERT to value 0"]
impl crate::Resettable for R8AdcConvertSpec {}
