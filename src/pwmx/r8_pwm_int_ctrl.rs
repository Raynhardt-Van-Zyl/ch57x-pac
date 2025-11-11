#[doc = "Register `R8_PWM_INT_CTRL` reader"]
pub type R = crate::R<R8PwmIntCtrlSpec>;
#[doc = "Register `R8_PWM_INT_CTRL` writer"]
pub type W = crate::W<R8PwmIntCtrlSpec>;
#[doc = "Field `RB_PWM_IE_CYC` reader - RW, enable interrupt for PWM cycle end"]
pub type RbPwmIeCycR = crate::BitReader;
#[doc = "Field `RB_PWM_IE_CYC` writer - RW, enable interrupt for PWM cycle end"]
pub type RbPwmIeCycW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `RB_PWM_CYC_PRE` reader - RW, select PWM cycle interrupt point"]
pub type RbPwmCycPreR = crate::BitReader;
#[doc = "Field `RB_PWM_CYC_PRE` writer - RW, select PWM cycle interrupt point"]
pub type RbPwmCycPreW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `RB_PWM_IF_CYC` reader - RW1, interrupt flag for PWM cycle end"]
pub type RbPwmIfCycR = crate::BitReader;
#[doc = "Field `RB_PWM_IF_CYC` writer - RW1, interrupt flag for PWM cycle end"]
pub type RbPwmIfCycW<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bit 0 - RW, enable interrupt for PWM cycle end"]
    #[inline(always)]
    pub fn rb_pwm_ie_cyc(&self) -> RbPwmIeCycR {
        RbPwmIeCycR::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - RW, select PWM cycle interrupt point"]
    #[inline(always)]
    pub fn rb_pwm_cyc_pre(&self) -> RbPwmCycPreR {
        RbPwmCycPreR::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 7 - RW1, interrupt flag for PWM cycle end"]
    #[inline(always)]
    pub fn rb_pwm_if_cyc(&self) -> RbPwmIfCycR {
        RbPwmIfCycR::new(((self.bits >> 7) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - RW, enable interrupt for PWM cycle end"]
    #[inline(always)]
    pub fn rb_pwm_ie_cyc(&mut self) -> RbPwmIeCycW<'_, R8PwmIntCtrlSpec> {
        RbPwmIeCycW::new(self, 0)
    }
    #[doc = "Bit 1 - RW, select PWM cycle interrupt point"]
    #[inline(always)]
    pub fn rb_pwm_cyc_pre(&mut self) -> RbPwmCycPreW<'_, R8PwmIntCtrlSpec> {
        RbPwmCycPreW::new(self, 1)
    }
    #[doc = "Bit 7 - RW1, interrupt flag for PWM cycle end"]
    #[inline(always)]
    pub fn rb_pwm_if_cyc(&mut self) -> RbPwmIfCycW<'_, R8PwmIntCtrlSpec> {
        RbPwmIfCycW::new(self, 7)
    }
}
#[doc = "RW, PWM interrupt control\n\nYou can [`read`](crate::Reg::read) this register and get [`r8_pwm_int_ctrl::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`r8_pwm_int_ctrl::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct R8PwmIntCtrlSpec;
impl crate::RegisterSpec for R8PwmIntCtrlSpec {
    type Ux = u8;
}
#[doc = "`read()` method returns [`r8_pwm_int_ctrl::R`](R) reader structure"]
impl crate::Readable for R8PwmIntCtrlSpec {}
#[doc = "`write(|w| ..)` method takes [`r8_pwm_int_ctrl::W`](W) writer structure"]
impl crate::Writable for R8PwmIntCtrlSpec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets R8_PWM_INT_CTRL to value 0"]
impl crate::Resettable for R8PwmIntCtrlSpec {}
