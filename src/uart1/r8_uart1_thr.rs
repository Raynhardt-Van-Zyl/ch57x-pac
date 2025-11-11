#[doc = "Register `R8_UART1_THR` writer"]
pub type W = crate::W<R8Uart1ThrSpec>;
#[doc = "Field `R8_UART1_RBR` writer - WO, UART1 transmitter holding, transmittal byte"]
pub type R8Uart1RbrW<'a, REG> = crate::FieldWriter<'a, REG, 8>;
impl W {
    #[doc = "Bits 0:7 - WO, UART1 transmitter holding, transmittal byte"]
    #[inline(always)]
    pub fn r8_uart1_rbr(&mut self) -> R8Uart1RbrW<'_, R8Uart1ThrSpec> {
        R8Uart1RbrW::new(self, 0)
    }
}
#[doc = "WO, UART1 transmitter holding, transmittal byte\n\nYou can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`r8_uart1_thr::W`](W). See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct R8Uart1ThrSpec;
impl crate::RegisterSpec for R8Uart1ThrSpec {
    type Ux = u8;
}
#[doc = "`write(|w| ..)` method takes [`r8_uart1_thr::W`](W) writer structure"]
impl crate::Writable for R8Uart1ThrSpec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets R8_UART1_THR to value 0"]
impl crate::Resettable for R8Uart1ThrSpec {}
