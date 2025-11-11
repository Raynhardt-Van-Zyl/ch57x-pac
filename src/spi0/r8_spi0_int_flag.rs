#[doc = "Register `R8_SPI0_INT_FLAG` reader"]
pub type R = crate::R<R8Spi0IntFlagSpec>;
#[doc = "Register `R8_SPI0_INT_FLAG` writer"]
pub type W = crate::W<R8Spi0IntFlagSpec>;
#[doc = "Field `RB_SPI_IF_CNT_END` reader - RW1, interrupt flag for SPI total byte count end"]
pub type RbSpiIfCntEndR = crate::BitReader;
#[doc = "Field `RB_SPI_IF_CNT_END` writer - RW1, interrupt flag for SPI total byte count end"]
pub type RbSpiIfCntEndW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `RB_SPI_IF_BYTE_END` reader - RW1, interrupt flag for SPI byte exchanged"]
pub type RbSpiIfByteEndR = crate::BitReader;
#[doc = "Field `RB_SPI_IF_BYTE_END` writer - RW1, interrupt flag for SPI byte exchanged"]
pub type RbSpiIfByteEndW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `RB_SPI_IF_FIFO_HF` reader - RW1, interrupt flag for SPI FIFO half"]
pub type RbSpiIfFifoHfR = crate::BitReader;
#[doc = "Field `RB_SPI_IF_FIFO_HF` writer - RW1, interrupt flag for SPI FIFO half"]
pub type RbSpiIfFifoHfW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `RB_SPI_IF_DMA_END` reader - RW1, interrupt flag for SPI0 DMA completion"]
pub type RbSpiIfDmaEndR = crate::BitReader;
#[doc = "Field `RB_SPI_IF_DMA_END` writer - RW1, interrupt flag for SPI0 DMA completion"]
pub type RbSpiIfDmaEndW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `RB_SPI_IF_FIFO_OV` reader - RW1, interrupt flag for SPI0 FIFO overflow"]
pub type RbSpiIfFifoOvR = crate::BitReader;
#[doc = "Field `RB_SPI_IF_FIFO_OV` writer - RW1, interrupt flag for SPI0 FIFO overflow"]
pub type RbSpiIfFifoOvW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `RB_SPI_FREE` reader - RO, current SPI free status"]
pub type RbSpiFreeR = crate::BitReader;
#[doc = "Field `RB_SPI_FREE` writer - RO, current SPI free status"]
pub type RbSpiFreeW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `RB_SPI_IF_FST_BYTE` reader - RW1, interrupt flag for SPI0 slave mode first byte received"]
pub type RbSpiIfFstByteR = crate::BitReader;
#[doc = "Field `RB_SPI_IF_FST_BYTE` writer - RW1, interrupt flag for SPI0 slave mode first byte received"]
pub type RbSpiIfFstByteW<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bit 0 - RW1, interrupt flag for SPI total byte count end"]
    #[inline(always)]
    pub fn rb_spi_if_cnt_end(&self) -> RbSpiIfCntEndR {
        RbSpiIfCntEndR::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - RW1, interrupt flag for SPI byte exchanged"]
    #[inline(always)]
    pub fn rb_spi_if_byte_end(&self) -> RbSpiIfByteEndR {
        RbSpiIfByteEndR::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - RW1, interrupt flag for SPI FIFO half"]
    #[inline(always)]
    pub fn rb_spi_if_fifo_hf(&self) -> RbSpiIfFifoHfR {
        RbSpiIfFifoHfR::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3 - RW1, interrupt flag for SPI0 DMA completion"]
    #[inline(always)]
    pub fn rb_spi_if_dma_end(&self) -> RbSpiIfDmaEndR {
        RbSpiIfDmaEndR::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bit 4 - RW1, interrupt flag for SPI0 FIFO overflow"]
    #[inline(always)]
    pub fn rb_spi_if_fifo_ov(&self) -> RbSpiIfFifoOvR {
        RbSpiIfFifoOvR::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bit 6 - RO, current SPI free status"]
    #[inline(always)]
    pub fn rb_spi_free(&self) -> RbSpiFreeR {
        RbSpiFreeR::new(((self.bits >> 6) & 1) != 0)
    }
    #[doc = "Bit 7 - RW1, interrupt flag for SPI0 slave mode first byte received"]
    #[inline(always)]
    pub fn rb_spi_if_fst_byte(&self) -> RbSpiIfFstByteR {
        RbSpiIfFstByteR::new(((self.bits >> 7) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - RW1, interrupt flag for SPI total byte count end"]
    #[inline(always)]
    pub fn rb_spi_if_cnt_end(&mut self) -> RbSpiIfCntEndW<'_, R8Spi0IntFlagSpec> {
        RbSpiIfCntEndW::new(self, 0)
    }
    #[doc = "Bit 1 - RW1, interrupt flag for SPI byte exchanged"]
    #[inline(always)]
    pub fn rb_spi_if_byte_end(&mut self) -> RbSpiIfByteEndW<'_, R8Spi0IntFlagSpec> {
        RbSpiIfByteEndW::new(self, 1)
    }
    #[doc = "Bit 2 - RW1, interrupt flag for SPI FIFO half"]
    #[inline(always)]
    pub fn rb_spi_if_fifo_hf(&mut self) -> RbSpiIfFifoHfW<'_, R8Spi0IntFlagSpec> {
        RbSpiIfFifoHfW::new(self, 2)
    }
    #[doc = "Bit 3 - RW1, interrupt flag for SPI0 DMA completion"]
    #[inline(always)]
    pub fn rb_spi_if_dma_end(&mut self) -> RbSpiIfDmaEndW<'_, R8Spi0IntFlagSpec> {
        RbSpiIfDmaEndW::new(self, 3)
    }
    #[doc = "Bit 4 - RW1, interrupt flag for SPI0 FIFO overflow"]
    #[inline(always)]
    pub fn rb_spi_if_fifo_ov(&mut self) -> RbSpiIfFifoOvW<'_, R8Spi0IntFlagSpec> {
        RbSpiIfFifoOvW::new(self, 4)
    }
    #[doc = "Bit 6 - RO, current SPI free status"]
    #[inline(always)]
    pub fn rb_spi_free(&mut self) -> RbSpiFreeW<'_, R8Spi0IntFlagSpec> {
        RbSpiFreeW::new(self, 6)
    }
    #[doc = "Bit 7 - RW1, interrupt flag for SPI0 slave mode first byte received"]
    #[inline(always)]
    pub fn rb_spi_if_fst_byte(&mut self) -> RbSpiIfFstByteW<'_, R8Spi0IntFlagSpec> {
        RbSpiIfFstByteW::new(self, 7)
    }
}
#[doc = "RW1, SPI0 interrupt flag\n\nYou can [`read`](crate::Reg::read) this register and get [`r8_spi0_int_flag::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`r8_spi0_int_flag::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct R8Spi0IntFlagSpec;
impl crate::RegisterSpec for R8Spi0IntFlagSpec {
    type Ux = u8;
}
#[doc = "`read()` method returns [`r8_spi0_int_flag::R`](R) reader structure"]
impl crate::Readable for R8Spi0IntFlagSpec {}
#[doc = "`write(|w| ..)` method takes [`r8_spi0_int_flag::W`](W) writer structure"]
impl crate::Writable for R8Spi0IntFlagSpec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets R8_SPI0_INT_FLAG to value 0x40"]
impl crate::Resettable for R8Spi0IntFlagSpec {
    const RESET_VALUE: u8 = 0x40;
}
