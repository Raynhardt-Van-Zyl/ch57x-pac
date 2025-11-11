#[doc = "Register `R8_PWM6_DATA` reader"]
pub type R = crate::R<R8Pwm6DataSpec>;
#[doc = "Register `R8_PWM6_DATA` writer"]
pub type W = crate::W<R8Pwm6DataSpec>;
#[doc = "Field `R8_PWM6_DATA` reader - RW, PWM6 data holding"]
pub type R8Pwm6DataR = crate::FieldReader;
#[doc = "Field `R8_PWM6_DATA` writer - RW, PWM6 data holding"]
pub type R8Pwm6DataW<'a, REG> = crate::FieldWriter<'a, REG, 8>;
impl R {
    #[doc = "Bits 0:7 - RW, PWM6 data holding"]
    #[inline(always)]
    pub fn r8_pwm6_data(&self) -> R8Pwm6DataR {
        R8Pwm6DataR::new(self.bits)
    }
}
impl W {
    #[doc = "Bits 0:7 - RW, PWM6 data holding"]
    #[inline(always)]
    pub fn r8_pwm6_data(&mut self) -> R8Pwm6DataW<'_, R8Pwm6DataSpec> {
        R8Pwm6DataW::new(self, 0)
    }
}
#[doc = "RW, PWM6 data holding\n\nYou can [`read`](crate::Reg::read) this register and get [`r8_pwm6_data::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`r8_pwm6_data::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct R8Pwm6DataSpec;
impl crate::RegisterSpec for R8Pwm6DataSpec {
    type Ux = u8;
}
#[doc = "`read()` method returns [`r8_pwm6_data::R`](R) reader structure"]
impl crate::Readable for R8Pwm6DataSpec {}
#[doc = "`write(|w| ..)` method takes [`r8_pwm6_data::W`](W) writer structure"]
impl crate::Writable for R8Pwm6DataSpec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets R8_PWM6_DATA to value 0"]
impl crate::Resettable for R8Pwm6DataSpec {}
