#[doc = "Register `R32_PFIC_IPRR1` reader"]
pub type R = crate::R<R32PficIprr1Spec>;
#[doc = "Register `R32_PFIC_IPRR1` writer"]
pub type W = crate::W<R32PficIprr1Spec>;
#[doc = "Field `PENDRESET` reader - RW1,PENDRESET"]
pub type PendresetR = crate::FieldReader<u32>;
#[doc = "Field `PENDRESET` writer - RW1,PENDRESET"]
pub type PendresetW<'a, REG> = crate::FieldWriter<'a, REG, 20, u32>;
impl R {
    #[doc = "Bits 12:31 - RW1,PENDRESET"]
    #[inline(always)]
    pub fn pendreset(&self) -> PendresetR {
        PendresetR::new((self.bits >> 12) & 0x000f_ffff)
    }
}
impl W {
    #[doc = "Bits 12:31 - RW1,PENDRESET"]
    #[inline(always)]
    pub fn pendreset(&mut self) -> PendresetW<'_, R32PficIprr1Spec> {
        PendresetW::new(self, 12)
    }
}
#[doc = "Interrupt Pending Clear Register\n\nYou can [`read`](crate::Reg::read) this register and get [`r32_pfic_iprr1::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`r32_pfic_iprr1::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct R32PficIprr1Spec;
impl crate::RegisterSpec for R32PficIprr1Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`r32_pfic_iprr1::R`](R) reader structure"]
impl crate::Readable for R32PficIprr1Spec {}
#[doc = "`write(|w| ..)` method takes [`r32_pfic_iprr1::W`](W) writer structure"]
impl crate::Writable for R32PficIprr1Spec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets R32_PFIC_IPRR1 to value 0"]
impl crate::Resettable for R32PficIprr1Spec {}
