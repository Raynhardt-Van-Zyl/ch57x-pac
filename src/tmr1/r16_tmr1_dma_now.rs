#[doc = "Register `R16_TMR1_DMA_NOW` reader"]
pub type R = crate::R<R16Tmr1DmaNowSpec>;
#[doc = "Field `R16_TMR1_DMA_NOW` reader - RW1,TMR1 FIFO register"]
pub type R16Tmr1DmaNowR = crate::FieldReader<u16>;
impl R {
    #[doc = "Bits 0:15 - RW1,TMR1 FIFO register"]
    #[inline(always)]
    pub fn r16_tmr1_dma_now(&self) -> R16Tmr1DmaNowR {
        R16Tmr1DmaNowR::new(self.bits)
    }
}
#[doc = "RO, TMR1 DMA current address\n\nYou can [`read`](crate::Reg::read) this register and get [`r16_tmr1_dma_now::R`](R). See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct R16Tmr1DmaNowSpec;
impl crate::RegisterSpec for R16Tmr1DmaNowSpec {
    type Ux = u16;
}
#[doc = "`read()` method returns [`r16_tmr1_dma_now::R`](R) reader structure"]
impl crate::Readable for R16Tmr1DmaNowSpec {}
#[doc = "`reset()` method sets R16_TMR1_DMA_NOW to value 0"]
impl crate::Resettable for R16Tmr1DmaNowSpec {}
