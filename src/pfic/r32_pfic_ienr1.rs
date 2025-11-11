#[doc = "Register `R32_PFIC_IENR1` reader"]
pub type R = crate::R<R32PficIenr1Spec>;
#[doc = "Register `R32_PFIC_IENR1` writer"]
pub type W = crate::W<R32PficIenr1Spec>;
#[doc = "Field `INTEN` reader - RW1,INTEN"]
pub type IntenR = crate::FieldReader<u32>;
#[doc = "Field `INTEN` writer - RW1,INTEN"]
pub type IntenW<'a, REG> = crate::FieldWriter<'a, REG, 20, u32>;
impl R {
    #[doc = "Bits 12:31 - RW1,INTEN"]
    #[inline(always)]
    pub fn inten(&self) -> IntenR {
        IntenR::new((self.bits >> 12) & 0x000f_ffff)
    }
}
impl W {
    #[doc = "Bits 12:31 - RW1,INTEN"]
    #[inline(always)]
    pub fn inten(&mut self) -> IntenW<'_, R32PficIenr1Spec> {
        IntenW::new(self, 12)
    }
}
#[doc = "Interrupt Setting Register\n\nYou can [`read`](crate::Reg::read) this register and get [`r32_pfic_ienr1::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`r32_pfic_ienr1::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct R32PficIenr1Spec;
impl crate::RegisterSpec for R32PficIenr1Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`r32_pfic_ienr1::R`](R) reader structure"]
impl crate::Readable for R32PficIenr1Spec {}
#[doc = "`write(|w| ..)` method takes [`r32_pfic_ienr1::W`](W) writer structure"]
impl crate::Writable for R32PficIenr1Spec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets R32_PFIC_IENR1 to value 0"]
impl crate::Resettable for R32PficIenr1Spec {}
