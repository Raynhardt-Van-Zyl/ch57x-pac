#[doc = "Register `R8_UART1_RBR` reader"]
pub type R = crate::R<R8Uart1RbrSpec>;
#[doc = "Field `R8_UART1_RBR` reader - RO, UART1 receiver buffer, receiving byte"]
pub type R8Uart1RbrR = crate::FieldReader;
impl R {
    #[doc = "Bits 0:7 - RO, UART1 receiver buffer, receiving byte"]
    #[inline(always)]
    pub fn r8_uart1_rbr(&self) -> R8Uart1RbrR {
        R8Uart1RbrR::new(self.bits)
    }
}
#[doc = "RO, UART1 receiver buffer, receiving byte\n\nYou can [`read`](crate::Reg::read) this register and get [`r8_uart1_rbr::R`](R). See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct R8Uart1RbrSpec;
impl crate::RegisterSpec for R8Uart1RbrSpec {
    type Ux = u8;
}
#[doc = "`read()` method returns [`r8_uart1_rbr::R`](R) reader structure"]
impl crate::Readable for R8Uart1RbrSpec {}
#[doc = "`reset()` method sets R8_UART1_RBR to value 0"]
impl crate::Resettable for R8Uart1RbrSpec {}
