#[doc = "Register `R16_PA_INT_EN` reader"]
pub type R = crate::R<R16PaIntEnSpec>;
#[doc = "Register `R16_PA_INT_EN` writer"]
pub type W = crate::W<R16PaIntEnSpec>;
#[doc = "Field `R16_PA_INT_EN` reader - GPIO PA interrupt enable"]
pub type R16PaIntEnR = crate::FieldReader<u16>;
#[doc = "Field `R16_PA_INT_EN` writer - GPIO PA interrupt enable"]
pub type R16PaIntEnW<'a, REG> = crate::FieldWriter<'a, REG, 16, u16>;
impl R {
    #[doc = "Bits 0:15 - GPIO PA interrupt enable"]
    #[inline(always)]
    pub fn r16_pa_int_en(&self) -> R16PaIntEnR {
        R16PaIntEnR::new(self.bits)
    }
}
impl W {
    #[doc = "Bits 0:15 - GPIO PA interrupt enable"]
    #[inline(always)]
    pub fn r16_pa_int_en(&mut self) -> R16PaIntEnW<'_, R16PaIntEnSpec> {
        R16PaIntEnW::new(self, 0)
    }
}
#[doc = "RW, GPIO PA interrupt enable\n\nYou can [`read`](crate::Reg::read) this register and get [`r16_pa_int_en::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`r16_pa_int_en::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct R16PaIntEnSpec;
impl crate::RegisterSpec for R16PaIntEnSpec {
    type Ux = u16;
}
#[doc = "`read()` method returns [`r16_pa_int_en::R`](R) reader structure"]
impl crate::Readable for R16PaIntEnSpec {}
#[doc = "`write(|w| ..)` method takes [`r16_pa_int_en::W`](W) writer structure"]
impl crate::Writable for R16PaIntEnSpec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets R16_PA_INT_EN to value 0"]
impl crate::Resettable for R16PaIntEnSpec {}
