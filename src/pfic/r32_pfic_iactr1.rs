#[doc = "Register `R32_PFIC_IACTR1` reader"]
pub type R = crate::R<R32PficIactr1Spec>;
#[doc = "Register `R32_PFIC_IACTR1` writer"]
pub type W = crate::W<R32PficIactr1Spec>;
#[doc = "Field `IACTS` reader - RW1,IACTS"]
pub type IactsR = crate::FieldReader<u32>;
#[doc = "Field `IACTS` writer - RW1,IACTS"]
pub type IactsW<'a, REG> = crate::FieldWriter<'a, REG, 20, u32>;
impl R {
    #[doc = "Bits 12:31 - RW1,IACTS"]
    #[inline(always)]
    pub fn iacts(&self) -> IactsR {
        IactsR::new((self.bits >> 12) & 0x000f_ffff)
    }
}
impl W {
    #[doc = "Bits 12:31 - RW1,IACTS"]
    #[inline(always)]
    pub fn iacts(&mut self) -> IactsW<'_, R32PficIactr1Spec> {
        IactsW::new(self, 12)
    }
}
#[doc = "Interrupt ACTIVE Register\n\nYou can [`read`](crate::Reg::read) this register and get [`r32_pfic_iactr1::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`r32_pfic_iactr1::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct R32PficIactr1Spec;
impl crate::RegisterSpec for R32PficIactr1Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`r32_pfic_iactr1::R`](R) reader structure"]
impl crate::Readable for R32PficIactr1Spec {}
#[doc = "`write(|w| ..)` method takes [`r32_pfic_iactr1::W`](W) writer structure"]
impl crate::Writable for R32PficIactr1Spec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets R32_PFIC_IACTR1 to value 0"]
impl crate::Resettable for R32PficIactr1Spec {}
