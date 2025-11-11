#[doc = "Register `R32_PFIC_ITHRESDR` reader"]
pub type R = crate::R<R32PficIthresdrSpec>;
#[doc = "Register `R32_PFIC_ITHRESDR` writer"]
pub type W = crate::W<R32PficIthresdrSpec>;
#[doc = "Field `THRESHOLD` reader - RW,THRESHOLD"]
pub type ThresholdR = crate::FieldReader;
#[doc = "Field `THRESHOLD` writer - RW,THRESHOLD"]
pub type ThresholdW<'a, REG> = crate::FieldWriter<'a, REG, 8>;
impl R {
    #[doc = "Bits 0:7 - RW,THRESHOLD"]
    #[inline(always)]
    pub fn threshold(&self) -> ThresholdR {
        ThresholdR::new((self.bits & 0xff) as u8)
    }
}
impl W {
    #[doc = "Bits 0:7 - RW,THRESHOLD"]
    #[inline(always)]
    pub fn threshold(&mut self) -> ThresholdW<'_, R32PficIthresdrSpec> {
        ThresholdW::new(self, 0)
    }
}
#[doc = "RW,Interrupt Priority Register\n\nYou can [`read`](crate::Reg::read) this register and get [`r32_pfic_ithresdr::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`r32_pfic_ithresdr::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct R32PficIthresdrSpec;
impl crate::RegisterSpec for R32PficIthresdrSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`r32_pfic_ithresdr::R`](R) reader structure"]
impl crate::Readable for R32PficIthresdrSpec {}
#[doc = "`write(|w| ..)` method takes [`r32_pfic_ithresdr::W`](W) writer structure"]
impl crate::Writable for R32PficIthresdrSpec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets R32_PFIC_ITHRESDR to value 0"]
impl crate::Resettable for R32PficIthresdrSpec {}
