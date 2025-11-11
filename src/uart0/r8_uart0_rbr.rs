#[doc = "Register `R8_UART0_RBR` reader"]
pub type R = crate::R<R8Uart0RbrSpec>;
#[doc = "Field `R8_UART0_RBR` reader - RO, UART0 receiver buffer, receiving byte"]
pub type R8Uart0RbrR = crate::FieldReader;
impl R {
    #[doc = "Bits 0:7 - RO, UART0 receiver buffer, receiving byte"]
    #[inline(always)]
    pub fn r8_uart0_rbr(&self) -> R8Uart0RbrR {
        R8Uart0RbrR::new(self.bits)
    }
}
#[doc = "RO, UART0 receiver buffer, receiving byte\n\nYou can [`read`](crate::Reg::read) this register and get [`r8_uart0_rbr::R`](R). See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct R8Uart0RbrSpec;
impl crate::RegisterSpec for R8Uart0RbrSpec {
    type Ux = u8;
}
#[doc = "`read()` method returns [`r8_uart0_rbr::R`](R) reader structure"]
impl crate::Readable for R8Uart0RbrSpec {}
#[doc = "`reset()` method sets R8_UART0_RBR to value 0"]
impl crate::Resettable for R8Uart0RbrSpec {}
