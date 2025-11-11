#[doc = "Register `R8_UART3_THR` writer"]
pub type W = crate::W<R8Uart3ThrSpec>;
#[doc = "Field `R8_UART3_THR` writer - WO, UART3 transmitter holding, transmittal byte"]
pub type R8Uart3ThrW<'a, REG> = crate::FieldWriter<'a, REG, 8>;
impl W {
    #[doc = "Bits 0:7 - WO, UART3 transmitter holding, transmittal byte"]
    #[inline(always)]
    pub fn r8_uart3_thr(&mut self) -> R8Uart3ThrW<'_, R8Uart3ThrSpec> {
        R8Uart3ThrW::new(self, 0)
    }
}
#[doc = "WO, UART3 transmitter holding, transmittal byte\n\nYou can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`r8_uart3_thr::W`](W). See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct R8Uart3ThrSpec;
impl crate::RegisterSpec for R8Uart3ThrSpec {
    type Ux = u8;
}
#[doc = "`write(|w| ..)` method takes [`r8_uart3_thr::W`](W) writer structure"]
impl crate::Writable for R8Uart3ThrSpec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets R8_UART3_THR to value 0"]
impl crate::Resettable for R8Uart3ThrSpec {}
