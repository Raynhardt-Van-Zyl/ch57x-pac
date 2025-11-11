#[doc = "Register `R16_TMR2_DMA_NOW` reader"]
pub type R = crate::R<R16Tmr2DmaNowSpec>;
#[doc = "Field `R16_TMR2_DMA_NOW` reader - RW, TMR2 current count"]
pub type R16Tmr2DmaNowR = crate::FieldReader<u16>;
impl R {
    #[doc = "Bits 0:15 - RW, TMR2 current count"]
    #[inline(always)]
    pub fn r16_tmr2_dma_now(&self) -> R16Tmr2DmaNowR {
        R16Tmr2DmaNowR::new(self.bits)
    }
}
#[doc = "RO, TMR2 DMA current address\n\nYou can [`read`](crate::Reg::read) this register and get [`r16_tmr2_dma_now::R`](R). See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct R16Tmr2DmaNowSpec;
impl crate::RegisterSpec for R16Tmr2DmaNowSpec {
    type Ux = u16;
}
#[doc = "`read()` method returns [`r16_tmr2_dma_now::R`](R) reader structure"]
impl crate::Readable for R16Tmr2DmaNowSpec {}
#[doc = "`reset()` method sets R16_TMR2_DMA_NOW to value 0"]
impl crate::Resettable for R16Tmr2DmaNowSpec {}
