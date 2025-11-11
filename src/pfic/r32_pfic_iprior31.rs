#[doc = "Register `R32_PFIC_IPRIOR31` reader"]
pub type R = crate::R<R32PficIprior31Spec>;
#[doc = "Register `R32_PFIC_IPRIOR31` writer"]
pub type W = crate::W<R32PficIprior31Spec>;
#[doc = "Field `IPRIOR31` reader - RW,IPRIOR31"]
pub type Iprior31R = crate::FieldReader<u32>;
#[doc = "Field `IPRIOR31` writer - RW,IPRIOR31"]
pub type Iprior31W<'a, REG> = crate::FieldWriter<'a, REG, 32, u32>;
impl R {
    #[doc = "Bits 0:31 - RW,IPRIOR31"]
    #[inline(always)]
    pub fn iprior31(&self) -> Iprior31R {
        Iprior31R::new(self.bits)
    }
}
impl W {
    #[doc = "Bits 0:31 - RW,IPRIOR31"]
    #[inline(always)]
    pub fn iprior31(&mut self) -> Iprior31W<'_, R32PficIprior31Spec> {
        Iprior31W::new(self, 0)
    }
}
#[doc = "Interrupt Priority configuration Register\n\nYou can [`read`](crate::Reg::read) this register and get [`r32_pfic_iprior31::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`r32_pfic_iprior31::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct R32PficIprior31Spec;
impl crate::RegisterSpec for R32PficIprior31Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`r32_pfic_iprior31::R`](R) reader structure"]
impl crate::Readable for R32PficIprior31Spec {}
#[doc = "`write(|w| ..)` method takes [`r32_pfic_iprior31::W`](W) writer structure"]
impl crate::Writable for R32PficIprior31Spec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets R32_PFIC_IPRIOR31 to value 0"]
impl crate::Resettable for R32PficIprior31Spec {}
