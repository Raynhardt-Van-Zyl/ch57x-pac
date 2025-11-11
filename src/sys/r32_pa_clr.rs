#[doc = "Register `R32_PA_CLR` reader"]
pub type R = crate::R<R32PaClrSpec>;
#[doc = "Register `R32_PA_CLR` writer"]
pub type W = crate::W<R32PaClrSpec>;
#[doc = "Field `R8_PA_CLR_0` reader - GPIO PA clear output byte 0"]
pub type R8PaClr0R = crate::FieldReader;
#[doc = "Field `R8_PA_CLR_0` writer - GPIO PA clear output byte 0"]
pub type R8PaClr0W<'a, REG> = crate::FieldWriter<'a, REG, 8>;
#[doc = "Field `R8_PA_CLR_1` reader - GPIO PA clear output byte 1"]
pub type R8PaClr1R = crate::FieldReader;
#[doc = "Field `R8_PA_CLR_1` writer - GPIO PA clear output byte 1"]
pub type R8PaClr1W<'a, REG> = crate::FieldWriter<'a, REG, 8>;
impl R {
    #[doc = "Bits 0:7 - GPIO PA clear output byte 0"]
    #[inline(always)]
    pub fn r8_pa_clr_0(&self) -> R8PaClr0R {
        R8PaClr0R::new((self.bits & 0xff) as u8)
    }
    #[doc = "Bits 8:15 - GPIO PA clear output byte 1"]
    #[inline(always)]
    pub fn r8_pa_clr_1(&self) -> R8PaClr1R {
        R8PaClr1R::new(((self.bits >> 8) & 0xff) as u8)
    }
}
impl W {
    #[doc = "Bits 0:7 - GPIO PA clear output byte 0"]
    #[inline(always)]
    pub fn r8_pa_clr_0(&mut self) -> R8PaClr0W<'_, R32PaClrSpec> {
        R8PaClr0W::new(self, 0)
    }
    #[doc = "Bits 8:15 - GPIO PA clear output byte 1"]
    #[inline(always)]
    pub fn r8_pa_clr_1(&mut self) -> R8PaClr1W<'_, R32PaClrSpec> {
        R8PaClr1W::new(self, 8)
    }
}
#[doc = "WZ, GPIO PA clear output: 0=keep, 1=clear\n\nYou can [`read`](crate::Reg::read) this register and get [`r32_pa_clr::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`r32_pa_clr::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct R32PaClrSpec;
impl crate::RegisterSpec for R32PaClrSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`r32_pa_clr::R`](R) reader structure"]
impl crate::Readable for R32PaClrSpec {}
#[doc = "`write(|w| ..)` method takes [`r32_pa_clr::W`](W) writer structure"]
impl crate::Writable for R32PaClrSpec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets R32_PA_CLR to value 0"]
impl crate::Resettable for R32PaClrSpec {}
