#[doc = "Register `R8_PWM_POLAR` reader"]
pub type R = crate::R<R8PwmPolarSpec>;
#[doc = "Register `R8_PWM_POLAR` writer"]
pub type W = crate::W<R8PwmPolarSpec>;
#[doc = "Field `RB_PWM4_POLAR` reader - RW, PWM4 output polarity: 0=default low and high action, 1=default high and low action"]
pub type RbPwm4PolarR = crate::BitReader;
#[doc = "Field `RB_PWM4_POLAR` writer - RW, PWM4 output polarity: 0=default low and high action, 1=default high and low action"]
pub type RbPwm4PolarW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `RB_PWM5_POLAR` reader - RW, PWM5 output polarity: 0=default low and high action, 1=default high and low action"]
pub type RbPwm5PolarR = crate::BitReader;
#[doc = "Field `RB_PWM5_POLAR` writer - RW, PWM5 output polarity: 0=default low and high action, 1=default high and low action"]
pub type RbPwm5PolarW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `RB_PWM6_POLAR` reader - RW, PWM6 output polarity: 0=default low and high action, 1=default high and low action"]
pub type RbPwm6PolarR = crate::BitReader;
#[doc = "Field `RB_PWM6_POLAR` writer - RW, PWM6 output polarity: 0=default low and high action, 1=default high and low action"]
pub type RbPwm6PolarW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `RB_PWM7_POLAR` reader - RW, PWM7 output polarity: 0=default low and high action, 1=default high and low action"]
pub type RbPwm7PolarR = crate::BitReader;
#[doc = "Field `RB_PWM7_POLAR` writer - RW, PWM7 output polarity: 0=default low and high action, 1=default high and low action"]
pub type RbPwm7PolarW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `RB_PWM8_POLAR` reader - RW, PWM8 output polarity: 0=default low and high action, 1=default high and low action"]
pub type RbPwm8PolarR = crate::BitReader;
#[doc = "Field `RB_PWM8_POLAR` writer - RW, PWM8 output polarity: 0=default low and high action, 1=default high and low action"]
pub type RbPwm8PolarW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `RB_PWM9_POLAR` reader - RW, PWM9 output polarity: 0=default low and high action, 1=default high and low action"]
pub type RbPwm9PolarR = crate::BitReader;
#[doc = "Field `RB_PWM9_POLAR` writer - RW, PWM9 output polarity: 0=default low and high action, 1=default high and low action"]
pub type RbPwm9PolarW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `RB_PWM10_POLAR` reader - RW, PWM10 output polarity: 0=default low and high action, 1=default high and low action"]
pub type RbPwm10PolarR = crate::BitReader;
#[doc = "Field `RB_PWM10_POLAR` writer - RW, PWM10 output polarity: 0=default low and high action, 1=default high and low action"]
pub type RbPwm10PolarW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `RB_PWM11_POLAR` reader - RW, PWM11 output polarity: 0=default low and high action, 1=default high and low action"]
pub type RbPwm11PolarR = crate::BitReader;
#[doc = "Field `RB_PWM11_POLAR` writer - RW, PWM11 output polarity: 0=default low and high action, 1=default high and low action"]
pub type RbPwm11PolarW<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bit 0 - RW, PWM4 output polarity: 0=default low and high action, 1=default high and low action"]
    #[inline(always)]
    pub fn rb_pwm4_polar(&self) -> RbPwm4PolarR {
        RbPwm4PolarR::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - RW, PWM5 output polarity: 0=default low and high action, 1=default high and low action"]
    #[inline(always)]
    pub fn rb_pwm5_polar(&self) -> RbPwm5PolarR {
        RbPwm5PolarR::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - RW, PWM6 output polarity: 0=default low and high action, 1=default high and low action"]
    #[inline(always)]
    pub fn rb_pwm6_polar(&self) -> RbPwm6PolarR {
        RbPwm6PolarR::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3 - RW, PWM7 output polarity: 0=default low and high action, 1=default high and low action"]
    #[inline(always)]
    pub fn rb_pwm7_polar(&self) -> RbPwm7PolarR {
        RbPwm7PolarR::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bit 4 - RW, PWM8 output polarity: 0=default low and high action, 1=default high and low action"]
    #[inline(always)]
    pub fn rb_pwm8_polar(&self) -> RbPwm8PolarR {
        RbPwm8PolarR::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bit 5 - RW, PWM9 output polarity: 0=default low and high action, 1=default high and low action"]
    #[inline(always)]
    pub fn rb_pwm9_polar(&self) -> RbPwm9PolarR {
        RbPwm9PolarR::new(((self.bits >> 5) & 1) != 0)
    }
    #[doc = "Bit 6 - RW, PWM10 output polarity: 0=default low and high action, 1=default high and low action"]
    #[inline(always)]
    pub fn rb_pwm10_polar(&self) -> RbPwm10PolarR {
        RbPwm10PolarR::new(((self.bits >> 6) & 1) != 0)
    }
    #[doc = "Bit 7 - RW, PWM11 output polarity: 0=default low and high action, 1=default high and low action"]
    #[inline(always)]
    pub fn rb_pwm11_polar(&self) -> RbPwm11PolarR {
        RbPwm11PolarR::new(((self.bits >> 7) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - RW, PWM4 output polarity: 0=default low and high action, 1=default high and low action"]
    #[inline(always)]
    pub fn rb_pwm4_polar(&mut self) -> RbPwm4PolarW<'_, R8PwmPolarSpec> {
        RbPwm4PolarW::new(self, 0)
    }
    #[doc = "Bit 1 - RW, PWM5 output polarity: 0=default low and high action, 1=default high and low action"]
    #[inline(always)]
    pub fn rb_pwm5_polar(&mut self) -> RbPwm5PolarW<'_, R8PwmPolarSpec> {
        RbPwm5PolarW::new(self, 1)
    }
    #[doc = "Bit 2 - RW, PWM6 output polarity: 0=default low and high action, 1=default high and low action"]
    #[inline(always)]
    pub fn rb_pwm6_polar(&mut self) -> RbPwm6PolarW<'_, R8PwmPolarSpec> {
        RbPwm6PolarW::new(self, 2)
    }
    #[doc = "Bit 3 - RW, PWM7 output polarity: 0=default low and high action, 1=default high and low action"]
    #[inline(always)]
    pub fn rb_pwm7_polar(&mut self) -> RbPwm7PolarW<'_, R8PwmPolarSpec> {
        RbPwm7PolarW::new(self, 3)
    }
    #[doc = "Bit 4 - RW, PWM8 output polarity: 0=default low and high action, 1=default high and low action"]
    #[inline(always)]
    pub fn rb_pwm8_polar(&mut self) -> RbPwm8PolarW<'_, R8PwmPolarSpec> {
        RbPwm8PolarW::new(self, 4)
    }
    #[doc = "Bit 5 - RW, PWM9 output polarity: 0=default low and high action, 1=default high and low action"]
    #[inline(always)]
    pub fn rb_pwm9_polar(&mut self) -> RbPwm9PolarW<'_, R8PwmPolarSpec> {
        RbPwm9PolarW::new(self, 5)
    }
    #[doc = "Bit 6 - RW, PWM10 output polarity: 0=default low and high action, 1=default high and low action"]
    #[inline(always)]
    pub fn rb_pwm10_polar(&mut self) -> RbPwm10PolarW<'_, R8PwmPolarSpec> {
        RbPwm10PolarW::new(self, 6)
    }
    #[doc = "Bit 7 - RW, PWM11 output polarity: 0=default low and high action, 1=default high and low action"]
    #[inline(always)]
    pub fn rb_pwm11_polar(&mut self) -> RbPwm11PolarW<'_, R8PwmPolarSpec> {
        RbPwm11PolarW::new(self, 7)
    }
}
#[doc = "RW, PWM output polarity control\n\nYou can [`read`](crate::Reg::read) this register and get [`r8_pwm_polar::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`r8_pwm_polar::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct R8PwmPolarSpec;
impl crate::RegisterSpec for R8PwmPolarSpec {
    type Ux = u8;
}
#[doc = "`read()` method returns [`r8_pwm_polar::R`](R) reader structure"]
impl crate::Readable for R8PwmPolarSpec {}
#[doc = "`write(|w| ..)` method takes [`r8_pwm_polar::W`](W) writer structure"]
impl crate::Writable for R8PwmPolarSpec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets R8_PWM_POLAR to value 0"]
impl crate::Resettable for R8PwmPolarSpec {}
