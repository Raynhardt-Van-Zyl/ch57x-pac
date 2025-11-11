#[doc = "Register `R32_TMR3_COUNT` reader"]
pub type R = crate::R<R32Tmr3CountSpec>;
#[doc = "Field `R32_TMR3_COUNT` reader - R0, TMR3 current count"]
pub type R32Tmr3CountR = crate::FieldReader<u32>;
impl R {
    #[doc = "Bits 0:31 - R0, TMR3 current count"]
    #[inline(always)]
    pub fn r32_tmr3_count(&self) -> R32Tmr3CountR {
        R32Tmr3CountR::new(self.bits)
    }
}
#[doc = "RO, TMR3 current count\n\nYou can [`read`](crate::Reg::read) this register and get [`r32_tmr3_count::R`](R). See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct R32Tmr3CountSpec;
impl crate::RegisterSpec for R32Tmr3CountSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`r32_tmr3_count::R`](R) reader structure"]
impl crate::Readable for R32Tmr3CountSpec {}
#[doc = "`reset()` method sets R32_TMR3_COUNT to value 0"]
impl crate::Resettable for R32Tmr3CountSpec {}
