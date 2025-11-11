#[doc = "Register `R16_SPI0_DMA_BEG` reader"]
pub type R = crate::R<R16Spi0DmaBegSpec>;
#[doc = "Register `R16_SPI0_DMA_BEG` writer"]
pub type W = crate::W<R16Spi0DmaBegSpec>;
#[doc = "Field `R16_SPI0_DMA_BEG` reader - RW, SPI0 DMA begin address"]
pub type R16Spi0DmaBegR = crate::FieldReader<u16>;
#[doc = "Field `R16_SPI0_DMA_BEG` writer - RW, SPI0 DMA begin address"]
pub type R16Spi0DmaBegW<'a, REG> = crate::FieldWriter<'a, REG, 16, u16>;
impl R {
    #[doc = "Bits 0:15 - RW, SPI0 DMA begin address"]
    #[inline(always)]
    pub fn r16_spi0_dma_beg(&self) -> R16Spi0DmaBegR {
        R16Spi0DmaBegR::new(self.bits)
    }
}
impl W {
    #[doc = "Bits 0:15 - RW, SPI0 DMA begin address"]
    #[inline(always)]
    pub fn r16_spi0_dma_beg(&mut self) -> R16Spi0DmaBegW<'_, R16Spi0DmaBegSpec> {
        R16Spi0DmaBegW::new(self, 0)
    }
}
#[doc = "RW, SPI0 DMA begin address\n\nYou can [`read`](crate::Reg::read) this register and get [`r16_spi0_dma_beg::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`r16_spi0_dma_beg::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct R16Spi0DmaBegSpec;
impl crate::RegisterSpec for R16Spi0DmaBegSpec {
    type Ux = u16;
}
#[doc = "`read()` method returns [`r16_spi0_dma_beg::R`](R) reader structure"]
impl crate::Readable for R16Spi0DmaBegSpec {}
#[doc = "`write(|w| ..)` method takes [`r16_spi0_dma_beg::W`](W) writer structure"]
impl crate::Writable for R16Spi0DmaBegSpec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets R16_SPI0_DMA_BEG to value 0"]
impl crate::Resettable for R16Spi0DmaBegSpec {}
