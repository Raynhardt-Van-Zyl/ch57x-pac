#[doc = "Register `R8_XT32K_TUNE` reader"]
pub type R = crate::R<R8Xt32kTuneSpec>;
#[doc = "Register `R8_XT32K_TUNE` writer"]
pub type W = crate::W<R8Xt32kTuneSpec>;
#[doc = "Field `RB_XT32K_I_TUNE` reader - RWA, external 32KHz oscillator current tune: 00=75% current, 01=standard current, 10=150% current, 11=200% current"]
pub type RbXt32kITuneR = crate::FieldReader;
#[doc = "Field `RB_XT32K_I_TUNE` writer - RWA, external 32KHz oscillator current tune: 00=75% current, 01=standard current, 10=150% current, 11=200% current"]
pub type RbXt32kITuneW<'a, REG> = crate::FieldWriter<'a, REG, 2>;
#[doc = "Field `RB_XT32K_C_LOAD` reader - RWA, external 32KHz oscillator load capacitor tune: Cap = RB_XT32K_C_LOAD + 12pF"]
pub type RbXt32kCLoadR = crate::FieldReader;
#[doc = "Field `RB_XT32K_C_LOAD` writer - RWA, external 32KHz oscillator load capacitor tune: Cap = RB_XT32K_C_LOAD + 12pF"]
pub type RbXt32kCLoadW<'a, REG> = crate::FieldWriter<'a, REG, 4>;
impl R {
    #[doc = "Bits 0:1 - RWA, external 32KHz oscillator current tune: 00=75% current, 01=standard current, 10=150% current, 11=200% current"]
    #[inline(always)]
    pub fn rb_xt32k_i_tune(&self) -> RbXt32kITuneR {
        RbXt32kITuneR::new(self.bits & 3)
    }
    #[doc = "Bits 4:7 - RWA, external 32KHz oscillator load capacitor tune: Cap = RB_XT32K_C_LOAD + 12pF"]
    #[inline(always)]
    pub fn rb_xt32k_c_load(&self) -> RbXt32kCLoadR {
        RbXt32kCLoadR::new((self.bits >> 4) & 0x0f)
    }
}
impl W {
    #[doc = "Bits 0:1 - RWA, external 32KHz oscillator current tune: 00=75% current, 01=standard current, 10=150% current, 11=200% current"]
    #[inline(always)]
    pub fn rb_xt32k_i_tune(&mut self) -> RbXt32kITuneW<'_, R8Xt32kTuneSpec> {
        RbXt32kITuneW::new(self, 0)
    }
    #[doc = "Bits 4:7 - RWA, external 32KHz oscillator load capacitor tune: Cap = RB_XT32K_C_LOAD + 12pF"]
    #[inline(always)]
    pub fn rb_xt32k_c_load(&mut self) -> RbXt32kCLoadW<'_, R8Xt32kTuneSpec> {
        RbXt32kCLoadW::new(self, 4)
    }
}
#[doc = "RWA, external 32KHz oscillator tune control, SAM\n\nYou can [`read`](crate::Reg::read) this register and get [`r8_xt32k_tune::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`r8_xt32k_tune::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct R8Xt32kTuneSpec;
impl crate::RegisterSpec for R8Xt32kTuneSpec {
    type Ux = u8;
}
#[doc = "`read()` method returns [`r8_xt32k_tune::R`](R) reader structure"]
impl crate::Readable for R8Xt32kTuneSpec {}
#[doc = "`write(|w| ..)` method takes [`r8_xt32k_tune::W`](W) writer structure"]
impl crate::Writable for R8Xt32kTuneSpec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets R8_XT32K_TUNE to value 0xc3"]
impl crate::Resettable for R8Xt32kTuneSpec {
    const RESET_VALUE: u8 = 0xc3;
}
