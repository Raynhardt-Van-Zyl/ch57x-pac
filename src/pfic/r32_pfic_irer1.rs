#[doc = "Register `R32_PFIC_IRER1` reader"]
pub type R = crate::R<R32PficIrer1Spec>;
#[doc = "Register `R32_PFIC_IRER1` writer"]
pub type W = crate::W<R32PficIrer1Spec>;
#[doc = "Field `INTRESET` reader - RW1,INTRESET"]
pub type IntresetR = crate::FieldReader<u32>;
#[doc = "Field `INTRESET` writer - RW1,INTRESET"]
pub type IntresetW<'a, REG> = crate::FieldWriter<'a, REG, 20, u32>;
impl R {
    #[doc = "Bits 12:31 - RW1,INTRESET"]
    #[inline(always)]
    pub fn intreset(&self) -> IntresetR {
        IntresetR::new((self.bits >> 12) & 0x000f_ffff)
    }
}
impl W {
    #[doc = "Bits 12:31 - RW1,INTRESET"]
    #[inline(always)]
    pub fn intreset(&mut self) -> IntresetW<'_, R32PficIrer1Spec> {
        IntresetW::new(self, 12)
    }
}
#[doc = "Interrupt Clear Register\n\nYou can [`read`](crate::Reg::read) this register and get [`r32_pfic_irer1::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`r32_pfic_irer1::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct R32PficIrer1Spec;
impl crate::RegisterSpec for R32PficIrer1Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`r32_pfic_irer1::R`](R) reader structure"]
impl crate::Readable for R32PficIrer1Spec {}
#[doc = "`write(|w| ..)` method takes [`r32_pfic_irer1::W`](W) writer structure"]
impl crate::Writable for R32PficIrer1Spec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets R32_PFIC_IRER1 to value 0"]
impl crate::Resettable for R32PficIrer1Spec {}
