#[doc = "Register `R8_ADC_AUTO_CYCLE` reader"]
pub type R = crate::R<R8AdcAutoCycleSpec>;
#[doc = "Register `R8_ADC_AUTO_CYCLE` writer"]
pub type W = crate::W<R8AdcAutoCycleSpec>;
#[doc = "Field `R8_ADC_AUTO_CYCLE` reader - auto ADC cycle value, unit is 16 Fsys"]
pub type R8AdcAutoCycleR = crate::FieldReader;
#[doc = "Field `R8_ADC_AUTO_CYCLE` writer - auto ADC cycle value, unit is 16 Fsys"]
pub type R8AdcAutoCycleW<'a, REG> = crate::FieldWriter<'a, REG, 8>;
impl R {
    #[doc = "Bits 0:7 - auto ADC cycle value, unit is 16 Fsys"]
    #[inline(always)]
    pub fn r8_adc_auto_cycle(&self) -> R8AdcAutoCycleR {
        R8AdcAutoCycleR::new(self.bits)
    }
}
impl W {
    #[doc = "Bits 0:7 - auto ADC cycle value, unit is 16 Fsys"]
    #[inline(always)]
    pub fn r8_adc_auto_cycle(&mut self) -> R8AdcAutoCycleW<'_, R8AdcAutoCycleSpec> {
        R8AdcAutoCycleW::new(self, 0)
    }
}
#[doc = "RO, ADC interrupt flag\n\nYou can [`read`](crate::Reg::read) this register and get [`r8_adc_auto_cycle::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`r8_adc_auto_cycle::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct R8AdcAutoCycleSpec;
impl crate::RegisterSpec for R8AdcAutoCycleSpec {
    type Ux = u8;
}
#[doc = "`read()` method returns [`r8_adc_auto_cycle::R`](R) reader structure"]
impl crate::Readable for R8AdcAutoCycleSpec {}
#[doc = "`write(|w| ..)` method takes [`r8_adc_auto_cycle::W`](W) writer structure"]
impl crate::Writable for R8AdcAutoCycleSpec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets R8_ADC_AUTO_CYCLE to value 0"]
impl crate::Resettable for R8AdcAutoCycleSpec {}
