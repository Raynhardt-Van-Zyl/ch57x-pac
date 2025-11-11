#[doc = "Register `R32_PFIC_IPRIOR48` reader"]
pub type R = crate::R<R32PficIprior48Spec>;
#[doc = "Register `R32_PFIC_IPRIOR48` writer"]
pub type W = crate::W<R32PficIprior48Spec>;
#[doc = "Field `IPRIOR48` reader - RW,IPRIOR48"]
pub type Iprior48R = crate::FieldReader<u32>;
#[doc = "Field `IPRIOR48` writer - RW,IPRIOR48"]
pub type Iprior48W<'a, REG> = crate::FieldWriter<'a, REG, 32, u32>;
impl R {
    #[doc = "Bits 0:31 - RW,IPRIOR48"]
    #[inline(always)]
    pub fn iprior48(&self) -> Iprior48R {
        Iprior48R::new(self.bits)
    }
}
impl W {
    #[doc = "Bits 0:31 - RW,IPRIOR48"]
    #[inline(always)]
    pub fn iprior48(&mut self) -> Iprior48W<'_, R32PficIprior48Spec> {
        Iprior48W::new(self, 0)
    }
}
#[doc = "Interrupt Priority configuration Register\n\nYou can [`read`](crate::Reg::read) this register and get [`r32_pfic_iprior48::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`r32_pfic_iprior48::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct R32PficIprior48Spec;
impl crate::RegisterSpec for R32PficIprior48Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`r32_pfic_iprior48::R`](R) reader structure"]
impl crate::Readable for R32PficIprior48Spec {}
#[doc = "`write(|w| ..)` method takes [`r32_pfic_iprior48::W`](W) writer structure"]
impl crate::Writable for R32PficIprior48Spec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets R32_PFIC_IPRIOR48 to value 0"]
impl crate::Resettable for R32PficIprior48Spec {}
