#[doc = "Register `R8_PWM8_DATA` reader"]
pub type R = crate::R<R8Pwm8DataSpec>;
#[doc = "Register `R8_PWM8_DATA` writer"]
pub type W = crate::W<R8Pwm8DataSpec>;
#[doc = "Field `R8_PWM8_DATA` reader - RW, PWM8 data holding"]
pub type R8Pwm8DataR = crate::FieldReader;
#[doc = "Field `R8_PWM8_DATA` writer - RW, PWM8 data holding"]
pub type R8Pwm8DataW<'a, REG> = crate::FieldWriter<'a, REG, 8>;
impl R {
    #[doc = "Bits 0:7 - RW, PWM8 data holding"]
    #[inline(always)]
    pub fn r8_pwm8_data(&self) -> R8Pwm8DataR {
        R8Pwm8DataR::new(self.bits)
    }
}
impl W {
    #[doc = "Bits 0:7 - RW, PWM8 data holding"]
    #[inline(always)]
    pub fn r8_pwm8_data(&mut self) -> R8Pwm8DataW<'_, R8Pwm8DataSpec> {
        R8Pwm8DataW::new(self, 0)
    }
}
#[doc = "RW, PWM8 data holding\n\nYou can [`read`](crate::Reg::read) this register and get [`r8_pwm8_data::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`r8_pwm8_data::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct R8Pwm8DataSpec;
impl crate::RegisterSpec for R8Pwm8DataSpec {
    type Ux = u8;
}
#[doc = "`read()` method returns [`r8_pwm8_data::R`](R) reader structure"]
impl crate::Readable for R8Pwm8DataSpec {}
#[doc = "`write(|w| ..)` method takes [`r8_pwm8_data::W`](W) writer structure"]
impl crate::Writable for R8Pwm8DataSpec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets R8_PWM8_DATA to value 0"]
impl crate::Resettable for R8Pwm8DataSpec {}
