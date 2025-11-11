#[doc = "Register `R8_SAFE_ACCESS_SIG` reader"]
pub type R = crate::R<R8SafeAccessSigSpec>;
#[doc = "Register `R8_SAFE_ACCESS_SIG` writer"]
pub type W = crate::W<R8SafeAccessSigSpec>;
#[doc = "Field `RB_SAFE_ACC_MODE` reader - RO, current safe accessing mode: 11=safe unlocked (SAM), other=locked (00..01..10..11)"]
pub type RbSafeAccModeR = crate::FieldReader;
#[doc = "Field `RB_SAFE_ACC_MODE` writer - RO, current safe accessing mode: 11=safe unlocked (SAM), other=locked (00..01..10..11)"]
pub type RbSafeAccModeW<'a, REG> = crate::FieldWriter<'a, REG, 2>;
#[doc = "Field `R8_SAFE_ACCESS_SIG` reader - WO, safe accessing sign register, must write 0x57 then 0xA8 to enter safe accessing mode"]
pub type R8SafeAccessSigR = crate::FieldReader;
#[doc = "Field `R8_SAFE_ACCESS_SIG` writer - WO, safe accessing sign register, must write 0x57 then 0xA8 to enter safe accessing mode"]
pub type R8SafeAccessSigW<'a, REG> = crate::FieldWriter<'a, REG, 8>;
#[doc = "Field `RB_SAFE_ACC_ACT` reader - RO, indicate safe accessing status now: 0=locked, read only, 1=safe/unlocked (SAM), write enabled"]
pub type RbSafeAccActR = crate::BitReader;
#[doc = "Field `RB_SAFE_ACC_ACT` writer - RO, indicate safe accessing status now: 0=locked, read only, 1=safe/unlocked (SAM), write enabled"]
pub type RbSafeAccActW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `RB_SAFE_ACC_TIMER` reader - RO, safe accessing timer bit mask (16*clock number)"]
pub type RbSafeAccTimerR = crate::FieldReader;
#[doc = "Field `RB_SAFE_ACC_TIMER` writer - RO, safe accessing timer bit mask (16*clock number)"]
pub type RbSafeAccTimerW<'a, REG> = crate::FieldWriter<'a, REG, 3>;
impl R {
    #[doc = "Bits 0:1 - RO, current safe accessing mode: 11=safe unlocked (SAM), other=locked (00..01..10..11)"]
    #[inline(always)]
    pub fn rb_safe_acc_mode(&self) -> RbSafeAccModeR {
        RbSafeAccModeR::new(self.bits & 3)
    }
    #[doc = "Bits 0:7 - WO, safe accessing sign register, must write 0x57 then 0xA8 to enter safe accessing mode"]
    #[inline(always)]
    pub fn r8_safe_access_sig(&self) -> R8SafeAccessSigR {
        R8SafeAccessSigR::new(self.bits)
    }
    #[doc = "Bit 3 - RO, indicate safe accessing status now: 0=locked, read only, 1=safe/unlocked (SAM), write enabled"]
    #[inline(always)]
    pub fn rb_safe_acc_act(&self) -> RbSafeAccActR {
        RbSafeAccActR::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bits 4:6 - RO, safe accessing timer bit mask (16*clock number)"]
    #[inline(always)]
    pub fn rb_safe_acc_timer(&self) -> RbSafeAccTimerR {
        RbSafeAccTimerR::new((self.bits >> 4) & 7)
    }
}
impl W {
    #[doc = "Bits 0:1 - RO, current safe accessing mode: 11=safe unlocked (SAM), other=locked (00..01..10..11)"]
    #[inline(always)]
    pub fn rb_safe_acc_mode(&mut self) -> RbSafeAccModeW<'_, R8SafeAccessSigSpec> {
        RbSafeAccModeW::new(self, 0)
    }
    #[doc = "Bits 0:7 - WO, safe accessing sign register, must write 0x57 then 0xA8 to enter safe accessing mode"]
    #[inline(always)]
    pub fn r8_safe_access_sig(&mut self) -> R8SafeAccessSigW<'_, R8SafeAccessSigSpec> {
        R8SafeAccessSigW::new(self, 0)
    }
    #[doc = "Bit 3 - RO, indicate safe accessing status now: 0=locked, read only, 1=safe/unlocked (SAM), write enabled"]
    #[inline(always)]
    pub fn rb_safe_acc_act(&mut self) -> RbSafeAccActW<'_, R8SafeAccessSigSpec> {
        RbSafeAccActW::new(self, 3)
    }
    #[doc = "Bits 4:6 - RO, safe accessing timer bit mask (16*clock number)"]
    #[inline(always)]
    pub fn rb_safe_acc_timer(&mut self) -> RbSafeAccTimerW<'_, R8SafeAccessSigSpec> {
        RbSafeAccTimerW::new(self, 4)
    }
}
#[doc = "WO, safe accessing sign register, must write SAFE_ACCESS_SIG1 then SAFE_ACCESS_SIG2 to enter safe accessing mode\n\nYou can [`read`](crate::Reg::read) this register and get [`r8_safe_access_sig::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`r8_safe_access_sig::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct R8SafeAccessSigSpec;
impl crate::RegisterSpec for R8SafeAccessSigSpec {
    type Ux = u8;
}
#[doc = "`read()` method returns [`r8_safe_access_sig::R`](R) reader structure"]
impl crate::Readable for R8SafeAccessSigSpec {}
#[doc = "`write(|w| ..)` method takes [`r8_safe_access_sig::W`](W) writer structure"]
impl crate::Writable for R8SafeAccessSigSpec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets R8_SAFE_ACCESS_SIG to value 0"]
impl crate::Resettable for R8SafeAccessSigSpec {}
