#[doc = "Register `R32_TMR3_FIFO` reader"]
pub type R = crate::R<R32Tmr3FifoSpec>;
#[doc = "Register `R32_TMR3_FIFO` writer"]
pub type W = crate::W<R32Tmr3FifoSpec>;
#[doc = "Field `R32_TMR3_FIFO` reader - RO/WO, TMR3 FIFO register, only low 26 bit"]
pub type R32Tmr3FifoR = crate::FieldReader<u32>;
#[doc = "Field `R32_TMR3_FIFO` writer - RO/WO, TMR3 FIFO register, only low 26 bit"]
pub type R32Tmr3FifoW<'a, REG> = crate::FieldWriter<'a, REG, 32, u32>;
impl R {
    #[doc = "Bits 0:31 - RO/WO, TMR3 FIFO register, only low 26 bit"]
    #[inline(always)]
    pub fn r32_tmr3_fifo(&self) -> R32Tmr3FifoR {
        R32Tmr3FifoR::new(self.bits)
    }
}
impl W {
    #[doc = "Bits 0:31 - RO/WO, TMR3 FIFO register, only low 26 bit"]
    #[inline(always)]
    pub fn r32_tmr3_fifo(&mut self) -> R32Tmr3FifoW<'_, R32Tmr3FifoSpec> {
        R32Tmr3FifoW::new(self, 0)
    }
}
#[doc = "RO/WO, TMR3 FIFO register, only low 26 bit\n\nYou can [`read`](crate::Reg::read) this register and get [`r32_tmr3_fifo::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`r32_tmr3_fifo::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct R32Tmr3FifoSpec;
impl crate::RegisterSpec for R32Tmr3FifoSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`r32_tmr3_fifo::R`](R) reader structure"]
impl crate::Readable for R32Tmr3FifoSpec {}
#[doc = "`write(|w| ..)` method takes [`r32_tmr3_fifo::W`](W) writer structure"]
impl crate::Writable for R32Tmr3FifoSpec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets R32_TMR3_FIFO to value 0"]
impl crate::Resettable for R32Tmr3FifoSpec {}
