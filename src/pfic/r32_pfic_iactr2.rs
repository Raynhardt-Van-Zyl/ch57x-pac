#[doc = "Register `R32_PFIC_IACTR2` reader"]
pub type R = crate::R<R32PficIactr2Spec>;
#[doc = "Register `R32_PFIC_IACTR2` writer"]
pub type W = crate::W<R32PficIactr2Spec>;
#[doc = "Field `IACTS` reader - RW1,IACTS"]
pub type IactsR = crate::FieldReader;
#[doc = "Field `IACTS` writer - RW1,IACTS"]
pub type IactsW<'a, REG> = crate::FieldWriter<'a, REG, 4>;
impl R {
    #[doc = "Bits 0:3 - RW1,IACTS"]
    #[inline(always)]
    pub fn iacts(&self) -> IactsR {
        IactsR::new((self.bits & 0x0f) as u8)
    }
}
impl W {
    #[doc = "Bits 0:3 - RW1,IACTS"]
    #[inline(always)]
    pub fn iacts(&mut self) -> IactsW<'_, R32PficIactr2Spec> {
        IactsW::new(self, 0)
    }
}
#[doc = "Interrupt ACTIVE Register\n\nYou can [`read`](crate::Reg::read) this register and get [`r32_pfic_iactr2::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`r32_pfic_iactr2::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct R32PficIactr2Spec;
impl crate::RegisterSpec for R32PficIactr2Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`r32_pfic_iactr2::R`](R) reader structure"]
impl crate::Readable for R32PficIactr2Spec {}
#[doc = "`write(|w| ..)` method takes [`r32_pfic_iactr2::W`](W) writer structure"]
impl crate::Writable for R32PficIactr2Spec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets R32_PFIC_IACTR2 to value 0"]
impl crate::Resettable for R32PficIactr2Spec {}
