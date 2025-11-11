#[doc = "Register `R8_SPI0_FIFO_COUNT` reader"]
pub type R = crate::R<R8Spi0FifoCountSpec>;
#[doc = "Field `R8_SPI0_FIFO_COUNT` reader - RO, SPI0 FIFO count status"]
pub type R8Spi0FifoCountR = crate::FieldReader;
impl R {
    #[doc = "Bits 0:7 - RO, SPI0 FIFO count status"]
    #[inline(always)]
    pub fn r8_spi0_fifo_count(&self) -> R8Spi0FifoCountR {
        R8Spi0FifoCountR::new(self.bits)
    }
}
#[doc = "RO, SPI0 FIFO count status\n\nYou can [`read`](crate::Reg::read) this register and get [`r8_spi0_fifo_count::R`](R). See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct R8Spi0FifoCountSpec;
impl crate::RegisterSpec for R8Spi0FifoCountSpec {
    type Ux = u8;
}
#[doc = "`read()` method returns [`r8_spi0_fifo_count::R`](R) reader structure"]
impl crate::Readable for R8Spi0FifoCountSpec {}
#[doc = "`reset()` method sets R8_SPI0_FIFO_COUNT to value 0"]
impl crate::Resettable for R8Spi0FifoCountSpec {}
