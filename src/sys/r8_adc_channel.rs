#[doc = "Register `R8_ADC_CHANNEL` reader"]
pub type R = crate::R<R8AdcChannelSpec>;
#[doc = "Register `R8_ADC_CHANNEL` writer"]
pub type W = crate::W<R8AdcChannelSpec>;
#[doc = "Field `RB_ADC_CH_INX` reader - RW, ADC input channel index"]
pub type RbAdcChInxR = crate::FieldReader;
#[doc = "Field `RB_ADC_CH_INX` writer - RW, ADC input channel index"]
pub type RbAdcChInxW<'a, REG> = crate::FieldWriter<'a, REG, 4>;
impl R {
    #[doc = "Bits 0:3 - RW, ADC input channel index"]
    #[inline(always)]
    pub fn rb_adc_ch_inx(&self) -> RbAdcChInxR {
        RbAdcChInxR::new(self.bits & 0x0f)
    }
}
impl W {
    #[doc = "Bits 0:3 - RW, ADC input channel index"]
    #[inline(always)]
    pub fn rb_adc_ch_inx(&mut self) -> RbAdcChInxW<'_, R8AdcChannelSpec> {
        RbAdcChInxW::new(self, 0)
    }
}
#[doc = "RW, ADC input channel selection\n\nYou can [`read`](crate::Reg::read) this register and get [`r8_adc_channel::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`r8_adc_channel::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct R8AdcChannelSpec;
impl crate::RegisterSpec for R8AdcChannelSpec {
    type Ux = u8;
}
#[doc = "`read()` method returns [`r8_adc_channel::R`](R) reader structure"]
impl crate::Readable for R8AdcChannelSpec {}
#[doc = "`write(|w| ..)` method takes [`r8_adc_channel::W`](W) writer structure"]
impl crate::Writable for R8AdcChannelSpec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets R8_ADC_CHANNEL to value 0x0f"]
impl crate::Resettable for R8AdcChannelSpec {
    const RESET_VALUE: u8 = 0x0f;
}
