#[doc = "Register `R32_PFIC_IPRIOR13` reader"]
pub type R = crate::R<R32PficIprior13Spec>;
#[doc = "Register `R32_PFIC_IPRIOR13` writer"]
pub type W = crate::W<R32PficIprior13Spec>;
#[doc = "Field `IPRIOR13` reader - RW,IPRIOR13"]
pub type Iprior13R = crate::FieldReader<u32>;
#[doc = "Field `IPRIOR13` writer - RW,IPRIOR13"]
pub type Iprior13W<'a, REG> = crate::FieldWriter<'a, REG, 32, u32>;
impl R {
    #[doc = "Bits 0:31 - RW,IPRIOR13"]
    #[inline(always)]
    pub fn iprior13(&self) -> Iprior13R {
        Iprior13R::new(self.bits)
    }
}
impl W {
    #[doc = "Bits 0:31 - RW,IPRIOR13"]
    #[inline(always)]
    pub fn iprior13(&mut self) -> Iprior13W<'_, R32PficIprior13Spec> {
        Iprior13W::new(self, 0)
    }
}
#[doc = "Interrupt Priority configuration Register\n\nYou can [`read`](crate::Reg::read) this register and get [`r32_pfic_iprior13::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`r32_pfic_iprior13::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct R32PficIprior13Spec;
impl crate::RegisterSpec for R32PficIprior13Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`r32_pfic_iprior13::R`](R) reader structure"]
impl crate::Readable for R32PficIprior13Spec {}
#[doc = "`write(|w| ..)` method takes [`r32_pfic_iprior13::W`](W) writer structure"]
impl crate::Writable for R32PficIprior13Spec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets R32_PFIC_IPRIOR13 to value 0"]
impl crate::Resettable for R32PficIprior13Spec {}
