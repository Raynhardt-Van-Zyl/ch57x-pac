#[doc = "Register `R8_PWM_CONFIG` reader"]
pub type R = crate::R<R8PwmConfigSpec>;
#[doc = "Register `R8_PWM_CONFIG` writer"]
pub type W = crate::W<R8PwmConfigSpec>;
#[doc = "Field `RB_PWM_CYCLE_SEL` reader - RW, PWM cycle selection: 0=256;128;64;32 clocks, 1=255;127;63;31 clocks"]
pub type RbPwmCycleSelR = crate::BitReader;
#[doc = "Field `RB_PWM_CYCLE_SEL` writer - RW, PWM cycle selection: 0=256;128;64;32 clocks, 1=255;127;63;31 clocks"]
pub type RbPwmCycleSelW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `RB_PWM_STAG_ST` reader - RO, PWM stagger cycle status"]
pub type RbPwmStagStR = crate::BitReader;
#[doc = "Field `RB_PWM_CYC_MOD` reader - RW, PWM data width mode: 00=8 bits data, 01=7 bits data, 10=6 bits data, 11=5 bits data"]
pub type RbPwmCycModR = crate::FieldReader;
#[doc = "Field `RB_PWM_CYC_MOD` writer - RW, PWM data width mode: 00=8 bits data, 01=7 bits data, 10=6 bits data, 11=5 bits data"]
pub type RbPwmCycModW<'a, REG> = crate::FieldWriter<'a, REG, 2>;
#[doc = "Field `RB_PWM4_5_STAG_EN` reader - RW, PWM4/5 stagger output enable: 0=independent output, 1=stagger output"]
pub type RbPwm4_5StagEnR = crate::BitReader;
#[doc = "Field `RB_PWM4_5_STAG_EN` writer - RW, PWM4/5 stagger output enable: 0=independent output, 1=stagger output"]
pub type RbPwm4_5StagEnW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `RB_PWM6_7_STAG_EN` reader - RW, PWM6/7 stagger output enable: 0=independent output, 1=stagger output"]
pub type RbPwm6_7StagEnR = crate::BitReader;
#[doc = "Field `RB_PWM6_7_STAG_EN` writer - RW, PWM6/7 stagger output enable: 0=independent output, 1=stagger output"]
pub type RbPwm6_7StagEnW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `RB_PWM8_9_STAG_EN` reader - RW, PWM8/9 stagger output enable: 0=independent output, 1=stagger output"]
pub type RbPwm8_9StagEnR = crate::BitReader;
#[doc = "Field `RB_PWM8_9_STAG_EN` writer - RW, PWM8/9 stagger output enable: 0=independent output, 1=stagger output"]
pub type RbPwm8_9StagEnW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `RB_PWM10_11_STAG_EN` reader - RW, PWM10/11 stagger output enable: 0=independent output, 1=stagger output"]
pub type RbPwm10_11StagEnR = crate::BitReader;
#[doc = "Field `RB_PWM10_11_STAG_EN` writer - RW, PWM10/11 stagger output enable: 0=independent output, 1=stagger output"]
pub type RbPwm10_11StagEnW<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bit 0 - RW, PWM cycle selection: 0=256;128;64;32 clocks, 1=255;127;63;31 clocks"]
    #[inline(always)]
    pub fn rb_pwm_cycle_sel(&self) -> RbPwmCycleSelR {
        RbPwmCycleSelR::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - RO, PWM stagger cycle status"]
    #[inline(always)]
    pub fn rb_pwm_stag_st(&self) -> RbPwmStagStR {
        RbPwmStagStR::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bits 2:3 - RW, PWM data width mode: 00=8 bits data, 01=7 bits data, 10=6 bits data, 11=5 bits data"]
    #[inline(always)]
    pub fn rb_pwm_cyc_mod(&self) -> RbPwmCycModR {
        RbPwmCycModR::new((self.bits >> 2) & 3)
    }
    #[doc = "Bit 4 - RW, PWM4/5 stagger output enable: 0=independent output, 1=stagger output"]
    #[inline(always)]
    pub fn rb_pwm4_5_stag_en(&self) -> RbPwm4_5StagEnR {
        RbPwm4_5StagEnR::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bit 5 - RW, PWM6/7 stagger output enable: 0=independent output, 1=stagger output"]
    #[inline(always)]
    pub fn rb_pwm6_7_stag_en(&self) -> RbPwm6_7StagEnR {
        RbPwm6_7StagEnR::new(((self.bits >> 5) & 1) != 0)
    }
    #[doc = "Bit 6 - RW, PWM8/9 stagger output enable: 0=independent output, 1=stagger output"]
    #[inline(always)]
    pub fn rb_pwm8_9_stag_en(&self) -> RbPwm8_9StagEnR {
        RbPwm8_9StagEnR::new(((self.bits >> 6) & 1) != 0)
    }
    #[doc = "Bit 7 - RW, PWM10/11 stagger output enable: 0=independent output, 1=stagger output"]
    #[inline(always)]
    pub fn rb_pwm10_11_stag_en(&self) -> RbPwm10_11StagEnR {
        RbPwm10_11StagEnR::new(((self.bits >> 7) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - RW, PWM cycle selection: 0=256;128;64;32 clocks, 1=255;127;63;31 clocks"]
    #[inline(always)]
    pub fn rb_pwm_cycle_sel(&mut self) -> RbPwmCycleSelW<'_, R8PwmConfigSpec> {
        RbPwmCycleSelW::new(self, 0)
    }
    #[doc = "Bits 2:3 - RW, PWM data width mode: 00=8 bits data, 01=7 bits data, 10=6 bits data, 11=5 bits data"]
    #[inline(always)]
    pub fn rb_pwm_cyc_mod(&mut self) -> RbPwmCycModW<'_, R8PwmConfigSpec> {
        RbPwmCycModW::new(self, 2)
    }
    #[doc = "Bit 4 - RW, PWM4/5 stagger output enable: 0=independent output, 1=stagger output"]
    #[inline(always)]
    pub fn rb_pwm4_5_stag_en(&mut self) -> RbPwm4_5StagEnW<'_, R8PwmConfigSpec> {
        RbPwm4_5StagEnW::new(self, 4)
    }
    #[doc = "Bit 5 - RW, PWM6/7 stagger output enable: 0=independent output, 1=stagger output"]
    #[inline(always)]
    pub fn rb_pwm6_7_stag_en(&mut self) -> RbPwm6_7StagEnW<'_, R8PwmConfigSpec> {
        RbPwm6_7StagEnW::new(self, 5)
    }
    #[doc = "Bit 6 - RW, PWM8/9 stagger output enable: 0=independent output, 1=stagger output"]
    #[inline(always)]
    pub fn rb_pwm8_9_stag_en(&mut self) -> RbPwm8_9StagEnW<'_, R8PwmConfigSpec> {
        RbPwm8_9StagEnW::new(self, 6)
    }
    #[doc = "Bit 7 - RW, PWM10/11 stagger output enable: 0=independent output, 1=stagger output"]
    #[inline(always)]
    pub fn rb_pwm10_11_stag_en(&mut self) -> RbPwm10_11StagEnW<'_, R8PwmConfigSpec> {
        RbPwm10_11StagEnW::new(self, 7)
    }
}
#[doc = "RW, PWM configuration\n\nYou can [`read`](crate::Reg::read) this register and get [`r8_pwm_config::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`r8_pwm_config::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct R8PwmConfigSpec;
impl crate::RegisterSpec for R8PwmConfigSpec {
    type Ux = u8;
}
#[doc = "`read()` method returns [`r8_pwm_config::R`](R) reader structure"]
impl crate::Readable for R8PwmConfigSpec {}
#[doc = "`write(|w| ..)` method takes [`r8_pwm_config::W`](W) writer structure"]
impl crate::Writable for R8PwmConfigSpec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets R8_PWM_CONFIG to value 0"]
impl crate::Resettable for R8PwmConfigSpec {}
