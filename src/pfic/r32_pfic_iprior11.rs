#[doc = "Register `R32_PFIC_IPRIOR11` reader"]
pub type R = crate::R<R32PficIprior11Spec>;
#[doc = "Register `R32_PFIC_IPRIOR11` writer"]
pub type W = crate::W<R32PficIprior11Spec>;
#[doc = "Field `IPRIOR11` reader - RW,IPRIOR11"]
pub type Iprior11R = crate::FieldReader<u32>;
#[doc = "Field `IPRIOR11` writer - RW,IPRIOR11"]
pub type Iprior11W<'a, REG> = crate::FieldWriter<'a, REG, 32, u32>;
impl R {
    #[doc = "Bits 0:31 - RW,IPRIOR11"]
    #[inline(always)]
    pub fn iprior11(&self) -> Iprior11R {
        Iprior11R::new(self.bits)
    }
}
impl W {
    #[doc = "Bits 0:31 - RW,IPRIOR11"]
    #[inline(always)]
    pub fn iprior11(&mut self) -> Iprior11W<'_, R32PficIprior11Spec> {
        Iprior11W::new(self, 0)
    }
}
#[doc = "Interrupt Priority configuration Register\n\nYou can [`read`](crate::Reg::read) this register and get [`r32_pfic_iprior11::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`r32_pfic_iprior11::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct R32PficIprior11Spec;
impl crate::RegisterSpec for R32PficIprior11Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`r32_pfic_iprior11::R`](R) reader structure"]
impl crate::Readable for R32PficIprior11Spec {}
#[doc = "`write(|w| ..)` method takes [`r32_pfic_iprior11::W`](W) writer structure"]
impl crate::Writable for R32PficIprior11Spec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets R32_PFIC_IPRIOR11 to value 0"]
impl crate::Resettable for R32PficIprior11Spec {}
