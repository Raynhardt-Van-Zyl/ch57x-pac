#[doc = "Register `R32_PFIC_IENR2` reader"]
pub type R = crate::R<R32PficIenr2Spec>;
#[doc = "Register `R32_PFIC_IENR2` writer"]
pub type W = crate::W<R32PficIenr2Spec>;
#[doc = "Field `INTEN` reader - RW1,INTEN"]
pub type IntenR = crate::FieldReader;
#[doc = "Field `INTEN` writer - RW1,INTEN"]
pub type IntenW<'a, REG> = crate::FieldWriter<'a, REG, 4>;
impl R {
    #[doc = "Bits 0:3 - RW1,INTEN"]
    #[inline(always)]
    pub fn inten(&self) -> IntenR {
        IntenR::new((self.bits & 0x0f) as u8)
    }
}
impl W {
    #[doc = "Bits 0:3 - RW1,INTEN"]
    #[inline(always)]
    pub fn inten(&mut self) -> IntenW<'_, R32PficIenr2Spec> {
        IntenW::new(self, 0)
    }
}
#[doc = "Interrupt Setting Register\n\nYou can [`read`](crate::Reg::read) this register and get [`r32_pfic_ienr2::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`r32_pfic_ienr2::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct R32PficIenr2Spec;
impl crate::RegisterSpec for R32PficIenr2Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`r32_pfic_ienr2::R`](R) reader structure"]
impl crate::Readable for R32PficIenr2Spec {}
#[doc = "`write(|w| ..)` method takes [`r32_pfic_ienr2::W`](W) writer structure"]
impl crate::Writable for R32PficIenr2Spec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets R32_PFIC_IENR2 to value 0"]
impl crate::Resettable for R32PficIenr2Spec {}
