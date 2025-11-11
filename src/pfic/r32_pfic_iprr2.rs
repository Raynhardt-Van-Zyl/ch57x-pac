#[doc = "Register `R32_PFIC_IPRR2` reader"]
pub type R = crate::R<R32PficIprr2Spec>;
#[doc = "Register `R32_PFIC_IPRR2` writer"]
pub type W = crate::W<R32PficIprr2Spec>;
#[doc = "Field `PENDRESET` reader - RW1,PENDRESET"]
pub type PendresetR = crate::FieldReader;
#[doc = "Field `PENDRESET` writer - RW1,PENDRESET"]
pub type PendresetW<'a, REG> = crate::FieldWriter<'a, REG, 4>;
impl R {
    #[doc = "Bits 0:3 - RW1,PENDRESET"]
    #[inline(always)]
    pub fn pendreset(&self) -> PendresetR {
        PendresetR::new((self.bits & 0x0f) as u8)
    }
}
impl W {
    #[doc = "Bits 0:3 - RW1,PENDRESET"]
    #[inline(always)]
    pub fn pendreset(&mut self) -> PendresetW<'_, R32PficIprr2Spec> {
        PendresetW::new(self, 0)
    }
}
#[doc = "Interrupt Pending Clear Register\n\nYou can [`read`](crate::Reg::read) this register and get [`r32_pfic_iprr2::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`r32_pfic_iprr2::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct R32PficIprr2Spec;
impl crate::RegisterSpec for R32PficIprr2Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`r32_pfic_iprr2::R`](R) reader structure"]
impl crate::Readable for R32PficIprr2Spec {}
#[doc = "`write(|w| ..)` method takes [`r32_pfic_iprr2::W`](W) writer structure"]
impl crate::Writable for R32PficIprr2Spec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets R32_PFIC_IPRR2 to value 0"]
impl crate::Resettable for R32PficIprr2Spec {}
