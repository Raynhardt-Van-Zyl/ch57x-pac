#[doc = "Register `R32_PFIC_IPRIOR30` reader"]
pub type R = crate::R<R32PficIprior30Spec>;
#[doc = "Register `R32_PFIC_IPRIOR30` writer"]
pub type W = crate::W<R32PficIprior30Spec>;
#[doc = "Field `IPRIOR30` reader - RW,IPRIOR30"]
pub type Iprior30R = crate::FieldReader<u32>;
#[doc = "Field `IPRIOR30` writer - RW,IPRIOR30"]
pub type Iprior30W<'a, REG> = crate::FieldWriter<'a, REG, 32, u32>;
impl R {
    #[doc = "Bits 0:31 - RW,IPRIOR30"]
    #[inline(always)]
    pub fn iprior30(&self) -> Iprior30R {
        Iprior30R::new(self.bits)
    }
}
impl W {
    #[doc = "Bits 0:31 - RW,IPRIOR30"]
    #[inline(always)]
    pub fn iprior30(&mut self) -> Iprior30W<'_, R32PficIprior30Spec> {
        Iprior30W::new(self, 0)
    }
}
#[doc = "Interrupt Priority configuration Register\n\nYou can [`read`](crate::Reg::read) this register and get [`r32_pfic_iprior30::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`r32_pfic_iprior30::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct R32PficIprior30Spec;
impl crate::RegisterSpec for R32PficIprior30Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`r32_pfic_iprior30::R`](R) reader structure"]
impl crate::Readable for R32PficIprior30Spec {}
#[doc = "`write(|w| ..)` method takes [`r32_pfic_iprior30::W`](W) writer structure"]
impl crate::Writable for R32PficIprior30Spec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets R32_PFIC_IPRIOR30 to value 0"]
impl crate::Resettable for R32PficIprior30Spec {}
