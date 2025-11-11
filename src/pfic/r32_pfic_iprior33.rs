#[doc = "Register `R32_PFIC_IPRIOR33` reader"]
pub type R = crate::R<R32PficIprior33Spec>;
#[doc = "Register `R32_PFIC_IPRIOR33` writer"]
pub type W = crate::W<R32PficIprior33Spec>;
#[doc = "Field `IPRIOR33` reader - RW,IPRIOR33"]
pub type Iprior33R = crate::FieldReader<u32>;
#[doc = "Field `IPRIOR33` writer - RW,IPRIOR33"]
pub type Iprior33W<'a, REG> = crate::FieldWriter<'a, REG, 32, u32>;
impl R {
    #[doc = "Bits 0:31 - RW,IPRIOR33"]
    #[inline(always)]
    pub fn iprior33(&self) -> Iprior33R {
        Iprior33R::new(self.bits)
    }
}
impl W {
    #[doc = "Bits 0:31 - RW,IPRIOR33"]
    #[inline(always)]
    pub fn iprior33(&mut self) -> Iprior33W<'_, R32PficIprior33Spec> {
        Iprior33W::new(self, 0)
    }
}
#[doc = "Interrupt Priority configuration Register\n\nYou can [`read`](crate::Reg::read) this register and get [`r32_pfic_iprior33::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`r32_pfic_iprior33::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct R32PficIprior33Spec;
impl crate::RegisterSpec for R32PficIprior33Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`r32_pfic_iprior33::R`](R) reader structure"]
impl crate::Readable for R32PficIprior33Spec {}
#[doc = "`write(|w| ..)` method takes [`r32_pfic_iprior33::W`](W) writer structure"]
impl crate::Writable for R32PficIprior33Spec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets R32_PFIC_IPRIOR33 to value 0"]
impl crate::Resettable for R32PficIprior33Spec {}
