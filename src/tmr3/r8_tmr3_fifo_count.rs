#[doc = "Register `R8_TMR3_FIFO_COUNT` reader"]
pub type R = crate::R<R8Tmr3FifoCountSpec>;
#[doc = "Field `R8_TMR3_FIFO_COUNT` reader - R0, TMR3 FIFO count status"]
pub type R8Tmr3FifoCountR = crate::FieldReader;
impl R {
    #[doc = "Bits 0:7 - R0, TMR3 FIFO count status"]
    #[inline(always)]
    pub fn r8_tmr3_fifo_count(&self) -> R8Tmr3FifoCountR {
        R8Tmr3FifoCountR::new(self.bits)
    }
}
#[doc = "RO, TMR3 FIFO count status\n\nYou can [`read`](crate::Reg::read) this register and get [`r8_tmr3_fifo_count::R`](R). See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct R8Tmr3FifoCountSpec;
impl crate::RegisterSpec for R8Tmr3FifoCountSpec {
    type Ux = u8;
}
#[doc = "`read()` method returns [`r8_tmr3_fifo_count::R`](R) reader structure"]
impl crate::Readable for R8Tmr3FifoCountSpec {}
#[doc = "`reset()` method sets R8_TMR3_FIFO_COUNT to value 0"]
impl crate::Resettable for R8Tmr3FifoCountSpec {}
