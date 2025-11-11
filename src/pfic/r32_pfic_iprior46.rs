#[doc = "Register `R32_PFIC_IPRIOR46` reader"]
pub type R = crate::R<R32PficIprior46Spec>;
#[doc = "Register `R32_PFIC_IPRIOR46` writer"]
pub type W = crate::W<R32PficIprior46Spec>;
#[doc = "Field `IPRIOR46` reader - RW,IPRIOR46"]
pub type Iprior46R = crate::FieldReader<u32>;
#[doc = "Field `IPRIOR46` writer - RW,IPRIOR46"]
pub type Iprior46W<'a, REG> = crate::FieldWriter<'a, REG, 32, u32>;
impl R {
    #[doc = "Bits 0:31 - RW,IPRIOR46"]
    #[inline(always)]
    pub fn iprior46(&self) -> Iprior46R {
        Iprior46R::new(self.bits)
    }
}
impl W {
    #[doc = "Bits 0:31 - RW,IPRIOR46"]
    #[inline(always)]
    pub fn iprior46(&mut self) -> Iprior46W<'_, R32PficIprior46Spec> {
        Iprior46W::new(self, 0)
    }
}
#[doc = "Interrupt Priority configuration Register\n\nYou can [`read`](crate::Reg::read) this register and get [`r32_pfic_iprior46::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`r32_pfic_iprior46::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct R32PficIprior46Spec;
impl crate::RegisterSpec for R32PficIprior46Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`r32_pfic_iprior46::R`](R) reader structure"]
impl crate::Readable for R32PficIprior46Spec {}
#[doc = "`write(|w| ..)` method takes [`r32_pfic_iprior46::W`](W) writer structure"]
impl crate::Writable for R32PficIprior46Spec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets R32_PFIC_IPRIOR46 to value 0"]
impl crate::Resettable for R32PficIprior46Spec {}
