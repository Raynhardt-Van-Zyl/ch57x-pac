#[doc = "Register `R16_SPI0_DMA_END` reader"]
pub type R = crate::R<R16Spi0DmaEndSpec>;
#[doc = "Register `R16_SPI0_DMA_END` writer"]
pub type W = crate::W<R16Spi0DmaEndSpec>;
#[doc = "Field `R16_SPI0_DMA_END` reader - RW, SPI0 DMA end address"]
pub type R16Spi0DmaEndR = crate::FieldReader<u16>;
#[doc = "Field `R16_SPI0_DMA_END` writer - RW, SPI0 DMA end address"]
pub type R16Spi0DmaEndW<'a, REG> = crate::FieldWriter<'a, REG, 16, u16>;
impl R {
    #[doc = "Bits 0:15 - RW, SPI0 DMA end address"]
    #[inline(always)]
    pub fn r16_spi0_dma_end(&self) -> R16Spi0DmaEndR {
        R16Spi0DmaEndR::new(self.bits)
    }
}
impl W {
    #[doc = "Bits 0:15 - RW, SPI0 DMA end address"]
    #[inline(always)]
    pub fn r16_spi0_dma_end(&mut self) -> R16Spi0DmaEndW<'_, R16Spi0DmaEndSpec> {
        R16Spi0DmaEndW::new(self, 0)
    }
}
#[doc = "RW, SPI0 DMA end address\n\nYou can [`read`](crate::Reg::read) this register and get [`r16_spi0_dma_end::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`r16_spi0_dma_end::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct R16Spi0DmaEndSpec;
impl crate::RegisterSpec for R16Spi0DmaEndSpec {
    type Ux = u16;
}
#[doc = "`read()` method returns [`r16_spi0_dma_end::R`](R) reader structure"]
impl crate::Readable for R16Spi0DmaEndSpec {}
#[doc = "`write(|w| ..)` method takes [`r16_spi0_dma_end::W`](W) writer structure"]
impl crate::Writable for R16Spi0DmaEndSpec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets R16_SPI0_DMA_END to value 0"]
impl crate::Resettable for R16Spi0DmaEndSpec {}
