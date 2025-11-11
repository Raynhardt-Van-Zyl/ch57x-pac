#[doc = "Register `R32_ADC_DMA_CTRL` reader"]
pub type R = crate::R<R32AdcDmaCtrlSpec>;
#[doc = "Register `R32_ADC_DMA_CTRL` writer"]
pub type W = crate::W<R32AdcDmaCtrlSpec>;
#[doc = "Field `R32_ADC_DMA_CTRL` reader - RW, ADC DMA enable"]
pub type R32AdcDmaCtrlR = crate::FieldReader;
#[doc = "Field `R32_ADC_DMA_CTRL` writer - RW, ADC DMA enable"]
pub type R32AdcDmaCtrlW<'a, REG> = crate::FieldWriter<'a, REG, 8>;
impl R {
    #[doc = "Bits 0:7 - RW, ADC DMA enable"]
    #[inline(always)]
    pub fn r32_adc_dma_ctrl(&self) -> R32AdcDmaCtrlR {
        R32AdcDmaCtrlR::new(self.bits)
    }
}
impl W {
    #[doc = "Bits 0:7 - RW, ADC DMA enable"]
    #[inline(always)]
    pub fn r32_adc_dma_ctrl(&mut self) -> R32AdcDmaCtrlW<'_, R32AdcDmaCtrlSpec> {
        R32AdcDmaCtrlW::new(self, 0)
    }
}
#[doc = "RO, ADC DMA control and status register\n\nYou can [`read`](crate::Reg::read) this register and get [`r32_adc_dma_ctrl::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`r32_adc_dma_ctrl::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct R32AdcDmaCtrlSpec;
impl crate::RegisterSpec for R32AdcDmaCtrlSpec {
    type Ux = u8;
}
#[doc = "`read()` method returns [`r32_adc_dma_ctrl::R`](R) reader structure"]
impl crate::Readable for R32AdcDmaCtrlSpec {}
#[doc = "`write(|w| ..)` method takes [`r32_adc_dma_ctrl::W`](W) writer structure"]
impl crate::Writable for R32AdcDmaCtrlSpec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets R32_ADC_DMA_CTRL to value 0"]
impl crate::Resettable for R32AdcDmaCtrlSpec {}
