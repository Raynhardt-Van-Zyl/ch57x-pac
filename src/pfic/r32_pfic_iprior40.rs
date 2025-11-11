#[doc = "Register `R32_PFIC_IPRIOR40` reader"]
pub type R = crate::R<R32PficIprior40Spec>;
#[doc = "Register `R32_PFIC_IPRIOR40` writer"]
pub type W = crate::W<R32PficIprior40Spec>;
#[doc = "Field `IPRIOR40` reader - RW,IPRIOR40"]
pub type Iprior40R = crate::FieldReader<u32>;
#[doc = "Field `IPRIOR40` writer - RW,IPRIOR40"]
pub type Iprior40W<'a, REG> = crate::FieldWriter<'a, REG, 32, u32>;
impl R {
    #[doc = "Bits 0:31 - RW,IPRIOR40"]
    #[inline(always)]
    pub fn iprior40(&self) -> Iprior40R {
        Iprior40R::new(self.bits)
    }
}
impl W {
    #[doc = "Bits 0:31 - RW,IPRIOR40"]
    #[inline(always)]
    pub fn iprior40(&mut self) -> Iprior40W<'_, R32PficIprior40Spec> {
        Iprior40W::new(self, 0)
    }
}
#[doc = "Interrupt Priority configuration Register\n\nYou can [`read`](crate::Reg::read) this register and get [`r32_pfic_iprior40::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`r32_pfic_iprior40::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct R32PficIprior40Spec;
impl crate::RegisterSpec for R32PficIprior40Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`r32_pfic_iprior40::R`](R) reader structure"]
impl crate::Readable for R32PficIprior40Spec {}
#[doc = "`write(|w| ..)` method takes [`r32_pfic_iprior40::W`](W) writer structure"]
impl crate::Writable for R32PficIprior40Spec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets R32_PFIC_IPRIOR40 to value 0"]
impl crate::Resettable for R32PficIprior40Spec {}
