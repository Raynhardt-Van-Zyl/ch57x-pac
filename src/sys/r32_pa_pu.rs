#[doc = "Register `R32_PA_PU` reader"]
pub type R = crate::R<R32PaPuSpec>;
#[doc = "Register `R32_PA_PU` writer"]
pub type W = crate::W<R32PaPuSpec>;
#[doc = "Field `R8_PA_PU_0` reader - GPIO PA pullup resistance enable byte 0"]
pub type R8PaPu0R = crate::FieldReader;
#[doc = "Field `R8_PA_PU_0` writer - GPIO PA pullup resistance enable byte 0"]
pub type R8PaPu0W<'a, REG> = crate::FieldWriter<'a, REG, 8>;
#[doc = "Field `R8_PA_PU_1` reader - GPIO PA pullup resistance enable byte 0"]
pub type R8PaPu1R = crate::FieldReader;
#[doc = "Field `R8_PA_PU_1` writer - GPIO PA pullup resistance enable byte 0"]
pub type R8PaPu1W<'a, REG> = crate::FieldWriter<'a, REG, 8>;
impl R {
    #[doc = "Bits 0:7 - GPIO PA pullup resistance enable byte 0"]
    #[inline(always)]
    pub fn r8_pa_pu_0(&self) -> R8PaPu0R {
        R8PaPu0R::new((self.bits & 0xff) as u8)
    }
    #[doc = "Bits 8:15 - GPIO PA pullup resistance enable byte 0"]
    #[inline(always)]
    pub fn r8_pa_pu_1(&self) -> R8PaPu1R {
        R8PaPu1R::new(((self.bits >> 8) & 0xff) as u8)
    }
}
impl W {
    #[doc = "Bits 0:7 - GPIO PA pullup resistance enable byte 0"]
    #[inline(always)]
    pub fn r8_pa_pu_0(&mut self) -> R8PaPu0W<'_, R32PaPuSpec> {
        R8PaPu0W::new(self, 0)
    }
    #[doc = "Bits 8:15 - GPIO PA pullup resistance enable byte 0"]
    #[inline(always)]
    pub fn r8_pa_pu_1(&mut self) -> R8PaPu1W<'_, R32PaPuSpec> {
        R8PaPu1W::new(self, 8)
    }
}
#[doc = "RW, GPIO PA pullup resistance enable\n\nYou can [`read`](crate::Reg::read) this register and get [`r32_pa_pu::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`r32_pa_pu::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct R32PaPuSpec;
impl crate::RegisterSpec for R32PaPuSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`r32_pa_pu::R`](R) reader structure"]
impl crate::Readable for R32PaPuSpec {}
#[doc = "`write(|w| ..)` method takes [`r32_pa_pu::W`](W) writer structure"]
impl crate::Writable for R32PaPuSpec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets R32_PA_PU to value 0"]
impl crate::Resettable for R32PaPuSpec {}
