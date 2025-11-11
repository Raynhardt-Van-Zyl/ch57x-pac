#[doc = "Register `R16_TMR1_DMA_END` reader"]
pub type R = crate::R<R16Tmr1DmaEndSpec>;
#[doc = "Register `R16_TMR1_DMA_END` writer"]
pub type W = crate::W<R16Tmr1DmaEndSpec>;
#[doc = "Field `R16_TMR1_DMA_END` reader - RW1,TMR1 FIFO register"]
pub type R16Tmr1DmaEndR = crate::FieldReader<u16>;
#[doc = "Field `R16_TMR1_DMA_END` writer - RW1,TMR1 FIFO register"]
pub type R16Tmr1DmaEndW<'a, REG> = crate::FieldWriter<'a, REG, 16, u16>;
impl R {
    #[doc = "Bits 0:15 - RW1,TMR1 FIFO register"]
    #[inline(always)]
    pub fn r16_tmr1_dma_end(&self) -> R16Tmr1DmaEndR {
        R16Tmr1DmaEndR::new(self.bits)
    }
}
impl W {
    #[doc = "Bits 0:15 - RW1,TMR1 FIFO register"]
    #[inline(always)]
    pub fn r16_tmr1_dma_end(&mut self) -> R16Tmr1DmaEndW<'_, R16Tmr1DmaEndSpec> {
        R16Tmr1DmaEndW::new(self, 0)
    }
}
#[doc = "RW, TMR1 DMA end address\n\nYou can [`read`](crate::Reg::read) this register and get [`r16_tmr1_dma_end::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`r16_tmr1_dma_end::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct R16Tmr1DmaEndSpec;
impl crate::RegisterSpec for R16Tmr1DmaEndSpec {
    type Ux = u16;
}
#[doc = "`read()` method returns [`r16_tmr1_dma_end::R`](R) reader structure"]
impl crate::Readable for R16Tmr1DmaEndSpec {}
#[doc = "`write(|w| ..)` method takes [`r16_tmr1_dma_end::W`](W) writer structure"]
impl crate::Writable for R16Tmr1DmaEndSpec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets R16_TMR1_DMA_END to value 0"]
impl crate::Resettable for R16Tmr1DmaEndSpec {}
