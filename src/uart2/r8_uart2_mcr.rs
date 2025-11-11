#[doc = "Register `R8_UART2_MCR` reader"]
pub type R = crate::R<R8Uart2McrSpec>;
#[doc = "Register `R8_UART2_MCR` writer"]
pub type W = crate::W<R8Uart2McrSpec>;
#[doc = "Field `RB_MCR_OUT2__RB_MCR_INT_OE` reader - RW, UART control OUT2;UART interrupt output enable"]
pub type RbMcrOut2_RbMcrIntOeR = crate::BitReader;
#[doc = "Field `RB_MCR_OUT2__RB_MCR_INT_OE` writer - RW, UART control OUT2;UART interrupt output enable"]
pub type RbMcrOut2_RbMcrIntOeW<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bit 3 - RW, UART control OUT2;UART interrupt output enable"]
    #[inline(always)]
    pub fn rb_mcr_out2__rb_mcr_int_oe(&self) -> RbMcrOut2_RbMcrIntOeR {
        RbMcrOut2_RbMcrIntOeR::new(((self.bits >> 3) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 3 - RW, UART control OUT2;UART interrupt output enable"]
    #[inline(always)]
    pub fn rb_mcr_out2__rb_mcr_int_oe(&mut self) -> RbMcrOut2_RbMcrIntOeW<'_, R8Uart2McrSpec> {
        RbMcrOut2_RbMcrIntOeW::new(self, 3)
    }
}
#[doc = "RW, UART2 modem control\n\nYou can [`read`](crate::Reg::read) this register and get [`r8_uart2_mcr::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`r8_uart2_mcr::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct R8Uart2McrSpec;
impl crate::RegisterSpec for R8Uart2McrSpec {
    type Ux = u8;
}
#[doc = "`read()` method returns [`r8_uart2_mcr::R`](R) reader structure"]
impl crate::Readable for R8Uart2McrSpec {}
#[doc = "`write(|w| ..)` method takes [`r8_uart2_mcr::W`](W) writer structure"]
impl crate::Writable for R8Uart2McrSpec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets R8_UART2_MCR to value 0"]
impl crate::Resettable for R8Uart2McrSpec {}
