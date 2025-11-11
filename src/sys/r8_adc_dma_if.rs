#[doc = "Register `R8_ADC_DMA_IF` reader"]
pub type R = crate::R<R8AdcDmaIfSpec>;
#[doc = "Register `R8_ADC_DMA_IF` writer"]
pub type W = crate::W<R8AdcDmaIfSpec>;
#[doc = "Field `RB_ADC_IF_DMA_END` reader - interrupt flag for ADC DMA completion"]
pub type RbAdcIfDmaEndR = crate::BitReader;
#[doc = "Field `RB_ADC_IF_DMA_END` writer - interrupt flag for ADC DMA completion"]
pub type RbAdcIfDmaEndW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `RB_ADC_IF_END_ADC` reader - interrupt flag for end of ADC conversion"]
pub type RbAdcIfEndAdcR = crate::BitReader;
#[doc = "Field `RB_ADC_IF_END_ADC` writer - interrupt flag for end of ADC conversion"]
pub type RbAdcIfEndAdcW<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bit 3 - interrupt flag for ADC DMA completion"]
    #[inline(always)]
    pub fn rb_adc_if_dma_end(&self) -> RbAdcIfDmaEndR {
        RbAdcIfDmaEndR::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bit 4 - interrupt flag for end of ADC conversion"]
    #[inline(always)]
    pub fn rb_adc_if_end_adc(&self) -> RbAdcIfEndAdcR {
        RbAdcIfEndAdcR::new(((self.bits >> 4) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 3 - interrupt flag for ADC DMA completion"]
    #[inline(always)]
    pub fn rb_adc_if_dma_end(&mut self) -> RbAdcIfDmaEndW<'_, R8AdcDmaIfSpec> {
        RbAdcIfDmaEndW::new(self, 3)
    }
    #[doc = "Bit 4 - interrupt flag for end of ADC conversion"]
    #[inline(always)]
    pub fn rb_adc_if_end_adc(&mut self) -> RbAdcIfEndAdcW<'_, R8AdcDmaIfSpec> {
        RbAdcIfEndAdcW::new(self, 4)
    }
}
#[doc = "RO, ADC interrupt flag\n\nYou can [`read`](crate::Reg::read) this register and get [`r8_adc_dma_if::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`r8_adc_dma_if::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct R8AdcDmaIfSpec;
impl crate::RegisterSpec for R8AdcDmaIfSpec {
    type Ux = u8;
}
#[doc = "`read()` method returns [`r8_adc_dma_if::R`](R) reader structure"]
impl crate::Readable for R8AdcDmaIfSpec {}
#[doc = "`write(|w| ..)` method takes [`r8_adc_dma_if::W`](W) writer structure"]
impl crate::Writable for R8AdcDmaIfSpec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets R8_ADC_DMA_IF to value 0"]
impl crate::Resettable for R8AdcDmaIfSpec {}
