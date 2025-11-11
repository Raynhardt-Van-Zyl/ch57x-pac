#[doc = "Register `R16_ADC_DMA_NOW` reader"]
pub type R = crate::R<R16AdcDmaNowSpec>;
#[doc = "Field `R16_ADC_DMA_NOW` reader - ADC DMA current address"]
pub type R16AdcDmaNowR = crate::FieldReader<u16>;
impl R {
    #[doc = "Bits 0:15 - ADC DMA current address"]
    #[inline(always)]
    pub fn r16_adc_dma_now(&self) -> R16AdcDmaNowR {
        R16AdcDmaNowR::new(self.bits)
    }
}
#[doc = "RO, ADC DMA current address\n\nYou can [`read`](crate::Reg::read) this register and get [`r16_adc_dma_now::R`](R). See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct R16AdcDmaNowSpec;
impl crate::RegisterSpec for R16AdcDmaNowSpec {
    type Ux = u16;
}
#[doc = "`read()` method returns [`r16_adc_dma_now::R`](R) reader structure"]
impl crate::Readable for R16AdcDmaNowSpec {}
#[doc = "`reset()` method sets R16_ADC_DMA_NOW to value 0"]
impl crate::Resettable for R16AdcDmaNowSpec {}
