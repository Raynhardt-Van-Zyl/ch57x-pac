#[doc = "Register `R8_PWM_OUT_EN` reader"]
pub type R = crate::R<R8PwmOutEnSpec>;
#[doc = "Register `R8_PWM_OUT_EN` writer"]
pub type W = crate::W<R8PwmOutEnSpec>;
#[doc = "Field `RB_PWM4_OUT_EN` reader - RW, PWM4 output enable"]
pub type RbPwm4OutEnR = crate::BitReader;
#[doc = "Field `RB_PWM4_OUT_EN` writer - RW, PWM4 output enable"]
pub type RbPwm4OutEnW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `RB_PWM5_OUT_EN` reader - RW, PWM5 output enable"]
pub type RbPwm5OutEnR = crate::BitReader;
#[doc = "Field `RB_PWM5_OUT_EN` writer - RW, PWM5 output enable"]
pub type RbPwm5OutEnW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `RB_PWM6_OUT_EN` reader - RW, PWM6 output enable"]
pub type RbPwm6OutEnR = crate::BitReader;
#[doc = "Field `RB_PWM6_OUT_EN` writer - RW, PWM6 output enable"]
pub type RbPwm6OutEnW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `RB_PWM7_OUT_EN` reader - RW, PWM7 output enable"]
pub type RbPwm7OutEnR = crate::BitReader;
#[doc = "Field `RB_PWM7_OUT_EN` writer - RW, PWM7 output enable"]
pub type RbPwm7OutEnW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `RB_PWM8_OUT_EN` reader - RW, PWM8 output enable"]
pub type RbPwm8OutEnR = crate::BitReader;
#[doc = "Field `RB_PWM8_OUT_EN` writer - RW, PWM8 output enable"]
pub type RbPwm8OutEnW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `RB_PWM9_OUT_EN` reader - RW, PWM9 output enable"]
pub type RbPwm9OutEnR = crate::BitReader;
#[doc = "Field `RB_PWM9_OUT_EN` writer - RW, PWM9 output enable"]
pub type RbPwm9OutEnW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `RB_PWM10_OUT_EN` reader - RW, PWM10 output enable"]
pub type RbPwm10OutEnR = crate::BitReader;
#[doc = "Field `RB_PWM10_OUT_EN` writer - RW, PWM10 output enable"]
pub type RbPwm10OutEnW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `RB_PWM11_OUT_EN` reader - RW, PWM11 output enable"]
pub type RbPwm11OutEnR = crate::BitReader;
#[doc = "Field `RB_PWM11_OUT_EN` writer - RW, PWM11 output enable"]
pub type RbPwm11OutEnW<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bit 0 - RW, PWM4 output enable"]
    #[inline(always)]
    pub fn rb_pwm4_out_en(&self) -> RbPwm4OutEnR {
        RbPwm4OutEnR::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - RW, PWM5 output enable"]
    #[inline(always)]
    pub fn rb_pwm5_out_en(&self) -> RbPwm5OutEnR {
        RbPwm5OutEnR::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - RW, PWM6 output enable"]
    #[inline(always)]
    pub fn rb_pwm6_out_en(&self) -> RbPwm6OutEnR {
        RbPwm6OutEnR::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3 - RW, PWM7 output enable"]
    #[inline(always)]
    pub fn rb_pwm7_out_en(&self) -> RbPwm7OutEnR {
        RbPwm7OutEnR::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bit 4 - RW, PWM8 output enable"]
    #[inline(always)]
    pub fn rb_pwm8_out_en(&self) -> RbPwm8OutEnR {
        RbPwm8OutEnR::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bit 5 - RW, PWM9 output enable"]
    #[inline(always)]
    pub fn rb_pwm9_out_en(&self) -> RbPwm9OutEnR {
        RbPwm9OutEnR::new(((self.bits >> 5) & 1) != 0)
    }
    #[doc = "Bit 6 - RW, PWM10 output enable"]
    #[inline(always)]
    pub fn rb_pwm10_out_en(&self) -> RbPwm10OutEnR {
        RbPwm10OutEnR::new(((self.bits >> 6) & 1) != 0)
    }
    #[doc = "Bit 7 - RW, PWM11 output enable"]
    #[inline(always)]
    pub fn rb_pwm11_out_en(&self) -> RbPwm11OutEnR {
        RbPwm11OutEnR::new(((self.bits >> 7) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - RW, PWM4 output enable"]
    #[inline(always)]
    pub fn rb_pwm4_out_en(&mut self) -> RbPwm4OutEnW<'_, R8PwmOutEnSpec> {
        RbPwm4OutEnW::new(self, 0)
    }
    #[doc = "Bit 1 - RW, PWM5 output enable"]
    #[inline(always)]
    pub fn rb_pwm5_out_en(&mut self) -> RbPwm5OutEnW<'_, R8PwmOutEnSpec> {
        RbPwm5OutEnW::new(self, 1)
    }
    #[doc = "Bit 2 - RW, PWM6 output enable"]
    #[inline(always)]
    pub fn rb_pwm6_out_en(&mut self) -> RbPwm6OutEnW<'_, R8PwmOutEnSpec> {
        RbPwm6OutEnW::new(self, 2)
    }
    #[doc = "Bit 3 - RW, PWM7 output enable"]
    #[inline(always)]
    pub fn rb_pwm7_out_en(&mut self) -> RbPwm7OutEnW<'_, R8PwmOutEnSpec> {
        RbPwm7OutEnW::new(self, 3)
    }
    #[doc = "Bit 4 - RW, PWM8 output enable"]
    #[inline(always)]
    pub fn rb_pwm8_out_en(&mut self) -> RbPwm8OutEnW<'_, R8PwmOutEnSpec> {
        RbPwm8OutEnW::new(self, 4)
    }
    #[doc = "Bit 5 - RW, PWM9 output enable"]
    #[inline(always)]
    pub fn rb_pwm9_out_en(&mut self) -> RbPwm9OutEnW<'_, R8PwmOutEnSpec> {
        RbPwm9OutEnW::new(self, 5)
    }
    #[doc = "Bit 6 - RW, PWM10 output enable"]
    #[inline(always)]
    pub fn rb_pwm10_out_en(&mut self) -> RbPwm10OutEnW<'_, R8PwmOutEnSpec> {
        RbPwm10OutEnW::new(self, 6)
    }
    #[doc = "Bit 7 - RW, PWM11 output enable"]
    #[inline(always)]
    pub fn rb_pwm11_out_en(&mut self) -> RbPwm11OutEnW<'_, R8PwmOutEnSpec> {
        RbPwm11OutEnW::new(self, 7)
    }
}
#[doc = "RW, PWM output enable control\n\nYou can [`read`](crate::Reg::read) this register and get [`r8_pwm_out_en::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`r8_pwm_out_en::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct R8PwmOutEnSpec;
impl crate::RegisterSpec for R8PwmOutEnSpec {
    type Ux = u8;
}
#[doc = "`read()` method returns [`r8_pwm_out_en::R`](R) reader structure"]
impl crate::Readable for R8PwmOutEnSpec {}
#[doc = "`write(|w| ..)` method takes [`r8_pwm_out_en::W`](W) writer structure"]
impl crate::Writable for R8PwmOutEnSpec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets R8_PWM_OUT_EN to value 0"]
impl crate::Resettable for R8PwmOutEnSpec {}
