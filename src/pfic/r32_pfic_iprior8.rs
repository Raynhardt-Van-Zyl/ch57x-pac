#[doc = "Register `R32_PFIC_IPRIOR8` reader"]
pub type R = crate::R<R32PficIprior8Spec>;
#[doc = "Register `R32_PFIC_IPRIOR8` writer"]
pub type W = crate::W<R32PficIprior8Spec>;
#[doc = "Field `IPRIOR8` reader - >RW,IPRIOR8"]
pub type Iprior8R = crate::FieldReader<u32>;
#[doc = "Field `IPRIOR8` writer - >RW,IPRIOR8"]
pub type Iprior8W<'a, REG> = crate::FieldWriter<'a, REG, 32, u32>;
impl R {
    #[doc = "Bits 0:31 - >RW,IPRIOR8"]
    #[inline(always)]
    pub fn iprior8(&self) -> Iprior8R {
        Iprior8R::new(self.bits)
    }
}
impl W {
    #[doc = "Bits 0:31 - >RW,IPRIOR8"]
    #[inline(always)]
    pub fn iprior8(&mut self) -> Iprior8W<'_, R32PficIprior8Spec> {
        Iprior8W::new(self, 0)
    }
}
#[doc = "Interrupt Priority configuration Register\n\nYou can [`read`](crate::Reg::read) this register and get [`r32_pfic_iprior8::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`r32_pfic_iprior8::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct R32PficIprior8Spec;
impl crate::RegisterSpec for R32PficIprior8Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`r32_pfic_iprior8::R`](R) reader structure"]
impl crate::Readable for R32PficIprior8Spec {}
#[doc = "`write(|w| ..)` method takes [`r32_pfic_iprior8::W`](W) writer structure"]
impl crate::Writable for R32PficIprior8Spec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets R32_PFIC_IPRIOR8 to value 0"]
impl crate::Resettable for R32PficIprior8Spec {}
