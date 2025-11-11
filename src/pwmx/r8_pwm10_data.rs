#[doc = "Register `R8_PWM10_DATA` reader"]
pub type R = crate::R<R8Pwm10DataSpec>;
#[doc = "Register `R8_PWM10_DATA` writer"]
pub type W = crate::W<R8Pwm10DataSpec>;
#[doc = "Field `R8_PWM10_DATA` reader - RW, PWM10 data holding"]
pub type R8Pwm10DataR = crate::FieldReader;
#[doc = "Field `R8_PWM10_DATA` writer - RW, PWM10 data holding"]
pub type R8Pwm10DataW<'a, REG> = crate::FieldWriter<'a, REG, 8>;
impl R {
    #[doc = "Bits 0:7 - RW, PWM10 data holding"]
    #[inline(always)]
    pub fn r8_pwm10_data(&self) -> R8Pwm10DataR {
        R8Pwm10DataR::new(self.bits)
    }
}
impl W {
    #[doc = "Bits 0:7 - RW, PWM10 data holding"]
    #[inline(always)]
    pub fn r8_pwm10_data(&mut self) -> R8Pwm10DataW<'_, R8Pwm10DataSpec> {
        R8Pwm10DataW::new(self, 0)
    }
}
#[doc = "RW, PWM10 data holding\n\nYou can [`read`](crate::Reg::read) this register and get [`r8_pwm10_data::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`r8_pwm10_data::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct R8Pwm10DataSpec;
impl crate::RegisterSpec for R8Pwm10DataSpec {
    type Ux = u8;
}
#[doc = "`read()` method returns [`r8_pwm10_data::R`](R) reader structure"]
impl crate::Readable for R8Pwm10DataSpec {}
#[doc = "`write(|w| ..)` method takes [`r8_pwm10_data::W`](W) writer structure"]
impl crate::Writable for R8Pwm10DataSpec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets R8_PWM10_DATA to value 0"]
impl crate::Resettable for R8Pwm10DataSpec {}
