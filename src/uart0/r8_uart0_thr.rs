#[doc = "Register `R8_UART0_THR` writer"]
pub type W = crate::W<R8Uart0ThrSpec>;
#[doc = "Field `R8_UART0_THR` writer - RO, UART0 transmitter holding, transmittal byte"]
pub type R8Uart0ThrW<'a, REG> = crate::FieldWriter<'a, REG, 8>;
impl W {
    #[doc = "Bits 0:7 - RO, UART0 transmitter holding, transmittal byte"]
    #[inline(always)]
    pub fn r8_uart0_thr(&mut self) -> R8Uart0ThrW<'_, R8Uart0ThrSpec> {
        R8Uart0ThrW::new(self, 0)
    }
}
#[doc = "WO, UART0 transmitter holding, transmittal byte\n\nYou can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`r8_uart0_thr::W`](W). See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct R8Uart0ThrSpec;
impl crate::RegisterSpec for R8Uart0ThrSpec {
    type Ux = u8;
}
#[doc = "`write(|w| ..)` method takes [`r8_uart0_thr::W`](W) writer structure"]
impl crate::Writable for R8Uart0ThrSpec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets R8_UART0_THR to value 0"]
impl crate::Resettable for R8Uart0ThrSpec {}
