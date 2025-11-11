#[doc = "Register `R8_ADC_CTRL_DMA` reader"]
pub type R = crate::R<R8AdcCtrlDmaSpec>;
#[doc = "Register `R8_ADC_CTRL_DMA` writer"]
pub type W = crate::W<R8AdcCtrlDmaSpec>;
#[doc = "Field `RB_ADC_DMA_ENABLE` reader - RW, ADC DMA enable"]
pub type RbAdcDmaEnableR = crate::BitReader;
#[doc = "Field `RB_ADC_DMA_ENABLE` writer - RW, ADC DMA enable"]
pub type RbAdcDmaEnableW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `RB_ADC_DMA_LOOP` reader - RW, ADC DMA address loop enable"]
pub type RbAdcDmaLoopR = crate::BitReader;
#[doc = "Field `RB_ADC_DMA_LOOP` writer - RW, ADC DMA address loop enable"]
pub type RbAdcDmaLoopW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `RB_ADC_IE_DMA_END` reader - RW, enable interrupt for ADC DMA completion"]
pub type RbAdcIeDmaEndR = crate::BitReader;
#[doc = "Field `RB_ADC_IE_DMA_END` writer - RW, enable interrupt for ADC DMA completion"]
pub type RbAdcIeDmaEndW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `RB_ADC_IE_EOC` reader - RW, enable interrupt for end of ADC conversion"]
pub type RbAdcIeEocR = crate::BitReader;
#[doc = "Field `RB_ADC_IE_EOC` writer - RW, enable interrupt for end of ADC conversion"]
pub type RbAdcIeEocW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `RB_ADC_AUTO_EN` reader - RW, enable auto continuing ADC for DMA"]
pub type RbAdcAutoEnR = crate::BitReader;
#[doc = "Field `RB_ADC_AUTO_EN` writer - RW, enable auto continuing ADC for DMA"]
pub type RbAdcAutoEnW<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bit 0 - RW, ADC DMA enable"]
    #[inline(always)]
    pub fn rb_adc_dma_enable(&self) -> RbAdcDmaEnableR {
        RbAdcDmaEnableR::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 2 - RW, ADC DMA address loop enable"]
    #[inline(always)]
    pub fn rb_adc_dma_loop(&self) -> RbAdcDmaLoopR {
        RbAdcDmaLoopR::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3 - RW, enable interrupt for ADC DMA completion"]
    #[inline(always)]
    pub fn rb_adc_ie_dma_end(&self) -> RbAdcIeDmaEndR {
        RbAdcIeDmaEndR::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bit 4 - RW, enable interrupt for end of ADC conversion"]
    #[inline(always)]
    pub fn rb_adc_ie_eoc(&self) -> RbAdcIeEocR {
        RbAdcIeEocR::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bit 7 - RW, enable auto continuing ADC for DMA"]
    #[inline(always)]
    pub fn rb_adc_auto_en(&self) -> RbAdcAutoEnR {
        RbAdcAutoEnR::new(((self.bits >> 7) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - RW, ADC DMA enable"]
    #[inline(always)]
    pub fn rb_adc_dma_enable(&mut self) -> RbAdcDmaEnableW<'_, R8AdcCtrlDmaSpec> {
        RbAdcDmaEnableW::new(self, 0)
    }
    #[doc = "Bit 2 - RW, ADC DMA address loop enable"]
    #[inline(always)]
    pub fn rb_adc_dma_loop(&mut self) -> RbAdcDmaLoopW<'_, R8AdcCtrlDmaSpec> {
        RbAdcDmaLoopW::new(self, 2)
    }
    #[doc = "Bit 3 - RW, enable interrupt for ADC DMA completion"]
    #[inline(always)]
    pub fn rb_adc_ie_dma_end(&mut self) -> RbAdcIeDmaEndW<'_, R8AdcCtrlDmaSpec> {
        RbAdcIeDmaEndW::new(self, 3)
    }
    #[doc = "Bit 4 - RW, enable interrupt for end of ADC conversion"]
    #[inline(always)]
    pub fn rb_adc_ie_eoc(&mut self) -> RbAdcIeEocW<'_, R8AdcCtrlDmaSpec> {
        RbAdcIeEocW::new(self, 4)
    }
    #[doc = "Bit 7 - RW, enable auto continuing ADC for DMA"]
    #[inline(always)]
    pub fn rb_adc_auto_en(&mut self) -> RbAdcAutoEnW<'_, R8AdcCtrlDmaSpec> {
        RbAdcAutoEnW::new(self, 7)
    }
}
#[doc = "RW, ADC DMA control\n\nYou can [`read`](crate::Reg::read) this register and get [`r8_adc_ctrl_dma::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`r8_adc_ctrl_dma::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct R8AdcCtrlDmaSpec;
impl crate::RegisterSpec for R8AdcCtrlDmaSpec {
    type Ux = u8;
}
#[doc = "`read()` method returns [`r8_adc_ctrl_dma::R`](R) reader structure"]
impl crate::Readable for R8AdcCtrlDmaSpec {}
#[doc = "`write(|w| ..)` method takes [`r8_adc_ctrl_dma::W`](W) writer structure"]
impl crate::Writable for R8AdcCtrlDmaSpec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets R8_ADC_CTRL_DMA to value 0"]
impl crate::Resettable for R8AdcCtrlDmaSpec {}
