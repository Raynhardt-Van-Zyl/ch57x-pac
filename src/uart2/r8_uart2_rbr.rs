#[doc = "Register `R8_UART2_RBR` reader"]
pub type R = crate::R<R8Uart2RbrSpec>;
#[doc = "Field `R8_UART2_RBR` reader - RO, UART2 receiver buffer, receiving byte"]
pub type R8Uart2RbrR = crate::FieldReader;
impl R {
    #[doc = "Bits 0:7 - RO, UART2 receiver buffer, receiving byte"]
    #[inline(always)]
    pub fn r8_uart2_rbr(&self) -> R8Uart2RbrR {
        R8Uart2RbrR::new(self.bits)
    }
}
#[doc = "RO, UART2 receiver buffer, receiving byte\n\nYou can [`read`](crate::Reg::read) this register and get [`r8_uart2_rbr::R`](R). See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct R8Uart2RbrSpec;
impl crate::RegisterSpec for R8Uart2RbrSpec {
    type Ux = u8;
}
#[doc = "`read()` method returns [`r8_uart2_rbr::R`](R) reader structure"]
impl crate::Readable for R8Uart2RbrSpec {}
#[doc = "`reset()` method sets R8_UART2_RBR to value 0"]
impl crate::Resettable for R8Uart2RbrSpec {}
