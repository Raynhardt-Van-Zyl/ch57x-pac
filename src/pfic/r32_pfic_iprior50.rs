#[doc = "Register `R32_PFIC_IPRIOR50` reader"]
pub type R = crate::R<R32PficIprior50Spec>;
#[doc = "Register `R32_PFIC_IPRIOR50` writer"]
pub type W = crate::W<R32PficIprior50Spec>;
#[doc = "Field `IPRIOR50` reader - RW,IPRIOR50"]
pub type Iprior50R = crate::FieldReader<u32>;
#[doc = "Field `IPRIOR50` writer - RW,IPRIOR50"]
pub type Iprior50W<'a, REG> = crate::FieldWriter<'a, REG, 32, u32>;
impl R {
    #[doc = "Bits 0:31 - RW,IPRIOR50"]
    #[inline(always)]
    pub fn iprior50(&self) -> Iprior50R {
        Iprior50R::new(self.bits)
    }
}
impl W {
    #[doc = "Bits 0:31 - RW,IPRIOR50"]
    #[inline(always)]
    pub fn iprior50(&mut self) -> Iprior50W<'_, R32PficIprior50Spec> {
        Iprior50W::new(self, 0)
    }
}
#[doc = "Interrupt Priority configuration Register\n\nYou can [`read`](crate::Reg::read) this register and get [`r32_pfic_iprior50::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`r32_pfic_iprior50::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct R32PficIprior50Spec;
impl crate::RegisterSpec for R32PficIprior50Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`r32_pfic_iprior50::R`](R) reader structure"]
impl crate::Readable for R32PficIprior50Spec {}
#[doc = "`write(|w| ..)` method takes [`r32_pfic_iprior50::W`](W) writer structure"]
impl crate::Writable for R32PficIprior50Spec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets R32_PFIC_IPRIOR50 to value 0"]
impl crate::Resettable for R32PficIprior50Spec {}
