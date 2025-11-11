#[doc = "Register `R32_PFIC_IPRIOR44` reader"]
pub type R = crate::R<R32PficIprior44Spec>;
#[doc = "Register `R32_PFIC_IPRIOR44` writer"]
pub type W = crate::W<R32PficIprior44Spec>;
#[doc = "Field `IPRIOR44` reader - RW,IPRIOR44"]
pub type Iprior44R = crate::FieldReader<u32>;
#[doc = "Field `IPRIOR44` writer - RW,IPRIOR44"]
pub type Iprior44W<'a, REG> = crate::FieldWriter<'a, REG, 32, u32>;
impl R {
    #[doc = "Bits 0:31 - RW,IPRIOR44"]
    #[inline(always)]
    pub fn iprior44(&self) -> Iprior44R {
        Iprior44R::new(self.bits)
    }
}
impl W {
    #[doc = "Bits 0:31 - RW,IPRIOR44"]
    #[inline(always)]
    pub fn iprior44(&mut self) -> Iprior44W<'_, R32PficIprior44Spec> {
        Iprior44W::new(self, 0)
    }
}
#[doc = "Interrupt Priority configuration Register\n\nYou can [`read`](crate::Reg::read) this register and get [`r32_pfic_iprior44::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`r32_pfic_iprior44::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct R32PficIprior44Spec;
impl crate::RegisterSpec for R32PficIprior44Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`r32_pfic_iprior44::R`](R) reader structure"]
impl crate::Readable for R32PficIprior44Spec {}
#[doc = "`write(|w| ..)` method takes [`r32_pfic_iprior44::W`](W) writer structure"]
impl crate::Writable for R32PficIprior44Spec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets R32_PFIC_IPRIOR44 to value 0"]
impl crate::Resettable for R32PficIprior44Spec {}
