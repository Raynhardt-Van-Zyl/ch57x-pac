#[doc = "Register `R32_PFIC_IPRIOR27` reader"]
pub type R = crate::R<R32PficIprior27Spec>;
#[doc = "Register `R32_PFIC_IPRIOR27` writer"]
pub type W = crate::W<R32PficIprior27Spec>;
#[doc = "Field `IPRIOR27` reader - RW,IPRIOR27"]
pub type Iprior27R = crate::FieldReader<u32>;
#[doc = "Field `IPRIOR27` writer - RW,IPRIOR27"]
pub type Iprior27W<'a, REG> = crate::FieldWriter<'a, REG, 32, u32>;
impl R {
    #[doc = "Bits 0:31 - RW,IPRIOR27"]
    #[inline(always)]
    pub fn iprior27(&self) -> Iprior27R {
        Iprior27R::new(self.bits)
    }
}
impl W {
    #[doc = "Bits 0:31 - RW,IPRIOR27"]
    #[inline(always)]
    pub fn iprior27(&mut self) -> Iprior27W<'_, R32PficIprior27Spec> {
        Iprior27W::new(self, 0)
    }
}
#[doc = "Interrupt Priority configuration Register\n\nYou can [`read`](crate::Reg::read) this register and get [`r32_pfic_iprior27::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`r32_pfic_iprior27::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct R32PficIprior27Spec;
impl crate::RegisterSpec for R32PficIprior27Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`r32_pfic_iprior27::R`](R) reader structure"]
impl crate::Readable for R32PficIprior27Spec {}
#[doc = "`write(|w| ..)` method takes [`r32_pfic_iprior27::W`](W) writer structure"]
impl crate::Writable for R32PficIprior27Spec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets R32_PFIC_IPRIOR27 to value 0"]
impl crate::Resettable for R32PficIprior27Spec {}
