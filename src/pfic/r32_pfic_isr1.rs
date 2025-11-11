#[doc = "Register `R32_PFIC_ISR1` reader"]
pub type R = crate::R<R32PficIsr1Spec>;
#[doc = "Register `R32_PFIC_ISR1` writer"]
pub type W = crate::W<R32PficIsr1Spec>;
#[doc = "Field `INTENSTA` reader - Interrupt ID Status"]
pub type IntenstaR = crate::FieldReader<u32>;
#[doc = "Field `INTENSTA` writer - Interrupt ID Status"]
pub type IntenstaW<'a, REG> = crate::FieldWriter<'a, REG, 20, u32>;
impl R {
    #[doc = "Bits 12:31 - Interrupt ID Status"]
    #[inline(always)]
    pub fn intensta(&self) -> IntenstaR {
        IntenstaR::new((self.bits >> 12) & 0x000f_ffff)
    }
}
impl W {
    #[doc = "Bits 12:31 - Interrupt ID Status"]
    #[inline(always)]
    pub fn intensta(&mut self) -> IntenstaW<'_, R32PficIsr1Spec> {
        IntenstaW::new(self, 12)
    }
}
#[doc = "RO,Interrupt Status Register\n\nYou can [`read`](crate::Reg::read) this register and get [`r32_pfic_isr1::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`r32_pfic_isr1::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct R32PficIsr1Spec;
impl crate::RegisterSpec for R32PficIsr1Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`r32_pfic_isr1::R`](R) reader structure"]
impl crate::Readable for R32PficIsr1Spec {}
#[doc = "`write(|w| ..)` method takes [`r32_pfic_isr1::W`](W) writer structure"]
impl crate::Writable for R32PficIsr1Spec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets R32_PFIC_ISR1 to value 0"]
impl crate::Resettable for R32PficIsr1Spec {}
