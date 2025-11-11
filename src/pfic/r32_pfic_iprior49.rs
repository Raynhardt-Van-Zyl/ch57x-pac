#[doc = "Register `R32_PFIC_IPRIOR49` reader"]
pub type R = crate::R<R32PficIprior49Spec>;
#[doc = "Register `R32_PFIC_IPRIOR49` writer"]
pub type W = crate::W<R32PficIprior49Spec>;
#[doc = "Field `IPRIOR49` reader - RW,IPRIOR49"]
pub type Iprior49R = crate::FieldReader<u32>;
#[doc = "Field `IPRIOR49` writer - RW,IPRIOR49"]
pub type Iprior49W<'a, REG> = crate::FieldWriter<'a, REG, 32, u32>;
impl R {
    #[doc = "Bits 0:31 - RW,IPRIOR49"]
    #[inline(always)]
    pub fn iprior49(&self) -> Iprior49R {
        Iprior49R::new(self.bits)
    }
}
impl W {
    #[doc = "Bits 0:31 - RW,IPRIOR49"]
    #[inline(always)]
    pub fn iprior49(&mut self) -> Iprior49W<'_, R32PficIprior49Spec> {
        Iprior49W::new(self, 0)
    }
}
#[doc = "Interrupt Priority configuration Register\n\nYou can [`read`](crate::Reg::read) this register and get [`r32_pfic_iprior49::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`r32_pfic_iprior49::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct R32PficIprior49Spec;
impl crate::RegisterSpec for R32PficIprior49Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`r32_pfic_iprior49::R`](R) reader structure"]
impl crate::Readable for R32PficIprior49Spec {}
#[doc = "`write(|w| ..)` method takes [`r32_pfic_iprior49::W`](W) writer structure"]
impl crate::Writable for R32PficIprior49Spec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets R32_PFIC_IPRIOR49 to value 0"]
impl crate::Resettable for R32PficIprior49Spec {}
