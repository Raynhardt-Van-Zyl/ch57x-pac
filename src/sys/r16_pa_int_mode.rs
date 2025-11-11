#[doc = "Register `R16_PA_INT_MODE` reader"]
pub type R = crate::R<R16PaIntModeSpec>;
#[doc = "Register `R16_PA_INT_MODE` writer"]
pub type W = crate::W<R16PaIntModeSpec>;
#[doc = "Field `R16_PA_INT_MODE` reader - GPIO PA interrupt mode"]
pub type R16PaIntModeR = crate::FieldReader<u16>;
#[doc = "Field `R16_PA_INT_MODE` writer - GPIO PA interrupt mode"]
pub type R16PaIntModeW<'a, REG> = crate::FieldWriter<'a, REG, 16, u16>;
impl R {
    #[doc = "Bits 0:15 - GPIO PA interrupt mode"]
    #[inline(always)]
    pub fn r16_pa_int_mode(&self) -> R16PaIntModeR {
        R16PaIntModeR::new(self.bits)
    }
}
impl W {
    #[doc = "Bits 0:15 - GPIO PA interrupt mode"]
    #[inline(always)]
    pub fn r16_pa_int_mode(&mut self) -> R16PaIntModeW<'_, R16PaIntModeSpec> {
        R16PaIntModeW::new(self, 0)
    }
}
#[doc = "RW, GPIO PA interrupt mode: 0=level action, 1=edge action\n\nYou can [`read`](crate::Reg::read) this register and get [`r16_pa_int_mode::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`r16_pa_int_mode::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct R16PaIntModeSpec;
impl crate::RegisterSpec for R16PaIntModeSpec {
    type Ux = u16;
}
#[doc = "`read()` method returns [`r16_pa_int_mode::R`](R) reader structure"]
impl crate::Readable for R16PaIntModeSpec {}
#[doc = "`write(|w| ..)` method takes [`r16_pa_int_mode::W`](W) writer structure"]
impl crate::Writable for R16PaIntModeSpec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets R16_PA_INT_MODE to value 0"]
impl crate::Resettable for R16PaIntModeSpec {}
