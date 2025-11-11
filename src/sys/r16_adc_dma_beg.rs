#[doc = "Register `R16_ADC_DMA_BEG` reader"]
pub type R = crate::R<R16AdcDmaBegSpec>;
#[doc = "Register `R16_ADC_DMA_BEG` writer"]
pub type W = crate::W<R16AdcDmaBegSpec>;
#[doc = "Field `R16_ADC_DMA_BEG` reader - ADC DMA begin address"]
pub type R16AdcDmaBegR = crate::FieldReader<u16>;
#[doc = "Field `R16_ADC_DMA_BEG` writer - ADC DMA begin address"]
pub type R16AdcDmaBegW<'a, REG> = crate::FieldWriter<'a, REG, 16, u16>;
impl R {
    #[doc = "Bits 0:15 - ADC DMA begin address"]
    #[inline(always)]
    pub fn r16_adc_dma_beg(&self) -> R16AdcDmaBegR {
        R16AdcDmaBegR::new(self.bits)
    }
}
impl W {
    #[doc = "Bits 0:15 - ADC DMA begin address"]
    #[inline(always)]
    pub fn r16_adc_dma_beg(&mut self) -> R16AdcDmaBegW<'_, R16AdcDmaBegSpec> {
        R16AdcDmaBegW::new(self, 0)
    }
}
#[doc = "RW, ADC DMA begin address\n\nYou can [`read`](crate::Reg::read) this register and get [`r16_adc_dma_beg::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`r16_adc_dma_beg::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct R16AdcDmaBegSpec;
impl crate::RegisterSpec for R16AdcDmaBegSpec {
    type Ux = u16;
}
#[doc = "`read()` method returns [`r16_adc_dma_beg::R`](R) reader structure"]
impl crate::Readable for R16AdcDmaBegSpec {}
#[doc = "`write(|w| ..)` method takes [`r16_adc_dma_beg::W`](W) writer structure"]
impl crate::Writable for R16AdcDmaBegSpec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets R16_ADC_DMA_BEG to value 0"]
impl crate::Resettable for R16AdcDmaBegSpec {}
