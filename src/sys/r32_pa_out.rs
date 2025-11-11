#[doc = "Register `R32_PA_OUT` reader"]
pub type R = crate::R<R32PaOutSpec>;
#[doc = "Register `R32_PA_OUT` writer"]
pub type W = crate::W<R32PaOutSpec>;
#[doc = "Field `R8_PA_OUT_0` reader - GPIO PA output byte 0"]
pub type R8PaOut0R = crate::FieldReader;
#[doc = "Field `R8_PA_OUT_0` writer - GPIO PA output byte 0"]
pub type R8PaOut0W<'a, REG> = crate::FieldWriter<'a, REG, 8>;
#[doc = "Field `R8_PA_OUT_1` reader - GPIO PA output byte 1"]
pub type R8PaOut1R = crate::FieldReader;
#[doc = "Field `R8_PA_OUT_1` writer - GPIO PA output byte 1"]
pub type R8PaOut1W<'a, REG> = crate::FieldWriter<'a, REG, 8>;
impl R {
    #[doc = "Bits 0:7 - GPIO PA output byte 0"]
    #[inline(always)]
    pub fn r8_pa_out_0(&self) -> R8PaOut0R {
        R8PaOut0R::new((self.bits & 0xff) as u8)
    }
    #[doc = "Bits 8:15 - GPIO PA output byte 1"]
    #[inline(always)]
    pub fn r8_pa_out_1(&self) -> R8PaOut1R {
        R8PaOut1R::new(((self.bits >> 8) & 0xff) as u8)
    }
}
impl W {
    #[doc = "Bits 0:7 - GPIO PA output byte 0"]
    #[inline(always)]
    pub fn r8_pa_out_0(&mut self) -> R8PaOut0W<'_, R32PaOutSpec> {
        R8PaOut0W::new(self, 0)
    }
    #[doc = "Bits 8:15 - GPIO PA output byte 1"]
    #[inline(always)]
    pub fn r8_pa_out_1(&mut self) -> R8PaOut1W<'_, R32PaOutSpec> {
        R8PaOut1W::new(self, 8)
    }
}
#[doc = "RW, GPIO PA output\n\nYou can [`read`](crate::Reg::read) this register and get [`r32_pa_out::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`r32_pa_out::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct R32PaOutSpec;
impl crate::RegisterSpec for R32PaOutSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`r32_pa_out::R`](R) reader structure"]
impl crate::Readable for R32PaOutSpec {}
#[doc = "`write(|w| ..)` method takes [`r32_pa_out::W`](W) writer structure"]
impl crate::Writable for R32PaOutSpec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets R32_PA_OUT to value 0"]
impl crate::Resettable for R32PaOutSpec {}
