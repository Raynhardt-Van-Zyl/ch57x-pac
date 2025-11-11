#[doc = "Register `R32_PFIC_IPRIOR12` reader"]
pub type R = crate::R<R32PficIprior12Spec>;
#[doc = "Register `R32_PFIC_IPRIOR12` writer"]
pub type W = crate::W<R32PficIprior12Spec>;
#[doc = "Field `IPRIOR12` reader - RW,IPRIOR12"]
pub type Iprior12R = crate::FieldReader<u32>;
#[doc = "Field `IPRIOR12` writer - RW,IPRIOR12"]
pub type Iprior12W<'a, REG> = crate::FieldWriter<'a, REG, 32, u32>;
impl R {
    #[doc = "Bits 0:31 - RW,IPRIOR12"]
    #[inline(always)]
    pub fn iprior12(&self) -> Iprior12R {
        Iprior12R::new(self.bits)
    }
}
impl W {
    #[doc = "Bits 0:31 - RW,IPRIOR12"]
    #[inline(always)]
    pub fn iprior12(&mut self) -> Iprior12W<'_, R32PficIprior12Spec> {
        Iprior12W::new(self, 0)
    }
}
#[doc = "Interrupt Priority configuration Register\n\nYou can [`read`](crate::Reg::read) this register and get [`r32_pfic_iprior12::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`r32_pfic_iprior12::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct R32PficIprior12Spec;
impl crate::RegisterSpec for R32PficIprior12Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`r32_pfic_iprior12::R`](R) reader structure"]
impl crate::Readable for R32PficIprior12Spec {}
#[doc = "`write(|w| ..)` method takes [`r32_pfic_iprior12::W`](W) writer structure"]
impl crate::Writable for R32PficIprior12Spec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets R32_PFIC_IPRIOR12 to value 0"]
impl crate::Resettable for R32PficIprior12Spec {}
