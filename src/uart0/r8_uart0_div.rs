#[doc = "Register `R8_UART0_DIV` reader"]
pub type R = crate::R<R8Uart0DivSpec>;
#[doc = "Register `R8_UART0_DIV` writer"]
pub type W = crate::W<R8Uart0DivSpec>;
#[doc = "Field `R8_UART0_DIV` reader - RW,UART0 pre-divisor latch byte, only low 7 bit, from 1 to 0/128"]
pub type R8Uart0DivR = crate::FieldReader;
#[doc = "Field `R8_UART0_DIV` writer - RW,UART0 pre-divisor latch byte, only low 7 bit, from 1 to 0/128"]
pub type R8Uart0DivW<'a, REG> = crate::FieldWriter<'a, REG, 8>;
impl R {
    #[doc = "Bits 0:7 - RW,UART0 pre-divisor latch byte, only low 7 bit, from 1 to 0/128"]
    #[inline(always)]
    pub fn r8_uart0_div(&self) -> R8Uart0DivR {
        R8Uart0DivR::new(self.bits)
    }
}
impl W {
    #[doc = "Bits 0:7 - RW,UART0 pre-divisor latch byte, only low 7 bit, from 1 to 0/128"]
    #[inline(always)]
    pub fn r8_uart0_div(&mut self) -> R8Uart0DivW<'_, R8Uart0DivSpec> {
        R8Uart0DivW::new(self, 0)
    }
}
#[doc = "RW, UART0 pre-divisor latch byte, only low 7 bit, from 1 to 0/128\n\nYou can [`read`](crate::Reg::read) this register and get [`r8_uart0_div::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`r8_uart0_div::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct R8Uart0DivSpec;
impl crate::RegisterSpec for R8Uart0DivSpec {
    type Ux = u8;
}
#[doc = "`read()` method returns [`r8_uart0_div::R`](R) reader structure"]
impl crate::Readable for R8Uart0DivSpec {}
#[doc = "`write(|w| ..)` method takes [`r8_uart0_div::W`](W) writer structure"]
impl crate::Writable for R8Uart0DivSpec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets R8_UART0_DIV to value 0"]
impl crate::Resettable for R8Uart0DivSpec {}
