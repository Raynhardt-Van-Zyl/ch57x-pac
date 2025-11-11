#[doc = "Register `R32_PFIC_IPRIOR57` reader"]
pub type R = crate::R<R32PficIprior57Spec>;
#[doc = "Register `R32_PFIC_IPRIOR57` writer"]
pub type W = crate::W<R32PficIprior57Spec>;
#[doc = "Field `IPRIOR57` reader - RW,IPRIOR57"]
pub type Iprior57R = crate::FieldReader<u32>;
#[doc = "Field `IPRIOR57` writer - RW,IPRIOR57"]
pub type Iprior57W<'a, REG> = crate::FieldWriter<'a, REG, 32, u32>;
impl R {
    #[doc = "Bits 0:31 - RW,IPRIOR57"]
    #[inline(always)]
    pub fn iprior57(&self) -> Iprior57R {
        Iprior57R::new(self.bits)
    }
}
impl W {
    #[doc = "Bits 0:31 - RW,IPRIOR57"]
    #[inline(always)]
    pub fn iprior57(&mut self) -> Iprior57W<'_, R32PficIprior57Spec> {
        Iprior57W::new(self, 0)
    }
}
#[doc = "Interrupt Priority configuration Register\n\nYou can [`read`](crate::Reg::read) this register and get [`r32_pfic_iprior57::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`r32_pfic_iprior57::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct R32PficIprior57Spec;
impl crate::RegisterSpec for R32PficIprior57Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`r32_pfic_iprior57::R`](R) reader structure"]
impl crate::Readable for R32PficIprior57Spec {}
#[doc = "`write(|w| ..)` method takes [`r32_pfic_iprior57::W`](W) writer structure"]
impl crate::Writable for R32PficIprior57Spec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets R32_PFIC_IPRIOR57 to value 0"]
impl crate::Resettable for R32PficIprior57Spec {}
