#[doc = "Register `R16_PIN_ALTERNATE` reader"]
pub type R = crate::R<R16PinAlternateSpec>;
#[doc = "Register `R16_PIN_ALTERNATE` writer"]
pub type W = crate::W<R16PinAlternateSpec>;
#[doc = "Field `RB_PIN_TMR0` reader - RW, TMR0 alternate pin enable"]
pub type RbPinTmr0R = crate::BitReader;
#[doc = "Field `RB_PIN_TMR0` writer - RW, TMR0 alternate pin enable"]
pub type RbPinTmr0W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `RB_PIN_TMR1` reader - RW, TMR1 alternate pin enable"]
pub type RbPinTmr1R = crate::BitReader;
#[doc = "Field `RB_PIN_TMR1` writer - RW, TMR1 alternate pin enable"]
pub type RbPinTmr1W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `RB_PIN_TMR2` reader - RW, TMR2 alternate pin enable"]
pub type RbPinTmr2R = crate::BitReader;
#[doc = "Field `RB_PIN_TMR2` writer - RW, TMR2 alternate pin enable"]
pub type RbPinTmr2W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `RB_PIN_UART0` reader - RW, RXD0/TXD0 alternate pin enable"]
pub type RbPinUart0R = crate::BitReader;
#[doc = "Field `RB_PIN_UART0` writer - RW, RXD0/TXD0 alternate pin enable"]
pub type RbPinUart0W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `RB_PIN_UART1` reader - RW, RXD1/TXD1 alternate pin enable"]
pub type RbPinUart1R = crate::BitReader;
#[doc = "Field `RB_PIN_UART1` writer - RW, RXD1/TXD1 alternate pin enable"]
pub type RbPinUart1W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `RB_PIN_SPI0` reader - RW, SCS/SCK0/MOSI/MISO alternate pin enable"]
pub type RbPinSpi0R = crate::BitReader;
#[doc = "Field `RB_PIN_SPI0` writer - RW, SCS/SCK0/MOSI/MISO alternate pin enable"]
pub type RbPinSpi0W<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bit 0 - RW, TMR0 alternate pin enable"]
    #[inline(always)]
    pub fn rb_pin_tmr0(&self) -> RbPinTmr0R {
        RbPinTmr0R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - RW, TMR1 alternate pin enable"]
    #[inline(always)]
    pub fn rb_pin_tmr1(&self) -> RbPinTmr1R {
        RbPinTmr1R::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - RW, TMR2 alternate pin enable"]
    #[inline(always)]
    pub fn rb_pin_tmr2(&self) -> RbPinTmr2R {
        RbPinTmr2R::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 4 - RW, RXD0/TXD0 alternate pin enable"]
    #[inline(always)]
    pub fn rb_pin_uart0(&self) -> RbPinUart0R {
        RbPinUart0R::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bit 5 - RW, RXD1/TXD1 alternate pin enable"]
    #[inline(always)]
    pub fn rb_pin_uart1(&self) -> RbPinUart1R {
        RbPinUart1R::new(((self.bits >> 5) & 1) != 0)
    }
    #[doc = "Bit 8 - RW, SCS/SCK0/MOSI/MISO alternate pin enable"]
    #[inline(always)]
    pub fn rb_pin_spi0(&self) -> RbPinSpi0R {
        RbPinSpi0R::new(((self.bits >> 8) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - RW, TMR0 alternate pin enable"]
    #[inline(always)]
    pub fn rb_pin_tmr0(&mut self) -> RbPinTmr0W<'_, R16PinAlternateSpec> {
        RbPinTmr0W::new(self, 0)
    }
    #[doc = "Bit 1 - RW, TMR1 alternate pin enable"]
    #[inline(always)]
    pub fn rb_pin_tmr1(&mut self) -> RbPinTmr1W<'_, R16PinAlternateSpec> {
        RbPinTmr1W::new(self, 1)
    }
    #[doc = "Bit 2 - RW, TMR2 alternate pin enable"]
    #[inline(always)]
    pub fn rb_pin_tmr2(&mut self) -> RbPinTmr2W<'_, R16PinAlternateSpec> {
        RbPinTmr2W::new(self, 2)
    }
    #[doc = "Bit 4 - RW, RXD0/TXD0 alternate pin enable"]
    #[inline(always)]
    pub fn rb_pin_uart0(&mut self) -> RbPinUart0W<'_, R16PinAlternateSpec> {
        RbPinUart0W::new(self, 4)
    }
    #[doc = "Bit 5 - RW, RXD1/TXD1 alternate pin enable"]
    #[inline(always)]
    pub fn rb_pin_uart1(&mut self) -> RbPinUart1W<'_, R16PinAlternateSpec> {
        RbPinUart1W::new(self, 5)
    }
    #[doc = "Bit 8 - RW, SCS/SCK0/MOSI/MISO alternate pin enable"]
    #[inline(always)]
    pub fn rb_pin_spi0(&mut self) -> RbPinSpi0W<'_, R16PinAlternateSpec> {
        RbPinSpi0W::new(self, 8)
    }
}
#[doc = "RW, function pin alternate configuration\n\nYou can [`read`](crate::Reg::read) this register and get [`r16_pin_alternate::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`r16_pin_alternate::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct R16PinAlternateSpec;
impl crate::RegisterSpec for R16PinAlternateSpec {
    type Ux = u16;
}
#[doc = "`read()` method returns [`r16_pin_alternate::R`](R) reader structure"]
impl crate::Readable for R16PinAlternateSpec {}
#[doc = "`write(|w| ..)` method takes [`r16_pin_alternate::W`](W) writer structure"]
impl crate::Writable for R16PinAlternateSpec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets R16_PIN_ALTERNATE to value 0"]
impl crate::Resettable for R16PinAlternateSpec {}
