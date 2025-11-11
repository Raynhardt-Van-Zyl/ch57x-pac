#[doc = "Register `R32_PFIC_IPRIOR22` reader"]
pub type R = crate::R<R32PficIprior22Spec>;
#[doc = "Register `R32_PFIC_IPRIOR22` writer"]
pub type W = crate::W<R32PficIprior22Spec>;
#[doc = "Field `IPRIOR22` reader - RW,IPRIOR22"]
pub type Iprior22R = crate::FieldReader<u32>;
#[doc = "Field `IPRIOR22` writer - RW,IPRIOR22"]
pub type Iprior22W<'a, REG> = crate::FieldWriter<'a, REG, 32, u32>;
impl R {
    #[doc = "Bits 0:31 - RW,IPRIOR22"]
    #[inline(always)]
    pub fn iprior22(&self) -> Iprior22R {
        Iprior22R::new(self.bits)
    }
}
impl W {
    #[doc = "Bits 0:31 - RW,IPRIOR22"]
    #[inline(always)]
    pub fn iprior22(&mut self) -> Iprior22W<'_, R32PficIprior22Spec> {
        Iprior22W::new(self, 0)
    }
}
#[doc = "Interrupt Priority configuration Register\n\nYou can [`read`](crate::Reg::read) this register and get [`r32_pfic_iprior22::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`r32_pfic_iprior22::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct R32PficIprior22Spec;
impl crate::RegisterSpec for R32PficIprior22Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`r32_pfic_iprior22::R`](R) reader structure"]
impl crate::Readable for R32PficIprior22Spec {}
#[doc = "`write(|w| ..)` method takes [`r32_pfic_iprior22::W`](W) writer structure"]
impl crate::Writable for R32PficIprior22Spec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets R32_PFIC_IPRIOR22 to value 0"]
impl crate::Resettable for R32PficIprior22Spec {}
