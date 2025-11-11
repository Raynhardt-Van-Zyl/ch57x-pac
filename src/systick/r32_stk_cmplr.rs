#[doc = "Register `R32_STK_CMPLR` reader"]
pub type R = crate::R<R32StkCmplrSpec>;
#[doc = "Register `R32_STK_CMPLR` writer"]
pub type W = crate::W<R32StkCmplrSpec>;
#[doc = "Field `CMPL` reader - RW,CMPL"]
pub type CmplR = crate::FieldReader<u32>;
#[doc = "Field `CMPL` writer - RW,CMPL"]
pub type CmplW<'a, REG> = crate::FieldWriter<'a, REG, 32, u32>;
impl R {
    #[doc = "Bits 0:31 - RW,CMPL"]
    #[inline(always)]
    pub fn cmpl(&self) -> CmplR {
        CmplR::new(self.bits)
    }
}
impl W {
    #[doc = "Bits 0:31 - RW,CMPL"]
    #[inline(always)]
    pub fn cmpl(&mut self) -> CmplW<'_, R32StkCmplrSpec> {
        CmplW::new(self, 0)
    }
}
#[doc = "Systick compare low register\n\nYou can [`read`](crate::Reg::read) this register and get [`r32_stk_cmplr::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`r32_stk_cmplr::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct R32StkCmplrSpec;
impl crate::RegisterSpec for R32StkCmplrSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`r32_stk_cmplr::R`](R) reader structure"]
impl crate::Readable for R32StkCmplrSpec {}
#[doc = "`write(|w| ..)` method takes [`r32_stk_cmplr::W`](W) writer structure"]
impl crate::Writable for R32StkCmplrSpec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets R32_STK_CMPLR to value 0"]
impl crate::Resettable for R32StkCmplrSpec {}
