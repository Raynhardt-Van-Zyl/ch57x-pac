#[doc = "Register `R32_STK_CNTL` reader"]
pub type R = crate::R<R32StkCntlSpec>;
#[doc = "Register `R32_STK_CNTL` writer"]
pub type W = crate::W<R32StkCntlSpec>;
#[doc = "Field `CNTL` reader - RW,CNTL"]
pub type CntlR = crate::FieldReader<u32>;
#[doc = "Field `CNTL` writer - RW,CNTL"]
pub type CntlW<'a, REG> = crate::FieldWriter<'a, REG, 32, u32>;
impl R {
    #[doc = "Bits 0:31 - RW,CNTL"]
    #[inline(always)]
    pub fn cntl(&self) -> CntlR {
        CntlR::new(self.bits)
    }
}
impl W {
    #[doc = "Bits 0:31 - RW,CNTL"]
    #[inline(always)]
    pub fn cntl(&mut self) -> CntlW<'_, R32StkCntlSpec> {
        CntlW::new(self, 0)
    }
}
#[doc = "Systick counter low register\n\nYou can [`read`](crate::Reg::read) this register and get [`r32_stk_cntl::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`r32_stk_cntl::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct R32StkCntlSpec;
impl crate::RegisterSpec for R32StkCntlSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`r32_stk_cntl::R`](R) reader structure"]
impl crate::Readable for R32StkCntlSpec {}
#[doc = "`write(|w| ..)` method takes [`r32_stk_cntl::W`](W) writer structure"]
impl crate::Writable for R32StkCntlSpec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets R32_STK_CNTL to value 0"]
impl crate::Resettable for R32StkCntlSpec {}
