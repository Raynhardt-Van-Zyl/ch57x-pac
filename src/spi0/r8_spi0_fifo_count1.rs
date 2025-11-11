#[doc = "Register `R8_SPI0_FIFO_COUNT1` reader"]
pub type R = crate::R<R8Spi0FifoCount1Spec>;
#[doc = "Field `R8_SPI0_FIFO_COUNT1` reader - RO, SPI0 FIFO count status"]
pub type R8Spi0FifoCount1R = crate::FieldReader;
impl R {
    #[doc = "Bits 0:7 - RO, SPI0 FIFO count status"]
    #[inline(always)]
    pub fn r8_spi0_fifo_count1(&self) -> R8Spi0FifoCount1R {
        R8Spi0FifoCount1R::new(self.bits)
    }
}
#[doc = "RO, SPI0 FIFO count status\n\nYou can [`read`](crate::Reg::read) this register and get [`r8_spi0_fifo_count1::R`](R). See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct R8Spi0FifoCount1Spec;
impl crate::RegisterSpec for R8Spi0FifoCount1Spec {
    type Ux = u8;
}
#[doc = "`read()` method returns [`r8_spi0_fifo_count1::R`](R) reader structure"]
impl crate::Readable for R8Spi0FifoCount1Spec {}
#[doc = "`reset()` method sets R8_SPI0_FIFO_COUNT1 to value 0"]
impl crate::Resettable for R8Spi0FifoCount1Spec {}
