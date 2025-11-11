#[doc = "Register `R16_INT32K_TUNE` reader"]
pub type R = crate::R<R16Int32kTuneSpec>;
#[doc = "Register `R16_INT32K_TUNE` writer"]
pub type W = crate::W<R16Int32kTuneSpec>;
#[doc = "Field `RB_INT32K_TUNE` reader - RWA, internal 32KHz oscillator frequency tune"]
pub type RbInt32kTuneR = crate::FieldReader<u16>;
#[doc = "Field `RB_INT32K_TUNE` writer - RWA, internal 32KHz oscillator frequency tune"]
pub type RbInt32kTuneW<'a, REG> = crate::FieldWriter<'a, REG, 12, u16>;
impl R {
    #[doc = "Bits 0:11 - RWA, internal 32KHz oscillator frequency tune"]
    #[inline(always)]
    pub fn rb_int32k_tune(&self) -> RbInt32kTuneR {
        RbInt32kTuneR::new(self.bits & 0x0fff)
    }
}
impl W {
    #[doc = "Bits 0:11 - RWA, internal 32KHz oscillator frequency tune"]
    #[inline(always)]
    pub fn rb_int32k_tune(&mut self) -> RbInt32kTuneW<'_, R16Int32kTuneSpec> {
        RbInt32kTuneW::new(self, 0)
    }
}
#[doc = "RWA, internal 32KHz oscillator tune control, SAM\n\nYou can [`read`](crate::Reg::read) this register and get [`r16_int32k_tune::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`r16_int32k_tune::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct R16Int32kTuneSpec;
impl crate::RegisterSpec for R16Int32kTuneSpec {
    type Ux = u16;
}
#[doc = "`read()` method returns [`r16_int32k_tune::R`](R) reader structure"]
impl crate::Readable for R16Int32kTuneSpec {}
#[doc = "`write(|w| ..)` method takes [`r16_int32k_tune::W`](W) writer structure"]
impl crate::Writable for R16Int32kTuneSpec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets R16_INT32K_TUNE to value 0x0800"]
impl crate::Resettable for R16Int32kTuneSpec {
    const RESET_VALUE: u16 = 0x0800;
}
