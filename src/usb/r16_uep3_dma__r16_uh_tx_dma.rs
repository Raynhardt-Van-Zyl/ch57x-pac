#[doc = "Register `R16_UEP3_DMA__R16_UH_TX_DMA` reader"]
pub type R = crate::R<R16Uep3Dma_R16UhTxDmaSpec>;
#[doc = "Register `R16_UEP3_DMA__R16_UH_TX_DMA` writer"]
pub type W = crate::W<R16Uep3Dma_R16UhTxDmaSpec>;
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        write!(f, "{}", self.bits())
    }
}
impl W {}
#[doc = "endpoint 3 DMA buffer address;host tx endpoint buffer high address\n\nYou can [`read`](crate::Reg::read) this register and get [`r16_uep3_dma__r16_uh_tx_dma::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`r16_uep3_dma__r16_uh_tx_dma::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct R16Uep3Dma_R16UhTxDmaSpec;
impl crate::RegisterSpec for R16Uep3Dma_R16UhTxDmaSpec {
    type Ux = u16;
}
#[doc = "`read()` method returns [`r16_uep3_dma__r16_uh_tx_dma::R`](R) reader structure"]
impl crate::Readable for R16Uep3Dma_R16UhTxDmaSpec {}
#[doc = "`write(|w| ..)` method takes [`r16_uep3_dma__r16_uh_tx_dma::W`](W) writer structure"]
impl crate::Writable for R16Uep3Dma_R16UhTxDmaSpec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets R16_UEP3_DMA__R16_UH_TX_DMA to value 0"]
impl crate::Resettable for R16Uep3Dma_R16UhTxDmaSpec {}
