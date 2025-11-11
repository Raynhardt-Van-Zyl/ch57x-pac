#[doc = "Register `R8_PWM11_DATA` reader"]
pub type R = crate::R<R8Pwm11DataSpec>;
#[doc = "Register `R8_PWM11_DATA` writer"]
pub type W = crate::W<R8Pwm11DataSpec>;
#[doc = "Field `R8_PWM11_DATA` reader - RW, PWM11 data holding"]
pub type R8Pwm11DataR = crate::FieldReader;
#[doc = "Field `R8_PWM11_DATA` writer - RW, PWM11 data holding"]
pub type R8Pwm11DataW<'a, REG> = crate::FieldWriter<'a, REG, 8>;
impl R {
    #[doc = "Bits 0:7 - RW, PWM11 data holding"]
    #[inline(always)]
    pub fn r8_pwm11_data(&self) -> R8Pwm11DataR {
        R8Pwm11DataR::new(self.bits)
    }
}
impl W {
    #[doc = "Bits 0:7 - RW, PWM11 data holding"]
    #[inline(always)]
    pub fn r8_pwm11_data(&mut self) -> R8Pwm11DataW<'_, R8Pwm11DataSpec> {
        R8Pwm11DataW::new(self, 0)
    }
}
#[doc = "RW, PWM11 data holding\n\nYou can [`read`](crate::Reg::read) this register and get [`r8_pwm11_data::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`r8_pwm11_data::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct R8Pwm11DataSpec;
impl crate::RegisterSpec for R8Pwm11DataSpec {
    type Ux = u8;
}
#[doc = "`read()` method returns [`r8_pwm11_data::R`](R) reader structure"]
impl crate::Readable for R8Pwm11DataSpec {}
#[doc = "`write(|w| ..)` method takes [`r8_pwm11_data::W`](W) writer structure"]
impl crate::Writable for R8Pwm11DataSpec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets R8_PWM11_DATA to value 0"]
impl crate::Resettable for R8Pwm11DataSpec {}
