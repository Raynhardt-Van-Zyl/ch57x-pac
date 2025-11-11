#[doc = "Register `R32_PFIC_IPRIOR26` reader"]
pub type R = crate::R<R32PficIprior26Spec>;
#[doc = "Register `R32_PFIC_IPRIOR26` writer"]
pub type W = crate::W<R32PficIprior26Spec>;
#[doc = "Field `IPRIOR26` reader - RW,IPRIOR26"]
pub type Iprior26R = crate::FieldReader<u32>;
#[doc = "Field `IPRIOR26` writer - RW,IPRIOR26"]
pub type Iprior26W<'a, REG> = crate::FieldWriter<'a, REG, 32, u32>;
impl R {
    #[doc = "Bits 0:31 - RW,IPRIOR26"]
    #[inline(always)]
    pub fn iprior26(&self) -> Iprior26R {
        Iprior26R::new(self.bits)
    }
}
impl W {
    #[doc = "Bits 0:31 - RW,IPRIOR26"]
    #[inline(always)]
    pub fn iprior26(&mut self) -> Iprior26W<'_, R32PficIprior26Spec> {
        Iprior26W::new(self, 0)
    }
}
#[doc = "Interrupt Priority configuration Register\n\nYou can [`read`](crate::Reg::read) this register and get [`r32_pfic_iprior26::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`r32_pfic_iprior26::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct R32PficIprior26Spec;
impl crate::RegisterSpec for R32PficIprior26Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`r32_pfic_iprior26::R`](R) reader structure"]
impl crate::Readable for R32PficIprior26Spec {}
#[doc = "`write(|w| ..)` method takes [`r32_pfic_iprior26::W`](W) writer structure"]
impl crate::Writable for R32PficIprior26Spec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets R32_PFIC_IPRIOR26 to value 0"]
impl crate::Resettable for R32PficIprior26Spec {}
