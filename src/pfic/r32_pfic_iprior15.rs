#[doc = "Register `R32_PFIC_IPRIOR15` reader"]
pub type R = crate::R<R32PficIprior15Spec>;
#[doc = "Register `R32_PFIC_IPRIOR15` writer"]
pub type W = crate::W<R32PficIprior15Spec>;
#[doc = "Field `IPRIOR15` reader - RW,IPRIOR15"]
pub type Iprior15R = crate::FieldReader<u32>;
#[doc = "Field `IPRIOR15` writer - RW,IPRIOR15"]
pub type Iprior15W<'a, REG> = crate::FieldWriter<'a, REG, 32, u32>;
impl R {
    #[doc = "Bits 0:31 - RW,IPRIOR15"]
    #[inline(always)]
    pub fn iprior15(&self) -> Iprior15R {
        Iprior15R::new(self.bits)
    }
}
impl W {
    #[doc = "Bits 0:31 - RW,IPRIOR15"]
    #[inline(always)]
    pub fn iprior15(&mut self) -> Iprior15W<'_, R32PficIprior15Spec> {
        Iprior15W::new(self, 0)
    }
}
#[doc = "Interrupt Priority configuration Register\n\nYou can [`read`](crate::Reg::read) this register and get [`r32_pfic_iprior15::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`r32_pfic_iprior15::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct R32PficIprior15Spec;
impl crate::RegisterSpec for R32PficIprior15Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`r32_pfic_iprior15::R`](R) reader structure"]
impl crate::Readable for R32PficIprior15Spec {}
#[doc = "`write(|w| ..)` method takes [`r32_pfic_iprior15::W`](W) writer structure"]
impl crate::Writable for R32PficIprior15Spec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets R32_PFIC_IPRIOR15 to value 0"]
impl crate::Resettable for R32PficIprior15Spec {}
