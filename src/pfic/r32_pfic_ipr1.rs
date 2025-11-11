#[doc = "Register `R32_PFIC_IPR1` reader"]
pub type R = crate::R<R32PficIpr1Spec>;
#[doc = "Register `R32_PFIC_IPR1` writer"]
pub type W = crate::W<R32PficIpr1Spec>;
#[doc = "Field `PENDSTA` reader - PENDSTA"]
pub type PendstaR = crate::FieldReader<u32>;
#[doc = "Field `PENDSTA` writer - PENDSTA"]
pub type PendstaW<'a, REG> = crate::FieldWriter<'a, REG, 20, u32>;
impl R {
    #[doc = "Bits 12:31 - PENDSTA"]
    #[inline(always)]
    pub fn pendsta(&self) -> PendstaR {
        PendstaR::new((self.bits >> 12) & 0x000f_ffff)
    }
}
impl W {
    #[doc = "Bits 12:31 - PENDSTA"]
    #[inline(always)]
    pub fn pendsta(&mut self) -> PendstaW<'_, R32PficIpr1Spec> {
        PendstaW::new(self, 12)
    }
}
#[doc = "RO,Interrupt Pending Register\n\nYou can [`read`](crate::Reg::read) this register and get [`r32_pfic_ipr1::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`r32_pfic_ipr1::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct R32PficIpr1Spec;
impl crate::RegisterSpec for R32PficIpr1Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`r32_pfic_ipr1::R`](R) reader structure"]
impl crate::Readable for R32PficIpr1Spec {}
#[doc = "`write(|w| ..)` method takes [`r32_pfic_ipr1::W`](W) writer structure"]
impl crate::Writable for R32PficIpr1Spec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets R32_PFIC_IPR1 to value 0"]
impl crate::Resettable for R32PficIpr1Spec {}
