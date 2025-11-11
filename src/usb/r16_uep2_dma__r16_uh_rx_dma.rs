#[doc = "Register `R16_UEP2_DMA__R16_UH_RX_DMA` reader"]
pub type R = crate::R<R16Uep2Dma_R16UhRxDmaSpec>;
#[doc = "Register `R16_UEP2_DMA__R16_UH_RX_DMA` writer"]
pub type W = crate::W<R16Uep2Dma_R16UhRxDmaSpec>;
#[doc = "Field `R16_UEP2_DMA` reader - RW,endpoint 2 DMA buffer address;host rx endpoint buffer high address"]
pub type R16Uep2DmaR = crate::FieldReader<u16>;
#[doc = "Field `R16_UEP2_DMA` writer - RW,endpoint 2 DMA buffer address;host rx endpoint buffer high address"]
pub type R16Uep2DmaW<'a, REG> = crate::FieldWriter<'a, REG, 16, u16>;
impl R {
    #[doc = "Bits 0:15 - RW,endpoint 2 DMA buffer address;host rx endpoint buffer high address"]
    #[inline(always)]
    pub fn r16_uep2_dma(&self) -> R16Uep2DmaR {
        R16Uep2DmaR::new(self.bits)
    }
}
impl W {
    #[doc = "Bits 0:15 - RW,endpoint 2 DMA buffer address;host rx endpoint buffer high address"]
    #[inline(always)]
    pub fn r16_uep2_dma(&mut self) -> R16Uep2DmaW<'_, R16Uep2Dma_R16UhRxDmaSpec> {
        R16Uep2DmaW::new(self, 0)
    }
}
#[doc = "endpoint 2 DMA buffer address;host rx endpoint buffer high address\n\nYou can [`read`](crate::Reg::read) this register and get [`r16_uep2_dma__r16_uh_rx_dma::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`r16_uep2_dma__r16_uh_rx_dma::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct R16Uep2Dma_R16UhRxDmaSpec;
impl crate::RegisterSpec for R16Uep2Dma_R16UhRxDmaSpec {
    type Ux = u16;
}
#[doc = "`read()` method returns [`r16_uep2_dma__r16_uh_rx_dma::R`](R) reader structure"]
impl crate::Readable for R16Uep2Dma_R16UhRxDmaSpec {}
#[doc = "`write(|w| ..)` method takes [`r16_uep2_dma__r16_uh_rx_dma::W`](W) writer structure"]
impl crate::Writable for R16Uep2Dma_R16UhRxDmaSpec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets R16_UEP2_DMA__R16_UH_RX_DMA to value 0"]
impl crate::Resettable for R16Uep2Dma_R16UhRxDmaSpec {}
