#[doc = "Register `R32_TMR0_FIFO` reader"]
pub type R = crate::R<R32Tmr0FifoSpec>;
#[doc = "Field `R32_TMR0_FIFO` reader - RW1,TMR0 FIFO register"]
pub type R32Tmr0FifoR = crate::FieldReader<u32>;
impl R {
    #[doc = "Bits 0:31 - RW1,TMR0 FIFO register"]
    #[inline(always)]
    pub fn r32_tmr0_fifo(&self) -> R32Tmr0FifoR {
        R32Tmr0FifoR::new(self.bits)
    }
}
#[doc = "RO/WO, TMR0 FIFO register, only low 26 bit\n\nYou can [`read`](crate::Reg::read) this register and get [`r32_tmr0_fifo::R`](R). See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct R32Tmr0FifoSpec;
impl crate::RegisterSpec for R32Tmr0FifoSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`r32_tmr0_fifo::R`](R) reader structure"]
impl crate::Readable for R32Tmr0FifoSpec {}
#[doc = "`reset()` method sets R32_TMR0_FIFO to value 0"]
impl crate::Resettable for R32Tmr0FifoSpec {}
