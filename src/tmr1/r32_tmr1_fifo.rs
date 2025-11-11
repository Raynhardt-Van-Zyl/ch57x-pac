#[doc = "Register `R32_TMR1_FIFO` reader"]
pub type R = crate::R<R32Tmr1FifoSpec>;
#[doc = "Field `R32_TMR1_FIFO` reader - RW1,TMR1 FIFO register"]
pub type R32Tmr1FifoR = crate::FieldReader<u32>;
impl R {
    #[doc = "Bits 0:31 - RW1,TMR1 FIFO register"]
    #[inline(always)]
    pub fn r32_tmr1_fifo(&self) -> R32Tmr1FifoR {
        R32Tmr1FifoR::new(self.bits)
    }
}
#[doc = "RO, TMR1 FIFO register, only low 26 bit\n\nYou can [`read`](crate::Reg::read) this register and get [`r32_tmr1_fifo::R`](R). See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct R32Tmr1FifoSpec;
impl crate::RegisterSpec for R32Tmr1FifoSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`r32_tmr1_fifo::R`](R) reader structure"]
impl crate::Readable for R32Tmr1FifoSpec {}
#[doc = "`reset()` method sets R32_TMR1_FIFO to value 0"]
impl crate::Resettable for R32Tmr1FifoSpec {}
