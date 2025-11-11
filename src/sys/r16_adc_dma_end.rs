#[doc = "Register `R16_ADC_DMA_END` reader"]
pub type R = crate::R<R16AdcDmaEndSpec>;
#[doc = "Register `R16_ADC_DMA_END` writer"]
pub type W = crate::W<R16AdcDmaEndSpec>;
#[doc = "Field `R16_ADC_DMA_END` reader - ADC DMA end address"]
pub type R16AdcDmaEndR = crate::FieldReader<u16>;
#[doc = "Field `R16_ADC_DMA_END` writer - ADC DMA end address"]
pub type R16AdcDmaEndW<'a, REG> = crate::FieldWriter<'a, REG, 16, u16>;
impl R {
    #[doc = "Bits 0:15 - ADC DMA end address"]
    #[inline(always)]
    pub fn r16_adc_dma_end(&self) -> R16AdcDmaEndR {
        R16AdcDmaEndR::new(self.bits)
    }
}
impl W {
    #[doc = "Bits 0:15 - ADC DMA end address"]
    #[inline(always)]
    pub fn r16_adc_dma_end(&mut self) -> R16AdcDmaEndW<'_, R16AdcDmaEndSpec> {
        R16AdcDmaEndW::new(self, 0)
    }
}
#[doc = "RW, ADC DMA end address\n\nYou can [`read`](crate::Reg::read) this register and get [`r16_adc_dma_end::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`r16_adc_dma_end::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct R16AdcDmaEndSpec;
impl crate::RegisterSpec for R16AdcDmaEndSpec {
    type Ux = u16;
}
#[doc = "`read()` method returns [`r16_adc_dma_end::R`](R) reader structure"]
impl crate::Readable for R16AdcDmaEndSpec {}
#[doc = "`write(|w| ..)` method takes [`r16_adc_dma_end::W`](W) writer structure"]
impl crate::Writable for R16AdcDmaEndSpec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets R16_ADC_DMA_END to value 0"]
impl crate::Resettable for R16AdcDmaEndSpec {}
