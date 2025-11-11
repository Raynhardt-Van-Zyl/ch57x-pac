#[doc = "Register `R8_UART0_IIR` reader"]
pub type R = crate::R<R8Uart0IirSpec>;
#[doc = "Field `RB_IIR_NO_INT` reader - RO, UART no interrupt flag: 0=interrupt action, 1=no interrupt"]
pub type RbIirNoIntR = crate::BitReader;
#[doc = "Field `RB_IIR_INT_MASK` reader - RO, UART interrupt flag bit mask"]
pub type RbIirIntMaskR = crate::FieldReader;
#[doc = "Field `RB_IIR_FIFO_ID` reader - RO, UART FIFO enabled flag"]
pub type RbIirFifoIdR = crate::FieldReader;
impl R {
    #[doc = "Bit 0 - RO, UART no interrupt flag: 0=interrupt action, 1=no interrupt"]
    #[inline(always)]
    pub fn rb_iir_no_int(&self) -> RbIirNoIntR {
        RbIirNoIntR::new((self.bits & 1) != 0)
    }
    #[doc = "Bits 0:3 - RO, UART interrupt flag bit mask"]
    #[inline(always)]
    pub fn rb_iir_int_mask(&self) -> RbIirIntMaskR {
        RbIirIntMaskR::new(self.bits & 0x0f)
    }
    #[doc = "Bits 6:7 - RO, UART FIFO enabled flag"]
    #[inline(always)]
    pub fn rb_iir_fifo_id(&self) -> RbIirFifoIdR {
        RbIirFifoIdR::new((self.bits >> 6) & 3)
    }
}
#[doc = "RO, UART0 interrupt identification\n\nYou can [`read`](crate::Reg::read) this register and get [`r8_uart0_iir::R`](R). See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct R8Uart0IirSpec;
impl crate::RegisterSpec for R8Uart0IirSpec {
    type Ux = u8;
}
#[doc = "`read()` method returns [`r8_uart0_iir::R`](R) reader structure"]
impl crate::Readable for R8Uart0IirSpec {}
#[doc = "`reset()` method sets R8_UART0_IIR to value 0x01"]
impl crate::Resettable for R8Uart0IirSpec {
    const RESET_VALUE: u8 = 0x01;
}
