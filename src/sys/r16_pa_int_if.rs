#[doc = "Register `R16_PA_INT_IF` reader"]
pub type R = crate::R<R16PaIntIfSpec>;
#[doc = "Register `R16_PA_INT_IF` writer"]
pub type W = crate::W<R16PaIntIfSpec>;
#[doc = "Field `R16_PA_INT_IF` reader - GPIO PA interrupt flag"]
pub type R16PaIntIfR = crate::FieldReader<u16>;
#[doc = "Field `R16_PA_INT_IF` writer - GPIO PA interrupt flag"]
pub type R16PaIntIfW<'a, REG> = crate::FieldWriter<'a, REG, 16, u16>;
impl R {
    #[doc = "Bits 0:15 - GPIO PA interrupt flag"]
    #[inline(always)]
    pub fn r16_pa_int_if(&self) -> R16PaIntIfR {
        R16PaIntIfR::new(self.bits)
    }
}
impl W {
    #[doc = "Bits 0:15 - GPIO PA interrupt flag"]
    #[inline(always)]
    pub fn r16_pa_int_if(&mut self) -> R16PaIntIfW<'_, R16PaIntIfSpec> {
        R16PaIntIfW::new(self, 0)
    }
}
#[doc = "RW1, GPIO PA interrupt flag\n\nYou can [`read`](crate::Reg::read) this register and get [`r16_pa_int_if::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`r16_pa_int_if::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct R16PaIntIfSpec;
impl crate::RegisterSpec for R16PaIntIfSpec {
    type Ux = u16;
}
#[doc = "`read()` method returns [`r16_pa_int_if::R`](R) reader structure"]
impl crate::Readable for R16PaIntIfSpec {}
#[doc = "`write(|w| ..)` method takes [`r16_pa_int_if::W`](W) writer structure"]
impl crate::Writable for R16PaIntIfSpec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets R16_PA_INT_IF to value 0"]
impl crate::Resettable for R16PaIntIfSpec {}
