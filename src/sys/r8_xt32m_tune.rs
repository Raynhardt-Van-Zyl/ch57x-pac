#[doc = "Register `R8_XT32M_TUNE` reader"]
pub type R = crate::R<R8Xt32mTuneSpec>;
#[doc = "Register `R8_XT32M_TUNE` writer"]
pub type W = crate::W<R8Xt32mTuneSpec>;
#[doc = "Field `RB_XT32M_I_BIAS` reader - RWA, external 32MHz oscillator bias current tune: 00=75% current, 01=standard current, 10=125% current, 11=150% current"]
pub type RbXt32mIBiasR = crate::FieldReader;
#[doc = "Field `RB_XT32M_I_BIAS` writer - RWA, external 32MHz oscillator bias current tune: 00=75% current, 01=standard current, 10=125% current, 11=150% current"]
pub type RbXt32mIBiasW<'a, REG> = crate::FieldWriter<'a, REG, 2>;
#[doc = "Field `RB_XT32M_C_LOAD` reader - RWA, external 32MHz oscillator load capacitor tune: Cap = RB_XT32M_C_LOAD * 2 + 10pF"]
pub type RbXt32mCLoadR = crate::FieldReader;
#[doc = "Field `RB_XT32M_C_LOAD` writer - RWA, external 32MHz oscillator load capacitor tune: Cap = RB_XT32M_C_LOAD * 2 + 10pF"]
pub type RbXt32mCLoadW<'a, REG> = crate::FieldWriter<'a, REG, 3>;
impl R {
    #[doc = "Bits 0:1 - RWA, external 32MHz oscillator bias current tune: 00=75% current, 01=standard current, 10=125% current, 11=150% current"]
    #[inline(always)]
    pub fn rb_xt32m_i_bias(&self) -> RbXt32mIBiasR {
        RbXt32mIBiasR::new(self.bits & 3)
    }
    #[doc = "Bits 4:6 - RWA, external 32MHz oscillator load capacitor tune: Cap = RB_XT32M_C_LOAD * 2 + 10pF"]
    #[inline(always)]
    pub fn rb_xt32m_c_load(&self) -> RbXt32mCLoadR {
        RbXt32mCLoadR::new((self.bits >> 4) & 7)
    }
}
impl W {
    #[doc = "Bits 0:1 - RWA, external 32MHz oscillator bias current tune: 00=75% current, 01=standard current, 10=125% current, 11=150% current"]
    #[inline(always)]
    pub fn rb_xt32m_i_bias(&mut self) -> RbXt32mIBiasW<'_, R8Xt32mTuneSpec> {
        RbXt32mIBiasW::new(self, 0)
    }
    #[doc = "Bits 4:6 - RWA, external 32MHz oscillator load capacitor tune: Cap = RB_XT32M_C_LOAD * 2 + 10pF"]
    #[inline(always)]
    pub fn rb_xt32m_c_load(&mut self) -> RbXt32mCLoadW<'_, R8Xt32mTuneSpec> {
        RbXt32mCLoadW::new(self, 4)
    }
}
#[doc = "RWA, external 32MHz oscillator tune control, SAM\n\nYou can [`read`](crate::Reg::read) this register and get [`r8_xt32m_tune::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`r8_xt32m_tune::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct R8Xt32mTuneSpec;
impl crate::RegisterSpec for R8Xt32mTuneSpec {
    type Ux = u8;
}
#[doc = "`read()` method returns [`r8_xt32m_tune::R`](R) reader structure"]
impl crate::Readable for R8Xt32mTuneSpec {}
#[doc = "`write(|w| ..)` method takes [`r8_xt32m_tune::W`](W) writer structure"]
impl crate::Writable for R8Xt32mTuneSpec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets R8_XT32M_TUNE to value 0x31"]
impl crate::Resettable for R8Xt32mTuneSpec {
    const RESET_VALUE: u8 = 0x31;
}
