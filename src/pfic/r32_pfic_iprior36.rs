#[doc = "Register `R32_PFIC_IPRIOR36` reader"]
pub type R = crate::R<R32PficIprior36Spec>;
#[doc = "Register `R32_PFIC_IPRIOR36` writer"]
pub type W = crate::W<R32PficIprior36Spec>;
#[doc = "Field `IPRIOR36` reader - RW,IPRIOR36"]
pub type Iprior36R = crate::FieldReader<u32>;
#[doc = "Field `IPRIOR36` writer - RW,IPRIOR36"]
pub type Iprior36W<'a, REG> = crate::FieldWriter<'a, REG, 32, u32>;
impl R {
    #[doc = "Bits 0:31 - RW,IPRIOR36"]
    #[inline(always)]
    pub fn iprior36(&self) -> Iprior36R {
        Iprior36R::new(self.bits)
    }
}
impl W {
    #[doc = "Bits 0:31 - RW,IPRIOR36"]
    #[inline(always)]
    pub fn iprior36(&mut self) -> Iprior36W<'_, R32PficIprior36Spec> {
        Iprior36W::new(self, 0)
    }
}
#[doc = "Interrupt Priority configuration Register\n\nYou can [`read`](crate::Reg::read) this register and get [`r32_pfic_iprior36::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`r32_pfic_iprior36::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct R32PficIprior36Spec;
impl crate::RegisterSpec for R32PficIprior36Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`r32_pfic_iprior36::R`](R) reader structure"]
impl crate::Readable for R32PficIprior36Spec {}
#[doc = "`write(|w| ..)` method takes [`r32_pfic_iprior36::W`](W) writer structure"]
impl crate::Writable for R32PficIprior36Spec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets R32_PFIC_IPRIOR36 to value 0"]
impl crate::Resettable for R32PficIprior36Spec {}
